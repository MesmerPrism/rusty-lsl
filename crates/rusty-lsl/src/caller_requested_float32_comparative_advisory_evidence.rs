// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded caller-requested comparison of P43 packages with exact P44 evidence.
//!
//! This module is deliberately undeclared. It retains every input unchanged,
//! derives only exact count relations, and cannot apply its advisory evidence.

use crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage;
use crate::morphospace_float32_advisory_report_package_delta_proposal::{
    MorphospaceFloat32AdvisoryReportPackageCount,
    MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    MorphospaceFloat32AdvisoryReportPackageRelation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceConfigError {
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds {
    maximum_facts: usize,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds {
    pub(crate) fn new(
        maximum_facts: usize,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceConfigError> {
        if maximum_facts == 0 {
            return Err(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceConfigError::ZeroMaximumFacts,
            );
        }
        u64::try_from(maximum_facts).map_err(|_| {
            CallerRequestedFloat32ComparativeAdvisoryEvidenceConfigError::BoundUnrepresentable {
                requested: maximum_facts,
            }
        })?;
        Ok(Self { maximum_facts })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceSide {
    Earlier,
    Later,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceFact {
    count: MorphospaceFloat32AdvisoryReportPackageCount,
    side: CallerRequestedFloat32ComparativeAdvisoryEvidenceSide,
    package_value: u64,
    proposal_value: u64,
    relation: MorphospaceFloat32AdvisoryReportPackageRelation,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceFact {
    pub(crate) const fn count(&self) -> MorphospaceFloat32AdvisoryReportPackageCount {
        self.count
    }
    pub(crate) const fn side(&self) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceSide {
        self.side
    }
    pub(crate) const fn package_value(&self) -> u64 {
        self.package_value
    }
    pub(crate) const fn proposal_value(&self) -> u64 {
        self.proposal_value
    }
    pub(crate) const fn relation(&self) -> MorphospaceFloat32AdvisoryReportPackageRelation {
        self.relation
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidence {
    earlier: CallerRequestedFloat32AdvisoryReportPackage,
    later: CallerRequestedFloat32AdvisoryReportPackage,
    delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    fact_count: u64,
    facts: Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceFact>,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidence {
    pub(crate) const fn earlier(&self) -> &CallerRequestedFloat32AdvisoryReportPackage {
        &self.earlier
    }
    pub(crate) const fn later(&self) -> &CallerRequestedFloat32AdvisoryReportPackage {
        &self.later
    }
    pub(crate) const fn delta_proposal(
        &self,
    ) -> &MorphospaceFloat32AdvisoryReportPackageDeltaProposal {
        &self.delta_proposal
    }
    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
    pub(crate) fn facts(&self) -> &[CallerRequestedFloat32ComparativeAdvisoryEvidenceFact] {
        &self.facts
    }
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackage,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) {
        (self.earlier, self.later, self.delta_proposal)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceError {
    FactCountOverflow {
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    FactCountUnrepresentable {
        actual: usize,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    FactLimit {
        limit: usize,
        required: usize,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    DifferenceOverflow {
        count: MorphospaceFloat32AdvisoryReportPackageCount,
        side: CallerRequestedFloat32ComparativeAdvisoryEvidenceSide,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    Allocation {
        requested: usize,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackage,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceError::*;
        match self {
            FactCountOverflow {
                earlier,
                later,
                delta_proposal,
            }
            | FactCountUnrepresentable {
                earlier,
                later,
                delta_proposal,
                ..
            }
            | FactLimit {
                earlier,
                later,
                delta_proposal,
                ..
            }
            | DifferenceOverflow {
                earlier,
                later,
                delta_proposal,
                ..
            }
            | Allocation {
                earlier,
                later,
                delta_proposal,
                ..
            } => (earlier, later, delta_proposal),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner {
    bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner {
    pub(crate) const fn new(
        bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn compose(
        &self,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) -> Result<
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceError,
    > {
        self.compose_with(
            earlier,
            later,
            delta_proposal,
            |facts, requested| facts.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |larger, smaller| larger.checked_sub(smaller).ok_or(()),
        )
    }

    fn compose_with<R, C, A, S>(
        &self,
        earlier: CallerRequestedFloat32AdvisoryReportPackage,
        later: CallerRequestedFloat32AdvisoryReportPackage,
        delta_proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
        reserve: R,
        convert: C,
        mut add: A,
        mut subtract: S,
    ) -> Result<
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceFact>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
        S: FnMut(u64, u64) -> Result<u64, ()>,
    {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* earlier, later, delta_proposal }) }; }

        let required = match add(delta_proposal.facts().len(), delta_proposal.facts().len()) {
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

        let earlier_totals = earlier.totals();
        let later_totals = later.totals();
        let package_value = |side, count| match (side, count) {
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier,
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryValues,
            ) => earlier_totals.history_value_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier,
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryEvidence,
            ) => earlier_totals.history_evidence_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier,
                MorphospaceFloat32AdvisoryReportPackageCount::SummaryFacts,
            ) => earlier_totals.summary_fact_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier,
                MorphospaceFloat32AdvisoryReportPackageCount::PackageFacts,
            ) => earlier_totals.package_fact_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later,
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryValues,
            ) => later_totals.history_value_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later,
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryEvidence,
            ) => later_totals.history_evidence_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later,
                MorphospaceFloat32AdvisoryReportPackageCount::SummaryFacts,
            ) => later_totals.summary_fact_count(),
            (
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later,
                MorphospaceFloat32AdvisoryReportPackageCount::PackageFacts,
            ) => later_totals.package_fact_count(),
        };

        let mut derived = Vec::new();
        if reserve(&mut derived, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for proposal_fact in delta_proposal.facts() {
            for side in [
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier,
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later,
            ] {
                let count = proposal_fact.count();
                let package_value = package_value(side, count);
                let proposal_value = match side {
                    CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier => {
                        proposal_fact.earlier()
                    }
                    CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later => {
                        proposal_fact.later()
                    }
                };
                let relation = if proposal_value > package_value {
                    MorphospaceFloat32AdvisoryReportPackageRelation::Increase {
                        amount: match subtract(proposal_value, package_value) {
                            Ok(value) => value,
                            Err(()) => fail!(DifferenceOverflow {
                                count: count,
                                side: side
                            }),
                        },
                    }
                } else if proposal_value < package_value {
                    MorphospaceFloat32AdvisoryReportPackageRelation::Decrease {
                        amount: match subtract(package_value, proposal_value) {
                            Ok(value) => value,
                            Err(()) => fail!(DifferenceOverflow {
                                count: count,
                                side: side
                            }),
                        },
                    }
                } else {
                    MorphospaceFloat32AdvisoryReportPackageRelation::Equal
                };
                derived.push(CallerRequestedFloat32ComparativeAdvisoryEvidenceFact {
                    count,
                    side,
                    package_value,
                    proposal_value,
                    relation,
                });
            }
        }
        Ok(CallerRequestedFloat32ComparativeAdvisoryEvidence {
            earlier,
            later,
            delta_proposal,
            fact_count,
            facts: derived,
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
    use crate::morphospace_float32_advisory_report_package_delta_proposal::{
        MorphospaceFloat32AdvisoryReportPackageDeltaBounds,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner,
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
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            evidence(base + 20, 20.0),
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                .unwrap()
                .append(snapshot())
                .unwrap(),
        )
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

    fn proposal(
        earlier_count: usize,
        later_count: usize,
    ) -> MorphospaceFloat32AdvisoryReportPackageDeltaProposal {
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(package(earlier_count, 100), package(later_count, 200))
        .unwrap()
    }

    fn owner(limit: usize) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner {
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(limit).unwrap(),
        )
    }

    fn package_pointers(package: &CallerRequestedFloat32AdvisoryReportPackage) -> Vec<*const f32> {
        package
            .history()
            .values()
            .iter()
            .map(|value| value.report().sample().sample().values().as_ptr())
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

    fn identities(
        earlier: &CallerRequestedFloat32AdvisoryReportPackage,
        later: &CallerRequestedFloat32AdvisoryReportPackage,
        proposal: &MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) -> (Vec<*const f32>, Vec<*const f32>, Vec<*const f32>, Vec<*const f32>, *const crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageDeltaFact){
        (
            package_pointers(earlier),
            package_pointers(later),
            package_pointers(proposal.earlier()),
            package_pointers(proposal.later()),
            proposal.facts().as_ptr(),
        )
    }

    #[test]
    fn complete_inputs_identity_and_fact_order_are_exact() {
        let earlier = package(1, 1);
        let later = package(2, 10);
        let proposal = proposal(1, 2);
        let before = identities(&earlier, &later, &proposal);
        let evidence = owner(8).compose(earlier, later, proposal).unwrap();
        assert_eq!(
            identities(
                evidence.earlier(),
                evidence.later(),
                evidence.delta_proposal()
            ),
            before
        );
        assert_eq!(evidence.fact_count(), 8);
        assert_eq!(evidence.facts().len(), 8);
        for (index, pair) in evidence.facts().chunks_exact(2).enumerate() {
            assert_eq!(
                pair[0].side(),
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Earlier
            );
            assert_eq!(
                pair[1].side(),
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSide::Later
            );
            assert_eq!(
                pair[0].count(),
                evidence.delta_proposal().facts()[index].count()
            );
            assert_eq!(
                pair[1].count(),
                evidence.delta_proposal().facts()[index].count()
            );
            assert_eq!(
                pair[0].proposal_value(),
                evidence.delta_proposal().facts()[index].earlier()
            );
            assert_eq!(
                pair[1].proposal_value(),
                evidence.delta_proposal().facts()[index].later()
            );
        }
    }

    #[test]
    fn equality_increase_decrease_and_ties_are_derived_only_from_exact_counts() {
        let evidence = owner(8)
            .compose(package(1, 1), package(2, 2), proposal(2, 1))
            .unwrap();
        assert_eq!(
            evidence.facts()[0].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 }
        );
        assert_eq!(
            evidence.facts()[1].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { amount: 1 }
        );
        assert_eq!(
            evidence.facts()[4].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Equal
        );
        assert_eq!(
            evidence.facts()[5].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Equal
        );
        assert_eq!(evidence.facts()[6].package_value(), 4);
        assert_eq!(evidence.facts()[6].proposal_value(), 6);
        assert_eq!(
            evidence.facts()[6].relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 2 }
        );
    }

    #[test]
    fn zero_exact_and_one_past_bounds_are_closed() {
        assert_eq!(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(0),
            Err(CallerRequestedFloat32ComparativeAdvisoryEvidenceConfigError::ZeroMaximumFacts)
        );
        assert_eq!(
            owner(8)
                .compose(package(1, 1), package(1, 2), proposal(1, 1))
                .unwrap()
                .fact_count(),
            8
        );
        assert!(matches!(
            owner(7).compose(package(1, 1), package(1, 2), proposal(1, 1)),
            Err(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceError::FactLimit {
                    limit: 7,
                    required: 8,
                    ..
                }
            )
        ));
    }

    #[test]
    fn allocation_conversion_arithmetic_and_difference_failures_roll_back_all_inputs() {
        for failure in 0..4 {
            let earlier = package(1, 1);
            let later = package(2, 2);
            let proposal = proposal(2, 1);
            let before = identities(&earlier, &later, &proposal);
            let mut subtractions = 0;
            let error = owner(8)
                .compose_with(
                    earlier,
                    later,
                    proposal,
                    |_, _| if failure == 2 { Err(()) } else { Ok(()) },
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
                        subtractions += 1;
                        if failure == 3 && subtractions == 1 {
                            Err(())
                        } else {
                            left.checked_sub(right).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            let (earlier, later, proposal) = error.into_parts();
            assert_eq!(identities(&earlier, &later, &proposal), before);
        }
    }

    #[test]
    fn one_past_limit_failure_rolls_back_complete_inputs() {
        let earlier = package(1, 1);
        let later = package(1, 2);
        let proposal = proposal(1, 1);
        let before = identities(&earlier, &later, &proposal);
        let (earlier, later, proposal) = owner(7)
            .compose(earlier, later, proposal)
            .unwrap_err()
            .into_parts();
        assert_eq!(identities(&earlier, &later, &proposal), before);
    }

    #[test]
    fn consuming_extraction_returns_every_original_allocation() {
        let evidence = owner(8)
            .compose(package(1, 1), package(2, 2), proposal(1, 2))
            .unwrap();
        let before = identities(
            evidence.earlier(),
            evidence.later(),
            evidence.delta_proposal(),
        );
        let (earlier, later, proposal) = evidence.into_parts();
        assert_eq!(identities(&earlier, &later, &proposal), before);
    }

    #[test]
    fn private_default_inert_non_applying_boundary_is_exact() {
        let source = include_str!("caller_requested_float32_comparative_advisory_evidence.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
            concat!("fn ro", "ute("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs")
            .contains("CallerRequestedFloat32ComparativeAdvisoryEvidence"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod caller_requested_float32_comparative_advisory_evidence"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_comparative_advisory_evidence"));
    }
}
