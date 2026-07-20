// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External-consumer checks for the supported crate-root and role/plane API.

use std::collections::VecDeque;

use rusty_lsl::{
    contract, runtime, StreamInfoRuntimeAcquisition, StreamInfoRuntimeEvidenceLimit,
    StreamInfoRuntimeProvider, StreamInfoRuntimeProviderOutput, StreamInfoRuntimeValues,
    StreamInfoRuntimeWitness, StreamInfoTransportAcquisition, StreamInfoTransportAcquisitionError,
    StreamInfoTransportEvidenceError, StreamInfoTransportEvidenceLimit,
    StreamInfoTransportProvider, StreamInfoTransportProviderOutput, StreamInfoTransportValues,
    StreamInfoTransportWitness, StreamInfoVolatileFieldError, StreamInfoVolatileFieldLimits,
    StreamInfoVolatileFieldRole,
};

struct Lslc005zOneShotRuntimeProvider(Option<StreamInfoRuntimeProviderOutput>);

impl StreamInfoRuntimeProvider for Lslc005zOneShotRuntimeProvider {
    type Error = ();

    fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error> {
        self.0.take().ok_or(())
    }
}

fn lslc_005z_runtime_witness() -> StreamInfoRuntimeWitness {
    StreamInfoRuntimeWitness::new(
        StreamInfoRuntimeEvidenceLimit::new(32).expect("nonzero evidence limit"),
        "runtime-owner".into(),
        17,
        23,
    )
    .expect("bounded runtime witness")
}

fn same_type<T>(_: &T, _: &T) {}

#[test]
fn root_names_remain_type_identical_to_role_plane_facades() {
    let root = rusty_lsl::RawSourceTimestamp::new(1.25).expect("finite timestamp");
    let namespaced = contract::RawSourceTimestamp::new(1.25).expect("finite timestamp");
    same_type(&root, &namespaced);

    let root_module = rusty_lsl::RuntimeModule::StreamHandshake;
    let namespaced_module = runtime::RuntimeModule::StreamHandshake;
    same_type(&root_module, &namespaced_module);

    assert_eq!(
        rusty_lsl::ACCEPTED_FEATURE_LOCK_FINGERPRINT,
        runtime::ACCEPTED_FEATURE_LOCK_FINGERPRINT
    );
    assert_eq!(
        rusty_lsl::ACCEPTED_FEATURE_LOCK_REVISION,
        runtime::ACCEPTED_FEATURE_LOCK_REVISION
    );
    assert_eq!(
        rusty_lsl::DOCUMENTED_DEFAULT_DISCOVERY_PORT,
        contract::DOCUMENTED_DEFAULT_DISCOVERY_PORT
    );
}

#[test]
fn consumer_can_construct_an_inert_public_contract_without_private_modules() {
    let limits = contract::SampleLimits::new(1).expect("nonzero limits");
    let sample = contract::Sample::new(limits, 1, vec![7.0_f32]).expect("bounded sample");
    assert_eq!(sample.values(), &[7.0_f32]);
}

#[test]
fn lslc_005z_runtime_acquisition_parts_preserve_borrowed_witness_and_all_four_value_allocations() {
    let created_at = String::from("created-at-value");
    let uid = String::from("uid-value");
    let session_id = String::from("session-id-value");
    let hostname = String::from("hostname-value");
    let original_pointers = [
        created_at.as_ptr(),
        uid.as_ptr(),
        session_id.as_ptr(),
        hostname.as_ptr(),
    ];

    let mut provider = Lslc005zOneShotRuntimeProvider(Some(StreamInfoRuntimeProviderOutput::new(
        lslc_005z_runtime_witness(),
        StreamInfoRuntimeValues::new(created_at, uid, session_id, hostname),
    )));
    let accepted = StreamInfoRuntimeAcquisition::acquire(
        &mut provider,
        &lslc_005z_runtime_witness(),
        StreamInfoVolatileFieldLimits::new(32, 32, 32).expect("nonzero field limits"),
    )
    .expect("matching bounded runtime acquisition");

    assert_eq!(accepted.witness().provider_identity(), "runtime-owner");
    assert_eq!(accepted.witness().epoch(), 17);
    assert_eq!(accepted.witness().revision(), 23);
    assert_eq!(accepted.values().created_at(), "created-at-value");
    assert_eq!(accepted.values().uid(), "uid-value");
    assert_eq!(accepted.values().session_id(), "session-id-value");
    assert_eq!(accepted.values().hostname(), "hostname-value");
    assert_eq!(
        [
            accepted.values().created_at().as_ptr(),
            accepted.values().uid().as_ptr(),
            accepted.values().session_id().as_ptr(),
            accepted.values().hostname().as_ptr(),
        ],
        original_pointers
    );

    let (accepted_witness, accepted_values) = accepted.into_parts();
    assert_eq!(accepted_witness.provider_identity(), "runtime-owner");
    assert_eq!(
        (accepted_witness.epoch(), accepted_witness.revision()),
        (17, 23)
    );

    let parts = accepted_values.into_parts();
    assert_eq!(
        [
            parts.0.as_ptr(),
            parts.1.as_ptr(),
            parts.2.as_ptr(),
            parts.3.as_ptr()
        ],
        original_pointers
    );
    assert_eq!(
        [
            parts.0.as_str(),
            parts.1.as_str(),
            parts.2.as_str(),
            parts.3.as_str()
        ],
        [
            "created-at-value",
            "uid-value",
            "session-id-value",
            "hostname-value",
        ]
    );
}

