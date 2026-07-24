// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stable, bounded, data-only reporting for requested-processing execution.
//!
//! This module accepts only facts already produced by processing, finite-recovery,
//! and bounded-queue owners. It performs no work on their behalf.

/// Fixed admission bounds for one report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedProcessingExecutionReportLimits {
    execution_limit: usize,
    recovery_attempt_limit: usize,
    queue_capacity: usize,
}

impl RequestedProcessingExecutionReportLimits {
    /// Creates explicit nonzero bounds.
    pub const fn new(
        execution_limit: usize,
        recovery_attempt_limit: usize,
        queue_capacity: usize,
    ) -> Result<Self, RequestedProcessingExecutionReportError> {
        if execution_limit == 0 {
            return Err(RequestedProcessingExecutionReportError::ZeroExecutionLimit);
        }
        if recovery_attempt_limit == 0 {
            return Err(RequestedProcessingExecutionReportError::ZeroRecoveryAttemptLimit);
        }
        if queue_capacity == 0 {
            return Err(RequestedProcessingExecutionReportError::ZeroQueueCapacity);
        }
        Ok(Self {
            execution_limit,
            recovery_attempt_limit,
            queue_capacity,
        })
    }

    /// Returns the exact maximum execution extent.
    pub const fn execution_limit(self) -> usize {
        self.execution_limit
    }
    /// Returns the exact per-execution recovery attempt bound.
    pub const fn recovery_attempt_limit(self) -> usize {
        self.recovery_attempt_limit
    }
    /// Returns the exact bounded-queue capacity.
    pub const fn queue_capacity(self) -> usize {
        self.queue_capacity
    }
}

/// Existing owner at which cancellation or deadline was observed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedProcessingExecutionStage {
    /// The finite-recovery owner stopped before producing a sample.
    Recovery,
    /// Requested processing stopped after successful recovery.
    RequestedProcessing,
    /// The queue owner stopped after successful requested processing.
    Queue,
}

/// Exact terminal fact for the first execution outside the completed prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedProcessingExecutionTermination {
    /// Every bounded execution completed processing and queue admission.
    Complete,
    /// Cancellation was observed at the named owner stage.
    Cancelled {
        /// Exact owner stage.
        stage: RequestedProcessingExecutionStage,
        /// Attempts completed by recovery before cancellation.
        completed_recovery_attempts: usize,
        /// Exact queue length, present only for the queue stage.
        queue_len: Option<usize>,
    },
    /// A deadline was observed at the named owner stage.
    Deadline {
        /// Exact owner stage.
        stage: RequestedProcessingExecutionStage,
        /// Attempts completed by recovery before the deadline.
        completed_recovery_attempts: usize,
        /// Exact queue length, present only for the queue stage.
        queue_len: Option<usize>,
    },
    /// Retryable recovery failures consumed the full attempt bound.
    RecoveryExhausted {
        /// Exact consumed attempt count.
        attempts: usize,
    },
    /// Recovery classified a terminal failure.
    RecoveryTerminal {
        /// Exact attempts completed before the terminal result.
        completed_attempts: usize,
    },
    /// The queue owner reported full-capacity backpressure.
    QueueBackpressure {
        /// One-based attempt on which recovery succeeded.
        successful_recovery_attempt: usize,
        /// Exact full queue length.
        queue_len: usize,
    },
    /// The queue owner reported closure.
    QueueClosed {
        /// One-based attempt on which recovery succeeded.
        successful_recovery_attempt: usize,
        /// Exact in-capacity queue length.
        queue_len: usize,
    },
}

/// Exact fixed-size health derived solely from the admitted execution evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedProcessingExecutionHealth {
    fully_completed_count: usize,
    recovered_current: bool,
    processed_current: bool,
    queue_observed_current: bool,
    queue_backpressure_count: usize,
    queue_closed_count: usize,
}

