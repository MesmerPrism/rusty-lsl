// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional composition of P62 execution, reporting, and P61 observability.

use crate::*;
use std::sync::atomic::AtomicBool;

/// Successful bounded execution with its stable report and committed P61 health.
#[derive(Debug)]
pub struct CompleteRequestedProcessingRecoveryQueueExecution<'a> {
    /// Existing execution-owner outcome.
    pub execution: RequestedProcessingRecoveryQueueExecutionOutcome<'a>,
    /// Stable bounded report derived from that outcome.
    pub report: RequestedProcessingRecoveryQueueExecutionReport,
    /// Committed immutable P61 health projection.
    pub health: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth,
}

/// Owner-preserving composed refusal.
#[derive(Debug)]
pub enum CompleteRequestedProcessingRecoveryQueueExecutionError<'a> {
    /// Execution stopped after a valid report and representable health were committed.
    Execution {
        /// Existing owner-preserving execution refusal.
        execution: RequestedProcessingRecoveryQueueExecutionError<'a>,
        /// Stable exact report for the refusal.
        report: RequestedProcessingRecoveryQueueExecutionReport,
        /// Committed exact P61 health.
        health: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth,
    },
    /// Stable report validation refused the exact execution evidence.
    Report {
        /// Unchanged execution evidence.
        execution: Result<
            RequestedProcessingRecoveryQueueExecutionOutcome<'a>,
            RequestedProcessingRecoveryQueueExecutionError<'a>,
        >,
        /// Exact report-contract refusal.
        error: RequestedProcessingExecutionReportError,
    },
    /// P61 refused a staged observation; caller health remains unchanged.
    Observability {
        /// Unchanged execution evidence.
        execution: Result<
            RequestedProcessingRecoveryQueueExecutionOutcome<'a>,
            RequestedProcessingRecoveryQueueExecutionError<'a>,
        >,
        /// Already validated stable report.
        report: RequestedProcessingRecoveryQueueExecutionReport,
        /// Existing transactional P61 refusal.
        error: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError,
    },
    /// Queue poisoning has no stable-report classification and changes no P61 health.
    UnreportableQueuePoisoned {
        /// Existing queue-owner refusal retaining the sample.
        execution: RequestedProcessingRecoveryQueueExecutionError<'a>,
    },
    /// Empty completed evidence has no valid nonzero stable report.
    EmptyExecution {
        /// Existing execution refusal retaining the lifecycle borrow.
        execution: RequestedProcessingRecoveryQueueExecutionError<'a>,
    },
    /// Recovery setup/allocation refusal has no stable terminal classification.
    UnreportableRecovery {
        /// Existing recovery-owner refusal and retained execution evidence.
        execution: RequestedProcessingRecoveryQueueExecutionError<'a>,
    },
}

fn successful_attempt(states: &[FiniteSampleRecoveryState]) -> usize {
    states
        .iter()
        .rev()
        .find_map(|state| match state {
            FiniteSampleRecoveryState::Recovered { attempt } => Some(*attempt),
            _ => None,
        })
        .expect("recovered execution retains its successful attempt")
}

fn completed_attempts(states: &[FiniteSampleRecoveryState]) -> usize {
    states
        .iter()
        .rev()
        .find_map(|state| match state {
            FiniteSampleRecoveryState::TerminalFailure { attempt, .. } => Some(*attempt),
            FiniteSampleRecoveryState::Exhausted { attempts }
            | FiniteSampleRecoveryState::Cancelled {
                completed_attempts: attempts,
            }
            | FiniteSampleRecoveryState::Deadline {
                completed_attempts: attempts,
            } => Some(*attempts),
            _ => None,
        })
        .unwrap_or(0)
}

fn loss_fact(
    record: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord<'_>,
) -> RequestedPostProcessingSequenceLossFact {
    match record.classification() {
        CompleteTypedUdpDiscoveryFloat32SequenceClassification::First => {
            RequestedPostProcessingSequenceLossFact::First
        }
        CompleteTypedUdpDiscoveryFloat32SequenceClassification::Contiguous => {
            RequestedPostProcessingSequenceLossFact::Contiguous
        }
        CompleteTypedUdpDiscoveryFloat32SequenceClassification::Gap {
            missing_sequence_count,
        } => RequestedPostProcessingSequenceLossFact::Gap {
            missing_sequence_count,
        },
        CompleteTypedUdpDiscoveryFloat32SequenceClassification::Duplicate => {
            RequestedPostProcessingSequenceLossFact::Duplicate
        }
        CompleteTypedUdpDiscoveryFloat32SequenceClassification::OutOfOrder {
            behind_high_water_by,
        } => RequestedPostProcessingSequenceLossFact::OutOfOrder {
            behind_high_water_by,
        },
    }
}

