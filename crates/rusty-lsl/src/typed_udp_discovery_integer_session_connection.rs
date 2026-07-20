// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected typed-discovery response adapters for bounded integer inlet sessions.

use crate::typed_udp_discovery_session_contract::{
    validate_selected_typed_udp_discovery_session_contract,
    TypedUdpDiscoverySessionContractMismatch,
};
use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, ChannelFormat,
    FixedWidthNumericSampleActivation, StreamHandshakeIdentity, StreamHandshakeIdentityRole,
    StreamHandshakeLimits, TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

macro_rules! integer_discovery_session_adapter {
    (
        $error:ident, $connect:ident, $run:ident, $inlet:ident, $connected:ident,
        $report:ident, $session_error:ident, $io_limits:ident, $limits:ident,
        $preflight_error:ident, $format:ident, $label:literal
    ) => {
        #[doc = concat!("Failure from the caller-selected discovery-to-", $label, " session composition.")]
        #[derive(Debug, Eq, PartialEq)]
        pub enum $error {
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
            /// The selected endpoint or requested shape failed bounded session preflight.
            Preflight(crate::$preflight_error),
            /// Connect, transfer, terminal close, or cleanup failed.
            Session(crate::$session_error),
        }

        #[doc = concat!("Projects one caller-selected completed discovery response and connects one bounded ", $label, " inlet.")]
        ///
        /// The caller retains discovery execution, receive-order selection, expected identity,
        /// limits, cancellation, and activation. Endpoint projection precedes selected-response
        /// contract validation, which precedes the existing concrete session preflight and connect
        /// owners. Only their accepted one-channel,
        /// one-record and two-channel, three-record shapes are admitted.
        #[allow(clippy::too_many_arguments)]
        pub fn $connect(
            discovery: &TypedUdpDiscoveryRun,
            response_index: usize,
            session_activation: FixedWidthNumericSampleActivation,
            expected_identity: &StreamHandshakeIdentity,
            handshake_limits: StreamHandshakeLimits,
            io_limits: crate::$io_limits,
            session_limits: crate::$limits,
            channel_count: usize,
            record_count: usize,
            session_cancelled: &AtomicBool,
        ) -> Result<crate::$connected, $error> {
            let endpoint =
                propose_typed_udp_discovery_ipv4_service_endpoint(discovery, response_index)
                    .map_err($error::Endpoint)?;
            validate_selected_typed_udp_discovery_session_contract(
                &discovery.responses()[response_index],
                ChannelFormat::$format,
                channel_count,
                expected_identity,
            )
            .map_err(|mismatch| match mismatch {
                TypedUdpDiscoverySessionContractMismatch::Format { expected, actual } =>
                    $error::FormatMismatch { expected, actual },
                TypedUdpDiscoverySessionContractMismatch::ChannelCount { expected, actual } =>
                    $error::ChannelCountMismatch { expected, actual },
                TypedUdpDiscoverySessionContractMismatch::Identity { role, expected, actual } =>
                    $error::IdentityMismatch {
                        role,
                        expected: expected.to_owned(),
                        actual: actual.to_owned(),
                    },
            })?;
            let session = crate::$inlet::preflight_bounded(
                session_activation,
                endpoint.into(),
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
            )
            .map_err($error::Preflight)?;
            session.connect(session_cancelled).map_err($error::Session)
        }

        #[doc = concat!("Runs the selected bounded ", $label, " inlet to its canonical completion report.")]
        #[allow(clippy::too_many_arguments)]
        pub fn $run(
            discovery: &TypedUdpDiscoveryRun,
            response_index: usize,
            session_activation: FixedWidthNumericSampleActivation,
            expected_identity: &StreamHandshakeIdentity,
            handshake_limits: StreamHandshakeLimits,
            io_limits: crate::$io_limits,
            session_limits: crate::$limits,
            channel_count: usize,
            record_count: usize,
            session_cancelled: &AtomicBool,
        ) -> Result<crate::$report, $error> {
            $connect(
                discovery, response_index, session_activation, expected_identity,
                handshake_limits, io_limits, session_limits, channel_count, record_count,
                session_cancelled,
            )?
            .finish(session_cancelled)
            .map_err($error::Session)
        }
    };
}

