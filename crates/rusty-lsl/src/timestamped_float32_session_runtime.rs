// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded one- or two-record Float32 outlet/inlet session ownership.

use crate::{
    stream_handshake::{accept_handshake_stream, connect_handshake_stream},
    timestamped_float32_sample_runtime::{
        read_initialization, read_record, write_initialization, write_record,
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
        require_record_count(records.len())?;
        for (index, record) in records.iter().enumerate() {
            let actual = record.sample().declared_channels();
            if actual != 1 {
                return Err(TimestampedFloat32SessionPreflightError::ChannelCount {
                    index,
                    actual,
                });
            }
        }
        Ok(Self {
            activation,
            listener: Some(listener),
            identity,
            handshake_limits,
            sample_limits,
            records,
            stream: None,
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
        write_initialization(stream, self.sample_limits, cancelled)
            .map_err(|error| TimestampedFloat32SessionError::Record { index: None, error })?;
        for (index, record) in self.records.iter().enumerate() {
            write_record(stream, record, self.sample_limits, cancelled).map_err(|error| {
                TimestampedFloat32SessionError::Record {
                    index: Some(index),
                    error,
                }
            })?;
        }
        terminal_close(self.stream.take());
        Ok(TimestampedFloat32OutletSessionReport {
            local,
            peer,
            records: self.records.len(),
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
        require_record_count(record_count)?;
        Ok(Self {
            activation,
            peer,
            identity,
            handshake_limits,
            sample_limits,
            record_count,
            stream: None,
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
        read_initialization(stream, self.sample_limits, cancelled)
            .map_err(|error| TimestampedFloat32SessionError::Record { index: None, error })?;
        let mut records = Vec::with_capacity(self.record_count);
        for index in 0..self.record_count {
            records.push(
                read_record(stream, self.sample_limits, cancelled).map_err(|error| {
                    TimestampedFloat32SessionError::Record {
                        index: Some(index),
                        error,
                    }
                })?,
            );
        }
        require_peer_close(stream, self.sample_limits, cancelled)?;
        terminal_close(self.stream.take());
        Ok(TimestampedFloat32InletSessionReport {
            peer: self.peer,
            records,
            completion: TimestampedFloat32SessionCompletion::Complete,
        })
    }
}

impl Drop for TimestampedFloat32InletSession<'_> {
    fn drop(&mut self) {
        terminal_close(self.stream.take());
    }
}

fn require_record_count(actual: usize) -> Result<(), TimestampedFloat32SessionPreflightError> {
    if matches!(actual, 1 | 2) {
        Ok(())
    } else {
        Err(TimestampedFloat32SessionPreflightError::RecordCount { actual })
    }
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
}
