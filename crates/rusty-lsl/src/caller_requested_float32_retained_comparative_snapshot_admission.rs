// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested admission for retained Float32 comparative evidence.
//!
//! This crate-private, default-inert owner validates explicit bounds and exact
//! extents over the actual P49 delta history and retained package. Admission
//! moves both complete inputs into an ownership-bearing plan without cloning
//! their nested allocations. It constructs no report, infers no loss,
//! continuity, or causality, and grants no runtime, activation, application,
//! liblsl-equivalence, Manifold, session, transport, or control authority.

use crate::caller_requested_float32_retained_comparative_snapshot_package::CallerRequestedFloat32RetainedComparativeSnapshotPackage;
use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_history::MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotAdmissionConfigError {
    ZeroMaximumHistoryProposals,
    ZeroMaximumHistoryFacts,
    ZeroMaximumPackageSnapshots,
    ZeroMaximumPackageDeltaFacts,
    ZeroMaximumPackageSummaryEntries,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds {
    maximum_history_proposals: usize,
    maximum_history_facts: usize,
    maximum_package_snapshots: usize,
    maximum_package_delta_facts: usize,
    maximum_package_summary_entries: usize,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds {
    pub(crate) fn new(
        maximum_history_proposals: usize,
        maximum_history_facts: usize,
        maximum_package_snapshots: usize,
        maximum_package_delta_facts: usize,
        maximum_package_summary_entries: usize,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotAdmissionConfigError> {
        use CallerRequestedFloat32RetainedComparativeSnapshotAdmissionConfigError::*;
        for (value, zero) in [
            (maximum_history_proposals, ZeroMaximumHistoryProposals),
            (maximum_history_facts, ZeroMaximumHistoryFacts),
            (maximum_package_snapshots, ZeroMaximumPackageSnapshots),
            (maximum_package_delta_facts, ZeroMaximumPackageDeltaFacts),
            (
                maximum_package_summary_entries,
                ZeroMaximumPackageSummaryEntries,
            ),
        ] {
            if value == 0 {
                return Err(zero);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_history_proposals,
            maximum_history_facts,
            maximum_package_snapshots,
            maximum_package_delta_facts,
            maximum_package_summary_entries,
        })
    }
}

/// Exact caller-requested P49 input extents. Empty individual collections are
/// permitted; only the independently selected admission bounds are nonzero.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotExtents {
    pub(crate) history_proposals: u64,
    pub(crate) history_facts: u64,
    pub(crate) package_snapshots: u64,
    pub(crate) package_delta_facts: u64,
    pub(crate) package_summary_entries: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure {
    ExtentUnrepresentable {
        extent: &'static str,
        actual: u64,
    },
    ExtentCountUnrepresentable {
        extent: &'static str,
        actual: usize,
    },
    ExtentArithmeticOverflow,
    HistoryProposalTotalMismatch {
        stored: u64,
        actual: u64,
    },
    HistoryFactTotalMismatch {
        stored: u64,
        actual: u64,
    },
    PackageSummaryTotalMismatch {
        stored: u64,
        actual: u64,
    },
    BoundExceeded {
        extent: &'static str,
        maximum: usize,
        actual: usize,
    },
    RequestedExtentMismatch {
        extent: &'static str,
        requested: u64,
        actual: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotAdmissionError {
    failure: CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure,
    history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
    package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotAdmissionError {
    pub(crate) const fn failure(
        &self,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure {
        self.failure
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    ) {
        (self.history, self.package)
    }
}

/// Validation-complete ownership for a later canonical report composition.
#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotPlan {
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds,
    extents: CallerRequestedFloat32RetainedComparativeSnapshotExtents,
    history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
    package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotPlan {
    pub(crate) const fn bounds(
        &self,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds {
        self.bounds
    }

    pub(crate) const fn extents(&self) -> CallerRequestedFloat32RetainedComparativeSnapshotExtents {
        self.extents
    }

    pub(crate) const fn history(
        &self,
    ) -> &MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory {
        &self.history
    }

    pub(crate) const fn package(
        &self,
    ) -> &CallerRequestedFloat32RetainedComparativeSnapshotPackage {
        &self.package
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds,
        CallerRequestedFloat32RetainedComparativeSnapshotExtents,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    ) {
        (self.bounds, self.extents, self.history, self.package)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotAdmission {
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotAdmission {
    pub(crate) const fn new(
        bounds: CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn admit(
        self,
        requested: CallerRequestedFloat32RetainedComparativeSnapshotExtents,
        history: MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        package: CallerRequestedFloat32RetainedComparativeSnapshotPackage,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotPlan,
        CallerRequestedFloat32RetainedComparativeSnapshotAdmissionError,
    > {
        let result = validate(self.bounds, requested, &history, &package);
        match result {
            Ok(extents) => Ok(CallerRequestedFloat32RetainedComparativeSnapshotPlan {
                bounds: self.bounds,
                extents,
                history,
                package,
            }),
            Err(failure) => Err(
                CallerRequestedFloat32RetainedComparativeSnapshotAdmissionError {
                    failure,
                    history,
                    package,
                },
            ),
        }
    }
}

fn validate(
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds,
    requested: CallerRequestedFloat32RetainedComparativeSnapshotExtents,
    history: &MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
    package: &CallerRequestedFloat32RetainedComparativeSnapshotPackage,
) -> Result<
    CallerRequestedFloat32RetainedComparativeSnapshotExtents,
    CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure,
> {
    use CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure::*;

    let history_proposals_usize = history.proposals().len();
    let history_facts_stored = history.totals().fact_count();
    let package_snapshots_usize = package.history().snapshots().len();
    let package_delta_facts_usize = package.delta_proposal().facts().len();
    let package_summary_usize = package.summary().len();

    let history_proposals = to_u64("history proposals", history_proposals_usize)?;
    let history_facts = to_usize("history facts", history_facts_stored)?;
    let package_snapshots = to_u64("package snapshots", package_snapshots_usize)?;
    let package_delta_facts = to_u64("package delta facts", package_delta_facts_usize)?;
    let package_summary = to_u64("package summary entries", package_summary_usize)?;

    if history.totals().proposal_count() != history_proposals {
        return Err(HistoryProposalTotalMismatch {
            stored: history.totals().proposal_count(),
            actual: history_proposals,
        });
    }
    let recomputed_history_facts = history
        .proposals()
        .iter()
        .try_fold(0_u64, |total, proposal| {
            total.checked_add(proposal.fact_count())
        })
        .ok_or(ExtentArithmeticOverflow)?;
    if history_facts_stored != recomputed_history_facts {
        return Err(HistoryFactTotalMismatch {
            stored: history_facts_stored,
            actual: recomputed_history_facts,
        });
    }
    if package.summary_count() != package_summary {
        return Err(PackageSummaryTotalMismatch {
            stored: package.summary_count(),
            actual: package_summary,
        });
    }

    // Exercise both exact arithmetic domains before accepting composition.
    history_proposals_usize
        .checked_add(history_facts)
        .and_then(|value| value.checked_add(package_snapshots_usize))
        .and_then(|value| value.checked_add(package_delta_facts_usize))
        .and_then(|value| value.checked_add(package_summary_usize))
        .ok_or(ExtentArithmeticOverflow)?;
    history_proposals
        .checked_add(history_facts_stored)
        .and_then(|value| value.checked_add(package_snapshots))
        .and_then(|value| value.checked_add(package_delta_facts))
        .and_then(|value| value.checked_add(package_summary))
        .ok_or(ExtentArithmeticOverflow)?;

    for (name, actual, maximum) in [
        (
            "history proposals",
            history_proposals_usize,
            bounds.maximum_history_proposals,
        ),
        ("history facts", history_facts, bounds.maximum_history_facts),
        (
            "package snapshots",
            package_snapshots_usize,
            bounds.maximum_package_snapshots,
        ),
        (
            "package delta facts",
            package_delta_facts_usize,
            bounds.maximum_package_delta_facts,
        ),
        (
            "package summary entries",
            package_summary_usize,
            bounds.maximum_package_summary_entries,
        ),
    ] {
        if actual > maximum {
            return Err(BoundExceeded {
                extent: name,
                maximum,
                actual,
            });
        }
    }

    let actual = CallerRequestedFloat32RetainedComparativeSnapshotExtents {
        history_proposals,
        history_facts: history_facts_stored,
        package_snapshots,
        package_delta_facts,
        package_summary_entries: package_summary,
    };
    for (name, requested, actual) in [
        (
            "history proposals",
            requested.history_proposals,
            actual.history_proposals,
        ),
        (
            "history facts",
            requested.history_facts,
            actual.history_facts,
        ),
        (
            "package snapshots",
            requested.package_snapshots,
            actual.package_snapshots,
        ),
        (
            "package delta facts",
            requested.package_delta_facts,
            actual.package_delta_facts,
        ),
        (
            "package summary entries",
            requested.package_summary_entries,
            actual.package_summary_entries,
        ),
    ] {
        if requested != actual {
            return Err(RequestedExtentMismatch {
                extent: name,
                requested,
                actual,
            });
        }
    }
    Ok(actual)
}

fn to_u64(
    extent: &'static str,
    actual: usize,
) -> Result<u64, CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure> {
    u64::try_from(actual).map_err(|_| {
        CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure::ExtentCountUnrepresentable {
            extent,
            actual,
        }
    })
}

fn to_usize(
    extent: &'static str,
    actual: u64,
) -> Result<usize, CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure> {
    usize::try_from(actual).map_err(|_| {
        CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure::ExtentUnrepresentable {
            extent,
            actual,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_reject_each_zero_in_precedence_order() {
        use CallerRequestedFloat32RetainedComparativeSnapshotAdmissionConfigError::*;
        for (values, expected) in [
            ([0, 1, 1, 1, 1], ZeroMaximumHistoryProposals),
            ([1, 0, 1, 1, 1], ZeroMaximumHistoryFacts),
            ([1, 1, 0, 1, 1], ZeroMaximumPackageSnapshots),
            ([1, 1, 1, 0, 1], ZeroMaximumPackageDeltaFacts),
            ([1, 1, 1, 1, 0], ZeroMaximumPackageSummaryEntries),
        ] {
            assert_eq!(
                CallerRequestedFloat32RetainedComparativeSnapshotAdmissionBounds::new(
                    values[0], values[1], values[2], values[3], values[4]
                ),
                Err(expected)
            );
        }
    }

    #[test]
    fn checked_conversion_boundary_is_exact() {
        assert_eq!(to_u64("x", 0), Ok(0));
        assert_eq!(to_usize("x", 0), Ok(0));
        let largest = usize::try_from(u64::MAX).unwrap_or(usize::MAX);
        assert_eq!(to_u64("x", largest), Ok(largest as u64));
        if usize::BITS < u64::BITS {
            let too_large = (usize::MAX as u64).checked_add(1).unwrap();
            assert_eq!(
                to_usize("x", too_large),
                Err(CallerRequestedFloat32RetainedComparativeSnapshotAdmissionFailure::ExtentUnrepresentable {
                    extent: "x",
                    actual: too_large
                })
            );
        }
    }
}
