// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crate-private bounded session lifecycle shared by sealed format strategies.

use crate::{StreamHandshakeError, StreamHandshakeIdentity, StreamHandshakeLimits};
use core::marker::PhantomData;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SessionShape {
    channels: usize,
    records: usize,
}

impl SessionShape {
    pub(crate) const fn channels(self) -> usize {
        self.channels
    }

    pub(crate) const fn records(self) -> usize {
        self.records
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionShapeError {
    RecordCount {
        actual: usize,
    },
    ChannelCount {
        index: usize,
        actual: usize,
    },
    InconsistentChannelCount {
        index: usize,
        expected: usize,
        actual: usize,
    },
}

pub(crate) fn preflight_shape(
    max_channels: usize,
    max_records: usize,
    channels: usize,
    records: usize,
) -> Result<SessionShape, SessionShapeError> {
    if records == 0 || records > max_records {
        return Err(SessionShapeError::RecordCount { actual: records });
    }
    if channels == 0 || channels > max_channels {
        return Err(SessionShapeError::ChannelCount {
            index: 0,
            actual: channels,
        });
    }
    Ok(SessionShape { channels, records })
}

pub(crate) fn preflight_outlet_shape<T>(
    max_channels: usize,
    max_records: usize,
    records: &[T],
    channels: impl Fn(&T) -> usize,
) -> Result<SessionShape, SessionShapeError> {
    let channel_count = records.first().map(&channels).unwrap_or(0);
    let shape = preflight_shape(max_channels, max_records, channel_count, records.len())?;
    for (index, record) in records.iter().enumerate().skip(1) {
        let actual = channels(record);
        if actual != channel_count {
            return Err(SessionShapeError::InconsistentChannelCount {
                index,
                expected: channel_count,
                actual,
            });
        }
    }
    Ok(shape)
}

#[derive(Clone, Copy)]
pub(crate) struct CompletedOutletSession {
    local: SocketAddr,
    peer: SocketAddr,
    shape: SessionShape,
}

impl CompletedOutletSession {
    pub(crate) const fn local(&self) -> SocketAddr {
        self.local
    }
    pub(crate) const fn peer(&self) -> SocketAddr {
        self.peer
    }
    pub(crate) const fn shape(&self) -> SessionShape {
        self.shape
    }
}

pub(crate) struct CompletedInletSession<T> {
    records: Vec<T>,
    shape: SessionShape,
}

impl<T> CompletedInletSession<T> {
    pub(crate) fn into_records(self) -> Vec<T> {
        self.records
    }
    pub(crate) const fn shape(&self) -> SessionShape {
        self.shape
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TransferError<E> {
    Overrun { declared: usize },
    Session(E),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrematureCompletion {
    pub(crate) completed: usize,
    pub(crate) declared: usize,
}

struct SessionStream(Option<TcpStream>);

impl Drop for SessionStream {
    fn drop(&mut self) {
        terminal_close(self.0.take());
    }
}

pub(crate) struct ConnectedInletSession<F: SealedSessionStrategy> {
    stream: SessionStream,
    peer: SocketAddr,
    limits: F::Limits,
    shape: SessionShape,
    initialized: bool,
    cursor: usize,
    records: Vec<F::Sample>,
    strategy: PhantomData<F>,
}

pub(crate) struct AcceptedOutletSession<F: SealedSessionStrategy> {
    stream: SessionStream,
    local: SocketAddr,
    peer: SocketAddr,
    limits: F::Limits,
    shape: SessionShape,
    initialized: bool,
    cursor: usize,
    strategy: PhantomData<F>,
}

impl<F: SealedSessionStrategy> AcceptedOutletSession<F> {
    pub(crate) const fn local(&self) -> SocketAddr {
        self.local
    }

    pub(crate) const fn peer(&self) -> SocketAddr {
        self.peer
    }

    pub(crate) const fn shape(&self) -> SessionShape {
        self.shape
    }

    pub(crate) fn finish(
        mut self,
        records: &[F::Sample],
        cancelled: &AtomicBool,
    ) -> Result<CompletedOutletSession, F::SessionError> {
        for record in records {
            match self.transfer_next(record, cancelled) {
                Ok(()) => {}
                Err(TransferError::Session(error)) => return Err(error),
                Err(TransferError::Overrun { .. }) => unreachable!("preflighted shape drift"),
            }
        }
        self.require_complete()
            .expect("preflighted records complete the canonical shape");
        terminal_close(self.stream.0.take());
        Ok(CompletedOutletSession {
            local: self.local,
            peer: self.peer,
            shape: self.shape,
        })
    }

    pub(crate) fn require_complete(&self) -> Result<(), PrematureCompletion> {
        if self.cursor == self.shape.records() {
            Ok(())
        } else {
            Err(PrematureCompletion {
                completed: self.cursor,
                declared: self.shape.records(),
            })
        }
    }

    pub(crate) fn transfer_next(
        &mut self,
        record: &F::Sample,
        cancelled: &AtomicBool,
    ) -> Result<(), TransferError<F::SessionError>> {
        if self.cursor == self.shape.records() {
            return Err(TransferError::Overrun {
                declared: self.shape.records(),
            });
        }
        let socket = self.stream.0.as_mut().expect("session stream is present");
        if !self.initialized {
            F::write_initialization(socket, self.shape.channels(), self.limits, cancelled)
                .map_err(|error| TransferError::Session(F::record_error(None, error)))?;
            self.initialized = true;
        }
        F::write_record(
            socket,
            record,
            self.shape.channels(),
            self.limits,
            cancelled,
        )
        .map_err(|error| TransferError::Session(F::record_error(Some(self.cursor), error)))?;
        self.cursor += 1;
        Ok(())
    }

    pub(crate) fn close(mut self) {
        terminal_close(self.stream.0.take());
    }
}

impl<F: SealedSessionStrategy> ConnectedInletSession<F> {
    pub(crate) const fn peer(&self) -> SocketAddr {
        self.peer
    }

    pub(crate) const fn shape(&self) -> SessionShape {
        self.shape
    }

    pub(crate) fn finish(
        mut self,
        cancelled: &AtomicBool,
    ) -> Result<CompletedInletSession<F::Sample>, F::SessionError> {
        while self.cursor < self.shape.records() {
            match self.transfer_next(cancelled) {
                Ok(()) => {}
                Err(TransferError::Session(error)) => return Err(error),
                Err(TransferError::Overrun { .. }) => unreachable!("canonical cursor drift"),
            }
        }
        self.require_complete()
            .expect("the canonical inlet cursor reached its declared count");
        let socket = self.stream.0.as_mut().expect("session stream is present");
        require_peer_close::<F>(socket, self.limits, cancelled)?;
        terminal_close(self.stream.0.take());
        Ok(CompletedInletSession {
            records: self.records,
            shape: self.shape,
        })
    }

    pub(crate) fn require_complete(&self) -> Result<(), PrematureCompletion> {
        if self.cursor == self.shape.records() {
            Ok(())
        } else {
            Err(PrematureCompletion {
                completed: self.cursor,
                declared: self.shape.records(),
            })
        }
    }

    pub(crate) fn transfer_next(
        &mut self,
        cancelled: &AtomicBool,
    ) -> Result<(), TransferError<F::SessionError>> {
        if self.cursor == self.shape.records() {
            return Err(TransferError::Overrun {
                declared: self.shape.records(),
            });
        }
        let socket = self.stream.0.as_mut().expect("session stream is present");
        if !self.initialized {
            F::read_initialization(socket, self.shape.channels(), self.limits, cancelled)
                .map_err(|error| TransferError::Session(F::record_error(None, error)))?;
            self.initialized = true;
        }
        let record = F::read_record(socket, self.shape.channels(), self.limits, cancelled)
            .map_err(|error| TransferError::Session(F::record_error(Some(self.cursor), error)))?;
        self.records.push(record);
        self.cursor += 1;
        Ok(())
    }

    pub(crate) fn close(mut self) {
        terminal_close(self.stream.0.take());
    }
}

pub(crate) fn connect_inlet<F: SealedSessionStrategy>(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    format_limits: F::Limits,
    shape: SessionShape,
    cancelled: &AtomicBool,
) -> Result<ConnectedInletSession<F>, F::SessionError> {
    let stream = F::connect(peer, identity, handshake_limits, format_limits, cancelled)
        .map_err(F::handshake_error)?;
    Ok(ConnectedInletSession {
        stream: SessionStream(Some(stream)),
        peer,
        limits: format_limits,
        shape,
        initialized: false,
        cursor: 0,
        records: Vec::with_capacity(shape.records()),
        strategy: PhantomData,
    })
}

pub(crate) fn finish_outlet<F: SealedSessionStrategy>(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    format_limits: F::Limits,
    records: &[F::Sample],
    shape: SessionShape,
    cancelled: &AtomicBool,
) -> Result<CompletedOutletSession, F::SessionError> {
    accept_outlet::<F>(
        listener,
        identity,
        handshake_limits,
        format_limits,
        shape,
        cancelled,
    )?
    .finish(records, cancelled)
}

pub(crate) fn accept_outlet<F: SealedSessionStrategy>(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    format_limits: F::Limits,
    shape: SessionShape,
    cancelled: &AtomicBool,
) -> Result<AcceptedOutletSession<F>, F::SessionError> {
    let (stream, local, peer) = F::accept(
        listener,
        identity,
        handshake_limits,
        format_limits,
        cancelled,
    )
    .map_err(F::handshake_error)?;
    Ok(AcceptedOutletSession {
        stream: SessionStream(Some(stream)),
        local,
        peer,
        limits: format_limits,
        shape,
        initialized: false,
        cursor: 0,
        strategy: PhantomData,
    })
}

pub(crate) fn finish_inlet<F: SealedSessionStrategy>(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    format_limits: F::Limits,
    shape: SessionShape,
    cancelled: &AtomicBool,
) -> Result<CompletedInletSession<F::Sample>, F::SessionError> {
    connect_inlet::<F>(
        peer,
        identity,
        handshake_limits,
        format_limits,
        shape,
        cancelled,
    )?
    .finish(cancelled)
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