#[test]
fn lslc_004u_typed_udp_projection_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryResponse>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryResponseError>() > 0);
}

#[test]
fn lslc_004v_typed_udp_discovery_run_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryRun>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryRunError>() > 0);
}

#[test]
fn lslc_004x_selected_endpoint_projection_is_public() {
    let _projection: fn(
        &rusty_lsl::TypedUdpDiscoveryRun,
        usize,
    ) -> Result<
        std::net::SocketAddrV4,
        rusty_lsl::TypedUdpDiscoveryEndpointError,
    > = rusty_lsl::propose_typed_udp_discovery_ipv4_service_endpoint;
}

#[test]
fn lslc_004y_selected_discovery_handshake_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryHandshakeError>() > 0);
    let _composition = rusty_lsl::run_selected_typed_udp_discovery_inlet_handshake;
}

#[test]
fn lslc_004z_selected_discovery_float32_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32Error>() > 0);
    let _composition = rusty_lsl::run_selected_typed_udp_discovery_float32_inlet;
}

#[test]
fn lslc_005a_selected_discovery_float32_queue_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32QueueError>() > 0);
    let _composition = rusty_lsl::run_selected_typed_udp_discovery_float32_inlet_into_queue;
}

#[test]
fn lslc_005b_selected_discovery_float32_recovery_queue_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32RecoveryQueueError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32RecoveryQueueOutcome>() > 0);
    let _composition =
        rusty_lsl::run_recovering_selected_typed_udp_discovery_float32_inlet_into_queue::<
            fn(
                usize,
                &rusty_lsl::TypedUdpDiscoveryFloat32Error,
            ) -> rusty_lsl::RecoveryAttemptFailure,
        >;
}

#[test]
fn lslc_005c_selected_discovery_float32_clock_correction_queue_is_public() {
    struct PublicClock;
    impl rusty_lsl::ClockSource for PublicClock {
        fn now(&mut self) -> f64 {
            0.0
        }
    }
    assert!(
        core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32ClockCorrectionQueueError>() > 0
    );
    let _composition =
        rusty_lsl::run_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue::<
            PublicClock,
        >;
}

#[test]
fn lslc_005d_recovery_clock_correction_queue_is_public() {
    struct PublicClock;
    impl rusty_lsl::ClockSource for PublicClock {
        fn now(&mut self) -> f64 {
            0.0
        }
    }
    assert!(
        core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError>(
        ) > 0
    );
    assert!(
        core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome>(
        ) > 0
    );
    let _composition = rusty_lsl::run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue::<
        PublicClock,
        fn(usize, &rusty_lsl::TypedUdpDiscoveryFloat32Error) -> rusty_lsl::RecoveryAttemptFailure,
    >;
}

#[test]
fn p4_bounded_recovery_clock_queue_pipeline_is_public() {
    struct PublicClock;
    impl rusty_lsl::ClockSource for PublicClock {
        fn now(&mut self) -> f64 {
            0.0
        }
    }
    assert!(core::mem::size_of::<rusty_lsl::BoundedFloat32PipelineCancellation<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::BoundedFloat32PipelineOutcome>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::BoundedFloat32PipelineError>() > 0);
    let _run = rusty_lsl::run_bounded_float32_recovery_clock_queue::<
        PublicClock,
        fn(usize) -> Result<rusty_lsl::TimestampedSample<f32>, rusty_lsl::RecoveryAttemptFailure>,
    >;
    let _runtime_run = rusty_lsl::runtime::run_bounded_float32_recovery_clock_queue::<
        PublicClock,
        fn(usize) -> Result<rusty_lsl::TimestampedSample<f32>, rusty_lsl::RecoveryAttemptFailure>,
    >;
}