fn observe_prefix(
    next: &mut CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle,
    completed: &CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    queued: &[RequestedProcessingRecoveryQueueRecordOutcome],
) -> Result<(), CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError> {
    for outcome in queued {
        let record = completed
            .record(outcome.index)
            .expect("execution index came from completed evidence");
        next.observe_completed(
            completed,
            RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: successful_attempt(&outcome.states),
                },
                Some(loss_fact(record)),
            ),
            RequestedPostProcessingQueueObservation::Accepted {
                queue_len_after: outcome.queue_len_after,
            },
        )?;
    }
    Ok(())
}

/// Runs the existing execution owner, transactionally constructs its stable report, and then
/// commits only exact representable P61 observations. `observe_queue_len` remains the caller's
/// queue-observation authority and is invoked immediately after each admission or refusal.
#[allow(clippy::too_many_arguments)]
pub fn run_complete_requested_processing_recovery_queue_execution<'a, A, Q>(
    completed: &'a CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    recovery_cancelled: &AtomicBool,
    queue_cancelled: &AtomicBool,
    report_limits: RequestedProcessingExecutionReportLimits,
    observability: &mut CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle,
    attempt: A,
    observe_queue_len: Q,
) -> Result<
    CompleteRequestedProcessingRecoveryQueueExecution<'a>,
    CompleteRequestedProcessingRecoveryQueueExecutionError<'a>,