impl RequestedProcessingExecutionHealth {
    /// Returns the exact prefix that completed all three stages.
    pub const fn fully_completed_count(self) -> usize {
        self.fully_completed_count
    }
    /// Reports whether recovery produced the current sample.
    pub const fn recovered_current(self) -> bool {
        self.recovered_current
    }
    /// Reports whether requested processing completed for the current sample.
    pub const fn processed_current(self) -> bool {
        self.processed_current
    }
    /// Reports whether the queue owner observed the current sample.
    pub const fn queue_observed_current(self) -> bool {
        self.queue_observed_current
    }
    /// Returns one only for a current backpressure termination.
    pub const fn queue_backpressure_count(self) -> usize {
        self.queue_backpressure_count
    }
    /// Returns one only for a current closed-queue termination.
    pub const fn queue_closed_count(self) -> usize {
        self.queue_closed_count
    }
}

/// Immutable accepted execution report. It owns no samples or subordinate state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedProcessingRecoveryQueueExecutionReport {
    total_execution_count: usize,
    completed_execution_count: usize,
    remaining_execution_count: usize,
    current_execution_index: Option<usize>,
    termination: RequestedProcessingExecutionTermination,
    health: RequestedProcessingExecutionHealth,
}

impl RequestedProcessingRecoveryQueueExecutionReport {
    /// Transactionally validates and constructs the complete report.
    pub fn new(
        limits: RequestedProcessingExecutionReportLimits,
        total_execution_count: usize,
        completed_execution_count: usize,
        termination: RequestedProcessingExecutionTermination,
    ) -> Result<Self, RequestedProcessingExecutionReportError> {
        if total_execution_count == 0 {
            return Err(RequestedProcessingExecutionReportError::ZeroExecutionCount);
        }
        if total_execution_count > limits.execution_limit {
            return Err(
                RequestedProcessingExecutionReportError::ExecutionLimitExceeded {
                    limit: limits.execution_limit,
                    actual: total_execution_count,
                },
            );
        }
        if completed_execution_count > total_execution_count {
            return Err(
                RequestedProcessingExecutionReportError::CompletedExceedsTotal {
                    total: total_execution_count,
                    completed: completed_execution_count,
                },
            );
        }

        let complete = matches!(
            termination,
            RequestedProcessingExecutionTermination::Complete
        );
        if complete != (completed_execution_count == total_execution_count) {
            return Err(RequestedProcessingExecutionReportError::CompletionContradiction);
        }

        let current_execution_index = if complete {
            None
        } else {
            Some(completed_execution_count)
        };
        let remaining_execution_count = total_execution_count - completed_execution_count;
        let mut health = RequestedProcessingExecutionHealth {
            fully_completed_count: completed_execution_count,
            recovered_current: false,
            processed_current: false,
            queue_observed_current: false,
            queue_backpressure_count: 0,
            queue_closed_count: 0,
        };

        match termination {
            RequestedProcessingExecutionTermination::Complete => {}
            RequestedProcessingExecutionTermination::RecoveryExhausted { attempts } => {
                validate_positive_attempt(attempts, limits)?;
                if attempts != limits.recovery_attempt_limit {
                    return Err(
                        RequestedProcessingExecutionReportError::PrematureRecoveryExhaustion {
                            required: limits.recovery_attempt_limit,
                            actual: attempts,
                        },
                    );
                }
            }
            RequestedProcessingExecutionTermination::RecoveryTerminal { completed_attempts } => {
                validate_completed_attempts(completed_attempts, limits)?;
            }
            RequestedProcessingExecutionTermination::QueueBackpressure {
                successful_recovery_attempt,
                queue_len,
            } => {
                validate_positive_attempt(successful_recovery_attempt, limits)?;
                validate_queue_len(queue_len, limits)?;
                if queue_len != limits.queue_capacity {
                    return Err(
                        RequestedProcessingExecutionReportError::BackpressureWithoutFullQueue {
                            capacity: limits.queue_capacity,
                            actual: queue_len,
                        },
                    );
                }
                health.recovered_current = true;
                health.processed_current = true;
                health.queue_observed_current = true;
                health.queue_backpressure_count = 1;
            }
            RequestedProcessingExecutionTermination::QueueClosed {
                successful_recovery_attempt,
                queue_len,
            } => {
                validate_positive_attempt(successful_recovery_attempt, limits)?;
                validate_queue_len(queue_len, limits)?;
                health.recovered_current = true;
                health.processed_current = true;
                health.queue_observed_current = true;
                health.queue_closed_count = 1;
            }
            RequestedProcessingExecutionTermination::Cancelled {
                stage,
                completed_recovery_attempts,
                queue_len,
            }
            | RequestedProcessingExecutionTermination::Deadline {
                stage,
                completed_recovery_attempts,
                queue_len,
            } => {
                validate_completed_attempts(completed_recovery_attempts, limits)?;
                validate_stage(
                    stage,
                    completed_recovery_attempts,
                    queue_len,
                    limits,
                    &mut health,
                )?;
            }
        }

        Ok(Self {
            total_execution_count,
            completed_execution_count,
            remaining_execution_count,
            current_execution_index,
            termination,
            health,
        })
    }

