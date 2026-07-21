// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exact bounded delta proposal over two actual P45 comparative evidence values.
//!
//! This module is deliberately undeclared. It retains both inputs unchanged,
//! derives only exact count relations from their existing totals and evidence,
//! and cannot apply the resulting advisory proposal.

use crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence;
use crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageRelation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaConfigError {
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds {
    maximum_facts: usize,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds {
    pub(crate) fn new(
        maximum_facts: usize,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaConfigError> {
        if maximum_facts == 0 {
            return Err(
                MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaConfigError::ZeroMaximumFacts,
            );
        }
        u64::try_from(maximum_facts).map_err(|_| {
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaConfigError::BoundUnrepresentable {
                requested: maximum_facts,
            }
        })?;
        Ok(Self { maximum_facts })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceCount {
    Facts,
    EqualRelations,
    IncreaseRelations,
    DecreaseRelations,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact {
    count: MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
    earlier: u64,
    later: u64,
    relation: MorphospaceFloat32AdvisoryReportPackageRelation,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact {
    pub(crate) const fn count(&self) -> MorphospaceFloat32ComparativeAdvisoryEvidenceCount {
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
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal {
    earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    fact_count: u64,
    facts: Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact>,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal {
    pub(crate) const fn earlier(&self) -> &CallerRequestedFloat32ComparativeAdvisoryEvidence {
        &self.earlier
    }
    pub(crate) const fn later(&self) -> &CallerRequestedFloat32ComparativeAdvisoryEvidence {
        &self.later
    }
    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
    pub(crate) fn facts(&self) -> &[MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact] {
        &self.facts
    }
    pub(crate) fn into_evidence(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
    ) {
        (self.earlier, self.later)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError {
    FactCountOverflow {
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    FactCountUnrepresentable {
        actual: usize,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    FactLimit {
        limit: usize,
        required: usize,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    EvidenceCountOverflow {
        side: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaSide,
        count: MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    DifferenceOverflow {
        count: MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    Allocation {
        requested: usize,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaSide {
    Earlier,
    Later,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError {
    pub(crate) fn into_evidence(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
    ) {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError::*;
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
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner {
    bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner {
    pub(crate) const fn new(
        bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn propose(
        &self,
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    ) -> Result<
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError,
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
        earlier: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        later: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        reserve: R,
        convert: C,
        mut add: A,
        mut subtract: S,
    ) -> Result<
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError,
    >
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        A: FnMut(u64, u64) -> Result<u64, ()>,
        S: FnMut(u64, u64) -> Result<u64, ()>,
    {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* earlier, later }) }; }

        let required = 4usize;
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
        if add(0, fact_count).is_err() {
            fail!(FactCountOverflow {});
        }

        let collect = |evidence: &CallerRequestedFloat32ComparativeAdvisoryEvidence,
                       side,
                       add: &mut A|
         -> Result<
            [u64; 4],
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaSide,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
            ),
        > {
            let mut totals = [evidence.fact_count(), 0, 0, 0];
            for fact in evidence.facts() {
                let (index, count) = match fact.relation() {
                    MorphospaceFloat32AdvisoryReportPackageRelation::Equal => (
                        1,
                        MorphospaceFloat32ComparativeAdvisoryEvidenceCount::EqualRelations,
                    ),
                    MorphospaceFloat32AdvisoryReportPackageRelation::Increase { .. } => (
                        2,
                        MorphospaceFloat32ComparativeAdvisoryEvidenceCount::IncreaseRelations,
                    ),
                    MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { .. } => (
                        3,
                        MorphospaceFloat32ComparativeAdvisoryEvidenceCount::DecreaseRelations,
                    ),
                };
                totals[index] = add(totals[index], 1).map_err(|_| (side, count))?;
            }
            Ok(totals)
        };
        let earlier_totals = match collect(
            &earlier,
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaSide::Earlier,
            &mut add,
        ) {
            Ok(value) => value,
            Err((side, count)) => fail!(EvidenceCountOverflow {
                side: side,
                count: count
            }),
        };
        let later_totals = match collect(
            &later,
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaSide::Later,
            &mut add,
        ) {
            Ok(value) => value,
            Err((side, count)) => fail!(EvidenceCountOverflow {
                side: side,
                count: count
            }),
        };
        let counts = [
            MorphospaceFloat32ComparativeAdvisoryEvidenceCount::Facts,
            MorphospaceFloat32ComparativeAdvisoryEvidenceCount::EqualRelations,
            MorphospaceFloat32ComparativeAdvisoryEvidenceCount::IncreaseRelations,
            MorphospaceFloat32ComparativeAdvisoryEvidenceCount::DecreaseRelations,
        ];
        let mut relations = [MorphospaceFloat32AdvisoryReportPackageRelation::Equal; 4];
        for index in 0..4 {
            relations[index] = if later_totals[index] > earlier_totals[index] {
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase {
                    amount: match subtract(later_totals[index], earlier_totals[index]) {
                        Ok(value) => value,
                        Err(()) => fail!(DifferenceOverflow {
                            count: counts[index]
                        }),
                    },
                }
            } else if later_totals[index] < earlier_totals[index] {
                MorphospaceFloat32AdvisoryReportPackageRelation::Decrease {
                    amount: match subtract(earlier_totals[index], later_totals[index]) {
                        Ok(value) => value,
                        Err(()) => fail!(DifferenceOverflow {
                            count: counts[index]
                        }),
                    },
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
        facts.extend((0..4).map(
            |index| MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact {
                count: counts[index],
                earlier: earlier_totals[index],
                later: later_totals[index],
                relation: relations[index],
            },
        ));
        Ok(MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal {
            earlier,
            later,
            fact_count,
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
    use crate::caller_requested_float32_comparative_advisory_evidence::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner,
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

    fn report_evidence(sequence: u64) -> crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidence{
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
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![sequence as f32]).unwrap(),
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

    fn package(history_count: usize, base: u64) -> crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage{
        let mut history =
            CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(history_count.max(1)).unwrap();
        for index in 0..history_count {
            history = history
                .append(report_evidence(base + index as u64))
                .unwrap();
        }
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            report_evidence(base + 20),
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
                    .and_then(|value| value.checked_add(2))
                    .unwrap()
                    .max(2),
            )
            .unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }

    fn comparative(
        package_history: usize,
        proposal_earlier: usize,
        proposal_later: usize,
        base: u64,
    ) -> CallerRequestedFloat32ComparativeAdvisoryEvidence {
        let delta = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(
            package(proposal_earlier, base + 100),
            package(proposal_later, base + 200),
        )
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
        )
        .compose(
            package(package_history, base),
            package(package_history, base + 10),
            delta,
        )
        .unwrap()
    }

    fn pointers(value: &CallerRequestedFloat32ComparativeAdvisoryEvidence) -> (Vec<*const f32>, Vec<*const f32>, *const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidenceFact){
        let package_pointers = |package: &crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage| {
            package.history().values().iter().map(|item| item.report().sample().sample().values().as_ptr())
                .chain(std::iter::once(package.summary().retained().report().sample().sample().values().as_ptr())).collect::<Vec<_>>()
        };
        (
            package_pointers(value.earlier()),
            package_pointers(value.later()),
            value.facts().as_ptr(),
        )
    }

    fn owner(limit: usize) -> MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner {
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(limit).unwrap(),
        )
    }

    #[test]
    fn bounds_and_platform_conversion_boundary_are_explicit() {
        assert_eq!(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(0),
            Err(MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaConfigError::ZeroMaximumFacts)
        );
        assert!(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(usize::MAX).is_ok()
                || usize::BITS > u64::BITS
        );
        assert_eq!(
            owner(4)
                .propose(comparative(1, 1, 1, 0), comparative(1, 1, 1, 1000))
                .unwrap()
                .fact_count(),
            4
        );
        assert!(matches!(
            owner(3).propose(comparative(1, 1, 1, 0), comparative(1, 1, 1, 1000)),
            Err(
                MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError::FactLimit {
                    limit: 3,
                    required: 4,
                    ..
                }
            )
        ));
    }

    #[test]
    fn relations_order_ties_and_exact_existing_counts_are_deterministic() {
        let proposal = owner(4)
            .propose(comparative(2, 1, 2, 0), comparative(1, 1, 2, 1000))
            .unwrap();
        assert_eq!(
            proposal
                .facts()
                .iter()
                .map(|fact| fact.count())
                .collect::<Vec<_>>(),
            vec![
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::Facts,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::EqualRelations,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::IncreaseRelations,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::DecreaseRelations,
            ]
        );
        assert_eq!(
            (
                proposal.facts()[0].earlier(),
                proposal.facts()[0].later(),
                proposal.facts()[0].relation()
            ),
            (8, 8, MorphospaceFloat32AdvisoryReportPackageRelation::Equal)
        );
        assert!(proposal.facts().iter().any(|fact| matches!(
            fact.relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { .. }
        )));
        assert!(proposal.facts().iter().any(|fact| matches!(
            fact.relation(),
            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { .. }
        )));
        for (side, evidence) in [(0, proposal.earlier()), (1, proposal.later())] {
            let expected = [
                evidence.fact_count(),
                evidence
                    .facts()
                    .iter()
                    .filter(|fact| {
                        matches!(
                            fact.relation(),
                            MorphospaceFloat32AdvisoryReportPackageRelation::Equal
                        )
                    })
                    .count() as u64,
                evidence
                    .facts()
                    .iter()
                    .filter(|fact| {
                        matches!(
                            fact.relation(),
                            MorphospaceFloat32AdvisoryReportPackageRelation::Increase { .. }
                        )
                    })
                    .count() as u64,
                evidence
                    .facts()
                    .iter()
                    .filter(|fact| {
                        matches!(
                            fact.relation(),
                            MorphospaceFloat32AdvisoryReportPackageRelation::Decrease { .. }
                        )
                    })
                    .count() as u64,
            ];
            for (index, value) in expected.into_iter().enumerate() {
                assert_eq!(
                    if side == 0 {
                        proposal.facts()[index].earlier()
                    } else {
                        proposal.facts()[index].later()
                    },
                    value
                );
            }
        }
    }

    fn assert_returned(
        error: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaError,
        before_a: (Vec<*const f32>, Vec<*const f32>, *const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidenceFact),
        before_b: (Vec<*const f32>, Vec<*const f32>, *const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidenceFact),
    ) {
        let (a, b) = error.into_evidence();
        assert_eq!(pointers(&a), before_a);
        assert_eq!(pointers(&b), before_b);
    }

    #[test]
    fn allocation_conversion_arithmetic_difference_and_limit_failures_roll_back() {
        for failure in 0..5 {
            let a = comparative(2, 1, 2, 0);
            let b = comparative(1, 1, 2, 1000);
            let before_a = pointers(&a);
            let before_b = pointers(&b);
            let mut additions = 0;
            let error = if failure == 4 {
                owner(3).propose(a, b).unwrap_err()
            } else {
                owner(4)
                    .propose_with(
                        a,
                        b,
                        |_, _| if failure == 0 { Err(()) } else { Ok(()) },
                        |value| {
                            if failure == 1 {
                                Err(())
                            } else {
                                Ok(value as u64)
                            }
                        },
                        |left, right| {
                            additions += 1;
                            if failure == 2 && additions > 1 {
                                Err(())
                            } else {
                                left.checked_add(right).ok_or(())
                            }
                        },
                        |left, right| {
                            if failure == 3 {
                                Err(())
                            } else {
                                left.checked_sub(right).ok_or(())
                            }
                        },
                    )
                    .unwrap_err()
            };
            assert_returned(error, before_a, before_b);
        }
    }

    #[test]
    fn success_and_consuming_extraction_preserve_both_inputs_and_allocations() {
        let a = comparative(2, 1, 2, 0);
        let b = comparative(1, 1, 2, 1000);
        let before_a = pointers(&a);
        let before_b = pointers(&b);
        let proposal = owner(4).propose(a, b).unwrap();
        assert_eq!(pointers(proposal.earlier()), before_a);
        assert_eq!(pointers(proposal.later()), before_b);
        let (a, b) = proposal.into_evidence();
        assert_eq!(pointers(&a), before_a);
        assert_eq!(pointers(&b), before_b);
    }

    #[test]
    fn private_default_inert_non_applying_boundary_is_exact() {
        let source =
            include_str!("morphospace_float32_comparative_advisory_evidence_delta_proposal.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
            concat!("fn ro", "ute("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs")
            .contains("MorphospaceFloat32ComparativeAdvisoryEvidenceDelta"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod morphospace_float32_comparative_advisory_evidence_delta_proposal"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_comparative_advisory_evidence_delta_proposal"));
    }
}
