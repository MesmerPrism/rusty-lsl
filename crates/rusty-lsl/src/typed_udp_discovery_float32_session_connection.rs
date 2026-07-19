// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected typed-discovery response to bounded Float32 inlet-session composition.

use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedFloat32InletSession, TimestampedFloat32InletSessionReport,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    TimestampedFloat32SessionError, TimestampedFloat32SessionLimits,
    TimestampedFloat32SessionPreflightError, TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Stable failure preserving the existing endpoint and session owner types.
#[derive(Debug, PartialEq)]
pub enum TypedUdpDiscoveryFloat32SessionConnectionError {
    /// The caller-selected response could not produce a strict IPv4 service endpoint.
    Endpoint(TypedUdpDiscoveryEndpointError),
    /// The sole session owner rejected the bounded shape before TCP I/O.
    Preflight(TimestampedFloat32SessionPreflightError),
    /// The sole session owner failed after preflight.
    Session(TimestampedFloat32SessionError),
}

/// Projects one caller-selected completed discovery response and finishes one bounded inlet.
///
/// The caller retains the completed discovery run and every selection/configuration decision.
/// Endpoint projection precedes shape preflight, which precedes session I/O. This adapter owns no
/// discovery, selection, fallback, identity derivation, codec, socket lifecycle, or success report.
#[allow(clippy::too_many_arguments)]
pub fn run_selected_typed_udp_discovery_float32_session_inlet(
    discovery: &TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: TimestampedFloat32SampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    session_limits: TimestampedFloat32SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<TimestampedFloat32InletSessionReport, TypedUdpDiscoveryFloat32SessionConnectionError> {
    let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(discovery, response_index)
        .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint)?;
    let session = TimestampedFloat32InletSession::preflight_bounded(
        session_activation,
        endpoint.into(),
        expected_identity,
        handshake_limits,
        sample_limits,
        session_limits,
        channel_count,
        record_count,
    )
    .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::Preflight)?;
    session
        .finish(session_cancelled)
        .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::Session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamHandshakeError, StreamInfoObservedAdmissionLimits, StreamInfoVolatileFieldLimits,
        TimestampedFloat32OutletSession, TimestampedSample, UdpDiscoveryActivation,
        UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn identity(uid: &str) -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            uid.into(),
            "host".into(),
            "source".into(),
            "session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn discovery_activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap()
    }

    fn session_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, limits).unwrap(),
            limits,
        )
        .unwrap()
    }

    fn admission_limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
        )
    }

    fn document(address: &str, port: u16) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", "2".to_owned()),
            ("channel_format", "float32".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "11111111-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "session".to_owned()),
            ("hostname", "host".to_owned()),
            ("v4address", address.to_owned()),
            ("v4data_port", "43001".to_owned()),
            ("v4service_port", port.to_string()),
            ("v6address", "2001:db8::10".to_owned()),
            ("v6data_port", "43003".to_owned()),
            ("v6service_port", "43004".to_owned()),
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in fields {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn completed_discovery(document: String) -> TypedUdpDiscoveryRun {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let bytes = document.len();
        let responder = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut query).unwrap();
            socket
                .send_to(format!("19\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let run = run_typed_udp_discovery(
            discovery_activation(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    bytes + 32,
                    1,
                    Duration::from_millis(5),
                    Duration::from_millis(250),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            ),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            admission_limits(),
        )
        .unwrap();
        responder.join().unwrap();
        run
    }

    fn record(timestamp: f64, values: [f32; 2]) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(2).unwrap(), 2, values.to_vec()).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            None,
        )
    }

    #[test]
    fn selected_response_finishes_session_and_preserves_caller_discovery() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                &[record(11.25, [1.5, -2.5]), record(12.5, [3.25, -4.75])],
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port()));
        let report = run_selected_typed_udp_discovery_float32_session_inlet(
            &discovery,
            0,
            session_activation(),
            &identity("11111111-2222-4333-8444-555555555555"),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
            2,
            2,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(discovery.responses().len(), 1);
        assert_eq!(report.peer(), endpoint);
        assert_eq!(report.records()[1].sample().values(), &[3.25, -4.75]);
        assert_eq!(outlet.join().unwrap().record_count(), 2);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn endpoint_then_preflight_rejection_precedes_tcp_io() {
        let invalid_endpoint = completed_discovery(document("0.0.0.0", 9));
        assert!(matches!(
            run_selected_typed_udp_discovery_float32_session_inlet(
                &invalid_endpoint,
                0,
                session_activation(),
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                3,
                2,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(
                TypedUdpDiscoveryEndpointError::NonConcreteUnicastAddress
            ))
        ));
        let valid_endpoint = completed_discovery(document("127.0.0.1", 9));
        assert!(matches!(
            run_selected_typed_udp_discovery_float32_session_inlet(
                &valid_endpoint,
                0,
                session_activation(),
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                3,
                2,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Preflight(
                TimestampedFloat32SessionPreflightError::ChannelCount {
                    index: 0,
                    actual: 3
                }
            ))
        ));
    }

    #[test]
    fn session_cancellation_and_identity_failure_delegate_unchanged() {
        let discovery = completed_discovery(document("127.0.0.1", 9));
        assert!(matches!(
            run_selected_typed_udp_discovery_float32_session_inlet(
                &discovery,
                0,
                session_activation(),
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                2,
                2,
                &AtomicBool::new(true),
            ),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Session(
                TimestampedFloat32SessionError::Handshake(StreamHandshakeError::Cancelled)
            ))
        ));

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                &[record(1.0, [1.0, 2.0]), record(2.0, [3.0, 4.0])],
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port()));
        assert!(matches!(
            run_selected_typed_udp_discovery_float32_session_inlet(
                &discovery,
                0,
                session_activation(),
                &identity("aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                2,
                2,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Session(
                TimestampedFloat32SessionError::Handshake(StreamHandshakeError::InvalidHeader)
            ))
        ));
        assert!(matches!(
            outlet.join().unwrap(),
            Err(TimestampedFloat32SessionError::Handshake(
                StreamHandshakeError::IdentityMismatch
            ))
        ));
        TcpListener::bind(endpoint).unwrap();
    }
}
