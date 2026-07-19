// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded caller-selected discovery-to-Float32-session inlet composition.

use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, run_typed_udp_discovery, ShortInfoQueryWire,
    ShortInfoResponseEnvelopeLimits, StreamHandshakeIdentity, StreamHandshakeLimits,
    StreamInfoObservedAdmissionLimits, TimestampedFloat32InletSession,
    TimestampedFloat32InletSessionReport, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits, TimestampedFloat32SessionError,
    TimestampedFloat32SessionLimits, TimestampedFloat32SessionPreflightError,
    TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun, TypedUdpDiscoveryRunError,
    UdpDiscoveryActivation, UdpDiscoveryConfig,
};
use std::sync::atomic::AtomicBool;

/// Completed discovery and consuming inlet-session reports.
#[derive(Debug)]
pub struct TypedUdpDiscoveryFloat32SessionConnection {
    discovery: TypedUdpDiscoveryRun,
    session: TimestampedFloat32InletSessionReport,
}

impl TypedUdpDiscoveryFloat32SessionConnection {
    /// Returns the completed typed discovery run without changing its ownership.
    pub const fn discovery(&self) -> &TypedUdpDiscoveryRun {
        &self.discovery
    }

    /// Returns the completed inlet-session report without changing record ownership.
    pub const fn session(&self) -> &TimestampedFloat32InletSessionReport {
        &self.session
    }

    /// Consumes the composition and recovers both existing owner reports unchanged.
    pub fn into_parts(self) -> (TypedUdpDiscoveryRun, TimestampedFloat32InletSessionReport) {
        (self.discovery, self.session)
    }
}

/// Stable owner-preserving failure from the composed vertical.
#[derive(Debug, PartialEq)]
pub enum TypedUdpDiscoveryFloat32SessionConnectionError {
    /// The existing bounded typed discovery call failed unchanged.
    Discovery(TypedUdpDiscoveryRunError),
    /// The caller-selected response could not produce a strict IPv4 service endpoint.
    Endpoint {
        /// Unchanged strict endpoint projection failure.
        error: TypedUdpDiscoveryEndpointError,
        /// Completed discovery ownership retained for the caller.
        discovery: TypedUdpDiscoveryRun,
    },
    /// The sole session owner rejected the caller-selected bounded shape before TCP I/O.
    SessionPreflight {
        /// Unchanged session-owner preflight failure.
        error: TimestampedFloat32SessionPreflightError,
        /// Completed discovery ownership retained for the caller.
        discovery: TypedUdpDiscoveryRun,
    },
    /// The sole session owner failed after its preflight completed.
    Session {
        /// Unchanged started-session failure.
        error: TimestampedFloat32SessionError,
        /// Completed discovery ownership retained for the caller.
        discovery: TypedUdpDiscoveryRun,
    },
}

