// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Finite caller-invoked recovery for one accepted Float32 sample.

use crate::TimestampedSample;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Selected finite recovery feature.
pub const FINITE_SAMPLE_RECOVERY_FEATURE_ID: &str = "finite-sample-recovery";
/// Exact marker required beside explicit caller policy.
pub const FINITE_SAMPLE_RECOVERY_EFFECTIVE_MARKER: &str =
    "rusty.lsl.finite_sample_recovery.effective";

/// Closed activation for finite recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FiniteSampleRecoveryActivation;

impl FiniteSampleRecoveryActivation {
    /// Admits only the selected feature and exact marker.
    pub fn new(feature: &str, marker: &str) -> Result<Self, FiniteSampleRecoveryActivationError> {
        if feature != FINITE_SAMPLE_RECOVERY_FEATURE_ID {
            return Err(FiniteSampleRecoveryActivationError::FeatureMismatch);
        }
        if marker != FINITE_SAMPLE_RECOVERY_EFFECTIVE_MARKER {
            return Err(FiniteSampleRecoveryActivationError::MarkerMismatch);
        }
        Ok(Self)
    }
}

/// Rejected recovery activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteSampleRecoveryActivationError {
    /// Feature identity differed.
    FeatureMismatch,
    /// Effective marker differed.
    MarkerMismatch,
}

/// Explicit finite recovery limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FiniteSampleRecoveryPolicy {
    max_attempts: usize,
    max_states: usize,
    retry_delay: Duration,
    delay_slice: Duration,
    total_deadline: Duration,
}

impl FiniteSampleRecoveryPolicy {
    /// Requires nonzero attempt, state, delay-slice, and deadline bounds.
    pub fn new(
        max_attempts: usize,
        max_states: usize,
        retry_delay: Duration,
        delay_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, FiniteSampleRecoveryPolicyError> {
        if max_attempts == 0 {
            return Err(FiniteSampleRecoveryPolicyError::ZeroAttempts);
        }
        let required_states = max_attempts
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or(FiniteSampleRecoveryPolicyError::StateCountOverflow)?;
        if max_states < required_states {
            return Err(FiniteSampleRecoveryPolicyError::InsufficientStates {
                required: required_states,
                actual: max_states,
            });
        }
        if delay_slice.is_zero() {
            return Err(FiniteSampleRecoveryPolicyError::ZeroDelaySlice);
        }
        if total_deadline.is_zero() {
            return Err(FiniteSampleRecoveryPolicyError::ZeroTotalDeadline);
        }
        Ok(Self {
            max_attempts,
            max_states,
            retry_delay,
            delay_slice,
            total_deadline,
        })
    }
}

/// Invalid recovery policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteSampleRecoveryPolicyError {
    /// Maximum attempts was zero.
    ZeroAttempts,
    /// Required state count overflowed.
    StateCountOverflow,
    /// State capacity could not represent every bounded transition.
    InsufficientStates {
        /// Required worst-case states.
        required: usize,
        /// Supplied state capacity.
        actual: usize,
    },
    /// Cancellation polling delay was zero.
    ZeroDelaySlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
}

/// Caller-owned classification of one attempt failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryFailureClass {
    /// Another bounded attempt is permitted.
    Retryable,
    /// No further attempt is permitted.
    Terminal,
}

/// Opaque caller-labelled attempt failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryAttemptFailure {
    class: RecoveryFailureClass,
    code: u32,
}

impl RecoveryAttemptFailure {
    /// Constructs one caller-classified opaque failure.
    #[must_use]
    pub const fn new(class: RecoveryFailureClass, code: u32) -> Self {
        Self { class, code }
    }

    /// Returns the caller's classification.
    #[must_use]
    pub const fn class(&self) -> RecoveryFailureClass {
        self.class
    }

    /// Returns the unchanged opaque code.
    #[must_use]
    pub const fn code(&self) -> u32 {
        self.code
    }
}

/// One ordered recovery transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteSampleRecoveryState {
    /// A numbered attempt began.
    Attempting {
        /// One-based attempt number.
        attempt: usize,
    },
    /// Caller labelled the attempt retryable.
    RetryableFailure {
        /// One-based attempt number.
        attempt: usize,
        /// Unchanged opaque caller code.
        code: u32,
    },
    /// A sample was recovered.
    Recovered {
        /// One-based successful attempt number.
        attempt: usize,
    },
    /// Caller labelled the attempt terminal.
    TerminalFailure {
        /// One-based attempt number.
        attempt: usize,
        /// Unchanged opaque caller code.
        code: u32,
    },
    /// All permitted attempts were consumed.
    Exhausted {
        /// Completed attempt count.
        attempts: usize,
    },
    /// Explicit cancellation was observed.
    Cancelled {
        /// Attempts completed before cancellation.
        completed_attempts: usize,
    },
    /// Total deadline elapsed.
    Deadline {
        /// Attempts completed before deadline.
        completed_attempts: usize,
    },
}

