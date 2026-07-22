// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded, data-only health for caller-observed post-processing queue outcomes.
//!
//! This module does not own a queue or infer queue activity. The caller feeds
//! exact outcomes from its existing queue/backpressure owner.

/// Validated fixed bounds for one queue-health owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedPostProcessingQueueHealthConfig {
    capacity: usize,
    observation_limit: u64,
}

impl RequestedPostProcessingQueueHealthConfig {
    /// Requires a nonzero queue capacity and observation bound.
    pub const fn new(
        capacity: usize,
        observation_limit: u64,
    ) -> Result<Self, RequestedPostProcessingQueueHealthConfigError> {
        if capacity == 0 {
            return Err(RequestedPostProcessingQueueHealthConfigError::ZeroCapacity);
        }
        if observation_limit == 0 {
            return Err(RequestedPostProcessingQueueHealthConfigError::ZeroObservationLimit);
        }
        Ok(Self {
            capacity,
            observation_limit,
        })
    }

    /// Returns the exact capacity of the separately owned queue.
    #[must_use]
    pub const fn capacity(self) -> usize {
        self.capacity
    }

    /// Returns the maximum number of admitted observations.
    #[must_use]
    pub const fn observation_limit(self) -> u64 {
        self.observation_limit
    }
}

/// Invalid queue-health configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingQueueHealthConfigError {
    /// The separately owned queue capacity was zero.
    ZeroCapacity,
    /// The health observation bound was zero.
    ZeroObservationLimit,
}

/// One exact outcome supplied by the owner of queue admission and policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingQueueObservation {
    /// One processed item was accepted; `queue_len_after` is the resulting length.
    Accepted {
        /// Exact queue length after admission.
        queue_len_after: usize,
    },
    /// The caller explicitly dropped one item; the queue itself makes no drop decision.
    Dropped {
        /// Exact retained queue length.
        queue_len: usize,
    },
    /// Admission observed a full queue.
    Backpressure {
        /// Exact full queue length.
        queue_len: usize,
    },
    /// Queue closure was observed at the stated retained length.
    Closed {
        /// Exact retained queue length at closure.
        queue_len: usize,
    },
    /// Caller cancellation prevented admission at the stated queue length.
    Cancelled {
        /// Exact retained queue length at cancellation.
        queue_len: usize,
    },
}

impl RequestedPostProcessingQueueObservation {
    const fn queue_len(self) -> usize {
        match self {
            Self::Accepted { queue_len_after } => queue_len_after,
            Self::Dropped { queue_len }
            | Self::Backpressure { queue_len }
            | Self::Closed { queue_len }
            | Self::Cancelled { queue_len } => queue_len,
        }
    }
}

/// Immutable exact counters and high-water evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestedPostProcessingQueueHealthSnapshot {
    capacity: usize,
    observation_count: u64,
    accepted_count: u64,
    dropped_count: u64,
    backpressure_count: u64,
    closed_count: u64,
    cancelled_count: u64,
    high_water_len: usize,
    last_observation: Option<RequestedPostProcessingQueueObservation>,
}

impl RequestedPostProcessingQueueHealthSnapshot {
    /// Returns the separately owned queue's fixed capacity.
    #[must_use]
    pub const fn capacity(self) -> usize {
        self.capacity
    }
    /// Returns the exact admitted observation count.
    #[must_use]
    pub const fn observation_count(self) -> u64 {
        self.observation_count
    }
    /// Returns the exact accepted count.
    #[must_use]
    pub const fn accepted_count(self) -> u64 {
        self.accepted_count
    }
    /// Returns the exact caller-dropped count.
    #[must_use]
    pub const fn dropped_count(self) -> u64 {
        self.dropped_count
    }
    /// Returns the exact full-queue backpressure count.
    #[must_use]
    pub const fn backpressure_count(self) -> u64 {
        self.backpressure_count
    }
    /// Returns the exact closure count.
    #[must_use]
    pub const fn closed_count(self) -> u64 {
        self.closed_count
    }
    /// Returns the exact cancelled-admission count.
    #[must_use]
    pub const fn cancelled_count(self) -> u64 {
        self.cancelled_count
    }
    /// Returns the maximum exact observed queue length.
    #[must_use]
    pub const fn high_water_len(self) -> usize {
        self.high_water_len
    }
    /// Returns the last admitted exact queue fact.
    #[must_use]
    pub const fn last_observation(self) -> Option<RequestedPostProcessingQueueObservation> {
        self.last_observation
    }
}

