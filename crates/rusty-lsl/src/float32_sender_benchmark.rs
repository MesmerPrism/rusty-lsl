// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::timestamped_float32_session_runtime::codec::Float32WriterState;
use crate::{
    RawSourceTimestamp, Sample, SampleLimits, TimestampedFloat32SampleLimits, TimestampedSample,
};
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::{Duration, Instant};

const MARKER: &str = "RUSTY_LSL_FLOAT32_SENDER_SAMPLES ";
const MAX_CHANNELS: usize = 4096;
const MAX_RECORDS: usize = 4096;
const MAX_WARMUP: usize = 100_000;
const MAX_ITERATIONS: usize = 100_000;
const MAX_WRITES: usize = 10_000_000;
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

fn benchmark_sample(channels: usize) -> TimestampedSample<f32> {
    let values = (0..channels).map(|channel| channel as f32 + 0.25).collect();
    TimestampedSample::new(
        Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap(),
        RawSourceTimestamp::new(123_456.75).unwrap(),
        None,
    )
}

fn drain(listener: TcpListener, expected_bytes: usize) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut remaining = expected_bytes;
        let mut buffer = [0u8; 8192];
        while remaining != 0 {
            let extent = remaining.min(buffer.len());
            stream.read_exact(&mut buffer[..extent]).unwrap();
            remaining -= extent;
        }
    })
}

fn write_batch(
    state: &mut Float32WriterState,
    stream: &mut TcpStream,
    sample: &TimestampedSample<f32>,
    limits: TimestampedFloat32SampleLimits,
    records: usize,
) {
    let cancelled = AtomicBool::new(false);
    for _ in 0..records {
        state
            .write_record(stream, std::hint::black_box(sample), limits, &cancelled)
            .unwrap();
    }
}

#[test]
#[ignore = "descriptive release-mode sender benchmark"]
fn perf_001_float32_sender_benchmark() {
    let channels = bounded_env("RUSTY_LSL_BENCH_CHANNELS", 1, 1, MAX_CHANNELS);
    let records = bounded_env("RUSTY_LSL_BENCH_RECORDS", 10, 1, MAX_RECORDS);
    let warmup = bounded_env("RUSTY_LSL_BENCH_WARMUP", 20, 0, MAX_WARMUP);
    let iterations = bounded_env("RUSTY_LSL_BENCH_ITERATIONS", 200, 1, MAX_ITERATIONS);
    let batches = warmup.checked_add(iterations).expect("bounded batch count");
    let writes = batches
        .checked_mul(records)
        .filter(|writes| *writes <= MAX_WRITES)
        .expect("benchmark writes exceed the 10,000,000 operation bound");

    let limits =
        TimestampedFloat32SampleLimits::new(Duration::from_millis(50), Duration::from_secs(5))
            .unwrap();
    let sample = benchmark_sample(channels);
    let mut state = Float32WriterState::new(channels).unwrap();
    let record_bytes = state.buffer_identity().1;
    let expected_bytes = writes
        .checked_mul(record_bytes)
        .filter(|bytes| *bytes <= MAX_BYTES)
        .expect("benchmark transport exceeds the 1 GiB byte bound");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let reader = drain(listener, expected_bytes);
    let mut stream = TcpStream::connect(address).unwrap();
    stream.set_nodelay(true).unwrap();

    for _ in 0..warmup {
        write_batch(&mut state, &mut stream, &sample, limits, records);
    }
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let started = Instant::now();
        write_batch(&mut state, &mut stream, &sample, limits, records);
        samples.push(started.elapsed().as_nanos());
    }
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
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
