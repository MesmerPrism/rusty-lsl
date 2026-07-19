// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded one- or two-record Float32 outlet/inlet session ownership.

use crate::{
    stream_handshake::{accept_handshake_stream, connect_handshake_stream},
    timestamped_float32_sample_runtime::{
        read_initialization_for_channels, read_record_for_channels,
        write_initialization_for_channels, write_record_for_channels,
    },
    StreamHandshakeError, StreamHandshakeIdentity, StreamHandshakeLimits,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleError,
    TimestampedFloat32SampleLimits, TimestampedSample,
};
use std::io::{ErrorKind, Read};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Explicit role owned by a completed Float32 session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SessionRole {
    /// Accepts one caller-selected listener connection and writes records.
    Outlet,
    /// Connects to one caller-selected peer and reads records.
    Inlet,
}

/// The terminal phase reached by a successful consuming finish.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SessionCompletion {
    /// Handshake, one initialization sequence, records, terminal close, and cleanup completed.
    Complete,
}

/// Invalid preflight input. Preflight performs no socket operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SessionPreflightError {
    /// Only one or two caller records are admitted.
    RecordCount {
        /// Caller-selected count.
        actual: usize,
    },
    /// One outlet record did not contain exactly one channel.
    ChannelCount {
        /// Zero-based caller-record index.
        index: usize,
        /// Caller-declared channel count.
        actual: usize,
    },
    /// A later record did not match the first record's channel count.
    InconsistentChannelCount {
        /// Zero-based caller-record index.
        index: usize,
        /// Channel count selected by the first record.
        expected: usize,
        /// Caller-declared channel count.
        actual: usize,
    },
}

/// Explicit nonzero bounds for a Float32 session shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32SessionLimits {
    max_channels: usize,
    max_records: usize,
}

impl TimestampedFloat32SessionLimits {
    /// Validates channel capacity before record capacity.
    pub const fn new(
        max_channels: usize,
        max_records: usize,
    ) -> Result<Self, TimestampedFloat32SessionLimitError> {
        if max_channels == 0 {
            return Err(TimestampedFloat32SessionLimitError::ZeroMaxChannels);
        }
        if max_records == 0 {
            return Err(TimestampedFloat32SessionLimitError::ZeroMaxRecords);
        }
        Ok(Self {
            max_channels,
            max_records,
        })
    }
    /// Maximum admitted homogeneous channel count.
    pub const fn max_channels(self) -> usize {
        self.max_channels
    }
    /// Maximum admitted caller-record count.
    pub const fn max_records(self) -> usize {
        self.max_records
    }
}

/// Invalid bounded session construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SessionLimitError {
    /// Maximum channels was zero.
    ZeroMaxChannels,
    /// Maximum records was zero.
    ZeroMaxRecords,
}

/// Stable failure from a started session.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SessionError {
    /// Accept/connect and handshake failed before initialization.
    Handshake(StreamHandshakeError),
    /// Initialization or a caller record failed in the sealed codec/transport owner.
    Record {
        /// `None` identifies initialization or terminal-close work.
        index: Option<usize>,
        /// Unchanged codec/transport failure.
        error: TimestampedFloat32SampleError,
    },
    /// The inlet observed a byte after its exact admitted record count.
    TrailingByte {
        /// First trailing byte.
        actual: u8,
    },
}

/// Successful outlet lifecycle report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32OutletSessionReport {
    local: SocketAddr,
    peer: SocketAddr,
    records: usize,
    channels: usize,
    completion: TimestampedFloat32SessionCompletion,
}

impl TimestampedFloat32OutletSessionReport {
    /// Explicit completed role.
    pub const fn role(&self) -> TimestampedFloat32SessionRole {
        TimestampedFloat32SessionRole::Outlet
    }
    /// Bound listener address selected by the caller.
    pub const fn local_address(&self) -> SocketAddr {
        self.local
    }
    /// Accepted peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Exact caller-record count written.
    pub const fn record_count(&self) -> usize {
        self.records
    }
    /// Exact homogeneous channel count written per record.
    pub const fn channel_count(&self) -> usize {
        self.channels
    }
    /// Terminal completion classification.
    pub const fn completion(&self) -> TimestampedFloat32SessionCompletion {
        self.completion
    }
}

/// Successful inlet lifecycle report retaining received record ownership.
#[derive(Debug)]
pub struct TimestampedFloat32InletSessionReport {
    peer: SocketAddr,
    records: Vec<TimestampedSample<f32>>,
    completion: TimestampedFloat32SessionCompletion,
    channels: usize,
}

