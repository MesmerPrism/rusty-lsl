// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected typed-discovery response to bounded Float32 inlet-session composition.

use crate::typed_udp_discovery_session_contract::{
    validate_selected_typed_udp_discovery_session_contract,
    TypedUdpDiscoverySessionContractMismatch,
};
use crate::ChannelFormat;
use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, StreamHandshakeIdentity,
    StreamHandshakeIdentityRole, StreamHandshakeLimits, TimestampedFloat32ConnectedInletSession,
    TimestampedFloat32InletSession, TimestampedFloat32InletSessionReport,
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
    /// The selected response declared a different data format.
    Format {
        /// Required Float32 format.
        expected: ChannelFormat,
        /// Selected response format.
        actual: ChannelFormat,
    },
    /// The selected response declared a different channel count.
    ChannelCount {
        /// Caller-requested channel count.
        expected: usize,
        /// Selected response channel count.
        actual: usize,
    },
    /// One selected-response identity field differed from the caller expectation.
    Identity {
        /// Identity field checked in the foundation contract's fixed order.
        role: StreamHandshakeIdentityRole,
        /// Exact caller-owned expected evidence.
        expected: String,
        /// Exact discovery-owned actual evidence.
        actual: String,
    },
    /// The sole session owner rejected the bounded shape before TCP I/O.
    Preflight(TimestampedFloat32SessionPreflightError),
    /// The sole session owner failed after preflight.
    Session(TimestampedFloat32SessionError),
}

impl From<TypedUdpDiscoverySessionContractMismatch<'_>>
    for TypedUdpDiscoveryFloat32SessionConnectionError
{
    fn from(mismatch: TypedUdpDiscoverySessionContractMismatch<'_>) -> Self {
        match mismatch {
            TypedUdpDiscoverySessionContractMismatch::Format { expected, actual } => {
                Self::Format { expected, actual }
            }
            TypedUdpDiscoverySessionContractMismatch::ChannelCount { expected, actual } => {
                Self::ChannelCount { expected, actual }
            }
            TypedUdpDiscoverySessionContractMismatch::Identity {
                role,
                expected,
                actual,
            } => Self::Identity {
                role,
                expected: expected.to_owned(),
                actual: actual.to_owned(),
            },
        }
    }
}

/// Socket-free resolved Float32 selection with the existing inlet preflight owner.
pub struct ResolvedTypedUdpDiscoveryFloat32Session<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    session: TimestampedFloat32InletSession<'a>,
}

impl<'a> ResolvedTypedUdpDiscoveryFloat32Session<'a> {
    /// Returns the caller-owned discovery run retained by this resolution.
    pub const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }

    /// Returns the caller-selected receive-order response index.
    pub const fn response_index(&self) -> usize {
        self.response_index
    }

    /// Connects through the existing concrete phased inlet owner.
    pub fn connect(
        self,
        session_cancelled: &AtomicBool,
    ) -> Result<
        TimestampedFloat32ConnectedInletSession,
        TypedUdpDiscoveryFloat32SessionConnectionError,
    > {
        self.session
            .connect(session_cancelled)
            .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::Session)
    }
}

/// Resolves one caller-selected response and performs Float32 inlet preflight without TCP I/O.
///
/// Strict endpoint projection precedes the foundation format, channel-count, and four-role
/// identity contract. Existing bounded session preflight follows that contract. The returned
/// concrete state borrows the caller's discovery run and expected identity and owns the sole
/// existing preflighted inlet session.
#[allow(clippy::too_many_arguments)]
pub fn resolve_selected_typed_udp_discovery_float32_session_inlet<'a>(
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: TimestampedFloat32SampleActivation,
    expected_identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    session_limits: TimestampedFloat32SessionLimits,
    channel_count: usize,
    record_count: usize,
) -> Result<
    ResolvedTypedUdpDiscoveryFloat32Session<'a>,
    TypedUdpDiscoveryFloat32SessionConnectionError,
> {
    let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(discovery, response_index)
        .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint)?;
    validate_selected_typed_udp_discovery_session_contract(
        &discovery.responses()[response_index],
        ChannelFormat::Float32,
        channel_count,
        expected_identity,
    )
    .map_err(TypedUdpDiscoveryFloat32SessionConnectionError::from)?;
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
    Ok(ResolvedTypedUdpDiscoveryFloat32Session {
        discovery,
        response_index,
        session,
    })
}

