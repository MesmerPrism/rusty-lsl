// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded transactional history of complete actual P42 retained advisory summaries.
//!
//! This crate-private, default-inert owner retains caller-supplied summaries in
//! insertion order without reconstructing their evidence. It does not infer
//! loss or continuity, apply advice, activate work, or grant Manifold, session,
//! stream, transport, control, or application authority, and it makes no claim
//! of liblsl equivalence.

use crate::morphospace_float32_retained_advisory_summary::MorphospaceFloat32RetainedAdvisorySummary;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedAdvisorySummaryHistoryConfigError {
    ZeroMaximumSummaries,
    MaximumSummariesUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedAdvisorySummaryHistoryTotals {
    summary_count: u64,
    retained_evidence_count: u64,
    history_snapshot_count: u64,
    history_evidence_count: u64,
    summary_fact_count: u64,
}

impl MorphospaceFloat32RetainedAdvisorySummaryHistoryTotals {
    pub(crate) const fn summary_count(&self) -> u64 {
        self.summary_count
    }

    pub(crate) const fn retained_evidence_count(&self) -> u64 {
        self.retained_evidence_count
    }

    pub(crate) const fn history_snapshot_count(&self) -> u64 {
        self.history_snapshot_count
    }

    pub(crate) const fn history_evidence_count(&self) -> u64 {
        self.history_evidence_count
    }

    pub(crate) const fn summary_fact_count(&self) -> u64 {
        self.summary_fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedAdvisorySummaryHistory {
    maximum_summaries: usize,
    maximum_summaries_u64: u64,
    summaries: Vec<MorphospaceFloat32RetainedAdvisorySummary>,
    totals: MorphospaceFloat32RetainedAdvisorySummaryHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError {
    CollectionLengthOverflow {
        history: MorphospaceFloat32RetainedAdvisorySummaryHistory,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
    },
    HistoryLimit {
        limit: usize,
        history: MorphospaceFloat32RetainedAdvisorySummaryHistory,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
    },
    SummaryCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32RetainedAdvisorySummaryHistory,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
    },
    CounterOverflow {
        history: MorphospaceFloat32RetainedAdvisorySummaryHistory,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
    },
    Allocation {
        requested_summaries: usize,
        history: MorphospaceFloat32RetainedAdvisorySummaryHistory,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
    },
}

impl MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32RetainedAdvisorySummaryHistory,
        MorphospaceFloat32RetainedAdvisorySummary,
    ) {
        use MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, candidate }
            | HistoryLimit {
                history, candidate, ..
            }
            | SummaryCountUnrepresentable {
                history, candidate, ..
            }
            | CounterOverflow { history, candidate }
            | Allocation {
                history, candidate, ..
            } => (history, candidate),
        }
    }
}

impl MorphospaceFloat32RetainedAdvisorySummaryHistory {
    pub(crate) fn new(
        maximum_summaries: usize,
    ) -> Result<Self, MorphospaceFloat32RetainedAdvisorySummaryHistoryConfigError> {
        if maximum_summaries == 0 {
            return Err(
                MorphospaceFloat32RetainedAdvisorySummaryHistoryConfigError::ZeroMaximumSummaries,
            );
        }
        let maximum_summaries_u64 = u64::try_from(maximum_summaries).map_err(|_| {
            MorphospaceFloat32RetainedAdvisorySummaryHistoryConfigError::MaximumSummariesUnrepresentable {
                requested: maximum_summaries,
            }
        })?;
        Ok(Self {
            maximum_summaries,
            maximum_summaries_u64,
            summaries: Vec::new(),
            totals: Default::default(),
        })
    }

    pub(crate) const fn maximum_summaries(&self) -> usize {
        self.maximum_summaries
    }

    pub(crate) fn summaries(&self) -> &[MorphospaceFloat32RetainedAdvisorySummary] {
        &self.summaries
    }

    pub(crate) const fn totals(&self) -> MorphospaceFloat32RetainedAdvisorySummaryHistoryTotals {
        self.totals
    }

    pub(crate) fn into_summaries(self) -> Vec<MorphospaceFloat32RetainedAdvisorySummary> {
        self.summaries
    }

    pub(crate) fn append(
        self,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
    ) -> Result<Self, MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError> {
        self.append_with(
            candidate,
            |summaries, requested| summaries.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        candidate: MorphospaceFloat32RetainedAdvisorySummary,
        reserve: R,
        mut to_u64: C,
        mut add_u64: U,
        mut add_usize: Z,
    ) -> Result<Self, MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32RetainedAdvisorySummary>, usize) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        U: FnMut(u64, u64) -> Result<u64, ()>,
        Z: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError::*;
        let next_len = match add_usize(self.summaries.len(), 1) {
            Ok(value) => value,
            Err(()) => {
                return Err(CollectionLengthOverflow {
                    history: self,
                    candidate,
                })
            }
        };
        if next_len > self.maximum_summaries {
            return Err(HistoryLimit {
                limit: self.maximum_summaries,
                history: self,
                candidate,
            });
        }
        let next_count = match to_u64(next_len) {
            Ok(value) => value,
            Err(()) => {
                return Err(SummaryCountUnrepresentable {
                    actual: next_len,
                    history: self,
                    candidate,
                })
            }
        };
        if next_count > self.maximum_summaries_u64 {
            return Err(HistoryLimit {
                limit: self.maximum_summaries,
                history: self,
                candidate,
            });
        }

