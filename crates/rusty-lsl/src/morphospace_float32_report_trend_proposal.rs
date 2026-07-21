// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic, effect-free trend advice over the concrete P37 observation window.
//!
//! The owner borrows only exact P36 observations and the window's checked totals before returning
//! the complete window. It is crate-private, default-inert proposal data and owns no downstream
//! action or authority.

use crate::morphospace_float32_report_observation_window::{
    MorphospaceFloat32ReportObservationWindow, MorphospaceFloat32ReportObservationWindowTotals,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportTrendThresholds {
    maximum_reports: usize,
    maximum_records_per_report: usize,
    maximum_total_records: u64,
    maximum_explicit_missing_sequences: u64,
    maximum_duplicates: u64,
    maximum_out_of_order: u64,
    maximum_retained_changed: u64,
    maximum_absolute_adjustment_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportTrendThresholdError {
    ZeroMaximumReports,
    ZeroMaximumRecordsPerReport,
    MaximumReportsUnrepresentable { requested: usize },
    MaximumRecordsPerReportUnrepresentable { requested: usize },
    NonFiniteMaximumAbsoluteAdjustment { bits: u64 },
    NegativeMaximumAbsoluteAdjustment { bits: u64 },
}

impl MorphospaceFloat32ReportTrendThresholds {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        maximum_reports: usize,
        maximum_records_per_report: usize,
        maximum_total_records: u64,
        maximum_explicit_missing_sequences: u64,
        maximum_duplicates: u64,
        maximum_out_of_order: u64,
        maximum_retained_changed: u64,
        maximum_absolute_adjustment: f64,
    ) -> Result<Self, MorphospaceFloat32ReportTrendThresholdError> {
        if maximum_reports == 0 {
            return Err(MorphospaceFloat32ReportTrendThresholdError::ZeroMaximumReports);
        }
        if maximum_records_per_report == 0 {
            return Err(MorphospaceFloat32ReportTrendThresholdError::ZeroMaximumRecordsPerReport);
        }
        u64::try_from(maximum_reports).map_err(|_| {
            MorphospaceFloat32ReportTrendThresholdError::MaximumReportsUnrepresentable {
                requested: maximum_reports,
            }
        })?;
        u64::try_from(maximum_records_per_report).map_err(|_| {
            MorphospaceFloat32ReportTrendThresholdError::MaximumRecordsPerReportUnrepresentable {
                requested: maximum_records_per_report,
            }
        })?;
        if !maximum_absolute_adjustment.is_finite() {
            return Err(
                MorphospaceFloat32ReportTrendThresholdError::NonFiniteMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                },
            );
        }
        if maximum_absolute_adjustment < 0.0 {
            return Err(
                MorphospaceFloat32ReportTrendThresholdError::NegativeMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                },
            );
        }
        Ok(Self {
            maximum_reports,
            maximum_records_per_report,
            maximum_total_records,
            maximum_explicit_missing_sequences,
            maximum_duplicates,
            maximum_out_of_order,
            maximum_retained_changed,
            maximum_absolute_adjustment_bits: maximum_absolute_adjustment.to_bits(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportTrendLargestAdjustment {
    pub(crate) report_index: u64,
    pub(crate) record_index: u64,
    pub(crate) sequence: u64,
    pub(crate) signed_adjustment_bits: u64,
    pub(crate) absolute_adjustment_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportTrendAggregates {
    pub(crate) window_totals: MorphospaceFloat32ReportObservationWindowTotals,
    pub(crate) largest_absolute_adjustment: Option<MorphospaceFloat32ReportTrendLargestAdjustment>,
}

/// Reasons are emitted in this declaration order and only for strict threshold exceedance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportTrendReviewReason {
    TotalRecords {
        observed: u64,
        maximum: u64,
    },
    ExplicitMissingSequences {
        observed: u64,
        maximum: u64,
    },
    Duplicates {
        observed: u64,
        maximum: u64,
    },
    OutOfOrder {
        observed: u64,
        maximum: u64,
    },
    RetainedChanged {
        observed: u64,
        maximum: u64,
    },
    AbsoluteAdjustment {
        report_index: u64,
        record_index: u64,
        sequence: u64,
        signed_adjustment_bits: u64,
        observed_absolute_bits: u64,
        maximum_absolute_bits: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportTrendProposal {
    Retain {
        window: MorphospaceFloat32ReportObservationWindow,
        aggregates: MorphospaceFloat32ReportTrendAggregates,
    },
    Review {
        window: MorphospaceFloat32ReportObservationWindow,
        aggregates: MorphospaceFloat32ReportTrendAggregates,
        reasons: Vec<MorphospaceFloat32ReportTrendReviewReason>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportTrendCounter {
    RecordCount,
    ObservationCount,
    ExplicitMissingSequences,
    Duplicates,
    OutOfOrder,
    RetainedChanged,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportTrendError {
    EmptyWindow {
        window: MorphospaceFloat32ReportObservationWindow,
    },
    ReportLimit {
        limit: usize,
        actual: usize,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    RecordLimit {
        report_index: u64,
        limit: usize,
        actual: usize,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    CounterOverflow {
        report_index: u64,
        counter: MorphospaceFloat32ReportTrendCounter,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    WindowTotalsMismatch {
        window: MorphospaceFloat32ReportObservationWindow,
    },
    Allocation {
        requested_reasons: usize,
        window: MorphospaceFloat32ReportObservationWindow,
    },
}

impl MorphospaceFloat32ReportTrendError {
    pub(crate) fn into_window(self) -> MorphospaceFloat32ReportObservationWindow {
        match self {
            Self::EmptyWindow { window }
            | Self::ReportLimit { window, .. }
            | Self::RecordLimit { window, .. }
            | Self::CounterOverflow { window, .. }
            | Self::WindowTotalsMismatch { window }
            | Self::Allocation { window, .. } => window,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportTrendProposalOwner {
    thresholds: MorphospaceFloat32ReportTrendThresholds,
}

impl MorphospaceFloat32ReportTrendProposalOwner {
    pub(crate) const fn new(thresholds: MorphospaceFloat32ReportTrendThresholds) -> Self {
        Self { thresholds }
    }

    pub(crate) fn propose(
        &self,
        window: MorphospaceFloat32ReportObservationWindow,
    ) -> Result<MorphospaceFloat32ReportTrendProposal, MorphospaceFloat32ReportTrendError> {
        self.propose_with(
            window,
            |reasons, requested| reasons.try_reserve_exact(requested).map_err(|_| ()),
            |current, value| current.checked_add(value).ok_or(()),
        )
    }

    fn propose_with<R, A>(
        &self,
        window: MorphospaceFloat32ReportObservationWindow,
        reserve: R,
        mut add: A,
    ) -> Result<MorphospaceFloat32ReportTrendProposal, MorphospaceFloat32ReportTrendError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportTrendReviewReason>, usize) -> Result<(), ()>,
        A: FnMut(u64, u64) -> Result<u64, ()>,
    {
        let observations = window.observations();
        if observations.is_empty() {
            return Err(MorphospaceFloat32ReportTrendError::EmptyWindow { window });
        }
        if observations.len() > self.thresholds.maximum_reports {
            return Err(MorphospaceFloat32ReportTrendError::ReportLimit {
                limit: self.thresholds.maximum_reports,
                actual: observations.len(),
                window,
            });
        }

        let collected = match collect_exact(
            &window,
            self.thresholds.maximum_records_per_report,
            &mut add,
        ) {
            Ok(value) => value,
            Err(failure) => return Err(failure.attach(window)),
        };
        if !collected.matches(window.totals()) {
            return Err(MorphospaceFloat32ReportTrendError::WindowTotalsMismatch { window });
        }
        let aggregates = MorphospaceFloat32ReportTrendAggregates {
            window_totals: window.totals(),
            largest_absolute_adjustment: collected.largest_absolute_adjustment,
        };

        const MAXIMUM_REASONS: usize = 6;
        let mut reasons = Vec::new();
        if reserve(&mut reasons, MAXIMUM_REASONS).is_err() {
            return Err(MorphospaceFloat32ReportTrendError::Allocation {
                requested_reasons: MAXIMUM_REASONS,
                window,
            });
        }
        let totals = aggregates.window_totals;
        push_threshold(
            &mut reasons,
            totals.record_count(),
            self.thresholds.maximum_total_records,
            |observed, maximum| MorphospaceFloat32ReportTrendReviewReason::TotalRecords {
                observed,
                maximum,
            },
        );
        push_threshold(
            &mut reasons,
            totals.explicit_missing_sequence_count(),
            self.thresholds.maximum_explicit_missing_sequences,
            |observed, maximum| {
                MorphospaceFloat32ReportTrendReviewReason::ExplicitMissingSequences {
                    observed,
                    maximum,
                }
            },
        );
        push_threshold(
            &mut reasons,
            totals.duplicate_count(),
            self.thresholds.maximum_duplicates,
            |observed, maximum| MorphospaceFloat32ReportTrendReviewReason::Duplicates {
                observed,
                maximum,
            },
        );
        push_threshold(
            &mut reasons,
            totals.out_of_order_count(),
            self.thresholds.maximum_out_of_order,
            |observed, maximum| MorphospaceFloat32ReportTrendReviewReason::OutOfOrder {
                observed,
                maximum,
            },
        );
        push_threshold(
            &mut reasons,
            totals.retained_changed_count(),
            self.thresholds.maximum_retained_changed,
            |observed, maximum| MorphospaceFloat32ReportTrendReviewReason::RetainedChanged {
                observed,
                maximum,
            },
        );
        if let Some(largest) = aggregates.largest_absolute_adjustment {
            if f64::from_bits(largest.absolute_adjustment_bits)
                > f64::from_bits(self.thresholds.maximum_absolute_adjustment_bits)
            {
                reasons.push(
                    MorphospaceFloat32ReportTrendReviewReason::AbsoluteAdjustment {
                        report_index: largest.report_index,
                        record_index: largest.record_index,
                        sequence: largest.sequence,
                        signed_adjustment_bits: largest.signed_adjustment_bits,
                        observed_absolute_bits: largest.absolute_adjustment_bits,
                        maximum_absolute_bits: self.thresholds.maximum_absolute_adjustment_bits,
                    },
                );
            }
        }

        if reasons.is_empty() {
            Ok(MorphospaceFloat32ReportTrendProposal::Retain { window, aggregates })
        } else {
            Ok(MorphospaceFloat32ReportTrendProposal::Review {
                window,
                aggregates,
                reasons,
            })
        }
    }
}

#[derive(Clone, Copy)]
struct Collected {
    record_count: u64,
    observation_count: u64,
    explicit_missing_sequences: u64,
    duplicates: u64,
    out_of_order: u64,
    retained_changed: u64,
    largest_absolute_adjustment: Option<MorphospaceFloat32ReportTrendLargestAdjustment>,
}

impl Collected {
    fn matches(self, totals: MorphospaceFloat32ReportObservationWindowTotals) -> bool {
        self.record_count == totals.record_count()
            && self.observation_count == totals.observation_count()
            && self.explicit_missing_sequences == totals.explicit_missing_sequence_count()
            && self.duplicates == totals.duplicate_count()
            && self.out_of_order == totals.out_of_order_count()
            && self.retained_changed == totals.retained_changed_count()
    }
}

enum CollectFailure {
    RecordLimit {
        report_index: u64,
        limit: usize,
        actual: usize,
    },
    CounterOverflow {
        report_index: u64,
        counter: MorphospaceFloat32ReportTrendCounter,
    },
}

impl CollectFailure {
    fn attach(
        self,
        window: MorphospaceFloat32ReportObservationWindow,
    ) -> MorphospaceFloat32ReportTrendError {
        match self {
            Self::RecordLimit {
                report_index,
                limit,
                actual,
            } => MorphospaceFloat32ReportTrendError::RecordLimit {
                report_index,
                limit,
                actual,
                window,
            },
            Self::CounterOverflow {
                report_index,
                counter,
            } => MorphospaceFloat32ReportTrendError::CounterOverflow {
                report_index,
                counter,
                window,
            },
        }
    }
}

fn collect_exact<A>(
    window: &MorphospaceFloat32ReportObservationWindow,
    maximum_records_per_report: usize,
    add: &mut A,
) -> Result<Collected, CollectFailure>
where
    A: FnMut(u64, u64) -> Result<u64, ()>,
{
    let mut collected = Collected {
        record_count: 0,
        observation_count: 0,
        explicit_missing_sequences: 0,
        duplicates: 0,
        out_of_order: 0,
        retained_changed: 0,
        largest_absolute_adjustment: None,
    };
    for (report_position, observation) in window.observations().iter().enumerate() {
        let report_index = u64::try_from(report_position)
            .expect("the concrete window validates its maximum report count");
        if observation.records().len() > maximum_records_per_report {
            return Err(CollectFailure::RecordLimit {
                report_index,
                limit: maximum_records_per_report,
                actual: observation.records().len(),
            });
        }
        let health = observation.terminal_health();
        checked_add(
            add,
            &mut collected.record_count,
            u64::try_from(observation.records().len())
                .expect("the concrete P36 observation validates its maximum record count"),
            report_index,
            MorphospaceFloat32ReportTrendCounter::RecordCount,
        )?;
        checked_add(
            add,
            &mut collected.observation_count,
            health.observation_count(),
            report_index,
            MorphospaceFloat32ReportTrendCounter::ObservationCount,
        )?;
        checked_add(
            add,
            &mut collected.explicit_missing_sequences,
            health.explicit_missing_sequence_count(),
            report_index,
            MorphospaceFloat32ReportTrendCounter::ExplicitMissingSequences,
        )?;
        checked_add(
            add,
            &mut collected.duplicates,
            health.duplicate_count(),
            report_index,
            MorphospaceFloat32ReportTrendCounter::Duplicates,
        )?;
        checked_add(
            add,
            &mut collected.out_of_order,
            health.out_of_order_count(),
            report_index,
            MorphospaceFloat32ReportTrendCounter::OutOfOrder,
        )?;
        checked_add(
            add,
            &mut collected.retained_changed,
            health.retained_changed_count(),
            report_index,
            MorphospaceFloat32ReportTrendCounter::RetainedChanged,
        )?;

        for record in observation.records() {
            let adjustment = record.processed().facts().adjustment();
            debug_assert!(
                adjustment.is_finite(),
                "P36 admits only finite adjustment facts"
            );
            let candidate = MorphospaceFloat32ReportTrendLargestAdjustment {
                report_index,
                record_index: record.index(),
                sequence: record.sequence(),
                signed_adjustment_bits: adjustment.to_bits(),
                absolute_adjustment_bits: adjustment.abs().to_bits(),
            };
            // Concrete windows and P36 records are ordered by ascending zero-based indices.
            // Strict replacement therefore makes report index, then record index, the tie order.
            if collected
                .largest_absolute_adjustment
                .map(|current| adjustment.abs() > f64::from_bits(current.absolute_adjustment_bits))
                .unwrap_or(true)
            {
                collected.largest_absolute_adjustment = Some(candidate);
            }
        }
    }
    Ok(collected)
}

fn checked_add<A>(
    add: &mut A,
    current: &mut u64,
    value: u64,
    report_index: u64,
    counter: MorphospaceFloat32ReportTrendCounter,
) -> Result<(), CollectFailure>
where
    A: FnMut(u64, u64) -> Result<u64, ()>,
{
    *current = add(*current, value).map_err(|_| CollectFailure::CounterOverflow {
        report_index,
        counter,
    })?;
    Ok(())
}

fn push_threshold(
    reasons: &mut Vec<MorphospaceFloat32ReportTrendReviewReason>,
    observed: u64,
    maximum: u64,
    make: fn(u64, u64) -> MorphospaceFloat32ReportTrendReviewReason,
) {
    if observed > maximum {
        reasons.push(make(observed, maximum));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_observation::{
        tests::outcome_with, MorphospaceFloat32ReportObservation,
        MorphospaceFloat32ReportObservationOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn observation(
        sequences: Vec<u64>,
        timestamps: Vec<f64>,
    ) -> MorphospaceFloat32ReportObservation {
        let count = timestamps.len();
        let records = timestamps
            .into_iter()
            .enumerate()
            .map(|(index, timestamp)| {
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![index as f32]).unwrap(),
                    RawSourceTimestamp::new(timestamp).unwrap(),
                    None,
                )
            })
            .collect();
        MorphospaceFloat32ReportObservationOwner::new(count)
            .unwrap()
            .observe(outcome_with(
                sequences,
                records,
                RequestedTimestampPostProcessing::Monotonic(
                    RequestedTimestampPostProcessingConfig::new(8, 1.0, f64::MAX).unwrap(),
                ),
            ))
            .unwrap()
    }

    fn window(
        observations: Vec<MorphospaceFloat32ReportObservation>,
    ) -> MorphospaceFloat32ReportObservationWindow {
        let records: usize = observations.iter().map(|value| value.records().len()).sum();
        observations.into_iter().fold(
            MorphospaceFloat32ReportObservationWindow::new(8, records.max(1)).unwrap(),
            |window, observation| window.append(observation).unwrap(),
        )
    }

    fn pointers(window: &MorphospaceFloat32ReportObservationWindow) -> Vec<*const f32> {
        window
            .observations()
            .iter()
            .flat_map(|observation| observation.records().iter())
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect()
    }

    fn thresholds(maximum: u64) -> MorphospaceFloat32ReportTrendThresholds {
        MorphospaceFloat32ReportTrendThresholds::new(
            8,
            8,
            maximum,
            maximum,
            maximum,
            maximum,
            maximum,
            maximum as f64,
        )
        .unwrap()
    }

    fn parts(
        proposal: MorphospaceFloat32ReportTrendProposal,
    ) -> (
        MorphospaceFloat32ReportObservationWindow,
        MorphospaceFloat32ReportTrendAggregates,
        Vec<MorphospaceFloat32ReportTrendReviewReason>,
    ) {
        match proposal {
            MorphospaceFloat32ReportTrendProposal::Retain { window, aggregates } => {
                (window, aggregates, Vec::new())
            }
            MorphospaceFloat32ReportTrendProposal::Review {
                window,
                aggregates,
                reasons,
            } => (window, aggregates, reasons),
        }
    }

    #[test]
    fn zero_extreme_empty_and_strict_threshold_edges_are_exact() {
        assert_eq!(
            MorphospaceFloat32ReportTrendThresholds::new(0, 1, 0, 0, 0, 0, 0, 0.0),
            Err(MorphospaceFloat32ReportTrendThresholdError::ZeroMaximumReports)
        );
        assert_eq!(
            MorphospaceFloat32ReportTrendThresholds::new(1, 0, 0, 0, 0, 0, 0, 0.0),
            Err(MorphospaceFloat32ReportTrendThresholdError::ZeroMaximumRecordsPerReport)
        );
        assert!(MorphospaceFloat32ReportTrendThresholds::new(
            usize::MAX,
            usize::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            f64::MAX
        )
        .is_ok());
        assert!(matches!(
            MorphospaceFloat32ReportTrendThresholds::new(1, 1, 0, 0, 0, 0, 0, f64::INFINITY),
            Err(
                MorphospaceFloat32ReportTrendThresholdError::NonFiniteMaximumAbsoluteAdjustment { .. }
            )
        ));

        let empty = MorphospaceFloat32ReportObservationWindow::new(1, 1).unwrap();
        assert!(matches!(
            MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(0)).propose(empty),
            Err(MorphospaceFloat32ReportTrendError::EmptyWindow { .. })
        ));

        let actual = window(vec![observation(vec![0], vec![1.0])]);
        let totals = actual.totals();
        let exact = MorphospaceFloat32ReportTrendThresholds::new(
            1,
            1,
            totals.record_count(),
            totals.explicit_missing_sequence_count(),
            totals.duplicate_count(),
            totals.out_of_order_count(),
            totals.retained_changed_count(),
            0.0,
        )
        .unwrap();
        assert!(matches!(
            MorphospaceFloat32ReportTrendProposalOwner::new(exact)
                .propose(actual)
                .unwrap(),
            MorphospaceFloat32ReportTrendProposal::Retain { .. }
        ));
    }

    #[test]
    fn real_repeated_windows_preserve_allocations_exact_totals_reason_order_and_ties() {
        let build = || {
            window(vec![
                observation(vec![7, 7], vec![4.0, 2.0]),
                observation(vec![7, 7], vec![4.0, 2.0]),
            ])
        };
        for _ in 0..2 {
            let actual = build();
            let expected_pointers = pointers(&actual);
            let expected_totals = actual.totals();
            let (actual, aggregates, reasons) = parts(
                MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(0))
                    .propose(actual)
                    .unwrap(),
            );
            assert_eq!(pointers(&actual), expected_pointers);
            assert_eq!(aggregates.window_totals, expected_totals);
            assert_eq!(
                aggregates.largest_absolute_adjustment.unwrap().report_index,
                0
            );
            assert_eq!(
                aggregates.largest_absolute_adjustment.unwrap().record_index,
                1
            );
            assert_eq!(aggregates.largest_absolute_adjustment.unwrap().sequence, 7);
            assert!(matches!(
                reasons.as_slice(),
                [
                    MorphospaceFloat32ReportTrendReviewReason::TotalRecords { .. },
                    MorphospaceFloat32ReportTrendReviewReason::Duplicates { .. },
                    MorphospaceFloat32ReportTrendReviewReason::RetainedChanged { .. },
                    MorphospaceFloat32ReportTrendReviewReason::AbsoluteAdjustment { .. },
                ]
            ));
            assert_eq!(
                aggregates.window_totals.explicit_missing_sequence_count(),
                actual.totals().explicit_missing_sequence_count()
            );
            assert_eq!(
                aggregates.window_totals.observation_count(),
                actual.totals().observation_count()
            );
        }

        for (first_sequence, second_sequence) in [(9, 7), (7, 9)] {
            let actual = window(vec![
                observation(vec![first_sequence, first_sequence], vec![4.0, 2.0]),
                observation(vec![second_sequence, second_sequence], vec![4.0, 2.0]),
            ]);
            let (_, aggregates, _) = parts(
                MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(u64::MAX))
                    .propose(actual)
                    .unwrap(),
            );
            let largest = aggregates.largest_absolute_adjustment.unwrap();
            assert_eq!((largest.report_index, largest.record_index), (0, 1));
            assert_eq!(largest.sequence, first_sequence);
        }

        let actual = window(vec![observation(vec![7, 7], vec![1.0, 2.0])]);
        let (_, aggregates, _) = parts(
            MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(u64::MAX))
                .propose(actual)
                .unwrap(),
        );
        assert_eq!(
            aggregates.largest_absolute_adjustment.unwrap().record_index,
            0
        );
    }

    #[test]
    fn every_trend_failure_returns_the_complete_real_window() {
        let report_limited = window(vec![
            observation(vec![0], vec![1.0]),
            observation(vec![1], vec![2.0]),
        ]);
        let report_pointers = pointers(&report_limited);
        let owner = MorphospaceFloat32ReportTrendProposalOwner::new(
            MorphospaceFloat32ReportTrendThresholds::new(1, 2, 9, 9, 9, 9, 9, 9.0).unwrap(),
        );
        assert_eq!(
            pointers(&owner.propose(report_limited).unwrap_err().into_window()),
            report_pointers
        );

        let record_limited = window(vec![observation(vec![0, 1], vec![2.0, 1.0])]);
        let record_pointers = pointers(&record_limited);
        let owner = MorphospaceFloat32ReportTrendProposalOwner::new(
            MorphospaceFloat32ReportTrendThresholds::new(2, 1, 9, 9, 9, 9, 9, 9.0).unwrap(),
        );
        assert_eq!(
            pointers(&owner.propose(record_limited).unwrap_err().into_window()),
            record_pointers
        );

        let overflow = window(vec![observation(vec![0], vec![1.0])]);
        let overflow_pointers = pointers(&overflow);
        let returned = MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(9))
            .propose_with(overflow, |_, _| Ok(()), |_, _| Err(()))
            .unwrap_err()
            .into_window();
        assert_eq!(pointers(&returned), overflow_pointers);

        let allocation = window(vec![observation(vec![0], vec![1.0])]);
        let allocation_pointers = pointers(&allocation);
        let returned = MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(9))
            .propose_with(
                allocation,
                |_, _| Err(()),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err()
            .into_window();
        assert_eq!(pointers(&returned), allocation_pointers);

        let mismatch = window(vec![observation(vec![0], vec![1.0])]);
        let mismatch_pointers = pointers(&mismatch);
        let returned = MorphospaceFloat32ReportTrendProposalOwner::new(thresholds(9))
            .propose_with(
                mismatch,
                |_, _| Ok(()),
                |a, b| Ok(a.saturating_add(b).saturating_add(1)),
            )
            .unwrap_err()
            .into_window();
        assert_eq!(pointers(&returned), mismatch_pointers);
    }

    #[test]
    fn private_structure_has_no_runtime_activation_or_authority_surface() {
        fn concrete_only(
            _: MorphospaceFloat32ReportObservationWindow,
        ) -> Result<MorphospaceFloat32ReportTrendProposal, MorphospaceFloat32ReportTrendError>
        {
            unreachable!()
        }
        let _ = concrete_only;
        let lib = include_str!("lib.rs");
        let runtime = include_str!("runtime.rs");
        let activation = include_str!("runtime_activation.rs");
        assert_eq!(
            lib.matches("mod morphospace_float32_report_trend_proposal;")
                .count(),
            1
        );
        assert!(!lib.contains("pub mod morphospace_float32_report_trend_proposal"));
        assert!(!runtime.contains("MorphospaceFloat32ReportTrend"));
        assert!(!activation.contains("MorphospaceFloat32ReportTrend"));
        assert!(!lib.contains("pub use morphospace_float32_report_trend_proposal"));
        let source = include_str!("morphospace_float32_report_trend_proposal.rs");
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn ad", "mit("),
            concat!("fn ro", "ute("),
            concat!("fn le", "ase("),
            concat!("fn re", "vise("),
            concat!("fn auth", "orize("),
            concat!("fn au", "dit("),
        ] {
            assert!(!source.contains(operation));
        }
    }
}
