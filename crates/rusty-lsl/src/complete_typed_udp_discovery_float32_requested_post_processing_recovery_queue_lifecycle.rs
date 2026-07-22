// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional observability over the completed P60 lifecycle and exact P61 facts.

use crate::{
    CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth,
    CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
    RequestedPostProcessingQueueHealth, RequestedPostProcessingQueueHealthConfig,
    RequestedPostProcessingQueueHealthError, RequestedPostProcessingQueueHealthSnapshot,
    RequestedPostProcessingQueueObservation, RequestedPostProcessingRecoveryConfig,
    RequestedPostProcessingRecoveryDisposition, RequestedPostProcessingRecoveryError,
    RequestedPostProcessingRecoveryObservation, RequestedPostProcessingRecoveryObserver,
    RequestedPostProcessingRecoverySnapshot,
};

/// Immutable, allocation-free projection across the three existing owners.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth {
    processing: Option<CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth>,
    recovery: RequestedPostProcessingRecoverySnapshot,
    queue: RequestedPostProcessingQueueHealthSnapshot,
}

impl CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth {
    /// Returns P60 committed processing health only when a recovered lifecycle completed.
    pub const fn processing(
        self,
    ) -> Option<CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth> {
        self.processing
    }

    /// Returns exact finite-recovery observation health.
    pub const fn recovery(self) -> RequestedPostProcessingRecoverySnapshot {
        self.recovery
    }

    /// Returns exact bounded-queue observation health.
    pub const fn queue(self) -> RequestedPostProcessingQueueHealthSnapshot {
        self.queue
    }
}

/// Typed projection refusal. No P61 state changes for any refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError {
    /// A completed P60 lifecycle was paired with a non-recovered disposition.
    CompletedLifecycleWithoutRecoveredSample,
    /// A terminal recovery disposition incorrectly carried a queue outcome.
    QueueObservationWithoutRecoveredSample,
    /// The existing recovery observer rejected the exact supplied fact.
    Recovery(RequestedPostProcessingRecoveryError),
    /// The existing queue-health owner rejected the exact supplied fact.
    Queue(RequestedPostProcessingQueueHealthError),
}

/// Bounded composition owner; it orchestrates projections and owns no subordinate runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle {
    processing: Option<CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth>,
    recovery: RequestedPostProcessingRecoveryObserver,
    queue: RequestedPostProcessingQueueHealth,
}

impl CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle {
    /// Constructs empty observability over caller-selected existing owners.
    pub const fn new(
        recovery: RequestedPostProcessingRecoveryConfig,
        queue: RequestedPostProcessingQueueHealthConfig,
    ) -> Self {
        Self {
            processing: None,
            recovery: RequestedPostProcessingRecoveryObserver::new(recovery),
            queue: RequestedPostProcessingQueueHealth::new(queue),
        }
    }

    /// Borrows no samples and returns only immutable exact counters/facts.
    pub const fn health(
        &self,
    ) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth {
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth {
            processing: self.processing,
            recovery: self.recovery.snapshot(),
            queue: self.queue.snapshot(),
        }
    }

    /// Atomically binds an already completed P60 lifecycle to exact recovered/queue facts.
    ///
    /// The lifecycle remains borrowed from its owner. Queue admission and recovery have already
    /// completed under their existing owners; this method neither repeats nor infers them.
    pub fn observe_completed(
        &mut self,
        completed: &CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
        recovery: RequestedPostProcessingRecoveryObservation,
        queue: RequestedPostProcessingQueueObservation,
    ) -> Result<
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError,
    > {
        self.observe(Some(completed.health()), recovery, Some(queue))
    }

    /// Atomically observes recovery exhaustion or cancellation, with no processing/queue claim.
    pub fn observe_terminal_recovery(
        &mut self,
        recovery: RequestedPostProcessingRecoveryObservation,
    ) -> Result<
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError,
    > {
        self.observe(None, recovery, None)
    }