#[test]
fn p9_float32_session_report_pipeline_adapter_is_public_on_both_facades() {
    struct PublicClock;
    impl rusty_lsl::ClockSource for PublicClock {
        fn now(&mut self) -> f64 {
            0.0
        }
    }
    assert!(core::mem::size_of::<rusty_lsl::Float32SessionReportPipelineError>() > 0);
    let _root = rusty_lsl::run_float32_inlet_session_report_recovery_clock_queue::<PublicClock>;
    let _runtime =
        rusty_lsl::runtime::run_float32_inlet_session_report_recovery_clock_queue::<PublicClock>;
}

#[test]
fn p4_session_batch_boundary_is_concrete_fail_closed_and_revision_33() {
    struct PublicClock;
    impl rusty_lsl::ClockSource for PublicClock {
        fn now(&mut self) -> f64 {
            0.0
        }
    }

    // The batch milestone broadens this concrete Float32 composition; it does not
    // introduce a caller-selectable strategy or a second lifecycle/queue owner.
    let _root =
        rusty_lsl::run_float32_inlet_session_report_batch_recovery_clock_queue::<PublicClock>;
    let _runtime = rusty_lsl::runtime::run_float32_inlet_session_report_batch_recovery_clock_queue::<
        PublicClock,
    >;

    let root_outcome: Option<rusty_lsl::Float32SessionReportBatchOutcome> = None;
    let _: Option<rusty_lsl::runtime::Float32SessionReportBatchOutcome> = root_outcome;
    let root_record: Option<rusty_lsl::Float32SessionReportRecordOutcome> = None;
    let _: Option<rusty_lsl::runtime::Float32SessionReportRecordOutcome> = root_record;
    let root_error: Option<rusty_lsl::Float32SessionReportBatchError> = None;
    let _: Option<rusty_lsl::runtime::Float32SessionReportBatchError> = root_error;
    let root_termination: Option<rusty_lsl::Float32SessionReportBatchTermination> = None;
    let _: Option<rusty_lsl::runtime::Float32SessionReportBatchTermination> = root_termination;
    assert_eq!(rusty_lsl::ACCEPTED_FEATURE_LOCK_REVISION, 33);
    assert_eq!(rusty_lsl::runtime::ACCEPTED_FEATURE_LOCK_REVISION, 33);
}

#[test]
fn float32_two_record_chunk_candidate_types_are_public() {
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkLimits>() > 0);
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkOutletSession<'static>>()
            > 0
    );
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkInletSession<'static>>()
            > 0
    );
    assert!(
        core::mem::size_of::<
            rusty_lsl::TimestampedFloat32TwoRecordChunkAcceptedOutletSession<'static>,
        >() > 0
    );
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkConnectedInletSession>()
            > 0
    );
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkOutletSessionReport>() > 0
    );
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedFloat32TwoRecordChunkInletSessionReport>() > 0
    );
    let _root_outlet = rusty_lsl::TimestampedFloat32TwoRecordChunkOutletSession::preflight;
    let _runtime_outlet =
        rusty_lsl::runtime::TimestampedFloat32TwoRecordChunkOutletSession::preflight;
    let _root_inlet = rusty_lsl::TimestampedFloat32TwoRecordChunkInletSession::preflight;
    let _runtime_inlet =
        rusty_lsl::runtime::TimestampedFloat32TwoRecordChunkInletSession::preflight;
    let _root_accept = rusty_lsl::TimestampedFloat32TwoRecordChunkOutletSession::accept;
    let _runtime_accept = rusty_lsl::runtime::TimestampedFloat32TwoRecordChunkOutletSession::accept;
    let _root_connect = rusty_lsl::TimestampedFloat32TwoRecordChunkInletSession::connect;
    let _runtime_connect =
        rusty_lsl::runtime::TimestampedFloat32TwoRecordChunkInletSession::connect;
}

#[test]
fn lslc_007b_float32_session_owner_and_reports_are_public() {
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedFloat32AcceptedOutletSession<'static>>() > 0
    );
    assert!(
        core::mem::size_of::<rusty_lsl::runtime::TimestampedFloat32AcceptedOutletSession<'static>>(
        ) > 0
    );
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32OutletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32InletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32OutletSessionReport>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32InletSessionReport>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32SessionError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32SessionPreflightError>() > 0);
    let _outlet_preflight = rusty_lsl::TimestampedFloat32OutletSession::preflight;
    let _outlet_accept = rusty_lsl::TimestampedFloat32OutletSession::accept;
    let _inlet_preflight = rusty_lsl::TimestampedFloat32InletSession::preflight;
}

