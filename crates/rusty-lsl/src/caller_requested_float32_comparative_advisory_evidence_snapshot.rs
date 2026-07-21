// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested bounded observation over actual P43--P46 evidence owners.
//!
//! This crate-private module is default-inert, advisory, and non-applying. It
//! retains its P46 history and proposal unchanged and indexes only exact facts
//! already exposed by them. It infers neither loss nor continuity and grants no
//! liblsl-equivalence, Manifold, session, stream, transport, or control authority.

use crate::caller_requested_float32_comparative_advisory_evidence_history::CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory;
use crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageRelation;
use crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::{
    MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
    MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotConfigError {
    ZeroMaximumHistoryEvidence,
    ZeroMaximumDeltaFacts,
    ZeroMaximumObservations,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds {
    maximum_history_evidence: usize,
    maximum_delta_facts: usize,
    maximum_observations: usize,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds {
    pub(crate) fn new(
        maximum_history_evidence: usize,
        maximum_delta_facts: usize,
        maximum_observations: usize,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotConfigError> {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotConfigError::*;
        for (value, error) in [
            (maximum_history_evidence, ZeroMaximumHistoryEvidence),
            (maximum_delta_facts, ZeroMaximumDeltaFacts),
            (maximum_observations, ZeroMaximumObservations),
        ] {
            if value == 0 {
                return Err(error);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_history_evidence,
            maximum_delta_facts,
            maximum_observations,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation {
    HistoryEvidence {
        evidence_index: u64,
        fact_count: u64,
    },
    DeltaFact {
        fact_index: u64,
        count: MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
        earlier: u64,
        later: u64,
        relation: MorphospaceFloat32AdvisoryReportPackageRelation,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
    history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
    delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    observation_count: u64,
    observations: Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation>,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
    pub(crate) const fn history(
        &self,
    ) -> &CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory {
        &self.history
    }
    pub(crate) const fn delta_proposal(
        &self,
    ) -> &MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal {
        &self.delta_proposal
    }
    pub(crate) const fn observation_count(&self) -> u64 {
        self.observation_count
    }
    pub(crate) fn observations(
        &self,
    ) -> &[CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation] {
        &self.observations
    }
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    ) {
        (self.history, self.delta_proposal)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotError {
    HistoryEvidenceLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    DeltaFactLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    ObservationCountOverflow {
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    ObservationLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    IndexUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    Allocation {
        requested: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    ) {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotError::*;
        match self {
            HistoryEvidenceLimit {
                history,
                delta_proposal,
                ..
            }
            | DeltaFactLimit {
                history,
                delta_proposal,
                ..
            }
            | ObservationCountOverflow {
                history,
                delta_proposal,
            }
            | ObservationLimit {
                history,
                delta_proposal,
                ..
            }
            | IndexUnrepresentable {
                history,
                delta_proposal,
                ..
            }
            | Allocation {
                history,
                delta_proposal,
                ..
            } => (history, delta_proposal),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner {
    bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner {
    pub(crate) const fn new(
        bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds,
    ) -> Self {
        Self { bounds }
    }
    pub(crate) fn snapshot(
        &self,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    ) -> Result<
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotError,
    > {
        self.snapshot_with(
            history,
            delta_proposal,
            |values, count| values.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
        )
    }

    fn snapshot_with<R, C, A>(
        &self,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
        reserve: R,
        mut convert: C,
        add: A,
    ) -> Result<
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation>,
            usize,
        ) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        A: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history, delta_proposal }) }; }
        let history_count = history.evidence().len();
        let delta_count = delta_proposal.facts().len();
        if history_count > self.bounds.maximum_history_evidence {
            fail!(HistoryEvidenceLimit {
                limit: self.bounds.maximum_history_evidence,
                actual: history_count
            });
        }
        if delta_count > self.bounds.maximum_delta_facts {
            fail!(DeltaFactLimit {
                limit: self.bounds.maximum_delta_facts,
                actual: delta_count
            });
        }
        let required = match add(history_count, delta_count) {
            Ok(value) => value,
            Err(()) => fail!(ObservationCountOverflow {}),
        };
        if required > self.bounds.maximum_observations {
            fail!(ObservationLimit {
                limit: self.bounds.maximum_observations,
                required: required
            });
        }
        let observation_count = match convert(required) {
            Ok(value) => value,
            Err(()) => fail!(IndexUnrepresentable { actual: required }),
        };
        for index in 0..history_count.max(delta_count) {
            if convert(index).is_err() {
                fail!(IndexUnrepresentable { actual: index });
            }
        }
        let mut observations = Vec::new();
        if reserve(&mut observations, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for (index, evidence) in history.evidence().iter().enumerate() {
            observations.push(CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::HistoryEvidence { evidence_index: u64::try_from(index).expect("validated index"), fact_count: evidence.fact_count() });
        }
        for (index, fact) in delta_proposal.facts().iter().enumerate() {
            observations.push(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                    fact_index: u64::try_from(index).expect("validated index"),
                    count: fact.count(),
                    earlier: fact.earlier(),
                    later: fact.later(),
                    relation: fact.relation(),
                },
            );
        }
        Ok(CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
            history,
            delta_proposal,
            observation_count,
            observations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_advisory_report_package::{
        CallerRequestedFloat32AdvisoryReportPackageBounds,
        CallerRequestedFloat32AdvisoryReportPackageOwner,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence_history::CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds;
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_advisory_report_package_delta_proposal::{
        MorphospaceFloat32AdvisoryReportPackageDeltaBounds,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner,
    };
    use crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner,
    };
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

    fn advisory_snapshot(
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

    fn package(sequence: u64) -> crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage{
        let evidence = |sequence| {
            let mut processor = Float32SessionReportRequestedPostProcessing::new(
                RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                    RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
                ))
                .unwrap(),
                ExactSequenceLossHealth::new(4),
            );
            let report = processor
                .process_record(
                    sequence,
                    TimestampedSample::new(
                        Sample::new(SampleLimits::new(1).unwrap(), 1, vec![sequence as f32])
                            .unwrap(),
                        RawSourceTimestamp::new(3.0).unwrap(),
                        None,
                    ),
                )
                .unwrap();
            CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
            )
            .compose(report, advisory_snapshot())
            .unwrap()
        };
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(evidence(sequence))
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            evidence(sequence + 1),
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                .unwrap()
                .append(advisory_snapshot())
                .unwrap(),
        )
        .unwrap();
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 2, 4).unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }

    fn comparative(seed: u64) -> crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence{
        let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(package(seed), package(seed + 10))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
        )
        .compose(package(seed + 20), package(seed + 30), proposal)
        .unwrap()
    }

    fn inputs() -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    ) {
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(2, 16).unwrap(),
        )
        .append(comparative(1))
        .unwrap()
        .append(comparative(50))
        .unwrap();
        let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(comparative(100), comparative(150))
        .unwrap();
        (history, delta)
    }

    fn identity(history: &CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory, delta: &MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal) -> (*const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence, *const crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact, *const f32){
        (
            history.evidence().as_ptr(),
            delta.facts().as_ptr(),
            history.evidence()[0].earlier().history().values()[0]
                .report()
                .sample()
                .sample()
                .values()
                .as_ptr(),
        )
    }

    #[test]
    fn bounds_are_explicit_and_checked() {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotConfigError::*;
        assert_eq!(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(0, 1, 1),
            Err(ZeroMaximumHistoryEvidence)
        );
        assert_eq!(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(1, 0, 1),
            Err(ZeroMaximumDeltaFacts)
        );
        assert_eq!(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(1, 1, 0),
            Err(ZeroMaximumObservations)
        );
    }

    #[test]
    fn actual_p43_through_p46_inputs_are_retained_in_fixed_order() {
        let (history, delta) = inputs();
        let expected = identity(&history, &delta);
        let value = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(2, 4, 6).unwrap(),
        )
        .snapshot(history, delta)
        .unwrap();
        assert_eq!(value.observation_count(), 6);
        assert!(matches!(
            value.observations()[0],
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::HistoryEvidence {
                evidence_index: 0,
                fact_count: 8
            }
        ));
        assert!(matches!(
            value.observations()[2],
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                fact_index: 0,
                count: MorphospaceFloat32ComparativeAdvisoryEvidenceCount::Facts,
                ..
            }
        ));
        assert!(matches!(
            value.observations()[5],
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                fact_index: 3,
                count: MorphospaceFloat32ComparativeAdvisoryEvidenceCount::DecreaseRelations,
                ..
            }
        ));
        assert_eq!(identity(value.history(), value.delta_proposal()), expected);
        let (history, delta) = value.into_parts();
        assert_eq!(identity(&history, &delta), expected);
    }

    #[test]
    fn every_failure_returns_both_complete_actual_inputs_without_mutation() {
        for failure in 0..6 {
            let (history, delta) = inputs();
            let expected = identity(&history, &delta);
            let owner = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
                match failure {
                    0 => CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(
                        1, 4, 6,
                    ),
                    1 => CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(
                        2, 3, 6,
                    ),
                    2 => CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(
                        2, 4, 5,
                    ),
                    _ => CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(
                        2, 4, 6,
                    ),
                }
                .unwrap(),
            );
            let result = match failure {
                0..=2 => owner.snapshot(history, delta),
                3 => owner.snapshot_with(
                    history,
                    delta,
                    |_, _| Ok(()),
                    |value| Ok(value as u64),
                    |_, _| Err(()),
                ),
                4 => owner.snapshot_with(
                    history,
                    delta,
                    |_, _| Ok(()),
                    |_| Err(()),
                    |a, b| a.checked_add(b).ok_or(()),
                ),
                _ => owner.snapshot_with(
                    history,
                    delta,
                    |_, _| Err(()),
                    |value| Ok(value as u64),
                    |a, b| a.checked_add(b).ok_or(()),
                ),
            };
            let (history, delta) = result.unwrap_err().into_parts();
            assert_eq!(identity(&history, &delta), expected);
            assert_eq!(history.totals().evidence_count(), 2);
            assert_eq!(delta.fact_count(), 4);
        }
    }

    #[test]
    fn source_boundary_is_private_inert_and_non_applying() {
        let source =
            include_str!("caller_requested_float32_comparative_advisory_evidence_snapshot.rs");
        for wording in [
            "crate-private",
            "default-inert",
            "infers neither loss nor continuity",
            "liblsl-equivalence, Manifold",
            "non-applying",
        ] {
            assert!(source.contains(wording));
        }
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(!include_str!("runtime.rs")
            .contains("CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_comparative_advisory_evidence_snapshot"));
    }
}
