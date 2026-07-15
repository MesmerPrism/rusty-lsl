// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! One bounded protocol-110 timestamped single-channel `float32` record.

use crate::{
    bounded_fixed_record_transport::{
        read_exact_bounded, write_exact_bounded, BoundedFixedRecordError,
    },
    stream_handshake::{accept_handshake_stream, connect_handshake_stream},
    RawSourceTimestamp, RuntimeModule, RuntimeModuleCapability, Sample, SampleLimits,
    StreamHandshakeActivation, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedSample,
};
use std::io::{ErrorKind, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// Feature selected for the one-record sample effect.
pub const TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID: &str = "timestamped-float32-sample";
/// Exact effective marker required at runtime.
pub const TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER: &str =
    "rusty.lsl.timestamped_float32_sample.effective";
const RECORD_BYTES: usize = 13;
const RECORD_MARKER: u8 = 2;
const INITIALIZATION_TIMESTAMP_BITS: u64 = 0x40fe240c9fbe76c9;
const INITIALIZATION_VALUE_BITS: [u32; 2] = [0x40800000, 0x40000000];

/// Closed activation composed with accepted handshake activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32SampleActivation {
    handshake: StreamHandshakeActivation,
}

impl TimestampedFloat32SampleActivation {
    /// Admits the exact selected feature and marker beside handshake activation.
    pub fn new(
        capability: RuntimeModuleCapability,
        handshake: StreamHandshakeActivation,
    ) -> Result<Self, TimestampedFloat32SampleActivationError> {
        if !capability.matches(RuntimeModule::TimestampedFloat32Sample) {
            return Err(TimestampedFloat32SampleActivationError::WrongModule);
        }
        Ok(Self { handshake })
    }
}

/// Rejected sample activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SampleActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}

/// Finite I/O limits for the one fixed-size record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32SampleLimits {
    io_slice: Duration,
    total_deadline: Duration,
}

impl TimestampedFloat32SampleLimits {
    /// Creates explicit nonzero I/O slice and total deadline limits.
    pub fn new(
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, TimestampedFloat32SampleLimitError> {
        if io_slice.is_zero() {
            return Err(TimestampedFloat32SampleLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(TimestampedFloat32SampleLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            io_slice,
            total_deadline,
        })
    }
}

/// Invalid one-record limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SampleLimitError {
    /// I/O slice was zero.
    ZeroIoSlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
}

/// Stable failure from connection setup or one-record transfer.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SampleError {
    /// Accepted handshake setup failed.
    Handshake(StreamHandshakeError),
    /// Caller supplied a shape other than one channel.
    ChannelCount {
        /// Actual channel count.
        actual: usize,
    },
    /// Caller cancellation was observed.
    Cancelled,
    /// Sample-stage deadline elapsed.
    Deadline,
    /// Sample-stage socket I/O failed.
    Io(ErrorKind),
    /// Peer closed before the fixed record completed.
    Truncated {
        /// Bytes received before close.
        actual: usize,
    },
    /// Fixed record marker differed.
    InvalidMarker {
        /// Observed marker.
        actual: u8,
    },
    /// Timestamp bytes decoded outside the finite raw timestamp domain.
    InvalidTimestamp,
    /// A required post-handshake initialization record differed.
    InvalidInitialization {
        /// Zero-based initialization record index.
        index: usize,
    },
}

fn initialization_sample(value_bits: u32) -> TimestampedSample<f32> {
    TimestampedSample::new(
        Sample::new(
            SampleLimits::new(1).unwrap(),
            1,
            vec![f32::from_bits(value_bits)],
        )
        .unwrap(),
        RawSourceTimestamp::new(f64::from_bits(INITIALIZATION_TIMESTAMP_BITS)).unwrap(),
        None,
    )
}

