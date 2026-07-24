// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional advisory composition over one exact P64 execution observation.
//!
//! This CPU/data-only facade validates caller-owned provenance and then retains the exact
//! observation. It performs no action and grants no execution or Manifold authority.

use crate::morphospace_requested_processing_execution_advisory_proposal::classify_requested_processing_execution_cycle;
use crate::morphospace_requested_processing_execution_observation::{
    observe_complete_requested_processing_execution,
    observe_stopped_requested_processing_execution,
    MorphospaceRequestedProcessingExecutionIdentity,
    MorphospaceRequestedProcessingExecutionObservation,
    MorphospaceRequestedProcessingExecutionObservationError,
    MorphospaceRequestedProcessingExecutionObservationLimits,
};
use crate::{
    CompleteRequestedProcessingRecoveryQueueExecutionBatch,
    RequestedProcessingRecoveryQueueExecutionBatchStopped,
    RequestedProcessingRecoveryQueueSupervision,
};

/// Exact caller-supplied facts expected for one committed cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceRequestedProcessingExecutionCycleProvenance {
    report_count: usize,
    total_execution_count: usize,
    first_completed_execution_count: usize,
    last_completed_execution_count: usize,
    last_remaining_execution_count: usize,
    last_current_execution_index: Option<usize>,
}

impl MorphospaceRequestedProcessingExecutionCycleProvenance {
    /// Creates an uninterpreted exact cycle provenance value.
    pub const fn new(
        report_count: usize,
        total_execution_count: usize,
        first_completed_execution_count: usize,
        last_completed_execution_count: usize,
        last_remaining_execution_count: usize,
        last_current_execution_index: Option<usize>,
    ) -> Self {
        Self {
            report_count,
            total_execution_count,
            first_completed_execution_count,
            last_completed_execution_count,
            last_remaining_execution_count,
            last_current_execution_index,
        }
    }
}

/// Caller-owned provenance that must match the derived observation exactly.
#[derive(Clone, Copy, Debug)]
pub struct MorphospaceRequestedProcessingExecutionAdvisoryProvenance<'a> {
    source: u128,
    execution: u128,
    budget_cycles: usize,
    cycles: &'a [MorphospaceRequestedProcessingExecutionCycleProvenance],
}

impl<'a> MorphospaceRequestedProcessingExecutionAdvisoryProvenance<'a> {
    /// Creates provenance without interpreting either opaque identity component.
    pub const fn new(
        source: u128,
        execution: u128,
        budget_cycles: usize,
        cycles: &'a [MorphospaceRequestedProcessingExecutionCycleProvenance],
    ) -> Self {
        Self {
            source,
            execution,
            budget_cycles,
            cycles,
        }
    }
}

/// Explicit nonzero bounds for observation and advisory projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MorphospaceRequestedProcessingExecutionAdvisoryLimits {
    cycle_limit: usize,
    report_limit: usize,
}

impl MorphospaceRequestedProcessingExecutionAdvisoryLimits {
    /// Creates finite bounds, rejecting either zero limit.
    pub const fn new(
        cycle_limit: usize,
        report_limit: usize,
    ) -> Result<Self, MorphospaceRequestedProcessingExecutionAdvisoryLimitsError> {
        if cycle_limit == 0 {
            return Err(MorphospaceRequestedProcessingExecutionAdvisoryLimitsError::ZeroCycleLimit);
        }
        if report_limit == 0 {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryLimitsError::ZeroReportLimit,
            );
        }
        Ok(Self {
            cycle_limit,
            report_limit,
        })
    }
}

/// Invalid advisory-limit configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceRequestedProcessingExecutionAdvisoryLimitsError {
    /// The maximum cycle count was zero.
    ZeroCycleLimit,
    /// The aggregate report maximum was zero.
    ZeroReportLimit,
}

