// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit composition from one accepted discovery response to the finite inlet handshake.

use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, run_stream_inlet_handshake,
    StreamHandshakeActivation, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeLimits, StreamInletHandshake, TypedUdpDiscoveryEndpointError,
    TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Stable owner-preserving failure from endpoint projection or handshake execution.
#[derive(Debug, Eq, PartialEq)]
pub enum TypedUdpDiscoveryHandshakeError {
    /// The caller-selected response could not produce a strict service endpoint.
    Endpoint(TypedUdpDiscoveryEndpointError),
    /// The existing finite inlet handshake rejected or failed.
    Handshake(StreamHandshakeError),
}

/// Projects one caller-selected response and runs the separately activated inlet handshake.
///
/// The caller retains response selection, identity, limits, cancellation, and activation
/// ownership. This performs no discovery, fallback, retry, admission, or sample transport.
pub fn run_selected_typed_udp_discovery_inlet_handshake(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
    activation: StreamHandshakeActivation,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
) -> Result<StreamInletHandshake, TypedUdpDiscoveryHandshakeError> {
    let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(run, response_index)
        .map_err(TypedUdpDiscoveryHandshakeError::Endpoint)?;
    run_stream_inlet_handshake(activation, endpoint.into(), identity, limits, cancelled)
        .map_err(TypedUdpDiscoveryHandshakeError::Handshake)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_stream_outlet_handshake, run_typed_udp_discovery, MetadataTreeLimits, RuntimeModule,
        ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamInfoObservedAdmissionLimits,
        StreamInfoVolatileFieldLimits, UdpDiscoveryActivation, UdpDiscoveryConfig,
        UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    fn limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            limits(),
        )
        .unwrap()
    }

    fn typed_run(service_port: u16) -> TypedUdpDiscoveryRun {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = peer.local_addr().unwrap();
        let roles = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", "1".to_owned()),
            ("channel_format", "float32".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "11111111-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "session".to_owned()),
            ("hostname", "host".to_owned()),
            ("v4address", "127.0.0.1".to_owned()),
            ("v4data_port", "43001".to_owned()),
            ("v4service_port", service_port.to_string()),
            ("v6address", "2001:db8::10".to_owned()),
            ("v6data_port", "43003".to_owned()),
            ("v6service_port", "43004".to_owned()),
        ];
        let mut document = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in roles {
            document.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        document.push_str("\t<desc />\n</info>\n");
        let response = format!("19\r\n{document}").into_bytes();
        let document_bytes = document.len();
        let worker = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = peer.recv_from(&mut query).unwrap();
            peer.send_to(&response, source).unwrap();
        });
        let query_limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, query_limits).unwrap(),
            query_limits,
        )
        .unwrap();
        let envelope_limits =
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap();
        let run = run_typed_udp_discovery(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    document_bytes + 32,
                    1,
                    Duration::from_millis(10),
                    Duration::from_secs(1),
                )
                .unwrap(),
                envelope_limits,
            ),
            &query,
            &AtomicBool::new(false),
            envelope_limits,
            StreamInfoObservedAdmissionLimits::new(
                StreamDescriptorLimits::new(64, 64, 64, 4).unwrap(),
                MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
                StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
            ),
        )
        .unwrap();
        worker.join().unwrap();
        run
    }

    #[test]
    fn lslc_004y_composes_selected_endpoint_with_separate_handshake_activation() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let cancelled = Arc::new(AtomicBool::new(false));
        let outlet_cancelled = Arc::clone(&cancelled);
        let worker = thread::spawn(move || {
            run_stream_outlet_handshake(
                StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                    .unwrap(),
                listener,
                &identity(),
                limits(),
                &outlet_cancelled,
            )
            .unwrap()
        });
        let result = run_selected_typed_udp_discovery_inlet_handshake(
            &typed_run(port),
            0,
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
            &identity(),
            limits(),
            &cancelled,
        )
        .unwrap();
        assert_eq!(result.peer().port(), port);
        worker.join().unwrap();
    }

    #[test]
    fn lslc_004y_rejects_projection_before_handshake_io() {
        let run = typed_run(9);
        assert_eq!(
            run_selected_typed_udp_discovery_inlet_handshake(
                &run,
                1,
                StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                    .unwrap(),
                &identity(),
                limits(),
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryHandshakeError::Endpoint(
                TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                    index: 1,
                    response_count: 1,
                },
            ))
        );
    }
}
