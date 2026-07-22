// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded caller-authored whole-session lifecycle advisory proposals.
//!
//! This module is data-only, default-inert, advisory-only, and non-applying. It validates and
//! retains caller statements; it does not observe native state or execute, connect, close,
//! recover, retry, schedule, mutate a queue, choose a route, admit a peer, grant a lease,
//! authorize, audit, or activate anything. Manifold retains all stream authority.

/// Opaque identity of one proposal, explicitly assigned by its caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleProposalIdentity {
    namespace: u128,
    proposal: u128,
}

impl MorphospaceStreamLifecycleProposalIdentity {
    pub const fn new(namespace: u128, proposal: u128) -> Self {
        Self {
            namespace,
            proposal,
        }
    }

    pub const fn namespace(self) -> u128 {
        self.namespace
    }
    pub const fn proposal(self) -> u128 {
        self.proposal
    }
}

/// Opaque caller-authorship evidence. Neither value authenticates or authorizes the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleCallerProvenance {
    caller: u128,
    authorship: u128,
}

impl MorphospaceStreamLifecycleCallerProvenance {
    pub const fn new(caller: u128, authorship: u128) -> Self {
        Self { caller, authorship }
    }
    pub const fn caller(self) -> u128 {
        self.caller
    }
    pub const fn authorship(self) -> u128 {
        self.authorship
    }
}

/// Exact P64 observation identity and finite extent expected by the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleObservationBinding {
    source: u128,
    execution: u128,
    budget_cycles: usize,
    committed_cycles: usize,
}

impl MorphospaceStreamLifecycleObservationBinding {
    pub const fn new(
        source: u128,
        execution: u128,
        budget_cycles: usize,
        committed_cycles: usize,
    ) -> Self {
        Self {
            source,
            execution,
            budget_cycles,
            committed_cycles,
        }
    }
    pub const fn source(self) -> u128 {
        self.source
    }
    pub const fn execution(self) -> u128 {
        self.execution
    }
    pub const fn budget_cycles(self) -> usize {
        self.budget_cycles
    }
    pub const fn committed_cycles(self) -> usize {
        self.committed_cycles
    }
}

/// Caller-supplied opaque discovery/session/stream identity; no field is inferred or resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleExpectedIdentity {
    discovery_source: u128,
    session: u128,
    stream: u128,
    selected_response_index: usize,
}

impl MorphospaceStreamLifecycleExpectedIdentity {
    pub const fn new(
        discovery_source: u128,
        session: u128,
        stream: u128,
        selected_response_index: usize,
    ) -> Self {
        Self {
            discovery_source,
            session,
            stream,
            selected_response_index,
        }
    }
    pub const fn discovery_source(self) -> u128 {
        self.discovery_source
    }
    pub const fn session(self) -> u128 {
        self.session
    }
    pub const fn stream(self) -> u128 {
        self.stream
    }
    pub const fn selected_response_index(self) -> usize {
        self.selected_response_index
    }
}

/// Every unit of possible work is capped by an explicit, nonzero caller maximum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleAdvisoryBudgets {
    maximum_cycles: usize,
    maximum_records: usize,
    maximum_recovery_attempts: usize,
    maximum_queue_admissions: usize,
    maximum_preconditions: usize,
}

impl MorphospaceStreamLifecycleAdvisoryBudgets {
    pub const fn new(
        maximum_cycles: usize,
        maximum_records: usize,
        maximum_recovery_attempts: usize,
        maximum_queue_admissions: usize,
        maximum_preconditions: usize,
    ) -> Self {
        Self {
            maximum_cycles,
            maximum_records,
            maximum_recovery_attempts,
            maximum_queue_admissions,
            maximum_preconditions,
        }
    }
    pub const fn maximum_cycles(self) -> usize {
        self.maximum_cycles
    }
    pub const fn maximum_records(self) -> usize {
        self.maximum_records
    }
    pub const fn maximum_recovery_attempts(self) -> usize {
        self.maximum_recovery_attempts
    }
    pub const fn maximum_queue_admissions(self) -> usize {
        self.maximum_queue_admissions
    }
    pub const fn maximum_preconditions(self) -> usize {
        self.maximum_preconditions
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleRequestedDisposition {
    Complete,
    StopWithoutCompletion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleRequestedClose {
    CanonicalCompletion,
    ReportFreeCallerClose,
}

/// A requested lifecycle description, never an instruction to a lifecycle owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleIntent {
    disposition: MorphospaceStreamLifecycleRequestedDisposition,
    close: MorphospaceStreamLifecycleRequestedClose,
}

impl MorphospaceStreamLifecycleIntent {
    pub const fn new(
        disposition: MorphospaceStreamLifecycleRequestedDisposition,
        close: MorphospaceStreamLifecycleRequestedClose,
    ) -> Self {
        Self { disposition, close }
    }
    pub const fn disposition(self) -> MorphospaceStreamLifecycleRequestedDisposition {
        self.disposition
    }
    pub const fn requested_close(self) -> MorphospaceStreamLifecycleRequestedClose {
        self.close
    }
}

/// Caller-requested P60 mode and its exact optional configuration bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecyclePostProcessingIntent {
    PassThrough,
    Monotonic {
        history_samples: usize,
        minimum_step_bits: u64,
        maximum_adjustment_bits: u64,
    },
    DeJitter {
        history_samples: usize,
        minimum_step_bits: u64,
        maximum_adjustment_bits: u64,
    },
}

/// Explicit caller assertions. Opposite members of a pair are contradictory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecyclePrecondition {
    SelectedResponseIdentityMatches,
    SelectedResponseIdentityDoesNotMatch,
    SessionNotStarted,
    SessionAlreadyStarted,
    ActivationRemainsCallerOwned,
    ActivationMayBeImplicit,
    ManifoldRetainsStreamAuthority,
    RustyLslMayAssumeStreamAuthority,
}