/// Discovers, projects one caller-selected response, and completes one bounded inlet session.
///
/// The caller retains every configuration and selection decision. Discovery completes first;
/// strict endpoint projection and session shape preflight then run in that order before any TCP
/// connection. No response selection, fallback, retry, identity derivation, or lifecycle owner is
/// added by this composition.
#[allow(clippy::too_many_arguments)]
pub fn run_typed_udp_discovery_float32_session_inlet(
    discovery_activation: UdpDiscoveryActivation,
    discovery_config: UdpDiscoveryConfig,
    query: &ShortInfoQueryWire,
    discovery_cancelled: &AtomicBool,
    envelope_limits: ShortInfoResponseEnvelopeLimits,
    admission_limits: StreamInfoObservedAdmissionLimits,
    response_index: usize,
    session_activation: TimestampedFloat32SampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    session_limits: TimestampedFloat32SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<TypedUdpDiscoveryFloat32SessionConnection, TypedUdpDiscoveryFloat32SessionConnectionError>
{
    let discovery = run_typed_udp_discovery(
        discovery_activation,
        discovery_config,
        query,
        discovery_cancelled,
        envelope_limits,
        admission_limits,
    )
    .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::Discovery)?;
    let endpoint =
        match propose_typed_udp_discovery_ipv4_service_endpoint(&discovery, response_index) {
            Ok(endpoint) => endpoint,
            Err(error) => {
                return Err(TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint {
                    error,
                    discovery,
                })
            }
        };
    let session = match TimestampedFloat32InletSession::preflight_bounded(
        session_activation,
        endpoint.into(),
        expected_identity,
        handshake_limits,
        sample_limits,
        session_limits,
        channel_count,
        record_count,
    ) {
        Ok(session) => session,
        Err(error) => {
            return Err(
                TypedUdpDiscoveryFloat32SessionConnectionError::SessionPreflight {
                    error,
                    discovery,
                },
            )
        }
    };
    let session = match session.finish(session_cancelled) {
        Ok(session) => session,
        Err(error) => {
            return Err(TypedUdpDiscoveryFloat32SessionConnectionError::Session {
                error,
                discovery,
            })
        }
    };
    Ok(TypedUdpDiscoveryFloat32SessionConnection { discovery, session })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample, SampleLimits,
        ShortInfoQuery, ShortInfoQueryWireLimits, StreamDescriptorLimits,
        StreamHandshakeActivation, StreamHandshakeError, StreamInfoVolatileFieldLimits,
        TimestampedFloat32OutletSession, TimestampedSample, UdpDiscoveryLimits,
        UdpDiscoveryTermination,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::atomic::AtomicBool;
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

    fn document(address: &str, port: &str) -> String {
        let fields = [
            ("name", "selected"),
            ("type", "independent"),
            ("channel_count", "2"),
            ("channel_format", "float32"),
            ("source_id", "source"),
            ("nominal_srate", "100.0000000000000"),
            ("version", "110"),
            ("created_at", "1"),
            ("uid", "11111111-2222-4333-8444-555555555555"),
            ("session_id", "session"),
            ("hostname", "host"),
            ("v4address", address),
            ("v4data_port", "43001"),
            ("v4service_port", port),
            ("v6address", "2001:db8::10"),
            ("v6data_port", "43003"),
            ("v6service_port", "43004"),
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in fields {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn discovery_config(
        destination: std::net::SocketAddr,
        document_bytes: usize,
    ) -> UdpDiscoveryConfig {
        UdpDiscoveryConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            destination,
            UdpDiscoveryLimits::new(
                document_bytes + 32,
                1,
                Duration::from_millis(5),
                Duration::from_millis(250),
            )
            .unwrap(),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
        )
    }

    fn record(timestamp: f64, values: [f32; 2]) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(2).unwrap(), 2, values.to_vec()).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            None,
        )
    }

    #[test]
    fn connection_uses_exact_selected_endpoint_forwards_shape_and_releases_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let service_address = listener.local_addr().unwrap();
        let records = [record(11.25, [1.5, -2.5]), record(12.5, [3.25, -4.75])];
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });

        let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = udp.local_addr().unwrap();
        let body = document("127.0.0.1", &service_address.port().to_string());
        let document_bytes = body.len();
        let responder = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (_, source) = udp.recv_from(&mut bytes).unwrap();
            udp.send_to(format!("19\r\n{body}").as_bytes(), source)
                .unwrap();
        });

        let result = run_typed_udp_discovery_float32_session_inlet(
            discovery_activation(),
            discovery_config(destination, document_bytes),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
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
        responder.join().unwrap();
        let sent = outlet.join().unwrap();
        assert_eq!(
            result.discovery().termination(),
            UdpDiscoveryTermination::ResponseLimit
        );
        assert_eq!(result.session().peer(), service_address);
        assert_eq!(
            (
                result.session().channel_count(),
                result.session().record_count()
            ),
            (2, 2)
        );
        assert_eq!(
            result.session().records()[1].sample().values(),
            &[3.25, -4.75]
        );
        let (discovery, session) = result.into_parts();
        assert_eq!(discovery.responses().len(), 1);
        assert_eq!(session.into_records().len(), 2);
        assert_eq!(sent.channel_count(), 2);
        TcpListener::bind(service_address).unwrap();
    }

    #[test]
    fn endpoint_then_shape_rejection_precedes_tcp_io() {
        for (address, channel_count, expected) in
            [("0.0.0.0", 2, "endpoint"), ("127.0.0.1", 3, "shape")]
        {
            let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
            let destination = udp.local_addr().unwrap();
            let body = document(address, "9");
            let document_bytes = body.len();
            let responder = thread::spawn(move || {
                let mut bytes = [0_u8; 256];
                let (_, source) = udp.recv_from(&mut bytes).unwrap();
                udp.send_to(format!("19\r\n{body}").as_bytes(), source)
                    .unwrap();
            });
            let error = run_typed_udp_discovery_float32_session_inlet(
                discovery_activation(),
                discovery_config(destination, document_bytes),
                &query(),
                &AtomicBool::new(false),
                ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
                admission_limits(),
                0,
                session_activation(),
                &identity("11111111-2222-4333-8444-555555555555"),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                channel_count,
                2,
                &AtomicBool::new(false),
            )
            .unwrap_err();
            responder.join().unwrap();
            match expected {
                "endpoint" => assert!(
                    matches!(error, TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint { error: TypedUdpDiscoveryEndpointError::NonConcreteUnicastAddress, discovery } if discovery.responses().len() == 1)
                ),
                _ => assert!(
                    matches!(error, TypedUdpDiscoveryFloat32SessionConnectionError::SessionPreflight { error: TimestampedFloat32SessionPreflightError::ChannelCount { index: 0, actual: 3 }, discovery } if discovery.responses().len() == 1)
                ),
            }
        }
    }

    #[test]
    fn discovery_cancellation_precedes_selection_and_session_cancellation_precedes_connect() {
        let body = document("127.0.0.1", "9");
        let document_bytes = body.len();
        let discovery_cancelled = AtomicBool::new(true);
        let error = run_typed_udp_discovery_float32_session_inlet(
            discovery_activation(),
            discovery_config("127.0.0.1:9".parse().unwrap(), document_bytes),
            &query(),
            &discovery_cancelled,
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
            7,
            session_activation(),
            &identity("11111111-2222-4333-8444-555555555555"),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
            2,
            2,
            &AtomicBool::new(false),
        )
        .unwrap_err();
        assert!(
            matches!(error, TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint {
            error: TypedUdpDiscoveryEndpointError::ResponseUnavailable { index: 7, response_count: 0 },
            discovery,
        } if discovery.termination() == UdpDiscoveryTermination::Cancelled)
        );

        let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = udp.local_addr().unwrap();
        let responder = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (_, source) = udp.recv_from(&mut bytes).unwrap();
            udp.send_to(format!("19\r\n{body}").as_bytes(), source)
                .unwrap();
        });
        let session_cancelled = AtomicBool::new(true);
        let error = run_typed_udp_discovery_float32_session_inlet(
            discovery_activation(),
            discovery_config(destination, document_bytes),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
            0,
            session_activation(),
            &identity("11111111-2222-4333-8444-555555555555"),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
            2,
            2,
            &session_cancelled,
        )
        .unwrap_err();
        responder.join().unwrap();
        assert!(
            matches!(error, TypedUdpDiscoveryFloat32SessionConnectionError::Session {
            error: TimestampedFloat32SessionError::Handshake(StreamHandshakeError::Cancelled),
            discovery,
        } if discovery.responses().len() == 1)
        );
    }

    #[test]
    fn deadline_and_identity_mismatch_retain_discovery_ownership() {
        let body = document("127.0.0.1", "9");
        let document_bytes = body.len();
        let silent = UdpSocket::bind("127.0.0.1:0").unwrap();
        let error = run_typed_udp_discovery_float32_session_inlet(
            discovery_activation(),
            discovery_config(silent.local_addr().unwrap(), document_bytes),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
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
        .unwrap_err();
        assert!(
            matches!(error, TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint {
            error: TypedUdpDiscoveryEndpointError::ResponseUnavailable { index: 0, response_count: 0 },
            discovery,
        } if discovery.termination() == UdpDiscoveryTermination::Deadline)
        );

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let service_address = listener.local_addr().unwrap();
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
        let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = udp.local_addr().unwrap();
        let body = document("127.0.0.1", &service_address.port().to_string());
        let document_bytes = body.len();
        let responder = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (_, source) = udp.recv_from(&mut bytes).unwrap();
            udp.send_to(format!("19\r\n{body}").as_bytes(), source)
                .unwrap();
        });
        let error = run_typed_udp_discovery_float32_session_inlet(
            discovery_activation(),
            discovery_config(destination, document_bytes),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
            0,
            session_activation(),
            &identity("aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
            2,
            2,
            &AtomicBool::new(false),
        )
        .unwrap_err();
        responder.join().unwrap();
        assert!(
            matches!(error, TypedUdpDiscoveryFloat32SessionConnectionError::Session {
            error: TimestampedFloat32SessionError::Handshake(StreamHandshakeError::InvalidHeader),
            discovery,
        } if discovery.responses().len() == 1)
        );
        assert!(matches!(
            outlet.join().unwrap(),
            Err(TimestampedFloat32SessionError::Handshake(
                StreamHandshakeError::IdentityMismatch
            ))
        ));
        TcpListener::bind(service_address).unwrap();
    }
}
