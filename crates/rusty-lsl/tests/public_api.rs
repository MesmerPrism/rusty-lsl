// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External-consumer checks for the supported crate-root and role/plane API.

use rusty_lsl::{contract, runtime};

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
