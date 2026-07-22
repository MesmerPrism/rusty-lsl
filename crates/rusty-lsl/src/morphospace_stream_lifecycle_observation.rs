// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Immutable, bounded whole-session lifecycle observation.
//!
//! The caller supplies already-observed facts from the selected-discovery, connection,
//! requested-processing execution, terminal-close, and cleanup owners. This module validates
//! their association; it performs none of those operations and grants no authority.

use std::net::SocketAddr;

/// Finite admission bounds for caller-owned textual identity and execution extents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceStreamLifecycleObservationLimits {
    identity_bytes: usize,
    cycle_limit: usize,
    report_limit: usize,
    record_limit: usize,
}

impl MorphospaceStreamLifecycleObservationLimits {
    pub(crate) const fn new(
        identity_bytes: usize,
        cycle_limit: usize,
        report_limit: usize,
        record_limit: usize,
    ) -> Result<Self, MorphospaceStreamLifecycleObservationConfigError> {
        if identity_bytes == 0 {
            return Err(MorphospaceStreamLifecycleObservationConfigError::ZeroIdentityBytes);
        }
        if cycle_limit == 0 {
            return Err(MorphospaceStreamLifecycleObservationConfigError::ZeroCycleLimit);
        }
        if report_limit == 0 {
            return Err(MorphospaceStreamLifecycleObservationConfigError::ZeroReportLimit);
        }
        if record_limit == 0 {
            return Err(MorphospaceStreamLifecycleObservationConfigError::ZeroRecordLimit);
        }
        Ok(Self {
            identity_bytes,
            cycle_limit,
            report_limit,
            record_limit,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceStreamLifecycleObservationConfigError {
    ZeroIdentityBytes,
    ZeroCycleLimit,
    ZeroReportLimit,
    ZeroRecordLimit,
}

/// Exact caller provenance and accepted native stream identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceStreamLifecycleIdentity<'a> {
    caller: u128,
    source_id: &'a str,
    session_id: &'a str,
    stream_uid: &'a str,
}

impl<'a> MorphospaceStreamLifecycleIdentity<'a> {
    pub(crate) const fn new(
        caller: u128,
        source_id: &'a str,
        session_id: &'a str,
        stream_uid: &'a str,
    ) -> Self {
        Self {
            caller,
            source_id,
            session_id,
            stream_uid,
        }
    }
    pub(crate) const fn caller(self) -> u128 {
        self.caller
    }
    pub(crate) const fn source_id(self) -> &'a str {
        self.source_id
    }
    pub(crate) const fn session_id(self) -> &'a str {
        self.session_id
    }
    pub(crate) const fn stream_uid(self) -> &'a str {
        self.stream_uid
    }
}

