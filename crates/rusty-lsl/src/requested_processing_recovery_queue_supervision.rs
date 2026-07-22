// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Immutable supervision of an ordered series of exact P62 execution reports.
//!
//! The supervisor accepts snapshots only. It performs no execution and treats facts
//! absent from P62, notably loss, scheduling, and policy, as unavailable.

use crate::{
    RequestedProcessingExecutionHealth, RequestedProcessingExecutionStage,
    RequestedProcessingExecutionTermination, RequestedProcessingRecoveryQueueExecutionReport,
};

/// Explicit statement of the loss evidence available to this contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedProcessingSupervisionLossFacts {
    /// P62 reports contain no loss count or loss classification.
    NotReportedByP62,
}

/// Fixed admission bound for one immutable supervision value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedProcessingSupervisionLimits {
    report_limit: usize,
}

impl RequestedProcessingSupervisionLimits {
    /// Creates a nonzero report bound.
    pub const fn new(report_limit: usize) -> Result<Self, RequestedProcessingSupervisionError> {
        if report_limit == 0 {
            return Err(RequestedProcessingSupervisionError::ZeroReportLimit);
        }
        Ok(Self { report_limit })
    }

    /// Returns the exact report bound.
    pub const fn report_limit(self) -> usize {
        self.report_limit
    }
}

/// Exact fixed-size termination totals from the admitted reports.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RequestedProcessingSupervisionTerminations {
    /// Complete reports.
    pub complete: usize,
    /// Cancellation reports.
    pub cancelled: usize,
    /// Deadline reports.
    pub deadline: usize,
    /// Exhausted-recovery reports.
    pub recovery_exhausted: usize,
    /// Terminal-recovery reports.
    pub recovery_terminal: usize,
    /// Full-queue backpressure reports.
    pub queue_backpressure: usize,
    /// Closed-queue reports.
    pub queue_closed: usize,
}

/// Exact fixed-size recovery and queue facts exposed by P62.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RequestedProcessingSupervisionOwnerFacts {
    /// Sum of exact recovery-attempt facts carried by terminations.
    pub completed_recovery_attempts: usize,
    /// Reports whose P62 health states that the current sample was recovered.
    pub recovered_current_reports: usize,
    /// Reports whose P62 health states that the current sample was processed.
    pub processed_current_reports: usize,
    /// Reports whose P62 health states that the queue observed the current sample.
    pub queue_observed_current_reports: usize,
    /// Exact total of current-report backpressure health facts.
    pub queue_backpressure_reports: usize,
    /// Exact total of current-report closure health facts.
    pub queue_closed_reports: usize,
    /// Last queue length explicitly carried by an admitted termination.
    pub last_queue_len: Option<usize>,
}

/// Immutable aggregation of one ordered, same-execution P62 snapshot series.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedProcessingRecoveryQueueSupervision {
    report_count: usize,
    total_execution_count: usize,
    first_completed_execution_count: usize,
    last_completed_execution_count: usize,
    last_remaining_execution_count: usize,
    last_current_execution_index: Option<usize>,
    last_health: RequestedProcessingExecutionHealth,
    terminations: RequestedProcessingSupervisionTerminations,
    owner_facts: RequestedProcessingSupervisionOwnerFacts,
}

