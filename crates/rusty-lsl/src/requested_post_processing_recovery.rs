// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded observation of finite recovery before requested timestamp post-processing.
//!
//! This data-only contract observes facts supplied by existing recovery and sequence
//! owners. It performs no recovery, cancellation, loss inference, timestamp
//! processing, transport, clock, queue, session, activation, or background work.

/// Fixed bounds for one recovery observation owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedPostProcessingRecoveryConfig {
    observation_limit: u64,
    max_attempts_per_observation: usize,
    explicit_missing_sequence_limit: u64,
}

impl RequestedPostProcessingRecoveryConfig {
    /// Constructs explicit nonzero observation and per-observation attempt bounds.
    ///
    /// A zero missing-sequence limit is valid and rejects every positive gap.
    pub const fn new(
        observation_limit: u64,
        max_attempts_per_observation: usize,
        explicit_missing_sequence_limit: u64,
    ) -> Result<Self, RequestedPostProcessingRecoveryConfigError> {
        if observation_limit == 0 {
            return Err(RequestedPostProcessingRecoveryConfigError::ZeroObservationLimit);
        }
        if max_attempts_per_observation == 0 {
            return Err(RequestedPostProcessingRecoveryConfigError::ZeroAttemptLimit);
        }
        Ok(Self {
            observation_limit,
            max_attempts_per_observation,
            explicit_missing_sequence_limit,
        })
    }

    /// Returns the exact caller-supplied observation bound.
    pub const fn observation_limit(self) -> u64 {
        self.observation_limit
    }

    /// Returns the exact caller-supplied per-observation attempt bound.
    pub const fn max_attempts_per_observation(self) -> usize {
        self.max_attempts_per_observation
    }

    /// Returns the exact caller-supplied cumulative explicit-loss bound.
    pub const fn explicit_missing_sequence_limit(self) -> u64 {
        self.explicit_missing_sequence_limit
    }
}

/// Invalid recovery-observation configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingRecoveryConfigError {
    /// The owner could admit no observations.
    ZeroObservationLimit,
    /// An observation could represent no recovery attempt.
    ZeroAttemptLimit,
}

/// Exact terminal disposition reported by the existing finite recovery owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingRecoveryDisposition {
    /// A sample was recovered on this one-based attempt.
    Recovered { successful_attempt: usize },
    /// Retryable failures consumed this exact number of attempts.
    Exhausted { attempts: usize },
    /// Cancellation was observed after this exact number of completed attempts.
    Cancelled { completed_attempts: usize },
}

impl RequestedPostProcessingRecoveryDisposition {
    const fn attempts(self) -> usize {
        match self {
            Self::Recovered { successful_attempt } => successful_attempt,
            Self::Exhausted { attempts } => attempts,
            Self::Cancelled { completed_attempts } => completed_attempts,
        }
    }
}

/// Exact sequence-loss evidence associated with a recovered sample.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingSequenceLossFact {
    /// The sample established the first admitted sequence.
    First,
    /// The sample was contiguous with the prior high-water sequence.
    Contiguous,
    /// The sample advanced with this exact intervening missing extent.
    Gap { missing_sequence_count: u64 },
    /// The sample repeated the current high-water sequence.
    Duplicate,
    /// The sample was behind the current high-water sequence by this exact distance.
    OutOfOrder { behind_high_water_by: u64 },
}

/// One caller-supplied completed recovery observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedPostProcessingRecoveryObservation {
    disposition: RequestedPostProcessingRecoveryDisposition,
    sequence_loss: Option<RequestedPostProcessingSequenceLossFact>,
}

impl RequestedPostProcessingRecoveryObservation {
    /// Creates one observation without interpreting or inferring either fact.
    pub const fn new(
        disposition: RequestedPostProcessingRecoveryDisposition,
        sequence_loss: Option<RequestedPostProcessingSequenceLossFact>,
    ) -> Self {
        Self {
            disposition,
            sequence_loss,
        }
    }

    /// Returns the exact recovery disposition.
    pub const fn disposition(self) -> RequestedPostProcessingRecoveryDisposition {
        self.disposition
    }

    /// Returns the optional exact sequence-loss evidence.
    pub const fn sequence_loss(self) -> Option<RequestedPostProcessingSequenceLossFact> {
        self.sequence_loss
    }
}

