// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic, non-applying comparison of two exact P37 observation windows.

use crate::morphospace_float32_report_observation_window::{
    MorphospaceFloat32ReportObservationWindow, MorphospaceFloat32ReportObservationWindowTotals,
};

const COUNTER_EVIDENCE_COUNT: usize = 11;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowDeltaBounds {
    maximum_reports_per_window: usize,
    maximum_records_per_report: usize,
    maximum_evidence: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaBoundsError {
    ZeroMaximumReportsPerWindow,
    ZeroMaximumRecordsPerReport,
    ZeroMaximumEvidence,
    MaximumReportsPerWindowUnrepresentable { requested: usize },
    MaximumRecordsPerReportUnrepresentable { requested: usize },
    MaximumEvidenceUnrepresentable { requested: usize },
}

impl MorphospaceFloat32ReportWindowDeltaBounds {
    pub(crate) fn new(
        maximum_reports_per_window: usize,
        maximum_records_per_report: usize,
        maximum_evidence: usize,
    ) -> Result<Self, MorphospaceFloat32ReportWindowDeltaBoundsError> {
        if maximum_reports_per_window == 0 {
            return Err(
                MorphospaceFloat32ReportWindowDeltaBoundsError::ZeroMaximumReportsPerWindow,
            );
        }
        if maximum_records_per_report == 0 {
            return Err(
                MorphospaceFloat32ReportWindowDeltaBoundsError::ZeroMaximumRecordsPerReport,
            );
        }
        if maximum_evidence == 0 {
            return Err(MorphospaceFloat32ReportWindowDeltaBoundsError::ZeroMaximumEvidence);
        }
        u64::try_from(maximum_reports_per_window).map_err(|_| {
            MorphospaceFloat32ReportWindowDeltaBoundsError::MaximumReportsPerWindowUnrepresentable {
                requested: maximum_reports_per_window,
            }
        })?;
        u64::try_from(maximum_records_per_report).map_err(|_| {
            MorphospaceFloat32ReportWindowDeltaBoundsError::MaximumRecordsPerReportUnrepresentable {
                requested: maximum_records_per_report,
            }
        })?;
        u64::try_from(maximum_evidence).map_err(|_| {
            MorphospaceFloat32ReportWindowDeltaBoundsError::MaximumEvidenceUnrepresentable {
                requested: maximum_evidence,
            }
        })?;
        Ok(Self {
            maximum_reports_per_window,
            maximum_records_per_report,
            maximum_evidence,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaDirection {
    Unchanged,
    Increased,
    Decreased,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaCounter {
    Reports,
    Records,
    Observations,
    First,
    Contiguous,
    Gaps,
    ExplicitObservedMissingSequences,
    Duplicates,
    OutOfOrder,
    RetainedUnchanged,
    RetainedChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowAdjustmentEvidence {
    pub(crate) report_index: u64,
    pub(crate) record_index: u64,
    pub(crate) sequence: u64,
    pub(crate) signed_bits: u64,
    pub(crate) absolute_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaEvidence {
    Counter {
        counter: MorphospaceFloat32ReportWindowDeltaCounter,
        direction: MorphospaceFloat32ReportWindowDeltaDirection,
        earlier: u64,
        later: u64,
        absolute_delta: u64,
    },
    LargestAbsoluteAdjustment {
        direction: MorphospaceFloat32ReportWindowDeltaDirection,
        earlier: Option<MorphospaceFloat32ReportWindowAdjustmentEvidence>,
        later: Option<MorphospaceFloat32ReportWindowAdjustmentEvidence>,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowDeltaProposal {
    earlier: MorphospaceFloat32ReportObservationWindow,
    later: MorphospaceFloat32ReportObservationWindow,
    evidence: Vec<MorphospaceFloat32ReportWindowDeltaEvidence>,
}

impl MorphospaceFloat32ReportWindowDeltaProposal {
    pub(crate) fn evidence(&self) -> &[MorphospaceFloat32ReportWindowDeltaEvidence] {
        &self.evidence
    }

    pub(crate) fn into_windows(
        self,
    ) -> (
        MorphospaceFloat32ReportObservationWindow,
        MorphospaceFloat32ReportObservationWindow,
    ) {
        (self.earlier, self.later)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaSide {
    Earlier,
    Later,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaError {
    ReportLimit {
        side: MorphospaceFloat32ReportWindowDeltaSide,
        limit: usize,
        actual: usize,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    },
    RecordLimit {
        side: MorphospaceFloat32ReportWindowDeltaSide,
        report_index: u64,
        limit: usize,
        actual: usize,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    },
    CounterOverflow {
        side: MorphospaceFloat32ReportWindowDeltaSide,
        report_index: u64,
        counter: MorphospaceFloat32ReportWindowDeltaCounter,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    },
    WindowTotalsMismatch {
        side: MorphospaceFloat32ReportWindowDeltaSide,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    },
    EvidenceLimit {
        limit: usize,
        required: usize,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    },
    Allocation {
        requested: usize,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    },
}

impl MorphospaceFloat32ReportWindowDeltaError {
    pub(crate) fn into_windows(
        self,
    ) -> (
        MorphospaceFloat32ReportObservationWindow,
        MorphospaceFloat32ReportObservationWindow,
    ) {
        match self {
            Self::ReportLimit { earlier, later, .. }
            | Self::RecordLimit { earlier, later, .. }
            | Self::CounterOverflow { earlier, later, .. }
            | Self::WindowTotalsMismatch { earlier, later, .. }
            | Self::EvidenceLimit { earlier, later, .. }
            | Self::Allocation { earlier, later, .. } => (earlier, later),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowDeltaProposalOwner {
    bounds: MorphospaceFloat32ReportWindowDeltaBounds,
}

impl MorphospaceFloat32ReportWindowDeltaProposalOwner {
    pub(crate) const fn new(bounds: MorphospaceFloat32ReportWindowDeltaBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn propose(
        &self,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    ) -> Result<MorphospaceFloat32ReportWindowDeltaProposal, MorphospaceFloat32ReportWindowDeltaError>
    {
        self.propose_with(
            earlier,
            later,
            |values, requested| values.try_reserve_exact(requested).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
        )
    }

    fn propose_with<R, A>(
        &self,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
        reserve: R,
        mut add: A,
    ) -> Result<MorphospaceFloat32ReportWindowDeltaProposal, MorphospaceFloat32ReportWindowDeltaError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportWindowDeltaEvidence>, usize) -> Result<(), ()>,
        A: FnMut(u64, u64) -> Result<u64, ()>,
    {
        let earlier_collected = match collect(&earlier, self.bounds, &mut add) {
            Ok(value) => value,
            Err(failure) => {
                return Err(failure.attach(
                    MorphospaceFloat32ReportWindowDeltaSide::Earlier,
                    earlier,
                    later,
                ))
            }
        };
        let later_collected = match collect(&later, self.bounds, &mut add) {
            Ok(value) => value,
            Err(failure) => {
                return Err(failure.attach(
                    MorphospaceFloat32ReportWindowDeltaSide::Later,
                    earlier,
                    later,
                ))
            }
        };
        if earlier_collected.totals != totals_values(earlier.totals()) {
            return Err(
                MorphospaceFloat32ReportWindowDeltaError::WindowTotalsMismatch {
                    side: MorphospaceFloat32ReportWindowDeltaSide::Earlier,
                    earlier,
                    later,
                },
            );
        }
        if later_collected.totals != totals_values(later.totals()) {
            return Err(
                MorphospaceFloat32ReportWindowDeltaError::WindowTotalsMismatch {
                    side: MorphospaceFloat32ReportWindowDeltaSide::Later,
                    earlier,
                    later,
                },
            );
        }
        let required = COUNTER_EVIDENCE_COUNT
            .checked_add(1)
            .expect("fixed evidence count fits usize");
        if required > self.bounds.maximum_evidence {
            return Err(MorphospaceFloat32ReportWindowDeltaError::EvidenceLimit {
                limit: self.bounds.maximum_evidence,
                required,
                earlier,
                later,
            });
        }
        let mut evidence = Vec::new();
        if reserve(&mut evidence, required).is_err() {
            return Err(MorphospaceFloat32ReportWindowDeltaError::Allocation {
                requested: required,
                earlier,
                later,
            });
        }
        let a = earlier.totals();
        let b = later.totals();
        for (counter, first, second) in [
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Reports,
                a.report_count(),
                b.report_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Records,
                a.record_count(),
                b.record_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Observations,
                a.observation_count(),
                b.observation_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::First,
                a.first_count(),
                b.first_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Contiguous,
                a.contiguous_count(),
                b.contiguous_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Gaps,
                a.gap_count(),
                b.gap_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::ExplicitObservedMissingSequences,
                a.explicit_missing_sequence_count(),
                b.explicit_missing_sequence_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Duplicates,
                a.duplicate_count(),
                b.duplicate_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::OutOfOrder,
                a.out_of_order_count(),
                b.out_of_order_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::RetainedUnchanged,
                a.retained_unchanged_count(),
                b.retained_unchanged_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::RetainedChanged,
                a.retained_changed_count(),
                b.retained_changed_count(),
            ),
        ] {
            let (direction, absolute_delta) = direction_and_delta(first, second);
            evidence.push(MorphospaceFloat32ReportWindowDeltaEvidence::Counter {
                counter,
                direction,
                earlier: first,
                later: second,
                absolute_delta,
            });
        }
        evidence.push(
            MorphospaceFloat32ReportWindowDeltaEvidence::LargestAbsoluteAdjustment {
                direction: adjustment_direction(
                    earlier_collected.largest_adjustment,
                    later_collected.largest_adjustment,
                ),
                earlier: earlier_collected.largest_adjustment,
                later: later_collected.largest_adjustment,
            },
        );
        Ok(MorphospaceFloat32ReportWindowDeltaProposal {
            earlier,
            later,
            evidence,
        })
    }
}

fn direction_and_delta(
    earlier: u64,
    later: u64,
) -> (MorphospaceFloat32ReportWindowDeltaDirection, u64) {
    use MorphospaceFloat32ReportWindowDeltaDirection::*;
    match later.cmp(&earlier) {
        std::cmp::Ordering::Equal => (Unchanged, 0),
        std::cmp::Ordering::Greater => (
            Increased,
            later.checked_sub(earlier).expect("ordered subtraction"),
        ),
        std::cmp::Ordering::Less => (
            Decreased,
            earlier.checked_sub(later).expect("ordered subtraction"),
        ),
    }
}

fn adjustment_direction(
    earlier: Option<MorphospaceFloat32ReportWindowAdjustmentEvidence>,
    later: Option<MorphospaceFloat32ReportWindowAdjustmentEvidence>,
) -> MorphospaceFloat32ReportWindowDeltaDirection {
    use MorphospaceFloat32ReportWindowDeltaDirection::*;
    match (earlier, later) {
        (None, None) => Unchanged,
        (None, Some(_)) => Increased,
        (Some(_), None) => Decreased,
        (Some(a), Some(b)) => {
            match f64::from_bits(b.absolute_bits).total_cmp(&f64::from_bits(a.absolute_bits)) {
                std::cmp::Ordering::Equal => Unchanged,
                std::cmp::Ordering::Greater => Increased,
                std::cmp::Ordering::Less => Decreased,
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Collected {
    totals: [u64; 11],
    largest_adjustment: Option<MorphospaceFloat32ReportWindowAdjustmentEvidence>,
}
enum CollectFailure {
    ReportLimit {
        limit: usize,
        actual: usize,
    },
    RecordLimit {
        report_index: u64,
        limit: usize,
        actual: usize,
    },
    CounterOverflow {
        report_index: u64,
        counter: MorphospaceFloat32ReportWindowDeltaCounter,
    },
}
impl CollectFailure {
    fn attach(
        self,
        side: MorphospaceFloat32ReportWindowDeltaSide,
        earlier: MorphospaceFloat32ReportObservationWindow,
        later: MorphospaceFloat32ReportObservationWindow,
    ) -> MorphospaceFloat32ReportWindowDeltaError {
        match self {
            Self::ReportLimit { limit, actual } => {
                MorphospaceFloat32ReportWindowDeltaError::ReportLimit {
                    side,
                    limit,
                    actual,
                    earlier,
                    later,
                }
            }
            Self::RecordLimit {
                report_index,
                limit,
                actual,
            } => MorphospaceFloat32ReportWindowDeltaError::RecordLimit {
                side,
                report_index,
                limit,
                actual,
                earlier,
                later,
            },
            Self::CounterOverflow {
                report_index,
                counter,
            } => MorphospaceFloat32ReportWindowDeltaError::CounterOverflow {
                side,
                report_index,
                counter,
                earlier,
                later,
            },
        }
    }
}

fn collect<A>(
    window: &MorphospaceFloat32ReportObservationWindow,
    bounds: MorphospaceFloat32ReportWindowDeltaBounds,
    add: &mut A,
) -> Result<Collected, CollectFailure>
where
    A: FnMut(u64, u64) -> Result<u64, ()>,
{
    if window.observations().len() > bounds.maximum_reports_per_window {
        return Err(CollectFailure::ReportLimit {
            limit: bounds.maximum_reports_per_window,
            actual: window.observations().len(),
        });
    }
    let mut totals = [0; 11];
    let mut largest = None;
    for (position, observation) in window.observations().iter().enumerate() {
        let report_index =
            u64::try_from(position).map_err(|_| CollectFailure::CounterOverflow {
                report_index: u64::MAX,
                counter: MorphospaceFloat32ReportWindowDeltaCounter::Reports,
            })?;
        if observation.records().len() > bounds.maximum_records_per_report {
            return Err(CollectFailure::RecordLimit {
                report_index,
                limit: bounds.maximum_records_per_report,
                actual: observation.records().len(),
            });
        }
        let health = observation.terminal_health();
        let values = [
            (MorphospaceFloat32ReportWindowDeltaCounter::Reports, 1),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Records,
                u64::try_from(observation.records().len()).map_err(|_| {
                    CollectFailure::CounterOverflow {
                        report_index,
                        counter: MorphospaceFloat32ReportWindowDeltaCounter::Records,
                    }
                })?,
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Observations,
                health.observation_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::First,
                health.first_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Contiguous,
                health.contiguous_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Gaps,
                health.gap_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::ExplicitObservedMissingSequences,
                health.explicit_missing_sequence_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::Duplicates,
                health.duplicate_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::OutOfOrder,
                health.out_of_order_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::RetainedUnchanged,
                health.retained_unchanged_count(),
            ),
            (
                MorphospaceFloat32ReportWindowDeltaCounter::RetainedChanged,
                health.retained_changed_count(),
            ),
        ];
        for (index, (counter, value)) in values.into_iter().enumerate() {
            totals[index] =
                add(totals[index], value).map_err(|_| CollectFailure::CounterOverflow {
                    report_index,
                    counter,
                })?;
        }
        for record in observation.records() {
            let adjustment = record.processed().facts().adjustment();
            let candidate = MorphospaceFloat32ReportWindowAdjustmentEvidence {
                report_index,
                record_index: record.index(),
                sequence: record.sequence(),
                signed_bits: adjustment.to_bits(),
                absolute_bits: adjustment.abs().to_bits(),
            };
            if largest
                .map(
                    |current: MorphospaceFloat32ReportWindowAdjustmentEvidence| {
                        adjustment.abs() > f64::from_bits(current.absolute_bits)
                    },
                )
                .unwrap_or(true)
            {
                largest = Some(candidate);
            }
        }
    }
    Ok(Collected {
        totals,
        largest_adjustment: largest,
    })
}

fn totals_values(value: MorphospaceFloat32ReportObservationWindowTotals) -> [u64; 11] {
    [
        value.report_count(),
        value.record_count(),
        value.observation_count(),
        value.first_count(),
        value.contiguous_count(),
        value.gap_count(),
        value.explicit_missing_sequence_count(),
        value.duplicate_count(),
        value.out_of_order_count(),
        value.retained_unchanged_count(),
        value.retained_changed_count(),
    ]
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
            .collect::<Vec<_>>();
        MorphospaceFloat32ReportObservationOwner::new(records.len())
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
        let records = observations
            .iter()
            .map(|value| value.records().len())
            .sum::<usize>();
        observations.into_iter().fold(
            MorphospaceFloat32ReportObservationWindow::new(8, records.max(1)).unwrap(),
            |window, observation| window.append(observation).unwrap(),
        )
    }

    fn pointers(window: &MorphospaceFloat32ReportObservationWindow) -> Vec<*const f32> {
        window
            .observations()
            .iter()
            .flat_map(|value| value.records())
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect()
    }

    fn owner(evidence: usize) -> MorphospaceFloat32ReportWindowDeltaProposalOwner {
        MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
            MorphospaceFloat32ReportWindowDeltaBounds::new(8, 8, evidence).unwrap(),
        )
    }

    fn directions(
        proposal: &MorphospaceFloat32ReportWindowDeltaProposal,
    ) -> Vec<MorphospaceFloat32ReportWindowDeltaDirection> {
        proposal
            .evidence()
            .iter()
            .map(|value| match value {
                MorphospaceFloat32ReportWindowDeltaEvidence::Counter { direction, .. }
                | MorphospaceFloat32ReportWindowDeltaEvidence::LargestAbsoluteAdjustment {
                    direction,
                    ..
                } => *direction,
            })
            .collect()
    }

    #[test]
    fn bounds_cover_zero_and_platform_extremes() {
        use MorphospaceFloat32ReportWindowDeltaBoundsError::*;
        assert_eq!(
            MorphospaceFloat32ReportWindowDeltaBounds::new(0, 1, 1),
            Err(ZeroMaximumReportsPerWindow)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowDeltaBounds::new(1, 0, 1),
            Err(ZeroMaximumRecordsPerReport)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowDeltaBounds::new(1, 1, 0),
            Err(ZeroMaximumEvidence)
        );
        assert!(
            MorphospaceFloat32ReportWindowDeltaBounds::new(usize::MAX, usize::MAX, usize::MAX)
                .is_ok()
        );
    }

    #[test]
    fn equality_direction_order_ties_sequence_extremes_and_allocations_are_exact() {
        let build = |sequences| window(vec![observation(sequences, vec![4.0, 2.0])]);
        let earlier = build(vec![u64::MAX, 0]);
        let later = build(vec![u64::MAX, 0]);
        let first_ptrs = pointers(&earlier);
        let second_ptrs = pointers(&later);
        let proposal = owner(12).propose(earlier, later).unwrap();
        assert_eq!(
            directions(&proposal),
            vec![MorphospaceFloat32ReportWindowDeltaDirection::Unchanged; 12]
        );
        let expected = [
            MorphospaceFloat32ReportWindowDeltaCounter::Reports,
            MorphospaceFloat32ReportWindowDeltaCounter::Records,
            MorphospaceFloat32ReportWindowDeltaCounter::Observations,
            MorphospaceFloat32ReportWindowDeltaCounter::First,
            MorphospaceFloat32ReportWindowDeltaCounter::Contiguous,
            MorphospaceFloat32ReportWindowDeltaCounter::Gaps,
            MorphospaceFloat32ReportWindowDeltaCounter::ExplicitObservedMissingSequences,
            MorphospaceFloat32ReportWindowDeltaCounter::Duplicates,
            MorphospaceFloat32ReportWindowDeltaCounter::OutOfOrder,
            MorphospaceFloat32ReportWindowDeltaCounter::RetainedUnchanged,
            MorphospaceFloat32ReportWindowDeltaCounter::RetainedChanged,
        ];
        for (actual, counter) in proposal.evidence()[..11].iter().zip(expected) {
            assert!(
                matches!(actual, MorphospaceFloat32ReportWindowDeltaEvidence::Counter { counter: found, absolute_delta: 0, .. } if *found == counter)
            );
        }
        match proposal.evidence()[11] {
            MorphospaceFloat32ReportWindowDeltaEvidence::LargestAbsoluteAdjustment {
                earlier: Some(a),
                later: Some(b),
                ..
            } => {
                assert_eq!((a.report_index, a.record_index, a.sequence), (0, 1, 0));
                assert_eq!((b.report_index, b.record_index, b.sequence), (0, 1, 0));
            }
            _ => panic!("adjustment evidence"),
        }
        let (earlier, later) = proposal.into_windows();
        assert_eq!(pointers(&earlier), first_ptrs);
        assert_eq!(pointers(&later), second_ptrs);
    }

    #[test]
    fn counters_and_adjustments_cover_increased_decreased_and_explicit_missing_only() {
        let small = || window(vec![observation(vec![10], vec![1.0])]);
        let large = || window(vec![observation(vec![10, 12], vec![4.0, 2.0])]);
        let increased = owner(12).propose(small(), large()).unwrap();
        assert!(directions(&increased)
            .contains(&MorphospaceFloat32ReportWindowDeltaDirection::Increased));
        let missing = increased.evidence().iter().find(|value| matches!(value, MorphospaceFloat32ReportWindowDeltaEvidence::Counter { counter: MorphospaceFloat32ReportWindowDeltaCounter::ExplicitObservedMissingSequences, .. })).unwrap();
        assert!(matches!(
            missing,
            MorphospaceFloat32ReportWindowDeltaEvidence::Counter {
                earlier: 0,
                later: 1,
                absolute_delta: 1,
                direction: MorphospaceFloat32ReportWindowDeltaDirection::Increased,
                ..
            }
        ));
        let decreased = owner(12).propose(large(), small()).unwrap();
        assert!(directions(&decreased)
            .contains(&MorphospaceFloat32ReportWindowDeltaDirection::Decreased));
        assert!(matches!(
            decreased.evidence()[11],
            MorphospaceFloat32ReportWindowDeltaEvidence::LargestAbsoluteAdjustment {
                direction: MorphospaceFloat32ReportWindowDeltaDirection::Decreased,
                ..
            }
        ));
    }

    fn assert_returned(
        error: MorphospaceFloat32ReportWindowDeltaError,
        first: Vec<*const f32>,
        second: Vec<*const f32>,
    ) {
        let (a, b) = error.into_windows();
        assert_eq!(pointers(&a), first);
        assert_eq!(pointers(&b), second);
    }

    #[test]
    fn every_error_returns_both_complete_inputs_without_partial_mutation() {
        let make = || {
            (
                window(vec![observation(vec![0], vec![1.0])]),
                window(vec![
                    observation(vec![0], vec![1.0]),
                    observation(vec![1], vec![2.0]),
                ]),
            )
        };
        let run = |kind: u8| {
            let (a, b) = make();
            let pa = pointers(&a);
            let pb = pointers(&b);
            let error = match kind {
                0 => MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
                    MorphospaceFloat32ReportWindowDeltaBounds::new(1, 8, 12).unwrap(),
                )
                .propose(a, b)
                .unwrap_err(),
                1 => {
                    let wide = window(vec![observation(vec![0, 1], vec![2.0, 1.0])]);
                    let wide_pointers = pointers(&wide);
                    let error = MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
                        MorphospaceFloat32ReportWindowDeltaBounds::new(8, 1, 12).unwrap(),
                    )
                    .propose(a, wide)
                    .unwrap_err();
                    assert_returned(error, pa, wide_pointers);
                    return;
                }
                2 => owner(11).propose(a, b).unwrap_err(),
                3 => owner(12)
                    .propose_with(a, b, |_, _| Err(()), |x, y| x.checked_add(y).ok_or(()))
                    .unwrap_err(),
                4 => owner(12)
                    .propose_with(a, b, |_, _| Ok(()), |_, _| Err(()))
                    .unwrap_err(),
                _ => owner(12)
                    .propose_with(
                        a,
                        b,
                        |_, _| Ok(()),
                        |x, y| Ok(x.saturating_add(y).saturating_add(1)),
                    )
                    .unwrap_err(),
            };
            assert_returned(error, pa, pb);
        };
        for kind in 0..6 {
            run(kind);
        }
    }

    #[test]
    fn empty_windows_are_valid_zero_evidence_and_owner_is_private_inert() {
        let empty = || MorphospaceFloat32ReportObservationWindow::new(1, 1).unwrap();
        let proposal = owner(12).propose(empty(), empty()).unwrap();
        assert_eq!(
            directions(&proposal),
            vec![MorphospaceFloat32ReportWindowDeltaDirection::Unchanged; 12]
        );
        assert!(matches!(
            proposal.evidence()[11],
            MorphospaceFloat32ReportWindowDeltaEvidence::LargestAbsoluteAdjustment {
                earlier: None,
                later: None,
                ..
            }
        ));
        let source = include_str!("morphospace_float32_report_window_delta_proposal.rs");
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(!include_str!("runtime.rs").contains("MorphospaceFloat32ReportWindowDelta"));
    }
}
