// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded deterministic stability advice over an owned P38 observation history.
//!
//! This crate-private owner consumes and always returns the complete history. It
//! compares only adjacent exact P37 windows, never infers continuity or loss,
//! and has no application, acceptance, routing, authorization, or activation
//! authority.

use crate::morphospace_float32_report_observation_history::{
    MorphospaceFloat32ReportObservationHistory, MorphospaceFloat32ReportObservationHistoryTotals,
};
use crate::morphospace_float32_report_observation_window::{
    MorphospaceFloat32ReportObservationWindow, MorphospaceFloat32ReportObservationWindowTotals,
};

const COUNTER_COUNT: usize = 11;
const EVIDENCE_PER_TRANSITION: usize = COUNTER_COUNT + 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityConfigError {
    ZeroMaximumWindows,
    ZeroMaximumReportsPerWindow,
    ZeroMaximumRecordsPerReport,
    ZeroMaximumEvidence,
    MaximumWindowsUnrepresentable { requested: usize },
    MaximumReportsPerWindowUnrepresentable { requested: usize },
    MaximumRecordsPerReportUnrepresentable { requested: usize },
    MaximumEvidenceUnrepresentable { requested: usize },
    InvalidMaximumAdjustmentChange { bits: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowStabilityBounds {
    maximum_windows: usize,
    maximum_reports_per_window: usize,
    maximum_records_per_report: usize,
    maximum_evidence: usize,
    maximum_counter_change: u64,
    maximum_adjustment_change: f64,
}

impl MorphospaceFloat32ReportWindowStabilityBounds {
    pub(crate) fn new(
        maximum_windows: usize,
        maximum_reports_per_window: usize,
        maximum_records_per_report: usize,
        maximum_evidence: usize,
        maximum_counter_change: u64,
        maximum_adjustment_change: f64,
    ) -> Result<Self, MorphospaceFloat32ReportWindowStabilityConfigError> {
        use MorphospaceFloat32ReportWindowStabilityConfigError::*;
        if maximum_windows == 0 {
            return Err(ZeroMaximumWindows);
        }
        if maximum_reports_per_window == 0 {
            return Err(ZeroMaximumReportsPerWindow);
        }
        if maximum_records_per_report == 0 {
            return Err(ZeroMaximumRecordsPerReport);
        }
        if maximum_evidence == 0 {
            return Err(ZeroMaximumEvidence);
        }
        u64::try_from(maximum_windows).map_err(|_| MaximumWindowsUnrepresentable {
            requested: maximum_windows,
        })?;
        u64::try_from(maximum_reports_per_window).map_err(|_| {
            MaximumReportsPerWindowUnrepresentable {
                requested: maximum_reports_per_window,
            }
        })?;
        u64::try_from(maximum_records_per_report).map_err(|_| {
            MaximumRecordsPerReportUnrepresentable {
                requested: maximum_records_per_report,
            }
        })?;
        u64::try_from(maximum_evidence).map_err(|_| MaximumEvidenceUnrepresentable {
            requested: maximum_evidence,
        })?;
        if !maximum_adjustment_change.is_finite() || maximum_adjustment_change < 0.0 {
            return Err(InvalidMaximumAdjustmentChange {
                bits: maximum_adjustment_change.to_bits(),
            });
        }
        Ok(Self {
            maximum_windows,
            maximum_reports_per_window,
            maximum_records_per_report,
            maximum_evidence,
            maximum_counter_change,
            maximum_adjustment_change,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityDirection {
    Equal,
    Increase,
    Decrease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityAssessment {
    WithinThreshold,
    ExceedsThreshold,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityCounter {
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
pub(crate) struct MorphospaceFloat32ReportWindowStabilityAdjustment {
    pub(crate) report_index: u64,
    pub(crate) record_index: u64,
    pub(crate) sequence: u64,
    pub(crate) signed_bits: u64,
    pub(crate) absolute_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityEvidence {
    Counter {
        earlier_window_index: u64,
        later_window_index: u64,
        counter: MorphospaceFloat32ReportWindowStabilityCounter,
        direction: MorphospaceFloat32ReportWindowStabilityDirection,
        assessment: MorphospaceFloat32ReportWindowStabilityAssessment,
        earlier: u64,
        later: u64,
        absolute_change: u64,
    },
    LargestAbsoluteAdjustment {
        earlier_window_index: u64,
        later_window_index: u64,
        direction: MorphospaceFloat32ReportWindowStabilityDirection,
        assessment: MorphospaceFloat32ReportWindowStabilityAssessment,
        earlier: Option<MorphospaceFloat32ReportWindowStabilityAdjustment>,
        later: Option<MorphospaceFloat32ReportWindowStabilityAdjustment>,
        absolute_change_bits: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowStabilityProposal {
    history: MorphospaceFloat32ReportObservationHistory,
    stable: bool,
    evidence: Vec<MorphospaceFloat32ReportWindowStabilityEvidence>,
}

impl MorphospaceFloat32ReportWindowStabilityProposal {
    pub(crate) const fn is_stable(&self) -> bool {
        self.stable
    }
    pub(crate) fn evidence(&self) -> &[MorphospaceFloat32ReportWindowStabilityEvidence] {
        &self.evidence
    }
    pub(crate) fn into_history(self) -> MorphospaceFloat32ReportObservationHistory {
        self.history
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityError {
    HistoryLimit {
        limit: usize,
        actual: usize,
        history: MorphospaceFloat32ReportObservationHistory,
    },
    WindowReportLimit {
        window_index: u64,
        limit: usize,
        actual: usize,
        history: MorphospaceFloat32ReportObservationHistory,
    },
    ReportRecordLimit {
        window_index: u64,
        report_index: u64,
        limit: usize,
        actual: usize,
        history: MorphospaceFloat32ReportObservationHistory,
    },
    IndexUnrepresentable {
        history: MorphospaceFloat32ReportObservationHistory,
    },
    CounterOverflow {
        window_index: u64,
        counter: MorphospaceFloat32ReportWindowStabilityCounter,
        history: MorphospaceFloat32ReportObservationHistory,
    },
    HistoryTotalsMismatch {
        history: MorphospaceFloat32ReportObservationHistory,
    },
    WindowTotalsMismatch {
        window_index: u64,
        history: MorphospaceFloat32ReportObservationHistory,
    },
    EvidenceCountOverflow {
        history: MorphospaceFloat32ReportObservationHistory,
    },
    EvidenceLimit {
        limit: usize,
        required: usize,
        history: MorphospaceFloat32ReportObservationHistory,
    },
    Allocation {
        requested: usize,
        history: MorphospaceFloat32ReportObservationHistory,
    },
}

impl MorphospaceFloat32ReportWindowStabilityError {
    pub(crate) fn into_history(self) -> MorphospaceFloat32ReportObservationHistory {
        match self {
            Self::HistoryLimit { history, .. }
            | Self::WindowReportLimit { history, .. }
            | Self::ReportRecordLimit { history, .. }
            | Self::IndexUnrepresentable { history }
            | Self::CounterOverflow { history, .. }
            | Self::HistoryTotalsMismatch { history }
            | Self::WindowTotalsMismatch { history, .. }
            | Self::EvidenceCountOverflow { history }
            | Self::EvidenceLimit { history, .. }
            | Self::Allocation { history, .. } => history,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowStabilityProposalOwner {
    bounds: MorphospaceFloat32ReportWindowStabilityBounds,
}

impl MorphospaceFloat32ReportWindowStabilityProposalOwner {
    pub(crate) const fn new(bounds: MorphospaceFloat32ReportWindowStabilityBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn propose(
        &self,
        history: MorphospaceFloat32ReportObservationHistory,
    ) -> Result<
        MorphospaceFloat32ReportWindowStabilityProposal,
        MorphospaceFloat32ReportWindowStabilityError,
    > {
        self.propose_with(
            history,
            |evidence, requested| evidence.try_reserve_exact(requested).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
            |a, b| a.checked_mul(b).ok_or(()),
        )
    }

    fn propose_with<R, A, M>(
        &self,
        history: MorphospaceFloat32ReportObservationHistory,
        reserve: R,
        mut add: A,
        multiply: M,
    ) -> Result<
        MorphospaceFloat32ReportWindowStabilityProposal,
        MorphospaceFloat32ReportWindowStabilityError,
    >
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ReportWindowStabilityEvidence>,
            usize,
        ) -> Result<(), ()>,
        A: FnMut(u64, u64) -> Result<u64, ()>,
        M: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        let windows = history.windows();
        if windows.len() > self.bounds.maximum_windows {
            return Err(MorphospaceFloat32ReportWindowStabilityError::HistoryLimit {
                limit: self.bounds.maximum_windows,
                actual: windows.len(),
                history,
            });
        }
        let mut collected = Vec::new();
        if collected.try_reserve_exact(windows.len()).is_err() {
            return Err(MorphospaceFloat32ReportWindowStabilityError::Allocation {
                requested: windows.len(),
                history,
            });
        }
        let mut history_totals = [0u64; 3];
        for (wi, window) in windows.iter().enumerate() {
            let window_index = match u64::try_from(wi) {
                Ok(v) => v,
                Err(_) => {
                    return Err(
                        MorphospaceFloat32ReportWindowStabilityError::IndexUnrepresentable {
                            history,
                        },
                    )
                }
            };
            match collect_window(window, window_index, self.bounds, &mut add) {
                Ok(value) => {
                    if value.totals != totals_values(window.totals()) {
                        return Err(
                            MorphospaceFloat32ReportWindowStabilityError::WindowTotalsMismatch {
                                window_index,
                                history,
                            },
                        );
                    }
                    let additions = [1, value.totals[0], value.totals[1]];
                    for index in 0..3 {
                        history_totals[index] =
                            match add(history_totals[index], additions[index]) {
                                Ok(v) => v,
                                Err(_) => return Err(
                                    MorphospaceFloat32ReportWindowStabilityError::CounterOverflow {
                                        window_index,
                                        counter: if index == 1 {
                                            MorphospaceFloat32ReportWindowStabilityCounter::Reports
                                        } else {
                                            MorphospaceFloat32ReportWindowStabilityCounter::Records
                                        },
                                        history,
                                    },
                                ),
                            };
                    }
                    collected.push(value);
                }
                Err(failure) => return Err(failure.attach(history)),
            }
        }
        if history_totals != history_totals_values(history.totals()) {
            return Err(
                MorphospaceFloat32ReportWindowStabilityError::HistoryTotalsMismatch { history },
            );
        }
        let transitions = windows.len().saturating_sub(1);
        let required = match multiply(transitions, EVIDENCE_PER_TRANSITION) {
            Ok(v) => v,
            Err(_) => {
                return Err(
                    MorphospaceFloat32ReportWindowStabilityError::EvidenceCountOverflow { history },
                )
            }
        };
        if required > self.bounds.maximum_evidence {
            return Err(
                MorphospaceFloat32ReportWindowStabilityError::EvidenceLimit {
                    limit: self.bounds.maximum_evidence,
                    required,
                    history,
                },
            );
        }
        let mut evidence = Vec::new();
        if reserve(&mut evidence, required).is_err() {
            return Err(MorphospaceFloat32ReportWindowStabilityError::Allocation {
                requested: required,
                history,
            });
        }
        let mut stable = true;
        for pair in 0..transitions {
            let earlier_window_index = u64::try_from(pair).expect("validated window index");
            let later_window_index = earlier_window_index.checked_add(1).expect("bounded index");
            let earlier = collected[pair];
            let later = collected[pair + 1];
            for (index, counter) in COUNTERS.into_iter().enumerate() {
                let (direction, absolute_change) =
                    direction_and_change(earlier.totals[index], later.totals[index]);
                let assessment = assessment(absolute_change <= self.bounds.maximum_counter_change);
                stable &= assessment
                    == MorphospaceFloat32ReportWindowStabilityAssessment::WithinThreshold;
                evidence.push(MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                    earlier_window_index,
                    later_window_index,
                    counter,
                    direction,
                    assessment,
                    earlier: earlier.totals[index],
                    later: later.totals[index],
                    absolute_change,
                });
            }
            let (direction, absolute_change) =
                adjustment_change(earlier.largest_adjustment, later.largest_adjustment);
            let adjustment_assessment =
                assessment(absolute_change <= self.bounds.maximum_adjustment_change);
            stable &= adjustment_assessment
                == MorphospaceFloat32ReportWindowStabilityAssessment::WithinThreshold;
            evidence.push(
                MorphospaceFloat32ReportWindowStabilityEvidence::LargestAbsoluteAdjustment {
                    earlier_window_index,
                    later_window_index,
                    direction,
                    assessment: adjustment_assessment,
                    earlier: earlier.largest_adjustment,
                    later: later.largest_adjustment,
                    absolute_change_bits: absolute_change.to_bits(),
                },
            );
        }
        Ok(MorphospaceFloat32ReportWindowStabilityProposal {
            history,
            stable,
            evidence,
        })
    }
}

const COUNTERS: [MorphospaceFloat32ReportWindowStabilityCounter; COUNTER_COUNT] = [
    MorphospaceFloat32ReportWindowStabilityCounter::Reports,
    MorphospaceFloat32ReportWindowStabilityCounter::Records,
    MorphospaceFloat32ReportWindowStabilityCounter::Observations,
    MorphospaceFloat32ReportWindowStabilityCounter::First,
    MorphospaceFloat32ReportWindowStabilityCounter::Contiguous,
    MorphospaceFloat32ReportWindowStabilityCounter::Gaps,
    MorphospaceFloat32ReportWindowStabilityCounter::ExplicitObservedMissingSequences,
    MorphospaceFloat32ReportWindowStabilityCounter::Duplicates,
    MorphospaceFloat32ReportWindowStabilityCounter::OutOfOrder,
    MorphospaceFloat32ReportWindowStabilityCounter::RetainedUnchanged,
    MorphospaceFloat32ReportWindowStabilityCounter::RetainedChanged,
];

#[derive(Clone, Copy)]
struct Collected {
    totals: [u64; COUNTER_COUNT],
    largest_adjustment: Option<MorphospaceFloat32ReportWindowStabilityAdjustment>,
}
enum CollectFailure {
    WindowReportLimit {
        window_index: u64,
        limit: usize,
        actual: usize,
    },
    ReportRecordLimit {
        window_index: u64,
        report_index: u64,
        limit: usize,
        actual: usize,
    },
    IndexUnrepresentable,
    CounterOverflow {
        window_index: u64,
        counter: MorphospaceFloat32ReportWindowStabilityCounter,
    },
}
impl CollectFailure {
    fn attach(
        self,
        history: MorphospaceFloat32ReportObservationHistory,
    ) -> MorphospaceFloat32ReportWindowStabilityError {
        match self {
            Self::WindowReportLimit {
                window_index,
                limit,
                actual,
            } => MorphospaceFloat32ReportWindowStabilityError::WindowReportLimit {
                window_index,
                limit,
                actual,
                history,
            },
            Self::ReportRecordLimit {
                window_index,
                report_index,
                limit,
                actual,
            } => MorphospaceFloat32ReportWindowStabilityError::ReportRecordLimit {
                window_index,
                report_index,
                limit,
                actual,
                history,
            },
            Self::IndexUnrepresentable => {
                MorphospaceFloat32ReportWindowStabilityError::IndexUnrepresentable { history }
            }
            Self::CounterOverflow {
                window_index,
                counter,
            } => MorphospaceFloat32ReportWindowStabilityError::CounterOverflow {
                window_index,
                counter,
                history,
            },
        }
    }
}

fn collect_window<A>(
    window: &MorphospaceFloat32ReportObservationWindow,
    window_index: u64,
    bounds: MorphospaceFloat32ReportWindowStabilityBounds,
    add: &mut A,
) -> Result<Collected, CollectFailure>
where
    A: FnMut(u64, u64) -> Result<u64, ()>,
{
    if window.observations().len() > bounds.maximum_reports_per_window {
        return Err(CollectFailure::WindowReportLimit {
            window_index,
            limit: bounds.maximum_reports_per_window,
            actual: window.observations().len(),
        });
    }
    let mut totals = [0u64; COUNTER_COUNT];
    let mut largest = None;
    for (ri, observation) in window.observations().iter().enumerate() {
        let report_index = u64::try_from(ri).map_err(|_| CollectFailure::IndexUnrepresentable)?;
        if observation.records().len() > bounds.maximum_records_per_report {
            return Err(CollectFailure::ReportRecordLimit {
                window_index,
                report_index,
                limit: bounds.maximum_records_per_report,
                actual: observation.records().len(),
            });
        }
        let health = observation.terminal_health();
        let values = [
            1,
            u64::try_from(observation.records().len())
                .map_err(|_| CollectFailure::IndexUnrepresentable)?,
            health.observation_count(),
            health.first_count(),
            health.contiguous_count(),
            health.gap_count(),
            health.explicit_missing_sequence_count(),
            health.duplicate_count(),
            health.out_of_order_count(),
            health.retained_unchanged_count(),
            health.retained_changed_count(),
        ];
        for index in 0..COUNTER_COUNT {
            totals[index] =
                add(totals[index], values[index]).map_err(|_| CollectFailure::CounterOverflow {
                    window_index,
                    counter: COUNTERS[index],
                })?;
        }
        for record in observation.records() {
            let adjustment = record.processed().facts().adjustment();
            let candidate = MorphospaceFloat32ReportWindowStabilityAdjustment {
                report_index,
                record_index: record.index(),
                sequence: record.sequence(),
                signed_bits: adjustment.to_bits(),
                absolute_bits: adjustment.abs().to_bits(),
            };
            if largest
                .map(
                    |current: MorphospaceFloat32ReportWindowStabilityAdjustment| {
                        adjustment
                            .abs()
                            .total_cmp(&f64::from_bits(current.absolute_bits))
                            .is_gt()
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

fn totals_values(v: MorphospaceFloat32ReportObservationWindowTotals) -> [u64; COUNTER_COUNT] {
    [
        v.report_count(),
        v.record_count(),
        v.observation_count(),
        v.first_count(),
        v.contiguous_count(),
        v.gap_count(),
        v.explicit_missing_sequence_count(),
        v.duplicate_count(),
        v.out_of_order_count(),
        v.retained_unchanged_count(),
        v.retained_changed_count(),
    ]
}
fn history_totals_values(v: MorphospaceFloat32ReportObservationHistoryTotals) -> [u64; 3] {
    [v.window_count(), v.report_count(), v.record_count()]
}
fn direction_and_change(a: u64, b: u64) -> (MorphospaceFloat32ReportWindowStabilityDirection, u64) {
    match b.cmp(&a) {
        std::cmp::Ordering::Equal => (MorphospaceFloat32ReportWindowStabilityDirection::Equal, 0),
        std::cmp::Ordering::Greater => (
            MorphospaceFloat32ReportWindowStabilityDirection::Increase,
            b.checked_sub(a).expect("ordered"),
        ),
        std::cmp::Ordering::Less => (
            MorphospaceFloat32ReportWindowStabilityDirection::Decrease,
            a.checked_sub(b).expect("ordered"),
        ),
    }
}
fn adjustment_change(
    a: Option<MorphospaceFloat32ReportWindowStabilityAdjustment>,
    b: Option<MorphospaceFloat32ReportWindowStabilityAdjustment>,
) -> (MorphospaceFloat32ReportWindowStabilityDirection, f64) {
    let av = a.map(|v| f64::from_bits(v.absolute_bits)).unwrap_or(0.0);
    let bv = b.map(|v| f64::from_bits(v.absolute_bits)).unwrap_or(0.0);
    match bv.total_cmp(&av) {
        std::cmp::Ordering::Equal => (MorphospaceFloat32ReportWindowStabilityDirection::Equal, 0.0),
        std::cmp::Ordering::Greater => (
            MorphospaceFloat32ReportWindowStabilityDirection::Increase,
            bv - av,
        ),
        std::cmp::Ordering::Less => (
            MorphospaceFloat32ReportWindowStabilityDirection::Decrease,
            av - bv,
        ),
    }
}
const fn assessment(within: bool) -> MorphospaceFloat32ReportWindowStabilityAssessment {
    if within {
        MorphospaceFloat32ReportWindowStabilityAssessment::WithinThreshold
    } else {
        MorphospaceFloat32ReportWindowStabilityAssessment::ExceedsThreshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_observation::{
        tests::outcome_with, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
    use crate::morphospace_float32_report_window_delta_proposal::{
        MorphospaceFloat32ReportWindowDeltaBounds, MorphospaceFloat32ReportWindowDeltaProposalOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn window(
        sequences: Vec<u64>,
        timestamps: Vec<f64>,
    ) -> MorphospaceFloat32ReportObservationWindow {
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
        let observation = MorphospaceFloat32ReportObservationOwner::new(records.len())
            .unwrap()
            .observe(outcome_with(
                sequences,
                records,
                RequestedTimestampPostProcessing::Monotonic(
                    RequestedTimestampPostProcessingConfig::new(32, 1.0, f64::MAX).unwrap(),
                ),
            ))
            .unwrap();
        MorphospaceFloat32ReportObservationWindow::new(1, observation.records().len())
            .unwrap()
            .append(observation)
            .unwrap()
    }

    fn history(
        windows: Vec<MorphospaceFloat32ReportObservationWindow>,
    ) -> MorphospaceFloat32ReportObservationHistory {
        windows.into_iter().fold(
            MorphospaceFloat32ReportObservationHistory::new(8, 1).unwrap(),
            |history, window| history.append(window).unwrap(),
        )
    }

    fn pointers(history: &MorphospaceFloat32ReportObservationHistory) -> Vec<*const f32> {
        history
            .windows()
            .iter()
            .flat_map(|w| w.observations())
            .flat_map(|o| o.records())
            .map(|r| r.processed().sample().sample().values().as_ptr())
            .collect()
    }

    fn owner(
        counter: u64,
        adjustment: f64,
        evidence: usize,
    ) -> MorphospaceFloat32ReportWindowStabilityProposalOwner {
        MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(
                8, 1, 8, evidence, counter, adjustment,
            )
            .unwrap(),
        )
    }

    fn through_delta_history(
        pairs: [(
            MorphospaceFloat32ReportObservationWindow,
            MorphospaceFloat32ReportObservationWindow,
        ); 2],
    ) -> MorphospaceFloat32ReportObservationHistory {
        let deltas = pairs.map(|(earlier, later)| {
            MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
                MorphospaceFloat32ReportWindowDeltaBounds::new(1, 8, 12).unwrap(),
            )
            .propose(earlier, later)
            .unwrap()
        });
        let delta_history = deltas.into_iter().fold(
            MorphospaceFloat32ReportWindowDeltaHistory::new(2, 12).unwrap(),
            |history, delta| history.append(delta).unwrap(),
        );
        assert_eq!(delta_history.totals().delta_count(), 2);
        assert_eq!(delta_history.totals().window_count(), 4);
        assert_eq!(delta_history.totals().evidence_count(), 24);
        delta_history
            .into_deltas()
            .into_iter()
            .flat_map(|delta| {
                let (earlier, later) = delta.into_windows();
                [earlier, later]
            })
            .fold(
                MorphospaceFloat32ReportObservationHistory::new(4, 1).unwrap(),
                |history, window| history.append(window).unwrap(),
            )
    }

    #[test]
    fn empty_history_zero_bounds_and_capacity_are_explicit() {
        use MorphospaceFloat32ReportWindowStabilityConfigError::*;
        assert_eq!(
            MorphospaceFloat32ReportWindowStabilityBounds::new(0, 1, 1, 1, 0, 0.0),
            Err(ZeroMaximumWindows)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 0, 1, 1, 0, 0.0),
            Err(ZeroMaximumReportsPerWindow)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 0, 1, 0, 0.0),
            Err(ZeroMaximumRecordsPerReport)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 0, 0, 0.0),
            Err(ZeroMaximumEvidence)
        );
        assert!(matches!(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, f64::NAN),
            Err(InvalidMaximumAdjustmentChange { .. })
        ));
        let empty = MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap();
        let proposal = owner(0, 0.0, 1).propose(empty).unwrap();
        assert!(proposal.is_stable());
        assert!(proposal.evidence().is_empty());
        assert!(proposal.into_history().windows().is_empty());
        let h = history(vec![window(vec![0], vec![1.0]), window(vec![0], vec![1.0])]);
        let p = owner(0, 0.0, EVIDENCE_PER_TRANSITION).propose(h).unwrap();
        assert_eq!(p.evidence().len(), EVIDENCE_PER_TRANSITION);
    }

    #[test]
    fn repeated_and_changing_windows_have_stable_direction_reason_and_tie_order() {
        let a = window(vec![10, 11], vec![4.0, 2.0]);
        let b = window(vec![10, 11], vec![4.0, 2.0]);
        let c = window(vec![10, 13], vec![4.0, 8.0]);
        let proposal = owner(0, 0.0, 24).propose(history(vec![a, b, c])).unwrap();
        assert!(!proposal.is_stable());
        assert!(proposal.evidence()[..12].iter().all(|e| match e {
            MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                direction,
                assessment,
                ..
            } =>
                *direction == MorphospaceFloat32ReportWindowStabilityDirection::Equal
                    && *assessment
                        == MorphospaceFloat32ReportWindowStabilityAssessment::WithinThreshold,
            MorphospaceFloat32ReportWindowStabilityEvidence::LargestAbsoluteAdjustment {
                direction,
                ..
            } => *direction == MorphospaceFloat32ReportWindowStabilityDirection::Equal,
        }));
        assert!(matches!(
            proposal.evidence()[12],
            MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                counter: MorphospaceFloat32ReportWindowStabilityCounter::Reports,
                direction: MorphospaceFloat32ReportWindowStabilityDirection::Equal,
                ..
            }
        ));
        let explicit = proposal.evidence().iter().find(|e| matches!(e, MorphospaceFloat32ReportWindowStabilityEvidence::Counter { earlier_window_index: 1, counter: MorphospaceFloat32ReportWindowStabilityCounter::ExplicitObservedMissingSequences, .. })).unwrap();
        assert!(matches!(
            explicit,
            MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                direction: MorphospaceFloat32ReportWindowStabilityDirection::Increase,
                assessment: MorphospaceFloat32ReportWindowStabilityAssessment::ExceedsThreshold,
                ..
            }
        ));
        if let MorphospaceFloat32ReportWindowStabilityEvidence::LargestAbsoluteAdjustment {
            later: Some(later),
            ..
        } = proposal.evidence()[23]
        {
            assert_eq!(later.record_index, 0); // equal magnitudes retain first record
        } else {
            panic!("adjustment evidence");
        }
    }

    #[test]
    fn p38_p39_delta_history_composes_in_order_into_stability_with_exact_identity() {
        let a = window(vec![10, 11], vec![4.0, 2.0]);
        let b = window(vec![10, 11], vec![4.0, 2.0]);
        let c = window(vec![10, 13], vec![4.0, 8.0]);
        let d = window(vec![10, 11], vec![4.0, 2.0]);
        let history = through_delta_history([(a, b), (c, d)]);
        let expected_pointers = pointers(&history);
        let proposal = owner(0, 0.0, 36).propose(history).unwrap();
        assert!(!proposal.is_stable());
        assert!(proposal.evidence()[..12].iter().all(|e| match e {
            MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                direction,
                assessment,
                ..
            } =>
                *direction == MorphospaceFloat32ReportWindowStabilityDirection::Equal
                    && *assessment
                        == MorphospaceFloat32ReportWindowStabilityAssessment::WithinThreshold,
            MorphospaceFloat32ReportWindowStabilityEvidence::LargestAbsoluteAdjustment {
                direction,
                ..
            } => *direction == MorphospaceFloat32ReportWindowStabilityDirection::Equal,
        }));
        let missing = |index| match proposal.evidence()[index] {
            MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                counter:
                    MorphospaceFloat32ReportWindowStabilityCounter::ExplicitObservedMissingSequences,
                direction,
                ..
            } => direction,
            _ => panic!("explicit missing-sequence evidence"),
        };
        assert_eq!(
            missing(12 + 6),
            MorphospaceFloat32ReportWindowStabilityDirection::Increase
        );
        assert_eq!(
            missing(24 + 6),
            MorphospaceFloat32ReportWindowStabilityDirection::Decrease
        );
        assert!(matches!(
            proposal.evidence()[23],
            MorphospaceFloat32ReportWindowStabilityEvidence::LargestAbsoluteAdjustment {
                later: Some(MorphospaceFloat32ReportWindowStabilityAdjustment {
                    record_index: 0,
                    ..
                }),
                ..
            }
        ));
        assert_eq!(pointers(&proposal.into_history()), expected_pointers);
    }

    #[test]
    fn decreasing_extreme_sequences_and_threshold_edges_are_exact() {
        let a = window(vec![0, u64::MAX], vec![1.0, 9.0]);
        let b = window(vec![u64::MAX], vec![1.0]);
        let p = owner(u64::MAX, f64::MAX, 12)
            .propose(history(vec![a, b]))
            .unwrap();
        assert!(p.is_stable());
        assert!(matches!(
            p.evidence()[1],
            MorphospaceFloat32ReportWindowStabilityEvidence::Counter {
                counter: MorphospaceFloat32ReportWindowStabilityCounter::Records,
                direction: MorphospaceFloat32ReportWindowStabilityDirection::Decrease,
                absolute_change: 1,
                ..
            }
        ));
    }

    #[test]
    fn p38_p39_every_failure_returns_complete_history_without_partial_mutation() {
        for kind in 0..7 {
            let h = history(vec![window(vec![0], vec![1.0]), window(vec![0], vec![2.0])]);
            let expected = pointers(&h);
            let error = match kind {
                0 => MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
                    MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 8, 12, 0, 0.0)
                        .unwrap(),
                )
                .propose(h)
                .unwrap_err(),
                1 => MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
                    MorphospaceFloat32ReportWindowStabilityBounds::new(
                        8,
                        1,
                        0usize.saturating_add(1),
                        12,
                        0,
                        0.0,
                    )
                    .unwrap(),
                )
                .propose(history(vec![window(vec![0, 1], vec![1.0, 2.0])]))
                .unwrap_err(),
                2 => owner(0, 0.0, 11).propose(h).unwrap_err(),
                3 => owner(0, 0.0, 12)
                    .propose_with(
                        h,
                        |_, _| Err(()),
                        |a, b| a.checked_add(b).ok_or(()),
                        |a, b| a.checked_mul(b).ok_or(()),
                    )
                    .unwrap_err(),
                4 => owner(0, 0.0, 12)
                    .propose_with(
                        h,
                        |_, _| Ok(()),
                        |_, _| Err(()),
                        |a, b| a.checked_mul(b).ok_or(()),
                    )
                    .unwrap_err(),
                5 => owner(0, 0.0, 12)
                    .propose_with(
                        h,
                        |_, _| Ok(()),
                        |a, b| Ok(a.saturating_add(b).saturating_add(1)),
                        |a, b| a.checked_mul(b).ok_or(()),
                    )
                    .unwrap_err(),
                _ => owner(0, 0.0, 12)
                    .propose_with(
                        h,
                        |_, _| Ok(()),
                        |a, b| a.checked_add(b).ok_or(()),
                        |_, _| Err(()),
                    )
                    .unwrap_err(),
            };
            let returned = error.into_history();
            if kind != 1 {
                assert_eq!(pointers(&returned), expected);
            }
        }
        let source = include_str!("morphospace_float32_report_window_stability_proposal.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
            concat!("fn act", "ivate("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs").contains("MorphospaceFloat32ReportWindowStability"));
    }
}