/// Descriptive classification only; neither variant requests behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceRequestedProcessingExecutionAdvisoryClassification {
    /// Every committed cycle has no remaining execution and no current index.
    AllCommittedCyclesComplete,
    /// At least one committed cycle retains an incomplete final fact.
    IncompleteCommittedCyclePresent,
    /// No cycle committed before the exact P63 refusal.
    NoCommittedCycle,
}

/// Immutable proposal retaining the exact observation from which it was derived.
#[derive(Debug)]
pub struct MorphospaceRequestedProcessingExecutionAdvisoryProposal<'observation, 'evidence> {
    observation: MorphospaceRequestedProcessingExecutionObservation<'observation, 'evidence>,
    classification: MorphospaceRequestedProcessingExecutionAdvisoryClassification,
}

impl<'observation, 'evidence>
    MorphospaceRequestedProcessingExecutionAdvisoryProposal<'observation, 'evidence>
{
    /// Returns the exact opaque source identity.
    pub const fn source(&self) -> u128 {
        self.observation.identity().source()
    }
    /// Returns the exact opaque execution identity.
    pub const fn execution(&self) -> u128 {
        self.observation.identity().execution()
    }
    /// Returns the caller's exact finite execution budget.
    pub const fn budget_cycles(&self) -> usize {
        self.observation.budget_cycles()
    }
    /// Returns the number of exactly retained committed cycles.
    pub fn committed_cycle_count(&self) -> usize {
        self.observation.cycles().len()
    }
    /// Returns the descriptive advisory classification.
    pub const fn classification(
        &self,
    ) -> MorphospaceRequestedProcessingExecutionAdvisoryClassification {
        self.classification
    }
}

/// Transactional refusal; no proposal or partial observation is returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MorphospaceRequestedProcessingExecutionAdvisoryError {
    /// The exact observation projection refused its input.
    ObservationRefused,
    /// Either opaque identity component differed from the bound provenance.
    IdentityDrift,
    /// The finite budget differed from the bound provenance.
    BudgetExtentDrift {
        /// Budget bound by the caller's provenance.
        expected: usize,
        /// Budget retained by the exact observation.
        actual: usize,
    },
    /// The committed-cycle count differed from the bound provenance.
    CycleExtentDrift {
        /// Committed-cycle count bound by the caller's provenance.
        expected: usize,
        /// Committed-cycle count retained by the exact observation.
        actual: usize,
    },
    /// One exact committed-cycle fact differed from the bound provenance.
    CycleFactDrift {
        /// Zero-based committed cycle containing the first differing fact.
        cycle: usize,
    },
}

fn derive<'observation, 'evidence>(
    provenance: MorphospaceRequestedProcessingExecutionAdvisoryProvenance<'_>,
    observation: MorphospaceRequestedProcessingExecutionObservation<'observation, 'evidence>,
) -> Result<
    MorphospaceRequestedProcessingExecutionAdvisoryProposal<'observation, 'evidence>,
    MorphospaceRequestedProcessingExecutionAdvisoryError,
