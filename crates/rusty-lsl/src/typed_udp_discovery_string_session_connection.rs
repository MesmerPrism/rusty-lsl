// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected typed-discovery response to the bounded String inlet session.

use crate::typed_udp_discovery_session_contract::{
    validate_selected_typed_udp_discovery_session_contract,
    TypedUdpDiscoverySessionContractMismatch,
};
use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, ChannelFormat, StreamHandshakeIdentity,
    StreamHandshakeIdentityRole, StreamHandshakeLimits, StringSampleActivation, StringSampleLimits,
    TimestampedStringConnectedInletSession, TimestampedStringInletSession,
    TimestampedStringInletSessionReport, TimestampedStringSessionError,
    TimestampedStringSessionLimits, TimestampedStringSessionPreflightError,
    TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Failure from the caller-selected discovery-to-String session composition.
#[derive(Debug, Eq, PartialEq)]
pub enum TypedUdpDiscoveryStringSessionConnectionError {
    /// Strict projection of the caller-selected response failed.
    Endpoint(TypedUdpDiscoveryEndpointError),
    /// The selected response advertises a different sample format.
    FormatMismatch {
        /// Format required by the concrete adapter.
        expected: ChannelFormat,
        /// Format advertised by the selected response.
        actual: ChannelFormat,
    },
    /// The selected response advertises a different channel count.
    ChannelCountMismatch {
        /// Channel count requested by the caller.
        expected: usize,
        /// Channel count advertised by the selected response.
        actual: usize,
    },
    /// The selected response advertises a different handshake identity field.
    IdentityMismatch {
        /// Identity role whose value differs.
        role: StreamHandshakeIdentityRole,
        /// Caller-owned expected identity value.
        expected: String,
        /// Selected-response identity value.
        actual: String,
    },
    /// The selected endpoint or exact 1x1 shape failed socket-free preflight.
    Preflight(TimestampedStringSessionPreflightError),
    /// Connect, transfer, terminal close, or cleanup failed.
    Session(TimestampedStringSessionError),
}

fn contract_error(
    mismatch: TypedUdpDiscoverySessionContractMismatch<'_>,
) -> TypedUdpDiscoveryStringSessionConnectionError {
    match mismatch {
        TypedUdpDiscoverySessionContractMismatch::Format { expected, actual } => {
            TypedUdpDiscoveryStringSessionConnectionError::FormatMismatch { expected, actual }
        }
        TypedUdpDiscoverySessionContractMismatch::ChannelCount { expected, actual } => {
            TypedUdpDiscoveryStringSessionConnectionError::ChannelCountMismatch { expected, actual }
        }
        TypedUdpDiscoverySessionContractMismatch::Identity {
            role,
            expected,
            actual,
        } => TypedUdpDiscoveryStringSessionConnectionError::IdentityMismatch {
            role,
            expected: expected.to_owned(),
            actual: actual.to_owned(),
        },
    }
}