impl RequestedProcessingRecoveryQueueSupervision {
    /// Validates and aggregates an ordered series of exact P62 snapshots.
    ///
    /// Every snapshot must describe the same total extent. The completed prefix may
    /// only advance, and a complete snapshot is final. These rules describe snapshot
    /// consistency only; they do not claim that P63 scheduled or caused progress.
    pub fn new(
        limits: RequestedProcessingSupervisionLimits,
        reports: &[RequestedProcessingRecoveryQueueExecutionReport],
    ) -> Result<Self, RequestedProcessingSupervisionError> {
        if reports.is_empty() {
            return Err(RequestedProcessingSupervisionError::EmptyReports);
        }
        if reports.len() > limits.report_limit {
            return Err(RequestedProcessingSupervisionError::ReportLimitExceeded {
                limit: limits.report_limit,
                actual: reports.len(),
            });
        }

        let first = reports[0];
        let total = first.total_execution_count();
        let mut prior_completed = first.completed_execution_count();
        let mut prior_complete = false;
        let mut terminations = RequestedProcessingSupervisionTerminations::default();
        let mut owner_facts = RequestedProcessingSupervisionOwnerFacts::default();

        for (report_index, report) in reports.iter().copied().enumerate() {
            if report.total_execution_count() != total {
                return Err(
                    RequestedProcessingSupervisionError::ExecutionExtentChanged {
                        report_index,
                        expected: total,
                        actual: report.total_execution_count(),
                    },
                );
            }
            if report_index != 0 && report.completed_execution_count() < prior_completed {
                return Err(
                    RequestedProcessingSupervisionError::CompletedPrefixRegressed {
                        report_index,
                        previous: prior_completed,
                        actual: report.completed_execution_count(),
                    },
                );
            }
            if prior_complete {
                return Err(RequestedProcessingSupervisionError::ReportAfterCompletion {
                    report_index,
                });
            }
            validate_health(report_index, report)?;
            observe_termination(report.termination(), &mut terminations, &mut owner_facts)?;
            observe_health(report.health(), &mut owner_facts)?;
            prior_completed = report.completed_execution_count();
            prior_complete = matches!(
                report.termination(),
                RequestedProcessingExecutionTermination::Complete
            );
        }

        let last = reports[reports.len() - 1];
        Ok(Self {
            report_count: reports.len(),
            total_execution_count: total,
            first_completed_execution_count: first.completed_execution_count(),
            last_completed_execution_count: last.completed_execution_count(),
            last_remaining_execution_count: last.remaining_execution_count(),
            last_current_execution_index: last.current_execution_index(),
            last_health: last.health(),
            terminations,
            owner_facts,
        })
    }

    /// Returns the number of admitted reports.
    pub const fn report_count(self) -> usize {
        self.report_count
    }
    /// Returns the unchanged execution extent shared by every report.
    pub const fn total_execution_count(self) -> usize {
        self.total_execution_count
    }
    /// Returns the completed prefix in the first report.
    pub const fn first_completed_execution_count(self) -> usize {
        self.first_completed_execution_count
    }
    /// Returns the completed prefix in the last report.
    pub const fn last_completed_execution_count(self) -> usize {
        self.last_completed_execution_count
    }
    /// Returns the remaining extent in the last report.
    pub const fn last_remaining_execution_count(self) -> usize {
        self.last_remaining_execution_count
    }
    /// Returns the exact current index from the last report.
    pub const fn last_current_execution_index(self) -> Option<usize> {
        self.last_current_execution_index
    }
    /// Returns the unchanged derived P62 health from the last report.
    pub const fn last_health(self) -> RequestedProcessingExecutionHealth {
        self.last_health
    }
    /// Returns exact termination totals.
    pub const fn terminations(self) -> RequestedProcessingSupervisionTerminations {
        self.terminations
    }
    /// Returns exact aggregated recovery and queue facts.
    pub const fn owner_facts(self) -> RequestedProcessingSupervisionOwnerFacts {
        self.owner_facts
    }
    /// States explicitly that P62 supplies no loss fact.
    pub const fn loss_facts(self) -> RequestedProcessingSupervisionLossFacts {
        RequestedProcessingSupervisionLossFacts::NotReportedByP62
    }
}

fn checked_increment(value: &mut usize) -> Result<(), RequestedProcessingSupervisionError> {
    *value = value
        .checked_add(1)
        .ok_or(RequestedProcessingSupervisionError::CounterOverflow)?;
    Ok(())
}

fn checked_add(
    value: &mut usize,
    amount: usize,
) -> Result<(), RequestedProcessingSupervisionError> {
    *value = value
        .checked_add(amount)
        .ok_or(RequestedProcessingSupervisionError::CounterOverflow)?;
    Ok(())
}

fn observe_health(
    health: RequestedProcessingExecutionHealth,
    facts: &mut RequestedProcessingSupervisionOwnerFacts,
) -> Result<(), RequestedProcessingSupervisionError> {
    if health.recovered_current() {
        checked_increment(&mut facts.recovered_current_reports)?;
    }
    if health.processed_current() {
        checked_increment(&mut facts.processed_current_reports)?;
    }
    if health.queue_observed_current() {
        checked_increment(&mut facts.queue_observed_current_reports)?;
    }
    checked_add(
        &mut facts.queue_backpressure_reports,
        health.queue_backpressure_count(),
    )?;
    checked_add(&mut facts.queue_closed_reports, health.queue_closed_count())
}

