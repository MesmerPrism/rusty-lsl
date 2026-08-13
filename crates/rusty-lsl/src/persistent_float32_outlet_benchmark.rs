// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::runtime_activation::test_capability;
use crate::stream_handshake::connect_handshake_stream;
use crate::timestamped_float32_session_runtime::codec::read_initialization_for_channels;
use crate::{
    PersistentFloat32Outlet, PersistentFloat32OutletActivation, PersistentFloat32OutletLimits,
    RawSourceTimestamp, RuntimeModule, StreamHandshakeActivation, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
};
use std::env;
use std::io::Read;
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::{Duration, Instant};

const MARKER: &str = "RUSTY_LSL_PERSISTENT_FLOAT32_OUTLET_SAMPLES ";
const MAX_CHANNELS: usize = 4096;
const MAX_RECORDS: usize = 4096;
const MAX_WARMUP: usize = 100_000;
const MAX_ITERATIONS: usize = 100_000;
const MAX_PUSHES: usize = 1_000_000;
const MAX_BYTES: usize = 1_073_741_824;

fn bounded_env(name: &str, default: usize, minimum: usize, maximum: usize) -> usize {
    let value = env::var(name)
        .ok()
        .map(|raw| {
            raw.parse::<usize>()
                .expect("benchmark bound must be an integer")
        })
        .unwrap_or(default);
    assert!(
        (minimum..=maximum).contains(&value),
        "{name} must be in {minimum}..={maximum}"
    );
    value
}

fn handshake_limits() -> StreamHandshakeLimits {
    StreamHandshakeLimits::new(1024, 128, Duration::from_millis(10), Duration::from_secs(5))
        .unwrap()
}

fn sample_limits() -> TimestampedFloat32SampleLimits {
    TimestampedFloat32SampleLimits::new(Duration::from_millis(50), Duration::from_secs(5)).unwrap()
}

fn identity() -> StreamHandshakeIdentity {
    StreamHandshakeIdentity::new(
        "70000000-0000-4000-8000-000000000099".into(),
        "persistent-benchmark-host".into(),
        "persistent-benchmark-source".into(),
        "persistent-benchmark-session".into(),
        handshake_limits(),
    )
    .unwrap()
}

fn activation() -> PersistentFloat32OutletActivation {
    PersistentFloat32OutletActivation::new(
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap(),
    )
}

fn drain(address: SocketAddr, channels: usize, expected_bytes: usize) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let cancelled = AtomicBool::new(false);
        let mut stream =
            connect_handshake_stream(address, &identity(), handshake_limits(), &cancelled).unwrap();
        read_initialization_for_channels(&mut stream, channels, sample_limits(), &cancelled)
            .unwrap();
        let mut remaining = expected_bytes;
        let mut buffer = [0u8; 8192];
        while remaining != 0 {
            let extent = remaining.min(buffer.len());
            stream.read_exact(&mut buffer[..extent]).unwrap();
            remaining -= extent;
        }
    })
}

#[test]
#[ignore = "descriptive release-mode persistent chunk outlet benchmark"]
fn perf_002_persistent_float32_outlet_benchmark() {
    let channels = bounded_env("RUSTY_LSL_PERSISTENT_BENCH_CHANNELS", 1, 1, MAX_CHANNELS);
    let records = bounded_env("RUSTY_LSL_PERSISTENT_BENCH_RECORDS", 10, 1, MAX_RECORDS);
    let warmup = bounded_env("RUSTY_LSL_PERSISTENT_BENCH_WARMUP", 20, 0, MAX_WARMUP);
    let iterations = bounded_env(
        "RUSTY_LSL_PERSISTENT_BENCH_ITERATIONS",
        200,
        1,
        MAX_ITERATIONS,
    );
    let pushes = warmup
        .checked_add(iterations)
        .filter(|pushes| *pushes <= MAX_PUSHES)
        .expect("benchmark pushes exceed the 1,000,000 operation bound");
    let record_bytes = 9usize
        .checked_add(channels.checked_mul(4).expect("bounded record shape"))
        .expect("bounded record shape");
    let expected_bytes = pushes
        .checked_mul(records)
        .and_then(|samples| samples.checked_mul(record_bytes))
        .filter(|bytes| *bytes <= MAX_BYTES)
        .expect("benchmark transport exceeds the 1 GiB byte bound");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut outlet = PersistentFloat32Outlet::new(
        activation(),
        listener,
        identity(),
        handshake_limits(),
        sample_limits(),
        channels,
        PersistentFloat32OutletLimits::new(records, 1).unwrap(),
    )
    .unwrap();
    let reader = drain(outlet.local_address(), channels, expected_bytes);
    for _ in 0..5000 {
        if outlet
            .poll_accept_consumer(&AtomicBool::new(false))
            .unwrap()
            .is_some()
        {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(outlet.connected_consumers(), 1);
    let values = (0..records * channels)
        .map(|index| index as f32 + 0.25)
        .collect::<Vec<_>>();
    let timestamps = (0..records)
        .map(|index| RawSourceTimestamp::new(123_456.75 + index as f64).unwrap())
        .collect::<Vec<_>>();
    let cancelled = AtomicBool::new(false);
    for _ in 0..warmup {
        outlet
            .push_chunk(
                std::hint::black_box(&values),
                std::hint::black_box(&timestamps),
                &cancelled,
            )
            .unwrap();
    }
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let started = Instant::now();
        outlet
            .push_chunk(
                std::hint::black_box(&values),
                std::hint::black_box(&timestamps),
                &cancelled,
            )
            .unwrap();
        samples.push(started.elapsed().as_nanos());
    }
    let _ = outlet.close();
    reader.join().unwrap();
    let samples = samples
        .iter()
        .map(u128::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{MARKER}{{\"channels\":{channels},\"records\":{records},\"warmup\":{warmup},\"iterations\":{iterations},\"samples_ns\":[{samples}]}}"
    );
}