    /// Returns the exact bounded execution extent.
    pub const fn total_execution_count(self) -> usize {
        self.total_execution_count
    }
    /// Returns the exact fully completed prefix length.
    pub const fn completed_execution_count(self) -> usize {
        self.completed_execution_count
    }
    /// Returns the exact current-and-suffix extent.
    pub const fn remaining_execution_count(self) -> usize {
        self.remaining_execution_count
    }
    /// Returns the zero-based current index for non-complete reports.
    pub const fn current_execution_index(self) -> Option<usize> {
        self.current_execution_index
    }
    /// Returns the exact supplied terminal fact.
    pub const fn termination(self) -> RequestedProcessingExecutionTermination {
        self.termination
    }
    /// Returns fixed-size health derived from admitted facts.
    pub const fn health(self) -> RequestedProcessingExecutionHealth {
        self.health
    }
}

fn validate_positive_attempt(
    attempts: usize,
    limits: RequestedProcessingExecutionReportLimits,
) -> Result<(), RequestedProcessingExecutionReportError> {
    if attempts == 0 {
        return Err(RequestedProcessingExecutionReportError::ZeroSuccessfulRecoveryAttempt);
    }
    validate_completed_attempts(attempts, limits)
}

fn validate_completed_attempts(
    attempts: usize,
    limits: RequestedProcessingExecutionReportLimits,
) -> Result<(), RequestedProcessingExecutionReportError> {
    if attempts > limits.recovery_attempt_limit {
        return Err(
            RequestedProcessingExecutionReportError::RecoveryAttemptLimitExceeded {
                limit: limits.recovery_attempt_limit,
                actual: attempts,
            },
        );
    }
    Ok(())
}

fn validate_queue_len(
    queue_len: usize,
    limits: RequestedProcessingExecutionReportLimits,
) -> Result<(), RequestedProcessingExecutionReportError> {
    if queue_len > limits.queue_capacity {
        return Err(
            RequestedProcessingExecutionReportError::QueueLengthExceedsCapacity {
                capacity: limits.queue_capacity,
                actual: queue_len,
            },
        );
    }
    Ok(())
}

fn validate_stage(
    stage: RequestedProcessingExecutionStage,
    attempts: usize,
    queue_len: Option<usize>,
    limits: RequestedProcessingExecutionReportLimits,
    health: &mut RequestedProcessingExecutionHealth,
) -> Result<(), RequestedProcessingExecutionReportError> {
    match stage {
        RequestedProcessingExecutionStage::Recovery => {
            if queue_len.is_some() {
                return Err(RequestedProcessingExecutionReportError::QueueFactBeforeQueueStage);
            }
        }
        RequestedProcessingExecutionStage::RequestedProcessing => {
            if attempts == 0 {
                return Err(
                    RequestedProcessingExecutionReportError::ProcessingWithoutRecoveredSample,
                );
            }
            if queue_len.is_some() {
                return Err(RequestedProcessingExecutionReportError::QueueFactBeforeQueueStage);
            }
            health.recovered_current = true;
        }
        RequestedProcessingExecutionStage::Queue => {
            if attempts == 0 {
                return Err(RequestedProcessingExecutionReportError::QueueWithoutRecoveredSample);
            }
            let queue_len = match queue_len {
                Some(value) => value,
                None => return Err(RequestedProcessingExecutionReportError::MissingQueueLength),
            };
            validate_queue_len(queue_len, limits)?;
            health.recovered_current = true;
            health.processed_current = true;
            health.queue_observed_current = true;
        }
    }
    Ok(())
}