fn observe_termination(
    termination: RequestedProcessingExecutionTermination,
    totals: &mut RequestedProcessingSupervisionTerminations,
    facts: &mut RequestedProcessingSupervisionOwnerFacts,
) -> Result<(), RequestedProcessingSupervisionError> {
    use RequestedProcessingExecutionTermination::*;
    let (counter, attempts, queue_len) = match termination {
        Complete => (&mut totals.complete, 0, None),
        Cancelled {
            completed_recovery_attempts,
            queue_len,
            ..
        } => (
            &mut totals.cancelled,
            completed_recovery_attempts,
            queue_len,
        ),
        Deadline {
            completed_recovery_attempts,
            queue_len,
            ..
        } => (&mut totals.deadline, completed_recovery_attempts, queue_len),
        RecoveryExhausted { attempts } => (&mut totals.recovery_exhausted, attempts, None),
        RecoveryTerminal { completed_attempts } => {
            (&mut totals.recovery_terminal, completed_attempts, None)
        }
        QueueBackpressure {
            successful_recovery_attempt,
            queue_len,
        } => (
            &mut totals.queue_backpressure,
            successful_recovery_attempt,
            Some(queue_len),
        ),
        QueueClosed {
            successful_recovery_attempt,
            queue_len,
        } => (
            &mut totals.queue_closed,
            successful_recovery_attempt,
            Some(queue_len),
        ),
    };
    checked_increment(counter)?;
    checked_add(&mut facts.completed_recovery_attempts, attempts)?;
    if let Some(value) = queue_len {
        facts.last_queue_len = Some(value);
    }
    Ok(())
}

fn validate_health(
    report_index: usize,
    report: RequestedProcessingRecoveryQueueExecutionReport,
) -> Result<(), RequestedProcessingSupervisionError> {
    let health = report.health();
    if health.fully_completed_count() != report.completed_execution_count()
        || health.processed_current() && !health.recovered_current()
        || health.queue_observed_current() && !health.processed_current()
        || health.queue_backpressure_count() > 1
        || health.queue_closed_count() > 1
        || health.queue_backpressure_count()
            != usize::from(matches!(
                report.termination(),
                RequestedProcessingExecutionTermination::QueueBackpressure { .. }
            ))
        || health.queue_closed_count()
            != usize::from(matches!(
                report.termination(),
                RequestedProcessingExecutionTermination::QueueClosed { .. }
            ))
    {
        return Err(RequestedProcessingSupervisionError::ContradictoryHealth { report_index });
    }
    if matches!(
        report.termination(),
        RequestedProcessingExecutionTermination::Cancelled {
            stage: RequestedProcessingExecutionStage::Recovery,
            ..
        } | RequestedProcessingExecutionTermination::Deadline {
            stage: RequestedProcessingExecutionStage::Recovery,
            ..
        }
    ) && health.recovered_current()
    {
        return Err(RequestedProcessingSupervisionError::ContradictoryHealth { report_index });
    }
    Ok(())
}