/// Projects one caller-selected completed discovery response and connects the 1x1 String inlet.
///
/// The caller retains discovery execution, receive-order selection, expected identity, limits,
/// cancellation, and activation. Strict endpoint projection precedes selected-response contract
/// validation, which precedes the existing socket-free String preflight and connect owners. The
/// returned concrete owner retains phased transfer,
/// the exact 0..=129 UTF-8-byte codec, allocation ownership, damage and trailing classification,
/// canonical completion, and report-free close.
#[allow(clippy::too_many_arguments)]
pub fn connect_selected_typed_udp_discovery_string_session_inlet(
    discovery: &TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: StringSampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: StringSampleLimits,
    session_limits: TimestampedStringSessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<TimestampedStringConnectedInletSession, TypedUdpDiscoveryStringSessionConnectionError> {
    let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(discovery, response_index)
        .map_err(TypedUdpDiscoveryStringSessionConnectionError::Endpoint)?;
    validate_selected_typed_udp_discovery_session_contract(
        &discovery.responses()[response_index],
        ChannelFormat::String,
        channel_count,
        expected_identity,
    )
    .map_err(contract_error)?;
    let session = TimestampedStringInletSession::preflight_bounded(
        session_activation,
        endpoint.into(),
        expected_identity,
        handshake_limits,
        io_limits,
        session_limits,
        channel_count,
        record_count,
    )
    .map_err(TypedUdpDiscoveryStringSessionConnectionError::Preflight)?;
    session
        .connect(session_cancelled)
        .map_err(TypedUdpDiscoveryStringSessionConnectionError::Session)
}

/// Runs the selected exact 1x1 String inlet to its canonical completion report.
#[allow(clippy::too_many_arguments)]
pub fn run_selected_typed_udp_discovery_string_session_inlet(
    discovery: &TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: StringSampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: StringSampleLimits,
    session_limits: TimestampedStringSessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<TimestampedStringInletSessionReport, TypedUdpDiscoveryStringSessionConnectionError> {
    connect_selected_typed_udp_discovery_string_session_inlet(
        discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        io_limits,
        session_limits,
        channel_count,
        record_count,
        session_cancelled,
    )?
    .finish(session_cancelled)
    .map_err(TypedUdpDiscoveryStringSessionConnectionError::Session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, MetadataTreeLimits, RuntimeModule, ShortInfoQuery,
        ShortInfoQueryWire, ShortInfoQueryWireLimits, ShortInfoResponseEnvelopeLimits,
        StreamDescriptorLimits, StreamHandshakeActivation, StreamInfoObservedAdmissionLimits,
        StreamInfoVolatileFieldLimits, StringSampleError, StringSampleRecord,
        TimestampedStringOutletSession, UdpDiscoveryActivation, UdpDiscoveryConfig,
        UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn io_limits() -> StringSampleLimits {
        StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "77777777-2222-4333-8444-555555555555".into(),
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

    fn session_activation() -> StringSampleActivation {
        StringSampleActivation::new(
            test_capability(RuntimeModule::StringSample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 20, limits).unwrap(),
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

    fn document(port: u16) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", "1".to_owned()),
            ("channel_format", "string".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "77777777-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "session".to_owned()),
            ("hostname", "host".to_owned()),
            ("v4address", "127.0.0.1".to_owned()),
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
                .send_to(format!("20\r\n{document}").as_bytes(), source)
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

    #[test]
    fn p24_selected_response_preserves_string_boundaries_allocation_and_caller_run() {
        for value in [String::new(), "x".repeat(129), "μ".repeat(64) + "a"] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let endpoint = listener.local_addr().unwrap();
            let expected = value.clone();
            let outlet = thread::spawn(move || {
                let records = [StringSampleRecord::new(17.25, value).unwrap()];
                TimestampedStringOutletSession::preflight_bounded(
                    session_activation(),
                    listener,
                    &identity(),
                    handshake_limits(),
                    io_limits(),
                    TimestampedStringSessionLimits::new(1, 1).unwrap(),
                    &records,
                )
                .unwrap()
                .finish(&AtomicBool::new(false))
                .unwrap()
            });
            let discovery = completed_discovery(document(endpoint.port()));
            let mut connected = connect_selected_typed_udp_discovery_string_session_inlet(
                &discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedStringSessionLimits::new(1, 1).unwrap(),
                1,
                1,
                &AtomicBool::new(false),
            )
            .unwrap();
            connected.transfer_next(&AtomicBool::new(false)).unwrap();
            let allocation = connected.received_records()[0].value().as_ptr();
            let report = connected
                .complete(&AtomicBool::new(false))
                .unwrap()
                .unwrap();
            assert_eq!(report.records()[0].value(), expected);
            assert_eq!(report.records()[0].value().as_ptr(), allocation);
            assert_eq!(discovery.responses().len(), 1);
            outlet.join().unwrap();
            TcpListener::bind(endpoint).unwrap();
        }
    }

    #[test]
    fn p24_endpoint_then_shape_preflight_precede_session_io() {
        let discovery = completed_discovery(document(9));
        let invalid_shape_discovery = completed_discovery(document(9).replace(
            "<channel_count>1</channel_count>",
            "<channel_count>2</channel_count>",
        ));
        assert!(matches!(
            connect_selected_typed_udp_discovery_string_session_inlet(
                &discovery,
                1,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedStringSessionLimits::new(1, 1).unwrap(),
                2,
                2,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryStringSessionConnectionError::Endpoint(_))
        ));
        assert!(matches!(
            connect_selected_typed_udp_discovery_string_session_inlet(
                &invalid_shape_discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedStringSessionLimits::new(1, 1).unwrap(),
                2,
                1,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryStringSessionConnectionError::Preflight(
                TimestampedStringSessionPreflightError::ChannelCount { actual: 2 }
            ))
        ));
    }

    #[test]
    fn p24_adapter_error_retains_indexed_damage_and_trailing_classifications() {
        let damage = TypedUdpDiscoveryStringSessionConnectionError::Session(
            TimestampedStringSessionError::Record {
                index: Some(0),
                error: StringSampleError::InvalidUtf8,
            },
        );
        assert!(matches!(
            damage,
            TypedUdpDiscoveryStringSessionConnectionError::Session(
                TimestampedStringSessionError::Record {
                    index: Some(0),
                    error: StringSampleError::InvalidUtf8
                }
            )
        ));
        let trailing = TypedUdpDiscoveryStringSessionConnectionError::Session(
            TimestampedStringSessionError::TrailingByte { actual: 0xa5 },
        );
        assert!(matches!(
            trailing,
            TypedUdpDiscoveryStringSessionConnectionError::Session(
                TimestampedStringSessionError::TrailingByte { actual: 0xa5 }
            )
        ));
    }

    fn contract_failure(document: String) -> TypedUdpDiscoveryStringSessionConnectionError {
        let discovery = completed_discovery(document);
        match connect_selected_typed_udp_discovery_string_session_inlet(
            &discovery,
            0,
            session_activation(),
            &identity(),
            handshake_limits(),
            io_limits(),
            TimestampedStringSessionLimits::new(1, 1).unwrap(),
            1,
            1,
            &AtomicBool::new(false),
        ) {
            Err(error) => error,
            Ok(_) => panic!("selected contract unexpectedly connected"),
        }
    }

    #[test]
    fn p24_string_adapter_rejects_owned_contract_mismatch_before_preflight_and_tcp() {
        assert_eq!(
            contract_failure(document(9).replace("string", "float32")),
            TypedUdpDiscoveryStringSessionConnectionError::FormatMismatch {
                expected: ChannelFormat::String,
                actual: ChannelFormat::Float32,
            }
        );
        assert_eq!(
            contract_failure(document(9).replace(
                "<channel_count>1</channel_count>",
                "<channel_count>2</channel_count>",
            )),
            TypedUdpDiscoveryStringSessionConnectionError::ChannelCountMismatch {
                expected: 1,
                actual: 2,
            }
        );
        assert_eq!(
            contract_failure(
                document(9).replace("<hostname>host</hostname>", "<hostname>host-x</hostname>",)
            ),
            TypedUdpDiscoveryStringSessionConnectionError::IdentityMismatch {
                role: StreamHandshakeIdentityRole::Hostname,
                expected: "host".to_owned(),
                actual: "host-x".to_owned(),
            }
        );
    }
}