fn write_initialization(
    stream: &mut TcpStream,
    limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32SampleError> {
    for value_bits in INITIALIZATION_VALUE_BITS {
        write_record(
            stream,
            &initialization_sample(value_bits),
            limits,
            cancelled,
        )?;
    }
    Ok(())
}

fn read_initialization(
    stream: &mut TcpStream,
    limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32SampleError> {
    for (index, expected_value) in INITIALIZATION_VALUE_BITS.into_iter().enumerate() {
        let record = read_record(stream, limits, cancelled)?;
        if record.raw_source_timestamp().value().to_bits() != INITIALIZATION_TIMESTAMP_BITS
            || record.sample().values()[0].to_bits() != expected_value
        {
            return Err(TimestampedFloat32SampleError::InvalidInitialization { index });
        }
    }
    Ok(())
}

fn write_record(
    stream: &mut TcpStream,
    sample: &TimestampedSample<f32>,
    limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32SampleError> {
    if sample.sample().declared_channels() != 1 {
        return Err(TimestampedFloat32SampleError::ChannelCount {
            actual: sample.sample().declared_channels(),
        });
    }
    let mut record = [0u8; RECORD_BYTES];
    record[0] = RECORD_MARKER;
    record[1..9].copy_from_slice(&sample.raw_source_timestamp().value().to_le_bytes());
    record[9..13].copy_from_slice(&sample.sample().values()[0].to_le_bytes());
    write_exact_bounded(
        stream,
        &record,
        limits.io_slice,
        limits.total_deadline,
        cancelled,
    )
    .map_err(map_transport_error)
}

fn map_transport_error(error: BoundedFixedRecordError) -> TimestampedFloat32SampleError {
    match error {
        BoundedFixedRecordError::Cancelled => TimestampedFloat32SampleError::Cancelled,
        BoundedFixedRecordError::Deadline => TimestampedFloat32SampleError::Deadline,
        BoundedFixedRecordError::Truncated { actual } => {
            TimestampedFloat32SampleError::Truncated { actual }
        }
        BoundedFixedRecordError::Io(kind) => TimestampedFloat32SampleError::Io(kind),
    }
}

fn read_record(
    stream: &mut TcpStream,
    limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<TimestampedSample<f32>, TimestampedFloat32SampleError> {
    let mut record = [0u8; RECORD_BYTES];
    read_exact_bounded(
        stream,
        &mut record,
        limits.io_slice,
        limits.total_deadline,
        cancelled,
    )
    .map_err(map_transport_error)?;
    if record[0] != RECORD_MARKER {
        return Err(TimestampedFloat32SampleError::InvalidMarker { actual: record[0] });
    }
    let timestamp = RawSourceTimestamp::new(f64::from_le_bytes(record[1..9].try_into().unwrap()))
        .map_err(|_| TimestampedFloat32SampleError::InvalidTimestamp)?;
    let value = f32::from_le_bytes(record[9..13].try_into().unwrap());
    let sample = Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap();
    Ok(TimestampedSample::new(sample, timestamp, None))
}

/// Opens the accepted outlet handshake, sends exactly one record, and closes on return.
pub fn run_timestamped_float32_outlet(
    activation: TimestampedFloat32SampleActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    sample: &TimestampedSample<f32>,
    cancelled: &AtomicBool,
) -> Result<SocketAddr, TimestampedFloat32SampleError> {
    let (mut stream, local, _) =
        accept_handshake_stream(listener, identity, handshake_limits, cancelled)
            .map_err(TimestampedFloat32SampleError::Handshake)?;
    let _ = activation.handshake;
    write_initialization(&mut stream, sample_limits, cancelled)?;
    write_record(&mut stream, sample, sample_limits, cancelled)?;
    Ok(local)
}

/// Opens the accepted inlet handshake, receives exactly one record, and closes on return.
pub fn run_timestamped_float32_inlet(
    activation: TimestampedFloat32SampleActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<TimestampedSample<f32>, TimestampedFloat32SampleError> {
    let mut stream = connect_handshake_stream(peer, identity, handshake_limits, cancelled)
        .map_err(TimestampedFloat32SampleError::Handshake)?;
    let _ = activation.handshake;
    read_initialization(&mut stream, sample_limits, cancelled)?;
    read_record(&mut stream, sample_limits, cancelled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use std::thread;

    fn handshake() -> StreamHandshakeActivation {
        StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake)).unwrap()
    }
    fn activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            handshake(),
        )
        .unwrap()
    }
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
            "synthetic-host".into(),
            "synthetic-source".into(),
            "synthetic-session".into(),
            handshake_limits(),
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

    #[test]
    fn lslc_002t_one_loopback_sample_preserves_exact_bits_and_releases_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let sent = sample(-0.0, f32::from_bits(0x7fc0_1234));
        let worker = thread::spawn(move || {
            run_timestamped_float32_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sent,
                &AtomicBool::new(false),
            )
        });
        let received = run_timestamped_float32_inlet(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(
            received.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(received.sample().values()[0].to_bits(), 0x7fc0_1234);
        assert_eq!(worker.join().unwrap().unwrap(), address);
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn lslc_002t_activation_limits_shape_and_cancellation_fail_closed() {
        assert_eq!(
            TimestampedFloat32SampleActivation::new(
                test_capability(RuntimeModule::UdpDiscovery),
                handshake()
            ),
            Err(TimestampedFloat32SampleActivationError::WrongModule)
        );
        assert_eq!(
            TimestampedFloat32SampleLimits::new(Duration::ZERO, Duration::ZERO),
            Err(TimestampedFloat32SampleLimitError::ZeroIoSlice)
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (_reader, _) = listener.accept().unwrap();
        let two = TimestampedSample::new(
            Sample::new(SampleLimits::new(2).unwrap(), 2, vec![1.0, 2.0]).unwrap(),
            RawSourceTimestamp::new(1.0).unwrap(),
            None,
        );
        assert_eq!(
            write_record(&mut writer, &two, sample_limits(), &AtomicBool::new(false)),
            Err(TimestampedFloat32SampleError::ChannelCount { actual: 2 })
        );
        let cancelled = AtomicBool::new(true);
        assert_eq!(
            run_timestamped_float32_inlet(
                activation(),
                "127.0.0.1:9".parse().unwrap(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                &cancelled,
            ),
            Err(TimestampedFloat32SampleError::Handshake(
                StreamHandshakeError::Cancelled
            ))
        );
    }

    #[test]
    fn lslc_002t_truncated_and_damaged_records_are_typed() {
        let mut nonfinite = vec![RECORD_MARKER];
        nonfinite.extend_from_slice(&f64::INFINITY.to_le_bytes());
        nonfinite.extend_from_slice(&1.0f32.to_le_bytes());
        for (record, expected) in [
            (
                vec![2, 0],
                TimestampedFloat32SampleError::Truncated { actual: 2 },
            ),
            (
                vec![9; RECORD_BYTES],
                TimestampedFloat32SampleError::InvalidMarker { actual: 9 },
            ),
            (nonfinite, TimestampedFloat32SampleError::InvalidTimestamp),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let worker = thread::spawn(move || {
                let (mut stream, _, _) = accept_handshake_stream(
                    listener,
                    &identity(),
                    handshake_limits(),
                    &AtomicBool::new(false),
                )
                .unwrap();
                stream.write_all(&record).unwrap();
            });
            assert_eq!(
                run_timestamped_float32_inlet(
                    activation(),
                    address,
                    &identity(),
                    handshake_limits(),
                    sample_limits(),
                    &AtomicBool::new(false),
                ),
                Err(expected)
            );
            worker.join().unwrap();
        }
    }

    #[test]
    fn lslc_002y_initialization_sequence_is_exact_and_ordered() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (mut reader, _) = listener.accept().unwrap();
        write_record(
            &mut writer,
            &initialization_sample(INITIALIZATION_VALUE_BITS[0]),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        write_record(
            &mut writer,
            &initialization_sample(INITIALIZATION_VALUE_BITS[1]),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        read_initialization(&mut reader, sample_limits(), &AtomicBool::new(false)).unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (mut reader, _) = listener.accept().unwrap();
        write_record(
            &mut writer,
            &initialization_sample(INITIALIZATION_VALUE_BITS[0]),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        write_record(
            &mut writer,
            &initialization_sample(3.0f32.to_bits()),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(
            read_initialization(&mut reader, sample_limits(), &AtomicBool::new(false)),
            Err(TimestampedFloat32SampleError::InvalidInitialization { index: 1 })
        );
    }
}