> {
    let identity = observation.identity();
    if (identity.source(), identity.execution()) != (provenance.source, provenance.execution) {
        return Err(MorphospaceRequestedProcessingExecutionAdvisoryError::IdentityDrift);
    }
    if observation.budget_cycles() != provenance.budget_cycles {
        return Err(
            MorphospaceRequestedProcessingExecutionAdvisoryError::BudgetExtentDrift {
                expected: provenance.budget_cycles,
                actual: observation.budget_cycles(),
            },
        );
    }
    if observation.cycles().len() != provenance.cycles.len() {
        return Err(
            MorphospaceRequestedProcessingExecutionAdvisoryError::CycleExtentDrift {
                expected: provenance.cycles.len(),
                actual: observation.cycles().len(),
            },
        );
    }
    let mut all_complete = !observation.cycles().is_empty();
    for (cycle, (actual, expected)) in observation
        .cycles()
        .iter()
        .zip(provenance.cycles)
        .enumerate()
    {
        let exact = MorphospaceRequestedProcessingExecutionCycleProvenance::new(
            actual.report_count(),
            actual.total_execution_count(),
            actual.first_completed_execution_count(),
            actual.last_completed_execution_count(),
            actual.last_remaining_execution_count(),
            actual.last_current_execution_index(),
        );
        if exact != *expected {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::CycleFactDrift { cycle },
            );
        }
        let Some(complete) = classify_requested_processing_execution_cycle(
            actual.total_execution_count(),
            actual.last_completed_execution_count(),
            actual.last_remaining_execution_count(),
            actual.last_current_execution_index(),
        ) else {
            return Err(
                MorphospaceRequestedProcessingExecutionAdvisoryError::CycleFactDrift { cycle },
            );
        };
        all_complete &= complete;
    }
    let classification = if observation.cycles().is_empty() {
        MorphospaceRequestedProcessingExecutionAdvisoryClassification::NoCommittedCycle
    } else if all_complete {
        MorphospaceRequestedProcessingExecutionAdvisoryClassification::AllCommittedCyclesComplete
    } else {
        MorphospaceRequestedProcessingExecutionAdvisoryClassification::IncompleteCommittedCyclePresent
    };
    Ok(MorphospaceRequestedProcessingExecutionAdvisoryProposal {
        observation,
        classification,
    })
}

/// Observes one completed P63 batch and transactionally derives its advisory proposal.
pub fn propose_complete_requested_processing_execution_advisory<'observation, 'evidence>(
    limits: MorphospaceRequestedProcessingExecutionAdvisoryLimits,
    source: u128,
    execution: u128,
    provenance: MorphospaceRequestedProcessingExecutionAdvisoryProvenance<'_>,
    complete: &'observation CompleteRequestedProcessingRecoveryQueueExecutionBatch<'evidence>,
) -> Result<
    MorphospaceRequestedProcessingExecutionAdvisoryProposal<'observation, 'evidence>,
    MorphospaceRequestedProcessingExecutionAdvisoryError,
> {
    let observation = observe_complete_requested_processing_execution(
        MorphospaceRequestedProcessingExecutionObservationLimits::new(
            limits.cycle_limit,
            limits.report_limit,
        )
        .expect("public limits are nonzero"),
        MorphospaceRequestedProcessingExecutionIdentity::new(source, execution),
        complete,
    )
    .map_err(
        |_: MorphospaceRequestedProcessingExecutionObservationError| {
            MorphospaceRequestedProcessingExecutionAdvisoryError::ObservationRefused
        },
    )?;
    derive(provenance, observation)
}

/// Observes one stopped P63 batch and transactionally derives its advisory proposal.
pub fn propose_stopped_requested_processing_execution_advisory<'observation, 'evidence>(
    limits: MorphospaceRequestedProcessingExecutionAdvisoryLimits,
    source: u128,
    execution: u128,
    provenance: MorphospaceRequestedProcessingExecutionAdvisoryProvenance<'_>,
    stopped: &'observation RequestedProcessingRecoveryQueueExecutionBatchStopped<'evidence>,
    supervision: &[RequestedProcessingRecoveryQueueSupervision],
) -> Result<
    MorphospaceRequestedProcessingExecutionAdvisoryProposal<'observation, 'evidence>,
    MorphospaceRequestedProcessingExecutionAdvisoryError,
> {
    let observation = observe_stopped_requested_processing_execution(
        MorphospaceRequestedProcessingExecutionObservationLimits::new(
            limits.cycle_limit,
            limits.report_limit,
        )
        .expect("public limits are nonzero"),
        MorphospaceRequestedProcessingExecutionIdentity::new(source, execution),
        stopped,
        supervision,
    )
    .map_err(
        |_: MorphospaceRequestedProcessingExecutionObservationError| {
            MorphospaceRequestedProcessingExecutionAdvisoryError::ObservationRefused
        },
    )?;
    derive(provenance, observation)
}