>
where
    A: FnMut(usize, usize) -> Result<(), RecoveryAttemptFailure>,
    Q: FnMut() -> usize,
{
    let execution = run_requested_processing_recovery_queue_execution(
        completed,
        recovery_activation,
        recovery_policy,
        queue,
        queue_wait,
        recovery_cancelled,
        queue_cancelled,
        attempt,
        observe_queue_len,
    );
    let total = completed.record_count();
    let (completed_count, termination) = match &execution {
        Ok(value) => (
            value.queued.len(),
            RequestedProcessingExecutionTermination::Complete,
        ),
        Err(RequestedProcessingRecoveryQueueExecutionError::Empty { .. }) => {
            return Err(
                CompleteRequestedProcessingRecoveryQueueExecutionError::EmptyExecution {
                    execution: execution.err().unwrap(),
                },
            );
        }
        Err(RequestedProcessingRecoveryQueueExecutionError::Recovery { .. }) => {
            return Err(
                CompleteRequestedProcessingRecoveryQueueExecutionError::UnreportableRecovery {
                    execution: execution.err().unwrap(),
                },
            )
        }
        Err(RequestedProcessingRecoveryQueueExecutionError::NotRecovered {
            termination,
            queued,
            ..
        }) => {
            let mapped = match termination {
                RequestedProcessingRecoveryQueueTermination::Terminal { states, .. } => {
                    RequestedProcessingExecutionTermination::RecoveryTerminal {
                        completed_attempts: completed_attempts(states),
                    }
                }
                RequestedProcessingRecoveryQueueTermination::Exhausted { states, .. } => {
                    RequestedProcessingExecutionTermination::RecoveryExhausted {
                        attempts: completed_attempts(states),
                    }
                }
                RequestedProcessingRecoveryQueueTermination::Cancelled { states } => {
                    RequestedProcessingExecutionTermination::Cancelled {
                        stage: RequestedProcessingExecutionStage::Recovery,
                        completed_recovery_attempts: completed_attempts(states),
                        queue_len: None,
                    }
                }
                RequestedProcessingRecoveryQueueTermination::Deadline { states } => {
                    RequestedProcessingExecutionTermination::Deadline {
                        stage: RequestedProcessingExecutionStage::Recovery,
                        completed_recovery_attempts: completed_attempts(states),
                        queue_len: None,
                    }
                }
            };
            (queued.len(), mapped)
        }
        Err(RequestedProcessingRecoveryQueueExecutionError::Queue {
            error,
            states,
            queue_len,
            queued,
            ..
        }) => {
            let attempt = successful_attempt(states);
            let mapped = match error {
                BoundedSampleQueuePushError::Full(_) => RequestedProcessingExecutionTermination::QueueBackpressure { successful_recovery_attempt: attempt, queue_len: *queue_len },
                BoundedSampleQueuePushError::Closed(_) => RequestedProcessingExecutionTermination::QueueClosed { successful_recovery_attempt: attempt, queue_len: *queue_len },
                BoundedSampleQueuePushError::Cancelled(_) => RequestedProcessingExecutionTermination::Cancelled { stage: RequestedProcessingExecutionStage::Queue, completed_recovery_attempts: attempt, queue_len: Some(*queue_len) },
                BoundedSampleQueuePushError::Deadline(_) => RequestedProcessingExecutionTermination::Deadline { stage: RequestedProcessingExecutionStage::Queue, completed_recovery_attempts: attempt, queue_len: Some(*queue_len) },
                BoundedSampleQueuePushError::Poisoned(_) => return Err(CompleteRequestedProcessingRecoveryQueueExecutionError::UnreportableQueuePoisoned { execution: execution.err().unwrap() }),
            };
            (queued.len(), mapped)
        }
    };
    let report = match RequestedProcessingRecoveryQueueExecutionReport::new(
        report_limits,
        total,
        completed_count,
        termination,
    ) {
        Ok(report) => report,
        Err(error) => {
            return Err(
                CompleteRequestedProcessingRecoveryQueueExecutionError::Report { execution, error },
            )
        }
    };

    let mut next = observability.clone();
    let observation_result = match &execution {
        Ok(value) => observe_prefix(&mut next, completed, &value.queued),
        Err(RequestedProcessingRecoveryQueueExecutionError::Recovery { queued, .. }) => {
            observe_prefix(&mut next, completed, queued)
        }
        Err(RequestedProcessingRecoveryQueueExecutionError::NotRecovered {
            termination,
            queued,
            ..
        }) => observe_prefix(&mut next, completed, queued).and_then(|_| match termination {
            RequestedProcessingRecoveryQueueTermination::Exhausted { states, .. } => next
                .observe_terminal_recovery(RequestedPostProcessingRecoveryObservation::new(
                    RequestedPostProcessingRecoveryDisposition::Exhausted {
                        attempts: completed_attempts(states),
                    },
                    None,
                ))
                .map(|_| ()),
            RequestedProcessingRecoveryQueueTermination::Cancelled { states } => next
                .observe_terminal_recovery(RequestedPostProcessingRecoveryObservation::new(
                    RequestedPostProcessingRecoveryDisposition::Cancelled {
                        completed_attempts: completed_attempts(states),
                    },
                    None,
                ))
                .map(|_| ()),
            RequestedProcessingRecoveryQueueTermination::Terminal { .. }
            | RequestedProcessingRecoveryQueueTermination::Deadline { .. } => Ok(()),
        }),
        Err(RequestedProcessingRecoveryQueueExecutionError::Queue { queued, .. }) => {
            observe_prefix(&mut next, completed, queued)
        }
        Err(RequestedProcessingRecoveryQueueExecutionError::Empty { .. }) => unreachable!(),
    };
    if let Err(error) = observation_result {
        return Err(
            CompleteRequestedProcessingRecoveryQueueExecutionError::Observability {
                execution,
                report,
                error,
            },
        );
    }
    *observability = next;
    let health = observability.health();
    match execution {
        Ok(execution) => Ok(CompleteRequestedProcessingRecoveryQueueExecution {
            execution,
            report,
            health,
        }),
        Err(execution) => Err(
            CompleteRequestedProcessingRecoveryQueueExecutionError::Execution {
                execution,
                report,
                health,
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_attempt_evidence_drives_report_facts_without_inference() {
        let retry_success = [
            FiniteSampleRecoveryState::Attempting { attempt: 1 },
            FiniteSampleRecoveryState::RetryableFailure {
                attempt: 1,
                code: 7,
            },
            FiniteSampleRecoveryState::Attempting { attempt: 2 },
            FiniteSampleRecoveryState::Recovered { attempt: 2 },
        ];
        assert_eq!(successful_attempt(&retry_success), 2);
        assert_eq!(completed_attempts(&retry_success), 0);

        for (state, expected) in [
            (
                FiniteSampleRecoveryState::Cancelled {
                    completed_attempts: 1,
                },
                1,
            ),
            (
                FiniteSampleRecoveryState::Deadline {
                    completed_attempts: 2,
                },
                2,
            ),
            (FiniteSampleRecoveryState::Exhausted { attempts: 3 }, 3),
            (
                FiniteSampleRecoveryState::TerminalFailure {
                    attempt: 2,
                    code: 11,
                },
                2,
            ),
        ] {
            assert_eq!(completed_attempts(&[state]), expected);
        }
    }

    #[test]
    fn staged_p61_refusal_does_not_mutate_caller_health() {
        let mut original =
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle::new(
                RequestedPostProcessingRecoveryConfig::new(1, 2, 1).unwrap(),
                RequestedPostProcessingQueueHealthConfig::new(1, 1).unwrap(),
            );
        original
            .observe_terminal_recovery(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Cancelled {
                    completed_attempts: 0,
                },
                None,
            ))
            .unwrap();
        let before = original.health();
        let mut staged = original.clone();
        assert!(staged
            .observe_terminal_recovery(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Exhausted { attempts: 2 },
                None,
            ))
            .is_err());
        assert_eq!(original.health(), before);
    }
}
