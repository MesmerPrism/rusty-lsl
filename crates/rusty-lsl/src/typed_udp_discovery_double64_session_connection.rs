// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected typed-discovery response to bounded Double64 inlet-session composition.

use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, FixedWidthNumericSampleActivation,
    StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedDouble64ConnectedInletSession,
    TimestampedDouble64InletSession, TimestampedDouble64SessionError,
    TimestampedDouble64SessionIoLimits, TimestampedDouble64SessionLimits,
    TimestampedDouble64SessionPreflightError, TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Projects one caller-selected completed discovery response and connects one bounded inlet.
///
/// The caller retains the completed discovery run, receive-order selection, expected identity,
/// limits, cancellation, and activation. The strict endpoint projector runs before the existing
/// Double64 preflight and connect owners. The returned concrete connected owner retains phased
/// transfer, canonical completion, allocation ownership, and report-free close. This adapter owns
/// no discovery, ranking, retry, identity derivation, codec, cursor, lifecycle, socket, or report.
#[allow(clippy::too_many_arguments)]
pub fn connect_selected_typed_udp_discovery_double64_session_inlet(
    discovery: &TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: FixedWidthNumericSampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: TimestampedDouble64SessionIoLimits,
    session_limits: TimestampedDouble64SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<
    Result<
        Result<TimestampedDouble64ConnectedInletSession, TimestampedDouble64SessionError>,
        TimestampedDouble64SessionPreflightError,
    >,
    TypedUdpDiscoveryEndpointError,
> {
    let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(discovery, response_index)?;
    let session = match TimestampedDouble64InletSession::preflight_bounded(
        session_activation,
        endpoint.into(),
        expected_identity,
        handshake_limits,
        io_limits,
        session_limits,
        channel_count,
        record_count,
    ) {
        Ok(session) => session,
        Err(error) => return Ok(Err(error)),
    };
    Ok(Ok(session.connect(session_cancelled)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamInfoObservedAdmissionLimits, StreamInfoVolatileFieldLimits,
        TimestampedDouble64OutletSession, TimestampedSample, UdpDiscoveryActivation,
        UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn io_limits() -> TimestampedDouble64SessionIoLimits {
        TimestampedDouble64SessionIoLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
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

    fn document(address: &str, port: u16, channels: usize) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", channels.to_string()),
            ("channel_format", "double64".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "77777777-2222-4333-8444-555555555555".to_owned()),
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

    fn records(channels: usize, count: usize) -> Vec<TimestampedSample<f64>> {
        (0..count)
            .map(|record| {
                let values = (0..channels)
                    .map(|channel| {
                        f64::from_bits(0x3ff0_0000_0000_0000 + (record * 4 + channel) as u64)
                    })
                    .collect();
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap(),
                    RawSourceTimestamp::new(10.0 + record as f64).unwrap(),
                    None,
                )
            })
            .collect()
    }

    fn assert_phased_shape(channels: usize, count: usize) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let sent = records(channels, count);
        let expected_bits: Vec<Vec<u64>> = sent
            .iter()
            .map(|record| {
                record
                    .sample()
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect()
            })
            .collect();
        let outlet = thread::spawn(move || {
            TimestampedDouble64OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(channels, count).unwrap(),
                &sent,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port(), channels));
        let mut connected = connect_selected_typed_udp_discovery_double64_session_inlet(
            &discovery,
            0,
            session_activation(),
            &identity(),
            handshake_limits(),
            io_limits(),
            TimestampedDouble64SessionLimits::new(channels, count).unwrap(),
            channels,
            count,
            &AtomicBool::new(false),
        )
        .unwrap()
        .unwrap()
        .unwrap();
        assert_eq!(discovery.responses().len(), 1);
        assert_eq!(connected.peer(), endpoint);
        for completed in 1..=count {
            connected.transfer_next(&AtomicBool::new(false)).unwrap();
            assert_eq!(connected.completed_record_count(), completed);
        }
        let report = connected
            .complete(&AtomicBool::new(false))
            .unwrap()
            .unwrap();
        let actual_bits: Vec<Vec<u64>> = report
            .records()
            .iter()
            .map(|record| {
                record
                    .sample()
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect()
            })
            .collect();
        assert_eq!(actual_bits, expected_bits);
        assert_eq!(outlet.join().unwrap().record_count(), count);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p20_selected_response_enters_phased_double64_for_only_accepted_shapes() {
        assert_phased_shape(1, 1);
        assert_phased_shape(2, 3);
    }

    #[test]
    fn p20_selection_and_shape_rejections_precede_tcp_io() {
        let discovery = completed_discovery(document("127.0.0.1", 9, 2));
        assert!(matches!(
            connect_selected_typed_udp_discovery_double64_session_inlet(
                &discovery,
                1,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
                2,
                3,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                index: 1,
                response_count: 1
            })
        ));
        assert!(matches!(
            connect_selected_typed_udp_discovery_double64_session_inlet(
                &discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(3, 3).unwrap(),
                3,
                3,
                &AtomicBool::new(false),
            ),
            Ok(Err(
                TimestampedDouble64SessionPreflightError::ChannelCount {
                    index: 0,
                    actual: 3
                }
            ))
        ));
        assert_eq!(discovery.responses().len(), 1);
    }
}
