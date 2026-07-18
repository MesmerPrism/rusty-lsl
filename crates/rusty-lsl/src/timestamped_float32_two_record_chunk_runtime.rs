// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exactly two ordered one-channel Float32 records over the accepted sample transport.

use crate::{
    bounded_fixed_record_transport::{
        read_exact_bounded, write_exact_bounded, BoundedFixedRecordError,
    },
    stream_handshake::{accept_handshake_stream, connect_handshake_stream},
    ChunkLimits, RawSourceTimestamp, Sample, SampleLimits, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedChunk, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleError, TimestampedSample,
};
use std::net::TcpStream;
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

const REQUIRED_RECORDS: usize = 2;
const RECORD_BYTES: usize = 13;
const RECORD_MARKER: u8 = 2;
const INITIALIZATION_TIMESTAMP_BITS: u64 = 0x40fe240c9fbe76c9;
const INITIALIZATION_VALUE_BITS: [u32; 2] = [0x40800000, 0x40000000];

/// Explicit nonzero I/O bounds for the exact two-record chunk stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32TwoRecordChunkLimits {
    io_slice: Duration,
    total_deadline: Duration,
}

impl TimestampedFloat32TwoRecordChunkLimits {
    /// Validates the I/O slice before the total deadline.
    pub fn new(
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, TimestampedFloat32TwoRecordChunkLimitError> {
        if io_slice.is_zero() {
            return Err(TimestampedFloat32TwoRecordChunkLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(TimestampedFloat32TwoRecordChunkLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            io_slice,
            total_deadline,
        })
    }
}

/// Invalid exact-chunk I/O bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32TwoRecordChunkLimitError {
    /// The I/O slice was zero.
    ZeroIoSlice,
    /// The total deadline was zero.
    ZeroTotalDeadline,
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

fn write_record(
    stream: &mut TcpStream,
    sample: &TimestampedSample<f32>,
    limits: TimestampedFloat32TwoRecordChunkLimits,
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

fn read_record(
    stream: &mut TcpStream,
    limits: TimestampedFloat32TwoRecordChunkLimits,
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
    Ok(TimestampedSample::new(
        Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
        timestamp,
        None,
    ))
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
    limits: TimestampedFloat32TwoRecordChunkLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32SampleError> {
    for bits in INITIALIZATION_VALUE_BITS {
        write_record(stream, &initialization_sample(bits), limits, cancelled)?;
    }
    Ok(())
}

fn read_initialization(
    stream: &mut TcpStream,
    limits: TimestampedFloat32TwoRecordChunkLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32SampleError> {
    for (index, expected) in INITIALIZATION_VALUE_BITS.into_iter().enumerate() {
        let record = read_record(stream, limits, cancelled)?;
        if record.raw_source_timestamp().value().to_bits() != INITIALIZATION_TIMESTAMP_BITS
            || record.sample().values()[0].to_bits() != expected
        {
            return Err(TimestampedFloat32SampleError::InvalidInitialization { index });
        }
    }
    Ok(())
}

/// Stable failure for the exact two-record Float32 chunk surface.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedFloat32TwoRecordChunkError {
    /// The caller supplied a chunk with a count other than two.
    RecordCount {
        /// Actual record count.
        actual: usize,
    },
    /// The accepted sample transport owner rejected setup or a named record.
    Sample {
        /// Zero-based caller-record index, or `None` during handshake/initialization.
        index: Option<usize>,
        /// Unchanged sample-owner failure.
        error: TimestampedFloat32SampleError,
    },
}

/// Opens one accepted outlet connection, sends exactly two ordered records, and closes on return.
pub fn run_timestamped_float32_two_record_chunk_outlet(
    activation: TimestampedFloat32SampleActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32TwoRecordChunkLimits,
    chunk: &TimestampedChunk<f32>,
    cancelled: &AtomicBool,
) -> Result<SocketAddr, TimestampedFloat32TwoRecordChunkError> {
    if chunk.samples().len() != REQUIRED_RECORDS {
        return Err(TimestampedFloat32TwoRecordChunkError::RecordCount {
            actual: chunk.samples().len(),
        });
    }
    let (mut stream, local, _) =
        accept_handshake_stream(listener, identity, handshake_limits, cancelled).map_err(
            |error| TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Handshake(error),
            },
        )?;
    let _ = activation;
    write_initialization(&mut stream, sample_limits, cancelled)
        .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample { index: None, error })?;
    for (index, sample) in chunk.samples().iter().enumerate() {
        write_record(&mut stream, sample, sample_limits, cancelled).map_err(|error| {
            TimestampedFloat32TwoRecordChunkError::Sample {
                index: Some(index),
                error,
            }
        })?;
    }
    Ok(local)
}

/// Opens one accepted inlet connection, receives exactly two ordered records, and closes on return.
pub fn run_timestamped_float32_two_record_chunk_inlet(
    activation: TimestampedFloat32SampleActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32TwoRecordChunkLimits,
    cancelled: &AtomicBool,
) -> Result<TimestampedChunk<f32>, TimestampedFloat32TwoRecordChunkError> {
    let mut stream = connect_handshake_stream(peer, identity, handshake_limits, cancelled)
        .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample {
            index: None,
            error: TimestampedFloat32SampleError::Handshake(error),
        })?;
    let _ = activation;
    read_initialization(&mut stream, sample_limits, cancelled)
        .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample { index: None, error })?;
    let mut samples = Vec::with_capacity(REQUIRED_RECORDS);
    for index in 0..REQUIRED_RECORDS {
        samples.push(
            read_record(&mut stream, sample_limits, cancelled).map_err(|error| {
                TimestampedFloat32TwoRecordChunkError::Sample {
                    index: Some(index),
                    error,
                }
            })?,
        );
    }
    Ok(TimestampedChunk::new(ChunkLimits::new(REQUIRED_RECORDS, 1).unwrap(), samples).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_activation::test_capability, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, StreamHandshakeActivation, TimestampedSample,
    };
    use std::thread;
    use std::time::Duration;

