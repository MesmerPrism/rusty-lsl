// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional composition of exact P65 lifecycle evidence with inert advice.
//!
//! Every input is caller-owned data. This facade validates association and retains the
//! subordinate observation and proposal; it performs no lifecycle operation and grants no
//! Manifold authority.

use std::net::SocketAddr;

use crate::morphospace_stream_lifecycle_advisory_proposal::{
    propose_morphospace_stream_lifecycle_advisory, MorphospaceStreamLifecycleAdvisoryDraft,
    MorphospaceStreamLifecycleAdvisoryError, MorphospaceStreamLifecycleAdvisoryProposal,
};
use crate::morphospace_stream_lifecycle_observation as observation;

/// Explicit nonzero bounds for the immutable observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleLimits {
    identity_bytes: usize,
    cycle_limit: usize,
    report_limit: usize,
    record_limit: usize,
}

impl MorphospaceStreamLifecycleLimits {
    /// Creates finite bounds, rejecting every zero bound.
    pub const fn new(
        identity_bytes: usize,
        cycle_limit: usize,
        report_limit: usize,
        record_limit: usize,
    ) -> Result<Self, MorphospaceStreamLifecycleLimitsError> {
        if identity_bytes == 0 {
            return Err(MorphospaceStreamLifecycleLimitsError::ZeroIdentityBytes);
        }
        if cycle_limit == 0 {
            return Err(MorphospaceStreamLifecycleLimitsError::ZeroCycleLimit);
        }
        if report_limit == 0 {
            return Err(MorphospaceStreamLifecycleLimitsError::ZeroReportLimit);
        }
        if record_limit == 0 {
            return Err(MorphospaceStreamLifecycleLimitsError::ZeroRecordLimit);
        }
        Ok(Self {
            identity_bytes,
            cycle_limit,
            report_limit,
            record_limit,
        })
    }
}

/// Invalid public observation bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleLimitsError {
    /// Identity text had no finite positive byte bound.
    ZeroIdentityBytes,
    /// Execution cycles had no finite positive bound.
    ZeroCycleLimit,
    /// Aggregate reports had no finite positive bound.
    ZeroReportLimit,
    /// Records had no finite positive bound.
    ZeroRecordLimit,
}

/// Exact caller and native identity text, without normalization or interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleIdentity<'a> {
    /// Opaque caller provenance.
    pub caller: u128,
    /// Accepted source ID.
    pub source_id: &'a str,
    /// Accepted session ID.
    pub session_id: &'a str,
    /// Accepted stream UID.
    pub stream_uid: &'a str,
}

/// Identity repeated by a subordinate owner, or explicitly unavailable from it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleIdentityEvidence<'a> {
    /// The owner exposed this exact identity.
    Observed(MorphospaceStreamLifecycleIdentity<'a>),
    /// The owner did not expose identity; no value is inferred.
    NotExposedByOwner,
}

/// Exact selected-discovery facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleSelection<'a> {
    /// Identity evidence from the selection owner.
    pub identity: MorphospaceStreamLifecycleIdentityEvidence<'a>,
    /// Zero-based receive-order selection.
    pub response_index: usize,
    /// Exact received response count.
    pub response_count: usize,
    /// Exact response datagram source.
    pub response_source: SocketAddr,
    /// Separately projected service endpoint.
    pub service_endpoint: SocketAddr,
}

/// Exact connection outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleConnection<'a> {
    /// No connection was attempted.
    NotAttempted,
    /// Connection refused with the available identity evidence.
    Refused {
        identity: MorphospaceStreamLifecycleIdentityEvidence<'a>,
    },
    /// Connection was established to the exact peer.
    Established {
        identity: MorphospaceStreamLifecycleIdentityEvidence<'a>,
        peer: SocketAddr,
    },
}

/// Exact caller-requested processing outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleProcessing<'a> {
    /// Processing was not requested.
    NotRequested,
    /// Processing refused while retaining this exact record count.
    Refused {
        identity: MorphospaceStreamLifecycleIdentityEvidence<'a>,
        retained_record_count: usize,
    },
    /// Processing completed for this exact record count.
    Completed {
        identity: MorphospaceStreamLifecycleIdentityEvidence<'a>,
        record_count: usize,
    },
}

