// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested retained package over actual P48 comparison owners.
//!
//! This crate-private package is default-inert, advisory, and non-applying. It
//! retains one complete snapshot history and one complete snapshot delta
//! proposal without cloning their nested allocations. Its compact summary
//! records only exact existing indexes and counts in fixed source order. It
//! infers no loss, continuity, or causality and grants no liblsl-equivalence,
//! runtime, activation, Manifold, session, transport, or control authority.

use crate::caller_requested_float32_comparative_advisory_evidence_snapshot_history::CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory;
use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_proposal::{
    MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount,
    MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotPackageConfigError {
    ZeroMaximumHistorySnapshots,
    ZeroMaximumDeltaFacts,
    ZeroMaximumSummaryEntries,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotPackageBounds {
    maximum_history_snapshots: usize,
    maximum_delta_facts: usize,
    maximum_summary_entries: usize,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotPackageBounds {
    pub(crate) fn new(
        maximum_history_snapshots: usize,
        maximum_delta_facts: usize,
        maximum_summary_entries: usize,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotPackageConfigError> {
        use CallerRequestedFloat32RetainedComparativeSnapshotPackageConfigError::*;
        for (value, zero) in [
            (maximum_history_snapshots, ZeroMaximumHistorySnapshots),
            (maximum_delta_facts, ZeroMaximumDeltaFacts),
            (maximum_summary_entries, ZeroMaximumSummaryEntries),
        ] {
            if value == 0 {
                return Err(zero);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_history_snapshots,
            maximum_delta_facts,
            maximum_summary_entries,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry {
    HistorySnapshot {
        snapshot_index: u64,
        observation_count: u64,
    },
    DeltaFact {
        fact_index: u64,
        count: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount,
        earlier: u64,
        later: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotPackage {
    history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
    delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    summary_count: u64,
    summary: Vec<CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry>,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotPackage {
    pub(crate) const fn history(
        &self,
    ) -> &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory {
        &self.history
    }

    pub(crate) const fn delta_proposal(
        &self,
    ) -> &MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal {
        &self.delta_proposal
    }

    pub(crate) const fn summary_count(&self) -> u64 {
        self.summary_count
    }

    pub(crate) fn summary(
        &self,
    ) -> &[CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry] {
        &self.summary
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    ) {
        (self.history, self.delta_proposal)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotPackageError {
    HistorySnapshotLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    DeltaFactLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    SummaryCountOverflow {
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    SummaryLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    CountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    IndexUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
    Allocation {
        requested: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    },
}

impl CallerRequestedFloat32RetainedComparativeSnapshotPackageError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    ) {
        use CallerRequestedFloat32RetainedComparativeSnapshotPackageError::*;
        match self {
            HistorySnapshotLimit {
                history,
                delta_proposal,
                ..
            }
            | DeltaFactLimit {
                history,
                delta_proposal,
                ..
            }
            | SummaryCountOverflow {
                history,
                delta_proposal,
            }
            | SummaryLimit {
                history,
                delta_proposal,
                ..
            }
            | CountUnrepresentable {
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
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotPackageOwner {
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotPackageBounds,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotPackageOwner {
    pub(crate) const fn new(
        bounds: CallerRequestedFloat32RetainedComparativeSnapshotPackageBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn package(
        &self,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotPackage,
        CallerRequestedFloat32RetainedComparativeSnapshotPackageError,
    > {
        self.package_with(
            history,
            delta_proposal,
            |summary, count| summary.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
        )
    }

    fn package_with<R, C, A>(
        &self,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        delta_proposal: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposal,
        reserve: R,
        mut convert: C,
        add: A,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotPackage,
        CallerRequestedFloat32RetainedComparativeSnapshotPackageError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry>,
            usize,
        ) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        A: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32RetainedComparativeSnapshotPackageError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history, delta_proposal }) }; }

        let history_count = history.snapshots().len();
        let delta_count = delta_proposal.facts().len();
        if history_count > self.bounds.maximum_history_snapshots {
            fail!(HistorySnapshotLimit {
                limit: self.bounds.maximum_history_snapshots,
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
            Err(()) => fail!(SummaryCountOverflow {}),
        };
        if required > self.bounds.maximum_summary_entries {
            fail!(SummaryLimit {
                limit: self.bounds.maximum_summary_entries,
                required: required
            });
        }
        let summary_count = match convert(required) {
            Ok(value) => value,
            Err(()) => fail!(CountUnrepresentable { actual: required }),
        };

        // Convert every emitted index before allocating, so conversion failure
        // cannot leave a partial summary or disturb either retained owner.
        for index in 0..history_count {
            if convert(index).is_err() {
                fail!(IndexUnrepresentable { actual: index });
            }
        }
        for index in 0..delta_count {
            if convert(index).is_err() {
                fail!(IndexUnrepresentable { actual: index });
            }
        }

        let mut summary = Vec::new();
        if reserve(&mut summary, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for (index, snapshot) in history.snapshots().iter().enumerate() {
            summary.push(
                CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry::HistorySnapshot {
                    snapshot_index: u64::try_from(index).expect("index conversion preflighted"),
                    observation_count: snapshot.observation_count(),
                },
            );
        }
        for (index, fact) in delta_proposal.facts().iter().enumerate() {
            summary.push(
                CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry::DeltaFact {
                    fact_index: u64::try_from(index).expect("index conversion preflighted"),
                    count: fact.count(),
                    earlier: fact.earlier(),
                    later: fact.later(),
                },
            );
        }
        Ok(CallerRequestedFloat32RetainedComparativeSnapshotPackage {
            history,
            delta_proposal,
            summary_count,
            summary,
        })
    }
}