    fn activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32TwoRecordChunkLimits {
        TimestampedFloat32TwoRecordChunkLimits::new(
            Duration::from_millis(5),
            Duration::from_secs(1),
        )
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
    fn candidate_two_record_chunk_preserves_order_bits_and_releases_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let sent = TimestampedChunk::new(
            ChunkLimits::new(2, 1).unwrap(),
            vec![sample(2345.125, 1.25), sample(2346.875, -2.5)],
        )
        .unwrap();
        let worker = thread::spawn(move || {
            run_timestamped_float32_two_record_chunk_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sent,
                &AtomicBool::new(false),
            )
        });
        let received = run_timestamped_float32_two_record_chunk_inlet(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(received.samples().len(), 2);
        assert_eq!(
            received.samples()[0]
                .raw_source_timestamp()
                .value()
                .to_bits(),
            2345.125f64.to_bits()
        );
        assert_eq!(
            received.samples()[0].sample().values()[0].to_bits(),
            1.25f32.to_bits()
        );
        assert_eq!(
            received.samples()[1]
                .raw_source_timestamp()
                .value()
                .to_bits(),
            2346.875f64.to_bits()
        );
        assert_eq!(
            received.samples()[1].sample().values()[0].to_bits(),
            (-2.5f32).to_bits()
        );
        assert_eq!(worker.join().unwrap().unwrap(), address);
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn candidate_wrong_record_count_rejects_before_socket_use() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let one =
            TimestampedChunk::new(ChunkLimits::new(2, 1).unwrap(), vec![sample(1.0, 2.0)]).unwrap();
        assert_eq!(
            run_timestamped_float32_two_record_chunk_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &one,
                &AtomicBool::new(false),
            ),
            Err(TimestampedFloat32TwoRecordChunkError::RecordCount { actual: 1 })
        );
        TcpListener::bind(address).unwrap();
    }
}