/// Typed fail-closed refusal. No report exists after any refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedProcessingExecutionReportError {
    /// The execution bound was zero.
    ZeroExecutionLimit,
    /// The recovery attempt bound was zero.
    ZeroRecoveryAttemptLimit,
    /// The queue capacity was zero.
    ZeroQueueCapacity,
    /// The reported execution extent was zero.
    ZeroExecutionCount,
    /// The reported extent exceeded its bound.
    ExecutionLimitExceeded {
        /// Configured bound.
        limit: usize,
        /// Reported extent.
        actual: usize,
    },
    /// The completed prefix exceeded the total extent.
    CompletedExceedsTotal {
        /// Reported total.
        total: usize,
        /// Reported completed prefix.
        completed: usize,
    },
    /// Complete and extent facts disagreed.
    CompletionContradiction,
    /// Successful recovery claimed attempt zero.
    ZeroSuccessfulRecoveryAttempt,
    /// An attempt count exceeded its bound.
    RecoveryAttemptLimitExceeded {
        /// Configured bound.
        limit: usize,
        /// Reported attempts.
        actual: usize,
    },
    /// Exhaustion was claimed before the full attempt budget was consumed.
    PrematureRecoveryExhaustion {
        /// Required exact attempts.
        required: usize,
        /// Reported attempts.
        actual: usize,
    },
    /// A queue length exceeded capacity.
    QueueLengthExceedsCapacity {
        /// Configured capacity.
        capacity: usize,
        /// Reported queue length.
        actual: usize,
    },
    /// Backpressure was claimed below full capacity.
    BackpressureWithoutFullQueue {
        /// Configured capacity.
        capacity: usize,
        /// Reported queue length.
        actual: usize,
    },
    /// Processing-stage evidence lacked successful recovery.
    ProcessingWithoutRecoveredSample,
    /// Queue-stage evidence lacked successful recovery.
    QueueWithoutRecoveredSample,
    /// Recovery or processing evidence incorrectly included a queue fact.
    QueueFactBeforeQueueStage,
    /// Queue-stage evidence omitted the exact queue length.
    MissingQueueLength,
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMITS: RequestedProcessingExecutionReportLimits =
        match RequestedProcessingExecutionReportLimits::new(4, 3, 2) {
            Ok(value) => value,
            Err(_) => panic!("valid limits"),
        };

    fn report(
        termination: RequestedProcessingExecutionTermination,
    ) -> RequestedProcessingRecoveryQueueExecutionReport {
        RequestedProcessingRecoveryQueueExecutionReport::new(LIMITS, 4, 2, termination).unwrap()
    }

    #[test]
    fn complete_report_has_exact_bounded_extent_and_no_current_execution() {
        let value = RequestedProcessingRecoveryQueueExecutionReport::new(
            LIMITS,
            4,
            4,
            RequestedProcessingExecutionTermination::Complete,
        )
        .unwrap();
        assert_eq!(
            (
                value.total_execution_count(),
                value.completed_execution_count(),
                value.remaining_execution_count()
            ),
            (4, 4, 0)
        );
        assert_eq!(value.current_execution_index(), None);
        assert_eq!(value.health().fully_completed_count(), 4);
    }

    #[test]
    fn recovery_terminal_classes_bypass_processing_and_queue_health() {
        for termination in [
            RequestedProcessingExecutionTermination::Cancelled {
                stage: RequestedProcessingExecutionStage::Recovery,
                completed_recovery_attempts: 0,
                queue_len: None,
            },
            RequestedProcessingExecutionTermination::Deadline {
                stage: RequestedProcessingExecutionStage::Recovery,
                completed_recovery_attempts: 1,
                queue_len: None,
            },
            RequestedProcessingExecutionTermination::RecoveryExhausted { attempts: 3 },
            RequestedProcessingExecutionTermination::RecoveryTerminal {
                completed_attempts: 1,
            },
        ] {
            let value = report(termination);
            assert_eq!(value.current_execution_index(), Some(2));
            assert_eq!(value.remaining_execution_count(), 2);
            assert!(!value.health().recovered_current());
            assert!(!value.health().processed_current());
            assert!(!value.health().queue_observed_current());
        }
    }

    #[test]
    fn queue_outcomes_require_recovery_processing_and_exact_queue_facts() {
        let backpressure = report(RequestedProcessingExecutionTermination::QueueBackpressure {
            successful_recovery_attempt: 2,
            queue_len: 2,
        });
        assert!(backpressure.health().recovered_current());
        assert!(backpressure.health().processed_current());
        assert!(backpressure.health().queue_observed_current());
        assert_eq!(backpressure.health().queue_backpressure_count(), 1);

        let closed = report(RequestedProcessingExecutionTermination::QueueClosed {
            successful_recovery_attempt: 1,
            queue_len: 0,
        });
        assert_eq!(closed.health().queue_closed_count(), 1);
    }

    #[test]
    fn staged_cancellation_and_deadline_preserve_only_existing_stage_evidence() {
        let processing = report(RequestedProcessingExecutionTermination::Cancelled {
            stage: RequestedProcessingExecutionStage::RequestedProcessing,
            completed_recovery_attempts: 2,
            queue_len: None,
        });
        assert!(processing.health().recovered_current());
        assert!(!processing.health().processed_current());

        let queue = report(RequestedProcessingExecutionTermination::Deadline {
            stage: RequestedProcessingExecutionStage::Queue,
            completed_recovery_attempts: 1,
            queue_len: Some(1),
        });
        assert!(queue.health().processed_current());
        assert!(queue.health().queue_observed_current());
    }

    #[test]
    fn contradictory_and_unbounded_facts_fail_closed() {
        let cases = [
            RequestedProcessingRecoveryQueueExecutionReport::new(
                LIMITS,
                5,
                5,
                RequestedProcessingExecutionTermination::Complete,
            ),
            RequestedProcessingRecoveryQueueExecutionReport::new(
                LIMITS,
                4,
                3,
                RequestedProcessingExecutionTermination::Complete,
            ),
            RequestedProcessingRecoveryQueueExecutionReport::new(
                LIMITS,
                4,
                2,
                RequestedProcessingExecutionTermination::RecoveryExhausted { attempts: 2 },
            ),
            RequestedProcessingRecoveryQueueExecutionReport::new(
                LIMITS,
                4,
                2,
                RequestedProcessingExecutionTermination::QueueBackpressure {
                    successful_recovery_attempt: 1,
                    queue_len: 1,
                },
            ),
            RequestedProcessingRecoveryQueueExecutionReport::new(
                LIMITS,
                4,
                2,
                RequestedProcessingExecutionTermination::Cancelled {
                    stage: RequestedProcessingExecutionStage::Queue,
                    completed_recovery_attempts: 0,
                    queue_len: Some(0),
                },
            ),
            RequestedProcessingRecoveryQueueExecutionReport::new(
                LIMITS,
                4,
                2,
                RequestedProcessingExecutionTermination::Deadline {
                    stage: RequestedProcessingExecutionStage::Recovery,
                    completed_recovery_attempts: 0,
                    queue_len: Some(0),
                },
            ),
        ];
        for case in cases {
            assert!(case.is_err());
        }
    }
}
