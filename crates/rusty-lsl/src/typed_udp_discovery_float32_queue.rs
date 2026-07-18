// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit composition from one selected discovery response through one Float32 record into a queue.

use crate::{
    run_selected_typed_udp_discovery_float32_inlet, BoundedSampleQueue,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    TypedUdpDiscoveryFloat32Error, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Stable owner-preserving failure from inlet execution or queue backpressure.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32QueueError {
    /// The existing selected-response Float32 inlet rejected or failed.
    Inlet(TypedUdpDiscoveryFloat32Error),
    /// The separately activated caller-owned queue rejected the unchanged record.
    Queue(BoundedSampleQueuePushError),
}

/// Receives one selected-response Float32 record and passes it to an existing queue.
///
/// The queue is constructed and activated separately. The caller independently owns
/// inlet and queue cancellation, wait bounds, response selection, identity, and limits.
/// Queue rejection retains the unchanged record through the existing queue error.
pub fn run_selected_typed_udp_discovery_float32_inlet_into_queue(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
    activation: TimestampedFloat32SampleActivation,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    inlet_cancelled: &AtomicBool,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    queue_cancelled: &AtomicBool,
) -> Result<(), TypedUdpDiscoveryFloat32QueueError> {
    let sample = run_selected_typed_udp_discovery_float32_inlet(
        run,
        response_index,
        activation,
        identity,
        handshake_limits,
        sample_limits,
        inlet_cancelled,
    )
    .map_err(TypedUdpDiscoveryFloat32QueueError::Inlet)?;
    queue
        .push(sample, queue_wait, queue_cancelled)
        .map_err(TypedUdpDiscoveryFloat32QueueError::Queue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_timestamped_float32_outlet, run_typed_udp_discovery, BoundedSampleQueueActivation,
        MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample, SampleLimits,
        ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
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

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn queue_wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(100)).unwrap()
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

    fn sample_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn queue() -> BoundedSampleQueue {
        BoundedSampleQueue::new(
            BoundedSampleQueueActivation::new(
                test_capability(RuntimeModule::BoundedSampleQueue),
                sample_activation(),
            )
            .unwrap(),
            1,
        )
        .unwrap()
    }

    fn sample(timestamp: f64, value: f32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            None,
        )
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

    fn outlet(listener: TcpListener, sent: TimestampedSample<f32>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            run_timestamped_float32_outlet(
                sample_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sent,
                &AtomicBool::new(false),
            )
            .unwrap();
        })
    }

    #[test]
    fn lslc_005a_preserves_raw_timestamp_and_value_bits_through_queue() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let worker = outlet(listener, sample(-0.0, f32::from_bits(0x7fc0_4321)));
        let queue = queue();
        run_selected_typed_udp_discovery_float32_inlet_into_queue(
            &typed_run(port),
            0,
            sample_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
            &queue,
            queue_wait(),
            &AtomicBool::new(false),
        )
        .unwrap();
        let drained = queue.try_pop().unwrap();
        assert_eq!(
            drained.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(drained.sample().values()[0].to_bits(), 0x7fc0_4321);
        worker.join().unwrap();
    }

    #[test]
    fn lslc_005a_queue_cancellation_retains_received_record() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let worker = outlet(listener, sample(1234.5, 7.25));
        let queue = queue();
        let queue_cancelled = AtomicBool::new(true);
        let error = run_selected_typed_udp_discovery_float32_inlet_into_queue(
            &typed_run(port),
            0,
            sample_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
            &queue,
            queue_wait(),
            &queue_cancelled,
        )
        .unwrap_err();
        let rejected = match error {
            TypedUdpDiscoveryFloat32QueueError::Queue(BoundedSampleQueuePushError::Cancelled(
                sample,
            )) => sample,
            other => panic!("expected queue cancellation, got {other:?}"),
        };
        assert_eq!(
            rejected.raw_source_timestamp().value().to_bits(),
            1234.5f64.to_bits()
        );
        assert_eq!(rejected.sample().values()[0].to_bits(), 7.25f32.to_bits());
        worker.join().unwrap();
    }
}