#[test]
fn p2_bounded_float32_shape_seam_is_public() {
    let limits = rusty_lsl::TimestampedFloat32SessionLimits::new(4, 16).unwrap();
    assert_eq!(limits.max_channels(), 4);
    assert_eq!(limits.max_records(), 16);
    assert_eq!(
        rusty_lsl::TimestampedFloat32SessionLimits::new(0, 1),
        Err(rusty_lsl::TimestampedFloat32SessionLimitError::ZeroMaxChannels)
    );
    let _outlet_bounded = rusty_lsl::TimestampedFloat32OutletSession::preflight_bounded;
    let _inlet_bounded = rusty_lsl::TimestampedFloat32InletSession::preflight_bounded;
}

#[test]
fn p17_phased_bounded_float32_shape_seam_is_concrete_and_public() {
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32SessionTransferError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedFloat32SessionIncomplete>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::runtime::TimestampedFloat32SessionTransferError>() > 0);
    let _outlet_next = rusty_lsl::TimestampedFloat32AcceptedOutletSession::transfer_next;
    let _outlet_complete = rusty_lsl::TimestampedFloat32AcceptedOutletSession::complete;
    let _inlet_next = rusty_lsl::TimestampedFloat32ConnectedInletSession::transfer_next;
    let _inlet_records = rusty_lsl::TimestampedFloat32ConnectedInletSession::received_records;
    let _inlet_complete = rusty_lsl::TimestampedFloat32ConnectedInletSession::complete;
}

#[test]
fn p2_bounded_double64_session_vertical_is_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TimestampedDouble64OutletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedDouble64InletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedDouble64SessionError>() > 0);
    let limits = rusty_lsl::TimestampedDouble64SessionIoLimits::new(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(2),
    )
    .unwrap();
    assert_eq!(limits.io_slice(), std::time::Duration::from_millis(1));
    let _root_outlet = rusty_lsl::TimestampedDouble64OutletSession::preflight_bounded;
    let _runtime_outlet = runtime::TimestampedDouble64OutletSession::preflight_bounded;
    let _root_inlet = rusty_lsl::TimestampedDouble64InletSession::preflight_bounded;
    let _runtime_inlet = runtime::TimestampedDouble64InletSession::preflight_bounded;
}

#[test]
fn p19_phased_double64_session_is_concrete_and_public_on_both_facades() {
    assert!(
        core::mem::size_of::<rusty_lsl::TimestampedDouble64AcceptedOutletSession<'static>>() > 0
    );
    assert!(core::mem::size_of::<rusty_lsl::TimestampedDouble64ConnectedInletSession>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedDouble64SessionTransferError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedDouble64SessionIncomplete>() > 0);
    let _root_accept = rusty_lsl::TimestampedDouble64OutletSession::accept;
    let _runtime_accept = runtime::TimestampedDouble64OutletSession::accept;
    let _root_connect = rusty_lsl::TimestampedDouble64InletSession::connect;
    let _runtime_connect = runtime::TimestampedDouble64InletSession::connect;
    let _outlet_next = rusty_lsl::TimestampedDouble64AcceptedOutletSession::transfer_next;
    let _outlet_complete = rusty_lsl::TimestampedDouble64AcceptedOutletSession::complete;
    let _inlet_next = rusty_lsl::TimestampedDouble64ConnectedInletSession::transfer_next;
    let _inlet_records = rusty_lsl::TimestampedDouble64ConnectedInletSession::received_records;
    let _inlet_complete = rusty_lsl::TimestampedDouble64ConnectedInletSession::complete;
}

#[test]
fn p10_bounded_integer_session_verticals_are_public_on_both_facades() {
    macro_rules! assert_facade {
        ($outlet:ty, $inlet:ty, $report:ty, $root_outlet:path, $runtime_outlet:path, $root_inlet:path, $runtime_inlet:path) => {{
            assert!(core::mem::size_of::<$outlet>() > 0);
            assert!(core::mem::size_of::<$inlet>() > 0);
            assert!(core::mem::size_of::<$report>() > 0);
            let _root_outlet = $root_outlet;
            let _runtime_outlet = $runtime_outlet;
            let _root_inlet = $root_inlet;
            let _runtime_inlet = $runtime_inlet;
        }};
    }
    assert_facade!(
        rusty_lsl::TimestampedInt32OutletSession<'static>,
        rusty_lsl::TimestampedInt32InletSession<'static>,
        rusty_lsl::TimestampedInt32InletSessionReport,
        rusty_lsl::TimestampedInt32OutletSession::preflight_bounded,
        runtime::TimestampedInt32OutletSession::preflight_bounded,
        rusty_lsl::TimestampedInt32InletSession::preflight_bounded,
        runtime::TimestampedInt32InletSession::preflight_bounded
    );
    assert_facade!(
        rusty_lsl::TimestampedInt16OutletSession<'static>,
        rusty_lsl::TimestampedInt16InletSession<'static>,
        rusty_lsl::TimestampedInt16InletSessionReport,
        rusty_lsl::TimestampedInt16OutletSession::preflight_bounded,
        runtime::TimestampedInt16OutletSession::preflight_bounded,
        rusty_lsl::TimestampedInt16InletSession::preflight_bounded,
        runtime::TimestampedInt16InletSession::preflight_bounded
    );
    assert_facade!(
        rusty_lsl::TimestampedInt8OutletSession<'static>,
        rusty_lsl::TimestampedInt8InletSession<'static>,
        rusty_lsl::TimestampedInt8InletSessionReport,
        rusty_lsl::TimestampedInt8OutletSession::preflight_bounded,
        runtime::TimestampedInt8OutletSession::preflight_bounded,
        rusty_lsl::TimestampedInt8InletSession::preflight_bounded,
        runtime::TimestampedInt8InletSession::preflight_bounded
    );
}