/// A bounded owner of data-only queue health.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestedPostProcessingQueueHealth {
    config: RequestedPostProcessingQueueHealthConfig,
    snapshot: RequestedPostProcessingQueueHealthSnapshot,
}

impl RequestedPostProcessingQueueHealth {
    /// Creates empty health without allocating queue or sample storage.
    #[must_use]
    pub const fn new(config: RequestedPostProcessingQueueHealthConfig) -> Self {
        Self {
            config,
            snapshot: RequestedPostProcessingQueueHealthSnapshot {
                capacity: config.capacity,
                observation_count: 0,
                accepted_count: 0,
                dropped_count: 0,
                backpressure_count: 0,
                closed_count: 0,
                cancelled_count: 0,
                high_water_len: 0,
                last_observation: None,
            },
        }
    }

    /// Validates and atomically admits one exact caller observation.
    pub fn observe(
        &mut self,
        observation: RequestedPostProcessingQueueObservation,
    ) -> Result<(), RequestedPostProcessingQueueHealthError> {
        if self.snapshot.observation_count >= self.config.observation_limit {
            return Err(
                RequestedPostProcessingQueueHealthError::ObservationLimitReached {
                    limit: self.config.observation_limit,
                },
            );
        }
        if self.snapshot.closed_count != 0 {
            return Err(RequestedPostProcessingQueueHealthError::ObservationAfterClosed);
        }
        let queue_len = observation.queue_len();
        if queue_len > self.config.capacity {
            return Err(
                RequestedPostProcessingQueueHealthError::QueueLengthExceedsCapacity {
                    capacity: self.config.capacity,
                    actual: queue_len,
                },
            );
        }
        if matches!(
            observation,
            RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 0 }
        ) {
            return Err(RequestedPostProcessingQueueHealthError::AcceptedWithZeroLength);
        }
        if let RequestedPostProcessingQueueObservation::Backpressure { queue_len } = observation {
            if queue_len != self.config.capacity {
                return Err(
                    RequestedPostProcessingQueueHealthError::BackpressureWithoutFullQueue {
                        capacity: self.config.capacity,
                        actual: queue_len,
                    },
                );
            }
        }

        let mut next = self.snapshot;
        next.observation_count = checked_increment(next.observation_count)?;
        match observation {
            RequestedPostProcessingQueueObservation::Accepted { .. } => {
                next.accepted_count = checked_increment(next.accepted_count)?;
            }
            RequestedPostProcessingQueueObservation::Dropped { .. } => {
                next.dropped_count = checked_increment(next.dropped_count)?;
            }
            RequestedPostProcessingQueueObservation::Backpressure { .. } => {
                next.backpressure_count = checked_increment(next.backpressure_count)?;
            }
            RequestedPostProcessingQueueObservation::Closed { .. } => {
                next.closed_count = checked_increment(next.closed_count)?;
            }
            RequestedPostProcessingQueueObservation::Cancelled { .. } => {
                next.cancelled_count = checked_increment(next.cancelled_count)?;
            }
        }
        next.high_water_len = next.high_water_len.max(queue_len);
        next.last_observation = Some(observation);
        self.snapshot = next;
        Ok(())
    }

    /// Returns a value copy containing no queue or sample ownership.
    #[must_use]
    pub const fn snapshot(&self) -> RequestedPostProcessingQueueHealthSnapshot {
        self.snapshot
    }
}

/// Typed fail-closed refusal; the health owner is unchanged on every variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestedPostProcessingQueueHealthError {
    /// The configured number of observations was already admitted.
    ObservationLimitReached {
        /// Configured maximum observation count.
        limit: u64,
    },
    /// The reported queue length exceeded the separately owned queue's capacity.
    QueueLengthExceedsCapacity {
        /// Configured queue capacity.
        capacity: usize,
        /// Reported queue length.
        actual: usize,
    },
    /// An accepted item cannot result in an empty queue.
    AcceptedWithZeroLength,
    /// Full-queue backpressure was reported below capacity.
    BackpressureWithoutFullQueue {
        /// Configured queue capacity.
        capacity: usize,
        /// Reported queue length.
        actual: usize,
    },
    /// An outcome was reported after the terminal close fact.
    ObservationAfterClosed,
    /// An affected exact counter could not be incremented.
    CounterOverflow,
}

