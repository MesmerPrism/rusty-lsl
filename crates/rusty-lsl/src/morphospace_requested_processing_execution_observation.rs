// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded immutable Morphospace observation of exact P63 execution facts.
//!
//! The projection is data-only. It preserves caller identity and existing P63 facts, while
//! explicitly representing facts that P62/P63 do not report. It makes no recommendation and
//! grants no policy, route, authorization, activation, device, Makepad, or Manifold authority.

use crate::{
    CompleteRequestedProcessingRecoveryQueueExecutionBatch, RequestedProcessingExecutionHealth,
    RequestedProcessingRecoveryQueueExecutionBatchStopped,
    RequestedProcessingRecoveryQueueExecutionError, RequestedProcessingRecoveryQueueSupervision,
    RequestedProcessingSupervisionLossFacts, RequestedProcessingSupervisionOwnerFacts,
    RequestedProcessingSupervisionTerminations,
};

/// Caller-owned opaque identity. P64 does not interpret either component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceRequestedProcessingExecutionIdentity {
    source: u128,
    execution: u128,
}

impl MorphospaceRequestedProcessingExecutionIdentity {
    pub(crate) const fn new(source: u128, execution: u128) -> Self {
        Self { source, execution }
    }

    pub(crate) const fn source(self) -> u128 {
        self.source
    }

    pub(crate) const fn execution(self) -> u128 {
        self.execution
    }
}

/// Explicit finite bounds for one observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceRequestedProcessingExecutionObservationLimits {
    cycle_limit: usize,
    report_limit: usize,
}

