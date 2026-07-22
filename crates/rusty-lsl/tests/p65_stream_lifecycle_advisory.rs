// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! P65 public facade composition checks.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use rusty_lsl::*;

fn identity() -> MorphospaceStreamLifecycleIdentity<'static> {
    MorphospaceStreamLifecycleIdentity {
        caller: 11,
        source_id: "source-a",
        session_id: "session-a",
        stream_uid: "stream-a",
    }
}

fn facts() -> MorphospaceStreamLifecycleFacts<'static> {
    let id = MorphospaceStreamLifecycleIdentityEvidence::Observed(identity());
    let peer = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 16572);
    MorphospaceStreamLifecycleFacts {
        selection: MorphospaceStreamLifecycleSelection {
            identity: id,
            response_index: 1,
            response_count: 2,
            response_source: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 16571),
            service_endpoint: peer,
        },
        connection: MorphospaceStreamLifecycleConnection::Established { identity: id, peer },
        processing: MorphospaceStreamLifecycleProcessing::Completed {
            identity: id,
            record_count: 2,
        },
        execution: MorphospaceStreamLifecycleExecution::Observed {
            identity: id,
            source: 21,
            execution: 22,
            budget_cycles: 2,
            committed_cycles: 2,
            report_count: 2,
            completed_record_count: 2,
            stopped_cycle: None,
        },
        terminal: MorphospaceStreamLifecycleTerminal::CanonicallyCompleted,
        close: MorphospaceStreamLifecycleClose::CanonicalTerminalClose,
        cleanup: MorphospaceStreamLifecycleCleanup::OwnerReleasedResources,
        loss: MorphospaceStreamLifecycleLoss::NotReportedByAcceptedOwners,
        recovery: MorphospaceStreamLifecycleRecovery::NotObserved,
        health: MorphospaceStreamLifecycleHealth::NotObserved,
    }
}

fn draft() -> MorphospaceStreamLifecycleAdvisoryDraft {
    MorphospaceStreamLifecycleAdvisoryDraft {
        proposal_identity: MorphospaceStreamLifecycleProposalIdentity::new(31, 32),
        caller_provenance: MorphospaceStreamLifecycleCallerProvenance::new(11, 12),
        observation: MorphospaceStreamLifecycleObservationBinding::new(21, 22, 2, 2),
        expected_identity: MorphospaceStreamLifecycleExpectedIdentity::new(41, 42, 43, 1),
        budgets: MorphospaceStreamLifecycleAdvisoryBudgets::new(2, 2, 1, 2, 3),
        lifecycle_intent: MorphospaceStreamLifecycleIntent::new(
            MorphospaceStreamLifecycleRequestedDisposition::Complete,
            MorphospaceStreamLifecycleRequestedClose::CanonicalCompletion,
        ),
        post_processing_intent: MorphospaceStreamLifecyclePostProcessingIntent::PassThrough,
        preconditions: vec![
            MorphospaceStreamLifecyclePrecondition::SelectedResponseIdentityMatches,
            MorphospaceStreamLifecyclePrecondition::ActivationRemainsCallerOwned,
            MorphospaceStreamLifecyclePrecondition::ManifoldRetainsStreamAuthority,
        ],
    }
}

fn binding() -> MorphospaceStreamLifecycleAdvisoryBinding<'static> {
    MorphospaceStreamLifecycleAdvisoryBinding {
        caller: 11,
        authorship: 12,
        source_id: "source-a",
        session_id: "session-a",
        stream_uid: "stream-a",
        discovery_source: 41,
        session: 42,
        stream: 43,
        post_processing_intent: MorphospaceStreamLifecyclePostProcessingIntent::PassThrough,
        cleanup: MorphospaceStreamLifecycleCleanup::OwnerReleasedResources,
        loss: MorphospaceStreamLifecycleLoss::NotReportedByAcceptedOwners,
        recovery: MorphospaceStreamLifecycleRecovery::NotObserved,
        health: MorphospaceStreamLifecycleHealth::NotObserved,
    }
}