#[test]
fn p22_phased_integer_sessions_are_concrete_and_public_on_both_facades() {
    macro_rules! assert_phased {
        ($accepted:ty, $connected:ty, $transfer:ty, $incomplete:ty, $outlet:ident, $inlet:ident) => {{
            assert!(core::mem::size_of::<$accepted>() > 0);
            assert!(core::mem::size_of::<$connected>() > 0);
            assert!(core::mem::size_of::<$transfer>() > 0);
            assert!(core::mem::size_of::<$incomplete>() > 0);
            let _root_accept = rusty_lsl::$outlet::accept;
            let _runtime_accept = runtime::$outlet::accept;
            let _root_connect = rusty_lsl::$inlet::connect;
            let _runtime_connect = runtime::$inlet::connect;
        }};
    }
    assert_phased!(
        rusty_lsl::TimestampedInt32AcceptedOutletSession<'static>,
        rusty_lsl::TimestampedInt32ConnectedInletSession,
        rusty_lsl::TimestampedInt32SessionTransferError,
        rusty_lsl::TimestampedInt32SessionIncomplete,
        TimestampedInt32OutletSession,
        TimestampedInt32InletSession
    );
    assert_phased!(
        rusty_lsl::TimestampedInt16AcceptedOutletSession<'static>,
        rusty_lsl::TimestampedInt16ConnectedInletSession,
        rusty_lsl::TimestampedInt16SessionTransferError,
        rusty_lsl::TimestampedInt16SessionIncomplete,
        TimestampedInt16OutletSession,
        TimestampedInt16InletSession
    );
    assert_phased!(
        rusty_lsl::TimestampedInt8AcceptedOutletSession<'static>,
        rusty_lsl::TimestampedInt8ConnectedInletSession,
        rusty_lsl::TimestampedInt8SessionTransferError,
        rusty_lsl::TimestampedInt8SessionIncomplete,
        TimestampedInt8OutletSession,
        TimestampedInt8InletSession
    );
}

#[test]
fn p29_phased_int64_sessions_are_concrete_and_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64AcceptedOutletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64ConnectedInletSession>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64InletSessionReport>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64OutletSessionReport>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64SessionError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64SessionTransferError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64SessionIncomplete>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64SessionIoLimitError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64SessionLimitError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedInt64SessionPreflightError>() > 0);

    for (channels, records) in [(1, 1), (2, 3)] {
        let limits = rusty_lsl::TimestampedInt64SessionLimits::new(channels, records).unwrap();
        let _io_limits = rusty_lsl::TimestampedInt64SessionIoLimits::new(
            core::time::Duration::from_secs(1),
            core::time::Duration::from_secs(2),
        )
        .unwrap();
        assert_eq!(limits.channel_count(), channels);
        assert_eq!(limits.record_count(), records);
    }

    let _root_preflight_outlet = rusty_lsl::TimestampedInt64OutletSession::preflight_bounded;
    let _runtime_preflight_outlet = runtime::TimestampedInt64OutletSession::preflight_bounded;
    let _root_preflight_inlet = rusty_lsl::TimestampedInt64InletSession::preflight_bounded;
    let _runtime_preflight_inlet = runtime::TimestampedInt64InletSession::preflight_bounded;
    let _root_accept = rusty_lsl::TimestampedInt64OutletSession::accept;
    let _runtime_accept = runtime::TimestampedInt64OutletSession::accept;
    let _root_connect = rusty_lsl::TimestampedInt64InletSession::connect;
    let _runtime_connect = runtime::TimestampedInt64InletSession::connect;
    let _outlet_next = rusty_lsl::TimestampedInt64AcceptedOutletSession::transfer_next;
    let _outlet_complete = rusty_lsl::TimestampedInt64AcceptedOutletSession::complete;
    let _inlet_next = rusty_lsl::TimestampedInt64ConnectedInletSession::transfer_next;
    let _inlet_records = rusty_lsl::TimestampedInt64ConnectedInletSession::received_records;
    let _inlet_complete = rusty_lsl::TimestampedInt64ConnectedInletSession::complete;
}