        let candidate_totals = candidate.totals();
        let additions = [
            (self.totals.summary_count, 1),
            (
                self.totals.retained_evidence_count,
                candidate_totals.retained_evidence_count(),
            ),
            (
                self.totals.history_snapshot_count,
                candidate_totals.history_snapshot_count(),
            ),
            (
                self.totals.history_evidence_count,
                candidate_totals.history_evidence_count(),
            ),
            (
                self.totals.summary_fact_count,
                candidate_totals.summary_fact_count(),
            ),
        ];
        let mut next = [0; 5];
        for (index, (left, right)) in additions.into_iter().enumerate() {
            next[index] = match add_u64(left, right) {
                Ok(value) => value,
                Err(()) => {
                    return Err(CounterOverflow {
                        history: self,
                        candidate,
                    })
                }
            };
        }
        if reserve(&mut self.summaries, 1).is_err() {
            return Err(Allocation {
                requested_summaries: 1,
                history: self,
                candidate,
            });
        }
        self.summaries.push(candidate);
        self.totals = MorphospaceFloat32RetainedAdvisorySummaryHistoryTotals {
            summary_count: next[0],
            retained_evidence_count: next[1],
            history_snapshot_count: next[2],
            history_evidence_count: next[3],
            summary_fact_count: next[4],
        };
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_advisory_snapshot_history::MorphospaceFloat32ReportAdvisorySnapshotHistory;
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::morphospace_float32_retained_advisory_summary::{
        MorphospaceFloat32RetainedAdvisorySummaryBounds,
        MorphospaceFloat32RetainedAdvisorySummaryOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessor,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn summary(value: f32) -> MorphospaceFloat32RetainedAdvisorySummary {
        let empty_stability = || {
            MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
                MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
            )
            .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
            .unwrap()
        };
        let snapshot = || {
            MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
                MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 1).unwrap(),
            )
            .snapshot(
                MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap(),
                MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap(),
                empty_stability(),
            )
            .unwrap()
        };
        let mut processor = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(4),
        );
        let sample = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(3.0).unwrap(),
            None,
        );
        let retained = CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
        .compose(processor.process_record(42, sample).unwrap(), snapshot())
        .unwrap();
        let history = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(snapshot())
            .unwrap();
        MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 3).unwrap(),
        )
        .summarize(retained, history)
        .unwrap()
    }

    fn pointer(summary: &MorphospaceFloat32RetainedAdvisorySummary) -> *const f32 {
        summary
            .retained()
            .report()
            .sample()
            .sample()
            .values()
            .as_ptr()
    }

    #[test]
    fn actual_p42_summaries_keep_insertion_order_identity_and_exact_totals() {
        let values = [summary(1.0), summary(2.0), summary(1.0)];
        let expected: Vec<_> = values.iter().map(pointer).collect();
        let history = values.into_iter().fold(
            MorphospaceFloat32RetainedAdvisorySummaryHistory::new(3).unwrap(),
            |history, candidate| history.append(candidate).unwrap(),
        );
        assert_eq!(history.maximum_summaries(), 3);
        assert_eq!(
            history.summaries().iter().map(pointer).collect::<Vec<_>>(),
            expected
        );
        assert_eq!(history.totals().summary_count(), 3);
        assert_eq!(history.totals().retained_evidence_count(), 3);
        assert_eq!(history.totals().history_snapshot_count(), 3);
        assert_eq!(history.totals().history_evidence_count(), 0);
        assert_eq!(history.totals().summary_fact_count(), 6);
        assert_eq!(
            history
                .into_summaries()
                .iter()
                .map(pointer)
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn explicit_nonzero_bound_and_capacity_failure_are_transactional() {
        assert_eq!(
            MorphospaceFloat32RetainedAdvisorySummaryHistory::new(0),
            Err(MorphospaceFloat32RetainedAdvisorySummaryHistoryConfigError::ZeroMaximumSummaries)
        );
        let kept = summary(4.0);
        let kept_pointer = pointer(&kept);
        let candidate = summary(5.0);
        let candidate_pointer = pointer(&candidate);
        let history = MorphospaceFloat32RetainedAdvisorySummaryHistory::new(1)
            .unwrap()
            .append(kept)
            .unwrap();
        let before = history.totals();
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32RetainedAdvisorySummaryHistoryAppendError::HistoryLimit {
                limit: 1,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(pointer(&history.summaries()[0]), kept_pointer);
        assert_eq!(pointer(&candidate), candidate_pointer);
    }

    #[test]
    fn every_checked_or_fallible_stage_rolls_back_both_owners() {
        for failure in 0..8 {
            let kept = summary(6.0);
            let kept_pointer = pointer(&kept);
            let candidate = summary(7.0);
            let candidate_pointer = pointer(&candidate);
            let history = MorphospaceFloat32RetainedAdvisorySummaryHistory::new(2)
                .unwrap()
                .append(kept)
                .unwrap();
            let before = history.totals();
            let mut additions = 0;
            let error = history
                .append_with(
                    candidate,
                    |_, requested| {
                        if failure == 7 {
                            assert_eq!(requested, 1);
                            Err(())
                        } else {
                            Ok(())
                        }
                    },
                    |value| {
                        if failure == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |left, right| {
                        let current = additions;
                        additions += 1;
                        if failure >= 2 && current == failure - 2 {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                    |left, right| {
                        if failure == 0 {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            let (history, candidate) = error.into_parts();
            assert_eq!(history.totals(), before);
            assert_eq!(history.summaries().len(), 1);
            assert_eq!(pointer(&history.summaries()[0]), kept_pointer);
            assert_eq!(pointer(&candidate), candidate_pointer);
        }
    }

    #[test]
    fn source_is_private_default_inert_non_applying_and_unexported() {
        let source = include_str!("morphospace_float32_retained_advisory_summary_history.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs")
            .contains("MorphospaceFloat32RetainedAdvisorySummaryHistory"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod morphospace_float32_retained_advisory_summary_history"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_retained_advisory_summary_history"));
    }
}