/// Completed finite recovery with its ordered state evidence.
#[derive(Debug)]
pub enum FiniteSampleRecoveryOutcome {
    /// One sample was recovered.
    Recovered {
        /// Recovered accepted sample.
        sample: TimestampedSample<f32>,
        /// Ordered bounded states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Caller supplied a terminal failure.
    Terminal {
        /// Unchanged caller failure.
        failure: RecoveryAttemptFailure,
        /// Ordered bounded states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Retryable failures consumed every attempt.
    Exhausted {
        /// Last retryable failure.
        failure: RecoveryAttemptFailure,
        /// Ordered bounded states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Caller cancellation terminated work.
    Cancelled {
        /// Ordered bounded states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Total deadline terminated work.
    Deadline {
        /// Ordered bounded states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Recovery setup failure before the first attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteSampleRecoveryError {
    /// State trace reservation failed.
    Allocation {
        /// Requested state capacity.
        requested: usize,
    },
}

fn terminal_state(states: &mut Vec<FiniteSampleRecoveryState>, state: FiniteSampleRecoveryState) {
    // The policy proves worst-case capacity before allocation.
    states.push(state);
}

/// Runs the caller operation synchronously under one explicit finite policy.
pub fn run_finite_sample_recovery<F>(
    _activation: FiniteSampleRecoveryActivation,
    policy: FiniteSampleRecoveryPolicy,
    cancelled: &AtomicBool,
    mut attempt: F,
) -> Result<FiniteSampleRecoveryOutcome, FiniteSampleRecoveryError>
where
    F: FnMut(usize) -> Result<TimestampedSample<f32>, RecoveryAttemptFailure>,
{
    let mut states = Vec::new();
    states
        .try_reserve(policy.max_states)
        .map_err(|_| FiniteSampleRecoveryError::Allocation {
            requested: policy.max_states,
        })?;
    let started = Instant::now();
    let mut completed = 0;
    for number in 1..=policy.max_attempts {
        if cancelled.load(Ordering::Acquire) {
            terminal_state(
                &mut states,
                FiniteSampleRecoveryState::Cancelled {
                    completed_attempts: completed,
                },
            );
            return Ok(FiniteSampleRecoveryOutcome::Cancelled { states });
        }
        if started.elapsed() >= policy.total_deadline {
            terminal_state(
                &mut states,
                FiniteSampleRecoveryState::Deadline {
                    completed_attempts: completed,
                },
            );
            return Ok(FiniteSampleRecoveryOutcome::Deadline { states });
        }
        states.push(FiniteSampleRecoveryState::Attempting { attempt: number });
        match attempt(number) {
            Ok(sample) => {
                states.push(FiniteSampleRecoveryState::Recovered { attempt: number });
                return Ok(FiniteSampleRecoveryOutcome::Recovered { sample, states });
            }
            Err(failure) if failure.class == RecoveryFailureClass::Terminal => {
                states.push(FiniteSampleRecoveryState::TerminalFailure {
                    attempt: number,
                    code: failure.code,
                });
                return Ok(FiniteSampleRecoveryOutcome::Terminal { failure, states });
            }
            Err(failure) => {
                completed = number;
                states.push(FiniteSampleRecoveryState::RetryableFailure {
                    attempt: number,
                    code: failure.code,
                });
                if number == policy.max_attempts {
                    states.push(FiniteSampleRecoveryState::Exhausted {
                        attempts: completed,
                    });
                    return Ok(FiniteSampleRecoveryOutcome::Exhausted { failure, states });
                }
                let delay_started = Instant::now();
                while delay_started.elapsed() < policy.retry_delay {
                    if cancelled.load(Ordering::Acquire) {
                        terminal_state(
                            &mut states,
                            FiniteSampleRecoveryState::Cancelled {
                                completed_attempts: completed,
                            },
                        );
                        return Ok(FiniteSampleRecoveryOutcome::Cancelled { states });
                    }
                    let Some(deadline_remaining) =
                        policy.total_deadline.checked_sub(started.elapsed())
                    else {
                        terminal_state(
                            &mut states,
                            FiniteSampleRecoveryState::Deadline {
                                completed_attempts: completed,
                            },
                        );
                        return Ok(FiniteSampleRecoveryOutcome::Deadline { states });
                    };
                    let delay_remaining =
                        policy.retry_delay.saturating_sub(delay_started.elapsed());
                    thread::sleep(
                        policy
                            .delay_slice
                            .min(deadline_remaining)
                            .min(delay_remaining),
                    );
                }
            }
        }
    }
    unreachable!("nonzero finite attempt loop always returns")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BoundedSampleQueue, BoundedSampleQueueActivation, RawSourceTimestamp, Sample, SampleLimits,
        BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER, BOUNDED_SAMPLE_QUEUE_FEATURE_ID,
    };

    fn activation() -> FiniteSampleRecoveryActivation {
        FiniteSampleRecoveryActivation::new(
            FINITE_SAMPLE_RECOVERY_FEATURE_ID,
            FINITE_SAMPLE_RECOVERY_EFFECTIVE_MARKER,
        )
        .unwrap()
    }

    fn policy(attempts: usize) -> FiniteSampleRecoveryPolicy {
        FiniteSampleRecoveryPolicy::new(
            attempts,
            attempts * 2 + 1,
            Duration::ZERO,
            Duration::from_millis(1),
            Duration::from_millis(100),
        )
        .unwrap()
    }

    fn sample() -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(0x7fc0_5678)],
            )
            .unwrap(),
            RawSourceTimestamp::new(-0.0).unwrap(),
            None,
        )
    }

