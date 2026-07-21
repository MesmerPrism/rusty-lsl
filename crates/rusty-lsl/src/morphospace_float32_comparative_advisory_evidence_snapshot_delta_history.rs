// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded retained history of actual P48 snapshot-delta proposals.
//!
//! This crate-private, caller-requested, default-inert collection retains each
//! complete proposal by move in exact append order, including equal proposals
//! and all nested allocations. It infers no loss, continuity, or causality,
//! claims no liblsl equivalence, applies or activates nothing, and grants no
//! Manifold, session, transport, control, runtime, or other authority.

use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryConfigError {
    ZeroMaximumProposals,
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryBounds {
    maximum_proposals: usize,
    maximum_proposals_u64: u64,
    maximum_facts: u64,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryBounds {
    pub(crate) fn new(
        maximum_proposals: usize,
        maximum_facts: usize,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryConfigError>
    {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryConfigError::*;
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
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryTotals {
    proposal_count: u64,
    fact_count: u64,
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryTotals {
    pub(crate) const fn proposal_count(&self) -> u64 {
        self.proposal_count
    }

    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory {
    bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryBounds,
    proposals: Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal>,
    totals: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryAppendError {
    CollectionLengthOverflow {
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    ProposalLimit {
        limit: usize,
        required: usize,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    ProposalCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    FactCountOverflow {
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    FactLimit {
        limit: u64,
        required: u64,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    Allocation {
        requested_proposals: usize,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
}

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    ) {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryAppendError::*;
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

impl MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory {
    pub(crate) fn new(
        bounds: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryBounds,
    ) -> Self {
        Self {
            bounds,
            proposals: Vec::new(),
            totals: Default::default(),
        }
    }

    pub(crate) fn proposals(
        &self,
    ) -> &[MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal] {
        &self.proposals
    }

    pub(crate) const fn totals(
        &self,
    ) -> MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryTotals {
        self.totals
    }

    pub(crate) fn into_proposals(
        self,
    ) -> Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal> {
        self.proposals
    }

    pub(crate) fn append(
        self,
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryAppendError>
    {
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
        candidate: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
        reserve: R,
        to_u64: C,
        add_u64: U,
        add_usize: Z,
    ) -> Result<Self, MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnOnce(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryAppendError::*;
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
        self.totals = MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryTotals {
            proposal_count,
            fact_count,
        };
        Ok(self)
    }
}