impl MorphospaceRequestedProcessingExecutionObservationLimits {
    pub(crate) const fn new(
        cycle_limit: usize,
        report_limit: usize,
    ) -> Result<Self, MorphospaceRequestedProcessingExecutionObservationConfigError> {
        if cycle_limit == 0 {
            return Err(
                MorphospaceRequestedProcessingExecutionObservationConfigError::ZeroCycleLimit,
            );
        }
        if report_limit == 0 {
            return Err(
                MorphospaceRequestedProcessingExecutionObservationConfigError::ZeroReportLimit,
            );
        }
        Ok(Self {
            cycle_limit,
            report_limit,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingExecutionObservationConfigError {
    ZeroCycleLimit,
    ZeroReportLimit,
}

/// Queue length is either an exact last P62 fact or explicitly absent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingQueueLengthObservation {
    Reported(usize),
    NotReportedByP62,
}

/// Exact immutable facts for one committed P63 cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceRequestedProcessingExecutionCycleObservation {
    cycle: usize,
    report_count: usize,
    total_execution_count: usize,
    first_completed_execution_count: usize,
    last_completed_execution_count: usize,
    last_remaining_execution_count: usize,
    last_current_execution_index: Option<usize>,
    terminations: RequestedProcessingSupervisionTerminations,
    owner_facts: RequestedProcessingSupervisionOwnerFacts,
    queue_length: MorphospaceRequestedProcessingQueueLengthObservation,
    loss_facts: RequestedProcessingSupervisionLossFacts,
    last_health: RequestedProcessingExecutionHealth,
}

impl MorphospaceRequestedProcessingExecutionCycleObservation {
    pub(crate) const fn cycle(self) -> usize {
        self.cycle
    }
    pub(crate) const fn report_count(self) -> usize {
        self.report_count
    }
    pub(crate) const fn total_execution_count(self) -> usize {
        self.total_execution_count
    }
    pub(crate) const fn first_completed_execution_count(self) -> usize {
        self.first_completed_execution_count
    }
    pub(crate) const fn last_completed_execution_count(self) -> usize {
        self.last_completed_execution_count
    }
    pub(crate) const fn last_remaining_execution_count(self) -> usize {
        self.last_remaining_execution_count
    }
    pub(crate) const fn last_current_execution_index(self) -> Option<usize> {
        self.last_current_execution_index
    }
    pub(crate) const fn terminations(self) -> RequestedProcessingSupervisionTerminations {
        self.terminations
    }
    pub(crate) const fn owner_facts(self) -> RequestedProcessingSupervisionOwnerFacts {
        self.owner_facts
    }
    pub(crate) const fn queue_length(self) -> MorphospaceRequestedProcessingQueueLengthObservation {
        self.queue_length
    }
    pub(crate) const fn loss_facts(self) -> RequestedProcessingSupervisionLossFacts {
        self.loss_facts
    }
    pub(crate) const fn last_health(self) -> RequestedProcessingExecutionHealth {
        self.last_health
    }
}

/// Exact P63 batch stop fact. A refusal remains borrowed from its original owner.
#[derive(Debug)]
pub(crate) enum MorphospaceRequestedProcessingExecutionStop<'observation, 'evidence> {
    BudgetCompleted,
    ExecutionRefused {
        cycle: usize,
        cause: &'observation RequestedProcessingRecoveryQueueExecutionError<'evidence>,
        supervision: MorphospaceRequestedProcessingStoppedCycleSupervision,
    },
}

/// P63 commits no report series for its refused current cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingStoppedCycleSupervision {
    NotReportedForUncommittedCycle,
}

/// Immutable bounded projection. Cycle order is the exact P63 committed-prefix order.
#[derive(Debug)]
pub(crate) struct MorphospaceRequestedProcessingExecutionObservation<'observation, 'evidence> {
    identity: MorphospaceRequestedProcessingExecutionIdentity,
    budget_cycles: usize,
    cycles: Vec<MorphospaceRequestedProcessingExecutionCycleObservation>,
    stop: MorphospaceRequestedProcessingExecutionStop<'observation, 'evidence>,
}

impl<'observation, 'evidence>
    MorphospaceRequestedProcessingExecutionObservation<'observation, 'evidence>
{
    pub(crate) const fn identity(&self) -> MorphospaceRequestedProcessingExecutionIdentity {
        self.identity
    }
    pub(crate) const fn budget_cycles(&self) -> usize {
        self.budget_cycles
    }
    pub(crate) fn cycles(&self) -> &[MorphospaceRequestedProcessingExecutionCycleObservation] {
        &self.cycles
    }
    pub(crate) const fn stop(
        &self,
    ) -> &MorphospaceRequestedProcessingExecutionStop<'observation, 'evidence> {
        &self.stop
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceRequestedProcessingExecutionObservationError {
    CycleLimit {
        limit: usize,
        actual: usize,
    },
    CycleAssociation {
        committed: usize,
        supervision: usize,
    },
    CompletedBudgetContradiction {
        budget: usize,
        committed: usize,
    },
    StoppedCycleContradiction {
        budget: usize,
        stopped_cycle: usize,
        committed: usize,
    },
    ReportLimit {
        limit: usize,
        actual: usize,
    },
    ReportCountOverflow,
    CommittedExtentContradiction {
        cycle: usize,
        queued: usize,
        supervised: usize,
    },
    Allocation {
        requested: usize,
    },
}

#[derive(Clone, Copy)]
struct ProjectionInput<'a> {
    budget: usize,
    committed_extents: &'a [usize],
    supervision: &'a [RequestedProcessingRecoveryQueueSupervision],
    stop_cycle: Option<usize>,
}

fn validate_input(
    limits: MorphospaceRequestedProcessingExecutionObservationLimits,
    input: ProjectionInput<'_>,
) -> Result<usize, MorphospaceRequestedProcessingExecutionObservationError> {
    let committed = input.committed_extents.len();
    if input.budget > limits.cycle_limit {
        return Err(
            MorphospaceRequestedProcessingExecutionObservationError::CycleLimit {
                limit: limits.cycle_limit,
                actual: input.budget,
            },
        );
    }
    if input.supervision.len() != committed {
        return Err(
            MorphospaceRequestedProcessingExecutionObservationError::CycleAssociation {
                committed,
                supervision: input.supervision.len(),
            },
        );
    }
    match input.stop_cycle {
        None if committed != input.budget => return Err(
            MorphospaceRequestedProcessingExecutionObservationError::CompletedBudgetContradiction {
                budget: input.budget,
                committed,
            },
        ),
        Some(stopped_cycle) if stopped_cycle != committed || stopped_cycle >= input.budget => {
            return Err(
                MorphospaceRequestedProcessingExecutionObservationError::StoppedCycleContradiction {
                    budget: input.budget,
                    stopped_cycle,
                    committed,
                },
            );
        }
        _ => {}
    }

    let mut reports = 0usize;
    for (cycle, (queued, supervised)) in input
        .committed_extents
        .iter()
        .zip(input.supervision)
        .enumerate()
    {
        if *queued != supervised.total_execution_count() {
            return Err(
                MorphospaceRequestedProcessingExecutionObservationError::CommittedExtentContradiction {
                    cycle,
                    queued: *queued,
                    supervised: supervised.total_execution_count(),
                },
            );
        }
        reports = reports
            .checked_add(supervised.report_count())
            .ok_or(MorphospaceRequestedProcessingExecutionObservationError::ReportCountOverflow)?;
        if reports > limits.report_limit {
            return Err(
                MorphospaceRequestedProcessingExecutionObservationError::ReportLimit {
                    limit: limits.report_limit,
                    actual: reports,
                },
            );
        }
    }
    Ok(reports)
}

fn project_cycles(
    supervision: &[RequestedProcessingRecoveryQueueSupervision],
) -> Result<
    Vec<MorphospaceRequestedProcessingExecutionCycleObservation>,
    MorphospaceRequestedProcessingExecutionObservationError,
> {
    let mut cycles = Vec::new();
    cycles.try_reserve_exact(supervision.len()).map_err(|_| {
        MorphospaceRequestedProcessingExecutionObservationError::Allocation {
            requested: supervision.len(),
        }
    })?;
    for (cycle, value) in supervision.iter().copied().enumerate() {
        let owner_facts = value.owner_facts();
        cycles.push(MorphospaceRequestedProcessingExecutionCycleObservation {
            cycle,
            report_count: value.report_count(),
            total_execution_count: value.total_execution_count(),
            first_completed_execution_count: value.first_completed_execution_count(),
            last_completed_execution_count: value.last_completed_execution_count(),
            last_remaining_execution_count: value.last_remaining_execution_count(),
            last_current_execution_index: value.last_current_execution_index(),
            terminations: value.terminations(),
            owner_facts,
            queue_length: match owner_facts.last_queue_len {
                Some(length) => {
                    MorphospaceRequestedProcessingQueueLengthObservation::Reported(length)
                }
                None => MorphospaceRequestedProcessingQueueLengthObservation::NotReportedByP62,
            },
            loss_facts: value.loss_facts(),
            last_health: value.last_health(),
        });
    }
    Ok(cycles)
}

fn collect_committed_extents<'borrow, 'evidence>(
    limit: usize,
    committed: impl ExactSizeIterator<
        Item = &'borrow crate::RequestedProcessingRecoveryQueueExecutionOutcome<'evidence>,
    >,
) -> Result<Vec<usize>, MorphospaceRequestedProcessingExecutionObservationError>
where
    'evidence: 'borrow,
{
    let actual = committed.len();
    if actual > limit {
        return Err(
            MorphospaceRequestedProcessingExecutionObservationError::CycleLimit { limit, actual },
        );
    }
    let mut extents = Vec::new();
    extents.try_reserve_exact(actual).map_err(|_| {
        MorphospaceRequestedProcessingExecutionObservationError::Allocation { requested: actual }
    })?;
    extents.extend(committed.map(|value| value.queued.len()));
    Ok(extents)
}

pub(crate) fn observe_complete_requested_processing_execution<'observation, 'evidence>(
    limits: MorphospaceRequestedProcessingExecutionObservationLimits,
    identity: MorphospaceRequestedProcessingExecutionIdentity,
    complete: &'observation CompleteRequestedProcessingRecoveryQueueExecutionBatch<'evidence>,
) -> Result<
    MorphospaceRequestedProcessingExecutionObservation<'observation, 'evidence>,
    MorphospaceRequestedProcessingExecutionObservationError,
> {
    let batch = complete.batch();
    let committed_extents =
        collect_committed_extents(limits.cycle_limit, batch.committed().iter())?;
    validate_input(
        limits,
        ProjectionInput {
            budget: batch.budget().cycles(),
            committed_extents: &committed_extents,
            supervision: complete.supervision(),
            stop_cycle: None,
        },
    )?;
    let cycles = project_cycles(complete.supervision())?;
    Ok(MorphospaceRequestedProcessingExecutionObservation {
        identity,
        budget_cycles: batch.budget().cycles(),
        cycles,
        stop: MorphospaceRequestedProcessingExecutionStop::BudgetCompleted,
    })
}

pub(crate) fn observe_stopped_requested_processing_execution<'observation, 'evidence>(
    limits: MorphospaceRequestedProcessingExecutionObservationLimits,
    identity: MorphospaceRequestedProcessingExecutionIdentity,
    stopped: &'observation RequestedProcessingRecoveryQueueExecutionBatchStopped<'evidence>,
    supervision: &[RequestedProcessingRecoveryQueueSupervision],
) -> Result<
    MorphospaceRequestedProcessingExecutionObservation<'observation, 'evidence>,
    MorphospaceRequestedProcessingExecutionObservationError,
> {
    let committed_extents =
        collect_committed_extents(limits.cycle_limit, stopped.committed().iter())?;
    validate_input(
        limits,
        ProjectionInput {
            budget: stopped.budget().cycles(),
            committed_extents: &committed_extents,
            supervision,
            stop_cycle: Some(stopped.stopped_cycle()),
        },
    )?;
    let cycles = project_cycles(supervision)?;
    Ok(MorphospaceRequestedProcessingExecutionObservation {
        identity,
        budget_cycles: stopped.budget().cycles(),
        cycles,
        stop: MorphospaceRequestedProcessingExecutionStop::ExecutionRefused {
            cycle: stopped.stopped_cycle(),
            cause: stopped.cause(),
            supervision: MorphospaceRequestedProcessingStoppedCycleSupervision::NotReportedForUncommittedCycle,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        RequestedProcessingExecutionReportLimits, RequestedProcessingExecutionStage,
        RequestedProcessingExecutionTermination, RequestedProcessingRecoveryQueueExecutionReport,
        RequestedProcessingSupervisionLimits,
    };

    fn supervision(total: usize, completed: usize) -> RequestedProcessingRecoveryQueueSupervision {
        let termination = if total == completed {
            RequestedProcessingExecutionTermination::Complete
        } else {
            RequestedProcessingExecutionTermination::Deadline {
                stage: RequestedProcessingExecutionStage::Queue,
                completed_recovery_attempts: 1,
                queue_len: Some(2),
            }
        };
        let report = RequestedProcessingRecoveryQueueExecutionReport::new(
            RequestedProcessingExecutionReportLimits::new(total, 2, 3).unwrap(),
            total,
            completed,
            termination,
        )
        .unwrap();
        RequestedProcessingRecoveryQueueSupervision::new(
            RequestedProcessingSupervisionLimits::new(1).unwrap(),
            &[report],
        )
        .unwrap()
    }

    #[test]
    fn exact_cycle_association_extents_and_explicit_absence_are_projected() {
        let values = [supervision(3, 1), supervision(2, 2)];
        let input = ProjectionInput {
            budget: 2,
            committed_extents: &[3, 2],
            supervision: &values,
            stop_cycle: None,
        };
        assert_eq!(
            validate_input(
                MorphospaceRequestedProcessingExecutionObservationLimits::new(2, 2).unwrap(),
                input
            ),
            Ok(2)
        );
        let cycles = project_cycles(&values).unwrap();
        assert_eq!(
            (
                cycles[0].cycle(),
                cycles[0].total_execution_count(),
                cycles[0].last_completed_execution_count()
            ),
            (0, 3, 1)
        );
        assert_eq!(
            cycles[0].queue_length(),
            MorphospaceRequestedProcessingQueueLengthObservation::Reported(2)
        );
        assert_eq!(
            cycles[1].queue_length(),
            MorphospaceRequestedProcessingQueueLengthObservation::NotReportedByP62
        );
        assert_eq!(
            cycles[1].loss_facts(),
            RequestedProcessingSupervisionLossFacts::NotReportedByP62
        );
    }

    #[test]
    fn contradictory_association_extent_and_stop_are_transactionally_refused() {
        let values = [supervision(3, 3)];
        let limits = MorphospaceRequestedProcessingExecutionObservationLimits::new(3, 3).unwrap();
        assert!(matches!(
            validate_input(
                limits,
                ProjectionInput {
                    budget: 2,
                    committed_extents: &[3],
                    supervision: &[],
                    stop_cycle: Some(1)
                }
            ),
            Err(MorphospaceRequestedProcessingExecutionObservationError::CycleAssociation { .. })
        ));
        assert!(matches!(validate_input(limits, ProjectionInput { budget: 2, committed_extents: &[2], supervision: &values, stop_cycle: Some(1) }), Err(MorphospaceRequestedProcessingExecutionObservationError::CommittedExtentContradiction { .. })));
        assert!(matches!(validate_input(limits, ProjectionInput { budget: 2, committed_extents: &[3], supervision: &values, stop_cycle: Some(0) }), Err(MorphospaceRequestedProcessingExecutionObservationError::StoppedCycleContradiction { .. })));
    }

    #[test]
    fn zero_and_exceeded_bounds_fail_closed() {
        assert_eq!(
            MorphospaceRequestedProcessingExecutionObservationLimits::new(0, 1),
            Err(MorphospaceRequestedProcessingExecutionObservationConfigError::ZeroCycleLimit)
        );
        assert_eq!(
            MorphospaceRequestedProcessingExecutionObservationLimits::new(1, 0),
            Err(MorphospaceRequestedProcessingExecutionObservationConfigError::ZeroReportLimit)
        );
        let values = [supervision(1, 1), supervision(1, 1)];
        assert!(matches!(
            validate_input(
                MorphospaceRequestedProcessingExecutionObservationLimits::new(2, 1).unwrap(),
                ProjectionInput {
                    budget: 2,
                    committed_extents: &[1, 1],
                    supervision: &values,
                    stop_cycle: None
                }
            ),
            Err(
                MorphospaceRequestedProcessingExecutionObservationError::ReportLimit {
                    actual: 2,
                    ..
                }
            )
        ));
    }

    #[test]
    fn source_and_execution_identity_remain_exact_and_uninterpreted() {
        let identity = MorphospaceRequestedProcessingExecutionIdentity::new(u128::MAX, 0);
        assert_eq!((identity.source(), identity.execution()), (u128::MAX, 0));
    }
}
