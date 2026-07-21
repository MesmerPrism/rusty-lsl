// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded pairwise advisory delta over two actual P47 caller snapshots.
//!
//! This crate-private, default-inert proposal retains both snapshots unchanged
//! and derives only exact count relations already present in their observations.
//! It infers no loss, continuity, or causality and grants no runtime authority.

use crate::caller_requested_float32_comparative_advisory_evidence_snapshot::{
    CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation,
};
use crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageRelation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaConfigError {
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
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
    use crate::caller_requested_float32_comparative_advisory_evidence_history::CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory;
    use crate::caller_requested_float32_comparative_advisory_evidence_history::CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds;
    use crate::caller_requested_float32_comparative_advisory_evidence_snapshot::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner,
    };
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
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
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

    fn snapshot(two: bool, seed: u64) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(2, 16).unwrap(),
        )
        .append(comparative(seed))
        .unwrap();
        let history = if two {
            history.append(comparative(seed + 50)).unwrap()
        } else {
            history
        };
        let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(comparative(seed + 100), comparative(seed + 150))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(2, 4, 6).unwrap(),
        )
        .snapshot(history, delta)
        .unwrap()
    }
    fn owner(
        limit: usize,
    ) -> MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner {
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds::new(limit).unwrap(),
        )
    }
    fn pointers(value: &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot) -> (*const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence, *const crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact, *const f32){
        identity(value.history(), value.delta_proposal())
    }
    #[test]
    fn equality_and_directional_order_are_exact() {
        let equal = owner(6)
            .propose(snapshot(false, 1), snapshot(false, 500))
            .unwrap();
        assert!(equal
            .facts()
            .iter()
            .all(|fact| fact.relation() == MorphospaceFloat32AdvisoryReportPackageRelation::Equal));
        let increase = owner(6)
            .propose(snapshot(false, 1), snapshot(true, 500))
            .unwrap();
        assert_eq!(
            increase.facts()[0].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 }
        );
        assert_eq!(
            increase.facts()[1].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 }
        );
        let decrease = owner(6)
            .propose(snapshot(true, 1), snapshot(false, 500))
            .unwrap();
        assert_eq!(
            decrease.facts()[0].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { amount: 1 }
        );
        assert_eq!(
            decrease.facts()[1].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { amount: 1 }
        );
    }
    #[test]
    fn zero_tight_and_extreme_bounds_are_closed() {
        assert_eq!(MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds::new(0), Err(MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaConfigError::ZeroMaximumFacts));
        assert!(
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds::new(usize::MAX)
                .is_ok()
                || usize::BITS > u64::BITS
        );
        assert_eq!(
            owner(6)
                .propose(snapshot(false, 1), snapshot(false, 500))
                .unwrap()
                .fact_count(),
            6
        );
        assert!(matches!(
            owner(5).propose(snapshot(false, 1), snapshot(false, 500)),
            Err(MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError::FactLimit { .. })
        ));
    }
    fn rollback(
        error: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError,
        a: (*const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence, *const crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact, *const f32),
        b: (*const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence, *const crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact, *const f32),
    ) {
        let (a2, b2) = error.into_snapshots();
        assert_eq!(pointers(&a2), a);
        assert_eq!(pointers(&b2), b);
    }
    #[test]
    fn allocation_and_overflow_rollback_preserves_ownership_identity() {
        for failure in 0..5 {
            let a = snapshot(false, 1);
            let b = snapshot(true, 500);
            let pa = pointers(&a);
            let pb = pointers(&b);
            let error = owner(6)
                .propose_with(
                    a,
                    b,
                    |_, _| if failure == 0 { Err(()) } else { Ok(()) },
                    |v| if failure == 1 { Err(()) } else { Ok(v as u64) },
                    |a, b| {
                        if failure == 2 {
                            Err(())
                        } else {
                            a.checked_add(b).ok_or(())
                        }
                    },
                    |a, b| {
                        if failure == 3 {
                            Err(())
                        } else {
                            a.checked_add(b).ok_or(())
                        }
                    },
                    |a, b| {
                        if failure == 4 {
                            Err(())
                        } else {
                            a.checked_sub(b).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            rollback(error, pa, pb);
        }
    }
    #[test]
    fn success_and_consuming_extraction_preserve_both_complete_owners() {
        let a = snapshot(false, 1);
        let b = snapshot(true, 500);
        let pa = pointers(&a);
        let pb = pointers(&b);
        let proposal = owner(6).propose(a, b).unwrap();
        assert_eq!(pointers(proposal.earlier()), pa);
        assert_eq!(pointers(proposal.later()), pb);
        let (a, b) = proposal.into_snapshots();
        assert_eq!(pointers(&a), pa);
        assert_eq!(pointers(&b), pb);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds {
    maximum_facts: usize,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds {
    pub(crate) fn new(
        maximum_facts: usize,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaConfigError> {
        if maximum_facts == 0 {
            return Err(
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaConfigError::ZeroMaximumFacts,
            );
        }
        u64::try_from(maximum_facts).map_err(|_| {
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaConfigError::BoundUnrepresentable {
                requested: maximum_facts,
            }
        })?;
        Ok(Self { maximum_facts })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount {
    Observations,
    HistoryEvidence,
    DeltaFacts,
    EqualDeltaRelations,
    IncreaseDeltaRelations,
    DecreaseDeltaRelations,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaFact {
    count: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount,
    earlier: u64,
    later: u64,
    relation: MorphospaceFloat32AdvisoryReportPackageRelation,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaFact {
    pub(crate) const fn count(&self) -> MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount {
        self.count
    }
    pub(crate) const fn earlier(&self) -> u64 {
        self.earlier
    }
    pub(crate) const fn later(&self) -> u64 {
        self.later
    }
    pub(crate) const fn relation(&self) -> MorphospaceFloat32AdvisoryReportPackageRelation {
        self.relation
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal {
    earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    fact_count: u64,
    facts: Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaFact>,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal {
    pub(crate) const fn earlier(
        &self,
    ) -> &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
        &self.earlier
    }
    pub(crate) const fn later(&self) -> &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
        &self.later
    }
    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
    pub(crate) fn facts(
        &self,
    ) -> &[MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaFact] {
        &self.facts
    }
    pub(crate) fn into_snapshots(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    ) {
        (self.earlier, self.later)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError {
    FactCountOverflow {
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    FactCountUnrepresentable {
        actual: usize,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    FactLimit {
        limit: usize,
        required: usize,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    EvidenceCountOverflow {
        count: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    DifferenceOverflow {
        count: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    Allocation {
        requested: usize,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError {
    pub(crate) fn into_snapshots(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    ) {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError::*;
        match self {
            FactCountOverflow { earlier, later }
            | FactCountUnrepresentable { earlier, later, .. }
            | FactLimit { earlier, later, .. }
            | EvidenceCountOverflow { earlier, later, .. }
            | DifferenceOverflow { earlier, later, .. }
            | Allocation { earlier, later, .. } => (earlier, later),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner {
    bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner {
    pub(crate) const fn new(
        bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn propose(
        &self,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    ) -> Result<
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError,
    > {
        self.propose_with(
            earlier,
            later,
            |facts, count| facts.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
            |a, b| a.checked_add(b).ok_or(()),
            |a, b| a.checked_sub(b).ok_or(()),
        )
    }

    fn propose_with<R, C, U, A, S>(
        &self,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        reserve: R,
        convert: C,
        required_add: U,
        mut add: A,
        mut subtract: S,
    ) -> Result<
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError,
    >
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaFact>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnOnce(usize, usize) -> Result<usize, ()>,
        A: FnMut(u64, u64) -> Result<u64, ()>,
        S: FnMut(u64, u64) -> Result<u64, ()>,
    {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* earlier, later }) }; }
        const COUNTS: [MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount; 6] = [
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::Observations,
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::HistoryEvidence,
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::DeltaFacts,
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::EqualDeltaRelations,
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::IncreaseDeltaRelations,
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::DecreaseDeltaRelations,
        ];
        let required = required_add(3, 3);
        let required = match required {
            Ok(value) => value,
            Err(()) => fail!(FactCountOverflow {}),
        };
        if required > self.bounds.maximum_facts {
            fail!(FactLimit {
                limit: self.bounds.maximum_facts,
                required: required
            });
        }
        let fact_count = match convert(required) {
            Ok(value) => value,
            Err(()) => fail!(FactCountUnrepresentable { actual: required }),
        };
        let mut counts = |snapshot: &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
                          kind|
         -> Result<u64, ()> {
            if kind == MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::Observations {
                return Ok(snapshot.observation_count());
            }
            let mut total = 0u64;
            for observation in snapshot.observations() {
                let matches = match (kind, observation) {
                    (MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::HistoryEvidence, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::HistoryEvidence { .. }) => true,
                    (MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::DeltaFacts, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact { .. }) => true,
                    (MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::EqualDeltaRelations, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact { relation: MorphospaceFloat32AdvisoryReportPackageRelation::Equal, .. }) => true,
                    (MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::IncreaseDeltaRelations, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact { relation: MorphospaceFloat32AdvisoryReportPackageRelation::Increase { .. }, .. }) => true,
                    (MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::DecreaseDeltaRelations, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact { relation: MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { .. }, .. }) => true,
                    _ => false,
                };
                if matches {
                    total = add(total, 1)?;
                }
            }
            Ok(total)
        };
        let mut facts = Vec::new();
        if reserve(&mut facts, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for count in COUNTS {
            let a = match counts(&earlier, count) {
                Ok(value) => value,
                Err(()) => fail!(EvidenceCountOverflow { count: count }),
            };
            let b = match counts(&later, count) {
                Ok(value) => value,
                Err(()) => fail!(EvidenceCountOverflow { count: count }),
            };
            let relation = if b > a {
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase {
                    amount: match subtract(b, a) {
                        Ok(value) => value,
                        Err(()) => fail!(DifferenceOverflow { count: count }),
                    },
                }
            } else if b < a {
                MorphospaceFloat32AdvisoryReportPackageRelation::Decrease {
                    amount: match subtract(a, b) {
                        Ok(value) => value,
                        Err(()) => fail!(DifferenceOverflow { count: count }),
                    },
                }
            } else {
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal
            };
            facts.push(
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaFact {
                    count,
                    earlier: a,
                    later: b,
                    relation,
                },
            );
        }
        Ok(
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal {
                earlier,
                later,
                fact_count,
                facts,
            },
        )
    }
}
