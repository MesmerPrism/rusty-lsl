// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded summary of retained actual P41 report/advisory evidence.
//!
//! This crate-private, default-inert owner consumes and retains the canonical
//! P41 composition and advisory history unchanged. Its separate summary index
//! contains only exact `Copy` facts already exposed by those inputs. It does
//! not infer loss or continuity and cannot accept or apply advice.

use crate::caller_requested_float32_report_advisory_evidence::{
    CallerRequestedFloat32ReportAdvisoryEvidence, CallerRequestedFloat32ReportAdvisoryEvidenceItem,
};
use crate::morphospace_float32_report_advisory_snapshot::MorphospaceFloat32ReportAdvisorySnapshotEvidence;
use crate::morphospace_float32_report_advisory_snapshot_history::MorphospaceFloat32ReportAdvisorySnapshotHistory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedAdvisorySummaryConfigError {
    ZeroMaximumRetainedEvidence,
    ZeroMaximumHistorySnapshots,
    ZeroMaximumSummaryFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedAdvisorySummaryBounds {
    maximum_retained_evidence: usize,
    maximum_history_snapshots: usize,
    maximum_summary_facts: usize,
}

impl MorphospaceFloat32RetainedAdvisorySummaryBounds {
    pub(crate) fn new(
        maximum_retained_evidence: usize,
        maximum_history_snapshots: usize,
        maximum_summary_facts: usize,
    ) -> Result<Self, MorphospaceFloat32RetainedAdvisorySummaryConfigError> {
        use MorphospaceFloat32RetainedAdvisorySummaryConfigError::*;
        for (value, zero) in [
            (maximum_retained_evidence, ZeroMaximumRetainedEvidence),
            (maximum_history_snapshots, ZeroMaximumHistorySnapshots),
            (maximum_summary_facts, ZeroMaximumSummaryFacts),
        ] {
            if value == 0 {
                return Err(zero);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_retained_evidence,
            maximum_history_snapshots,
            maximum_summary_facts,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedAdvisorySummaryFact {
    RetainedEvidence {
        retained_index: u64,
        evidence: CallerRequestedFloat32ReportAdvisoryEvidenceItem,
    },
    HistorySnapshot {
        snapshot_index: u64,
        evidence_count: u64,
    },
    HistoryEvidence {
        snapshot_index: u64,
        evidence_index: u64,
        evidence: MorphospaceFloat32ReportAdvisorySnapshotEvidence,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedAdvisorySummaryTotals {
    retained_evidence_count: u64,
    history_snapshot_count: u64,
    history_evidence_count: u64,
    summary_fact_count: u64,
}

impl MorphospaceFloat32RetainedAdvisorySummaryTotals {
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
pub(crate) struct MorphospaceFloat32RetainedAdvisorySummary {
    retained: CallerRequestedFloat32ReportAdvisoryEvidence,
    history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    totals: MorphospaceFloat32RetainedAdvisorySummaryTotals,
    facts: Vec<MorphospaceFloat32RetainedAdvisorySummaryFact>,
}

impl MorphospaceFloat32RetainedAdvisorySummary {
    pub(crate) const fn retained(&self) -> &CallerRequestedFloat32ReportAdvisoryEvidence {
        &self.retained
    }
    pub(crate) const fn history(&self) -> &MorphospaceFloat32ReportAdvisorySnapshotHistory {
        &self.history
    }
    pub(crate) const fn totals(&self) -> MorphospaceFloat32RetainedAdvisorySummaryTotals {
        self.totals
    }
    pub(crate) fn facts(&self) -> &[MorphospaceFloat32RetainedAdvisorySummaryFact] {
        &self.facts
    }
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ReportAdvisoryEvidence,
        MorphospaceFloat32ReportAdvisorySnapshotHistory,
    ) {
        (self.retained, self.history)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedAdvisorySummaryError {
    RetainedEvidenceLimit {
        limit: usize,
        actual: usize,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    },
    HistorySnapshotLimit {
        limit: usize,
        actual: usize,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    },
    CountUnrepresentable {
        actual: usize,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    },
    CountOverflow {
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    },
    SummaryFactLimit {
        limit: usize,
        required: usize,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    },
    Allocation {
        requested: usize,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    },
}

impl MorphospaceFloat32RetainedAdvisorySummaryError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ReportAdvisoryEvidence,
        MorphospaceFloat32ReportAdvisorySnapshotHistory,
    ) {
        use MorphospaceFloat32RetainedAdvisorySummaryError::*;
        match self {
            RetainedEvidenceLimit {
                retained, history, ..
            }
            | HistorySnapshotLimit {
                retained, history, ..
            }
            | CountUnrepresentable {
                retained, history, ..
            }
            | CountOverflow { retained, history }
            | SummaryFactLimit {
                retained, history, ..
            }
            | Allocation {
                retained, history, ..
            } => (retained, history),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedAdvisorySummaryOwner {
    bounds: MorphospaceFloat32RetainedAdvisorySummaryBounds,
}

impl MorphospaceFloat32RetainedAdvisorySummaryOwner {
    pub(crate) const fn new(bounds: MorphospaceFloat32RetainedAdvisorySummaryBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn summarize(
        &self,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
    ) -> Result<
        MorphospaceFloat32RetainedAdvisorySummary,
        MorphospaceFloat32RetainedAdvisorySummaryError,
    > {
        self.summarize_with(
            retained,
            history,
            |facts, requested| facts.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn summarize_with<R, C, A>(
        &self,
        retained: CallerRequestedFloat32ReportAdvisoryEvidence,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        reserve: R,
        mut convert: C,
        mut add: A,
    ) -> Result<
        MorphospaceFloat32RetainedAdvisorySummary,
        MorphospaceFloat32RetainedAdvisorySummaryError,
    >
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32RetainedAdvisorySummaryFact>, usize) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32RetainedAdvisorySummaryError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* retained, history }) }; }

        let retained_count = retained.ordered().len();
        if retained_count > self.bounds.maximum_retained_evidence {
            fail!(RetainedEvidenceLimit {
                limit: self.bounds.maximum_retained_evidence,
                actual: retained_count
            });
        }
        let snapshot_count = history.snapshots().len();
        if snapshot_count > self.bounds.maximum_history_snapshots {
            fail!(HistorySnapshotLimit {
                limit: self.bounds.maximum_history_snapshots,
                actual: snapshot_count
            });
        }
        let retained_count_u64 = match convert(retained_count) {
            Ok(v) => v,
            Err(()) => fail!(CountUnrepresentable {
                actual: retained_count
            }),
        };
        let snapshot_count_u64 = match convert(snapshot_count) {
            Ok(v) => v,
            Err(()) => fail!(CountUnrepresentable {
                actual: snapshot_count
            }),
        };

        let mut history_evidence_count = 0usize;
        for snapshot in history.snapshots() {
            let count = snapshot.evidence().len();
            if convert(count).is_err() {
                fail!(CountUnrepresentable { actual: count });
            }
            history_evidence_count = match add(history_evidence_count, count) {
                Ok(v) => v,
                Err(()) => fail!(CountOverflow {}),
            };
        }
        let required = match add(retained_count, snapshot_count) {
            Ok(v) => v,
            Err(()) => fail!(CountOverflow {}),
        };
        let required = match add(required, history_evidence_count) {
            Ok(v) => v,
            Err(()) => fail!(CountOverflow {}),
        };
        if required > self.bounds.maximum_summary_facts {
            fail!(SummaryFactLimit {
                limit: self.bounds.maximum_summary_facts,
                required: required
            });
        }
        let history_evidence_count_u64 = match convert(history_evidence_count) {
            Ok(v) => v,
            Err(()) => fail!(CountUnrepresentable {
                actual: history_evidence_count
            }),
        };
        let required_u64 = match convert(required) {
            Ok(v) => v,
            Err(()) => fail!(CountUnrepresentable { actual: required }),
        };

        for index in 0..retained_count {
            if convert(index).is_err() {
                fail!(CountUnrepresentable { actual: index });
            }
        }
        for (snapshot_index, snapshot) in history.snapshots().iter().enumerate() {
            if convert(snapshot_index).is_err() {
                fail!(CountUnrepresentable {
                    actual: snapshot_index
                });
            }
            for evidence_index in 0..snapshot.evidence().len() {
                if convert(evidence_index).is_err() {
                    fail!(CountUnrepresentable {
                        actual: evidence_index
                    });
                }
            }
        }

        let mut facts = Vec::new();
        if reserve(&mut facts, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        facts.extend(
            retained
                .ordered()
                .iter()
                .copied()
                .enumerate()
                .map(|(index, evidence)| {
                    MorphospaceFloat32RetainedAdvisorySummaryFact::RetainedEvidence {
                        retained_index: u64::try_from(index).expect("validated retained index"),
                        evidence,
                    }
                }),
        );
        for (snapshot_index, snapshot) in history.snapshots().iter().enumerate() {
            let snapshot_index = u64::try_from(snapshot_index).expect("validated snapshot index");
            facts.push(
                MorphospaceFloat32RetainedAdvisorySummaryFact::HistorySnapshot {
                    snapshot_index,
                    evidence_count: u64::try_from(snapshot.evidence().len())
                        .expect("validated evidence count"),
                },
            );
            facts.extend(snapshot.evidence().iter().copied().enumerate().map(
                |(evidence_index, evidence)| {
                    MorphospaceFloat32RetainedAdvisorySummaryFact::HistoryEvidence {
                        snapshot_index,
                        evidence_index: u64::try_from(evidence_index)
                            .expect("validated evidence index"),
                        evidence,
                    }
                },
            ));
        }
        Ok(MorphospaceFloat32RetainedAdvisorySummary {
            retained,
            history,
            totals: MorphospaceFloat32RetainedAdvisorySummaryTotals {
                retained_evidence_count: retained_count_u64,
                history_snapshot_count: snapshot_count_u64,
                history_evidence_count: history_evidence_count_u64,
                summary_fact_count: required_u64,
            },
            facts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_observation::{
        tests::outcome, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessor,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn snapshot(
    ) -> crate::morphospace_float32_report_advisory_snapshot::MorphospaceFloat32ReportAdvisorySnapshot
    {
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 1).unwrap(),
        )
        .snapshot(
            MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap(),
            MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap(),
            stability,
        )
        .unwrap()
    }

    fn retained() -> CallerRequestedFloat32ReportAdvisoryEvidence {
        let mut processor = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(4),
        );
        let sample = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![42.0]).unwrap(),
            RawSourceTimestamp::new(3.0).unwrap(),
            None,
        );
        let report = processor.process_record(42, sample).unwrap();
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
        .compose(report, snapshot())
        .unwrap()
    }

    fn history() -> MorphospaceFloat32ReportAdvisorySnapshotHistory {
        let sample = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![23.0]).unwrap(),
            RawSourceTimestamp::new(5.0).unwrap(),
            None,
        );
        let observation = MorphospaceFloat32ReportObservationOwner::new(1)
            .unwrap()
            .observe(outcome(vec![23.0f32.to_bits() as u64], vec![sample]))
            .unwrap();
        let observations = MorphospaceFloat32ReportObservationHistory::new(1, 1)
            .unwrap()
            .append(
                MorphospaceFloat32ReportObservationWindow::new(1, 1)
                    .unwrap()
                    .append(observation)
                    .unwrap(),
            )
            .unwrap();
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        let snapshot = MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 2).unwrap(),
        )
        .snapshot(
            observations,
            MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap(),
            stability,
        )
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(snapshot)
            .unwrap()
    }

    fn owner() -> MorphospaceFloat32RetainedAdvisorySummaryOwner {
        MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 3).unwrap(),
        )
    }

    fn report_pointer(value: &CallerRequestedFloat32ReportAdvisoryEvidence) -> *const f32 {
        value.report().sample().sample().values().as_ptr()
    }

    #[test]
    fn actual_p41_inputs_are_retained_and_only_exact_copy_facts_are_indexed() {
        let retained = retained();
        let pointer = report_pointer(&retained);
        let summary = owner().summarize(retained, history()).unwrap();
        assert_eq!(report_pointer(summary.retained()), pointer);
        assert_eq!(summary.history().totals().snapshot_count(), 1);
        assert_eq!(summary.totals().retained_evidence_count(), 1);
        assert_eq!(summary.totals().history_snapshot_count(), 1);
        assert_eq!(summary.totals().history_evidence_count(), 1);
        assert_eq!(summary.totals().summary_fact_count(), 3);
        assert!(matches!(
            summary.facts()[0],
            MorphospaceFloat32RetainedAdvisorySummaryFact::RetainedEvidence {
                retained_index: 0,
                ..
            }
        ));
        assert_eq!(
            summary.facts()[1],
            MorphospaceFloat32RetainedAdvisorySummaryFact::HistorySnapshot {
                snapshot_index: 0,
                evidence_count: 1
            }
        );
        assert!(matches!(
            summary.facts()[2],
            MorphospaceFloat32RetainedAdvisorySummaryFact::HistoryEvidence {
                snapshot_index: 0,
                evidence_index: 0,
                ..
            }
        ));
        let (retained, history) = summary.into_parts();
        assert_eq!(report_pointer(&retained), pointer);
        assert_eq!(history.totals().snapshot_count(), 1);
    }

    #[test]
    fn actual_p41_history_value_composes_into_nonzero_p42_summary() {
        let retained = retained();
        let pointer = report_pointer(&retained);
        let evidence_history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(retained)
            .unwrap();
        assert_eq!(evidence_history.totals().value_count(), 1);
        assert_eq!(evidence_history.totals().ordered_evidence_count(), 1);

        let retained = evidence_history.into_values().pop().unwrap();
        assert_eq!(report_pointer(&retained), pointer);
        let summary = owner().summarize(retained, history()).unwrap();

        assert_eq!(report_pointer(summary.retained()), pointer);
        assert_eq!(summary.totals().retained_evidence_count(), 1);
        assert_eq!(summary.totals().history_snapshot_count(), 1);
        assert_eq!(summary.totals().history_evidence_count(), 1);
        assert_eq!(summary.totals().summary_fact_count(), 3);
        let (retained, snapshot_history) = summary.into_parts();
        assert_eq!(report_pointer(&retained), pointer);
        assert_eq!(snapshot_history.totals().snapshot_count(), 1);
    }

    #[test]
    fn explicit_bounds_reject_zero_and_summary_extent() {
        use MorphospaceFloat32RetainedAdvisorySummaryConfigError::*;
        assert_eq!(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(0, 1, 1),
            Err(ZeroMaximumRetainedEvidence)
        );
        assert_eq!(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 0, 1),
            Err(ZeroMaximumHistorySnapshots)
        );
        assert_eq!(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 0),
            Err(ZeroMaximumSummaryFacts)
        );
        let error = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds {
                maximum_retained_evidence: 1,
                maximum_history_snapshots: 1,
                maximum_summary_facts: 1,
            },
        )
        .summarize(retained(), history())
        .unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32RetainedAdvisorySummaryError::SummaryFactLimit {
                limit: 1,
                required: 3,
                ..
            }
        ));
    }

    #[test]
    fn every_fallible_stage_returns_both_owners_without_partial_mutation() {
        for failure in 0..4 {
            let retained = retained();
            let pointer = report_pointer(&retained);
            let history = history();
            let before = history.totals();
            let mut conversions = 0;
            let mut additions = 0;
            let error = owner()
                .summarize_with(
                    retained,
                    history,
                    |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                    |value| {
                        conversions += 1;
                        if failure == 0 && conversions == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |left, right| {
                        additions += 1;
                        if failure == 1 && additions == 1 {
                            Err(())
                        } else if failure == 2 && additions == 2 {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            let (retained, history) = error.into_parts();
            assert_eq!(report_pointer(&retained), pointer);
            assert_eq!(history.totals(), before);
            assert_eq!(history.snapshots().len(), 1);
        }
    }

    #[test]
    fn source_is_private_default_inert_and_non_applying() {
        let source = include_str!("morphospace_float32_retained_advisory_summary.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs").contains("MorphospaceFloat32RetainedAdvisorySummary"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod morphospace_float32_retained_advisory_summary"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_retained_advisory_summary"));
    }
}