/// Projects one caller-selected completed discovery response and connects one bounded inlet.
///
/// The caller retains the completed discovery run and every selection/configuration decision.
/// Endpoint projection precedes shape preflight, which precedes session I/O. The returned concrete
/// connected owner delegates phased transfer, completion, and report-free close to the sole private
/// session lifecycle. This adapter owns no discovery, selection, fallback, identity derivation,
/// codec, cursor, record allocation, socket lifecycle, or completion report.
#[allow(clippy::too_many_arguments)]
pub fn connect_selected_typed_udp_discovery_float32_session_inlet(
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
) -> Result<TimestampedFloat32ConnectedInletSession, TypedUdpDiscoveryFloat32SessionConnectionError>
{
    resolve_selected_typed_udp_discovery_float32_session_inlet(
        discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        sample_limits,
        session_limits,
        channel_count,
        record_count,
    )?
    .connect(session_cancelled)
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
    connect_selected_typed_udp_discovery_float32_session_inlet(
        discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        sample_limits,
        session_limits,
        channel_count,
        record_count,
        session_cancelled,
    )?
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
        document_with_contract(
            address,
            port,
            "float32",
            "2",
            "11111111-2222-4333-8444-555555555555",
            "host",
            "source",
            "session",
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn document_with_contract(
        address: &str,
        port: u16,
        format: &str,
        channel_count: &str,
        uid: &str,
        hostname: &str,
        source_id: &str,
        session_id: &str,
    ) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", channel_count.to_owned()),
            ("channel_format", format.to_owned()),
            ("source_id", source_id.to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", uid.to_owned()),
            ("session_id", session_id.to_owned()),
            ("hostname", hostname.to_owned()),
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

    fn resolve<'a>(
        discovery: &'a TypedUdpDiscoveryRun,
        expected_identity: &'a StreamHandshakeIdentity,
        channel_count: usize,
    ) -> Result<
        ResolvedTypedUdpDiscoveryFloat32Session<'a>,
        TypedUdpDiscoveryFloat32SessionConnectionError,
    > {
        resolve_selected_typed_udp_discovery_float32_session_inlet(
            discovery,
            0,
            session_activation(),
            expected_identity,
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
            channel_count,
            2,
        )
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
                    Duration::from_secs(30),
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
    fn selected_resolution_p26_is_socket_free_and_retains_selection_borrows() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port()));
        let expected_identity = identity("11111111-2222-4333-8444-555555555555");
        let resolved = resolve(&discovery, &expected_identity, 2).unwrap();
        assert!(std::ptr::eq(resolved.discovery(), &discovery));
        assert_eq!(resolved.response_index(), 0);
        drop(resolved);
        drop(listener);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn selected_resolution_p26_contract_mismatches_return_owned_evidence_in_order() {
        let endpoint: std::net::SocketAddr = "127.0.0.1:43002".parse().unwrap();
        let expected = identity("11111111-2222-4333-8444-555555555555");
        let cases = [
            (
                document_with_contract(
                    "127.0.0.1",
                    endpoint.port(),
                    "double64",
                    "3",
                    "uid-x",
                    "host-x",
                    "source-x",
                    "session-x",
                ),
                TypedUdpDiscoveryFloat32SessionConnectionError::Format {
                    expected: ChannelFormat::Float32,
                    actual: ChannelFormat::Double64,
                },
            ),
            (
                document_with_contract(
                    "127.0.0.1",
                    endpoint.port(),
                    "float32",
                    "3",
                    "uid-x",
                    "host-x",
                    "source-x",
                    "session-x",
                ),
                TypedUdpDiscoveryFloat32SessionConnectionError::ChannelCount {
                    expected: 2,
                    actual: 3,
                },
            ),
        ];
        for (document, expected_error) in cases {
            let discovery = completed_discovery(document);
            assert_eq!(
                resolve(&discovery, &expected, 2).err().unwrap(),
                expected_error
            );
        }

        for (uid, hostname, source_id, session_id, role, expected_value, actual_value) in [
            (
                "uid-x",
                "host-x",
                "source-x",
                "session-x",
                StreamHandshakeIdentityRole::Uid,
                "11111111-2222-4333-8444-555555555555",
                "uid-x",
            ),
            (
                "11111111-2222-4333-8444-555555555555",
                "host-x",
                "source-x",
                "session-x",
                StreamHandshakeIdentityRole::Hostname,
                "host",
                "host-x",
            ),
            (
                "11111111-2222-4333-8444-555555555555",
                "host",
                "source-x",
                "session-x",
                StreamHandshakeIdentityRole::SourceId,
                "source",
                "source-x",
            ),
            (
                "11111111-2222-4333-8444-555555555555",
                "host",
                "source",
                "session-x",
                StreamHandshakeIdentityRole::SessionId,
                "session",
                "session-x",
            ),
        ] {
            let discovery = completed_discovery(document_with_contract(
                "127.0.0.1",
                endpoint.port(),
                "float32",
                "2",
                uid,
                hostname,
                source_id,
                session_id,
            ));
            assert_eq!(
                resolve(&discovery, &expected, 2).err().unwrap(),
                TypedUdpDiscoveryFloat32SessionConnectionError::Identity {
                    role,
                    expected: expected_value.to_owned(),
                    actual: actual_value.to_owned(),
                }
            );
        }
    }

    #[test]
    fn selected_resolution_p26_endpoint_contract_preflight_precedence_is_socket_free() {
        let expected = identity("11111111-2222-4333-8444-555555555555");
        let valid = completed_discovery(document("127.0.0.1", 9));
        assert!(matches!(
            resolve_selected_typed_udp_discovery_float32_session_inlet(
                &valid,
                1,
                session_activation(),
                &expected,
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                3,
                2,
            ),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(
                TypedUdpDiscoveryEndpointError::ResponseUnavailable { .. }
            ))
        ));
        let invalid_endpoint = completed_discovery(document_with_contract(
            "0.0.0.0",
            9,
            "double64",
            "3",
            "uid-x",
            "host-x",
            "source-x",
            "session-x",
        ));
        assert!(matches!(
            resolve(&invalid_endpoint, &expected, 3),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(_))
        ));
        let wrong_contract = completed_discovery(document_with_contract(
            "127.0.0.1",
            9,
            "double64",
            "3",
            "uid-x",
            "host-x",
            "source-x",
            "session-x",
        ));
        assert!(matches!(
            resolve(&wrong_contract, &expected, 3),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Format { .. })
        ));
        let preflight = completed_discovery(document_with_contract(
            "127.0.0.1",
            9,
            "float32",
            "3",
            "11111111-2222-4333-8444-555555555555",
            "host",
            "source",
            "session",
        ));
        assert!(matches!(
            resolve(&preflight, &expected, 3),
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Preflight(
                TimestampedFloat32SessionPreflightError::ChannelCount { .. }
            ))
        ));
    }

    #[test]
    fn p18_selected_response_connects_phased_session_and_preserves_caller_discovery() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            let outlet_identity = identity("11111111-2222-4333-8444-555555555555");
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &outlet_identity,
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
        let mut connected = connect_selected_typed_udp_discovery_float32_session_inlet(
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
        assert_eq!(connected.peer(), endpoint);
        assert_eq!(connected.channel_count(), 2);
        assert_eq!(connected.record_count(), 2);
        assert_eq!(connected.completed_record_count(), 0);
        assert!(connected.received_records().is_empty());
        connected.transfer_next(&AtomicBool::new(false)).unwrap();
        assert_eq!(connected.completed_record_count(), 1);
        assert_eq!(
            connected.received_records()[0].sample().values(),
            &[1.5, -2.5]
        );
        connected.transfer_next(&AtomicBool::new(false)).unwrap();
        let report = connected
            .complete(&AtomicBool::new(false))
            .unwrap()
            .unwrap();
        assert_eq!(report.peer(), endpoint);
        assert_eq!(report.records()[1].sample().values(), &[3.25, -4.75]);
        assert_eq!(outlet.join().unwrap().record_count(), 2);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p18_connected_owner_can_close_without_report_and_releases_the_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            let outlet_identity = identity("11111111-2222-4333-8444-555555555555");
            let records = [record(11.25, [1.5, -2.5]), record(12.5, [3.25, -4.75])];
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &outlet_identity,
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
                &records,
            )
            .unwrap()
            .accept(&AtomicBool::new(false))
            .unwrap()
            .close()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port()));
        let connected = connect_selected_typed_udp_discovery_float32_session_inlet(
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
        connected.close();
        outlet.join().unwrap();
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p18_legacy_whole_session_entrypoint_delegates_and_preserves_report() {
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
            Err(
                TypedUdpDiscoveryFloat32SessionConnectionError::ChannelCount {
                    expected: 3,
                    actual: 2
                }
            )
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

        let discovery = completed_discovery(document("127.0.0.1", 9));
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
            Err(TypedUdpDiscoveryFloat32SessionConnectionError::Identity {
                role: StreamHandshakeIdentityRole::Uid,
                expected,
                actual,
            }) if expected == "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"
                && actual == "11111111-2222-4333-8444-555555555555"
        ));
    }
}