#[test]
fn p11_bounded_string_session_vertical_is_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringOutletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringInletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringOutletSessionReport>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringInletSessionReport>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringSessionError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringSessionRole>() > 0);
    let _completion: rusty_lsl::TimestampedStringSessionCompletion =
        rusty_lsl::TimestampedStringSessionCompletion::Complete;
    let _root_outlet = rusty_lsl::TimestampedStringOutletSession::preflight_bounded;
    let _runtime_outlet = runtime::TimestampedStringOutletSession::preflight_bounded;
    let _root_inlet = rusty_lsl::TimestampedStringInletSession::preflight_bounded;
    let _runtime_inlet = runtime::TimestampedStringInletSession::preflight_bounded;
}

#[test]
fn p21_phased_string_session_is_concrete_and_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringAcceptedOutletSession<'static>>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringConnectedInletSession>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringSessionTransferError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TimestampedStringSessionIncomplete>() > 0);
    let _root_accept = rusty_lsl::TimestampedStringOutletSession::accept;
    let _runtime_accept = runtime::TimestampedStringOutletSession::accept;
    let _root_connect = rusty_lsl::TimestampedStringInletSession::connect;
    let _runtime_connect = runtime::TimestampedStringInletSession::connect;
    let _outlet_next = rusty_lsl::TimestampedStringAcceptedOutletSession::transfer_next;
    let _outlet_complete = rusty_lsl::TimestampedStringAcceptedOutletSession::complete;
    let _inlet_next = rusty_lsl::TimestampedStringConnectedInletSession::transfer_next;
    let _inlet_records = rusty_lsl::TimestampedStringConnectedInletSession::received_records;
    let _inlet_complete = rusty_lsl::TimestampedStringConnectedInletSession::complete;
}

#[test]
fn discovery_to_bounded_float32_session_vertical_is_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryFloat32SessionConnectionError>() > 0);
    let _root = rusty_lsl::run_selected_typed_udp_discovery_float32_session_inlet;
    let _runtime = runtime::run_selected_typed_udp_discovery_float32_session_inlet;
    same_type(&_root, &_runtime);
}

#[test]
fn p18_discovery_to_phased_float32_connection_is_public_on_both_facades() {
    let _root = rusty_lsl::connect_selected_typed_udp_discovery_float32_session_inlet;
    let _runtime = runtime::connect_selected_typed_udp_discovery_float32_session_inlet;
    same_type(&_root, &_runtime);
}

#[test]
fn selected_resolution_float32_socket_free_state_is_public_on_both_facades() {
    let _root = rusty_lsl::resolve_selected_typed_udp_discovery_float32_session_inlet;
    let _runtime = runtime::resolve_selected_typed_udp_discovery_float32_session_inlet;
    same_type(&_root, &_runtime);
    assert!(
        core::mem::size_of::<rusty_lsl::ResolvedTypedUdpDiscoveryFloat32Session<'static>>() > 0
    );
    assert!(core::mem::size_of::<runtime::ResolvedTypedUdpDiscoveryFloat32Session<'static>>() > 0);
}

#[test]
fn p20_discovery_to_phased_double64_connection_is_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryDouble64SessionConnectionError>() > 0);
    let _root_connect = rusty_lsl::connect_selected_typed_udp_discovery_double64_session_inlet;
    let _runtime_connect = runtime::connect_selected_typed_udp_discovery_double64_session_inlet;
    same_type(&_root_connect, &_runtime_connect);
    let _root_run = rusty_lsl::run_selected_typed_udp_discovery_double64_session_inlet;
    let _runtime_run = runtime::run_selected_typed_udp_discovery_double64_session_inlet;
    same_type(&_root_run, &_runtime_run);
}

#[test]
fn p24_discovery_to_phased_string_connection_is_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryStringSessionConnectionError>() > 0);
    same_type(
        &rusty_lsl::connect_selected_typed_udp_discovery_string_session_inlet,
        &runtime::connect_selected_typed_udp_discovery_string_session_inlet,
    );
    same_type(
        &rusty_lsl::run_selected_typed_udp_discovery_string_session_inlet,
        &runtime::run_selected_typed_udp_discovery_string_session_inlet,
    );
}