/// Exact P64 identity and bounded P63 aggregate extent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleExecution<'a> {
    /// No execution was observed.
    NotExecuted,
    /// An exact P64 execution extent was observed.
    Observed {
        /// Identity evidence exposed by the owner.
        identity: MorphospaceStreamLifecycleIdentityEvidence<'a>,
        /// Exact opaque P64 source.
        source: u128,
        /// Exact opaque P64 execution.
        execution: u128,
        /// Exact finite execution budget.
        budget_cycles: usize,
        /// Exact committed-cycle count.
        committed_cycles: usize,
        /// Exact aggregate report count.
        report_count: usize,
        /// Exact completed-record prefix.
        completed_record_count: usize,
        /// Next uncommitted cycle on refusal, otherwise absent on completion.
        stopped_cycle: Option<usize>,
    },
}

/// Exact terminal state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleTerminal {
    NotReached,
    CanonicallyCompleted,
    TransferRefused,
}
/// Exact close evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleClose {
    NotObserved,
    CanonicalTerminalClose,
    ReportFreeClose,
    CloseRefused,
}
/// Exact cleanup evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleCleanup {
    NotObserved,
    OwnerReleasedResources,
    CleanupRefused,
}
/// Exact loss evidence, preserving owner absence distinctly from zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleLoss {
    NotReportedByAcceptedOwners,
    Exact { lost_records: usize },
}
/// Exact recovery evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleRecovery {
    NotObserved,
    Exact {
        completed_attempts: usize,
        exhausted: bool,
    },
}
/// Exact health evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleHealth {
    NotObserved,
    Exact {
        processing_observations: u64,
        processing_gaps: u64,
        processing_duplicates: u64,
        completed_execution_prefix: usize,
    },
}

/// Complete caller-owned fact set admitted transactionally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleFacts<'a> {
    pub selection: MorphospaceStreamLifecycleSelection<'a>,
    pub connection: MorphospaceStreamLifecycleConnection<'a>,
    pub processing: MorphospaceStreamLifecycleProcessing<'a>,
    pub execution: MorphospaceStreamLifecycleExecution<'a>,
    pub terminal: MorphospaceStreamLifecycleTerminal,
    pub close: MorphospaceStreamLifecycleClose,
    pub cleanup: MorphospaceStreamLifecycleCleanup,
    pub loss: MorphospaceStreamLifecycleLoss,
    pub recovery: MorphospaceStreamLifecycleRecovery,
    pub health: MorphospaceStreamLifecycleHealth,
}

/// Caller-authored bridge binding textual native identity to the proposal's opaque identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleAdvisoryBinding<'a> {
    /// Exact proposal caller provenance.
    pub caller: u128,
    /// Exact proposal authorship provenance.
    pub authorship: u128,
    /// Textual source ID associated with the opaque discovery-source value.
    pub source_id: &'a str,
    /// Textual session ID associated with the opaque session value.
    pub session_id: &'a str,
    /// Textual stream UID associated with the opaque stream value.
    pub stream_uid: &'a str,
    /// Exact opaque discovery-source value expected in the proposal.
    pub discovery_source: u128,
    /// Exact opaque session value expected in the proposal.
    pub session: u128,
    /// Exact opaque stream value expected in the proposal.
    pub stream: u128,
    /// Exact caller-requested processing intent associated with the observation.
    pub post_processing_intent: crate::MorphospaceStreamLifecyclePostProcessingIntent,
    /// Exact cleanup state expected by the caller-authored proposal association.
    pub cleanup: MorphospaceStreamLifecycleCleanup,
    /// Exact loss state expected by the caller-authored proposal association.
    pub loss: MorphospaceStreamLifecycleLoss,
    /// Exact recovery state expected by the caller-authored proposal association.
    pub recovery: MorphospaceStreamLifecycleRecovery,
    /// Exact health state expected by the caller-authored proposal association.
    pub health: MorphospaceStreamLifecycleHealth,
}

/// Successfully associated immutable observation and default-inert proposal.
#[derive(Debug)]
pub struct MorphospaceStreamLifecycleAdvisory<'a> {
    observation: observation::MorphospaceStreamLifecycleObservation<'a>,
    facts: MorphospaceStreamLifecycleFacts<'a>,
    proposal: MorphospaceStreamLifecycleAdvisoryProposal,
}

