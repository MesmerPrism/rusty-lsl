// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Finite caller-budgeted collection of exact P62 execution outcomes.

use crate::{
    RequestedProcessingRecoveryQueueExecutionError,
    RequestedProcessingRecoveryQueueExecutionOutcome,
};

/// Nonzero number of P62 execution cycles explicitly permitted by the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedProcessingRecoveryQueueExecutionBatchBudget {
    cycles: usize,
}

impl RequestedProcessingRecoveryQueueExecutionBatchBudget {
    /// Constructs an exact finite cycle budget.
    pub const fn new(
        cycles: usize,
    ) -> Result<Self, RequestedProcessingRecoveryQueueExecutionBatchConfigError> {
        if cycles == 0 {
            return Err(RequestedProcessingRecoveryQueueExecutionBatchConfigError::ZeroCycles);
        }
        Ok(Self { cycles })
    }

    /// Returns the exact caller-permitted cycle count.
    pub const fn cycles(self) -> usize {
        self.cycles
    }
}

/// Invalid caller-supplied batch configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedProcessingRecoveryQueueExecutionBatchConfigError {
    /// A finite execution batch must permit at least one cycle.
    ZeroCycles,
}

/// Successful exhaustion of the caller's exact cycle budget.
#[derive(Debug)]
pub struct RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a> {
    budget: RequestedProcessingRecoveryQueueExecutionBatchBudget,
    committed: Vec<RequestedProcessingRecoveryQueueExecutionOutcome<'a>>,
}

impl<'a> RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a> {
    /// Returns the exact caller budget consumed by this completed batch.
    pub const fn budget(&self) -> RequestedProcessingRecoveryQueueExecutionBatchBudget {
        self.budget
    }

    /// Returns the ordered, fully successful P62 cycle outcomes.
    pub fn committed(&self) -> &[RequestedProcessingRecoveryQueueExecutionOutcome<'a>] {
        &self.committed
    }

    /// Consumes the batch and returns the ordered P62 outcomes without projection.
    pub fn into_committed(self) -> Vec<RequestedProcessingRecoveryQueueExecutionOutcome<'a>> {
        self.committed
    }
}

/// Exact first P62 refusal together with every fully committed earlier cycle.
#[derive(Debug)]
pub struct RequestedProcessingRecoveryQueueExecutionBatchStopped<'a> {
    budget: RequestedProcessingRecoveryQueueExecutionBatchBudget,
    stopped_cycle: usize,
    committed: Vec<RequestedProcessingRecoveryQueueExecutionOutcome<'a>>,
    cause: RequestedProcessingRecoveryQueueExecutionError<'a>,
}

/// Batch-owned refusal before completion of the exact caller budget.
#[derive(Debug)]
pub enum RequestedProcessingRecoveryQueueExecutionBatchError<'a> {
    /// The batch could not reserve its caller-bounded committed-prefix storage before execution.
    Allocation {
        /// Original exact caller budget.
        budget: RequestedProcessingRecoveryQueueExecutionBatchBudget,
    },
    /// The first exact P62 refusal stopped execution after a committed prefix.
    Stopped(RequestedProcessingRecoveryQueueExecutionBatchStopped<'a>),
}

impl<'a> RequestedProcessingRecoveryQueueExecutionBatchStopped<'a> {
    /// Returns the original exact caller budget.
    pub const fn budget(&self) -> RequestedProcessingRecoveryQueueExecutionBatchBudget {
        self.budget
    }

    /// Returns the zero-based cycle at which P62 refused execution.
    pub const fn stopped_cycle(&self) -> usize {
        self.stopped_cycle
    }

    /// Returns the ordered, fully committed P62 prefix before the refusal.
    pub fn committed(&self) -> &[RequestedProcessingRecoveryQueueExecutionOutcome<'a>] {
        &self.committed
    }