impl TimestampedFloat32InletSessionReport {
    /// Explicit completed role.
    pub const fn role(&self) -> TimestampedFloat32SessionRole {
        TimestampedFloat32SessionRole::Inlet
    }
    /// Caller-selected peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Ordered received records.
    pub fn records(&self) -> &[TimestampedSample<f32>] {
        &self.records
    }
    /// Exact received caller-record count.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
    /// Exact homogeneous channel count received per record.
    pub const fn channel_count(&self) -> usize {
        self.channels
    }
    /// Terminal completion classification.
    pub const fn completion(&self) -> TimestampedFloat32SessionCompletion {
        self.completion
    }
    /// Consumes the report without copying or reordering records.
    pub fn into_records(self) -> Vec<TimestampedSample<f32>> {
        self.records
    }
}

/// Preflighted outlet owner. Records and identity remain caller-owned borrows.
pub struct TimestampedFloat32OutletSession<'a> {
    activation: TimestampedFloat32SampleActivation,
    listener: Option<TcpListener>,
    identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    records: &'a [TimestampedSample<f32>],
    stream: Option<TcpStream>,
    channel_count: usize,
}

impl<'a> TimestampedFloat32OutletSession<'a> {
    /// Validates the exact one/two-record shape before any socket operation.
    pub fn preflight(
        activation: TimestampedFloat32SampleActivation,
        listener: TcpListener,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        sample_limits: TimestampedFloat32SampleLimits,
        records: &'a [TimestampedSample<f32>],
    ) -> Result<Self, TimestampedFloat32SessionPreflightError> {
        Self::preflight_bounded(
            activation,
            listener,
            identity,
            handshake_limits,
            sample_limits,
            TimestampedFloat32SessionLimits::new(1, 2).unwrap(),
            records,
        )
    }

    /// Validates a bounded homogeneous shape before any socket operation.
    pub fn preflight_bounded(
        activation: TimestampedFloat32SampleActivation,
        listener: TcpListener,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        sample_limits: TimestampedFloat32SampleLimits,
        session_limits: TimestampedFloat32SessionLimits,
        records: &'a [TimestampedSample<f32>],
    ) -> Result<Self, TimestampedFloat32SessionPreflightError> {
        let channel_count = require_outlet_shape(session_limits, records)?;
        Ok(Self {
            activation,
            listener: Some(listener),
            identity,
            handshake_limits,
            sample_limits,
            records,
            stream: None,
            channel_count,
        })
    }

    /// Consumes the owner and executes accept, handshake, initialization once, records, close, and report.
    pub fn finish(
        mut self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedFloat32OutletSessionReport, TimestampedFloat32SessionError> {
        let listener = self
            .listener
            .take()
            .expect("preflighted listener is present");
        let (stream, local, peer) =
            accept_handshake_stream(listener, self.identity, self.handshake_limits, cancelled)
                .map_err(TimestampedFloat32SessionError::Handshake)?;
        self.stream = Some(stream);
        let _ = self.activation;
        let stream = self.stream.as_mut().expect("accepted stream is present");
        write_initialization_for_channels(
            stream,
            self.channel_count,
            self.sample_limits,
            cancelled,
        )
        .map_err(|error| TimestampedFloat32SessionError::Record { index: None, error })?;
        for (index, record) in self.records.iter().enumerate() {
            write_record_for_channels(
                stream,
                record,
                self.channel_count,
                self.sample_limits,
                cancelled,
            )
            .map_err(|error| TimestampedFloat32SessionError::Record {
                index: Some(index),
                error,
            })?;
        }
        terminal_close(self.stream.take());
        Ok(TimestampedFloat32OutletSessionReport {
            local,
            peer,
            records: self.records.len(),
            channels: self.channel_count,
            completion: TimestampedFloat32SessionCompletion::Complete,
        })
    }
}

impl Drop for TimestampedFloat32OutletSession<'_> {
    fn drop(&mut self) {
        terminal_close(self.stream.take());
    }
}

/// Preflighted inlet owner for exactly one or two records.
pub struct TimestampedFloat32InletSession<'a> {
    activation: TimestampedFloat32SampleActivation,
    peer: SocketAddr,
    identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    record_count: usize,
    stream: Option<TcpStream>,
    channel_count: usize,
}

impl<'a> TimestampedFloat32InletSession<'a> {
    /// Validates the exact one/two-record count before connecting.
    pub fn preflight(
        activation: TimestampedFloat32SampleActivation,
        peer: SocketAddr,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        sample_limits: TimestampedFloat32SampleLimits,
        record_count: usize,
    ) -> Result<Self, TimestampedFloat32SessionPreflightError> {
        Self::preflight_bounded(
            activation,
            peer,
            identity,
            handshake_limits,
            sample_limits,
            TimestampedFloat32SessionLimits::new(1, 2).unwrap(),
            1,
            record_count,
        )
    }