integer_discovery_session_adapter!(
    TypedUdpDiscoveryInt64SessionConnectionError,
    connect_selected_typed_udp_discovery_int64_session_inlet,
    run_selected_typed_udp_discovery_int64_session_inlet,
    TimestampedInt64InletSession,
    TimestampedInt64ConnectedInletSession,
    TimestampedInt64InletSessionReport,
    TimestampedInt64SessionError,
    TimestampedInt64SessionIoLimits,
    TimestampedInt64SessionLimits,
    TimestampedInt64SessionPreflightError,
    Int64,
    "Int64"
);
integer_discovery_session_adapter!(
    TypedUdpDiscoveryInt32SessionConnectionError,
    connect_selected_typed_udp_discovery_int32_session_inlet,
    run_selected_typed_udp_discovery_int32_session_inlet,
    TimestampedInt32InletSession,
    TimestampedInt32ConnectedInletSession,
    TimestampedInt32InletSessionReport,
    TimestampedInt32SessionError,
    TimestampedInt32SessionIoLimits,
    TimestampedInt32SessionLimits,
    TimestampedInt32SessionPreflightError,
    Int32,
    "Int32"
);
integer_discovery_session_adapter!(
    TypedUdpDiscoveryInt16SessionConnectionError,
    connect_selected_typed_udp_discovery_int16_session_inlet,
    run_selected_typed_udp_discovery_int16_session_inlet,
    TimestampedInt16InletSession,
    TimestampedInt16ConnectedInletSession,
    TimestampedInt16InletSessionReport,
    TimestampedInt16SessionError,
    TimestampedInt16SessionIoLimits,
    TimestampedInt16SessionLimits,
    TimestampedInt16SessionPreflightError,
    Int16,
    "Int16"
);
integer_discovery_session_adapter!(
    TypedUdpDiscoveryInt8SessionConnectionError,
    connect_selected_typed_udp_discovery_int8_session_inlet,
    run_selected_typed_udp_discovery_int8_session_inlet,
    TimestampedInt8InletSession,
    TimestampedInt8ConnectedInletSession,
    TimestampedInt8InletSessionReport,
    TimestampedInt8SessionError,
    TimestampedInt8SessionIoLimits,
    TimestampedInt8SessionLimits,
    TimestampedInt8SessionPreflightError,
    Int8,
    "Int8"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamInfoObservedAdmissionLimits, StreamInfoVolatileFieldLimits, TimestampedSample,
        UdpDiscoveryActivation, UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "23232323-2222-4333-8444-555555555555".into(),
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

    fn session_activation() -> FixedWidthNumericSampleActivation {
        FixedWidthNumericSampleActivation::new(
            test_capability(RuntimeModule::FixedWidthNumericSample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 23, limits).unwrap(),
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

    fn document(address: &str, port: u16, channels: usize, format: &str) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", channels.to_string()),
            ("channel_format", format.to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "23232323-2222-4333-8444-555555555555".to_owned()),
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
                .send_to(format!("23\r\n{document}").as_bytes(), source)
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

    macro_rules! assert_integer_shapes {
        ($value:ty, $format:literal, $outlet:ident, $connect:ident, $io:ident, $limits:ident) => {{
            for (channels, count) in [(1, 1), (2, 3)] {
                let listener = TcpListener::bind("127.0.0.1:0").unwrap();
                let endpoint = listener.local_addr().unwrap();
                let sent: Vec<TimestampedSample<$value>> = (0..count)
                    .map(|record| {
                        let values = (0..channels)
                            .map(|channel| (record * 17 + channel + 1) as $value)
                            .collect();
                        TimestampedSample::new(
                            Sample::new(SampleLimits::new(channels).unwrap(), channels, values)
                                .unwrap(),
                            RawSourceTimestamp::new(10.0 + record as f64).unwrap(),
                            None,
                        )
                    })
                    .collect();
                let expected = sent.clone();
                let outlet = thread::spawn(move || {
                    crate::$outlet::preflight_bounded(
                        session_activation(),
                        listener,
                        &identity(),
                        handshake_limits(),
                        crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                        crate::$limits::new(channels, count).unwrap(),
                        &sent,
                    )
                    .unwrap()
                    .finish(&AtomicBool::new(false))
                    .unwrap()
                });
                let discovery =
                    completed_discovery(document("127.0.0.1", endpoint.port(), channels, $format));
                let mut connected = $connect(
                    &discovery,
                    0,
                    session_activation(),
                    &identity(),
                    handshake_limits(),
                    crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                    crate::$limits::new(channels, count).unwrap(),
                    channels,
                    count,
                    &AtomicBool::new(false),
                )
                .unwrap();
                assert_eq!(discovery.responses().len(), 1);
                assert_eq!(connected.peer(), endpoint);
                for completed in 1..=count {
                    connected.transfer_next(&AtomicBool::new(false)).unwrap();
                    assert_eq!(connected.completed_record_count(), completed);
                }
                assert_eq!(
                    connected
                        .complete(&AtomicBool::new(false))
                        .unwrap()
                        .unwrap()
                        .records(),
                    expected.as_slice()
                );
                assert_eq!(outlet.join().unwrap().record_count(), count);
                TcpListener::bind(endpoint).unwrap();
            }
        }};
    }

    macro_rules! assert_integer_run_shapes {
        ($value:ty, $format:literal, $outlet:ident, $run:ident, $io:ident, $limits:ident) => {{
            for (channels, count) in [(1, 1), (2, 3)] {
                let listener = TcpListener::bind("127.0.0.1:0").unwrap();
                let endpoint = listener.local_addr().unwrap();
                let sent: Vec<TimestampedSample<$value>> = (0..count)
                    .map(|record| {
                        let values = (0..channels)
                            .map(|channel| (record * 17 + channel + 1) as $value)
                            .collect();
                        TimestampedSample::new(
                            Sample::new(SampleLimits::new(channels).unwrap(), channels, values)
                                .unwrap(),
                            RawSourceTimestamp::new(20.0 + record as f64).unwrap(),
                            None,
                        )
                    })
                    .collect();
                let expected = sent.clone();
                let outlet = thread::spawn(move || {
                    crate::$outlet::preflight_bounded(
                        session_activation(),
                        listener,
                        &identity(),
                        handshake_limits(),
                        crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                        crate::$limits::new(channels, count).unwrap(),
                        &sent,
                    )
                    .unwrap()
                    .finish(&AtomicBool::new(false))
                    .unwrap()
                });
                let discovery =
                    completed_discovery(document("127.0.0.1", endpoint.port(), channels, $format));
                let report = $run(
                    &discovery,
                    0,
                    session_activation(),
                    &identity(),
                    handshake_limits(),
                    crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                    crate::$limits::new(channels, count).unwrap(),
                    channels,
                    count,
                    &AtomicBool::new(false),
                )
                .unwrap();
                assert_eq!(report.record_count(), count);
                assert_eq!(report.records(), expected.as_slice());
                assert_eq!(outlet.join().unwrap().record_count(), count);
                TcpListener::bind(endpoint).unwrap();
            }
        }};
    }

    #[test]
    fn p23_selected_response_enters_each_concrete_integer_owner_for_accepted_shapes() {
        assert_integer_shapes!(
            i32,
            "int32",
            TimestampedInt32OutletSession,
            connect_selected_typed_udp_discovery_int32_session_inlet,
            TimestampedInt32SessionIoLimits,
            TimestampedInt32SessionLimits
        );
        assert_integer_shapes!(
            i16,
            "int16",
            TimestampedInt16OutletSession,
            connect_selected_typed_udp_discovery_int16_session_inlet,
            TimestampedInt16SessionIoLimits,
            TimestampedInt16SessionLimits
        );
        assert_integer_shapes!(
            i8,
            "int8",
            TimestampedInt8OutletSession,
            connect_selected_typed_udp_discovery_int8_session_inlet,
            TimestampedInt8SessionIoLimits,
            TimestampedInt8SessionLimits
        );
    }

    #[test]
    fn p30r_selected_response_connects_int64_owner_for_both_accepted_shapes() {
        assert_integer_shapes!(
            i64,
            "int64",
            TimestampedInt64OutletSession,
            connect_selected_typed_udp_discovery_int64_session_inlet,
            TimestampedInt64SessionIoLimits,
            TimestampedInt64SessionLimits
        );
    }

    #[test]
    fn p30r_selected_response_run_returns_canonical_int64_report_for_both_accepted_shapes() {
        assert_integer_run_shapes!(
            i64,
            "int64",
            TimestampedInt64OutletSession,
            run_selected_typed_udp_discovery_int64_session_inlet,
            TimestampedInt64SessionIoLimits,
            TimestampedInt64SessionLimits
        );
    }

    #[test]
    fn p23_selection_and_shape_rejections_precede_tcp_io() {
        let discovery = completed_discovery(document("127.0.0.1", 9, 2, "int32"));
        let invalid_shape_discovery = completed_discovery(document("127.0.0.1", 9, 3, "int32"));
        assert!(matches!(
            connect_selected_typed_udp_discovery_int32_session_inlet(
                &discovery,
                1,
                session_activation(),
                &identity(),
                handshake_limits(),
                crate::TimestampedInt32SessionIoLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(1)
                )
                .unwrap(),
                crate::TimestampedInt32SessionLimits::new(2, 3).unwrap(),
                2,
                3,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryInt32SessionConnectionError::Endpoint(
                TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                    index: 1,
                    response_count: 1
                }
            ))
        ));
        assert!(matches!(
            connect_selected_typed_udp_discovery_int32_session_inlet(
                &invalid_shape_discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                crate::TimestampedInt32SessionIoLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(1)
                )
                .unwrap(),
                crate::TimestampedInt32SessionLimits::new(3, 3).unwrap(),
                3,
                3,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryInt32SessionConnectionError::Preflight(
                crate::TimestampedInt32SessionPreflightError::ChannelCount {
                    index: 0,
                    actual: 3
                }
            ))
        ));
        assert_eq!(discovery.responses().len(), 1);
    }

    macro_rules! assert_integer_contract_rejections {
        ($connect:ident, $error:ident, $format:ident, $wire:literal, $io:ident, $limits:ident) => {{
            let wrong_format = completed_discovery(document("127.0.0.1", 9, 1, "double64"));
            assert!(matches!(
                $connect(
                    &wrong_format, 0, session_activation(), &identity(), handshake_limits(),
                    crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                    crate::$limits::new(1, 1).unwrap(), 1, 1, &AtomicBool::new(false),
                ),
                Err($error::FormatMismatch {
                    expected: ChannelFormat::$format,
                    actual: ChannelFormat::Double64,
                })
            ));

            let wrong_channels = completed_discovery(document("127.0.0.1", 9, 2, $wire));
            assert!(matches!(
                $connect(
                    &wrong_channels, 0, session_activation(), &identity(), handshake_limits(),
                    crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                    crate::$limits::new(1, 1).unwrap(), 1, 1, &AtomicBool::new(false),
                ),
                Err($error::ChannelCountMismatch { expected: 1, actual: 2 })
            ));

            let wrong_uid = completed_discovery(
                document("127.0.0.1", 9, 1, $wire)
                    .replace("23232323-2222-4333-8444-555555555555", "uid-x"),
            );
            assert!(matches!(
                $connect(
                    &wrong_uid, 0, session_activation(), &identity(), handshake_limits(),
                    crate::$io::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                    crate::$limits::new(1, 1).unwrap(), 1, 1, &AtomicBool::new(false),
                ),
                Err($error::IdentityMismatch {
                    role: StreamHandshakeIdentityRole::Uid,
                    expected,
                    actual,
                }) if expected == "23232323-2222-4333-8444-555555555555" && actual == "uid-x"
            ));
        }};
    }

    #[test]
    fn selected_resolution_p23_each_integer_rejects_contract_mismatch_before_tcp() {
        assert_integer_contract_rejections!(
            connect_selected_typed_udp_discovery_int32_session_inlet,
            TypedUdpDiscoveryInt32SessionConnectionError,
            Int32,
            "int32",
            TimestampedInt32SessionIoLimits,
            TimestampedInt32SessionLimits
        );
        assert_integer_contract_rejections!(
            connect_selected_typed_udp_discovery_int16_session_inlet,
            TypedUdpDiscoveryInt16SessionConnectionError,
            Int16,
            "int16",
            TimestampedInt16SessionIoLimits,
            TimestampedInt16SessionLimits
        );
        assert_integer_contract_rejections!(
            connect_selected_typed_udp_discovery_int8_session_inlet,
            TypedUdpDiscoveryInt8SessionConnectionError,
            Int8,
            "int8",
            TimestampedInt8SessionIoLimits,
            TimestampedInt8SessionLimits
        );
    }

    #[test]
    fn p30r_int64_rejects_contract_mismatch_before_tcp() {
        assert_integer_contract_rejections!(
            connect_selected_typed_udp_discovery_int64_session_inlet,
            TypedUdpDiscoveryInt64SessionConnectionError,
            Int64,
            "int64",
            TimestampedInt64SessionIoLimits,
            TimestampedInt64SessionLimits
        );
    }

    #[test]
    fn p30r_int64_response_index_and_endpoint_rejections_precede_contract_and_preflight() {
        let wrong_contract = completed_discovery(document("not-an-address", 9, 3, "double64"));
        let invoke = |response_index| {
            connect_selected_typed_udp_discovery_int64_session_inlet(
                &wrong_contract,
                response_index,
                session_activation(),
                &identity(),
                handshake_limits(),
                crate::TimestampedInt64SessionIoLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(1),
                )
                .unwrap(),
                crate::TimestampedInt64SessionLimits::new(1, 1).unwrap(),
                1,
                1,
                &AtomicBool::new(false),
            )
        };

        assert!(matches!(
            invoke(1),
            Err(TypedUdpDiscoveryInt64SessionConnectionError::Endpoint(
                TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                    index: 1,
                    response_count: 1,
                }
            ))
        ));
        assert!(matches!(
            invoke(0),
            Err(TypedUdpDiscoveryInt64SessionConnectionError::Endpoint(
                TypedUdpDiscoveryEndpointError::InvalidAddress
            ))
        ));
    }

    #[test]
    fn p30r_int64_resolution_preserves_identity_field_and_preflight_precedence() {
        let expected = identity();
        let io_limits = || {
            crate::TimestampedInt64SessionIoLimits::new(
                Duration::from_millis(5),
                Duration::from_secs(1),
            )
            .unwrap()
        };
        let session_limits = || crate::TimestampedInt64SessionLimits::new(1, 1).unwrap();
        let assert_identity = |document: String, expected_role| {
            let discovery = completed_discovery(document);
            assert!(matches!(
                connect_selected_typed_udp_discovery_int64_session_inlet(
                    &discovery,
                    0,
                    session_activation(),
                    &expected,
                    handshake_limits(),
                    io_limits(),
                    session_limits(),
                    1,
                    1,
                    &AtomicBool::new(false),
                ),
                Err(TypedUdpDiscoveryInt64SessionConnectionError::IdentityMismatch {
                    role,
                    ..
                }) if role == expected_role
            ));
        };

        assert_identity(
            document("127.0.0.1", 9, 1, "int64")
                .replace("23232323-2222-4333-8444-555555555555", "wrong-uid")
                .replace(
                    "<hostname>host</hostname>",
                    "<hostname>wrong-host</hostname>",
                )
                .replace(
                    "<source_id>source</source_id>",
                    "<source_id>wrong-source</source_id>",
                )
                .replace(
                    "<session_id>session</session_id>",
                    "<session_id>wrong-session</session_id>",
                ),
            StreamHandshakeIdentityRole::Uid,
        );
        assert_identity(
            document("127.0.0.1", 9, 1, "int64")
                .replace(
                    "<hostname>host</hostname>",
                    "<hostname>wrong-host</hostname>",
                )
                .replace(
                    "<source_id>source</source_id>",
                    "<source_id>wrong-source</source_id>",
                )
                .replace(
                    "<session_id>session</session_id>",
                    "<session_id>wrong-session</session_id>",
                ),
            StreamHandshakeIdentityRole::Hostname,
        );
        assert_identity(
            document("127.0.0.1", 9, 1, "int64")
                .replace(
                    "<source_id>source</source_id>",
                    "<source_id>wrong-source</source_id>",
                )
                .replace(
                    "<session_id>session</session_id>",
                    "<session_id>wrong-session</session_id>",
                ),
            StreamHandshakeIdentityRole::SourceId,
        );
        assert_identity(
            document("127.0.0.1", 9, 1, "int64").replace(
                "<session_id>session</session_id>",
                "<session_id>wrong-session</session_id>",
            ),
            StreamHandshakeIdentityRole::SessionId,
        );

        let invalid_shape = completed_discovery(document("127.0.0.1", 9, 3, "int64"));
        assert!(matches!(
            connect_selected_typed_udp_discovery_int64_session_inlet(
                &invalid_shape,
                0,
                session_activation(),
                &expected,
                handshake_limits(),
                io_limits(),
                crate::TimestampedInt64SessionLimits::new(3, 3).unwrap(),
                3,
                3,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryInt64SessionConnectionError::Preflight(
                crate::TimestampedInt64SessionPreflightError::ChannelCount {
                    index: 0,
                    actual: 3,
                }
            ))
        ));
    }
}