    /// Returns the exact unchanged P62 refusal that stopped the batch.
    pub const fn cause(&self) -> &RequestedProcessingRecoveryQueueExecutionError<'a> {
        &self.cause
    }

    /// Consumes the stop value, preserving both the committed prefix and exact refusal.
    pub fn into_parts(
        self,
    ) -> (
        Vec<RequestedProcessingRecoveryQueueExecutionOutcome<'a>>,
        RequestedProcessingRecoveryQueueExecutionError<'a>,
    ) {
        (self.committed, self.cause)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum FiniteBatchResult<T, E> {
    Allocation,
    Complete(Vec<T>),
    Stopped {
        cycle: usize,
        committed: Vec<T>,
        cause: E,
    },
}

fn run_finite_batch<T, E, F>(cycles: usize, execute: F) -> FiniteBatchResult<T, E>
where
    F: FnMut(usize) -> Result<T, E>,
{
    run_finite_batch_with_reservation(
        cycles,
        |committed, requested| committed.try_reserve_exact(requested).map_err(|_| ()),
        execute,
    )
}

fn run_finite_batch_with_reservation<T, E, R, F>(
    cycles: usize,
    reserve: R,
    mut execute: F,
) -> FiniteBatchResult<T, E>
where
    R: FnOnce(&mut Vec<T>, usize) -> Result<(), ()>,
    F: FnMut(usize) -> Result<T, E>,
{
    let mut committed = Vec::new();
    if reserve(&mut committed, cycles).is_err() {
        return FiniteBatchResult::Allocation;
    }
    for cycle in 0..cycles {
        match execute(cycle) {
            Ok(outcome) => committed.push(outcome),
            Err(cause) => {
                return FiniteBatchResult::Stopped {
                    cycle,
                    committed,
                    cause,
                };
            }
        }
    }
    FiniteBatchResult::Complete(committed)
}

/// Executes at most the exact number of caller-permitted synchronous P62 cycles.
///
/// `execute_cycle` is the sole owner of each cycle's activation, recovery policy,
/// cancellation inputs, queue interaction, and queue-length observation. This adapter calls it
/// once per increasing zero-based cycle and stops at its first exact P62 error. It performs no
/// retry, delay, scheduling, recovery, queue, clock, processing, or activation work.
pub fn run_requested_processing_recovery_queue_execution_batch<'a, F>(
    budget: RequestedProcessingRecoveryQueueExecutionBatchBudget,
    mut execute_cycle: F,
) -> Result<
    RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
    RequestedProcessingRecoveryQueueExecutionBatchError<'a>,
>
where
    F: FnMut(
        usize,
    ) -> Result<
        RequestedProcessingRecoveryQueueExecutionOutcome<'a>,
        RequestedProcessingRecoveryQueueExecutionError<'a>,
    >,
{
    match run_finite_batch(budget.cycles(), |cycle| execute_cycle(cycle)) {
        FiniteBatchResult::Allocation => {
            Err(RequestedProcessingRecoveryQueueExecutionBatchError::Allocation { budget })
        }
        FiniteBatchResult::Complete(committed) => {
            Ok(RequestedProcessingRecoveryQueueExecutionBatchOutcome { budget, committed })
        }
        FiniteBatchResult::Stopped {
            cycle,
            committed,
            cause,
        } => Err(
            RequestedProcessingRecoveryQueueExecutionBatchError::Stopped(
                RequestedProcessingRecoveryQueueExecutionBatchStopped {
                    budget,
                    stopped_cycle: cycle,
                    committed,
                    cause,
                },
            ),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_budget_runs_once_per_ordered_cycle() {
        let mut observed = Vec::new();
        let result = run_finite_batch::<usize, (), _>(3, |cycle| {
            observed.push(cycle);
            Ok(cycle + 10)
        });
        assert_eq!(observed, [0, 1, 2]);
        assert_eq!(result, FiniteBatchResult::Complete(vec![10, 11, 12]));
    }

    #[test]
    fn first_refusal_preserves_prefix_and_exact_cause() {
        let mut calls = 0;
        let cause = String::from("queue deadline");
        let result = run_finite_batch(5, |cycle| {
            calls += 1;
            if cycle == 2 {
                Err(cause.clone())
            } else {
                Ok(cycle)
            }
        });
        assert_eq!(calls, 3);
        assert_eq!(
            result,
            FiniteBatchResult::Stopped {
                cycle: 2,
                committed: vec![0, 1],
                cause,
            }
        );
    }

    #[test]
    fn budget_is_nonzero_and_exact() {
        assert_eq!(
            RequestedProcessingRecoveryQueueExecutionBatchBudget::new(0),
            Err(RequestedProcessingRecoveryQueueExecutionBatchConfigError::ZeroCycles)
        );
        assert_eq!(
            RequestedProcessingRecoveryQueueExecutionBatchBudget::new(4)
                .unwrap()
                .cycles(),
            4
        );
    }

    #[test]
    fn allocation_refusal_precedes_every_cycle() {
        let mut calls = 0;
        let result = run_finite_batch_with_reservation::<usize, (), _, _>(
            2,
            |_, requested| {
                assert_eq!(requested, 2);
                Err(())
            },
            |_| {
                calls += 1;
                Ok(1)
            },
        );
        assert_eq!(result, FiniteBatchResult::Allocation);
        assert_eq!(calls, 0);
    }
}