    /// Validates an exact bounded inlet shape before connecting.
    pub fn preflight_bounded(
        activation: TimestampedFloat32SampleActivation,
        peer: SocketAddr,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        sample_limits: TimestampedFloat32SampleLimits,
        session_limits: TimestampedFloat32SessionLimits,
        channel_count: usize,
        record_count: usize,
    ) -> Result<Self, TimestampedFloat32SessionPreflightError> {
        require_shape(session_limits, channel_count, record_count)?;
        Ok(Self {
            activation,
            peer,
            identity,
            handshake_limits,
            sample_limits,
            record_count,
            stream: None,
            channel_count,
        })
    }

    /// Consumes the owner and executes connect, handshake, initialization once, records, close, and report.
    pub fn finish(
        mut self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedFloat32InletSessionReport, TimestampedFloat32SessionError> {
        let stream =
            connect_handshake_stream(self.peer, self.identity, self.handshake_limits, cancelled)
                .map_err(TimestampedFloat32SessionError::Handshake)?;
        self.stream = Some(stream);
        let _ = self.activation;
        let stream = self.stream.as_mut().expect("connected stream is present");
        read_initialization_for_channels(stream, self.channel_count, self.sample_limits, cancelled)
            .map_err(|error| TimestampedFloat32SessionError::Record { index: None, error })?;
        let mut records = Vec::with_capacity(self.record_count);
        for index in 0..self.record_count {
            records.push(
                read_record_for_channels(stream, self.channel_count, self.sample_limits, cancelled)
                    .map_err(|error| TimestampedFloat32SessionError::Record {
                        index: Some(index),
                        error,
                    })?,
            );
        }
        require_peer_close(stream, self.sample_limits, cancelled)?;
        terminal_close(self.stream.take());
        Ok(TimestampedFloat32InletSessionReport {
            peer: self.peer,
            records,
            completion: TimestampedFloat32SessionCompletion::Complete,
            channels: self.channel_count,
        })
    }
}

impl Drop for TimestampedFloat32InletSession<'_> {
    fn drop(&mut self) {
        terminal_close(self.stream.take());
    }
}

fn require_shape(
    limits: TimestampedFloat32SessionLimits,
    channel_count: usize,
    record_count: usize,
) -> Result<(), TimestampedFloat32SessionPreflightError> {
    if record_count == 0 || record_count > limits.max_records {
        return Err(TimestampedFloat32SessionPreflightError::RecordCount {
            actual: record_count,
        });
    }
    if channel_count == 0 || channel_count > limits.max_channels {
        return Err(TimestampedFloat32SessionPreflightError::ChannelCount {
            index: 0,
            actual: channel_count,
        });
    }
    Ok(())
}

fn require_outlet_shape(
    limits: TimestampedFloat32SessionLimits,
    records: &[TimestampedSample<f32>],
) -> Result<usize, TimestampedFloat32SessionPreflightError> {
    let channel_count = records
        .first()
        .map(|record| record.sample().declared_channels())
        .unwrap_or(0);
    require_shape(limits, channel_count, records.len())?;
    for (index, record) in records.iter().enumerate().skip(1) {
        let actual = record.sample().declared_channels();
        if actual != channel_count {
            return Err(
                TimestampedFloat32SessionPreflightError::InconsistentChannelCount {
                    index,
                    expected: channel_count,
                    actual,
                },
            );
        }
    }
    Ok(channel_count)
}

fn terminal_close(stream: Option<TcpStream>) {
    if let Some(stream) = stream {
        let _ = stream.shutdown(Shutdown::Both);
    }
}