/// Immutable exact counters from accepted observations.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RequestedPostProcessingRecoverySnapshot {
    observation_count: u64,
    attempt_count: u64,
    recovered_count: u64,
    exhausted_count: u64,
    cancelled_count: u64,
    cancellation_completed_attempt_count: u64,
    gap_count: u64,
    explicit_missing_sequence_count: u64,
    duplicate_count: u64,
    out_of_order_count: u64,
    last_disposition: Option<RequestedPostProcessingRecoveryDisposition>,
    last_sequence_loss: Option<RequestedPostProcessingSequenceLossFact>,
}

macro_rules! snapshot_accessors {
    ($($name:ident),+ $(,)?) => {$(
        #[doc = concat!("Returns the exact accepted `", stringify!($name), "`.")]
        pub const fn $name(&self) -> u64 { self.$name }
    )+};
}

impl RequestedPostProcessingRecoverySnapshot {
    snapshot_accessors!(
        observation_count,
        attempt_count,
        recovered_count,
        exhausted_count,
        cancelled_count,
        cancellation_completed_attempt_count,
        gap_count,
        explicit_missing_sequence_count,
        duplicate_count,
        out_of_order_count,
    );

    /// Returns the exact last accepted recovery disposition, if any.
    pub const fn last_disposition(&self) -> Option<RequestedPostProcessingRecoveryDisposition> {
        self.last_disposition
    }

    /// Returns the last accepted sequence-loss fact, if the last observation had one.
    pub const fn last_sequence_loss(&self) -> Option<RequestedPostProcessingSequenceLossFact> {
        self.last_sequence_loss
    }
}

/// Typed fail-closed refusal; every refusal leaves the owner unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingRecoveryError {
    /// The configured observation bound was already reached.
    ObservationLimitReached { limit: u64 },
    /// A required one-based attempt count was zero.
    ZeroAttempts {
        disposition: RequestedPostProcessingRecoveryDisposition,
    },
    /// The supplied attempt fact exceeded the configured per-observation bound.
    AttemptLimitExceeded { limit: usize, actual: usize },
    /// Exhaustion was claimed before every configured attempt was consumed.
    ExhaustionBeforeAttemptLimit { required: usize, actual: usize },
    /// A non-recovered disposition contradicted sample sequence evidence.
    SequenceLossWithoutRecoveredSample,
    /// A gap claimed no missing sequence.
    ZeroMissingSequenceGap,
    /// An out-of-order fact claimed no distance behind the high-water sequence.
    ZeroOutOfOrderDistance,
    /// The cumulative exact missing-sequence bound would be exceeded.
    ExplicitMissingSequenceLimitExceeded { limit: u64, required: u64 },
    /// An exact counter could not represent the accepted total.
    CounterOverflow,
    /// The platform attempt count could not be represented by the public counter.
    AttemptCountNotRepresentable { actual: usize },
}

/// Fixed-size owner of exact recovery, cancellation, and sequence-loss observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestedPostProcessingRecoveryObserver {
    config: RequestedPostProcessingRecoveryConfig,
    snapshot: RequestedPostProcessingRecoverySnapshot,
}

impl RequestedPostProcessingRecoveryObserver {
    /// Creates an empty observer without allocation or runtime activation.
    pub const fn new(config: RequestedPostProcessingRecoveryConfig) -> Self {
        Self {
            config,
            snapshot: RequestedPostProcessingRecoverySnapshot {
                observation_count: 0,
                attempt_count: 0,
                recovered_count: 0,
                exhausted_count: 0,
                cancelled_count: 0,
                cancellation_completed_attempt_count: 0,
                gap_count: 0,
                explicit_missing_sequence_count: 0,
                duplicate_count: 0,
                out_of_order_count: 0,
                last_disposition: None,
                last_sequence_loss: None,
            },
        }
    }

    /// Returns the immutable exact current snapshot.
    pub const fn snapshot(&self) -> RequestedPostProcessingRecoverySnapshot {
        self.snapshot
    }

