// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested exact delta proposal over two actual P43 report packages.
//!
//! This module is deliberately undeclared. The owner consumes and retains both
//! packages unchanged and allocates only an ordered set of exact count
//! relations. It is crate-private, default-inert, advisory, and non-applying.

use crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32AdvisoryReportPackageDeltaConfigError {
    ZeroMaximumRelations,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaBounds {
    maximum_relations: usize,
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaBounds {
    pub(crate) fn new(
        maximum_relations: usize,
    ) -> Result<Self, MorphospaceFloat32AdvisoryReportPackageDeltaConfigError> {
        if maximum_relations == 0 {
            return Err(
                MorphospaceFloat32AdvisoryReportPackageDeltaConfigError::ZeroMaximumRelations,
            );
        }
        u64::try_from(maximum_relations).map_err(|_| {
            MorphospaceFloat32AdvisoryReportPackageDeltaConfigError::BoundUnrepresentable {
                requested: maximum_relations,
            }
        })?;
        Ok(Self { maximum_relations })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32AdvisoryReportPackageCount {
    HistoryValues,
    HistoryEvidence,
    SummaryFacts,
    PackageFacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32AdvisoryReportPackageRelation {
    Equal,
    Increase { amount: u64 },
    Decrease { amount: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaFact {
    count: MorphospaceFloat32AdvisoryReportPackageCount,
    earlier: u64,
    later: u64,
    relation: MorphospaceFloat32AdvisoryReportPackageRelation,
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaFact {
    pub(crate) const fn count(&self) -> MorphospaceFloat32AdvisoryReportPackageCount {
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
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaProposal {
    earlier: CallerRequestedFloat32AdvisoryReportPackage,
    later: CallerRequestedFloat32AdvisoryReportPackage,
    relation_count: u64,
    facts: Vec<MorphospaceFloat32AdvisoryReportPackageDeltaFact>,
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaProposal {
    pub(crate) const fn earlier(&self) -> &CallerRequestedFloat32AdvisoryReportPackage {
        &self.earlier
    }
    pub(crate) const fn later(&self) -> &CallerRequestedFloat32AdvisoryReportPackage {
        &self.later
    }
    pub(crate) const fn relation_count(&self) -> u64 {
        self.relation_count
    }
    pub(crate) fn facts(&self) -> &[MorphospaceFloat32AdvisoryReportPackageDeltaFact] {
        &self.facts
    }
    pub(crate) fn into_packages(
        self,
    ) -> (
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackage,
    ) {
        (self.earlier, self.later)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32AdvisoryReportPackageDeltaError {
    RelationCountOverflow {
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
    },
    RelationCountUnrepresentable {
        actual: usize,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
    },
    RelationLimit {
        limit: usize,
        required: usize,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
    },
    DeltaOverflow {
        count: MorphospaceFloat32AdvisoryReportPackageCount,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
    },
    Allocation {
        requested: usize,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
    },
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaError {
    pub(crate) fn into_packages(
        self,
    ) -> (
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackage,
    ) {
        use MorphospaceFloat32AdvisoryReportPackageDeltaError::*;
        match self {
            RelationCountOverflow { earlier, later }
            | RelationCountUnrepresentable { earlier, later, .. }
            | RelationLimit { earlier, later, .. }
            | DeltaOverflow { earlier, later, .. }
            | Allocation { earlier, later, .. } => (earlier, later),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner {
    bounds: MorphospaceFloat32AdvisoryReportPackageDeltaBounds,
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner {
    pub(crate) const fn new(bounds: MorphospaceFloat32AdvisoryReportPackageDeltaBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn propose(
        &self,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
    ) -> Result<
        MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
        MorphospaceFloat32AdvisoryReportPackageDeltaError,
    > {
        self.propose_with(
            earlier,
            later,
            |facts, requested| facts.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |larger, smaller| larger.checked_sub(smaller).ok_or(()),
        )
    }

    fn propose_with<R, C, A, S>(
        &self,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        reserve: R,
        convert: C,
        mut add: A,
        mut subtract: S,
    ) -> Result<
        MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
        MorphospaceFloat32AdvisoryReportPackageDeltaError,
    >
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32AdvisoryReportPackageDeltaFact>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
        S: FnMut(u64, u64) -> Result<u64, ()>,
    {
        use MorphospaceFloat32AdvisoryReportPackageDeltaError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* earlier, later }) }; }

        let mut required = 0usize;
        for _ in 0..4 {
            required = match add(required, 1) {
                Ok(value) => value,
                Err(()) => fail!(RelationCountOverflow {}),
            };
        }
        if required > self.bounds.maximum_relations {
            fail!(RelationLimit {
                limit: self.bounds.maximum_relations,
                required: required
            });
        }
        let relation_count = match convert(required) {
            Ok(value) => value,
            Err(()) => fail!(RelationCountUnrepresentable { actual: required }),
        };

        let earlier_totals = earlier.totals();
        let later_totals = later.totals();
        let values = [
            (
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryValues,
                earlier_totals.history_value_count(),
                later_totals.history_value_count(),
            ),
            (
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryEvidence,
                earlier_totals.history_evidence_count(),
                later_totals.history_evidence_count(),
            ),
            (
                MorphospaceFloat32AdvisoryReportPackageCount::SummaryFacts,
                earlier_totals.summary_fact_count(),
                later_totals.summary_fact_count(),
            ),
            (
                MorphospaceFloat32AdvisoryReportPackageCount::PackageFacts,
                earlier_totals.package_fact_count(),
                later_totals.package_fact_count(),
            ),
        ];

        let mut derived = [MorphospaceFloat32AdvisoryReportPackageRelation::Equal; 4];
        for (index, (count, before, after)) in values.iter().copied().enumerate() {
            derived[index] = if after > before {
                match subtract(after, before) {
                    Ok(amount) => {
                        MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount }
                    }
                    Err(()) => fail!(DeltaOverflow { count: count }),
                }
            } else if after < before {
                match subtract(before, after) {
                    Ok(amount) => {
                        MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { amount }
                    }
                    Err(()) => fail!(DeltaOverflow { count: count }),
                }
            } else {
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal
            };
        }

        let mut facts = Vec::new();
        if reserve(&mut facts, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        facts.extend(
            values
                .into_iter()
                .zip(derived)
                .map(|((count, before, after), relation)| {
                    MorphospaceFloat32AdvisoryReportPackageDeltaFact {
                        count,
                        earlier: before,
                        later: after,
                        relation,
                    }
                }),
        );
        Ok(MorphospaceFloat32AdvisoryReportPackageDeltaProposal {
            earlier,
            later,
            relation_count,
            facts,
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

    fn evidence(sequence: u64, value: f32) -> crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidence{
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
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
                    RawSourceTimestamp::new(3.0).unwrap(),
                    None,
                ),
            )
            .unwrap();
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
        .compose(report, snapshot())
        .unwrap()
    }

    fn package(history_count: usize, base: u64) -> CallerRequestedFloat32AdvisoryReportPackage {
        let mut history =
            CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(history_count.max(1)).unwrap();
        for index in 0..history_count {
            history = history
                .append(evidence(base + index as u64, index as f32))
                .unwrap();
        }
        let retained = evidence(base + 20, 20.0);
        let snapshots = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(snapshot())
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(retained, snapshots)
        .unwrap();
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(
                history_count.max(1),
                history_count.max(1),
                2,
                history_count
                    .checked_mul(2)
                    .and_then(|v| v.checked_add(2))
                    .unwrap()
                    .max(2),
            )
            .unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }

    fn pointers(package: &CallerRequestedFloat32AdvisoryReportPackage) -> Vec<*const f32> {
        package
            .history()
            .values()
            .iter()
            .map(|v| v.report().sample().sample().values().as_ptr())
            .chain(std::iter::once(
                package
                    .summary()
                    .retained()
                    .report()
                    .sample()
                    .sample()
                    .values()
                    .as_ptr(),
            ))
            .collect()
    }

    fn owner(limit: usize) -> MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner {
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(limit).unwrap(),
        )
    }

    #[test]
    fn equal_increase_decrease_and_ties_are_explicit_and_ordered() {
        let equal = owner(4).propose(package(1, 1), package(1, 10)).unwrap();
        assert!(equal
            .facts()
            .iter()
            .all(|fact| fact.relation() == MorphospaceFloat32AdvisoryReportPackageRelation::Equal));
        assert_eq!(
            equal.facts().iter().map(|f| f.count()).collect::<Vec<_>>(),
            [
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryValues,
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryEvidence,
                MorphospaceFloat32AdvisoryReportPackageCount::SummaryFacts,
                MorphospaceFloat32AdvisoryReportPackageCount::PackageFacts
            ]
        );
        let increase = owner(4).propose(package(1, 1), package(2, 10)).unwrap();
        assert_eq!(
            increase.facts()[0].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 }
        );
        assert_eq!(
            increase.facts()[1].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 }
        );
        assert_eq!(
            increase.facts()[2].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Equal
        );
        assert_eq!(
            increase.facts()[3].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 2 }
        );
        let decrease = owner(4).propose(package(2, 1), package(1, 10)).unwrap();
        assert_eq!(
            decrease.facts()[0].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { amount: 1 }
        );
        assert_eq!(
            decrease.facts()[3].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { amount: 2 }
        );
    }

    #[test]
    fn zero_exact_and_one_past_bounds_are_closed() {
        assert_eq!(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(0),
            Err(MorphospaceFloat32AdvisoryReportPackageDeltaConfigError::ZeroMaximumRelations)
        );
        assert_eq!(
            owner(4)
                .propose(package(1, 1), package(1, 10))
                .unwrap()
                .relation_count(),
            4
        );
        assert!(matches!(
            owner(3).propose(package(1, 1), package(1, 10)),
            Err(
                MorphospaceFloat32AdvisoryReportPackageDeltaError::RelationLimit {
                    limit: 3,
                    required: 4,
                    ..
                }
            )
        ));
    }

    #[test]
    fn checked_overflow_and_honest_allocation_refusal_roll_back_without_partial_output() {
        for failure in 0..4 {
            let earlier = package(1, 1);
            let later = package(2, 10);
            let before_earlier = pointers(&earlier);
            let before_later = pointers(&later);
            let error = owner(4)
                .propose_with(
                    earlier,
                    later,
                    |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                    |value| {
                        if failure == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |left, right| {
                        if failure == 0 {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                    |left, right| {
                        if failure == 2 {
                            Err(())
                        } else {
                            left.checked_sub(right).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            let (earlier, later) = error.into_packages();
            assert_eq!(pointers(&earlier), before_earlier);
            assert_eq!(pointers(&later), before_later);
        }
    }

    #[test]
    fn both_packages_and_nested_samples_preserve_identity_on_success_and_consuming_extraction() {
        let earlier = package(1, 1);
        let later = package(2, 10);
        let before_earlier = pointers(&earlier);
        let before_later = pointers(&later);
        let proposal = owner(4).propose(earlier, later).unwrap();
        assert_eq!(pointers(proposal.earlier()), before_earlier);
        assert_eq!(pointers(proposal.later()), before_later);
        let (earlier, later) = proposal.into_packages();
        assert_eq!(pointers(&earlier), before_earlier);
        assert_eq!(pointers(&later), before_later);
    }

    #[test]
    fn source_is_private_default_inert_advisory_and_non_applying() {
        let source = include_str!("morphospace_float32_advisory_report_package_delta_proposal.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("packet_", "loss"),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(
            !include_str!("runtime.rs").contains("MorphospaceFloat32AdvisoryReportPackageDelta")
        );
        assert!(!include_str!("lib.rs")
            .contains("pub mod morphospace_float32_advisory_report_package_delta_proposal"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_advisory_report_package_delta_proposal"));
    }
}
