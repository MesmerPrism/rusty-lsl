// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exactly two ordered one-channel Float32 records over the accepted sample transport.

#[cfg(test)]
use crate::{
    stream_handshake::accept_handshake_stream,
    timestamped_float32_session_runtime::codec::{write_initialization, write_record},
};
use crate::{
    timestamped_float32_session_runtime::{
        TimestampedFloat32InletSession, TimestampedFloat32OutletSession,
        TimestampedFloat32SessionError, TimestampedFloat32SessionPreflightError,
    },
    ChunkLimits, StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedChunk,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleError,
    TimestampedFloat32SampleLimits,
};
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

#[cfg(test)]
use std::{
    io::{ErrorKind, Read},
    net::TcpStream,
    sync::atomic::Ordering,
    time::Instant,
};

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

#[cfg(test)]
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
    TimestampedFloat32OutletSession::preflight(
        activation,
        listener,
        identity,
        handshake_limits,
        sample_limits.sample_limits(),
        chunk.samples(),
    )
    .map_err(map_preflight_error)?
    .finish(cancelled)
    .map(|report| report.local_address())
    .map_err(map_session_error)
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
    let samples = TimestampedFloat32InletSession::preflight(
        activation,
        peer,
        identity,
        handshake_limits,
        sample_limits.sample_limits(),
        REQUIRED_RECORDS,
    )
    .expect("two records always pass session preflight")
    .finish(cancelled)
    .map_err(map_session_error)?
    .into_records();
    Ok(TimestampedChunk::new(ChunkLimits::new(REQUIRED_RECORDS, 1).unwrap(), samples).unwrap())
}

fn map_preflight_error(
    error: TimestampedFloat32SessionPreflightError,
) -> TimestampedFloat32TwoRecordChunkError {
    match error {
        TimestampedFloat32SessionPreflightError::RecordCount { actual } => {
            TimestampedFloat32TwoRecordChunkError::RecordCount { actual }
        }
        TimestampedFloat32SessionPreflightError::ChannelCount { index, actual } => {
            TimestampedFloat32TwoRecordChunkError::Sample {
                index: Some(index),
                error: TimestampedFloat32SampleError::ChannelCount { actual },
            }
        }
        TimestampedFloat32SessionPreflightError::InconsistentChannelCount {
            index, actual, ..
        } => TimestampedFloat32TwoRecordChunkError::Sample {
            index: Some(index),
            error: TimestampedFloat32SampleError::ChannelCount { actual },
        },
    }
}

fn map_session_error(
    error: TimestampedFloat32SessionError,
) -> TimestampedFloat32TwoRecordChunkError {
    match error {
        TimestampedFloat32SessionError::Handshake(error) => {
            TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Handshake(error),
            }
        }
        TimestampedFloat32SessionError::Record { index, error } => {
            TimestampedFloat32TwoRecordChunkError::Sample { index, error }
        }
        TimestampedFloat32SessionError::TrailingByte { actual } => {
            TimestampedFloat32TwoRecordChunkError::TrailingByte { actual }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_activation::test_capability, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, StreamHandshakeActivation, TimestampedSample,
    };
    use std::io::Write;
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
    fn lslc_006d_two_record_order_bits_cleanup_and_immediate_port_reuse() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let first_timestamp = f64::from_bits(0x4092_5220_0000_0001);
        let first_value = f32::from_bits(0x3fa0_0001);
        let second_timestamp = f64::from_bits(0x4092_5b80_0000_0002);
        let second_value = f32::from_bits(0xc020_0001);
        let sent = TimestampedChunk::new(
            ChunkLimits::new(2, 1).unwrap(),
            vec![
                sample(first_timestamp, first_value),
                sample(second_timestamp, second_value),
            ],
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
            first_timestamp.to_bits()
        );
        assert_eq!(
            received.samples()[0].sample().values()[0].to_bits(),
            first_value.to_bits()
        );
        assert_eq!(
            received.samples()[1]
                .raw_source_timestamp()
                .value()
                .to_bits(),
            second_timestamp.to_bits()
        );
        assert_eq!(
            received.samples()[1].sample().values()[0].to_bits(),
            second_value.to_bits()
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
    fn lslc_006d_terminal_deadline_and_cancellation_are_separate_and_cleanup() {
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

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = thread::spawn(move || TcpStream::connect(address).unwrap());
        let (mut stream, _) = listener.accept().unwrap();
        let peer_stream = peer.join().unwrap();
        let cancelled = AtomicBool::new(true);
        assert_eq!(
            require_peer_close(&mut stream, sample_limits(), &cancelled),
            Err(TimestampedFloat32TwoRecordChunkError::Sample {
                index: None,
                error: TimestampedFloat32SampleError::Cancelled,
            })
        );
        drop(peer_stream);
        drop(stream);
    }
}