    /// Transactionally admits one already-completed observation.
    pub fn observe(
        &mut self,
        observation: RequestedPostProcessingRecoveryObservation,
    ) -> Result<(), RequestedPostProcessingRecoveryError> {
        if self.snapshot.observation_count >= self.config.observation_limit {
            return Err(
                RequestedPostProcessingRecoveryError::ObservationLimitReached {
                    limit: self.config.observation_limit,
                },
            );
        }
        let attempts = observation.disposition.attempts();
        if attempts == 0
            && !matches!(
                observation.disposition,
                RequestedPostProcessingRecoveryDisposition::Cancelled { .. }
            )
        {
            return Err(RequestedPostProcessingRecoveryError::ZeroAttempts {
                disposition: observation.disposition,
            });
        }
        if attempts > self.config.max_attempts_per_observation {
            return Err(RequestedPostProcessingRecoveryError::AttemptLimitExceeded {
                limit: self.config.max_attempts_per_observation,
                actual: attempts,
            });
        }
        if matches!(
            observation.disposition,
            RequestedPostProcessingRecoveryDisposition::Exhausted { .. }
        ) && attempts != self.config.max_attempts_per_observation
        {
            return Err(
                RequestedPostProcessingRecoveryError::ExhaustionBeforeAttemptLimit {
                    required: self.config.max_attempts_per_observation,
                    actual: attempts,
                },
            );
        }
        if observation.sequence_loss.is_some()
            && !matches!(
                observation.disposition,
                RequestedPostProcessingRecoveryDisposition::Recovered { .. }
            )
        {
            return Err(RequestedPostProcessingRecoveryError::SequenceLossWithoutRecoveredSample);
        }
        let attempts = u64::try_from(attempts).map_err(|_| {
            RequestedPostProcessingRecoveryError::AttemptCountNotRepresentable { actual: attempts }
        })?;
        let mut next = self.snapshot;
        next.observation_count = checked_add(next.observation_count, 1)?;
        next.attempt_count = checked_add(next.attempt_count, attempts)?;
        match observation.disposition {
            RequestedPostProcessingRecoveryDisposition::Recovered { .. } => {
                next.recovered_count = checked_add(next.recovered_count, 1)?;
            }
            RequestedPostProcessingRecoveryDisposition::Exhausted { .. } => {
                next.exhausted_count = checked_add(next.exhausted_count, 1)?;
            }
            RequestedPostProcessingRecoveryDisposition::Cancelled { .. } => {
                next.cancelled_count = checked_add(next.cancelled_count, 1)?;
                next.cancellation_completed_attempt_count =
                    checked_add(next.cancellation_completed_attempt_count, attempts)?;
            }
        }
        match observation.sequence_loss {
            Some(RequestedPostProcessingSequenceLossFact::Gap {
                missing_sequence_count: 0,
            }) => {
                return Err(RequestedPostProcessingRecoveryError::ZeroMissingSequenceGap);
            }
            Some(RequestedPostProcessingSequenceLossFact::Gap {
                missing_sequence_count,
            }) => {
                let required =
                    checked_add(next.explicit_missing_sequence_count, missing_sequence_count)?;
                if required > self.config.explicit_missing_sequence_limit {
                    return Err(RequestedPostProcessingRecoveryError::ExplicitMissingSequenceLimitExceeded {
                        limit: self.config.explicit_missing_sequence_limit,
                        required,
                    });
                }
                next.gap_count = checked_add(next.gap_count, 1)?;
                next.explicit_missing_sequence_count = required;
            }
            Some(RequestedPostProcessingSequenceLossFact::Duplicate) => {
                next.duplicate_count = checked_add(next.duplicate_count, 1)?;
            }
            Some(RequestedPostProcessingSequenceLossFact::OutOfOrder {
                behind_high_water_by: 0,
            }) => {
                return Err(RequestedPostProcessingRecoveryError::ZeroOutOfOrderDistance);
            }
            Some(RequestedPostProcessingSequenceLossFact::OutOfOrder { .. }) => {
                next.out_of_order_count = checked_add(next.out_of_order_count, 1)?;
            }
            Some(RequestedPostProcessingSequenceLossFact::First)
            | Some(RequestedPostProcessingSequenceLossFact::Contiguous)
            | None => {}
        }
        next.last_disposition = Some(observation.disposition);
        next.last_sequence_loss = observation.sequence_loss;
        self.snapshot = next;
        Ok(())
    }
}

