// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exactly two ordered one-channel Float32 records over the accepted sample transport.

use crate::{
    stream_handshake::{accept_handshake_stream, connect_handshake_stream},
    timestamped_float32_sample_runtime::{
        read_initialization, read_record, write_initialization, write_record,
    },
    ChunkLimits, StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedChunk,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleError,
    TimestampedFloat32SampleLimits,
};
use std::io::{ErrorKind, Read};
use std::net::TcpStream;
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

const REQUIRED_RECORDS: usize = 2;

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

    fn sample_limits(self) -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(self.io_slice, self.total_deadline).unwrap()
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

fn require_peer_close(
    stream: &mut TcpStream,
    limits: TimestampedFloat32TwoRecordChunkLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32TwoRecordChunkError> {
    let start = Instant::now();
    let mut byte = [0u8; 1];
    loop {
        if cancelled.load(Ordering::Acquire) {
            return Err(TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Cancelled,
            });
        }
        let remaining = limits.total_deadline.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            return Err(TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Deadline,
            });
        }
        stream
            .set_read_timeout(Some(limits.io_slice.min(remaining)))
            .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Io(error.kind()),
            })?;
        match stream.read(&mut byte) {
            Ok(0) => return Ok(()),
            Ok(_) => {
                return Err(TimestampedFloat32TwoRecordChunkError::TrailingByte { actual: byte[0] })
            }
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(error) => {
                return Err(TimestampedFloat32TwoRecordChunkError::Sample {
                    index: None,
                    error: TimestampedFloat32SampleError::Io(error.kind()),
                })
            }
        }
    }
}

/// Stable failure for the exact two-record Float32 chunk surface.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedFloat32TwoRecordChunkError {
    /// The caller supplied a chunk with a count other than two.
    RecordCount {
        /// Actual record count.
        actual: usize,
    },
    /// The peer sent data after the exact second caller record.
    TrailingByte {
        /// First byte beyond the accepted two-record envelope.
        actual: u8,
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
    write_initialization(&mut stream, sample_limits.sample_limits(), cancelled)
        .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample { index: None, error })?;
    for (index, sample) in chunk.samples().iter().enumerate() {
        write_record(
            &mut stream,
            sample,
            sample_limits.sample_limits(),
            cancelled,
        )
        .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample {
            index: Some(index),
            error,
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
    read_initialization(&mut stream, sample_limits.sample_limits(), cancelled)
        .map_err(|error| TimestampedFloat32TwoRecordChunkError::Sample { index: None, error })?;
    let mut samples = Vec::with_capacity(REQUIRED_RECORDS);
    for index in 0..REQUIRED_RECORDS {
        samples.push(
            read_record(&mut stream, sample_limits.sample_limits(), cancelled).map_err(
                |error| TimestampedFloat32TwoRecordChunkError::Sample {
                    index: Some(index),
                    error,
                },
            )?,
        );
    }
    require_peer_close(&mut stream, sample_limits, cancelled)?;
    Ok(TimestampedChunk::new(ChunkLimits::new(REQUIRED_RECORDS, 1).unwrap(), samples).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_activation::test_capability, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, StreamHandshakeActivation, TimestampedSample,
    };
    use std::io::Write;
    use std::sync::Arc;
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

    fn spawn_peer(
        write_tail: impl FnOnce(&mut TcpStream) + Send + 'static,
        hold_open: Duration,
    ) -> (SocketAddr, thread::JoinHandle<()>) {
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
            write_initialization(
                &mut stream,
                sample_limits().sample_limits(),
                &AtomicBool::new(false),
            )
            .unwrap();
            write_tail(&mut stream);
            if !hold_open.is_zero() {
                thread::sleep(hold_open);
            }
        });
        (address, worker)
    }

    fn receive_from_peer(
        address: SocketAddr,
        limits: TimestampedFloat32TwoRecordChunkLimits,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedChunk<f32>, TimestampedFloat32TwoRecordChunkError> {
        run_timestamped_float32_two_record_chunk_inlet(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            limits,
            cancelled,
        )
    }

    #[test]
    fn candidate_damaged_marker_and_truncation_keep_record_ownership() {
        for (tail, expected) in [
            (
                vec![9; 13],
                TimestampedFloat32SampleError::InvalidMarker { actual: 9 },
            ),
            (
                vec![2, 0],
                TimestampedFloat32SampleError::Truncated { actual: 2 },
            ),
        ] {
            let (address, worker) = spawn_peer(
                move |stream| stream.write_all(&tail).unwrap(),
                Duration::ZERO,
            );
            assert_eq!(
                receive_from_peer(address, sample_limits(), &AtomicBool::new(false)),
                Err(TimestampedFloat32TwoRecordChunkError::Sample {
                    index: Some(0),
                    error: expected,
                })
            );
            worker.join().unwrap();
            TcpListener::bind(address).unwrap();
        }
    }

    #[test]
    fn candidate_extra_record_is_typed_and_releases_port() {
        let (address, worker) = spawn_peer(
            |stream| {
                for record in [sample(1.0, 2.0), sample(3.0, 4.0)] {
                    write_record(
                        stream,
                        &record,
                        sample_limits().sample_limits(),
                        &AtomicBool::new(false),
                    )
                    .unwrap();
                }
                stream.write_all(&[2]).unwrap();
            },
            Duration::ZERO,
        );
        assert_eq!(
            receive_from_peer(address, sample_limits(), &AtomicBool::new(false)),
            Err(TimestampedFloat32TwoRecordChunkError::TrailingByte { actual: 2 })
        );
        worker.join().unwrap();
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn candidate_terminal_deadline_and_cancellation_are_typed_and_cleanup() {
        let short = TimestampedFloat32TwoRecordChunkLimits::new(
            Duration::from_millis(2),
            Duration::from_millis(20),
        )
        .unwrap();
        let write_two = |stream: &mut TcpStream| {
            for record in [sample(1.0, 2.0), sample(3.0, 4.0)] {
                write_record(
                    stream,
                    &record,
                    sample_limits().sample_limits(),
                    &AtomicBool::new(false),
                )
                .unwrap();
            }
        };
        let (address, worker) = spawn_peer(write_two, Duration::from_millis(80));
        assert_eq!(
            receive_from_peer(address, short, &AtomicBool::new(false)),
            Err(TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Deadline,
            })
        );
        worker.join().unwrap();
        TcpListener::bind(address).unwrap();

        let (address, worker) = spawn_peer(write_two, Duration::from_millis(80));
        let cancelled = Arc::new(AtomicBool::new(false));
        let trigger = Arc::clone(&cancelled);
        let canceller = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            trigger.store(true, Ordering::Release);
        });
        assert_eq!(
            receive_from_peer(address, sample_limits(), &cancelled),
            Err(TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Cancelled,
            })
        );
        canceller.join().unwrap();
        worker.join().unwrap();
        TcpListener::bind(address).unwrap();
    }
}