    #[test]
    fn lslc_002w_retryable_then_recovered_composes_with_queue() {
        let mut calls = 0;
        let outcome =
            run_finite_sample_recovery(activation(), policy(2), &AtomicBool::new(false), |_| {
                calls += 1;
                if calls == 1 {
                    Err(RecoveryAttemptFailure::new(
                        RecoveryFailureClass::Retryable,
                        7,
                    ))
                } else {
                    Ok(sample())
                }
            })
            .unwrap();
        let (recovered, states) = match outcome {
            FiniteSampleRecoveryOutcome::Recovered { sample, states } => (sample, states),
            _ => panic!("expected recovery"),
        };
        assert_eq!(states.len(), 4);
        let queue = BoundedSampleQueue::new(
            BoundedSampleQueueActivation::new(
                BOUNDED_SAMPLE_QUEUE_FEATURE_ID,
                BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER,
            )
            .unwrap(),
            1,
        )
        .unwrap();
        queue.try_push(recovered).unwrap();
        let drained = queue.try_pop().unwrap();
        assert_eq!(
            drained.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(drained.sample().values()[0].to_bits(), 0x7fc0_5678);
    }

    #[test]
    fn lslc_002w_terminal_and_exhausted_retain_failure_and_states() {
        let terminal =
            run_finite_sample_recovery(activation(), policy(3), &AtomicBool::new(false), |_| {
                Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Terminal,
                    11,
                ))
            })
            .unwrap();
        assert!(
            matches!(terminal, FiniteSampleRecoveryOutcome::Terminal { failure, states } if failure.code() == 11 && states.len() == 2)
        );
        let exhausted =
            run_finite_sample_recovery(activation(), policy(2), &AtomicBool::new(false), |_| {
                Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Retryable,
                    13,
                ))
            })
            .unwrap();
        assert!(
            matches!(exhausted, FiniteSampleRecoveryOutcome::Exhausted { failure, states } if failure.code() == 13 && states.last() == Some(&FiniteSampleRecoveryState::Exhausted { attempts: 2 }))
        );
    }

    #[test]
    fn lslc_002w_cancellation_deadline_and_policy_fail_closed() {
        assert_eq!(
            FiniteSampleRecoveryPolicy::new(0, 1, Duration::ZERO, Duration::ZERO, Duration::ZERO),
            Err(FiniteSampleRecoveryPolicyError::ZeroAttempts)
        );
        assert_eq!(
            FiniteSampleRecoveryPolicy::new(
                2,
                4,
                Duration::ZERO,
                Duration::from_millis(1),
                Duration::from_millis(1)
            ),
            Err(FiniteSampleRecoveryPolicyError::InsufficientStates {
                required: 5,
                actual: 4
            })
        );
        let cancelled = AtomicBool::new(true);
        let outcome = run_finite_sample_recovery(activation(), policy(1), &cancelled, |_| {
            panic!("cancelled before attempt")
        })
        .unwrap();
        assert!(matches!(
            outcome,
            FiniteSampleRecoveryOutcome::Cancelled { .. }
        ));
        let deadline_policy = FiniteSampleRecoveryPolicy::new(
            2,
            5,
            Duration::from_millis(20),
            Duration::from_millis(1),
            Duration::from_millis(2),
        )
        .unwrap();
        let outcome = run_finite_sample_recovery(
            activation(),
            deadline_policy,
            &AtomicBool::new(false),
            |_| {
                Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Retryable,
                    1,
                ))
            },
        )
        .unwrap();
        assert!(matches!(
            outcome,
            FiniteSampleRecoveryOutcome::Deadline { .. }
        ));
    }
}