fn checked_add(left: u64, right: u64) -> Result<u64, RequestedPostProcessingRecoveryError> {
    left.checked_add(right)
        .ok_or(RequestedPostProcessingRecoveryError::CounterOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observer() -> RequestedPostProcessingRecoveryObserver {
        RequestedPostProcessingRecoveryObserver::new(
            RequestedPostProcessingRecoveryConfig::new(4, 3, 8).unwrap(),
        )
    }

    #[test]
    fn exact_recovery_cancellation_and_loss_facts_are_bounded() {
        let mut owner = observer();
        owner
            .observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: 2,
                },
                Some(RequestedPostProcessingSequenceLossFact::Gap {
                    missing_sequence_count: 3,
                }),
            ))
            .unwrap();
        owner
            .observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Exhausted { attempts: 3 },
                None,
            ))
            .unwrap();
        owner
            .observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Cancelled {
                    completed_attempts: 1,
                },
                None,
            ))
            .unwrap();
        let snapshot = owner.snapshot();
        assert_eq!(snapshot.observation_count(), 3);
        assert_eq!(snapshot.attempt_count(), 6);
        assert_eq!(snapshot.recovered_count(), 1);
        assert_eq!(snapshot.exhausted_count(), 1);
        assert_eq!(snapshot.cancelled_count(), 1);
        assert_eq!(snapshot.cancellation_completed_attempt_count(), 1);
        assert_eq!(snapshot.gap_count(), 1);
        assert_eq!(snapshot.explicit_missing_sequence_count(), 3);
    }

    #[test]
    fn pre_attempt_cancellation_is_truthful_and_zero_attempt_success_is_not() {
        let mut owner = observer();
        owner
            .observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Cancelled {
                    completed_attempts: 0,
                },
                None,
            ))
            .unwrap();
        let before = owner.clone();
        assert!(matches!(
            owner.observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: 0
                },
                None,
            )),
            Err(RequestedPostProcessingRecoveryError::ZeroAttempts { .. })
        ));
        assert_eq!(owner, before);
    }

    #[test]
    fn contradictory_or_invalid_loss_facts_fail_without_partial_mutation() {
        let invalid = [
            RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Exhausted { attempts: 1 },
                Some(RequestedPostProcessingSequenceLossFact::Duplicate),
            ),
            RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: 1,
                },
                Some(RequestedPostProcessingSequenceLossFact::Gap {
                    missing_sequence_count: 0,
                }),
            ),
            RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: 1,
                },
                Some(RequestedPostProcessingSequenceLossFact::OutOfOrder {
                    behind_high_water_by: 0,
                }),
            ),
            RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: 1,
                },
                Some(RequestedPostProcessingSequenceLossFact::Gap {
                    missing_sequence_count: 9,
                }),
            ),
        ];
        for observation in invalid {
            let mut owner = observer();
            let before = owner.clone();
            assert!(owner.observe(observation).is_err());
            assert_eq!(owner, before);
        }

        let mut owner = observer();
        let before = owner.clone();
        assert_eq!(
            owner.observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Exhausted { attempts: 2 },
                None,
            )),
            Err(
                RequestedPostProcessingRecoveryError::ExhaustionBeforeAttemptLimit {
                    required: 3,
                    actual: 2,
                }
            )
        );
        assert_eq!(owner, before);
    }

    #[test]
    fn configuration_attempt_and_observation_bounds_fail_closed() {
        assert_eq!(
            RequestedPostProcessingRecoveryConfig::new(0, 1, 0),
            Err(RequestedPostProcessingRecoveryConfigError::ZeroObservationLimit)
        );
        assert_eq!(
            RequestedPostProcessingRecoveryConfig::new(1, 0, 0),
            Err(RequestedPostProcessingRecoveryConfigError::ZeroAttemptLimit)
        );
        let config = RequestedPostProcessingRecoveryConfig::new(1, 1, 0).unwrap();
        let mut owner = RequestedPostProcessingRecoveryObserver::new(config);
        let before = owner.clone();
        assert_eq!(
            owner.observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Exhausted { attempts: 2 },
                None,
            )),
            Err(RequestedPostProcessingRecoveryError::AttemptLimitExceeded {
                limit: 1,
                actual: 2
            })
        );
        assert_eq!(owner, before);
        owner
            .observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Cancelled {
                    completed_attempts: 0,
                },
                None,
            ))
            .unwrap();
        let full = owner.clone();
        assert_eq!(
            owner.observe(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Recovered {
                    successful_attempt: 1
                },
                None,
            )),
            Err(RequestedPostProcessingRecoveryError::ObservationLimitReached { limit: 1 })
        );
        assert_eq!(owner, full);
    }
}
