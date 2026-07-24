// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested report over actual P49 retained comparison owners.
//!
//! This substantive crate-private owner is default-inert, advisory, and
//! non-applying. It retains one complete snapshot-delta history and one
//! complete retained comparison package without cloning or reconstructing
//! their nested allocations. Its bounded index copies only deterministic
//! existing indexes and exact counts. It infers no loss or continuity, claims
//! no liblsl equivalence, and grants no runtime, activation, Manifold, session,
//! stream, transport, control, routing, admission, or other authority.

use crate::caller_requested_float32_retained_comparative_snapshot_package::{
    CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry,
};
use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_history::MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportConfigError {
    ZeroMaximumHistoryProposals,
    ZeroMaximumPackageSummaryEntries,
    ZeroMaximumEvidenceEntries,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportBounds {
    maximum_history_proposals: usize,
    maximum_package_summary_entries: usize,
    maximum_evidence_entries: usize,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportBounds {
    pub(crate) fn new(
        maximum_history_proposals: usize,
        maximum_package_summary_entries: usize,
        maximum_evidence_entries: usize,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotReportConfigError> {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportConfigError::*;
        for (value, zero) in [
            (maximum_history_proposals, ZeroMaximumHistoryProposals),
            (
                maximum_package_summary_entries,
                ZeroMaximumPackageSummaryEntries,
            ),
            (maximum_evidence_entries, ZeroMaximumEvidenceEntries),
        ] {
            if value == 0 {
                return Err(zero);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_history_proposals,
            maximum_package_summary_entries,
            maximum_evidence_entries,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence {
    HistoryProposal {
        evidence_index: u64,
        proposal_index: u64,
        fact_count: u64,
    },
    PackageHistorySnapshot {
        evidence_index: u64,
        summary_index: u64,
        snapshot_index: u64,
        observation_count: u64,
    },
    PackageDeltaFact {
        evidence_index: u64,
        summary_index: u64,
        fact_index: u64,
        earlier_count: u64,
        later_count: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReport {
    delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
    comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    evidence_count: u64,
    evidence: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence>,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReport {
    pub(crate) const fn delta_history(
        &self,
    ) -> &MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory {
        &self.delta_history
    }

    pub(crate) const fn comparison_package(
        &self,
    ) -> &CallerRequestedFloat32RetainedComparativeSnapshotPackage {
        &self.comparison_package
    }

    pub(crate) const fn evidence_count(&self) -> u64 {
        self.evidence_count
    }

    pub(crate) fn evidence(
        &self,
    ) -> &[CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence] {
        &self.evidence
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    ) {
        (self.delta_history, self.comparison_package)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportError {
    HistoryProposalLimit {
        limit: usize,
        actual: usize,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
    PackageSummaryLimit {
        limit: usize,
        actual: usize,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
    EvidenceCountOverflow {
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
    EvidenceLimit {
        limit: usize,
        required: usize,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
    CountUnrepresentable {
        actual: usize,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
    IndexUnrepresentable {
        actual: usize,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
    Allocation {
        requested: usize,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    },
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    ) {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportError::*;
        match self {
            HistoryProposalLimit {
                delta_history,
                comparison_package,
                ..
            }
            | PackageSummaryLimit {
                delta_history,
                comparison_package,
                ..
            }
            | EvidenceCountOverflow {
                delta_history,
                comparison_package,
            }
            | EvidenceLimit {
                delta_history,
                comparison_package,
                ..
            }
            | CountUnrepresentable {
                delta_history,
                comparison_package,
                ..
            }
            | IndexUnrepresentable {
                delta_history,
                comparison_package,
                ..
            }
            | Allocation {
                delta_history,
                comparison_package,
                ..
            } => (delta_history, comparison_package),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportOwner {
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotReportBounds,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportOwner {
    pub(crate) const fn new(
        bounds: CallerRequestedFloat32RetainedComparativeSnapshotReportBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn report(
        &self,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotReport,
        CallerRequestedFloat32RetainedComparativeSnapshotReportError,
    > {
        self.report_with(
            delta_history,
            comparison_package,
            |evidence, count| evidence.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn report_with<R, C, A>(
        &self,
        delta_history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        comparison_package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
        reserve: R,
        mut convert: C,
        add: A,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotReport,
        CallerRequestedFloat32RetainedComparativeSnapshotReportError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence>,
            usize,
        ) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportError::*;
        macro_rules! fail {
            ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => {
                return Err($variant {
                    $($field: $value,)*
                    delta_history,
                    comparison_package,
                })
            };
        }

        let mut add = add;
        let history_count = delta_history.proposals().len();
        let package_count = comparison_package.summary().len();
        if history_count > self.bounds.maximum_history_proposals {
            fail!(HistoryProposalLimit {
                limit: self.bounds.maximum_history_proposals,
                actual: history_count
            });
        }
        if package_count > self.bounds.maximum_package_summary_entries {
            fail!(PackageSummaryLimit {
                limit: self.bounds.maximum_package_summary_entries,
                actual: package_count
            });
        }
        let required = match add(history_count, package_count) {
            Ok(value) => value,
            Err(()) => fail!(EvidenceCountOverflow {}),
        };
        if required > self.bounds.maximum_evidence_entries {
            fail!(EvidenceLimit {
                limit: self.bounds.maximum_evidence_entries,
                required: required
            });
        }
        let evidence_count = match convert(required) {
            Ok(value) => value,
            Err(()) => fail!(CountUnrepresentable { actual: required }),
        };

        // Convert every source and output index before allocating. A failed
        // conversion therefore returns both complete P49 owners and no partial
        // report allocation.
        for index in 0..history_count {
            if convert(index).is_err() {
                fail!(IndexUnrepresentable { actual: index });
            }
        }
        for index in 0..package_count {
            if convert(index).is_err() {
                fail!(IndexUnrepresentable { actual: index });
            }
            let evidence_index = match add(history_count, index) {
                Ok(value) => value,
                Err(()) => fail!(EvidenceCountOverflow {}),
            };
            if convert(evidence_index).is_err() {
                fail!(IndexUnrepresentable {
                    actual: evidence_index
                });
            }
        }

        let mut evidence = Vec::new();
        if reserve(&mut evidence, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for (index, proposal) in delta_history.proposals().iter().enumerate() {
            let index = u64::try_from(index).expect("index conversion preflighted");
            evidence.push(
                CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence::HistoryProposal {
                    evidence_index: index,
                    proposal_index: index,
                    fact_count: proposal.fact_count(),
                },
            );
        }
        for (index, summary) in comparison_package.summary().iter().enumerate() {
            let summary_index = u64::try_from(index).expect("index conversion preflighted");
            let evidence_index = u64::try_from(
                history_count
                    .checked_add(index)
                    .expect("evidence index addition preflighted"),
            )
            .expect("index conversion preflighted");
            let entry = match *summary {
                CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry::HistorySnapshot {
                    snapshot_index,
                    observation_count,
                } => CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence::PackageHistorySnapshot {
                    evidence_index,
                    summary_index,
                    snapshot_index,
                    observation_count,
                },
                CallerRequestedFloat32RetainedComparativeSnapshotPackageSummaryEntry::DeltaFact {
                    fact_index,
                    earlier,
                    later,
                    ..
                } => CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence::PackageDeltaFact {
                    evidence_index,
                    summary_index,
                    fact_index,
                    earlier_count: earlier,
                    later_count: later,
                },
            };
            evidence.push(entry);
        }

        Ok(CallerRequestedFloat32RetainedComparativeSnapshotReport {
            delta_history,
            comparison_package,
            evidence_count,
            evidence,
        })
    }
}
