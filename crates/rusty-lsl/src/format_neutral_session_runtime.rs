// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crate-private bounded session lifecycle shared by sealed format strategies.

use crate::{StreamHandshakeError, StreamHandshakeIdentity, StreamHandshakeLimits};
use std::io::{ErrorKind, Read};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub(crate) trait SealedSessionStrategy: Copy {
    type Sample;
    type Limits: Copy;
    type RecordError;
    type SessionError;

    fn accept(
        listener: TcpListener,
        identity: &StreamHandshakeIdentity,
        limits: StreamHandshakeLimits,
        format_limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError>;
    fn connect(
        peer: SocketAddr,
        identity: &StreamHandshakeIdentity,
        limits: StreamHandshakeLimits,
        format_limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<TcpStream, StreamHandshakeError>;
    fn write_initialization(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError>;
    fn read_initialization(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError>;
    fn write_record(
        stream: &mut TcpStream,
        sample: &Self::Sample,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError>;
    fn read_record(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<Self::Sample, Self::RecordError>;
    fn io_slice(limits: Self::Limits) -> Duration;
    fn total_deadline(limits: Self::Limits) -> Duration;
    fn handshake_error(error: StreamHandshakeError) -> Self::SessionError;
    fn record_error(index: Option<usize>, error: Self::RecordError) -> Self::SessionError;
    fn cancelled_error() -> Self::SessionError;
    fn deadline_error() -> Self::SessionError;
    fn io_error(kind: ErrorKind) -> Self::SessionError;
    fn trailing_byte(actual: u8) -> Self::SessionError;
}

struct SessionStream(Option<TcpStream>);

impl Drop for SessionStream {
    fn drop(&mut self) {
        terminal_close(self.0.take());
    }
}

pub(crate) fn finish_outlet<F: SealedSessionStrategy>(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    format_limits: F::Limits,
    records: &[F::Sample],
    channel_count: usize,
    cancelled: &AtomicBool,
) -> Result<(SocketAddr, SocketAddr), F::SessionError> {
    let (stream, local, peer) = F::accept(
        listener,
        identity,
        handshake_limits,
        format_limits,
        cancelled,
    )
    .map_err(F::handshake_error)?;
    let mut stream = SessionStream(Some(stream));
    let socket = stream.0.as_mut().expect("session stream is present");
    F::write_initialization(socket, channel_count, format_limits, cancelled)
        .map_err(|error| F::record_error(None, error))?;
    for (index, record) in records.iter().enumerate() {
        F::write_record(socket, record, channel_count, format_limits, cancelled)
            .map_err(|error| F::record_error(Some(index), error))?;
    }
    terminal_close(stream.0.take());
    Ok((local, peer))
}

pub(crate) fn finish_inlet<F: SealedSessionStrategy>(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    format_limits: F::Limits,
    record_count: usize,
    channel_count: usize,
    cancelled: &AtomicBool,
) -> Result<Vec<F::Sample>, F::SessionError> {
    let stream = F::connect(peer, identity, handshake_limits, format_limits, cancelled)
        .map_err(F::handshake_error)?;
    let mut stream = SessionStream(Some(stream));
    let socket = stream.0.as_mut().expect("session stream is present");
    F::read_initialization(socket, channel_count, format_limits, cancelled)
        .map_err(|error| F::record_error(None, error))?;
    let mut records = Vec::with_capacity(record_count);
    for index in 0..record_count {
        records.push(
            F::read_record(socket, channel_count, format_limits, cancelled)
                .map_err(|error| F::record_error(Some(index), error))?,
        );
    }
    require_peer_close::<F>(socket, format_limits, cancelled)?;
    terminal_close(stream.0.take());
    Ok(records)
}

pub(crate) fn terminal_close(stream: Option<TcpStream>) {
    if let Some(stream) = stream {
        let _ = stream.shutdown(Shutdown::Both);
    }
}

fn require_peer_close<F: SealedSessionStrategy>(
    stream: &mut TcpStream,
    limits: F::Limits,
    cancelled: &AtomicBool,
) -> Result<(), F::SessionError> {
    let started = Instant::now();
    let mut byte = [0u8; 1];
    loop {
        if cancelled.load(Ordering::Acquire) {
            return Err(F::cancelled_error());
        }
        let remaining = F::total_deadline(limits)
            .checked_sub(started.elapsed())
            .filter(|remaining| !remaining.is_zero())
            .ok_or_else(F::deadline_error)?;
        stream
            .set_read_timeout(Some(remaining.min(F::io_slice(limits))))
            .map_err(|error| F::io_error(error.kind()))?;
        match stream.read(&mut byte) {
            Ok(0) => return Ok(()),
            Ok(_) => return Err(F::trailing_byte(byte[0])),
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(error) => return Err(F::io_error(error.kind())),
        }
    }
}