/// Identity repeated by a subordinate owner, or explicitly unavailable from that owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceLifecycleIdentityEvidence<'a> {
    Observed(MorphospaceStreamLifecycleIdentity<'a>),
    NotExposedByOwner,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceSelectedDiscoveryObservation<'a> {
    pub(crate) identity: MorphospaceLifecycleIdentityEvidence<'a>,
    pub(crate) response_index: usize,
    pub(crate) response_count: usize,
    pub(crate) response_source: SocketAddr,
    pub(crate) service_endpoint: SocketAddr,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceConnectionObservation<'a> {
    NotAttempted,
    Refused {
        identity: MorphospaceLifecycleIdentityEvidence<'a>,
    },
    Established {
        identity: MorphospaceLifecycleIdentityEvidence<'a>,
        peer: SocketAddr,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingObservation<'a> {
    NotRequested,
    Refused {
        identity: MorphospaceLifecycleIdentityEvidence<'a>,
        retained_record_count: usize,
    },
    Completed {
        identity: MorphospaceLifecycleIdentityEvidence<'a>,
        record_count: usize,
    },
}

/// Exact P64 association and bounded P63 aggregate extents, without inventing per-cycle facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceExecutionObservation<'a> {
    NotExecuted,
    Observed {
        identity: MorphospaceLifecycleIdentityEvidence<'a>,
        execution: u128,
        budget_cycles: usize,
        committed_cycles: usize,
        report_count: usize,
        completed_record_count: usize,
        stopped_cycle: Option<usize>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceTerminalObservation {
    NotReached,
    CanonicallyCompleted,
    TransferRefused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceCloseObservation {
    NotObserved,
    CanonicalTerminalClose,
    ReportFreeClose,
    CloseRefused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceCleanupObservation {
    NotObserved,
    OwnerReleasedResources,
    CleanupRefused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceLossObservation {
    NotReportedByAcceptedOwners,
    Exact { lost_records: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRecoveryObservation {
    NotObserved,
    Exact {
        completed_attempts: usize,
        exhausted: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceObservableHealth {
    NotObserved,
    Exact {
        processing_observations: u64,
        processing_gaps: u64,
        processing_duplicates: u64,
        completed_execution_prefix: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceStreamLifecycleFacts<'a> {
    pub(crate) selected: MorphospaceSelectedDiscoveryObservation<'a>,
    pub(crate) connection: MorphospaceConnectionObservation<'a>,
    pub(crate) processing: MorphospaceRequestedProcessingObservation<'a>,
    pub(crate) execution: MorphospaceExecutionObservation<'a>,
    pub(crate) terminal: MorphospaceTerminalObservation,
    pub(crate) close: MorphospaceCloseObservation,
    pub(crate) cleanup: MorphospaceCleanupObservation,
    pub(crate) loss: MorphospaceLossObservation,
    pub(crate) recovery: MorphospaceRecoveryObservation,
    pub(crate) health: MorphospaceObservableHealth,
}

/// Immutable evidence. Accessors return only the exact admitted caller facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceStreamLifecycleObservation<'a> {
    identity: MorphospaceStreamLifecycleIdentity<'a>,
    facts: MorphospaceStreamLifecycleFacts<'a>,
}

impl<'a> MorphospaceStreamLifecycleObservation<'a> {
    pub(crate) const fn identity(&self) -> MorphospaceStreamLifecycleIdentity<'a> {
        self.identity
    }
    pub(crate) const fn facts(&self) -> MorphospaceStreamLifecycleFacts<'a> {
        self.facts
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceStreamLifecycleObservationError {
    EmptyIdentity {
        field: &'static str,
    },
    IdentityLimit {
        field: &'static str,
        limit: usize,
        actual: usize,
    },
    IdentityDrift {
        stage: &'static str,
    },
    SelectionOutOfBounds {
        selected: usize,
        responses: usize,
    },
    NonConcretePeer {
        stage: &'static str,
    },
    PeerDrift,
    LifecycleOrdering {
        earlier: &'static str,
        later: &'static str,
    },
    CycleLimit {
        limit: usize,
        actual: usize,
    },
    ReportLimit {
        limit: usize,
        actual: usize,
    },
    RecordLimit {
        limit: usize,
        actual: usize,
    },
    ExecutionExtent {
        budget: usize,
        committed: usize,
        stopped_cycle: Option<usize>,
    },
    ProcessingExecutionContradiction {
        processed: usize,
        executed: usize,
    },
    HealthContradiction,
    RecoveryContradiction,
    LossContradiction,
    CleanupContradiction,
}

fn validate_identity(
    limits: MorphospaceStreamLifecycleObservationLimits,
    identity: MorphospaceStreamLifecycleIdentity<'_>,
) -> Result<(), MorphospaceStreamLifecycleObservationError> {
    for (field, value) in [
        ("source_id", identity.source_id),
        ("session_id", identity.session_id),
        ("stream_uid", identity.stream_uid),
    ] {
        if value.is_empty() {
            return Err(MorphospaceStreamLifecycleObservationError::EmptyIdentity { field });
        }
        if value.len() > limits.identity_bytes {
            return Err(MorphospaceStreamLifecycleObservationError::IdentityLimit {
                field,
                limit: limits.identity_bytes,
                actual: value.len(),
            });
        }
    }
    Ok(())
}

fn bind_identity(
    expected: MorphospaceStreamLifecycleIdentity<'_>,
    evidence: MorphospaceLifecycleIdentityEvidence<'_>,
    stage: &'static str,
) -> Result<(), MorphospaceStreamLifecycleObservationError> {
    if let MorphospaceLifecycleIdentityEvidence::Observed(actual) = evidence {
        if actual != expected {
            return Err(MorphospaceStreamLifecycleObservationError::IdentityDrift { stage });
        }
    }
    Ok(())
}

fn concrete(address: SocketAddr) -> bool {
    !address.ip().is_unspecified() && !address.ip().is_multicast() && address.port() != 0
}

/// Transactionally validates all facts before publishing one observation.
pub(crate) fn observe_stream_lifecycle<'a>(
    limits: MorphospaceStreamLifecycleObservationLimits,
    identity: MorphospaceStreamLifecycleIdentity<'a>,
    facts: MorphospaceStreamLifecycleFacts<'a>,
) -> Result<MorphospaceStreamLifecycleObservation<'a>, MorphospaceStreamLifecycleObservationError> {
    validate_identity(limits, identity)?;
    bind_identity(identity, facts.selected.identity, "selected_discovery")?;
    if facts.selected.response_index >= facts.selected.response_count {
        return Err(
            MorphospaceStreamLifecycleObservationError::SelectionOutOfBounds {
                selected: facts.selected.response_index,
                responses: facts.selected.response_count,
            },
        );
    }
    if !concrete(facts.selected.response_source) {
        return Err(
            MorphospaceStreamLifecycleObservationError::NonConcretePeer {
                stage: "selected_discovery",
            },
        );
    }
    if !concrete(facts.selected.service_endpoint) {
        return Err(
            MorphospaceStreamLifecycleObservationError::NonConcretePeer {
                stage: "selected_service_endpoint",
            },
        );
    }

    let connected = match facts.connection {
        MorphospaceConnectionObservation::NotAttempted => false,
        MorphospaceConnectionObservation::Refused { identity: value } => {
            bind_identity(identity, value, "connection")?;
            false
        }
        MorphospaceConnectionObservation::Established {
            identity: value,
            peer,
        } => {
            bind_identity(identity, value, "connection")?;
            if !concrete(peer) {
                return Err(
                    MorphospaceStreamLifecycleObservationError::NonConcretePeer {
                        stage: "connection",
                    },
                );
            }
            if peer != facts.selected.service_endpoint {
                return Err(MorphospaceStreamLifecycleObservationError::PeerDrift);
            }
            true
        }
    };

    let processed = match facts.processing {
        MorphospaceRequestedProcessingObservation::NotRequested => 0,
        MorphospaceRequestedProcessingObservation::Refused {
            identity: value,
            retained_record_count,
        } => {
            bind_identity(identity, value, "requested_processing")?;
            if !connected {
                return Err(
                    MorphospaceStreamLifecycleObservationError::LifecycleOrdering {
                        earlier: "connection",
                        later: "requested_processing",
                    },
                );
            }
            retained_record_count
        }
        MorphospaceRequestedProcessingObservation::Completed {
            identity: value,
            record_count,
        } => {
            bind_identity(identity, value, "requested_processing")?;
            if !connected {
                return Err(
                    MorphospaceStreamLifecycleObservationError::LifecycleOrdering {
                        earlier: "connection",
                        later: "requested_processing",
                    },
                );
            }
            record_count
        }
    };
    if processed > limits.record_limit {
        return Err(MorphospaceStreamLifecycleObservationError::RecordLimit {
            limit: limits.record_limit,
            actual: processed,
        });
    }

    let executed = match facts.execution {
        MorphospaceExecutionObservation::NotExecuted => 0,
        MorphospaceExecutionObservation::Observed {
            identity: value,
            budget_cycles,
            committed_cycles,
            report_count,
            completed_record_count,
            stopped_cycle,
            ..
        } => {
            bind_identity(identity, value, "execution")?;
            if matches!(
                facts.processing,
                MorphospaceRequestedProcessingObservation::NotRequested
            ) {
                return Err(
                    MorphospaceStreamLifecycleObservationError::LifecycleOrdering {
                        earlier: "requested_processing",
                        later: "execution",
                    },
                );
            }
            if budget_cycles > limits.cycle_limit {
                return Err(MorphospaceStreamLifecycleObservationError::CycleLimit {
                    limit: limits.cycle_limit,
                    actual: budget_cycles,
                });
            }
            if report_count > limits.report_limit {
                return Err(MorphospaceStreamLifecycleObservationError::ReportLimit {
                    limit: limits.report_limit,
                    actual: report_count,
                });
            }
            if completed_record_count > limits.record_limit {
                return Err(MorphospaceStreamLifecycleObservationError::RecordLimit {
                    limit: limits.record_limit,
                    actual: completed_record_count,
                });
            }
            let valid_extent = match stopped_cycle {
                None => committed_cycles == budget_cycles,
                Some(cycle) => cycle == committed_cycles && cycle < budget_cycles,
            };
            if !valid_extent {
                return Err(
                    MorphospaceStreamLifecycleObservationError::ExecutionExtent {
                        budget: budget_cycles,
                        committed: committed_cycles,
                        stopped_cycle,
                    },
                );
            }
            completed_record_count
        }
    };
    if executed > processed {
        return Err(
            MorphospaceStreamLifecycleObservationError::ProcessingExecutionContradiction {
                processed,
                executed,
            },
        );
    }

    if !connected && !matches!(facts.terminal, MorphospaceTerminalObservation::NotReached) {
        return Err(
            MorphospaceStreamLifecycleObservationError::LifecycleOrdering {
                earlier: "connection",
                later: "terminal",
            },
        );
    }
    if matches!(facts.terminal, MorphospaceTerminalObservation::NotReached)
        && !matches!(
            facts.close,
            MorphospaceCloseObservation::NotObserved | MorphospaceCloseObservation::ReportFreeClose
        )
    {
        return Err(
            MorphospaceStreamLifecycleObservationError::LifecycleOrdering {
                earlier: "terminal",
                later: "close",
            },
        );
    }
    if matches!(facts.close, MorphospaceCloseObservation::NotObserved)
        && !matches!(facts.cleanup, MorphospaceCleanupObservation::NotObserved)
    {
        return Err(MorphospaceStreamLifecycleObservationError::CleanupContradiction);
    }
    if matches!(
        facts.cleanup,
        MorphospaceCleanupObservation::OwnerReleasedResources
    ) && matches!(facts.close, MorphospaceCloseObservation::CloseRefused)
    {
        return Err(MorphospaceStreamLifecycleObservationError::CleanupContradiction);
    }
    if let MorphospaceObservableHealth::Exact {
        processing_observations,
        processing_gaps,
        processing_duplicates,
        completed_execution_prefix,
    } = facts.health
    {
        if processing_observations as usize != processed
            || processing_gaps.saturating_add(processing_duplicates) > processing_observations
            || completed_execution_prefix != executed
        {
            return Err(MorphospaceStreamLifecycleObservationError::HealthContradiction);
        }
    }
    if matches!(
        facts.recovery,
        MorphospaceRecoveryObservation::Exact {
            completed_attempts: 0,
            ..
        }
    ) {
        return Err(MorphospaceStreamLifecycleObservationError::RecoveryContradiction);
    }
    if let MorphospaceLossObservation::Exact { lost_records } = facts.loss {
        if lost_records > processed {
            return Err(MorphospaceStreamLifecycleObservationError::LossContradiction);
        }
    }
    Ok(MorphospaceStreamLifecycleObservation { identity, facts })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity<'a>() -> MorphospaceStreamLifecycleIdentity<'a> {
        MorphospaceStreamLifecycleIdentity::new(7, "source", "session", "uid")
    }
    fn limits() -> MorphospaceStreamLifecycleObservationLimits {
        MorphospaceStreamLifecycleObservationLimits::new(16, 4, 8, 16).unwrap()
    }
    fn facts<'a>() -> MorphospaceStreamLifecycleFacts<'a> {
        let id = MorphospaceLifecycleIdentityEvidence::Observed(identity());
        MorphospaceStreamLifecycleFacts {
            selected: MorphospaceSelectedDiscoveryObservation {
                identity: id,
                response_index: 1,
                response_count: 2,
                response_source: "127.0.0.1:16572".parse().unwrap(),
                service_endpoint: "127.0.0.1:17572".parse().unwrap(),
            },
            connection: MorphospaceConnectionObservation::Established {
                identity: id,
                peer: "127.0.0.1:17572".parse().unwrap(),
            },
            processing: MorphospaceRequestedProcessingObservation::Completed {
                identity: id,
                record_count: 3,
            },
            execution: MorphospaceExecutionObservation::Observed {
                identity: id,
                execution: 9,
                budget_cycles: 2,
                committed_cycles: 2,
                report_count: 3,
                completed_record_count: 3,
                stopped_cycle: None,
            },
            terminal: MorphospaceTerminalObservation::CanonicallyCompleted,
            close: MorphospaceCloseObservation::CanonicalTerminalClose,
            cleanup: MorphospaceCleanupObservation::OwnerReleasedResources,
            loss: MorphospaceLossObservation::NotReportedByAcceptedOwners,
            recovery: MorphospaceRecoveryObservation::Exact {
                completed_attempts: 1,
                exhausted: false,
            },
            health: MorphospaceObservableHealth::Exact {
                processing_observations: 3,
                processing_gaps: 0,
                processing_duplicates: 0,
                completed_execution_prefix: 3,
            },
        }
    }

    #[test]
    fn admits_exact_complete_whole_session_and_preserves_unknown_loss() {
        let observed = observe_stream_lifecycle(limits(), identity(), facts()).unwrap();
        assert_eq!(observed.identity().caller(), 7);
        assert_eq!(observed.identity().source_id(), "source");
        assert_eq!(observed.identity().session_id(), "session");
        assert_eq!(observed.identity().stream_uid(), "uid");
        assert_eq!(
            observed.facts().loss,
            MorphospaceLossObservation::NotReportedByAcceptedOwners
        );
    }

    #[test]
    fn identity_and_peer_drift_refuse_transactionally() {
        let mut drift = facts();
        drift.processing = MorphospaceRequestedProcessingObservation::Completed {
            identity: MorphospaceLifecycleIdentityEvidence::Observed(
                MorphospaceStreamLifecycleIdentity::new(8, "source", "session", "uid"),
            ),
            record_count: 3,
        };
        assert_eq!(
            observe_stream_lifecycle(limits(), identity(), drift),
            Err(MorphospaceStreamLifecycleObservationError::IdentityDrift {
                stage: "requested_processing"
            })
        );
        let mut peer = facts();
        peer.connection = MorphospaceConnectionObservation::Established {
            identity: MorphospaceLifecycleIdentityEvidence::Observed(identity()),
            peer: "127.0.0.1:17573".parse().unwrap(),
        };
        assert_eq!(
            observe_stream_lifecycle(limits(), identity(), peer),
            Err(MorphospaceStreamLifecycleObservationError::PeerDrift)
        );
    }

    #[test]
    fn ordering_cleanup_and_execution_contradictions_fail_closed() {
        let mut ordering = facts();
        ordering.connection = MorphospaceConnectionObservation::NotAttempted;
        assert!(matches!(
            observe_stream_lifecycle(limits(), identity(), ordering),
            Err(MorphospaceStreamLifecycleObservationError::LifecycleOrdering { .. })
        ));
        let mut cleanup = facts();
        cleanup.close = MorphospaceCloseObservation::NotObserved;
        assert_eq!(
            observe_stream_lifecycle(limits(), identity(), cleanup),
            Err(MorphospaceStreamLifecycleObservationError::CleanupContradiction)
        );
        let mut extent = facts();
        extent.execution = MorphospaceExecutionObservation::Observed {
            identity: MorphospaceLifecycleIdentityEvidence::NotExposedByOwner,
            execution: 9,
            budget_cycles: 3,
            committed_cycles: 1,
            report_count: 1,
            completed_record_count: 1,
            stopped_cycle: None,
        };
        assert!(matches!(
            observe_stream_lifecycle(limits(), identity(), extent),
            Err(MorphospaceStreamLifecycleObservationError::ExecutionExtent { .. })
        ));
    }

    #[test]
    fn explicit_not_observed_states_are_admitted_without_inference() {
        let mut partial = facts();
        partial.connection = MorphospaceConnectionObservation::Refused {
            identity: MorphospaceLifecycleIdentityEvidence::NotExposedByOwner,
        };
        partial.processing = MorphospaceRequestedProcessingObservation::NotRequested;
        partial.execution = MorphospaceExecutionObservation::NotExecuted;
        partial.terminal = MorphospaceTerminalObservation::NotReached;
        partial.close = MorphospaceCloseObservation::NotObserved;
        partial.cleanup = MorphospaceCleanupObservation::NotObserved;
        partial.recovery = MorphospaceRecoveryObservation::NotObserved;
        partial.health = MorphospaceObservableHealth::NotObserved;
        assert!(observe_stream_lifecycle(limits(), identity(), partial).is_ok());
    }

    #[test]
    fn finite_bounds_and_health_loss_recovery_claims_are_checked() {
        let mut bounded = facts();
        bounded.execution = MorphospaceExecutionObservation::Observed {
            identity: MorphospaceLifecycleIdentityEvidence::Observed(identity()),
            execution: 9,
            budget_cycles: 5,
            committed_cycles: 5,
            report_count: 3,
            completed_record_count: 3,
            stopped_cycle: None,
        };
        assert!(matches!(
            observe_stream_lifecycle(limits(), identity(), bounded),
            Err(MorphospaceStreamLifecycleObservationError::CycleLimit { .. })
        ));
        let mut health = facts();
        health.health = MorphospaceObservableHealth::Exact {
            processing_observations: 2,
            processing_gaps: 0,
            processing_duplicates: 0,
            completed_execution_prefix: 3,
        };
        assert_eq!(
            observe_stream_lifecycle(limits(), identity(), health),
            Err(MorphospaceStreamLifecycleObservationError::HealthContradiction)
        );
        let mut recovery = facts();
        recovery.recovery = MorphospaceRecoveryObservation::Exact {
            completed_attempts: 0,
            exhausted: true,
        };
        assert_eq!(
            observe_stream_lifecycle(limits(), identity(), recovery),
            Err(MorphospaceStreamLifecycleObservationError::RecoveryContradiction)
        );
        let mut loss = facts();
        loss.loss = MorphospaceLossObservation::Exact { lost_records: 4 };
        assert_eq!(
            observe_stream_lifecycle(limits(), identity(), loss),
            Err(MorphospaceStreamLifecycleObservationError::LossContradiction)
        );
    }
}
