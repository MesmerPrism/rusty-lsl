// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Immutable, caller-owned advisory projection over exact P63 supervision facts.
//!
//! The proposal is crate-private, default-inert, non-applying, and data-only. It binds an
//! opaque caller identity and an ordered bounded supervision series, labels facts absent from
//! P63 as unavailable, and performs no action. It grants no Manifold authority.

use crate::{
    RequestedProcessingExecutionHealth, RequestedProcessingRecoveryQueueSupervision,
    RequestedProcessingSupervisionLossFacts, RequestedProcessingSupervisionOwnerFacts,
    RequestedProcessingSupervisionTerminations,
};

pub(crate) fn classify_requested_processing_execution_cycle(
    total: usize,
    completed: usize,
    remaining: usize,
    current: Option<usize>,
) -> Option<bool> {
    let extent_consistent = completed.checked_add(remaining) == Some(total);
    let complete = remaining == 0 && current.is_none();
    let incomplete = remaining != 0 && current == Some(completed);
    (extent_consistent && (complete || incomplete)).then_some(complete)
}

/// Exact opaque identity supplied and retained by the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceRequestedProcessingExecutionSourceIdentity {
    pub(crate) producer: u128,
    pub(crate) execution_batch: u128,
}

/// Explicit finite envelope for one proposal owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceRequestedProcessingExecutionAdvisoryBounds {
    maximum_cycles: usize,
    maximum_reports_per_cycle: usize,
    maximum_execution_extent: usize,
}

impl MorphospaceRequestedProcessingExecutionAdvisoryBounds {
    pub(crate) const fn new(
        maximum_cycles: usize,
        maximum_reports_per_cycle: usize,
        maximum_execution_extent: usize,
    ) -> Result<Self, MorphospaceRequestedProcessingExecutionAdvisoryConfigError> {
        if maximum_cycles == 0 {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryConfigError::ZeroMaximumCycles,
            );
        }
        if maximum_reports_per_cycle == 0 {
            return Err(MorphospaceRequestedProcessingExecutionAdvisoryConfigError::ZeroMaximumReportsPerCycle);
        }
        if maximum_execution_extent == 0 {
            return Err(MorphospaceRequestedProcessingExecutionAdvisoryConfigError::ZeroMaximumExecutionExtent);
        }
        Ok(Self {
            maximum_cycles,
            maximum_reports_per_cycle,
            maximum_execution_extent,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingExecutionAdvisoryConfigError {
    ZeroMaximumCycles,
    ZeroMaximumReportsPerCycle,
    ZeroMaximumExecutionExtent,
}

/// Caller-owned source. P64 borrows this exact P63 series and never replaces it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MorphospaceRequestedProcessingExecutionAdvisorySource<'a> {
    pub(crate) identity: MorphospaceRequestedProcessingExecutionSourceIdentity,
    pub(crate) declared_cycle_count: usize,
    pub(crate) supervision: &'a [RequestedProcessingRecoveryQueueSupervision],
}

/// Facts that P63 does not supply remain explicitly unknown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingExecutionUnknownFact {
    NotSuppliedByP63,
}

/// Exact copied evidence for one zero-based batch cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceRequestedProcessingExecutionCycleEvidence {
    pub(crate) cycle: usize,
    pub(crate) report_count: usize,
    pub(crate) total_execution_count: usize,
    pub(crate) first_completed_execution_count: usize,
    pub(crate) last_completed_execution_count: usize,
    pub(crate) last_remaining_execution_count: usize,
    pub(crate) last_current_execution_index: Option<usize>,
    pub(crate) last_health: RequestedProcessingExecutionHealth,
    pub(crate) terminations: RequestedProcessingSupervisionTerminations,
    pub(crate) owner_facts: RequestedProcessingSupervisionOwnerFacts,
    pub(crate) loss_facts: RequestedProcessingSupervisionLossFacts,
}

/// Descriptive advisory classification only; neither variant has effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingExecutionAdvisoryClassification {
    AllCyclesComplete,
    IncompleteEvidencePresent,
}

#[derive(Debug)]
pub(crate) struct MorphospaceRequestedProcessingExecutionAdvisoryProposal<'a> {
    source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    evidence: Vec<MorphospaceRequestedProcessingExecutionCycleEvidence>,
    classification: MorphospaceRequestedProcessingExecutionAdvisoryClassification,
    pub(crate) elapsed_time: MorphospaceRequestedProcessingExecutionUnknownFact,
    pub(crate) progress_cause: MorphospaceRequestedProcessingExecutionUnknownFact,
}

impl<'a> MorphospaceRequestedProcessingExecutionAdvisoryProposal<'a> {
    pub(crate) const fn source(
        &self,
    ) -> &MorphospaceRequestedProcessingExecutionAdvisorySource<'a> {
        &self.source
    }
    pub(crate) fn evidence(&self) -> &[MorphospaceRequestedProcessingExecutionCycleEvidence] {
        &self.evidence
    }
    pub(crate) const fn classification(
        &self,
    ) -> MorphospaceRequestedProcessingExecutionAdvisoryClassification {
        self.classification
    }
    pub(crate) fn into_source(self) -> MorphospaceRequestedProcessingExecutionAdvisorySource<'a> {
        self.source
    }
}

