// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded transactional history of actual P44 package-delta proposals.
//!
//! This deliberately undeclared, crate-private, default-inert owner retains
//! proposals in insertion order by move, including both complete packages,
//! every fact, and every nested sample allocation. It is advisory and
//! non-applying: it infers no loss, continuity, or causality, claims no liblsl
//! equivalence, and grants no Manifold, session, stream, control, or device
//! authority.

use crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageDeltaProposal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32AdvisoryReportPackageDeltaHistoryConfigError {
    ZeroMaximumProposals,
    ZeroMaximumPackages,
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds {
    maximum_proposals: usize,
    maximum_proposals_u64: u64,
    maximum_packages: u64,
    maximum_facts: u64,
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds {
    pub(crate) fn new(
        maximum_proposals: usize,
        maximum_packages: usize,
        maximum_facts: usize,
    ) -> Result<Self, MorphospaceFloat32AdvisoryReportPackageDeltaHistoryConfigError> {
        use MorphospaceFloat32AdvisoryReportPackageDeltaHistoryConfigError::*;
        for (value, error) in [
            (maximum_proposals, ZeroMaximumProposals),
            (maximum_packages, ZeroMaximumPackages),
            (maximum_facts, ZeroMaximumFacts),
        ] {
            if value == 0 {
                return Err(error);
            }
        }
        Ok(Self {
            maximum_proposals,
            maximum_proposals_u64: u64::try_from(maximum_proposals).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_proposals,
                }
            })?,
            maximum_packages: u64::try_from(maximum_packages).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_packages,
                }
            })?,
            maximum_facts: u64::try_from(maximum_facts).map_err(|_| BoundUnrepresentable {
                requested: maximum_facts,
            })?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaHistoryTotals {
    proposal_count: u64,
    package_count: u64,
    fact_count: u64,
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaHistoryTotals {
    pub(crate) const fn proposal_count(&self) -> u64 {
        self.proposal_count
    }
    pub(crate) const fn package_count(&self) -> u64 {
        self.package_count
    }
    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32AdvisoryReportPackageDeltaHistory {
    bounds: MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds,
    proposals: Vec<MorphospaceFloat32AdvisoryReportPackageDeltaProposal>,
    totals: MorphospaceFloat32AdvisoryReportPackageDeltaHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32AdvisoryReportPackageDeltaHistoryAppendError {
    CollectionLengthOverflow {
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    ProposalLimit {
        limit: usize,
        required: usize,
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    ProposalCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    FactCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    CounterOverflow {
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    PackageLimit {
        limit: u64,
        required: u64,
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    FactLimit {
        limit: u64,
        required: u64,
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
    Allocation {
        requested_proposals: usize,
        history: MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    },
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) {
        use MorphospaceFloat32AdvisoryReportPackageDeltaHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, proposal }
            | ProposalLimit {
                history, proposal, ..
            }
            | ProposalCountUnrepresentable {
                history, proposal, ..
            }
            | FactCountUnrepresentable {
                history, proposal, ..
            }
            | CounterOverflow { history, proposal }
            | PackageLimit {
                history, proposal, ..
            }
            | FactLimit {
                history, proposal, ..
            }
            | Allocation {
                history, proposal, ..
            } => (history, proposal),
        }
    }
}

impl MorphospaceFloat32AdvisoryReportPackageDeltaHistory {
    pub(crate) fn new(bounds: MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds) -> Self {
        Self {
            bounds,
            proposals: Vec::new(),
            totals: Default::default(),
        }
    }
    pub(crate) fn proposals(&self) -> &[MorphospaceFloat32AdvisoryReportPackageDeltaProposal] {
        &self.proposals
    }
    pub(crate) const fn totals(&self) -> MorphospaceFloat32AdvisoryReportPackageDeltaHistoryTotals {
        self.totals
    }
    pub(crate) fn into_proposals(
        self,
    ) -> Vec<MorphospaceFloat32AdvisoryReportPackageDeltaProposal> {
        self.proposals
    }

    pub(crate) fn append(
        self,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) -> Result<Self, MorphospaceFloat32AdvisoryReportPackageDeltaHistoryAppendError> {
        self.append_with(
            proposal,
            |items, requested| items.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        proposal: MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
        reserve: R,
        mut to_u64: C,
        mut add_u64: U,
        add_usize: Z,
    ) -> Result<Self, MorphospaceFloat32AdvisoryReportPackageDeltaHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32AdvisoryReportPackageDeltaProposal>,
            usize,
        ) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        U: FnMut(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32AdvisoryReportPackageDeltaHistoryAppendError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history: self, proposal }) }; }
        let next_len = match add_usize(self.proposals.len(), 1) {
            Ok(v) => v,
            Err(()) => fail!(CollectionLengthOverflow {}),
        };
        if next_len > self.bounds.maximum_proposals {
            fail!(ProposalLimit {
                limit: self.bounds.maximum_proposals,
                required: next_len
            });
        }
        let converted_next_len = match to_u64(next_len) {
            Ok(v) => v,
            Err(()) => fail!(ProposalCountUnrepresentable { actual: next_len }),
        };
        if converted_next_len > self.bounds.maximum_proposals_u64 {
            fail!(ProposalLimit {
                limit: self.bounds.maximum_proposals,
                required: next_len
            });
        }
        let candidate_facts = match to_u64(proposal.facts().len()) {
            Ok(v) => v,
            Err(()) => fail!(FactCountUnrepresentable {
                actual: proposal.facts().len()
            }),
        };
        let proposal_count = match add_u64(self.totals.proposal_count, 1) {
            Ok(v) if v == converted_next_len => v,
            Ok(_) | Err(()) => fail!(CounterOverflow {}),
        };
        let package_count = match add_u64(self.totals.package_count, 2) {
            Ok(v) => v,
            Err(()) => fail!(CounterOverflow {}),
        };
        let fact_count = match add_u64(self.totals.fact_count, candidate_facts) {
            Ok(v) => v,
            Err(()) => fail!(CounterOverflow {}),
        };
        if package_count > self.bounds.maximum_packages {
            fail!(PackageLimit {
                limit: self.bounds.maximum_packages,
                required: package_count
            });
        }
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
        self.proposals.push(proposal);
        self.totals = MorphospaceFloat32AdvisoryReportPackageDeltaHistoryTotals {
            proposal_count,
            package_count,
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
    fn package(seed: u64) -> crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage{
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(evidence(seed, seed as f32))
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            evidence(seed + 1, seed as f32 + 1.0),
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
    fn proposal(seed: u64) -> MorphospaceFloat32AdvisoryReportPackageDeltaProposal {
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(package(seed), package(seed + 10))
        .unwrap()
    }
    fn pointers(value: &MorphospaceFloat32AdvisoryReportPackageDeltaProposal) -> Vec<*const f32> {
        [value.earlier(), value.later()]
            .into_iter()
            .flat_map(|p| {
                p.history()
                    .values()
                    .iter()
                    .map(|v| v.report().sample().sample().values().as_ptr())
                    .chain(std::iter::once(
                        p.summary()
                            .retained()
                            .report()
                            .sample()
                            .sample()
                            .values()
                            .as_ptr(),
                    ))
            })
            .collect()
    }
    fn bounds(
        proposals: usize,
        packages: usize,
        facts: usize,
    ) -> MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds {
        MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds::new(proposals, packages, facts)
            .unwrap()
    }

    #[test]
    fn zero_exact_and_one_past_bounds_are_atomic() {
        use MorphospaceFloat32AdvisoryReportPackageDeltaHistoryConfigError::*;
        assert_eq!(
            MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds::new(0, 1, 1),
            Err(ZeroMaximumProposals)
        );
        assert_eq!(
            MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds::new(1, 0, 1),
            Err(ZeroMaximumPackages)
        );
        assert_eq!(
            MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds::new(1, 1, 0),
            Err(ZeroMaximumFacts)
        );
        let exact = MorphospaceFloat32AdvisoryReportPackageDeltaHistory::new(bounds(1, 2, 4))
            .append(proposal(1))
            .unwrap();
        assert_eq!(
            exact.totals(),
            MorphospaceFloat32AdvisoryReportPackageDeltaHistoryTotals {
                proposal_count: 1,
                package_count: 2,
                fact_count: 4
            }
        );
        for bound in [bounds(1, 4, 8), bounds(2, 1, 8), bounds(2, 4, 3)] {
            let kept = proposal(20);
            let candidate = proposal(40);
            let candidate_ptrs = pointers(&candidate);
            let history = MorphospaceFloat32AdvisoryReportPackageDeltaHistory::new(bound)
                .append(kept)
                .unwrap_or_else(|e| e.into_parts().0);
            let (history, candidate) = history.append(candidate).unwrap_err().into_parts();
            assert_eq!(pointers(&candidate), candidate_ptrs);
            assert!(history.proposals().len() <= 1);
        }
    }

    #[test]
    fn repeated_append_order_allocation_identity_and_consuming_extraction_are_exact() {
        let inputs = [proposal(1), proposal(20), proposal(1)];
        let expected: Vec<_> = inputs
            .iter()
            .map(|p| (p.facts().as_ptr(), pointers(p)))
            .collect();
        let history = inputs.into_iter().fold(
            MorphospaceFloat32AdvisoryReportPackageDeltaHistory::new(bounds(3, 6, 12)),
            |h, p| h.append(p).unwrap(),
        );
        assert_eq!(history.totals().proposal_count(), 3);
        assert_eq!(history.totals().package_count(), 6);
        assert_eq!(history.totals().fact_count(), 12);
        for (actual, expected) in history.into_proposals().iter().zip(expected) {
            assert_eq!(actual.facts().as_ptr(), expected.0);
            assert_eq!(pointers(actual), expected.1);
        }
    }

    #[test]
    fn allocation_conversion_and_every_counter_overflow_roll_back_for_retry() {
        for failure in 0..7 {
            let candidate = proposal(50);
            let expected = pointers(&candidate);
            let mut history =
                MorphospaceFloat32AdvisoryReportPackageDeltaHistory::new(bounds(1, 2, 4));
            if failure == 3 {
                history.totals.proposal_count = u64::MAX;
            }
            if failure == 4 {
                history.totals.package_count = u64::MAX - 1;
            }
            if failure == 5 {
                history.totals.fact_count = u64::MAX - 3;
            }
            let mut conversion_call = 0;
            let error = history
                .append_with(
                    candidate,
                    |_, _| if failure == 6 { Err(()) } else { Ok(()) },
                    |value| {
                        conversion_call += 1;
                        if (failure == 1 && conversion_call == 1)
                            || (failure == 2 && conversion_call == 2)
                        {
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
            let (mut history, candidate) = error.into_parts();
            assert!(history.proposals().is_empty());
            assert_eq!(pointers(&candidate), expected);
            history.totals = Default::default();
            assert_eq!(
                pointers(
                    &history
                        .append(candidate)
                        .unwrap()
                        .into_proposals()
                        .pop()
                        .unwrap()
                ),
                expected
            );
        }
    }

    #[test]
    fn source_is_private_default_inert_non_applying_and_authority_free() {
        let source = include_str!("morphospace_float32_advisory_report_package_delta_history.rs");
        for wording in [
            "deliberately undeclared",
            "default-inert",
            "infers no loss, continuity, or causality",
            "no liblsl",
            "no Manifold, session, stream, control, or device",
        ] {
            assert!(source.contains(wording));
        }
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("lib.rs")
            .contains("pub mod morphospace_float32_advisory_report_package_delta_history;"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_advisory_report_package_delta_history"));
        assert!(!include_str!("runtime.rs")
            .contains("MorphospaceFloat32AdvisoryReportPackageDeltaHistory"));
    }
}
