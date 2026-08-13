// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ignored host driver for a pinned official liblsl consumer qualification.

use crate::runtime_activation::test_capability;
use crate::{
    MetadataTreeLimits, PersistentFloat32Outlet, PersistentFloat32OutletActivation,
    PersistentFloat32OutletLimits, PersistentFloat32OutletService,
    PersistentFloat32OutletServiceLimits, RawSourceTimestamp, RuntimeModule,
    ShortInfoQueryWireLimits, ShortInfoResponderActivation, ShortInfoResponseEnvelopeLimits,
    StreamDescriptorLimits, StreamHandshakeActivation, StreamHandshakeIdentity,
    StreamHandshakeLimits, StreamInfoObservedAdmissionLimits, StreamInfoObservedDocumentParseLimit,
    StreamInfoVolatileFieldLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits,
};
use std::env;
use std::fs;
use std::net::{Ipv4Addr, TcpListener};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::{Duration, Instant};

const SOURCE_ID: &str = "rusty-lsl-interop-001-official-consumer";
const RECORDS: usize = 10;

fn required_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn handshake_limits() -> StreamHandshakeLimits {
    StreamHandshakeLimits::new(4096, 256, Duration::from_millis(10), Duration::from_secs(5))
        .unwrap()
}

fn sample_limits() -> TimestampedFloat32SampleLimits {
    TimestampedFloat32SampleLimits::new(Duration::from_millis(10), Duration::from_secs(5)).unwrap()
}

fn identity() -> StreamHandshakeIdentity {
    StreamHandshakeIdentity::new(
        "71000000-0000-4000-8000-000000000101".into(),
        "rusty-lsl-interop-host".into(),
        SOURCE_ID.into(),
        "rusty-lsl-interop-session".into(),
        handshake_limits(),
    )
    .unwrap()
}

fn outlet_activation() -> PersistentFloat32OutletActivation {
    PersistentFloat32OutletActivation::new(
        test_capability(RuntimeModule::PersistentFloat32Outlet),
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn responder_activation() -> ShortInfoResponderActivation {
    ShortInfoResponderActivation::new(test_capability(RuntimeModule::ShortInfoDiscoveryResponder))
        .unwrap()
}

fn body(interface: Ipv4Addr, port: u16) -> String {
    format!(
        "<?xml version=\"1.0\"?>\n<info>\n\
\t<name>Rusty LSL Interop 001</name>\n\
\t<type>qualification</type>\n\
\t<channel_count>1</channel_count>\n\
\t<channel_format>float32</channel_format>\n\
\t<source_id>{SOURCE_ID}</source_id>\n\
\t<nominal_srate>100.0000000000000</nominal_srate>\n\
\t<version>1.100000000000000</version>\n\
\t<created_at>1.0</created_at>\n\
\t<uid>71000000-0000-4000-8000-000000000101</uid>\n\
\t<session_id>rusty-lsl-interop-session</session_id>\n\
\t<hostname>rusty-lsl-interop-host</hostname>\n\
\t<v4address>{interface}</v4address>\n\
\t<v4data_port>{port}</v4data_port>\n\
\t<v4service_port>{port}</v4service_port>\n\
\t<v6address></v6address>\n\
\t<v6data_port>0</v6data_port>\n\
\t<v6service_port>0</v6service_port>\n\
\t<desc />\n</info>\n"
    )
}

fn service_limits(body_len: usize) -> PersistentFloat32OutletServiceLimits {
    PersistentFloat32OutletServiceLimits::new(
        4096,
        StreamInfoObservedDocumentParseLimit::new(body_len).unwrap(),
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(256, 256, 256, 512).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 4, 1).unwrap(),
            StreamInfoVolatileFieldLimits::new(256, 256, 256).unwrap(),
        ),
        ShortInfoQueryWireLimits::new(1024, 2048).unwrap(),
        ShortInfoResponseEnvelopeLimits::new(body_len, body_len + 64).unwrap(),
    )
    .unwrap()
}

#[test]
#[ignore = "requires pinned pylsl 1.18.2/liblsl 1.17 official consumer"]
fn interop_001_official_consumer_qualification_server() {
    let interface = required_env("RUSTY_LSL_INTEROP_INTERFACE")
        .parse::<Ipv4Addr>()
        .expect("explicit interface must be IPv4");
    let ready = PathBuf::from(required_env("RUSTY_LSL_INTEROP_READY_FILE"));
    let consumer_ready = PathBuf::from(required_env("RUSTY_LSL_INTEROP_CONSUMER_READY_FILE"));
    let ack = PathBuf::from(required_env("RUSTY_LSL_INTEROP_ACK_FILE"));
    let listener = TcpListener::bind((interface, 0)).unwrap();
    let outlet = PersistentFloat32Outlet::new(
        outlet_activation(),
        listener,
        identity(),
        handshake_limits(),
        sample_limits(),
        1,
        PersistentFloat32OutletLimits::new(RECORDS, 1).unwrap(),
    )
    .unwrap();
    let text = body(interface, outlet.local_address().port());
    let mut service = PersistentFloat32OutletService::new_explicit_ipv4_multicast(
        responder_activation(),
        interface,
        outlet,
        text.clone(),
        service_limits(text.len()),
    )
    .unwrap();
    fs::write(
        &ready,
        format!(
            "{{\"schema\":\"rusty.lsl.interop_001.server_ready.v1\",\"source_id\":\"{SOURCE_ID}\",\"channels\":1,\"records\":{RECORDS}}}"
        ),
    )
    .unwrap();

    let cancelled = AtomicBool::new(false);
    let started = Instant::now();
    let mut discovery_requests = 0usize;
    let mut accepted = false;
    let mut pushed = false;
    while started.elapsed() < Duration::from_secs(20) {
        let poll = service.poll(&cancelled).unwrap();
        if poll.discovery().is_some() {
            discovery_requests += 1;
        }
        if poll.consumer().is_some() {
            accepted = true;
        }
        if accepted && consumer_ready.is_file() && !pushed {
            let base_timestamp = fs::read_to_string(&consumer_ready)
                .unwrap()
                .trim()
                .parse::<f64>()
                .unwrap();
            let values = (0..RECORDS)
                .map(|index| f32::from(u16::try_from(index).unwrap()) + 0.25)
                .collect::<Vec<_>>();
            let timestamps = (0..RECORDS)
                .map(|index| {
                    RawSourceTimestamp::new(
                        base_timestamp + f64::from(u16::try_from(index).unwrap()) * 0.01,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let report = service
                .push_chunk(&values, &timestamps, &cancelled)
                .unwrap();
            assert_eq!(report.record_count(), RECORDS);
            assert_eq!(report.complete_deliveries(), 1);
            pushed = true;
        }
        if pushed && ack.is_file() {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(
        discovery_requests > 0,
        "official resolver query was not handled"
    );
    assert!(
        accepted,
        "official inlet did not complete the persistent handshake"
    );
    assert!(pushed, "qualified chunk was not pushed");
    assert!(
        ack.is_file(),
        "official consumer did not acknowledge exact data"
    );
    let close = service.close();
    println!(
        "RUSTY_LSL_INTEROP_001_SERVER {{\"discovery_requests\":{discovery_requests},\"accepted_consumers\":1,\"records\":{RECORDS},\"closed_consumers\":{}}}",
        close.outlet().closed_consumers()
    );
}