#[derive(Debug)]
pub(crate) enum MorphospaceRequestedProcessingExecutionAdvisoryError<'a> {
    IdentitySubstitution {
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    EmptySeries {
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    CycleExtentDrift {
        expected: usize,
        actual: usize,
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    CycleLimit {
        limit: usize,
        actual: usize,
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    ReportLimit {
        cycle: usize,
        limit: usize,
        actual: usize,
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    ExecutionExtentExpansion {
        cycle: usize,
        limit: usize,
        actual: usize,
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    EvidenceContradiction {
        cycle: usize,
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
    Allocation {
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    },
}

impl<'a> MorphospaceRequestedProcessingExecutionAdvisoryError<'a> {
    pub(crate) fn into_source(self) -> MorphospaceRequestedProcessingExecutionAdvisorySource<'a> {
        match self {
            Self::IdentitySubstitution { source }
            | Self::EmptySeries { source }
            | Self::CycleExtentDrift { source, .. }
            | Self::CycleLimit { source, .. }
            | Self::ReportLimit { source, .. }
            | Self::ExecutionExtentExpansion { source, .. }
            | Self::EvidenceContradiction { source, .. }
            | Self::Allocation { source } => source,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct MorphospaceRequestedProcessingExecutionAdvisoryProposalOwner {
    expected_identity: MorphospaceRequestedProcessingExecutionSourceIdentity,
    bounds: MorphospaceRequestedProcessingExecutionAdvisoryBounds,
}

impl MorphospaceRequestedProcessingExecutionAdvisoryProposalOwner {
    pub(crate) const fn new(
        expected_identity: MorphospaceRequestedProcessingExecutionSourceIdentity,
        bounds: MorphospaceRequestedProcessingExecutionAdvisoryBounds,
    ) -> Self {
        Self {
            expected_identity,
            bounds,
        }
    }

    pub(crate) fn propose<'a>(
        &self,
        source: MorphospaceRequestedProcessingExecutionAdvisorySource<'a>,
    ) -> Result<
        MorphospaceRequestedProcessingExecutionAdvisoryProposal<'a>,
        MorphospaceRequestedProcessingExecutionAdvisoryError<'a>,
    > {
        if source.identity != self.expected_identity {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::IdentitySubstitution {
                    source,
                },
            );
        }
        if source.supervision.is_empty() {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::EmptySeries { source },
            );
        }
        if source.declared_cycle_count != source.supervision.len() {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::CycleExtentDrift {
                    expected: source.declared_cycle_count,
                    actual: source.supervision.len(),
                    source,
                },
            );
        }
        if source.supervision.len() > self.bounds.maximum_cycles {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::CycleLimit {
                    limit: self.bounds.maximum_cycles,
                    actual: source.supervision.len(),
                    source,
                },
            );
        }
        let mut evidence = Vec::new();
        if evidence
            .try_reserve_exact(source.supervision.len())
            .is_err()
        {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::Allocation { source },
            );
        }
        let mut all_complete = true;
        for (cycle, value) in source.supervision.iter().copied().enumerate() {
            if value.report_count() > self.bounds.maximum_reports_per_cycle {
                return Err(
                    MorphospaceRequestedProcessingExecutionAdvisoryError::ReportLimit {
                        cycle,
                        limit: self.bounds.maximum_reports_per_cycle,
                        actual: value.report_count(),
                        source,
                    },
                );
            }
            if value.total_execution_count() > self.bounds.maximum_execution_extent {
                return Err(MorphospaceRequestedProcessingExecutionAdvisoryError::ExecutionExtentExpansion { cycle, limit: self.bounds.maximum_execution_extent, actual: value.total_execution_count(), source });
            }
            let Some(complete) = classify_requested_processing_execution_cycle(
                value.total_execution_count(),
                value.last_completed_execution_count(),
                value.last_remaining_execution_count(),
                value.last_current_execution_index(),
            ) else {
                return Err(
                    MorphospaceRequestedProcessingExecutionAdvisoryError::EvidenceContradiction {
                        cycle,
                        source,
                    },
                );
            };
            all_complete &= complete;
            evidence.push(MorphospaceRequestedProcessingExecutionCycleEvidence {
                cycle,
                report_count: value.report_count(),
                total_execution_count: value.total_execution_count(),
                first_completed_execution_count: value.first_completed_execution_count(),
                last_completed_execution_count: value.last_completed_execution_count(),
                last_remaining_execution_count: value.last_remaining_execution_count(),
                last_current_execution_index: value.last_current_execution_index(),
                last_health: value.last_health(),
                terminations: value.terminations(),
                owner_facts: value.owner_facts(),
                loss_facts: value.loss_facts(),
            });
        }
        Ok(MorphospaceRequestedProcessingExecutionAdvisoryProposal {
            source,
            evidence,
            classification: if all_complete {
                MorphospaceRequestedProcessingExecutionAdvisoryClassification::AllCyclesComplete
            } else {
                MorphospaceRequestedProcessingExecutionAdvisoryClassification::IncompleteEvidencePresent
            },
            elapsed_time: MorphospaceRequestedProcessingExecutionUnknownFact::NotSuppliedByP63,
            progress_cause: MorphospaceRequestedProcessingExecutionUnknownFact::NotSuppliedByP63,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        RequestedProcessingExecutionReportLimits, RequestedProcessingExecutionStage,
        RequestedProcessingExecutionTermination, RequestedProcessingRecoveryQueueExecutionReport,
        RequestedProcessingSupervisionLimits,
    };

    const ID: MorphospaceRequestedProcessingExecutionSourceIdentity =
        MorphospaceRequestedProcessingExecutionSourceIdentity {
            producer: 7,
            execution_batch: 11,
        };
    fn report(
        completed: usize,
        termination: RequestedProcessingExecutionTermination,
    ) -> RequestedProcessingRecoveryQueueExecutionReport {
        RequestedProcessingRecoveryQueueExecutionReport::new(
            RequestedProcessingExecutionReportLimits::new(3, 2, 2).unwrap(),
            3,
            completed,
            termination,
        )
        .unwrap()
    }
    fn supervision(complete: bool) -> RequestedProcessingRecoveryQueueSupervision {
        let reports = if complete {
            vec![report(3, RequestedProcessingExecutionTermination::Complete)]
        } else {
            vec![report(
                1,
                RequestedProcessingExecutionTermination::Cancelled {
                    stage: RequestedProcessingExecutionStage::Recovery,
                    completed_recovery_attempts: 0,
                    queue_len: None,
                },
            )]
        };
        RequestedProcessingRecoveryQueueSupervision::new(
            RequestedProcessingSupervisionLimits::new(2).unwrap(),
            &reports,
        )
        .unwrap()
    }
    fn owner() -> MorphospaceRequestedProcessingExecutionAdvisoryProposalOwner {
        MorphospaceRequestedProcessingExecutionAdvisoryProposalOwner::new(
            ID,
            MorphospaceRequestedProcessingExecutionAdvisoryBounds::new(2, 2, 3).unwrap(),
        )
    }

    #[test]
    fn exact_identity_order_evidence_and_unknowns_are_retained() {
        let values = [supervision(true), supervision(false)];
        let source = MorphospaceRequestedProcessingExecutionAdvisorySource {
            identity: ID,
            declared_cycle_count: 2,
            supervision: &values,
        };
        let proposal = owner().propose(source).unwrap();
        assert_eq!(proposal.source().supervision.as_ptr(), values.as_ptr());
        assert_eq!(
            proposal
                .evidence()
                .iter()
                .map(|v| (
                    v.cycle,
                    v.report_count,
                    v.total_execution_count,
                    v.last_completed_execution_count
                ))
                .collect::<Vec<_>>(),
            [(0, 1, 3, 3), (1, 1, 3, 1)]
        );
        assert_eq!(proposal.classification(), MorphospaceRequestedProcessingExecutionAdvisoryClassification::IncompleteEvidencePresent);
        assert_eq!(
            proposal.evidence()[0].loss_facts,
            RequestedProcessingSupervisionLossFacts::NotReportedByP62
        );
        assert_eq!(
            proposal.elapsed_time,
            MorphospaceRequestedProcessingExecutionUnknownFact::NotSuppliedByP63
        );
    }

    #[test]
    fn substitution_drift_and_expansion_return_the_exact_source() {
        let values = [supervision(true)];
        for source in [
            MorphospaceRequestedProcessingExecutionAdvisorySource {
                identity: MorphospaceRequestedProcessingExecutionSourceIdentity {
                    producer: 8,
                    execution_batch: 11,
                },
                declared_cycle_count: 1,
                supervision: &values,
            },
            MorphospaceRequestedProcessingExecutionAdvisorySource {
                identity: ID,
                declared_cycle_count: 2,
                supervision: &values,
            },
        ] {
            let pointer = source.supervision.as_ptr();
            assert_eq!(
                owner()
                    .propose(source)
                    .unwrap_err()
                    .into_source()
                    .supervision
                    .as_ptr(),
                pointer
            );
        }
        let narrow = MorphospaceRequestedProcessingExecutionAdvisoryProposalOwner::new(
            ID,
            MorphospaceRequestedProcessingExecutionAdvisoryBounds::new(1, 1, 2).unwrap(),
        );
        let source = MorphospaceRequestedProcessingExecutionAdvisorySource {
            identity: ID,
            declared_cycle_count: 1,
            supervision: &values,
        };
        assert!(matches!(
            narrow.propose(source),
            Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::ExecutionExtentExpansion { .. }
            )
        ));
    }

    #[test]
    fn boundary_is_private_inert_advisory_and_non_applying() {
        let source =
            include_str!("morphospace_requested_processing_execution_advisory_proposal.rs");
        for wording in [
            "crate-private",
            "default-inert",
            "non-applying",
            "performs no action",
            "Manifold authority",
            "NotSuppliedByP63",
        ] {
            assert!(source.contains(wording));
        }
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
            concat!("serde", "::"),
        ] {
            assert!(!source.contains(operation));
        }
    }
}
