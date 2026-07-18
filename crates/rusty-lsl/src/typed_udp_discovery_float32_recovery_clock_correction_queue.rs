// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Finite inlet recovery followed by explicit clock correction and queue admission.

use crate::{
    run_finite_sample_recovery, run_integrated_clock_correction,
    run_selected_typed_udp_discovery_float32_inlet, BoundedSampleQueue,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, ClockSource,
    FiniteSampleRecoveryActivation, FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryState, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, IntegratedClockCorrectionError, RecoveryAttemptFailure,
    StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits, TimestampedSample, TypedUdpDiscoveryFloat32Error,
    TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Terminal result of the combined bounded composition.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome {
    /// One recovered, corrected record entered the queue.
    Queued {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The caller classified one inlet failure as terminal.
    Terminal {
        /// Unchanged failure.
        failure: RecoveryAttemptFailure,
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Every permitted attempt failed retryably.
    Exhausted {
        /// Last unchanged failure.
        failure: RecoveryAttemptFailure,
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Recovery cancellation was observed.
    Cancelled {
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The recovery deadline elapsed.
    Deadline {
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Stable owner-preserving setup, correction, or queue failure.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError {
    /// Recovery trace setup failed.
    Recovery(FiniteSampleRecoveryError),
    /// Correction failed and returns the unchanged recovered record.
    Clock {
        /// Existing correction failure.
        error: IntegratedClockCorrectionError,
        /// Unchanged recovered record.
        sample: TimestampedSample<f32>,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Queue admission failed and retains the corrected record.
    Queue {
        /// Existing queue failure.
        error: BoundedSampleQueuePushError,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Recovers one inlet record, corrects only that record, then queues it.
#[allow(clippy::too_many_arguments)]
pub fn run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue<
    C,
    K,
>(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
    sample_activation: TimestampedFloat32SampleActivation,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    inlet_cancelled: &AtomicBool,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    recovery_cancelled: &AtomicBool,
    mut classify: K,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    clock_cancelled: &AtomicBool,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    queue_cancelled: &AtomicBool,
) -> Result<
    TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome,
    TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError,
>
where
    C: ClockSource,
    K: FnMut(usize, &TypedUdpDiscoveryFloat32Error) -> RecoveryAttemptFailure,
{
    let recovery = run_finite_sample_recovery(
        recovery_activation,
        recovery_policy,
        recovery_cancelled,
        |attempt| {
            run_selected_typed_udp_discovery_float32_inlet(
                run,
                response_index,
                sample_activation,
                identity,
                handshake_limits,
                sample_limits,
                inlet_cancelled,
            )
            .map_err(|error| classify(attempt, &error))
        },
    )
    .map_err(TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Recovery)?;
    let (sample, states) = match recovery {
        FiniteSampleRecoveryOutcome::Recovered { sample, states } => (sample, states),
        FiniteSampleRecoveryOutcome::Terminal { failure, states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Terminal {
                    failure,
                    states,
                },
            )
        }
        FiniteSampleRecoveryOutcome::Exhausted { failure, states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Exhausted {
                    failure,
                    states,
                },
            )
        }
        FiniteSampleRecoveryOutcome::Cancelled { states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Cancelled { states },
            )
        }
        FiniteSampleRecoveryOutcome::Deadline { states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Deadline { states },
            )
        }
    };
    let correction = match run_integrated_clock_correction(
        clock_activation,
        clock_config,
        clock,
        sample.raw_source_timestamp(),
        clock_cancelled,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Err(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Clock {
                    error,
                    sample,
                    states,
                },
            )
        }
    };
    let (values, raw, _) = sample.into_parts();
    let corrected = TimestampedSample::new(values, raw, Some(correction.application().derived()));
    match queue.push(corrected, queue_wait, queue_cancelled) {
        Ok(()) => {
            Ok(TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Queued { states })
        }
        Err(error) => {
            Err(TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Queue { error, states })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ClockOffset, ClockOffsetApplication, DerivedTimestampKind, RawSourceTimestamp, Sample,
        SampleLimits,
    };
    #[test]
    fn lslc_005d_reconstruction_preserves_raw_and_value_bits() {
        let raw = RawSourceTimestamp::new(-0.0).unwrap();
        let sample = TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(0x7fc0_5d5d)],
            )
            .unwrap(),
            raw,
            None,
        );
        let application =
            ClockOffsetApplication::apply(raw, ClockOffset::new(3.0).unwrap()).unwrap();
        let (values, retained_raw, _) = sample.into_parts();
        let corrected = TimestampedSample::new(values, retained_raw, Some(application.derived()));
        assert_eq!(
            corrected.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(corrected.sample().values()[0].to_bits(), 0x7fc0_5d5d);
        assert_eq!(
            corrected.derived_timestamp().unwrap().kind(),
            DerivedTimestampKind::ClockCorrected
        );
        assert_eq!(corrected.derived_timestamp().unwrap().value(), 3.0);
    }
}
