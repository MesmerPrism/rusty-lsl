// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded transactional history of actual P46 comparative evidence deltas.
//!
//! This crate-private, caller-requested and default-inert container retains
//! proposals in exact insertion order without cloning or reconstructing their
//! evidence. It infers neither loss nor continuity, applies no proposal, makes
//! no liblsl-equivalence claim, and grants no Manifold, session, stream,
//! transport, control, or runtime authority.

use crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryConfigError {
    ZeroMaximumProposals,
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds {
    maximum_proposals: usize,
    maximum_proposals_u64: u64,
    maximum_facts: u64,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds {
    pub(crate) fn new(
        maximum_proposals: usize,
        maximum_facts: usize,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryConfigError> {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryConfigError::*;
        if maximum_proposals == 0 {
            return Err(ZeroMaximumProposals);
        }
        if maximum_facts == 0 {
            return Err(ZeroMaximumFacts);
        }
        Ok(Self {
            maximum_proposals,
            maximum_proposals_u64: u64::try_from(maximum_proposals).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_proposals,
                }
            })?,
            maximum_facts: u64::try_from(maximum_facts).map_err(|_| BoundUnrepresentable {
                requested: maximum_facts,
            })?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryTotals {
    proposal_count: u64,
    fact_count: u64,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryTotals {
    pub(crate) const fn proposal_count(&self) -> u64 {
        self.proposal_count
    }

    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory {
    bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds,
    proposals: Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal>,
    totals: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError {
    CollectionLengthOverflow {
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    ProposalLimit {
        limit: usize,
        required: usize,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    ProposalCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    FactCountOverflow {
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    FactLimit {
        limit: u64,
        required: u64,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
    Allocation {
        requested_proposals: usize,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    },
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    ) {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, candidate }
            | ProposalLimit {
                history, candidate, ..
            }
            | ProposalCountUnrepresentable {
                history, candidate, ..
            }
            | FactCountOverflow { history, candidate }
            | FactLimit {
                history, candidate, ..
            }
            | Allocation {
                history, candidate, ..
            } => (history, candidate),
        }
    }
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory {
    pub(crate) fn new(
        bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds,
    ) -> Self {
        Self {
            bounds,
            proposals: Vec::new(),
            totals: Default::default(),
        }
    }

    pub(crate) fn proposals(
        &self,
    ) -> &[MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal] {
        &self.proposals
    }

    pub(crate) const fn totals(
        &self,
    ) -> MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryTotals {
        self.totals
    }

    pub(crate) fn into_proposals(
        self,
    ) -> Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal> {
        self.proposals
    }

    pub(crate) fn append(
        self,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError> {
        self.append_with(
            candidate,
            |values, requested| values.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal,
        reserve: R,
        to_u64: C,
        add_u64: U,
        add_usize: Z,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnOnce(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError::*;
        macro_rules! fail {
            ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => {
                return Err($variant { $($field: $value,)* history: self, candidate })
            };
        }

        let next_len = match add_usize(self.proposals.len(), 1) {
            Ok(value) => value,
            Err(()) => fail!(CollectionLengthOverflow {}),
        };
        if next_len > self.bounds.maximum_proposals {
            fail!(ProposalLimit {
                limit: self.bounds.maximum_proposals,
                required: next_len
            });
        }
        let proposal_count = match to_u64(next_len) {
            Ok(value) => value,
            Err(()) => fail!(ProposalCountUnrepresentable { actual: next_len }),
        };
        if proposal_count > self.bounds.maximum_proposals_u64 {
            fail!(ProposalLimit {
                limit: self.bounds.maximum_proposals,
                required: next_len
            });
        }
        let fact_count = match add_u64(self.totals.fact_count, candidate.fact_count()) {
            Ok(value) => value,
            Err(()) => fail!(FactCountOverflow {}),
        };
        if fact_count > self.bounds.maximum_facts {
            fail!(FactLimit {
                limit: self.bounds.maximum_facts,
                required: fact_count
            });
        }
        if reserve(&mut self.proposals, 1).is_err() {
            fail!(Allocation {
                requested_proposals: 1
            });
        }
        self.proposals.push(candidate);
        self.totals = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryTotals {
            proposal_count,
            fact_count,
        };
        Ok(self)
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

    fn package(seed: u64) -> crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage{
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(report_evidence(seed))
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            report_evidence(seed + 1),
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                .unwrap()
                .append(snapshot())
                .unwrap(),
        )
        .unwrap();
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 2, 4).unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }

    fn evidence(seed: u64) -> crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence{
        let delta = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(package(seed + 20), package(seed + 30))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
        )
        .compose(package(seed), package(seed + 10), delta)
        .unwrap()
    }

    fn proposal(seed: u64) -> MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal {
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(evidence(seed), evidence(seed + 100))
        .unwrap()
    }

    fn identity(value: &MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposal) -> (*const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidenceFact, *const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidenceFact, *const crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact){
        (
            value.earlier().facts().as_ptr(),
            value.later().facts().as_ptr(),
            value.facts().as_ptr(),
        )
    }

    fn bounds(
        proposals: usize,
        facts: usize,
    ) -> MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds {
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds::new(proposals, facts)
            .unwrap()
    }

    #[test]
    fn zero_bounds_and_capacity_boundary_return_exact_owners() {
        assert_eq!(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds::new(0, 1),
            Err(MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryConfigError::ZeroMaximumProposals)
        );
        assert_eq!(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds::new(1, 0),
            Err(MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryConfigError::ZeroMaximumFacts)
        );
        let history = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory::new(bounds(1, 8))
            .append(proposal(1))
            .unwrap();
        let candidate = proposal(2);
        let candidate_id = identity(&candidate);
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError::ProposalLimit {
                limit: 1,
                required: 2,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals().proposal_count(), 1);
        assert_eq!(identity(&candidate), candidate_id);
    }

    #[test]
    fn repeated_equal_evidence_keeps_deterministic_order_ties_and_allocations() {
        let values = [proposal(10), proposal(10), proposal(20)];
        let ids: Vec<_> = values.iter().map(identity).collect();
        let history = values.into_iter().fold(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory::new(bounds(3, 12)),
            |history, value| history.append(value).unwrap(),
        );
        assert_eq!(history.totals().proposal_count(), 3);
        assert_eq!(history.totals().fact_count(), 12);
        assert_eq!(
            history.proposals().iter().map(identity).collect::<Vec<_>>(),
            ids
        );
        assert_eq!(history.proposals()[0], history.proposals()[1]);
        assert_eq!(
            history
                .into_proposals()
                .iter()
                .map(identity)
                .collect::<Vec<_>>(),
            ids
        );
    }

    #[test]
    fn usize_and_u64_extremes_are_checked() {
        assert!(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds::new(
                usize::MAX,
                usize::MAX
            )
            .is_ok()
                || usize::BITS > u64::BITS
        );
        let candidate = proposal(30);
        let id = identity(&candidate);
        let error = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory::new(bounds(1, 3))
            .append(candidate)
            .unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryAppendError::FactLimit {
                limit: 3,
                required: 4,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert!(history.proposals().is_empty());
        assert_eq!(identity(&candidate), id);
    }

    #[test]
    fn every_injected_failure_rolls_back_history_candidate_and_totals() {
        for failure in 0..4 {
            let kept = proposal(40);
            let kept_id = identity(&kept);
            let candidate = proposal(50 + failure);
            let candidate_id = identity(&candidate);
            let mut history =
                MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory::new(bounds(2, 16))
                    .append(kept)
                    .unwrap();
            let before = history.totals();
            if failure == 2 {
                history.totals.fact_count = u64::MAX;
            }
            let error = history
                .append_with(
                    candidate,
                    |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                    |value| {
                        if failure == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |left, right| left.checked_add(right).ok_or(()),
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
            assert_eq!(identity(&history.proposals()[0]), kept_id);
            assert_eq!(identity(&candidate), candidate_id);
            assert_eq!(history.totals().proposal_count(), before.proposal_count());
            assert_eq!(
                history.totals().fact_count(),
                if failure == 2 {
                    u64::MAX
                } else {
                    before.fact_count()
                }
            );
        }
    }

    #[test]
    fn boundary_is_crate_private_default_inert_non_applying_and_non_authoritative() {
        let source =
            include_str!("morphospace_float32_comparative_advisory_evidence_delta_history.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
            concat!("fn ro", "ute("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs")
            .contains("MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod morphospace_float32_comparative_advisory_evidence_delta_history"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_comparative_advisory_evidence_delta_history"));
    }
}