#[test]
fn p23_discovery_to_phased_integer_connections_are_public_on_both_facades() {
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryInt32SessionConnectionError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryInt16SessionConnectionError>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::TypedUdpDiscoveryInt8SessionConnectionError>() > 0);
    same_type(
        &rusty_lsl::connect_selected_typed_udp_discovery_int32_session_inlet,
        &runtime::connect_selected_typed_udp_discovery_int32_session_inlet,
    );
    same_type(
        &rusty_lsl::connect_selected_typed_udp_discovery_int16_session_inlet,
        &runtime::connect_selected_typed_udp_discovery_int16_session_inlet,
    );
    same_type(
        &rusty_lsl::connect_selected_typed_udp_discovery_int8_session_inlet,
        &runtime::connect_selected_typed_udp_discovery_int8_session_inlet,
    );
    same_type(
        &rusty_lsl::run_selected_typed_udp_discovery_int32_session_inlet,
        &runtime::run_selected_typed_udp_discovery_int32_session_inlet,
    );
    same_type(
        &rusty_lsl::run_selected_typed_udp_discovery_int16_session_inlet,
        &runtime::run_selected_typed_udp_discovery_int16_session_inlet,
    );
    same_type(
        &rusty_lsl::run_selected_typed_udp_discovery_int8_session_inlet,
        &runtime::run_selected_typed_udp_discovery_int8_session_inlet,
    );
}

#[derive(Debug, Eq, PartialEq)]
enum StatefulProviderError {
    TemporarilyUnavailable(u64),
    Exhausted,
}

struct StatefulProvider {
    calls: usize,
    outputs: VecDeque<Result<StreamInfoTransportProviderOutput, StatefulProviderError>>,
}

impl StreamInfoTransportProvider for StatefulProvider {
    type Error = StatefulProviderError;

    fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
        self.calls += 1;
        self.outputs
            .pop_front()
            .unwrap_or(Err(StatefulProviderError::Exhausted))
    }
}

fn stateful_witness(epoch: u64, revision: u64) -> StreamInfoTransportWitness {
    StreamInfoTransportWitness::new(
        StreamInfoTransportEvidenceLimit::new(16).unwrap(),
        "stateful-owner".into(),
        epoch,
        revision,
    )
    .unwrap()
}

fn stateful_output(
    epoch: u64,
    revision: u64,
    values: [&str; 6],
) -> StreamInfoTransportProviderOutput {
    StreamInfoTransportProviderOutput::new(
        stateful_witness(epoch, revision),
        StreamInfoTransportValues::new(
            values[0].into(),
            values[1].into(),
            values[2].into(),
            values[3].into(),
            values[4].into(),
            values[5].into(),
        ),
    )
}

fn stateful_limits(max_transport_code_points: usize) -> StreamInfoVolatileFieldLimits {
    StreamInfoVolatileFieldLimits::new(1, 1, max_transport_code_points).unwrap()
}

#[test]
fn sequential_stateful_acquisitions_are_call_isolated_and_recover_after_typed_failures() {
    let mut provider = StatefulProvider {
        calls: 0,
        outputs: VecDeque::from([
            Ok(stateful_output(7, 11, ["a1", "d1", "s1", "A1", "D1", "S1"])),
            Err(StatefulProviderError::TemporarilyUnavailable(23)),
            Ok(stateful_output(
                7,
                11,
                ["oversized", "d3", "s3", "A3", "D3", "S3"],
            )),
            Ok(stateful_output(7, 11, ["a4", "d4", "s4", "A4", "D4", "S4"])),
        ]),
    };
    let expected = stateful_witness(7, 11);

    let first =
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, stateful_limits(2))
            .expect("the first queued acquisition should be accepted");
    assert_eq!(provider.calls, 1);
    assert_eq!(first.values().v4address(), "a1");
    assert_eq!(first.values().v6service_port(), "S1");

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, stateful_limits(2)),
        Err(StreamInfoTransportAcquisitionError::Provider(
            StatefulProviderError::TemporarilyUnavailable(23)
        ))
    );
    assert_eq!(provider.calls, 2);
    assert_eq!(first.values().v4address(), "a1");

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, stateful_limits(2)),
        Err(StreamInfoTransportAcquisitionError::Value(
            StreamInfoVolatileFieldError::TextLimitExceeded {
                role: StreamInfoVolatileFieldRole::V4Address,
                expected_max: 2,
                actual: 9,
            }
        ))
    );
    assert_eq!(provider.calls, 3);
    assert_eq!(first.values().v6service_port(), "S1");

    let recovered =
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, stateful_limits(2))
            .expect("a later valid provider output should recover independently");
    assert_eq!(provider.calls, 4);
    assert_eq!(recovered.witness(), &expected);
    assert_eq!(recovered.values().v4address(), "a4");
    assert_eq!(recovered.values().v6service_port(), "S4");
    assert_eq!(first.values().v4address(), "a1");

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, stateful_limits(2)),
        Err(StreamInfoTransportAcquisitionError::Provider(
            StatefulProviderError::Exhausted
        ))
    );
    assert_eq!(provider.calls, 5);
}