impl<'a> MorphospaceStreamLifecycleAdvisory<'a> {
    /// Returns the exact admitted identity text and caller provenance.
    pub fn identity(&self) -> MorphospaceStreamLifecycleIdentity<'a> {
        from_identity(self.observation.identity())
    }
    /// Returns every exact admitted fact, including explicit not-observed states.
    pub fn facts(&self) -> MorphospaceStreamLifecycleFacts<'a> {
        self.facts
    }
    /// Returns the exact inert subordinate proposal.
    pub const fn proposal(&self) -> &MorphospaceStreamLifecycleAdvisoryProposal {
        &self.proposal
    }
    /// Consumes the composition and returns the exact inert proposal.
    pub fn into_proposal(self) -> MorphospaceStreamLifecycleAdvisoryProposal {
        self.proposal
    }
}

/// Transactional refusal. No partial observation or proposal is returned.
#[derive(Debug)]
pub enum MorphospaceStreamLifecycleAdvisoryCompositionError {
    /// The observation candidate contradicted its exact facts.
    ObservationRefused,
    /// Caller, authorship, textual identity, or opaque expected identity drifted.
    IdentityOrProvenanceDrift,
    /// P64 source, execution, budget, or committed extent drifted.
    ExecutionBindingDrift,
    /// Selected receive-order identity drifted.
    SelectionBindingDrift,
    /// Processing, terminal, close, cleanup, recovery, loss, or health facts drifted.
    LifecycleFactDrift,
    /// A caller-authored work bound was smaller than the exact observed extent.
    BudgetDrift,
    /// The proposal candidate refused and returned its complete draft.
    ProposalRefused(MorphospaceStreamLifecycleAdvisoryError),
}

fn to_identity(
    value: MorphospaceStreamLifecycleIdentity<'_>,
) -> observation::MorphospaceStreamLifecycleIdentity<'_> {
    observation::MorphospaceStreamLifecycleIdentity::new(
        value.caller,
        value.source_id,
        value.session_id,
        value.stream_uid,
    )
}
fn from_identity(
    value: observation::MorphospaceStreamLifecycleIdentity<'_>,
) -> MorphospaceStreamLifecycleIdentity<'_> {
    MorphospaceStreamLifecycleIdentity {
        caller: value.caller(),
        source_id: value.source_id(),
        session_id: value.session_id(),
        stream_uid: value.stream_uid(),
    }
}
fn to_evidence(
    value: MorphospaceStreamLifecycleIdentityEvidence<'_>,
) -> observation::MorphospaceLifecycleIdentityEvidence<'_> {
    match value {
        MorphospaceStreamLifecycleIdentityEvidence::Observed(v) => {
            observation::MorphospaceLifecycleIdentityEvidence::Observed(to_identity(v))
        }
        MorphospaceStreamLifecycleIdentityEvidence::NotExposedByOwner => {
            observation::MorphospaceLifecycleIdentityEvidence::NotExposedByOwner
        }
    }
}

/// Validates every fact and binding before returning inert advisory data only.
pub fn compose_morphospace_stream_lifecycle_advisory<'a>(
    limits: MorphospaceStreamLifecycleLimits,
    identity: MorphospaceStreamLifecycleIdentity<'a>,
    facts: MorphospaceStreamLifecycleFacts<'a>,
    binding: MorphospaceStreamLifecycleAdvisoryBinding<'a>,
    draft: MorphospaceStreamLifecycleAdvisoryDraft,
) -> Result<
    MorphospaceStreamLifecycleAdvisory<'a>,
    MorphospaceStreamLifecycleAdvisoryCompositionError,
