// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit selected Float32 inlet, clock-correction, and queue composition.

use crate::{
    run_integrated_clock_correction, run_selected_typed_udp_discovery_float32_inlet,
    BoundedSampleQueue, BoundedSampleQueuePushError, BoundedSampleQueueWait, ClockSource,
    IntegratedClockCorrectionActivation, IntegratedClockCorrectionConfig,
    IntegratedClockCorrectionError, StreamHandshakeIdentity, StreamHandshakeLimits,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits, TimestampedSample,
    TypedUdpDiscoveryFloat32Error, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Stable owner-preserving failure from inlet, clock correction, or queue admission.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32ClockCorrectionQueueError {
    /// The existing selected-response Float32 inlet rejected or failed.
    Inlet(TypedUdpDiscoveryFloat32Error),
    /// The separate clock owner failed and the unchanged inlet record remains owned here.
    Clock {
        /// Existing integrated clock-correction failure.
        error: IntegratedClockCorrectionError,
        /// Unchanged inlet record returned to the caller.
        sample: TimestampedSample<f32>,
    },
    /// The separately activated caller-owned queue rejected the corrected record.
    Queue(BoundedSampleQueuePushError),
}

/// Receives one record, applies explicit clock correction, and admits it to an existing queue.
#[allow(clippy::too_many_arguments)]
pub fn run_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue<
    C: ClockSource,
>(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
    sample_activation: TimestampedFloat32SampleActivation,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    inlet_cancelled: &AtomicBool,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    clock_cancelled: &AtomicBool,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    queue_cancelled: &AtomicBool,
) -> Result<(), TypedUdpDiscoveryFloat32ClockCorrectionQueueError> {
    let sample = run_selected_typed_udp_discovery_float32_inlet(
        run,
        response_index,
        sample_activation,
        identity,
        handshake_limits,
        sample_limits,
        inlet_cancelled,
    )
    .map_err(TypedUdpDiscoveryFloat32ClockCorrectionQueueError::Inlet)?;
    let correction = match run_integrated_clock_correction(
        clock_activation,
        clock_config,
        clock,
        sample.raw_source_timestamp(),
        clock_cancelled,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Err(TypedUdpDiscoveryFloat32ClockCorrectionQueueError::Clock { error, sample })
        }
    };
    let (values, raw, _) = sample.into_parts();
    let corrected = TimestampedSample::new(values, raw, Some(correction.application().derived()));
    queue
        .push(corrected, queue_wait, queue_cancelled)
        .map_err(TypedUdpDiscoveryFloat32ClockCorrectionQueueError::Queue)
}

#[cfg(test)]
mod tests {
    use crate::{
        ClockOffset, ClockOffsetApplication, DerivedTimestampKind, RawSourceTimestamp, Sample,
        SampleLimits, TimestampedSample,
    };

    #[test]
    fn lslc_005c_reconstruction_preserves_raw_and_value_bits_beside_correction() {
        let raw = RawSourceTimestamp::new(-0.0).unwrap();
        let sample = TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(0x7fc0_1357)],
            )
            .unwrap(),
            raw,
            None,
        );
        let application =
            ClockOffsetApplication::apply(raw, ClockOffset::new(2.5).unwrap()).unwrap();
        let (values, retained_raw, _) = sample.into_parts();
        let corrected = TimestampedSample::new(values, retained_raw, Some(application.derived()));
        assert_eq!(
            corrected.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(corrected.sample().values()[0].to_bits(), 0x7fc0_1357);
        let derived = corrected.derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value(), 2.5);
    }
}
