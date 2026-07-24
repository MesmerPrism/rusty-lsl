// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit composition from one accepted discovery response to one finite Float32 record.

use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, run_timestamped_float32_inlet,
    StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleError, TimestampedFloat32SampleLimits, TimestampedSample,
    TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Stable owner-preserving failure from endpoint projection or Float32 inlet execution.
#[derive(Debug, PartialEq)]
pub enum TypedUdpDiscoveryFloat32Error {
    /// The caller-selected response could not produce a strict service endpoint.
    Endpoint(TypedUdpDiscoveryEndpointError),
    /// The existing finite one-record Float32 inlet rejected or failed.
    Sample(TimestampedFloat32SampleError),
}

/// Projects one caller-selected response and receives one timestamped Float32 record.
///
/// The caller retains response selection, identity, limits, cancellation, and activation
/// ownership. This performs no discovery, fallback, retry, recovery, or retained connection.
pub fn run_selected_typed_udp_discovery_float32_inlet(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
    activation: TimestampedFloat32SampleActivation,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<TimestampedSample<f32>, TypedUdpDiscoveryFloat32Error> {
    let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(run, response_index)
        .map_err(TypedUdpDiscoveryFloat32Error::Endpoint)?;
    run_timestamped_float32_inlet(
        activation,
        endpoint.into(),
        identity,
        handshake_limits,
        sample_limits,
        cancelled,
    )
    .map_err(TypedUdpDiscoveryFloat32Error::Sample)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_timestamped_float32_outlet, run_typed_udp_discovery, MetadataTreeLimits,
        RawSourceTimestamp, RuntimeModule, Sample, SampleLimits, ShortInfoQuery,
        ShortInfoQueryWire, ShortInfoQueryWireLimits, ShortInfoResponseEnvelopeLimits,
        StreamDescriptorLimits, StreamHandshakeActivation, StreamInfoObservedAdmissionLimits,
        StreamInfoVolatileFieldLimits, UdpDiscoveryActivation, UdpDiscoveryConfig,
        UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::Arc;
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
    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            handshake_limits(),
        )
        .unwrap()
    }
    fn activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
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
    fn lslc_004z_composes_selected_discovery_with_one_float32_record() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let cancelled = Arc::new(AtomicBool::new(false));
        let outlet_cancelled = Arc::clone(&cancelled);
        let expected = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![7.25]).unwrap(),
            RawSourceTimestamp::new(1234.5).unwrap(),
            None,
        );
        let worker_sample = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![7.25]).unwrap(),
            RawSourceTimestamp::new(1234.5).unwrap(),
            None,
        );
        let worker = thread::spawn(move || {
            run_timestamped_float32_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &worker_sample,
                &outlet_cancelled,
            )
            .unwrap()
        });
        let received = run_selected_typed_udp_discovery_float32_inlet(
            &typed_run(port),
            0,
            activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            &cancelled,
        )
        .unwrap();
        assert_eq!(received, expected);
        worker.join().unwrap();
    }

    #[test]
    fn lslc_004z_rejects_selection_before_tcp_io() {
        assert!(matches!(
            run_selected_typed_udp_discovery_float32_inlet(
                &typed_run(9),
                1,
                activation(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryFloat32Error::Endpoint(
                TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                    index: 1,
                    response_count: 1
                }
            ))
        ));
    }
}