> {
    let execution = match facts.execution {
        MorphospaceStreamLifecycleExecution::NotExecuted => {
            observation::MorphospaceExecutionObservation::NotExecuted
        }
        MorphospaceStreamLifecycleExecution::Observed {
            identity,
            execution,
            budget_cycles,
            committed_cycles,
            report_count,
            completed_record_count,
            stopped_cycle,
            ..
        } => observation::MorphospaceExecutionObservation::Observed {
            identity: to_evidence(identity),
            execution,
            budget_cycles,
            committed_cycles,
            report_count,
            completed_record_count,
            stopped_cycle,
        },
    };
    let observed = observation::observe_stream_lifecycle(
        observation::MorphospaceStreamLifecycleObservationLimits::new(
            limits.identity_bytes,
            limits.cycle_limit,
            limits.report_limit,
            limits.record_limit,
        )
        .expect("public limits are nonzero"),
        to_identity(identity),
        observation::MorphospaceStreamLifecycleFacts {
            selected: observation::MorphospaceSelectedDiscoveryObservation {
                identity: to_evidence(facts.selection.identity),
                response_index: facts.selection.response_index,
                response_count: facts.selection.response_count,
                response_source: facts.selection.response_source,
                service_endpoint: facts.selection.service_endpoint,
            },
            connection: match facts.connection {
                MorphospaceStreamLifecycleConnection::NotAttempted => {
                    observation::MorphospaceConnectionObservation::NotAttempted
                }
                MorphospaceStreamLifecycleConnection::Refused { identity } => {
                    observation::MorphospaceConnectionObservation::Refused {
                        identity: to_evidence(identity),
                    }
                }
                MorphospaceStreamLifecycleConnection::Established { identity, peer } => {
                    observation::MorphospaceConnectionObservation::Established {
                        identity: to_evidence(identity),
                        peer,
                    }
                }
            },
            processing: match facts.processing {
                MorphospaceStreamLifecycleProcessing::NotRequested => {
                    observation::MorphospaceRequestedProcessingObservation::NotRequested
                }
                MorphospaceStreamLifecycleProcessing::Refused {
                    identity,
                    retained_record_count,
                } => observation::MorphospaceRequestedProcessingObservation::Refused {
                    identity: to_evidence(identity),
                    retained_record_count,
                },
                MorphospaceStreamLifecycleProcessing::Completed {
                    identity,
                    record_count,
                } => observation::MorphospaceRequestedProcessingObservation::Completed {
                    identity: to_evidence(identity),
                    record_count,
                },
            },
            execution,
            terminal: match facts.terminal {
                MorphospaceStreamLifecycleTerminal::NotReached => {
                    observation::MorphospaceTerminalObservation::NotReached
                }
                MorphospaceStreamLifecycleTerminal::CanonicallyCompleted => {
                    observation::MorphospaceTerminalObservation::CanonicallyCompleted
                }
                MorphospaceStreamLifecycleTerminal::TransferRefused => {
                    observation::MorphospaceTerminalObservation::TransferRefused
                }
            },
            close: match facts.close {
                MorphospaceStreamLifecycleClose::NotObserved => {
                    observation::MorphospaceCloseObservation::NotObserved
                }
                MorphospaceStreamLifecycleClose::CanonicalTerminalClose => {
                    observation::MorphospaceCloseObservation::CanonicalTerminalClose
                }
                MorphospaceStreamLifecycleClose::ReportFreeClose => {
                    observation::MorphospaceCloseObservation::ReportFreeClose
                }
                MorphospaceStreamLifecycleClose::CloseRefused => {
                    observation::MorphospaceCloseObservation::CloseRefused
                }
            },
            cleanup: match facts.cleanup {
                MorphospaceStreamLifecycleCleanup::NotObserved => {
                    observation::MorphospaceCleanupObservation::NotObserved
                }
                MorphospaceStreamLifecycleCleanup::OwnerReleasedResources => {
                    observation::MorphospaceCleanupObservation::OwnerReleasedResources
                }
                MorphospaceStreamLifecycleCleanup::CleanupRefused => {
                    observation::MorphospaceCleanupObservation::CleanupRefused
                }
            },
            loss: match facts.loss {
                MorphospaceStreamLifecycleLoss::NotReportedByAcceptedOwners => {
                    observation::MorphospaceLossObservation::NotReportedByAcceptedOwners
                }
                MorphospaceStreamLifecycleLoss::Exact { lost_records } => {
                    observation::MorphospaceLossObservation::Exact { lost_records }
                }
            },
            recovery: match facts.recovery {
                MorphospaceStreamLifecycleRecovery::NotObserved => {
                    observation::MorphospaceRecoveryObservation::NotObserved
                }
                MorphospaceStreamLifecycleRecovery::Exact {
                    completed_attempts,
                    exhausted,
                } => observation::MorphospaceRecoveryObservation::Exact {
                    completed_attempts,
                    exhausted,
                },
            },
            health: match facts.health {
                MorphospaceStreamLifecycleHealth::NotObserved => {
                    observation::MorphospaceObservableHealth::NotObserved
                }
                MorphospaceStreamLifecycleHealth::Exact {
                    processing_observations,
                    processing_gaps,
                    processing_duplicates,
                    completed_execution_prefix,
                } => observation::MorphospaceObservableHealth::Exact {
                    processing_observations,
                    processing_gaps,
                    processing_duplicates,
                    completed_execution_prefix,
                },
            },
        },
    )
    .map_err(|_| MorphospaceStreamLifecycleAdvisoryCompositionError::ObservationRefused)?;

    let proposal_binding = draft.observation;
    let expected = draft.expected_identity;
    let provenance = draft.caller_provenance;
    if identity.caller != binding.caller
        || provenance.caller() != binding.caller
        || provenance.authorship() != binding.authorship
        || (identity.source_id, identity.session_id, identity.stream_uid)
            != (binding.source_id, binding.session_id, binding.stream_uid)
        || (
            expected.discovery_source(),
            expected.session(),
            expected.stream(),
        ) != (binding.discovery_source, binding.session, binding.stream)
    {
        return Err(MorphospaceStreamLifecycleAdvisoryCompositionError::IdentityOrProvenanceDrift);
    }
    if expected.selected_response_index() != facts.selection.response_index {
        return Err(MorphospaceStreamLifecycleAdvisoryCompositionError::SelectionBindingDrift);
    }
    let lifecycle_matches = match draft.lifecycle_intent.disposition() {
        crate::MorphospaceStreamLifecycleRequestedDisposition::Complete => matches!(
            facts.terminal,
            MorphospaceStreamLifecycleTerminal::CanonicallyCompleted
        ),
        crate::MorphospaceStreamLifecycleRequestedDisposition::StopWithoutCompletion => !matches!(
            facts.terminal,
            MorphospaceStreamLifecycleTerminal::CanonicallyCompleted
        ),
    } && match draft.lifecycle_intent.requested_close() {
        crate::MorphospaceStreamLifecycleRequestedClose::CanonicalCompletion => matches!(
            facts.close,
            MorphospaceStreamLifecycleClose::CanonicalTerminalClose
        ),
        crate::MorphospaceStreamLifecycleRequestedClose::ReportFreeCallerClose => matches!(
            facts.close,
            MorphospaceStreamLifecycleClose::ReportFreeClose
        ),
    };
    if !lifecycle_matches
        || draft.post_processing_intent != binding.post_processing_intent
        || facts.cleanup != binding.cleanup
        || facts.loss != binding.loss
        || facts.recovery != binding.recovery
        || facts.health != binding.health
    {
        return Err(MorphospaceStreamLifecycleAdvisoryCompositionError::LifecycleFactDrift);
    }
    let processed_records = match facts.processing {
        MorphospaceStreamLifecycleProcessing::NotRequested => 0,
        MorphospaceStreamLifecycleProcessing::Refused {
            retained_record_count,
            ..
        } => retained_record_count,
        MorphospaceStreamLifecycleProcessing::Completed { record_count, .. } => record_count,
    };
    let recovery_attempts = match facts.recovery {
        MorphospaceStreamLifecycleRecovery::NotObserved => 0,
        MorphospaceStreamLifecycleRecovery::Exact {
            completed_attempts, ..
        } => completed_attempts,
    };
    if processed_records > draft.budgets.maximum_records()
        || recovery_attempts > draft.budgets.maximum_recovery_attempts()
    {
        return Err(MorphospaceStreamLifecycleAdvisoryCompositionError::BudgetDrift);
    }
    match facts.execution {
        MorphospaceStreamLifecycleExecution::Observed {
            source,
            execution,
            budget_cycles,
            committed_cycles,
            ..
        } if (
            proposal_binding.source(),
            proposal_binding.execution(),
            proposal_binding.budget_cycles(),
            proposal_binding.committed_cycles(),
        ) == (source, execution, budget_cycles, committed_cycles) => {}
        _ => return Err(MorphospaceStreamLifecycleAdvisoryCompositionError::ExecutionBindingDrift),
    }
    let proposal = propose_morphospace_stream_lifecycle_advisory(draft)
        .map_err(MorphospaceStreamLifecycleAdvisoryCompositionError::ProposalRefused)?;
    Ok(MorphospaceStreamLifecycleAdvisory {
        observation: observed,
        facts,
        proposal,
    })
}