fn checked_increment(value: u64) -> Result<u64, RequestedPostProcessingQueueHealthError> {
    value
        .checked_add(1)
        .ok_or(RequestedPostProcessingQueueHealthError::CounterOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn queue_health(capacity: usize, limit: u64) -> RequestedPostProcessingQueueHealth {
        RequestedPostProcessingQueueHealth::new(
            RequestedPostProcessingQueueHealthConfig::new(capacity, limit).unwrap(),
        )
    }

    #[test]
    fn exact_outcomes_and_high_water_are_projected_without_queue_storage() {
        let mut health = queue_health(3, 5);
        let observations = [
            RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 1 },
            RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 3 },
            RequestedPostProcessingQueueObservation::Backpressure { queue_len: 3 },
            RequestedPostProcessingQueueObservation::Dropped { queue_len: 2 },
            RequestedPostProcessingQueueObservation::Cancelled { queue_len: 1 },
        ];
        for observation in observations {
            health.observe(observation).unwrap();
        }
        let snapshot = health.snapshot();
        assert_eq!(snapshot.capacity(), 3);
        assert_eq!(snapshot.observation_count(), 5);
        assert_eq!(snapshot.accepted_count(), 2);
        assert_eq!(snapshot.dropped_count(), 1);
        assert_eq!(snapshot.backpressure_count(), 1);
        assert_eq!(snapshot.cancelled_count(), 1);
        assert_eq!(snapshot.closed_count(), 0);
        assert_eq!(snapshot.high_water_len(), 3);
        assert_eq!(snapshot.last_observation(), observations.last().copied());
    }

    #[test]
    fn invalid_configuration_is_typed() {
        assert_eq!(
            RequestedPostProcessingQueueHealthConfig::new(0, 1),
            Err(RequestedPostProcessingQueueHealthConfigError::ZeroCapacity)
        );
        assert_eq!(
            RequestedPostProcessingQueueHealthConfig::new(1, 0),
            Err(RequestedPostProcessingQueueHealthConfigError::ZeroObservationLimit)
        );
    }

    #[test]
    fn contradictory_observations_fail_without_mutation() {
        let cases = [
            (
                RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 0 },
                RequestedPostProcessingQueueHealthError::AcceptedWithZeroLength,
            ),
            (
                RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 3 },
                RequestedPostProcessingQueueHealthError::QueueLengthExceedsCapacity {
                    capacity: 2,
                    actual: 3,
                },
            ),
            (
                RequestedPostProcessingQueueObservation::Backpressure { queue_len: 1 },
                RequestedPostProcessingQueueHealthError::BackpressureWithoutFullQueue {
                    capacity: 2,
                    actual: 1,
                },
            ),
        ];
        for (observation, expected) in cases {
            let mut health = queue_health(2, 1);
            let before = health.clone();
            assert_eq!(health.observe(observation), Err(expected));
            assert_eq!(health, before);
        }
    }

    #[test]
    fn close_is_terminal_and_bound_precedence_is_stable() {
        let mut health = queue_health(2, 1);
        health
            .observe(RequestedPostProcessingQueueObservation::Closed { queue_len: 1 })
            .unwrap();
        let before = health.clone();
        assert_eq!(
            health
                .observe(RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 1 }),
            Err(RequestedPostProcessingQueueHealthError::ObservationLimitReached { limit: 1 })
        );
        assert_eq!(health, before);

        let mut health = queue_health(2, 2);
        health
            .observe(RequestedPostProcessingQueueObservation::Closed { queue_len: 0 })
            .unwrap();
        let before = health.clone();
        assert_eq!(
            health.observe(RequestedPostProcessingQueueObservation::Cancelled { queue_len: 0 }),
            Err(RequestedPostProcessingQueueHealthError::ObservationAfterClosed)
        );
        assert_eq!(health, before);
    }
}