/// The sole accepted disposition. It has no applying state or operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleAdvisoryDisposition {
    InertAdvisoryOnly,
}

/// Complete caller input, returned byte-for-value unchanged on refusal.
#[derive(Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleAdvisoryDraft {
    pub proposal_identity: MorphospaceStreamLifecycleProposalIdentity,
    pub caller_provenance: MorphospaceStreamLifecycleCallerProvenance,
    pub observation: MorphospaceStreamLifecycleObservationBinding,
    pub expected_identity: MorphospaceStreamLifecycleExpectedIdentity,
    pub budgets: MorphospaceStreamLifecycleAdvisoryBudgets,
    pub lifecycle_intent: MorphospaceStreamLifecycleIntent,
    pub post_processing_intent: MorphospaceStreamLifecyclePostProcessingIntent,
    pub preconditions: Vec<MorphospaceStreamLifecyclePrecondition>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleAdvisoryProposal {
    draft: MorphospaceStreamLifecycleAdvisoryDraft,
    disposition: MorphospaceStreamLifecycleAdvisoryDisposition,
}

impl MorphospaceStreamLifecycleAdvisoryProposal {
    pub const fn draft(&self) -> &MorphospaceStreamLifecycleAdvisoryDraft {
        &self.draft
    }
    pub const fn disposition(&self) -> MorphospaceStreamLifecycleAdvisoryDisposition {
        self.disposition
    }
    pub fn into_draft(self) -> MorphospaceStreamLifecycleAdvisoryDraft {
        self.draft
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceStreamLifecycleAdvisoryRefusal {
    UnboundedCycles,
    UnboundedRecords,
    UnboundedRecovery,
    UnboundedQueueAdmissions,
    UnboundedPreconditions,
    ObservationExceedsCycleBudget { committed: usize, maximum: usize },
    PreconditionsExceedBudget { actual: usize, maximum: usize },
    EmptyPreconditions,
    ContradictoryPreconditions { first: usize, second: usize },
    ContradictoryLifecycleIntent,
    InvalidPostProcessingIntent,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MorphospaceStreamLifecycleAdvisoryError {
    refusal: MorphospaceStreamLifecycleAdvisoryRefusal,
    draft: MorphospaceStreamLifecycleAdvisoryDraft,
}

impl MorphospaceStreamLifecycleAdvisoryError {
    pub const fn refusal(&self) -> MorphospaceStreamLifecycleAdvisoryRefusal {
        self.refusal
    }
    pub fn into_draft(self) -> MorphospaceStreamLifecycleAdvisoryDraft {
        self.draft
    }
}

fn opposite(
    left: MorphospaceStreamLifecyclePrecondition,
    right: MorphospaceStreamLifecyclePrecondition,
) -> bool {
    use MorphospaceStreamLifecyclePrecondition::*;
    matches!(
        (left, right),
        (
            SelectedResponseIdentityMatches,
            SelectedResponseIdentityDoesNotMatch
        ) | (
            SelectedResponseIdentityDoesNotMatch,
            SelectedResponseIdentityMatches
        ) | (SessionNotStarted, SessionAlreadyStarted)
            | (SessionAlreadyStarted, SessionNotStarted)
            | (ActivationRemainsCallerOwned, ActivationMayBeImplicit)
            | (ActivationMayBeImplicit, ActivationRemainsCallerOwned)
            | (
                ManifoldRetainsStreamAuthority,
                RustyLslMayAssumeStreamAuthority
            )
            | (
                RustyLslMayAssumeStreamAuthority,
                ManifoldRetainsStreamAuthority
            )
    )
}

/// Transactionally validates caller-authored data. It performs no native observation or action.
pub fn propose_morphospace_stream_lifecycle_advisory(
    draft: MorphospaceStreamLifecycleAdvisoryDraft,
) -> Result<MorphospaceStreamLifecycleAdvisoryProposal, MorphospaceStreamLifecycleAdvisoryError> {
    macro_rules! refuse {
        ($refusal:expr $(,)?) => {
            return Err(MorphospaceStreamLifecycleAdvisoryError {
                refusal: $refusal,
                draft,
            })
        };
    }
    let budgets = draft.budgets;
    if budgets.maximum_cycles == 0 {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::UnboundedCycles);
    }
    if budgets.maximum_records == 0 {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::UnboundedRecords);
    }
    if budgets.maximum_recovery_attempts == 0 {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::UnboundedRecovery);
    }
    if budgets.maximum_queue_admissions == 0 {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::UnboundedQueueAdmissions);
    }
    if budgets.maximum_preconditions == 0 {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::UnboundedPreconditions);
    }
    if draft.observation.budget_cycles == 0
        || draft.observation.budget_cycles > budgets.maximum_cycles
        || draft.observation.committed_cycles > draft.observation.budget_cycles
    {
        refuse!(
            MorphospaceStreamLifecycleAdvisoryRefusal::ObservationExceedsCycleBudget {
                committed: draft.observation.committed_cycles,
                maximum: budgets.maximum_cycles,
            },
        );
    }
    if draft.preconditions.is_empty() {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::EmptyPreconditions);
    }
    if draft.preconditions.len() > budgets.maximum_preconditions {
        refuse!(
            MorphospaceStreamLifecycleAdvisoryRefusal::PreconditionsExceedBudget {
                actual: draft.preconditions.len(),
                maximum: budgets.maximum_preconditions,
            },
        );
    }
    if matches!(
        (
            draft.lifecycle_intent.disposition,
            draft.lifecycle_intent.close
        ),
        (
            MorphospaceStreamLifecycleRequestedDisposition::Complete,
            MorphospaceStreamLifecycleRequestedClose::ReportFreeCallerClose
        )
    ) {
        refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::ContradictoryLifecycleIntent);
    }
    match draft.post_processing_intent {
        MorphospaceStreamLifecyclePostProcessingIntent::PassThrough => {}
        MorphospaceStreamLifecyclePostProcessingIntent::Monotonic {
            history_samples,
            minimum_step_bits,
            maximum_adjustment_bits,
        }
        | MorphospaceStreamLifecyclePostProcessingIntent::DeJitter {
            history_samples,
            minimum_step_bits,
            maximum_adjustment_bits,
        } => {
            let minimum_step = f64::from_bits(minimum_step_bits);
            let maximum_adjustment = f64::from_bits(maximum_adjustment_bits);
            if !(2..=4096).contains(&history_samples)
                || !minimum_step.is_finite()
                || minimum_step <= 0.0
                || !maximum_adjustment.is_finite()
                || maximum_adjustment < 0.0
            {
                refuse!(MorphospaceStreamLifecycleAdvisoryRefusal::InvalidPostProcessingIntent,);
            }
        }
    }
    for first in 0..draft.preconditions.len() {
        for second in first + 1..draft.preconditions.len() {
            if opposite(draft.preconditions[first], draft.preconditions[second]) {
                refuse!(
                    MorphospaceStreamLifecycleAdvisoryRefusal::ContradictoryPreconditions {
                        first,
                        second,
                    },
                );
            }
        }
    }
    Ok(MorphospaceStreamLifecycleAdvisoryProposal {
        draft,
        disposition: MorphospaceStreamLifecycleAdvisoryDisposition::InertAdvisoryOnly,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> MorphospaceStreamLifecycleAdvisoryDraft {
        MorphospaceStreamLifecycleAdvisoryDraft {
            proposal_identity: MorphospaceStreamLifecycleProposalIdentity::new(1, 2),
            caller_provenance: MorphospaceStreamLifecycleCallerProvenance::new(3, 4),
            observation: MorphospaceStreamLifecycleObservationBinding::new(5, 6, 3, 2),
            expected_identity: MorphospaceStreamLifecycleExpectedIdentity::new(7, 8, 9, 1),
            budgets: MorphospaceStreamLifecycleAdvisoryBudgets::new(3, 32, 2, 32, 4),
            lifecycle_intent: MorphospaceStreamLifecycleIntent::new(
                MorphospaceStreamLifecycleRequestedDisposition::Complete,
                MorphospaceStreamLifecycleRequestedClose::CanonicalCompletion,
            ),
            post_processing_intent: MorphospaceStreamLifecyclePostProcessingIntent::Monotonic {
                history_samples: 4,
                minimum_step_bits: 0.25f64.to_bits(),
                maximum_adjustment_bits: 2.0f64.to_bits(),
            },
            preconditions: vec![
                MorphospaceStreamLifecyclePrecondition::SelectedResponseIdentityMatches,
                MorphospaceStreamLifecyclePrecondition::ActivationRemainsCallerOwned,
                MorphospaceStreamLifecyclePrecondition::ManifoldRetainsStreamAuthority,
            ],
        }
    }

    #[test]
    fn retains_exact_caller_authored_binding_as_inert_advice() {
        let expected = draft();
        let proposal = propose_morphospace_stream_lifecycle_advisory(draft()).unwrap();
        assert_eq!(proposal.draft(), &expected);
        assert_eq!(
            proposal.disposition(),
            MorphospaceStreamLifecycleAdvisoryDisposition::InertAdvisoryOnly
        );
    }

    #[test]
    fn rejects_each_unbounded_work_dimension_transactionally() {
        for index in 0..5 {
            let mut value = draft();
            let b = value.budgets;
            value.budgets = MorphospaceStreamLifecycleAdvisoryBudgets::new(
                if index == 0 { 0 } else { b.maximum_cycles },
                if index == 1 { 0 } else { b.maximum_records },
                if index == 2 {
                    0
                } else {
                    b.maximum_recovery_attempts
                },
                if index == 3 {
                    0
                } else {
                    b.maximum_queue_admissions
                },
                if index == 4 {
                    0
                } else {
                    b.maximum_preconditions
                },
            );
            let identity = value.proposal_identity;
            let error = propose_morphospace_stream_lifecycle_advisory(value).unwrap_err();
            assert_eq!(error.into_draft().proposal_identity, identity);
        }
    }

    #[test]
    fn contradictory_preconditions_and_intent_return_complete_draft() {
        let mut value = draft();
        value
            .preconditions
            .push(MorphospaceStreamLifecyclePrecondition::ActivationMayBeImplicit);
        let error = propose_morphospace_stream_lifecycle_advisory(value).unwrap_err();
        assert_eq!(
            error.refusal(),
            MorphospaceStreamLifecycleAdvisoryRefusal::ContradictoryPreconditions {
                first: 1,
                second: 3
            }
        );
        assert_eq!(error.into_draft().preconditions.len(), 4);
        let mut value = draft();
        value.lifecycle_intent = MorphospaceStreamLifecycleIntent::new(
            MorphospaceStreamLifecycleRequestedDisposition::Complete,
            MorphospaceStreamLifecycleRequestedClose::ReportFreeCallerClose,
        );
        assert_eq!(
            propose_morphospace_stream_lifecycle_advisory(value)
                .unwrap_err()
                .refusal(),
            MorphospaceStreamLifecycleAdvisoryRefusal::ContradictoryLifecycleIntent
        );
    }

    #[test]
    fn malformed_processing_and_observation_extents_refuse() {
        let mut value = draft();
        value.post_processing_intent = MorphospaceStreamLifecyclePostProcessingIntent::DeJitter {
            history_samples: 1,
            minimum_step_bits: 0.0f64.to_bits(),
            maximum_adjustment_bits: f64::NAN.to_bits(),
        };
        assert_eq!(
            propose_morphospace_stream_lifecycle_advisory(value)
                .unwrap_err()
                .refusal(),
            MorphospaceStreamLifecycleAdvisoryRefusal::InvalidPostProcessingIntent
        );
        let mut value = draft();
        value.observation = MorphospaceStreamLifecycleObservationBinding::new(5, 6, 2, 3);
        assert!(matches!(
            propose_morphospace_stream_lifecycle_advisory(value)
                .unwrap_err()
                .refusal(),
            MorphospaceStreamLifecycleAdvisoryRefusal::ObservationExceedsCycleBudget { .. }
        ));
    }

    #[test]
    fn source_declares_advisory_only_authority_boundary() {
        let source = include_str!("morphospace_stream_lifecycle_advisory_proposal.rs");
        for phrase in [
            "default-inert",
            "advisory-only",
            "does not observe native state",
            "Manifold retains all stream authority",
        ] {
            assert!(source.contains(phrase));
        }
        for operation in [
            concat!("fn con", "nect("),
            concat!("fn clo", "se("),
            concat!("fn auth", "orize("),
            concat!("fn ap", "ply("),
        ] {
            assert!(!source.contains(operation));
        }
    }
}