struct EvidenceLimitProvider(Option<StreamInfoTransportProviderOutput>);

impl StreamInfoTransportProvider for EvidenceLimitProvider {
    type Error = &'static str;

    fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
        self.0.take().ok_or("provider called more than once")
    }
}

fn evidence_limit_witness(identity: &str, epoch: u64, revision: u64) -> StreamInfoTransportWitness {
    StreamInfoTransportWitness::new(
        StreamInfoTransportEvidenceLimit::new(identity.chars().count()).unwrap(),
        identity.to_owned(),
        epoch,
        revision,
    )
    .unwrap()
}

#[test]
fn lslc_005w_evidence_limit_rejects_zero_and_retains_the_exact_nonzero_bound() {
    assert_eq!(
        StreamInfoTransportEvidenceLimit::new(0),
        Err(StreamInfoTransportEvidenceError::InvalidProviderIdentityLimit)
    );

    let limit = StreamInfoTransportEvidenceLimit::new(17).unwrap();
    assert_eq!(limit.max_provider_identity_code_points(), 17);
}

#[test]
fn lslc_005w_provider_identity_bound_counts_unicode_scalars_instead_of_utf8_bytes() {
    let limit = StreamInfoTransportEvidenceLimit::new(3).unwrap();
    let accepted = StreamInfoTransportWitness::new(limit, "Aé🦀".to_owned(), 5, 8).unwrap();
    assert_eq!(accepted.provider_identity(), "Aé🦀");
    assert_eq!((accepted.epoch(), accepted.revision()), (5, 8));

    assert_eq!(
        StreamInfoTransportWitness::new(limit, "Aé🦀Z".to_owned(), 13, 21),
        Err(
            StreamInfoTransportEvidenceError::ProviderIdentityLimitExceeded {
                expected_max: 3,
                actual: 4,
            }
        )
    );
}

#[test]
fn lslc_005w_empty_identity_rejection_has_its_exact_payload() {
    assert_eq!(
        StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(1).unwrap(),
            String::new(),
            u64::MAX,
            u64::MAX,
        ),
        Err(StreamInfoTransportEvidenceError::EmptyProviderIdentity)
    );
}

#[test]
fn lslc_005w_acquisition_rejects_identity_before_epoch_revision_and_value_damage() {
    let returned = StreamInfoTransportProviderOutput::new(
        evidence_limit_witness("other", 89, 144),
        StreamInfoTransportValues::new(
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
        ),
    );
    let mut provider = EvidenceLimitProvider(Some(returned));

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(
            &mut provider,
            &evidence_limit_witness("owner", 34, 55),
            StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
        ),
        Err(StreamInfoTransportAcquisitionError::ProviderIdentityMismatch)
    );
}

#[test]
fn p4_float32_session_batch_health_is_concrete_and_public_on_both_facades() {
    fn same_type<T>(_: &T, _: &T) {}

    assert!(core::mem::size_of::<rusty_lsl::Float32SessionBatchHealth>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::Float32SessionBatchHealthClassification>() > 0);
    assert!(core::mem::size_of::<rusty_lsl::runtime::Float32SessionBatchHealth>() > 0);
    assert!(
        core::mem::size_of::<rusty_lsl::runtime::Float32SessionBatchHealthClassification>() > 0
    );

    same_type(
        &rusty_lsl::Float32SessionBatchHealth::from_outcome,
        &rusty_lsl::runtime::Float32SessionBatchHealth::from_outcome,
    );
    same_type(
        &rusty_lsl::Float32SessionBatchHealth::from_error,
        &rusty_lsl::runtime::Float32SessionBatchHealth::from_error,
    );
    let _total = rusty_lsl::Float32SessionBatchHealth::total_record_count;
    let _completed = rusty_lsl::Float32SessionBatchHealth::completed_record_count;
    let _remaining = rusty_lsl::Float32SessionBatchHealth::remaining_record_count;
    let _current = rusty_lsl::Float32SessionBatchHealth::current_record_index;
    let _classification = rusty_lsl::Float32SessionBatchHealth::classification;

    use rusty_lsl::Float32SessionBatchHealthClassification as Classification;
    let _classes = [
        Classification::Complete,
        Classification::EmptyReport,
        Classification::Cancelled,
        Classification::Deadline,
        Classification::Terminal,
        Classification::Exhausted,
        Classification::RecoveryError,
        Classification::PipelineError,
        Classification::Invariant,
    ];
}
