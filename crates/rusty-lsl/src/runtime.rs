// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Consumer-facing explicit runtime operations and activation contracts.
//!
//! This is an additive plane facade over the crate-root compatibility surface.
//! It defines no types, defaults, effects, or authority.

pub use crate::{
    admit_runtime_activation, run_finite_sample_recovery, run_fixed_width_numeric_inlet,
    run_fixed_width_numeric_outlet, run_integrated_clock_correction, run_short_info_responder,
    run_stream_inlet_handshake, run_stream_outlet_handshake, run_timestamped_float32_inlet,
    run_timestamped_float32_outlet, run_udp_discovery, BoundedSampleQueue,
    BoundedSampleQueueActivation, BoundedSampleQueueActivationError, BoundedSampleQueueCloseError,
    BoundedSampleQueueCreateError, BoundedSampleQueuePopError, BoundedSampleQueuePushError,
    BoundedSampleQueueWait, BoundedSampleQueueWaitError, ClockSource,
    FiniteSampleRecoveryActivation, FiniteSampleRecoveryActivationError, FiniteSampleRecoveryError,
    FiniteSampleRecoveryOutcome, FiniteSampleRecoveryPolicy, FiniteSampleRecoveryPolicyError,
    FiniteSampleRecoveryState, FixedWidthNumericRecord, FixedWidthNumericSampleActivation,
    FixedWidthNumericSampleActivationError, FixedWidthNumericSampleError,
    FixedWidthNumericSampleLimitError, FixedWidthNumericSampleLimits, FixedWidthNumericValue,
    IntegratedClockCorrection, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionActivationError, IntegratedClockCorrectionConfig,
    IntegratedClockCorrectionConfigError, IntegratedClockCorrectionError, RecoveryAttemptFailure,
    RecoveryFailureClass, RuntimeActivationAdmission, RuntimeActivationError,
    RuntimeActivationOutcome, RuntimeActivationReceipt, RuntimeActivationSelection, RuntimeModule,
    RuntimeModuleCapability, ShortInfoResponderActivation, ShortInfoResponderActivationError,
    ShortInfoResponderError, ShortInfoResponderLimitError, ShortInfoResponderLimits,
    ShortInfoResponderRun, ShortInfoResponderTermination, StreamHandshakeActivation,
    StreamHandshakeActivationError, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeIdentityError, StreamHandshakeIdentityRole, StreamHandshakeLimitError,
    StreamHandshakeLimits, StreamInletHandshake, StreamOutletHandshake,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleActivationError,
    TimestampedFloat32SampleError, TimestampedFloat32SampleLimitError,
    TimestampedFloat32SampleLimits, UdpDiscoveryActivation, UdpDiscoveryActivationError,
    UdpDiscoveryConfig, UdpDiscoveryError, UdpDiscoveryLimitError, UdpDiscoveryLimits,
    UdpDiscoveryResponse, UdpDiscoveryRun, UdpDiscoveryTermination,
    ACCEPTED_FEATURE_LOCK_FINGERPRINT, ACCEPTED_FEATURE_LOCK_REVISION,
    BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER, BOUNDED_SAMPLE_QUEUE_FEATURE_ID,
    FINITE_SAMPLE_RECOVERY_EFFECTIVE_MARKER, FINITE_SAMPLE_RECOVERY_FEATURE_ID,
    FIXED_WIDTH_NUMERIC_SAMPLE_EFFECTIVE_MARKER, FIXED_WIDTH_NUMERIC_SAMPLE_FEATURE_ID,
    INTEGRATED_CLOCK_CORRECTION_EFFECTIVE_MARKER, INTEGRATED_CLOCK_CORRECTION_FEATURE_ID,
    SHORT_INFO_RESPONDER_EFFECTIVE_MARKER, SHORT_INFO_RESPONDER_FEATURE_ID,
    STREAM_HANDSHAKE_EFFECTIVE_MARKER, STREAM_HANDSHAKE_FEATURE_ID,
    TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER, TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID,
    UDP_DISCOVERY_EFFECTIVE_MARKER, UDP_DISCOVERY_FEATURE_ID,
};
