// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded CPU/data-only execution of completed requested-processing evidence.

use crate::{
    run_finite_sample_recovery, BoundedSampleQueue, BoundedSampleQueuePushError,
    BoundedSampleQueueWait, CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord,
    CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    FiniteSampleRecoveryActivation, FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryState, RecoveryAttemptFailure,
    TimestampedSample,
};
use std::sync::atomic::AtomicBool;

/// Exact completion evidence for one requested-processing record admitted to the queue.
#[derive(Debug, Eq, PartialEq)]
pub struct RequestedProcessingRecoveryQueueRecordOutcome {
    /// Zero-based index in the completed requested-processing lifecycle.
    pub index: usize,
    /// Exact requested-processing sequence associated with the queued sample.
    pub sequence: u64,
    /// Ordered states returned by the finite-recovery owner.
    pub states: Vec<FiniteSampleRecoveryState>,
}

/// Successful execution, retaining a borrow of the exact completed P60 evidence.
#[derive(Debug)]
pub struct RequestedProcessingRecoveryQueueExecutionOutcome<'a> {
    /// Exact completed requested-processing lifecycle consumed as immutable evidence.
    pub completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    /// Ordered queue-admission completions.
    pub queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
}

/// Recovery termination before one processed sample reached queue admission.
#[derive(Debug)]
pub enum RequestedProcessingRecoveryQueueTermination {
    /// The caller classified an attempt as terminal.
    Terminal {
        /// Unchanged caller-classified failure.
        failure: RecoveryAttemptFailure,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Every caller-permitted attempt failed retryably.
    Exhausted {
        /// Unchanged last caller-classified failure.
        failure: RecoveryAttemptFailure,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Recovery cancellation was observed.
    Cancelled {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The finite recovery deadline elapsed.
    Deadline {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Owner-preserving bounded execution refusal.
#[derive(Debug)]
pub enum RequestedProcessingRecoveryQueueExecutionError<'a> {
    /// The completed lifecycle unexpectedly exposed no processed records.
    Empty {
        /// Unchanged completed evidence.
        completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    },
    /// Finite recovery could not reserve its bounded state trace.
    Recovery {
        /// Current zero-based requested-processing record index.
        index: usize,
        /// Existing recovery-owner failure.
        error: FiniteSampleRecoveryError,
        /// Exact already-queued prefix.
        queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
        /// Unchanged completed evidence, including current record and suffix.
        completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    },
    /// Recovery terminated before queue admission.
    NotRecovered {
        /// Current zero-based requested-processing record index.
        index: usize,
        /// Exact recovery-owner termination.
        termination: RequestedProcessingRecoveryQueueTermination,
        /// Exact already-queued prefix.
        queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
        /// Unchanged completed evidence, including current record and suffix.
        completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    },
    /// The existing queue owner refused the recovered processed sample.
    Queue {
        /// Current zero-based requested-processing record index.
        index: usize,
        /// Existing queue error retaining the unchanged rejected sample.
        error: BoundedSampleQueuePushError,
        /// Ordered recovery states for the rejected current record.
        states: Vec<FiniteSampleRecoveryState>,
        /// Exact already-queued prefix.
        queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
        /// Unchanged completed requested-processing evidence.
        completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    },
}

#[derive(Debug)]
enum ExecutionCoreError {
    Empty,
    Recovery {
        index: usize,
        error: FiniteSampleRecoveryError,
        queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
    },
    NotRecovered {
        index: usize,
        termination: RequestedProcessingRecoveryQueueTermination,
        queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
    },
    Queue {
        index: usize,
        error: BoundedSampleQueuePushError,
        states: Vec<FiniteSampleRecoveryState>,
        queued: Vec<RequestedProcessingRecoveryQueueRecordOutcome>,
    },
}

fn execute_records<'a, R, A>(
    records: R,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    recovery_cancelled: &AtomicBool,
    queue_cancelled: &AtomicBool,
    mut attempt: A,
) -> Result<Vec<RequestedProcessingRecoveryQueueRecordOutcome>, ExecutionCoreError>
where
    R: Iterator<Item = (usize, u64, &'a TimestampedSample<f32>)>,
    A: FnMut(usize, usize) -> Result<(), RecoveryAttemptFailure>,
{
    let mut records = records.peekable();
    if records.peek().is_none() {
        return Err(ExecutionCoreError::Empty);
    }
    let mut queued = Vec::new();
    for (index, sequence, sample) in records {
        let recovery = run_finite_sample_recovery(
            recovery_activation,
            recovery_policy,
            recovery_cancelled,
            |attempt_number| {
                attempt(index, attempt_number)?;
                Ok(sample.clone())
            },
        );
        let (sample, states) = match recovery {
            Err(error) => {
                return Err(ExecutionCoreError::Recovery {
                    index,
                    error,
                    queued,
                });
            }
            Ok(FiniteSampleRecoveryOutcome::Recovered { sample, states }) => (sample, states),
            Ok(FiniteSampleRecoveryOutcome::Terminal { failure, states }) => {
                return Err(ExecutionCoreError::NotRecovered {
                    index,
                    termination: RequestedProcessingRecoveryQueueTermination::Terminal {
                        failure,
                        states,
                    },
                    queued,
                });
            }
            Ok(FiniteSampleRecoveryOutcome::Exhausted { failure, states }) => {
                return Err(ExecutionCoreError::NotRecovered {
                    index,
                    termination: RequestedProcessingRecoveryQueueTermination::Exhausted {
                        failure,
                        states,
                    },
                    queued,
                });
            }
            Ok(FiniteSampleRecoveryOutcome::Cancelled { states }) => {
                return Err(ExecutionCoreError::NotRecovered {
                    index,
                    termination: RequestedProcessingRecoveryQueueTermination::Cancelled { states },
                    queued,
                });
            }
            Ok(FiniteSampleRecoveryOutcome::Deadline { states }) => {
                return Err(ExecutionCoreError::NotRecovered {
                    index,
                    termination: RequestedProcessingRecoveryQueueTermination::Deadline { states },
                    queued,
                });
            }
        };
        if let Err(error) = queue.push(sample, queue_wait, queue_cancelled) {
            return Err(ExecutionCoreError::Queue {
                index,
                error,
                states,
                queued,
            });
        }
        queued.push(RequestedProcessingRecoveryQueueRecordOutcome {
            index,
            sequence,
            states,
        });
    }
    Ok(queued)
}

/// Executes exact completed requested-processing records through existing finite-recovery and
/// bounded-queue owners.
///
/// `attempt` owns failure classification only. Returning `Ok(())` permits the adapter to clone
/// the exact processed record once for transfer to the queue owner. The completed lifecycle is
/// always borrowed and returned as evidence; this adapter owns no storage, scheduling, clock,
/// transport, retry policy, or queue policy.
#[allow(clippy::too_many_arguments)]
pub fn run_requested_processing_recovery_queue_execution<'a, A>(
    completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    recovery_cancelled: &AtomicBool,
    queue_cancelled: &AtomicBool,
    attempt: A,
) -> Result<
    RequestedProcessingRecoveryQueueExecutionOutcome<'a>,
    RequestedProcessingRecoveryQueueExecutionError<'a>,
>
where
    A: FnMut(usize, usize) -> Result<(), RecoveryAttemptFailure>,
{
    let records = (0..completed.record_count()).map(|index| {
        let record: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord<'_> = completed
            .record(index)
            .expect("record_count bounds the immutable completed lifecycle");
        (index, record.sequence(), record.sample())
    });
    execute_records(
        records,
        recovery_activation,
        recovery_policy,
        queue,
        queue_wait,
        recovery_cancelled,
        queue_cancelled,
        attempt,
    )
    .map(|queued| RequestedProcessingRecoveryQueueExecutionOutcome { completed, queued })
    .map_err(|error| match error {
        ExecutionCoreError::Empty => {
            RequestedProcessingRecoveryQueueExecutionError::Empty { completed }
        }
        ExecutionCoreError::Recovery {
            index,
            error,
            queued,
        } => RequestedProcessingRecoveryQueueExecutionError::Recovery {
            index,
            error,
            queued,
            completed,
        },
        ExecutionCoreError::NotRecovered {
            index,
            termination,
            queued,
        } => RequestedProcessingRecoveryQueueExecutionError::NotRecovered {
            index,
            termination,
            queued,
            completed,
        },
        ExecutionCoreError::Queue {
            index,
            error,
            states,
            queued,
        } => RequestedProcessingRecoveryQueueExecutionError::Queue {
            index,
            error,
            states,
            queued,
            completed,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        BoundedSampleQueueActivation, RawSourceTimestamp, RecoveryFailureClass, RuntimeModule,
        Sample, SampleLimits, StreamHandshakeActivation, TimestampedFloat32SampleActivation,
    };
    use std::time::Duration;

    fn activations() -> (FiniteSampleRecoveryActivation, BoundedSampleQueueActivation) {
        let handshake =
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap();
        let sample = TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            handshake,
        )
        .unwrap();
        let queue = BoundedSampleQueueActivation::new(
            test_capability(RuntimeModule::BoundedSampleQueue),
            sample,
        )
        .unwrap();
        (
            FiniteSampleRecoveryActivation::new(
                test_capability(RuntimeModule::FiniteSampleRecovery),
                queue,
            )
            .unwrap(),
            queue,
        )
    }

    fn policy(attempts: usize) -> FiniteSampleRecoveryPolicy {
        FiniteSampleRecoveryPolicy::new(
            attempts,
            attempts * 2 + 1,
            Duration::ZERO,
            Duration::from_millis(1),
            Duration::from_secs(1),
        )
        .unwrap()
    }

    fn wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(1), Duration::from_millis(5)).unwrap()
    }

    fn sample(value: u32, timestamp: u64) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(value)],
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(timestamp)).unwrap(),
            None,
        )
    }

    #[test]
    fn ordered_success_and_retry_preserve_exact_bits_and_states() {
        let (recovery, queue_activation) = activations();
        let queue = BoundedSampleQueue::new(queue_activation, 2).unwrap();
        let records = vec![
            (0, u64::MIN, sample(0x7fc0_5678, (-0.0f64).to_bits())),
            (1, u64::MAX, sample(0x8000_0000, 1.25f64.to_bits())),
        ];
        let result = execute_records(
            records
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(2),
            &queue,
            wait(),
            &AtomicBool::new(false),
            &AtomicBool::new(false),
            |index, attempt| {
                if index == 1 && attempt == 1 {
                    Err(RecoveryAttemptFailure::new(
                        RecoveryFailureClass::Retryable,
                        41,
                    ))
                } else {
                    Ok(())
                }
            },
        )
        .unwrap();
        assert_eq!(
            result
                .iter()
                .map(|record| record.sequence)
                .collect::<Vec<_>>(),
            [u64::MIN, u64::MAX]
        );
        assert_eq!(result[1].states.len(), 4);
        let first = queue.try_pop().unwrap();
        let second = queue.try_pop().unwrap();
        assert_eq!(first.sample().values()[0].to_bits(), 0x7fc0_5678);
        assert_eq!(
            first.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(second.sample().values()[0].to_bits(), 0x8000_0000);
    }

    #[test]
    fn terminal_exhausted_and_cancelled_bypass_queue() {
        let (recovery, queue_activation) = activations();
        let queue = BoundedSampleQueue::new(queue_activation, 1).unwrap();
        let terminal = execute_records(
            [(0, 7, sample(1, 1.0f64.to_bits()))]
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(2),
            &queue,
            wait(),
            &AtomicBool::new(false),
            &AtomicBool::new(false),
            |_, _| {
                Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Terminal,
                    17,
                ))
            },
        )
        .unwrap_err();
        assert!(
            matches!(terminal, ExecutionCoreError::NotRecovered { termination: RequestedProcessingRecoveryQueueTermination::Terminal { failure, .. }, .. } if failure.code() == 17)
        );
        let exhausted = execute_records(
            [(0, 8, sample(2, 2.0f64.to_bits()))]
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(2),
            &queue,
            wait(),
            &AtomicBool::new(false),
            &AtomicBool::new(false),
            |_, _| {
                Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Retryable,
                    19,
                ))
            },
        )
        .unwrap_err();
        assert!(
            matches!(exhausted, ExecutionCoreError::NotRecovered { termination: RequestedProcessingRecoveryQueueTermination::Exhausted { failure, .. }, .. } if failure.code() == 19)
        );
        let cancelled = AtomicBool::new(true);
        let cancellation = execute_records(
            [(0, 9, sample(3, 3.0f64.to_bits()))]
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(1),
            &queue,
            wait(),
            &cancelled,
            &AtomicBool::new(false),
            |_, _| panic!("cancelled recovery invoked acquisition"),
        )
        .unwrap_err();
        assert!(matches!(
            cancellation,
            ExecutionCoreError::NotRecovered {
                termination: RequestedProcessingRecoveryQueueTermination::Cancelled { .. },
                ..
            }
        ));
        assert!(queue.try_pop().is_err());
    }

    #[test]
    fn full_closed_and_cancelled_queue_errors_retain_unchanged_samples() {
        let (recovery, queue_activation) = activations();
        let full = BoundedSampleQueue::new(queue_activation, 1).unwrap();
        full.try_push(sample(10, 10.0f64.to_bits())).unwrap();
        let error = execute_records(
            [(0, 1, sample(0x7fc0_1234, (-0.0f64).to_bits()))]
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(1),
            &full,
            wait(),
            &AtomicBool::new(false),
            &AtomicBool::new(false),
            |_, _| Ok(()),
        )
        .unwrap_err();
        assert!(
            matches!(error, ExecutionCoreError::Queue { error: BoundedSampleQueuePushError::Deadline(sample), .. } if sample.sample().values()[0].to_bits() == 0x7fc0_1234 && sample.raw_source_timestamp().value().to_bits() == (-0.0f64).to_bits())
        );
        let closed = BoundedSampleQueue::new(queue_activation, 1).unwrap();
        closed.close().unwrap();
        let error = execute_records(
            [(0, 2, sample(22, 22.0f64.to_bits()))]
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(1),
            &closed,
            wait(),
            &AtomicBool::new(false),
            &AtomicBool::new(false),
            |_, _| Ok(()),
        )
        .unwrap_err();
        assert!(
            matches!(error, ExecutionCoreError::Queue { error: BoundedSampleQueuePushError::Closed(sample), .. } if sample.sample().values()[0].to_bits() == 22)
        );
        let cancelled_queue = BoundedSampleQueue::new(queue_activation, 1).unwrap();
        let cancelled = AtomicBool::new(true);
        let error = execute_records(
            [(0, 3, sample(33, 33.0f64.to_bits()))]
                .iter()
                .map(|(index, sequence, sample)| (*index, *sequence, sample)),
            recovery,
            policy(1),
            &cancelled_queue,
            wait(),
            &AtomicBool::new(false),
            &cancelled,
            |_, _| Ok(()),
        )
        .unwrap_err();
        assert!(
            matches!(error, ExecutionCoreError::Queue { error: BoundedSampleQueuePushError::Cancelled(sample), .. } if sample.sample().values()[0].to_bits() == 33)
        );
    }
}