/// Typed refusal; no partial supervision value is returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedProcessingSupervisionError {
    /// The report bound was zero.
    ZeroReportLimit,
    /// No report was supplied.
    EmptyReports,
    /// The supplied report count exceeded its bound.
    ReportLimitExceeded {
        /// Configured report bound.
        limit: usize,
        /// Supplied report count.
        actual: usize,
    },
    /// A later report named a different execution extent.
    ExecutionExtentChanged {
        /// Zero-based report position.
        report_index: usize,
        /// Extent established by the first report.
        expected: usize,
        /// Conflicting later extent.
        actual: usize,
    },
    /// A later report reduced the exact completed prefix.
    CompletedPrefixRegressed {
        /// Zero-based report position.
        report_index: usize,
        /// Prior completed prefix.
        previous: usize,
        /// Regressed completed prefix.
        actual: usize,
    },
    /// A snapshot followed an already complete snapshot.
    ReportAfterCompletion {
        /// Zero-based position of the contradictory report.
        report_index: usize,
    },
    /// A report's derived health contradicted its report facts.
    ContradictoryHealth {
        /// Zero-based report position.
        report_index: usize,
    },
    /// An exact aggregate was not representable by `usize`.
    CounterOverflow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RequestedProcessingExecutionReportLimits, RequestedProcessingExecutionStage};

    fn report(
        total: usize,
        completed: usize,
        termination: RequestedProcessingExecutionTermination,
    ) -> RequestedProcessingRecoveryQueueExecutionReport {
        RequestedProcessingRecoveryQueueExecutionReport::new(
            RequestedProcessingExecutionReportLimits::new(8, 3, 4).unwrap(),
            total,
            completed,
            termination,
        )
        .unwrap()
    }

    #[test]
    fn aggregates_exact_progress_terminations_queue_recovery_and_health() {
        let reports = [
            report(
                3,
                0,
                RequestedProcessingExecutionTermination::Cancelled {
                    stage: RequestedProcessingExecutionStage::Recovery,
                    completed_recovery_attempts: 1,
                    queue_len: None,
                },
            ),
            report(
                3,
                1,
                RequestedProcessingExecutionTermination::QueueBackpressure {
                    successful_recovery_attempt: 2,
                    queue_len: 4,
                },
            ),
            report(3, 3, RequestedProcessingExecutionTermination::Complete),
        ];
        let value = RequestedProcessingRecoveryQueueSupervision::new(
            RequestedProcessingSupervisionLimits::new(3).unwrap(),
            &reports,
        )
        .unwrap();
        assert_eq!(
            (
                value.report_count(),
                value.first_completed_execution_count(),
                value.last_completed_execution_count(),
                value.last_remaining_execution_count()
            ),
            (3, 0, 3, 0)
        );
        assert_eq!(value.terminations().cancelled, 1);
        assert_eq!(value.terminations().queue_backpressure, 1);
        assert_eq!(value.owner_facts().completed_recovery_attempts, 3);
        assert_eq!(value.owner_facts().last_queue_len, Some(4));
        assert_eq!(
            value.loss_facts(),
            RequestedProcessingSupervisionLossFacts::NotReportedByP62
        );
    }

    #[test]
    fn rejects_changed_extent_regression_and_report_after_complete() {
        let cancelled = |total, completed| {
            report(
                total,
                completed,
                RequestedProcessingExecutionTermination::Cancelled {
                    stage: RequestedProcessingExecutionStage::Recovery,
                    completed_recovery_attempts: 0,
                    queue_len: None,
                },
            )
        };
        assert!(matches!(
            RequestedProcessingRecoveryQueueSupervision::new(
                RequestedProcessingSupervisionLimits::new(2).unwrap(),
                &[cancelled(3, 0), cancelled(4, 0)]
            ),
            Err(
                RequestedProcessingSupervisionError::ExecutionExtentChanged {
                    report_index: 1,
                    ..
                }
            )
        ));
        assert!(matches!(
            RequestedProcessingRecoveryQueueSupervision::new(
                RequestedProcessingSupervisionLimits::new(2).unwrap(),
                &[cancelled(3, 1), cancelled(3, 0)]
            ),
            Err(
                RequestedProcessingSupervisionError::CompletedPrefixRegressed {
                    report_index: 1,
                    ..
                }
            )
        ));
        assert!(matches!(
            RequestedProcessingRecoveryQueueSupervision::new(
                RequestedProcessingSupervisionLimits::new(2).unwrap(),
                &[
                    report(3, 3, RequestedProcessingExecutionTermination::Complete),
                    report(3, 3, RequestedProcessingExecutionTermination::Complete)
                ]
            ),
            Err(RequestedProcessingSupervisionError::ReportAfterCompletion { report_index: 1 })
        ));
    }

    #[test]
    fn rejects_empty_or_unbounded_series_without_state() {
        assert_eq!(
            RequestedProcessingSupervisionLimits::new(0),
            Err(RequestedProcessingSupervisionError::ZeroReportLimit)
        );
        assert!(matches!(
            RequestedProcessingRecoveryQueueSupervision::new(
                RequestedProcessingSupervisionLimits::new(1).unwrap(),
                &[]
            ),
            Err(RequestedProcessingSupervisionError::EmptyReports)
        ));
        let one = report(1, 1, RequestedProcessingExecutionTermination::Complete);
        assert!(matches!(
            RequestedProcessingRecoveryQueueSupervision::new(
                RequestedProcessingSupervisionLimits::new(1).unwrap(),
                &[one, one]
            ),
            Err(RequestedProcessingSupervisionError::ReportLimitExceeded { .. })
        ));
    }
}
