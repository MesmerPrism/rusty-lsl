// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! RLSL-P8 release-candidate contract for representative public production APIs.

use std::net::{TcpListener, UdpSocket};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rusty_lsl::*;

const CONSUMER: &str = "rlsl-p8-stable-public-api-candidate-v1";

fn selections() -> Vec<RuntimeActivationSelection<'static>> {
    [
        RuntimeModule::StreamHandshake,
        RuntimeModule::TimestampedFloat32Sample,
        RuntimeModule::BoundedSampleQueue,
        RuntimeModule::FiniteSampleRecovery,
        RuntimeModule::IntegratedClockCorrection,
        RuntimeModule::UdpDiscovery,
    ]
    .into_iter()
    .map(|module| RuntimeActivationSelection::new(module.id(), module.effective_marker()))
    .collect()
}

fn sample_activation(admission: &RuntimeActivationAdmission) -> TimestampedFloat32SampleActivation {
    TimestampedFloat32SampleActivation::new(
        admission
            .capability(RuntimeModule::TimestampedFloat32Sample)
            .unwrap(),
        StreamHandshakeActivation::new(
            admission
                .capability(RuntimeModule::StreamHandshake)
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn identity(limits: StreamHandshakeLimits) -> StreamHandshakeIdentity {
    StreamHandshakeIdentity::new(
        "88888888-2222-4888-8888-888888888888".into(),
        "p8-loopback-host".into(),
        "p8-explicit-source".into(),
        "p8-host-session".into(),
        limits,
    )
    .unwrap()
}

struct CandidateClock([f64; 2], usize);

impl ClockSource for CandidateClock {
    fn now(&mut self) -> f64 {
        let value = self.0[self.1];
        self.1 += 1;
        value
    }
}

#[test]
fn representative_public_production_lifecycle_is_release_candidate_ready() {
    let admission =
        admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, CONSUMER, &selections())
            .unwrap();
    assert_eq!(
        admission.receipt().lock_revision(),
        ACCEPTED_FEATURE_LOCK_REVISION
    );

    let responder = UdpSocket::bind("127.0.0.1:0").unwrap();
    let discovery_address = responder.local_addr().unwrap();
    let (release_discovery, hold_discovery) = mpsc::channel();
    let body = "<?xml version=\"1.0\"?>\n<info>\n\
\t<name>p8-loopback</name>\n\
\t<type>stable-api-candidate</type>\n\
\t<channel_count>1</channel_count>\n\
\t<channel_format>float32</channel_format>\n\
\t<source_id>p8-explicit-source</source_id>\n\
\t<nominal_srate>0</nominal_srate>\n\
\t<version>1.10</version>\n\
\t<created_at>1</created_at>\n\
\t<uid>88888888-2222-4888-8888-888888888888</uid>\n\
\t<session_id>p8-host-session</session_id>\n\
\t<hostname>p8-loopback-host</hostname>\n\
\t<v4address>127.0.0.1</v4address>\n\
\t<v4data_port>1</v4data_port>\n\
\t<v4service_port>1</v4service_port>\n\
\t<v6address>::</v6address>\n\
\t<v6data_port>0</v6data_port>\n\
\t<v6service_port>0</v6service_port>\n\
\t<desc />\n</info>\n";
    let envelope = format!("88\r\n{body}");
    let discovery_peer = thread::spawn(move || {
        let mut request = [0_u8; 512];
        let (_, source) = responder.recv_from(&mut request).unwrap();
        responder.send_to(envelope.as_bytes(), source).unwrap();
        hold_discovery.recv().unwrap();
    });
    let query_limits = ShortInfoQueryWireLimits::new(128, 256).unwrap();
    let query = ShortInfoQuery::new("name='p8-loopback'".into(), 18888, 88, query_limits).unwrap();
    let query = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
    let envelope_limits =
        ShortInfoResponseEnvelopeLimits::new(body.len(), body.len() + 32).unwrap();
    let discovery = run_udp_discovery(
        UdpDiscoveryActivation::new(admission.capability(RuntimeModule::UdpDiscovery).unwrap())
            .unwrap(),
        UdpDiscoveryConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            discovery_address,
            UdpDiscoveryLimits::new(2048, 1, Duration::from_millis(5), Duration::from_secs(2))
                .unwrap(),
            envelope_limits,
        ),
        &query,
        &AtomicBool::new(false),
    )
    .unwrap();
    release_discovery.send(()).unwrap();
    discovery_peer.join().unwrap();
    assert_eq!(
        discovery.termination(),
        UdpDiscoveryTermination::ResponseLimit
    );
    let selected_index = 0;
    assert_eq!(discovery.responses()[selected_index].query_id(), 88);
    assert_eq!(
        discovery.responses()[selected_index].source(),
        discovery_address
    );
    UdpSocket::bind(discovery_address).unwrap();

    let handshake_limits =
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(2))
            .unwrap();
    let chunk_limits = TimestampedFloat32TwoRecordChunkLimits::new(
        Duration::from_millis(5),
        Duration::from_secs(2),
    )
    .unwrap();
    let make_sample = |timestamp: u64, value: u32| {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(value)],
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(timestamp)).unwrap(),
            None,
        )
    };
    let sent = TimestampedChunk::new(
        ChunkLimits::new(2, 1).unwrap(),
        vec![
            make_sample(0x4092_5220_0000_0088, 0x3fa0_0088),
            make_sample(0x4092_5b80_0000_0089, 0xc020_0089),
        ],
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let session_address = listener.local_addr().unwrap();
    let outlet = thread::spawn(move || {
        run_timestamped_float32_two_record_chunk_outlet(
            sample_activation(&admission),
            listener,
            &identity(handshake_limits),
            handshake_limits,
            chunk_limits,
            &sent,
            &AtomicBool::new(false),
        )
    });

    let inlet_admission =
        admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, CONSUMER, &selections())
            .unwrap();
    let received = run_timestamped_float32_two_record_chunk_inlet(
        sample_activation(&inlet_admission),
        session_address,
        &identity(handshake_limits),
        handshake_limits,
        chunk_limits,
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(outlet.join().unwrap().unwrap(), session_address);
    assert_eq!(
        received.samples()[0].sample().values()[0].to_bits(),
        0x3fa0_0088
    );
    assert_eq!(
        received.samples()[1]
            .raw_source_timestamp()
            .value()
            .to_bits(),
        0x4092_5b80_0000_0089
    );
    TcpListener::bind(session_address).unwrap();

    let first = received.into_samples().into_iter().next().unwrap();
    let correction_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let correction_address = correction_socket.local_addr().unwrap();
    let correction_responder = thread::spawn(move || {
        let mut bytes = [0_u8; 256];
        let (length, source) = correction_socket.recv_from(&mut bytes).unwrap();
        let text = std::str::from_utf8(&bytes[..length]).unwrap();
        let mut fields = text.split("\r\n").nth(1).unwrap().split(' ');
        let id = fields.next().unwrap();
        let t0 = fields.next().unwrap();
        correction_socket
            .send_to(format!(" {id} {t0} 4.0 4.0").as_bytes(), source)
            .unwrap();
    });
    let queue_activation = BoundedSampleQueueActivation::new(
        inlet_admission
            .capability(RuntimeModule::BoundedSampleQueue)
            .unwrap(),
        sample_activation(&inlet_admission),
    )
    .unwrap();
    let recovery_activation = FiniteSampleRecoveryActivation::new(
        inlet_admission
            .capability(RuntimeModule::FiniteSampleRecovery)
            .unwrap(),
        queue_activation,
    )
    .unwrap();
    let clock_activation = IntegratedClockCorrectionActivation::new(
        inlet_admission
            .capability(RuntimeModule::IntegratedClockCorrection)
            .unwrap(),
        sample_activation(&inlet_admission),
    )
    .unwrap();
    let queue = BoundedSampleQueue::new(queue_activation, 1).unwrap();
    let off = AtomicBool::new(false);
    let mut first = Some(first);
    let outcome = run_bounded_float32_recovery_clock_queue(
        recovery_activation,
        FiniteSampleRecoveryPolicy::new(
            1,
            3,
            Duration::ZERO,
            Duration::from_millis(2),
            Duration::from_secs(2),
        )
        .unwrap(),
        |_| Ok(first.take().unwrap()),
        clock_activation,
        IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            correction_address,
            88,
            1,
            256,
            Duration::from_millis(5),
            Duration::from_secs(2),
        )
        .unwrap(),
        &mut CandidateClock([1.0, 2.0], 0),
        &queue,
        BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_secs(1)).unwrap(),
        BoundedFloat32PipelineCancellation::new(&off, &off, &off),
    )
    .unwrap();
    correction_responder.join().unwrap();
    assert!(matches!(
        outcome,
        BoundedFloat32PipelineOutcome::Queued { .. }
    ));
    let corrected = queue
        .pop(
            BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_secs(1)).unwrap(),
            &off,
        )
        .unwrap();
    assert_eq!(corrected.sample().values()[0].to_bits(), 0x3fa0_0088);
    assert!(corrected.derived_timestamp().is_some());
    queue.close().unwrap();
    assert_eq!(queue.try_pop(), Err(BoundedSampleQueuePopError::Closed));
    UdpSocket::bind(correction_address).unwrap();
}