    fn observe(
        &mut self,
        processing: Option<CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth>,
        recovery_observation: RequestedPostProcessingRecoveryObservation,
        queue_observation: Option<RequestedPostProcessingQueueObservation>,
    ) -> Result<
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueHealth,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError,
    > {
        let recovered = matches!(
            recovery_observation.disposition(),
            RequestedPostProcessingRecoveryDisposition::Recovered { .. }
        );
        if processing.is_some() && !recovered {
            return Err(CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError::CompletedLifecycleWithoutRecoveredSample);
        }
        if !recovered && queue_observation.is_some() {
            return Err(CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError::QueueObservationWithoutRecoveredSample);
        }

        let mut next_recovery = self.recovery.clone();
        let mut next_queue = self.queue.clone();
        next_recovery.observe(recovery_observation).map_err(
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError::Recovery,
        )?;
        if let Some(observation) = queue_observation {
            next_queue.observe(observation).map_err(
                CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError::Queue,
            )?;
        }
        self.processing = processing.or(self.processing);
        self.recovery = next_recovery;
        self.queue = next_queue;
        Ok(self.health())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RequestedPostProcessingSequenceLossFact;

    fn owner() -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle {
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle::new(
            RequestedPostProcessingRecoveryConfig::new(4, 2, 3).unwrap(),
            RequestedPostProcessingQueueHealthConfig::new(1, 4).unwrap(),
        )
    }

    #[test]
    fn terminal_exhaustion_and_cancellation_are_exact_and_bypass_queue() {
        let mut owner = owner();
        owner
            .observe_terminal_recovery(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Exhausted { attempts: 2 },
                None,
            ))
            .unwrap();
        owner
            .observe_terminal_recovery(RequestedPostProcessingRecoveryObservation::new(
                RequestedPostProcessingRecoveryDisposition::Cancelled {
                    completed_attempts: 1,
                },
                None,
            ))
            .unwrap();
        let health = owner.health();
        assert_eq!(health.recovery().exhausted_count(), 1);
        assert_eq!(health.recovery().cancelled_count(), 1);
        assert_eq!(health.queue().observation_count(), 0);
        assert_eq!(health.processing(), None);
    }

    #[test]
    fn recovered_success_and_full_queue_backpressure_are_observable() {
        let mut owner = owner();
        let recovered = RequestedPostProcessingRecoveryObservation::new(
            RequestedPostProcessingRecoveryDisposition::Recovered {
                successful_attempt: 2,
            },
            Some(RequestedPostProcessingSequenceLossFact::Gap {
                missing_sequence_count: 1,
            }),
        );
        owner
            .observe(
                None,
                recovered,
                Some(RequestedPostProcessingQueueObservation::Accepted { queue_len_after: 1 }),
            )
            .unwrap();
        let recovered = RequestedPostProcessingRecoveryObservation::new(
            RequestedPostProcessingRecoveryDisposition::Recovered {
                successful_attempt: 1,
            },
            Some(RequestedPostProcessingSequenceLossFact::Contiguous),
        );
        owner
            .observe(
                None,
                recovered,
                Some(RequestedPostProcessingQueueObservation::Backpressure { queue_len: 1 }),
            )
            .unwrap();
        let health = owner.health();
        assert_eq!(health.recovery().recovered_count(), 2);
        assert_eq!(health.recovery().explicit_missing_sequence_count(), 1);
        assert_eq!(health.queue().accepted_count(), 1);
        assert_eq!(health.queue().backpressure_count(), 1);
        assert_eq!(health.queue().high_water_len(), 1);
    }

    #[test]
    fn bounded_refusal_has_no_partial_state_mutation() {
        let mut owner = owner();
        let before = owner.clone();
        let error = owner
            .observe(
                None,
                RequestedPostProcessingRecoveryObservation::new(
                    RequestedPostProcessingRecoveryDisposition::Recovered {
                        successful_attempt: 1,
                    },
                    None,
                ),
                Some(RequestedPostProcessingQueueObservation::Backpressure { queue_len: 0 }),
            )
            .unwrap_err();
        assert_eq!(
            error,
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError::Queue(
                RequestedPostProcessingQueueHealthError::BackpressureWithoutFullQueue {
                    capacity: 1,
                    actual: 0
                }
            )
        );
        assert_eq!(owner, before);
    }

    #[test]
    fn contradictory_terminal_claims_fail_before_any_mutation() {
        let mut owner = owner();
        let before = owner.clone();
        let error = owner
            .observe(
                None,
                RequestedPostProcessingRecoveryObservation::new(
                    RequestedPostProcessingRecoveryDisposition::Cancelled {
                        completed_attempts: 0,
                    },
                    None,
                ),
                Some(RequestedPostProcessingQueueObservation::Cancelled { queue_len: 0 }),
            )
            .unwrap_err();
        assert_eq!(error, CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueError::QueueObservationWithoutRecoveredSample);
        assert_eq!(owner, before);
    }
}
