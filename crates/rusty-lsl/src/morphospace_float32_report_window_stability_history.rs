// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded ownership of complete actual P39 Float32 stability proposals.
//!
//! This crate-private, default-inert owner retains caller-supplied proposals in
//! insertion order, including their P38 histories, windows, samples, reasons,
//! evidence, and original allocations. It neither infers loss nor claims
//! liblsl equivalence and grants no public, application, or Manifold authority.

use crate::morphospace_float32_report_window_stability_proposal::MorphospaceFloat32ReportWindowStabilityProposal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityHistoryConfigError {
    ZeroMaximumProposals,
    ZeroMaximumEvidencePerProposal,
    MaximumProposalsUnrepresentable { requested: usize },
    MaximumEvidencePerProposalUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowStabilityHistoryTotals {
    proposal_count: u64,
    evidence_count: u64,
}

impl MorphospaceFloat32ReportWindowStabilityHistoryTotals {
    pub(crate) const fn proposal_count(&self) -> u64 {
        self.proposal_count
    }

    pub(crate) const fn evidence_count(&self) -> u64 {
        self.evidence_count
    }

    fn checked_with(self, evidence_count: u64) -> Option<Self> {
        Some(Self {
            proposal_count: self.proposal_count.checked_add(1)?,
            evidence_count: self.evidence_count.checked_add(evidence_count)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowStabilityHistory {
    maximum_proposals: usize,
    maximum_evidence_per_proposal: usize,
    maximum_evidence_per_proposal_u64: u64,
    proposals: Vec<MorphospaceFloat32ReportWindowStabilityProposal>,
    totals: MorphospaceFloat32ReportWindowStabilityHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowStabilityHistoryAppendError {
    CollectionLengthOverflow {
        history: MorphospaceFloat32ReportWindowStabilityHistory,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    HistoryLimit {
        limit: usize,
        history: MorphospaceFloat32ReportWindowStabilityHistory,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    EvidenceCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32ReportWindowStabilityHistory,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    EvidenceLimit {
        limit: usize,
        actual: u64,
        history: MorphospaceFloat32ReportWindowStabilityHistory,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    CounterOverflow {
        history: MorphospaceFloat32ReportWindowStabilityHistory,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    Allocation {
        requested_proposals: usize,
        history: MorphospaceFloat32ReportWindowStabilityHistory,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
}

impl MorphospaceFloat32ReportWindowStabilityHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportWindowStabilityHistory,
        MorphospaceFloat32ReportWindowStabilityProposal,
    ) {
        match self {
            Self::CollectionLengthOverflow { history, proposal }
            | Self::HistoryLimit {
                history, proposal, ..
            }
            | Self::EvidenceCountUnrepresentable {
                history, proposal, ..
            }
            | Self::EvidenceLimit {
                history, proposal, ..
            }
            | Self::CounterOverflow { history, proposal }
            | Self::Allocation {
                history, proposal, ..
            } => (history, proposal),
        }
    }
}

impl MorphospaceFloat32ReportWindowStabilityHistory {
    pub(crate) fn new(
        maximum_proposals: usize,
        maximum_evidence_per_proposal: usize,
    ) -> Result<Self, MorphospaceFloat32ReportWindowStabilityHistoryConfigError> {
        use MorphospaceFloat32ReportWindowStabilityHistoryConfigError::*;
        if maximum_proposals == 0 {
            return Err(ZeroMaximumProposals);
        }
        if maximum_evidence_per_proposal == 0 {
            return Err(ZeroMaximumEvidencePerProposal);
        }
        u64::try_from(maximum_proposals).map_err(|_| MaximumProposalsUnrepresentable {
            requested: maximum_proposals,
        })?;
        let maximum_evidence_per_proposal_u64 = u64::try_from(maximum_evidence_per_proposal)
            .map_err(|_| MaximumEvidencePerProposalUnrepresentable {
                requested: maximum_evidence_per_proposal,
            })?;
        Ok(Self {
            maximum_proposals,
            maximum_evidence_per_proposal,
            maximum_evidence_per_proposal_u64,
            proposals: Vec::new(),
            totals: Default::default(),
        })
    }

    pub(crate) const fn maximum_proposals(&self) -> usize {
        self.maximum_proposals
    }

    pub(crate) const fn maximum_evidence_per_proposal(&self) -> usize {
        self.maximum_evidence_per_proposal
    }

    pub(crate) fn proposals(&self) -> &[MorphospaceFloat32ReportWindowStabilityProposal] {
        &self.proposals
    }

    pub(crate) const fn totals(&self) -> MorphospaceFloat32ReportWindowStabilityHistoryTotals {
        self.totals
    }

    pub(crate) fn into_proposals(self) -> Vec<MorphospaceFloat32ReportWindowStabilityProposal> {
        self.proposals
    }

    pub(crate) fn append(
        self,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    ) -> Result<Self, MorphospaceFloat32ReportWindowStabilityHistoryAppendError> {
        self.append_with(proposal, |proposals, requested| {
            proposals.try_reserve_exact(requested).map_err(|_| ())
        })
    }

    fn append_with<R>(
        mut self,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
        reserve: R,
    ) -> Result<Self, MorphospaceFloat32ReportWindowStabilityHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ReportWindowStabilityProposal>,
            usize,
        ) -> Result<(), ()>,
    {
        let Some(next_len) = self.proposals.len().checked_add(1) else {
            return Err(Self::failure_length(self, proposal));
        };
        if next_len > self.maximum_proposals {
            return Err(
                MorphospaceFloat32ReportWindowStabilityHistoryAppendError::HistoryLimit {
                    limit: self.maximum_proposals,
                    history: self,
                    proposal,
                },
            );
        }
        let evidence_usize = proposal.evidence().len();
        let evidence_count = match u64::try_from(evidence_usize) {
            Ok(value) => value,
            Err(_) => {
                return Err(MorphospaceFloat32ReportWindowStabilityHistoryAppendError::EvidenceCountUnrepresentable {
                    actual: evidence_usize,
                    history: self,
                    proposal,
                })
            }
        };
        if evidence_count > self.maximum_evidence_per_proposal_u64 {
            return Err(
                MorphospaceFloat32ReportWindowStabilityHistoryAppendError::EvidenceLimit {
                    limit: self.maximum_evidence_per_proposal,
                    actual: evidence_count,
                    history: self,
                    proposal,
                },
            );
        }
        let Some(totals) = self.totals.checked_with(evidence_count) else {
            return Err(
                MorphospaceFloat32ReportWindowStabilityHistoryAppendError::CounterOverflow {
                    history: self,
                    proposal,
                },
            );
        };
        if reserve(&mut self.proposals, 1).is_err() {
            return Err(
                MorphospaceFloat32ReportWindowStabilityHistoryAppendError::Allocation {
                    requested_proposals: 1,
                    history: self,
                    proposal,
                },
            );
        }
        self.proposals.push(proposal);
        self.totals = totals;
        Ok(self)
    }

    fn failure_length(
        history: Self,
        proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    ) -> MorphospaceFloat32ReportWindowStabilityHistoryAppendError {
        MorphospaceFloat32ReportWindowStabilityHistoryAppendError::CollectionLengthOverflow {
            history,
            proposal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_observation::{
        tests::outcome_with, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityEvidence,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn window(value: f32, sequence: u64) -> MorphospaceFloat32ReportObservationWindow {
        let record = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(value as f64 + 10.0).unwrap(),
            None,
        );
        let observation = MorphospaceFloat32ReportObservationOwner::new(1)
            .unwrap()
            .observe(outcome_with(
                vec![sequence],
                vec![record],
                RequestedTimestampPostProcessing::Monotonic(
                    RequestedTimestampPostProcessingConfig::new(8, 1.0, f64::MAX).unwrap(),
                ),
            ))
            .unwrap();
        MorphospaceFloat32ReportObservationWindow::new(1, 1)
            .unwrap()
            .append(observation)
            .unwrap()
    }

    fn proposal(
        values: &[(f32, u64)],
    ) -> (
        MorphospaceFloat32ReportWindowStabilityProposal,
        Vec<*const f32>,
    ) {
        let history = values.iter().fold(
            MorphospaceFloat32ReportObservationHistory::new(values.len(), 1).unwrap(),
            |history, (value, sequence)| history.append(window(*value, *sequence)).unwrap(),
        );
        let pointers = history
            .windows()
            .iter()
            .flat_map(|window| window.observations())
            .flat_map(|observation| observation.records())
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect();
        let evidence = values
            .len()
            .saturating_sub(1)
            .checked_mul(12)
            .unwrap()
            .max(1);
        let proposal = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(
                values.len(),
                1,
                1,
                evidence,
                u64::MAX,
                f64::MAX,
            )
            .unwrap(),
        )
        .propose(history)
        .unwrap();
        (proposal, pointers)
    }

    fn extracted(proposal: MorphospaceFloat32ReportWindowStabilityProposal) -> Vec<*const f32> {
        proposal
            .into_history()
            .into_windows()
            .iter()
            .flat_map(|window| window.observations())
            .flat_map(|observation| observation.records())
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect()
    }

    #[test]
    fn nonzero_bounds_capacity_and_platform_extremes_are_explicit() {
        use MorphospaceFloat32ReportWindowStabilityHistoryConfigError::*;
        assert_eq!(
            MorphospaceFloat32ReportWindowStabilityHistory::new(0, 1),
            Err(ZeroMaximumProposals)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowStabilityHistory::new(1, 0),
            Err(ZeroMaximumEvidencePerProposal)
        );
        let history =
            MorphospaceFloat32ReportWindowStabilityHistory::new(usize::MAX, usize::MAX).unwrap();
        assert_eq!(history.maximum_proposals(), usize::MAX);
        assert_eq!(history.maximum_evidence_per_proposal(), usize::MAX);
        assert_eq!(history.totals(), Default::default());
    }

    #[test]
    fn first_middle_final_and_repeated_actual_proposals_keep_order_reasons_and_allocations() {
        let inputs = [
            proposal(&[(1.0, 1), (2.0, 2)]),
            proposal(&[(3.0, 7), (3.0, 7)]),
            proposal(&[(9.0, 9), (1.0, 1)]),
            proposal(&[(1.0, 1), (2.0, 2)]),
        ];
        let expected: Vec<_> = inputs
            .iter()
            .map(|(proposal, pointers)| {
                (
                    proposal.is_stable(),
                    proposal.evidence().to_vec(),
                    pointers.clone(),
                )
            })
            .collect();
        let history = inputs.into_iter().map(|(proposal, _)| proposal).fold(
            MorphospaceFloat32ReportWindowStabilityHistory::new(4, 12).unwrap(),
            |history, proposal| history.append(proposal).unwrap(),
        );
        assert_eq!(history.totals().proposal_count(), 4);
        assert_eq!(history.totals().evidence_count(), 48);
        for (actual, (stable, evidence, _)) in history.proposals().iter().zip(&expected) {
            assert_eq!(actual.is_stable(), *stable);
            assert_eq!(actual.evidence(), evidence);
            assert!(matches!(
                actual.evidence().first(),
                Some(MorphospaceFloat32ReportWindowStabilityEvidence::Counter { .. })
            ));
        }
        for (actual, (_, _, pointers)) in history.into_proposals().into_iter().zip(expected) {
            assert_eq!(extracted(actual), pointers);
        }
    }

    #[test]
    fn capacity_evidence_and_allocation_failures_roll_back_and_retry_unchanged() {
        let (kept, kept_pointers) = proposal(&[(1.0, 1), (2.0, 2)]);
        let (candidate, candidate_pointers) = proposal(&[(4.0, 4), (5.0, 5)]);
        let history = MorphospaceFloat32ReportWindowStabilityHistory::new(1, 12)
            .unwrap()
            .append(kept)
            .unwrap();
        let before = history.totals();
        let (history, candidate) = history.append(candidate).unwrap_err().into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(
            extracted(history.into_proposals().pop().unwrap()),
            kept_pointers
        );
        assert_eq!(
            extracted(
                MorphospaceFloat32ReportWindowStabilityHistory::new(1, 12)
                    .unwrap()
                    .append(candidate)
                    .unwrap()
                    .into_proposals()
                    .pop()
                    .unwrap()
            ),
            candidate_pointers
        );

        let (candidate, pointers) = proposal(&[(6.0, 6), (7.0, 7)]);
        let (history, candidate) = MorphospaceFloat32ReportWindowStabilityHistory::new(1, 11)
            .unwrap()
            .append(candidate)
            .unwrap_err()
            .into_parts();
        assert!(history.proposals().is_empty());
        let failure = MorphospaceFloat32ReportWindowStabilityHistory::new(1, 12)
            .unwrap()
            .append_with(candidate, |_, requested| {
                assert_eq!(requested, 1);
                Err(())
            })
            .unwrap_err();
        let (history, candidate) = failure.into_parts();
        assert_eq!(history.totals(), Default::default());
        assert_eq!(
            extracted(
                history
                    .append(candidate)
                    .unwrap()
                    .into_proposals()
                    .pop()
                    .unwrap()
            ),
            pointers
        );
    }

    #[test]
    fn checked_counter_overflow_returns_both_live_owners_for_ordered_retry() {
        for set in [
            |totals: &mut MorphospaceFloat32ReportWindowStabilityHistoryTotals| {
                totals.proposal_count = u64::MAX
            },
            |totals: &mut MorphospaceFloat32ReportWindowStabilityHistoryTotals| {
                totals.evidence_count = u64::MAX - 11
            },
        ] {
            let (candidate, pointers) = proposal(&[(8.0, 8), (9.0, 9)]);
            let mut history = MorphospaceFloat32ReportWindowStabilityHistory::new(1, 12).unwrap();
            set(&mut history.totals);
            let before = history.totals();
            let (mut history, candidate) = history.append(candidate).unwrap_err().into_parts();
            assert_eq!(history.totals(), before);
            history.totals = Default::default();
            assert_eq!(
                extracted(
                    history
                        .append(candidate)
                        .unwrap()
                        .into_proposals()
                        .pop()
                        .unwrap()
                ),
                pointers
            );
        }
    }

    #[test]
    fn owner_is_crate_private_default_inert_and_denies_external_authority() {
        let source = include_str!("morphospace_float32_report_window_stability_history.rs");
        for forbidden in [
            concat!("pub ", "struct"),
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(
            !include_str!("runtime.rs").contains("MorphospaceFloat32ReportWindowStabilityHistory")
        );
    }
}