fn require_peer_close(
    stream: &mut TcpStream,
    limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), TimestampedFloat32SessionError> {
    let started = Instant::now();
    let mut byte = [0u8; 1];
    loop {
        if cancelled.load(Ordering::Acquire) {
            return Err(TimestampedFloat32SessionError::Record {
                index: None,
                error: TimestampedFloat32SampleError::Cancelled,
            });
        }
        let remaining = limits
            .total_deadline()
            .checked_sub(started.elapsed())
            .ok_or(TimestampedFloat32SessionError::Record {
                index: None,
                error: TimestampedFloat32SampleError::Deadline,
            })?;
        stream
            .set_read_timeout(Some(remaining.min(limits.io_slice())))
            .map_err(|error| TimestampedFloat32SessionError::Record {
                index: None,
                error: TimestampedFloat32SampleError::Io(error.kind()),
            })?;
        match stream.read(&mut byte) {
            Ok(0) => return Ok(()),
            Ok(_) => return Err(TimestampedFloat32SessionError::TrailingByte { actual: byte[0] }),
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(error) => {
                return Err(TimestampedFloat32SessionError::Record {
                    index: None,
                    error: TimestampedFloat32SampleError::Io(error.kind()),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_activation::test_capability, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, StreamHandshakeActivation,
    };
    use std::{thread, time::Duration};

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

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "77777777-2222-4333-8444-555555555555".into(),
            "session-host".into(),
            "session-source".into(),
            "session-id".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn sample(timestamp_bits: u64, value_bits: u32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(value_bits)],
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(timestamp_bits)).unwrap(),
            None,
        )
    }

    fn shaped_sample(timestamp_bits: u64, value_bits: &[u32]) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(value_bits.len()).unwrap(),
                value_bits.len(),
                value_bits.iter().copied().map(f32::from_bits).collect(),
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(timestamp_bits)).unwrap(),
            None,
        )
    }

    #[test]
    fn lslc_007b_two_record_session_finishes_reports_and_releases_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let records = [
            sample(0x4092_5220_0000_0001, 0x3fa0_0001),
            sample(0x4092_5b80_0000_0002, 0xc020_0001),
        ];
        let outlet_records = records;
        let worker = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &outlet_records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let report = TimestampedFloat32InletSession::preflight(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            2,
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        assert_eq!(report.role(), TimestampedFloat32SessionRole::Inlet);
        assert_eq!(report.record_count(), 2);
        assert_eq!(
            report.records()[0].sample().values()[0].to_bits(),
            0x3fa0_0001
        );
        assert_eq!(
            report.records()[1].sample().values()[0].to_bits(),
            0xc020_0001
        );
        let outlet = worker.join().unwrap();
        assert_eq!(outlet.role(), TimestampedFloat32SessionRole::Outlet);
        assert_eq!(outlet.local_address(), address);
        assert_eq!(outlet.record_count(), 2);
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn lslc_007b_preflight_rejects_count_without_consuming_listener_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let session_identity = identity();
        let result = TimestampedFloat32OutletSession::preflight(
            activation(),
            listener,
            &session_identity,
            handshake_limits(),
            sample_limits(),
            &[],
        );
        assert!(matches!(
            result,
            Err(TimestampedFloat32SessionPreflightError::RecordCount { actual: 0 })
        ));
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn p2_bounded_shape_session_preserves_channels_records_and_port_reuse() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let records = [
            shaped_sample(0x4092_5220_0000_0001, &[0x3f80_0001, 0xbf80_0002]),
            shaped_sample(0x4092_5b80_0000_0002, &[0x4000_0003, 0xc000_0004]),
            shaped_sample(0x4092_64e0_0000_0003, &[0x4040_0005, 0xc040_0006]),
        ];
        let worker = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, 3).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let received = TimestampedFloat32InletSession::preflight_bounded(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 3).unwrap(),
            2,
            3,
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        assert_eq!(received.channel_count(), 2);
        assert_eq!(received.record_count(), 3);
        assert_eq!(
            received.records()[2].sample().values()[1].to_bits(),
            0xc040_0006
        );
        let sent = worker.join().unwrap();
        assert_eq!(sent.channel_count(), 2);
        assert_eq!(sent.record_count(), 3);
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn p2_shape_preflight_is_bounded_indexed_and_socket_free() {
        assert_eq!(
            TimestampedFloat32SessionLimits::new(0, 1),
            Err(TimestampedFloat32SessionLimitError::ZeroMaxChannels)
        );
        assert_eq!(
            TimestampedFloat32SessionLimits::new(1, 0),
            Err(TimestampedFloat32SessionLimitError::ZeroMaxRecords)
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let records = [
            sample(0x4092_5220_0000_0001, 0x3f80_0001),
            shaped_sample(0x4092_5b80_0000_0002, &[0x4000_0002, 0x4040_0003]),
        ];
        let session_identity = identity();
        let result = TimestampedFloat32OutletSession::preflight_bounded(
            activation(),
            listener,
            &session_identity,
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 2).unwrap(),
            &records,
        );
        assert!(matches!(
            result,
            Err(
                TimestampedFloat32SessionPreflightError::InconsistentChannelCount {
                    index: 1,
                    expected: 1,
                    actual: 2
                }
            )
        ));
        TcpListener::bind(address).unwrap();
    }
}