#[test]
fn p65_composes_exact_evidence_and_preserves_unknown_facts_as_inert_advice() {
    let composed = compose_morphospace_stream_lifecycle_advisory(
        MorphospaceStreamLifecycleLimits::new(32, 2, 2, 2).unwrap(),
        identity(),
        facts(),
        binding(),
        draft(),
    )
    .unwrap();
    assert_eq!(composed.identity(), identity());
    assert_eq!(composed.facts(), facts());
    assert_eq!(
        composed.facts().loss,
        MorphospaceStreamLifecycleLoss::NotReportedByAcceptedOwners
    );
    assert_eq!(
        composed.facts().recovery,
        MorphospaceStreamLifecycleRecovery::NotObserved
    );
    assert_eq!(
        composed.facts().health,
        MorphospaceStreamLifecycleHealth::NotObserved
    );
    assert_eq!(
        composed.proposal().disposition(),
        MorphospaceStreamLifecycleAdvisoryDisposition::InertAdvisoryOnly
    );
}

#[test]
fn p65_rejects_identity_execution_selection_and_lifecycle_drift_transactionally() {
    let limits = MorphospaceStreamLifecycleLimits::new(32, 2, 2, 2).unwrap();
    let mut wrong_binding = binding();
    wrong_binding.stream_uid = "other";
    assert!(matches!(
        compose_morphospace_stream_lifecycle_advisory(
            limits,
            identity(),
            facts(),
            wrong_binding,
            draft()
        ),
        Err(MorphospaceStreamLifecycleAdvisoryCompositionError::IdentityOrProvenanceDrift)
    ));
    let mut wrong_draft = draft();
    wrong_draft.observation = MorphospaceStreamLifecycleObservationBinding::new(21, 99, 2, 2);
    assert!(matches!(
        compose_morphospace_stream_lifecycle_advisory(
            limits,
            identity(),
            facts(),
            binding(),
            wrong_draft
        ),
        Err(MorphospaceStreamLifecycleAdvisoryCompositionError::ExecutionBindingDrift)
    ));
    let mut wrong_selection = draft();
    wrong_selection.expected_identity =
        MorphospaceStreamLifecycleExpectedIdentity::new(41, 42, 43, 0);
    assert!(matches!(
        compose_morphospace_stream_lifecycle_advisory(
            limits,
            identity(),
            facts(),
            binding(),
            wrong_selection
        ),
        Err(MorphospaceStreamLifecycleAdvisoryCompositionError::SelectionBindingDrift)
    ));
    let mut wrong_order = facts();
    wrong_order.connection = MorphospaceStreamLifecycleConnection::NotAttempted;
    assert!(matches!(
        compose_morphospace_stream_lifecycle_advisory(
            limits,
            identity(),
            wrong_order,
            binding(),
            draft()
        ),
        Err(MorphospaceStreamLifecycleAdvisoryCompositionError::ObservationRefused)
    ));
    let mut wrong_cleanup = binding();
    wrong_cleanup.cleanup = MorphospaceStreamLifecycleCleanup::NotObserved;
    assert!(matches!(
        compose_morphospace_stream_lifecycle_advisory(
            limits,
            identity(),
            facts(),
            wrong_cleanup,
            draft()
        ),
        Err(MorphospaceStreamLifecycleAdvisoryCompositionError::LifecycleFactDrift)
    ));
    let mut narrow = draft();
    narrow.budgets = MorphospaceStreamLifecycleAdvisoryBudgets::new(2, 1, 1, 2, 3);
    assert!(matches!(
        compose_morphospace_stream_lifecycle_advisory(
            limits,
            identity(),
            facts(),
            binding(),
            narrow
        ),
        Err(MorphospaceStreamLifecycleAdvisoryCompositionError::BudgetDrift)
    ));
}

#[test]
fn p65_root_and_runtime_facades_have_the_same_composition_surface() {
    fn same<T>(_: &T, _: &T) {}
    same(
        &rusty_lsl::compose_morphospace_stream_lifecycle_advisory,
        &rusty_lsl::runtime::compose_morphospace_stream_lifecycle_advisory,
    );
    assert!(
        core::mem::size_of::<rusty_lsl::runtime::MorphospaceStreamLifecycleAdvisory<'static>>() > 0
    );
}
