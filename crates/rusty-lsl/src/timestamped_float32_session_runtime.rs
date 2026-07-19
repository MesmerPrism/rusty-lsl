// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded one- or two-record Float32 outlet/inlet session ownership.

use crate::format_neutral_session_runtime::{
    finish_inlet, finish_outlet, preflight_outlet_shape, preflight_shape, terminal_close,
    SealedSessionStrategy, SessionShape, SessionShapeError,
};

fn validated_session_shape(channels: usize, records: usize) -> SessionShape {
    preflight_shape(usize::MAX, usize::MAX, channels, records)
        .expect("caller completed session shape preflight")
}
use crate::{
    bounded_fixed_record_transport::{
        read_exact_bounded, write_exact_bounded, BoundedFixedRecordError,
    },
    stream_handshake::{
        accept_handshake_stream, accept_handshake_stream_with_format, connect_handshake_stream,
        connect_handshake_stream_with_format,
    },
    FixedWidthNumericSampleActivation, FixedWidthNumericSampleError,
    FixedWidthNumericSampleLimitError, FixedWidthNumericSampleLimits, FixedWidthNumericValue,
    RawSourceTimestamp, Sample, SampleLimits, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeLimits, StringSampleActivation, StringSampleError, StringSampleLimits,
    StringSampleRecord, TimestampedFloat32SampleActivation, TimestampedFloat32SampleError,
    TimestampedFloat32SampleLimits, TimestampedSample,
};
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// The sole production Float32 framing and initialization owner, sealed beneath the session.
pub(crate) mod codec {
    use super::*;

    #[cfg(test)]
    pub(crate) const RECORD_BYTES: usize = 13;
    pub(crate) const RECORD_MARKER: u8 = 2;
    const INITIALIZATION_TIMESTAMP_BITS: u64 = 0x40fe240c9fbe76c9;
    pub(crate) const INITIALIZATION_VALUE_BITS: [u32; 2] = [0x40800000, 0x40000000];

    #[cfg(test)]
    pub(crate) fn initialization_sample(value_bits: u32) -> TimestampedSample<f32> {
        initialization_sample_for_channels(value_bits, 1)
            .expect("one initialization channel has a representable frame")
    }

    fn initialization_sample_for_channels(
        value_bits: u32,
        channel_count: usize,
    ) -> Result<TimestampedSample<f32>, TimestampedFloat32SampleError> {
        record_bytes(channel_count)?;
        let sample = Sample::new(
            SampleLimits::new(channel_count).map_err(|_| channel_error(channel_count))?,
            channel_count,
            vec![f32::from_bits(value_bits); channel_count],
        )
        .map_err(|_| channel_error(channel_count))?;
        Ok(TimestampedSample::new(
            sample,
            RawSourceTimestamp::new(f64::from_bits(INITIALIZATION_TIMESTAMP_BITS))
                .expect("fixed initialization timestamp is finite"),
            None,
        ))
    }

    pub(crate) fn write_initialization_for_channels(
        stream: &mut TcpStream,
        channel_count: usize,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedFloat32SampleError> {
        for value_bits in INITIALIZATION_VALUE_BITS {
            let sample = initialization_sample_for_channels(value_bits, channel_count)?;
            write_record_for_channels(stream, &sample, channel_count, limits, cancelled)?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn write_initialization(
        stream: &mut TcpStream,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedFloat32SampleError> {
        write_initialization_for_channels(stream, 1, limits, cancelled)
    }

    #[cfg(test)]
    pub(crate) fn read_initialization(
        stream: &mut TcpStream,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedFloat32SampleError> {
        read_initialization_for_channels(stream, 1, limits, cancelled)
    }

    pub(crate) fn read_initialization_for_channels(
        stream: &mut TcpStream,
        channel_count: usize,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedFloat32SampleError> {
        for (index, expected_value) in INITIALIZATION_VALUE_BITS.into_iter().enumerate() {
            let record = read_record_for_channels(stream, channel_count, limits, cancelled)?;
            if record.raw_source_timestamp().value().to_bits() != INITIALIZATION_TIMESTAMP_BITS
                || record
                    .sample()
                    .values()
                    .iter()
                    .any(|value| value.to_bits() != expected_value)
            {
                return Err(TimestampedFloat32SampleError::InvalidInitialization { index });
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn write_record(
        stream: &mut TcpStream,
        sample: &TimestampedSample<f32>,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedFloat32SampleError> {
        write_record_for_channels(stream, sample, 1, limits, cancelled)
    }

    pub(crate) fn write_record_for_channels(
        stream: &mut TcpStream,
        sample: &TimestampedSample<f32>,
        channel_count: usize,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedFloat32SampleError> {
        if sample.sample().declared_channels() != channel_count {
            return Err(channel_error(sample.sample().declared_channels()));
        }
        let mut record = vec![0u8; record_bytes(channel_count)?];
        record[0] = RECORD_MARKER;
        record[1..9].copy_from_slice(&sample.raw_source_timestamp().value().to_le_bytes());
        for (bytes, value) in record[9..]
            .chunks_exact_mut(core::mem::size_of::<f32>())
            .zip(sample.sample().values())
        {
            bytes.copy_from_slice(&value.to_le_bytes());
        }
        write_exact_bounded(
            stream,
            &record,
            limits.io_slice(),
            limits.total_deadline(),
            cancelled,
        )
        .map_err(map_transport_error)
    }

    pub(crate) fn read_record_for_channels(
        stream: &mut TcpStream,
        channel_count: usize,
        limits: TimestampedFloat32SampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedSample<f32>, TimestampedFloat32SampleError> {
        let mut record = vec![0u8; record_bytes(channel_count)?];
        read_exact_bounded(
            stream,
            &mut record,
            limits.io_slice(),
            limits.total_deadline(),
            cancelled,
        )
        .map_err(map_transport_error)?;
        if record[0] != RECORD_MARKER {
            return Err(TimestampedFloat32SampleError::InvalidMarker { actual: record[0] });
        }
        let timestamp = RawSourceTimestamp::new(f64::from_le_bytes(
            record[1..9].try_into().expect("fixed timestamp width"),
        ))
        .map_err(|_| TimestampedFloat32SampleError::InvalidTimestamp)?;
        let values = record[9..]
            .chunks_exact(core::mem::size_of::<f32>())
            .map(|bytes| f32::from_le_bytes(bytes.try_into().expect("fixed Float32 width")))
            .collect();
        let sample = Sample::new(
            SampleLimits::new(channel_count).map_err(|_| channel_error(channel_count))?,
            channel_count,
            values,
        )
        .map_err(|_| channel_error(channel_count))?;
        Ok(TimestampedSample::new(sample, timestamp, None))
    }

    fn record_bytes(channel_count: usize) -> Result<usize, TimestampedFloat32SampleError> {
        if channel_count == 0 {
            return Err(channel_error(channel_count));
        }
        channel_count
            .checked_mul(core::mem::size_of::<f32>())
            .and_then(|values| 9usize.checked_add(values))
            .ok_or_else(|| channel_error(channel_count))
    }

    fn channel_error(actual: usize) -> TimestampedFloat32SampleError {
        TimestampedFloat32SampleError::ChannelCount { actual }
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
}

use codec::{
    read_initialization_for_channels, read_record_for_channels, write_initialization_for_channels,
    write_record_for_channels,
};

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

mod session_format {
    use super::*;

    #[derive(Clone, Copy)]
    pub(super) struct Float32;

    impl SealedSessionStrategy for Float32 {
        type Sample = TimestampedSample<f32>;
        type Limits = TimestampedFloat32SampleLimits;
        type RecordError = TimestampedFloat32SampleError;
        type SessionError = TimestampedFloat32SessionError;

        fn accept(
            listener: TcpListener,
            identity: &StreamHandshakeIdentity,
            limits: StreamHandshakeLimits,
            _format_limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
            accept_handshake_stream(listener, identity, limits, cancelled)
        }
        fn connect(
            peer: SocketAddr,
            identity: &StreamHandshakeIdentity,
            limits: StreamHandshakeLimits,
            _format_limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<TcpStream, StreamHandshakeError> {
            connect_handshake_stream(peer, identity, limits, cancelled)
        }
        fn write_initialization(
            stream: &mut TcpStream,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(), Self::RecordError> {
            write_initialization_for_channels(stream, channels, limits, cancelled)
        }
        fn read_initialization(
            stream: &mut TcpStream,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(), Self::RecordError> {
            read_initialization_for_channels(stream, channels, limits, cancelled)
        }
        fn write_record(
            stream: &mut TcpStream,
            sample: &Self::Sample,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(), Self::RecordError> {
            write_record_for_channels(stream, sample, channels, limits, cancelled)
        }
        fn read_record(
            stream: &mut TcpStream,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<Self::Sample, Self::RecordError> {
            read_record_for_channels(stream, channels, limits, cancelled)
        }
        fn io_slice(limits: Self::Limits) -> Duration {
            limits.io_slice()
        }
        fn total_deadline(limits: Self::Limits) -> Duration {
            limits.total_deadline()
        }
        fn handshake_error(error: StreamHandshakeError) -> Self::SessionError {
            TimestampedFloat32SessionError::Handshake(error)
        }
        fn record_error(index: Option<usize>, error: Self::RecordError) -> Self::SessionError {
            TimestampedFloat32SessionError::Record { index, error }
        }
        fn cancelled_error() -> Self::SessionError {
            Self::record_error(None, TimestampedFloat32SampleError::Cancelled)
        }
        fn deadline_error() -> Self::SessionError {
            Self::record_error(None, TimestampedFloat32SampleError::Deadline)
        }
        fn io_error(kind: ErrorKind) -> Self::SessionError {
            Self::record_error(None, TimestampedFloat32SampleError::Io(kind))
        }
        fn trailing_byte(actual: u8) -> Self::SessionError {
            TimestampedFloat32SessionError::TrailingByte { actual }
        }
    }

    #[derive(Clone, Copy)]
    pub(super) struct StringSample;

    impl SealedSessionStrategy for StringSample {
        type Sample = StringSampleRecord;
        type Limits = StringSampleLimits;
        type RecordError = StringSampleError;
        type SessionError = TimestampedStringSessionError;

        fn accept(
            listener: TcpListener,
            identity: &StreamHandshakeIdentity,
            limits: StreamHandshakeLimits,
            _format_limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
            accept_handshake_stream_with_format(listener, identity, limits, cancelled, 0, false)
        }
        fn connect(
            peer: SocketAddr,
            identity: &StreamHandshakeIdentity,
            limits: StreamHandshakeLimits,
            _format_limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<TcpStream, StreamHandshakeError> {
            connect_handshake_stream_with_format(peer, identity, limits, cancelled, 0, false)
        }
        fn write_initialization(
            stream: &mut TcpStream,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(), Self::RecordError> {
            string_codec::require_shape(channels, 1)
                .map_err(|()| StringSampleError::Io(ErrorKind::InvalidInput))?;
            string_codec::write_initialization(stream, limits, cancelled)
        }
        fn read_initialization(
            stream: &mut TcpStream,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(), Self::RecordError> {
            string_codec::require_shape(channels, 1)
                .map_err(|()| StringSampleError::Io(ErrorKind::InvalidInput))?;
            string_codec::read_initialization(stream, limits, cancelled)
        }
        fn write_record(
            stream: &mut TcpStream,
            sample: &Self::Sample,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<(), Self::RecordError> {
            string_codec::require_shape(channels, 1)
                .map_err(|()| StringSampleError::Io(ErrorKind::InvalidInput))?;
            string_codec::write_record(stream, sample, limits, cancelled)
        }
        fn read_record(
            stream: &mut TcpStream,
            channels: usize,
            limits: Self::Limits,
            cancelled: &AtomicBool,
        ) -> Result<Self::Sample, Self::RecordError> {
            string_codec::require_shape(channels, 1)
                .map_err(|()| StringSampleError::Io(ErrorKind::InvalidInput))?;
            string_codec::read_record(stream, limits, cancelled)
        }
        fn io_slice(limits: Self::Limits) -> Duration {
            limits.io_slice()
        }
        fn total_deadline(limits: Self::Limits) -> Duration {
            limits.total_deadline()
        }
        fn handshake_error(error: StreamHandshakeError) -> Self::SessionError {
            TimestampedStringSessionError::Handshake(error)
        }
        fn record_error(index: Option<usize>, error: Self::RecordError) -> Self::SessionError {
            TimestampedStringSessionError::Record { index, error }
        }
        fn cancelled_error() -> Self::SessionError {
            Self::record_error(None, StringSampleError::Cancelled)
        }
        fn deadline_error() -> Self::SessionError {
            Self::record_error(None, StringSampleError::Deadline)
        }
        fn io_error(kind: ErrorKind) -> Self::SessionError {
            Self::record_error(None, StringSampleError::Io(kind))
        }
        fn trailing_byte(actual: u8) -> Self::SessionError {
            TimestampedStringSessionError::TrailingByte { actual }
        }
    }
}

pub(crate) mod string_codec {
    use super::*;

    const INITIALIZATION_TIMESTAMP: f64 = 123_456.789;
    const MAX_STRING_BYTES: usize = 129;

    pub(crate) fn require_shape(channels: usize, records: usize) -> Result<(), ()> {
        if channels != 1 || records != 1 {
            return Err(());
        }
        Ok(())
    }

    fn map_transport(error: BoundedFixedRecordError) -> StringSampleError {
        match error {
            BoundedFixedRecordError::Cancelled => StringSampleError::Cancelled,
            BoundedFixedRecordError::Deadline => StringSampleError::Deadline,
            BoundedFixedRecordError::Truncated { actual } => {
                StringSampleError::Truncated { actual }
            }
            BoundedFixedRecordError::Io(kind) => StringSampleError::Io(kind),
        }
    }

    pub(crate) fn write(
        stream: &mut TcpStream,
        bytes: &[u8],
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), StringSampleError> {
        write_exact_bounded(
            stream,
            bytes,
            limits.io_slice(),
            limits.total_deadline(),
            cancelled,
        )
        .map_err(map_transport)
    }

    pub(crate) fn read(
        stream: &mut TcpStream,
        bytes: &mut [u8],
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), StringSampleError> {
        read_exact_bounded(
            stream,
            bytes,
            limits.io_slice(),
            limits.total_deadline(),
            cancelled,
        )
        .map_err(map_transport)
    }

    fn encode(record: &StringSampleRecord) -> Vec<u8> {
        let value = record.value().as_bytes();
        let mut bytes = Vec::with_capacity(11 + value.len());
        bytes.push(2);
        bytes.extend_from_slice(&record.timestamp().to_le_bytes());
        bytes.push(1);
        bytes.push(value.len() as u8);
        bytes.extend_from_slice(value);
        bytes
    }

    pub(crate) fn write_initialization(
        stream: &mut TcpStream,
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), StringSampleError> {
        let record = StringSampleRecord::new(INITIALIZATION_TIMESTAMP, String::from("10"))?;
        write_record(stream, &record, limits, cancelled)?;
        write_record(stream, &record, limits, cancelled)
    }

    fn read_header(
        stream: &mut TcpStream,
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(f64, usize), StringSampleError> {
        let mut header = [0; 11];
        read(stream, &mut header, limits, cancelled)?;
        if header[0] != 2 {
            return Err(StringSampleError::InvalidMarker { actual: header[0] });
        }
        if header[9] != 1 {
            return Err(StringSampleError::InvalidLengthForm { actual: header[9] });
        }
        let length = header[10] as usize;
        if length > MAX_STRING_BYTES {
            return Err(StringSampleError::ValueTooLong { actual: length });
        }
        Ok((
            f64::from_le_bytes(header[1..9].try_into().expect("fixed timestamp width")),
            length,
        ))
    }

    pub(crate) fn write_record(
        stream: &mut TcpStream,
        record: &StringSampleRecord,
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), StringSampleError> {
        write(stream, &encode(record), limits, cancelled)
    }

    pub(crate) fn read_record(
        stream: &mut TcpStream,
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<StringSampleRecord, StringSampleError> {
        let (timestamp, length) = read_header(stream, limits, cancelled)?;
        let mut value = vec![0; length];
        read(stream, &mut value, limits, cancelled)?;
        let value = String::from_utf8(value).map_err(|_| StringSampleError::InvalidUtf8)?;
        StringSampleRecord::new(timestamp, value)
    }

    pub(crate) fn read_initialization(
        stream: &mut TcpStream,
        limits: StringSampleLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), StringSampleError> {
        for index in 0..2 {
            let record = read_record(stream, limits, cancelled)?;
            if record.timestamp().to_bits() != INITIALIZATION_TIMESTAMP.to_bits()
                || record.value() != "10"
            {
                return Err(StringSampleError::InvalidInitialization { index });
            }
        }
        Ok(())
    }
}

pub(crate) fn finish_string_outlet_session(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    limits: StringSampleLimits,
    record: &StringSampleRecord,
    cancelled: &AtomicBool,
) -> Result<SocketAddr, StringSampleError> {
    string_codec::require_shape(1, 1)
        .map_err(|()| StringSampleError::Io(ErrorKind::InvalidInput))?;
    let records = core::slice::from_ref(record);
    finish_outlet::<session_format::StringSample>(
        listener,
        identity,
        handshake_limits,
        limits,
        records,
        validated_session_shape(1, 1),
        cancelled,
    )
    .map(|completed| completed.local())
    .map_err(map_string_session_error_legacy)
}

pub(crate) fn finish_string_inlet_session(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<StringSampleRecord, StringSampleError> {
    string_codec::require_shape(1, 1)
        .map_err(|()| StringSampleError::Io(ErrorKind::InvalidInput))?;
    finish_inlet::<session_format::StringSample>(
        peer,
        identity,
        handshake_limits,
        limits,
        validated_session_shape(1, 1),
        cancelled,
    )
    .map(|completed| {
        completed
            .into_records()
            .pop()
            .expect("one preflighted String record")
    })
    .map_err(map_string_session_error_legacy)
}

fn map_string_session_error_legacy(error: TimestampedStringSessionError) -> StringSampleError {
    match error {
        TimestampedStringSessionError::Handshake(error) => StringSampleError::Handshake(error),
        TimestampedStringSessionError::Record { error, .. } => error,
        TimestampedStringSessionError::TrailingByte { .. } => {
            StringSampleError::Io(ErrorKind::InvalidData)
        }
    }
}

/// Explicit nonzero I/O bounds for a bounded Double64 session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedDouble64SessionIoLimits {
    io_slice: Duration,
    total_deadline: Duration,
}

impl TimestampedDouble64SessionIoLimits {
    /// Creates bounded I/O limits.
    pub const fn new(
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, TimestampedDouble64SessionIoLimitError> {
        if io_slice.is_zero() {
            return Err(TimestampedDouble64SessionIoLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(TimestampedDouble64SessionIoLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            io_slice,
            total_deadline,
        })
    }
    /// Per-operation socket wait slice.
    pub const fn io_slice(self) -> Duration {
        self.io_slice
    }
    /// Total deadline for each bounded operation.
    pub const fn total_deadline(self) -> Duration {
        self.total_deadline
    }
}

/// Invalid Double64 session I/O limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedDouble64SessionIoLimitError {
    /// The I/O slice was zero.
    ZeroIoSlice,
    /// The total deadline was zero.
    ZeroTotalDeadline,
}

/// Stable failure from a started Double64 session.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedDouble64SessionError {
    /// Accept/connect and handshake failed before initialization.
    Handshake(StreamHandshakeError),
    /// Initialization, a caller record, or terminal-close work failed.
    Record {
        /// Caller-record index, or `None` for initialization/terminal work.
        index: Option<usize>,
        /// Unchanged subordinate codec/transport failure.
        error: FixedWidthNumericSampleError,
    },
    /// The inlet observed a byte after its exact admitted record count.
    TrailingByte {
        /// First trailing byte.
        actual: u8,
    },
}

#[derive(Clone, Copy)]
struct Double64;

impl Double64 {
    const INITIALIZATION_TIMESTAMP_BITS: u64 = 123_456.789f64.to_bits();
    const INITIALIZATION_VALUE_BITS: [[u64; 2]; 2] = [
        [16_777_221.0f64.to_bits(), (-16_777_222.0f64).to_bits()],
        [16_777_219.0f64.to_bits(), (-16_777_220.0f64).to_bits()],
    ];

    fn record_bytes(channels: usize) -> Result<usize, FixedWidthNumericSampleError> {
        channels
            .checked_mul(8)
            .and_then(|bytes| bytes.checked_add(9))
            .filter(|_| channels != 0)
            .ok_or(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: channels,
            })
    }
    fn initialization(
        channels: usize,
        value_bits: [u64; 2],
    ) -> Result<TimestampedSample<f64>, FixedWidthNumericSampleError> {
        Self::record_bytes(channels)?;
        let sample = Sample::new(
            SampleLimits::new(channels).map_err(|_| {
                FixedWidthNumericSampleError::SequenceFormatMismatch {
                    record: 0,
                    channel: channels,
                }
            })?,
            channels,
            value_bits
                .into_iter()
                .cycle()
                .take(channels)
                .map(f64::from_bits)
                .collect(),
        )
        .map_err(|_| FixedWidthNumericSampleError::SequenceFormatMismatch {
            record: 0,
            channel: channels,
        })?;
        Ok(TimestampedSample::new(
            sample,
            RawSourceTimestamp::new(f64::from_bits(Self::INITIALIZATION_TIMESTAMP_BITS))
                .expect("fixed timestamp is finite"),
            None,
        ))
    }
    fn write(
        stream: &mut TcpStream,
        sample: &TimestampedSample<f64>,
        channels: usize,
        limits: TimestampedDouble64SessionIoLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), FixedWidthNumericSampleError> {
        if sample.sample().declared_channels() != channels {
            return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: sample.sample().declared_channels(),
            });
        }
        let mut bytes = vec![0; Self::record_bytes(channels)?];
        bytes[0] = 2;
        bytes[1..9].copy_from_slice(&sample.raw_source_timestamp().value().to_le_bytes());
        for (target, value) in bytes[9..].chunks_exact_mut(8).zip(sample.sample().values()) {
            target.copy_from_slice(&value.to_le_bytes());
        }
        write_exact_bounded(
            stream,
            &bytes,
            limits.io_slice,
            limits.total_deadline,
            cancelled,
        )
        .map_err(map_double_transport_error)
    }
    fn read(
        stream: &mut TcpStream,
        channels: usize,
        limits: TimestampedDouble64SessionIoLimits,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedSample<f64>, FixedWidthNumericSampleError> {
        let mut bytes = vec![0; Self::record_bytes(channels)?];
        read_exact_bounded(
            stream,
            &mut bytes,
            limits.io_slice,
            limits.total_deadline,
            cancelled,
        )
        .map_err(map_double_transport_error)?;
        if bytes[0] != 2 {
            return Err(FixedWidthNumericSampleError::InvalidMarker { actual: bytes[0] });
        }
        let timestamp = RawSourceTimestamp::new(f64::from_le_bytes(
            bytes[1..9].try_into().expect("fixed timestamp width"),
        ))
        .map_err(|_| FixedWidthNumericSampleError::InvalidTimestamp)?;
        let values = bytes[9..]
            .chunks_exact(8)
            .map(|value| f64::from_le_bytes(value.try_into().expect("fixed Double64 width")))
            .collect();
        let sample = Sample::new(
            SampleLimits::new(channels).map_err(|_| {
                FixedWidthNumericSampleError::SequenceFormatMismatch {
                    record: 0,
                    channel: channels,
                }
            })?,
            channels,
            values,
        )
        .map_err(|_| FixedWidthNumericSampleError::SequenceFormatMismatch {
            record: 0,
            channel: channels,
        })?;
        Ok(TimestampedSample::new(sample, timestamp, None))
    }
}

fn map_double_transport_error(error: BoundedFixedRecordError) -> FixedWidthNumericSampleError {
    match error {
        BoundedFixedRecordError::Cancelled => FixedWidthNumericSampleError::Cancelled,
        BoundedFixedRecordError::Deadline => FixedWidthNumericSampleError::Deadline,
        BoundedFixedRecordError::Truncated { actual } => {
            FixedWidthNumericSampleError::Truncated { actual }
        }
        BoundedFixedRecordError::Io(kind) => FixedWidthNumericSampleError::Io(kind),
    }
}

impl SealedSessionStrategy for Double64 {
    type Sample = TimestampedSample<f64>;
    type Limits = TimestampedDouble64SessionIoLimits;
    type RecordError = FixedWidthNumericSampleError;
    type SessionError = TimestampedDouble64SessionError;
    fn accept(
        listener: TcpListener,
        identity: &StreamHandshakeIdentity,
        limits: StreamHandshakeLimits,
        _format_limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
        accept_handshake_stream_with_format(listener, identity, limits, cancelled, 8, true)
    }
    fn connect(
        peer: SocketAddr,
        identity: &StreamHandshakeIdentity,
        limits: StreamHandshakeLimits,
        _format_limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<TcpStream, StreamHandshakeError> {
        connect_handshake_stream_with_format(peer, identity, limits, cancelled, 8, true)
    }
    fn write_initialization(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError> {
        for values in Self::INITIALIZATION_VALUE_BITS {
            Self::write(
                stream,
                &Self::initialization(channels, values)?,
                channels,
                limits,
                cancelled,
            )?;
        }
        Ok(())
    }
    fn read_initialization(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError> {
        for (index, values) in Self::INITIALIZATION_VALUE_BITS.into_iter().enumerate() {
            let sample = Self::read(stream, channels, limits, cancelled)?;
            if sample.raw_source_timestamp().value().to_bits()
                != Self::INITIALIZATION_TIMESTAMP_BITS
                || sample
                    .sample()
                    .values()
                    .iter()
                    .zip(values.into_iter().cycle())
                    .any(|(value, expected)| value.to_bits() != expected)
            {
                return Err(FixedWidthNumericSampleError::InvalidInitialization { index });
            }
        }
        Ok(())
    }
    fn write_record(
        stream: &mut TcpStream,
        sample: &Self::Sample,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError> {
        Self::write(stream, sample, channels, limits, cancelled)
    }
    fn read_record(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<Self::Sample, Self::RecordError> {
        Self::read(stream, channels, limits, cancelled)
    }
    fn io_slice(limits: Self::Limits) -> Duration {
        limits.io_slice
    }
    fn total_deadline(limits: Self::Limits) -> Duration {
        limits.total_deadline
    }
    fn handshake_error(error: StreamHandshakeError) -> Self::SessionError {
        TimestampedDouble64SessionError::Handshake(error)
    }
    fn record_error(index: Option<usize>, error: Self::RecordError) -> Self::SessionError {
        TimestampedDouble64SessionError::Record { index, error }
    }
    fn cancelled_error() -> Self::SessionError {
        Self::record_error(None, FixedWidthNumericSampleError::Cancelled)
    }
    fn deadline_error() -> Self::SessionError {
        Self::record_error(None, FixedWidthNumericSampleError::Deadline)
    }
    fn io_error(kind: ErrorKind) -> Self::SessionError {
        Self::record_error(None, FixedWidthNumericSampleError::Io(kind))
    }
    fn trailing_byte(actual: u8) -> Self::SessionError {
        TimestampedDouble64SessionError::TrailingByte { actual }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FixedWidthIntegerSessionRecord {
    pub(crate) timestamp: f64,
    pub(crate) values: Vec<FixedWidthNumericValue>,
}

#[derive(Clone, Copy)]
struct FixedWidthIntegerLimits {
    io: FixedWidthNumericSampleLimits,
    template: FixedWidthNumericValue,
}

#[derive(Clone, Copy)]
struct FixedWidthInteger;

impl FixedWidthInteger {
    fn record_bytes(
        limits: FixedWidthIntegerLimits,
        channels: usize,
    ) -> Result<usize, FixedWidthNumericSampleError> {
        channels
            .checked_mul(limits.template.width())
            .and_then(|n| n.checked_add(9))
            .filter(|_| channels != 0)
            .ok_or(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: channels,
            })
    }
    fn write(
        stream: &mut TcpStream,
        record: &FixedWidthIntegerSessionRecord,
        channels: usize,
        limits: FixedWidthIntegerLimits,
        cancelled: &AtomicBool,
    ) -> Result<(), FixedWidthNumericSampleError> {
        if record.values.len() != channels {
            return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: record.values.len(),
            });
        }
        let mut bytes = Vec::with_capacity(Self::record_bytes(limits, channels)?);
        bytes.push(2);
        bytes.extend_from_slice(&record.timestamp.to_le_bytes());
        for (channel, value) in record.values.iter().copied().enumerate() {
            if value.format() != limits.template.format() {
                return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                    record: 0,
                    channel,
                });
            }
            bytes.extend_from_slice(&value.bytes());
        }
        write_exact_bounded(
            stream,
            &bytes,
            limits.io.io_slice(),
            limits.io.total_deadline(),
            cancelled,
        )
        .map_err(map_double_transport_error)
    }
    fn read(
        stream: &mut TcpStream,
        channels: usize,
        limits: FixedWidthIntegerLimits,
        cancelled: &AtomicBool,
    ) -> Result<FixedWidthIntegerSessionRecord, FixedWidthNumericSampleError> {
        let width = limits.template.width();
        let mut bytes = vec![0; Self::record_bytes(limits, channels)?];
        read_exact_bounded(
            stream,
            &mut bytes,
            limits.io.io_slice(),
            limits.io.total_deadline(),
            cancelled,
        )
        .map_err(map_double_transport_error)?;
        if bytes[0] != 2 {
            return Err(FixedWidthNumericSampleError::InvalidMarker { actual: bytes[0] });
        }
        let timestamp = f64::from_le_bytes(bytes[1..9].try_into().expect("fixed timestamp width"));
        if !timestamp.is_finite() {
            return Err(FixedWidthNumericSampleError::InvalidTimestamp);
        }
        Ok(FixedWidthIntegerSessionRecord {
            timestamp,
            values: bytes[9..]
                .chunks_exact(width)
                .map(|b| FixedWidthNumericValue::from_bytes(limits.template, b))
                .collect(),
        })
    }
}

impl SealedSessionStrategy for FixedWidthInteger {
    type Sample = FixedWidthIntegerSessionRecord;
    type Limits = FixedWidthIntegerLimits;
    type RecordError = FixedWidthNumericSampleError;
    type SessionError = TimestampedFixedWidthIntegerSessionError;
    fn accept(
        listener: TcpListener,
        identity: &StreamHandshakeIdentity,
        limits: StreamHandshakeLimits,
        format_limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
        accept_handshake_stream_with_format(
            listener,
            identity,
            limits,
            cancelled,
            format_limits.template.width(),
            false,
        )
    }
    fn connect(
        peer: SocketAddr,
        identity: &StreamHandshakeIdentity,
        limits: StreamHandshakeLimits,
        format_limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<TcpStream, StreamHandshakeError> {
        connect_handshake_stream_with_format(
            peer,
            identity,
            limits,
            cancelled,
            format_limits.template.width(),
            false,
        )
    }
    fn write_initialization(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError> {
        let initializations: Vec<Vec<FixedWidthNumericValue>> = if channels == 1 {
            limits
                .template
                .initialization()
                .into_iter()
                .map(|bytes| vec![FixedWidthNumericValue::from_bytes(limits.template, &bytes)])
                .collect()
        } else {
            limits
                .template
                .sequence_initialization()
                .into_iter()
                .map(Vec::from)
                .collect()
        };
        for values in initializations {
            Self::write(
                stream,
                &FixedWidthIntegerSessionRecord {
                    timestamp: 123_456.789,
                    values,
                },
                channels,
                limits,
                cancelled,
            )?;
        }
        Ok(())
    }
    fn read_initialization(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError> {
        let expected: Vec<Vec<FixedWidthNumericValue>> = if channels == 1 {
            limits
                .template
                .initialization()
                .into_iter()
                .map(|bytes| vec![FixedWidthNumericValue::from_bytes(limits.template, &bytes)])
                .collect()
        } else {
            limits
                .template
                .sequence_initialization()
                .into_iter()
                .map(Vec::from)
                .collect()
        };
        for (index, values) in expected.into_iter().enumerate() {
            let actual = Self::read(stream, channels, limits, cancelled)?;
            if actual.timestamp.to_bits() != 123_456.789f64.to_bits() || actual.values != values {
                return Err(FixedWidthNumericSampleError::InvalidInitialization { index });
            }
        }
        Ok(())
    }
    fn write_record(
        stream: &mut TcpStream,
        sample: &Self::Sample,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<(), Self::RecordError> {
        Self::write(stream, sample, channels, limits, cancelled)
    }
    fn read_record(
        stream: &mut TcpStream,
        channels: usize,
        limits: Self::Limits,
        cancelled: &AtomicBool,
    ) -> Result<Self::Sample, Self::RecordError> {
        Self::read(stream, channels, limits, cancelled)
    }
    fn io_slice(limits: Self::Limits) -> Duration {
        limits.io.io_slice()
    }
    fn total_deadline(limits: Self::Limits) -> Duration {
        limits.io.total_deadline()
    }
    fn handshake_error(error: StreamHandshakeError) -> Self::SessionError {
        TimestampedFixedWidthIntegerSessionError::Handshake(error)
    }
    fn record_error(index: Option<usize>, error: Self::RecordError) -> Self::SessionError {
        TimestampedFixedWidthIntegerSessionError::Record { index, error }
    }
    fn cancelled_error() -> Self::SessionError {
        TimestampedFixedWidthIntegerSessionError::Record {
            index: None,
            error: FixedWidthNumericSampleError::Cancelled,
        }
    }
    fn deadline_error() -> Self::SessionError {
        TimestampedFixedWidthIntegerSessionError::Record {
            index: None,
            error: FixedWidthNumericSampleError::Deadline,
        }
    }
    fn io_error(kind: ErrorKind) -> Self::SessionError {
        TimestampedFixedWidthIntegerSessionError::Record {
            index: None,
            error: FixedWidthNumericSampleError::Io(kind),
        }
    }
    fn trailing_byte(actual: u8) -> Self::SessionError {
        TimestampedFixedWidthIntegerSessionError::TrailingByte { actual }
    }
}

/// Stable failure from a started typed fixed-width integer session.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedFixedWidthIntegerSessionError {
    /// Accept/connect and handshake failed before initialization.
    Handshake(StreamHandshakeError),
    /// Initialization, a caller record, or terminal-close work failed.
    Record {
        /// Caller-record index, or `None` for initialization/terminal work.
        index: Option<usize>,
        /// Unchanged subordinate codec/transport failure.
        error: FixedWidthNumericSampleError,
    },
    /// The inlet observed a byte after its exact admitted record count.
    TrailingByte {
        /// First trailing byte.
        actual: u8,
    },
}

fn map_integer_session_error_legacy(
    error: TimestampedFixedWidthIntegerSessionError,
) -> FixedWidthNumericSampleError {
    match error {
        TimestampedFixedWidthIntegerSessionError::Handshake(error) => {
            FixedWidthNumericSampleError::Handshake(error)
        }
        TimestampedFixedWidthIntegerSessionError::Record { error, .. } => error,
        TimestampedFixedWidthIntegerSessionError::TrailingByte { actual } => {
            FixedWidthNumericSampleError::InvalidMarker { actual }
        }
    }
}

pub(crate) fn finish_fixed_width_integer_outlet_session(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: FixedWidthNumericSampleLimits,
    template: FixedWidthNumericValue,
    records: &[FixedWidthIntegerSessionRecord],
    cancelled: &AtomicBool,
) -> Result<SocketAddr, FixedWidthNumericSampleError> {
    let channels = records.first().map(|r| r.values.len()).unwrap_or(0);
    require_evidenced_integer_shape(channels, records.len())?;
    for (record, item) in records.iter().enumerate() {
        if item.values.len() != channels {
            return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record,
                channel: item.values.len(),
            });
        }
        for (channel, value) in item.values.iter().enumerate() {
            if value.format() != template.format() {
                return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                    record,
                    channel,
                });
            }
        }
    }
    finish_outlet::<FixedWidthInteger>(
        listener,
        identity,
        handshake_limits,
        FixedWidthIntegerLimits {
            io: io_limits,
            template,
        },
        records,
        validated_session_shape(channels, records.len()),
        cancelled,
    )
    .map(|completed| completed.local())
    .map_err(map_integer_session_error_legacy)
}

pub(crate) fn finish_fixed_width_integer_inlet_session(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: FixedWidthNumericSampleLimits,
    template: FixedWidthNumericValue,
    channels: usize,
    records: usize,
    cancelled: &AtomicBool,
) -> Result<Vec<FixedWidthIntegerSessionRecord>, FixedWidthNumericSampleError> {
    require_evidenced_integer_shape(channels, records)?;
    finish_inlet::<FixedWidthInteger>(
        peer,
        identity,
        handshake_limits,
        FixedWidthIntegerLimits {
            io: io_limits,
            template,
        },
        validated_session_shape(channels, records),
        cancelled,
    )
    .map(|completed| completed.into_records())
    .map_err(map_integer_session_error_legacy)
}

fn require_evidenced_integer_shape(
    channels: usize,
    records: usize,
) -> Result<(), FixedWidthNumericSampleError> {
    match (channels, records) {
        (1, 1) | (2, 3) => Ok(()),
        (_, 1 | 3) => Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
            record: 0,
            channel: channels,
        }),
        _ => Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
            record: records,
            channel: 0,
        }),
    }
}

fn require_evidenced_integer_facade_shape(
    channels: usize,
    records: usize,
) -> Result<(), TimestampedFloat32SessionPreflightError> {
    match (channels, records) {
        (1, 1) | (2, 3) => Ok(()),
        (_, 1 | 3) => Err(TimestampedFloat32SessionPreflightError::ChannelCount {
            index: 0,
            actual: channels,
        }),
        _ => Err(TimestampedFloat32SessionPreflightError::RecordCount { actual: records }),
    }
}

macro_rules! fixed_width_integer_session_facade {
    (
        $value:ty, $variant:ident,
        $limits:ident, $limit_error:ident, $io_limits:ident, $io_limit_error:ident,
        $preflight_error:ident, $session_error:ident,
        $outlet_report:ident, $inlet_report:ident,
        $outlet:ident, $inlet:ident
    ) => {
        #[doc = concat!("Bounded homogeneous shape limits shared by the ", stringify!($variant), " session facade.")]
        pub type $limits = TimestampedFloat32SessionLimits;
        #[doc = concat!("Invalid bounded ", stringify!($variant), " session shape limits.")]
        pub type $limit_error = TimestampedFloat32SessionLimitError;
        #[doc = concat!("Bounded I/O limits for the ", stringify!($variant), " session facade.")]
        pub type $io_limits = FixedWidthNumericSampleLimits;
        #[doc = concat!("Invalid bounded I/O limits for the ", stringify!($variant), " session facade.")]
        pub type $io_limit_error = FixedWidthNumericSampleLimitError;
        #[doc = concat!("Socket-free ", stringify!($variant), " shape preflight failure.")]
        pub type $preflight_error = TimestampedFloat32SessionPreflightError;
        #[doc = concat!("Started ", stringify!($variant), " session failure.")]
        pub type $session_error = TimestampedFixedWidthIntegerSessionError;

        #[doc = concat!("Successful ", stringify!($variant), " outlet lifecycle report.")]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $outlet_report {
            local: SocketAddr,
            peer: SocketAddr,
            records: usize,
            channels: usize,
        }
        impl $outlet_report {
            /// Bound listener address selected by the caller.
            pub const fn local_address(&self) -> SocketAddr { self.local }
            /// Accepted peer address.
            pub const fn peer(&self) -> SocketAddr { self.peer }
            /// Exact caller-record count written.
            pub const fn record_count(&self) -> usize { self.records }
            /// Exact homogeneous channel count.
            pub const fn channel_count(&self) -> usize { self.channels }
            /// Terminal completion classification from the shared lifecycle.
            pub const fn completion(&self) -> TimestampedFloat32SessionCompletion {
                TimestampedFloat32SessionCompletion::Complete
            }
        }

        #[doc = concat!("Successful ", stringify!($variant), " inlet lifecycle report.")]
        #[derive(Debug)]
        pub struct $inlet_report {
            peer: SocketAddr,
            records: Vec<TimestampedSample<$value>>,
            channels: usize,
        }
        impl $inlet_report {
            /// Caller-selected peer address.
            pub const fn peer(&self) -> SocketAddr { self.peer }
            /// Ordered received records.
            pub fn records(&self) -> &[TimestampedSample<$value>] { &self.records }
            /// Exact received record count.
            pub fn record_count(&self) -> usize { self.records.len() }
            /// Exact homogeneous channel count.
            pub const fn channel_count(&self) -> usize { self.channels }
            /// Terminal completion classification from the shared lifecycle.
            pub const fn completion(&self) -> TimestampedFloat32SessionCompletion {
                TimestampedFloat32SessionCompletion::Complete
            }
            /// Consumes the report without copying or reordering records.
            pub fn into_records(self) -> Vec<TimestampedSample<$value>> { self.records }
        }

        #[doc = concat!("Preflighted ", stringify!($variant), " outlet facade over the sole session lifecycle.")]
        pub struct $outlet<'a> {
            activation: FixedWidthNumericSampleActivation,
            listener: Option<TcpListener>,
            identity: &'a StreamHandshakeIdentity,
            handshake_limits: StreamHandshakeLimits,
            io_limits: FixedWidthNumericSampleLimits,
            records: Vec<FixedWidthIntegerSessionRecord>,
            channel_count: usize,
        }
        impl<'a> $outlet<'a> {
            /// Validates a bounded homogeneous integer shape before socket I/O.
            pub fn preflight_bounded(
                activation: FixedWidthNumericSampleActivation,
                listener: TcpListener,
                identity: &'a StreamHandshakeIdentity,
                handshake_limits: StreamHandshakeLimits,
                io_limits: FixedWidthNumericSampleLimits,
                session_limits: $limits,
                records: &[TimestampedSample<$value>],
            ) -> Result<Self, $preflight_error> {
                let channel_count = require_outlet_shape_generic(session_limits, records)?;
                require_evidenced_integer_facade_shape(channel_count, records.len())?;
                let records = records.iter().map(|record| FixedWidthIntegerSessionRecord {
                    timestamp: record.raw_source_timestamp().value(),
                    values: record.sample().values().iter().copied().map(FixedWidthNumericValue::$variant).collect(),
                }).collect();
                Ok(Self { activation, listener: Some(listener), identity, handshake_limits, io_limits, records, channel_count })
            }
            /// Consumes the facade through the sole accept/handshake/initialize/records/close lifecycle.
            pub fn finish(mut self, cancelled: &AtomicBool) -> Result<$outlet_report, $session_error> {
                let listener = self.listener.take().expect("preflighted listener is present");
                let _ = self.activation;
                let completed = finish_outlet::<FixedWidthInteger>(
                    listener, self.identity, self.handshake_limits,
                    FixedWidthIntegerLimits { io: self.io_limits, template: FixedWidthNumericValue::$variant(0 as $value) },
                    &self.records, validated_session_shape(self.channel_count, self.records.len()), cancelled,
                )?;
                let shape = completed.shape();
                Ok($outlet_report { local: completed.local(), peer: completed.peer(), records: shape.records(), channels: shape.channels() })
            }
        }

        #[doc = concat!("Preflighted ", stringify!($variant), " inlet facade over the sole session lifecycle.")]
        pub struct $inlet<'a> {
            activation: FixedWidthNumericSampleActivation,
            peer: SocketAddr,
            identity: &'a StreamHandshakeIdentity,
            handshake_limits: StreamHandshakeLimits,
            io_limits: FixedWidthNumericSampleLimits,
            record_count: usize,
            channel_count: usize,
        }
        impl<'a> $inlet<'a> {
            /// Validates an exact bounded integer shape before connecting.
            pub fn preflight_bounded(
                activation: FixedWidthNumericSampleActivation,
                peer: SocketAddr,
                identity: &'a StreamHandshakeIdentity,
                handshake_limits: StreamHandshakeLimits,
                io_limits: FixedWidthNumericSampleLimits,
                session_limits: $limits,
                channel_count: usize,
                record_count: usize,
            ) -> Result<Self, $preflight_error> {
                require_shape(session_limits, channel_count, record_count)?;
                require_evidenced_integer_facade_shape(channel_count, record_count)?;
                Ok(Self { activation, peer, identity, handshake_limits, io_limits, record_count, channel_count })
            }
            /// Consumes the facade through the sole connect/handshake/initialize/records/close lifecycle.
            pub fn finish(self, cancelled: &AtomicBool) -> Result<$inlet_report, $session_error> {
                let _ = self.activation;
                let completed = finish_inlet::<FixedWidthInteger>(
                    self.peer, self.identity, self.handshake_limits,
                    FixedWidthIntegerLimits { io: self.io_limits, template: FixedWidthNumericValue::$variant(0 as $value) },
                    validated_session_shape(self.channel_count, self.record_count), cancelled,
                )?;
                let shape = completed.shape();
                let records = completed.into_records().into_iter().map(|record| {
                    let values = record.values.into_iter().map(|value| match value {
                        FixedWidthNumericValue::$variant(value) => value,
                        _ => unreachable!("sealed strategy retains the selected integer format"),
                    }).collect();
                    let sample = Sample::new(
                        SampleLimits::new(self.channel_count).expect("preflighted nonzero channel count"),
                        self.channel_count, values,
                    ).expect("sealed strategy returns the preflighted channel count");
                    TimestampedSample::new(
                        sample,
                        RawSourceTimestamp::new(record.timestamp).expect("sealed strategy validates finite timestamps"),
                        None,
                    )
                }).collect();
                Ok($inlet_report { peer: self.peer, records, channels: shape.channels() })
            }
        }
    };
}

fixed_width_integer_session_facade!(
    i32,
    Int32,
    TimestampedInt32SessionLimits,
    TimestampedInt32SessionLimitError,
    TimestampedInt32SessionIoLimits,
    TimestampedInt32SessionIoLimitError,
    TimestampedInt32SessionPreflightError,
    TimestampedInt32SessionError,
    TimestampedInt32OutletSessionReport,
    TimestampedInt32InletSessionReport,
    TimestampedInt32OutletSession,
    TimestampedInt32InletSession
);
fixed_width_integer_session_facade!(
    i16,
    Int16,
    TimestampedInt16SessionLimits,
    TimestampedInt16SessionLimitError,
    TimestampedInt16SessionIoLimits,
    TimestampedInt16SessionIoLimitError,
    TimestampedInt16SessionPreflightError,
    TimestampedInt16SessionError,
    TimestampedInt16OutletSessionReport,
    TimestampedInt16InletSessionReport,
    TimestampedInt16OutletSession,
    TimestampedInt16InletSession
);
fixed_width_integer_session_facade!(
    i8,
    Int8,
    TimestampedInt8SessionLimits,
    TimestampedInt8SessionLimitError,
    TimestampedInt8SessionIoLimits,
    TimestampedInt8SessionIoLimitError,
    TimestampedInt8SessionPreflightError,
    TimestampedInt8SessionError,
    TimestampedInt8OutletSessionReport,
    TimestampedInt8InletSessionReport,
    TimestampedInt8OutletSession,
    TimestampedInt8InletSession
);

/// Closed shape limits for the concrete one-channel, one-record String facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedStringSessionLimits;
impl TimestampedStringSessionLimits {
    /// Admits only the evidenced one-channel, one-record String shape.
    pub const fn new(
        channel_count: usize,
        record_count: usize,
    ) -> Result<Self, TimestampedStringSessionLimitError> {
        if record_count != 1 {
            return Err(TimestampedStringSessionLimitError::RecordCount {
                actual: record_count,
            });
        }
        if channel_count != 1 {
            return Err(TimestampedStringSessionLimitError::ChannelCount {
                actual: channel_count,
            });
        }
        Ok(Self)
    }
    /// Exact admitted channel count.
    pub const fn channel_count(self) -> usize {
        1
    }
    /// Exact admitted caller-record count.
    pub const fn record_count(self) -> usize {
        1
    }
}

/// Invalid closed String session shape limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedStringSessionLimitError {
    /// The caller-record count was not exactly one.
    RecordCount {
        /// Supplied caller-record count.
        actual: usize,
    },
    /// The channel count was not exactly one.
    ChannelCount {
        /// Supplied channel count.
        actual: usize,
    },
}

/// Socket-free String shape preflight failure.
pub type TimestampedStringSessionPreflightError = TimestampedStringSessionLimitError;

/// Role completed by a bounded String session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedStringSessionRole {
    /// Accepted and wrote the caller record.
    Outlet,
    /// Connected and read the caller record.
    Inlet,
}

/// Terminal classification of a successful bounded String session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedStringSessionCompletion {
    /// The exact record count completed and the peer closed cleanly.
    Complete,
}

/// Stable failure from a started String session.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedStringSessionError {
    /// Accept/connect and handshake failed before initialization.
    Handshake(StreamHandshakeError),
    /// Initialization, the caller record, or terminal-close work failed.
    Record {
        /// Caller-record index, or `None` for initialization/terminal work.
        index: Option<usize>,
        /// Unchanged subordinate String framing or transport failure.
        error: StringSampleError,
    },
    /// The inlet observed a byte after its exact admitted record count.
    TrailingByte {
        /// First trailing byte.
        actual: u8,
    },
}

/// Successful String outlet lifecycle report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedStringOutletSessionReport {
    local: SocketAddr,
    peer: SocketAddr,
}
impl TimestampedStringOutletSessionReport {
    /// Bound listener address selected by the caller.
    pub const fn local_address(&self) -> SocketAddr {
        self.local
    }
    /// Accepted peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Completed session role.
    pub const fn role(&self) -> TimestampedStringSessionRole {
        TimestampedStringSessionRole::Outlet
    }
    /// Exact caller-record count written.
    pub const fn record_count(&self) -> usize {
        1
    }
    /// Exact homogeneous channel count.
    pub const fn channel_count(&self) -> usize {
        1
    }
    /// Terminal completion classification from the shared lifecycle.
    pub const fn completion(&self) -> TimestampedStringSessionCompletion {
        TimestampedStringSessionCompletion::Complete
    }
}

/// Successful String inlet lifecycle report.
#[derive(Debug)]
pub struct TimestampedStringInletSessionReport {
    peer: SocketAddr,
    records: Vec<StringSampleRecord>,
}
impl TimestampedStringInletSessionReport {
    /// Caller-selected peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Completed session role.
    pub const fn role(&self) -> TimestampedStringSessionRole {
        TimestampedStringSessionRole::Inlet
    }
    /// The exact received caller record.
    pub fn records(&self) -> &[StringSampleRecord] {
        &self.records
    }
    /// Exact received record count.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
    /// Exact homogeneous channel count.
    pub const fn channel_count(&self) -> usize {
        1
    }
    /// Terminal completion classification from the shared lifecycle.
    pub const fn completion(&self) -> TimestampedStringSessionCompletion {
        TimestampedStringSessionCompletion::Complete
    }
    /// Consumes the report without copying, reallocating, or reordering the String record.
    pub fn into_records(self) -> Vec<StringSampleRecord> {
        self.records
    }
}

/// Preflighted concrete String outlet facade over the sole session lifecycle.
pub struct TimestampedStringOutletSession<'a> {
    activation: StringSampleActivation,
    listener: Option<TcpListener>,
    identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: StringSampleLimits,
    records: &'a [StringSampleRecord],
}
impl<'a> TimestampedStringOutletSession<'a> {
    /// Validates the exact one-channel, one-record, 0..=129-byte shape before socket I/O.
    pub fn preflight_bounded(
        activation: StringSampleActivation,
        listener: TcpListener,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        io_limits: StringSampleLimits,
        session_limits: TimestampedStringSessionLimits,
        records: &'a [StringSampleRecord],
    ) -> Result<Self, TimestampedStringSessionPreflightError> {
        require_evidenced_string_shape(session_limits, 1, records.len())?;
        debug_assert!(records.iter().all(|record| record.value().len() <= 129));
        Ok(Self {
            activation,
            listener: Some(listener),
            identity,
            handshake_limits,
            io_limits,
            records,
        })
    }
    /// Consumes the facade through the sole accept/handshake/initialize/record/close lifecycle.
    pub fn finish(
        mut self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedStringOutletSessionReport, TimestampedStringSessionError> {
        let listener = self
            .listener
            .take()
            .expect("preflighted listener is present");
        let _ = self.activation;
        let completed = finish_outlet::<session_format::StringSample>(
            listener,
            self.identity,
            self.handshake_limits,
            self.io_limits,
            self.records,
            validated_session_shape(1, 1),
            cancelled,
        )?;
        Ok(TimestampedStringOutletSessionReport {
            local: completed.local(),
            peer: completed.peer(),
        })
    }
}

/// Preflighted concrete String inlet facade over the sole session lifecycle.
pub struct TimestampedStringInletSession<'a> {
    activation: StringSampleActivation,
    peer: SocketAddr,
    identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: StringSampleLimits,
}
impl<'a> TimestampedStringInletSession<'a> {
    /// Validates the exact one-channel, one-record shape before connecting.
    pub fn preflight_bounded(
        activation: StringSampleActivation,
        peer: SocketAddr,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        io_limits: StringSampleLimits,
        session_limits: TimestampedStringSessionLimits,
        channel_count: usize,
        record_count: usize,
    ) -> Result<Self, TimestampedStringSessionPreflightError> {
        require_evidenced_string_shape(session_limits, channel_count, record_count)?;
        Ok(Self {
            activation,
            peer,
            identity,
            handshake_limits,
            io_limits,
        })
    }
    /// Consumes the facade through the sole connect/handshake/initialize/record/close lifecycle.
    pub fn finish(
        self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedStringInletSessionReport, TimestampedStringSessionError> {
        let _ = self.activation;
        let completed = finish_inlet::<session_format::StringSample>(
            self.peer,
            self.identity,
            self.handshake_limits,
            self.io_limits,
            validated_session_shape(1, 1),
            cancelled,
        )?;
        Ok(TimestampedStringInletSessionReport {
            peer: self.peer,
            records: completed.into_records(),
        })
    }
}

fn require_evidenced_string_shape(
    limits: TimestampedStringSessionLimits,
    channel_count: usize,
    record_count: usize,
) -> Result<(), TimestampedStringSessionPreflightError> {
    debug_assert_eq!(limits.channel_count(), 1);
    debug_assert_eq!(limits.record_count(), 1);
    preflight_shape(1, 1, channel_count, record_count)
        .map(|_| ())
        .map_err(|error| match error {
            SessionShapeError::RecordCount { actual } => {
                TimestampedStringSessionPreflightError::RecordCount { actual }
            }
            SessionShapeError::ChannelCount { actual, .. } => {
                TimestampedStringSessionPreflightError::ChannelCount { actual }
            }
            SessionShapeError::InconsistentChannelCount { .. } => {
                unreachable!("String inlet preflight has no record slice")
            }
        })
}

/// Bounded homogeneous shape limits shared by the Double64 session facade.
pub type TimestampedDouble64SessionLimits = TimestampedFloat32SessionLimits;
/// Invalid bounded Double64 session shape limits.
pub type TimestampedDouble64SessionLimitError = TimestampedFloat32SessionLimitError;
/// Socket-free Double64 shape preflight failure.
pub type TimestampedDouble64SessionPreflightError = TimestampedFloat32SessionPreflightError;

/// Successful Double64 outlet lifecycle report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedDouble64OutletSessionReport {
    local: SocketAddr,
    peer: SocketAddr,
    records: usize,
    channels: usize,
}
impl TimestampedDouble64OutletSessionReport {
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
    /// Exact homogeneous channel count.
    pub const fn channel_count(&self) -> usize {
        self.channels
    }
    /// Terminal completion classification from the shared lifecycle.
    pub const fn completion(&self) -> TimestampedFloat32SessionCompletion {
        TimestampedFloat32SessionCompletion::Complete
    }
}

/// Successful Double64 inlet lifecycle report.
#[derive(Debug)]
pub struct TimestampedDouble64InletSessionReport {
    peer: SocketAddr,
    records: Vec<TimestampedSample<f64>>,
    channels: usize,
}
impl TimestampedDouble64InletSessionReport {
    /// Caller-selected peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Ordered received records.
    pub fn records(&self) -> &[TimestampedSample<f64>] {
        &self.records
    }
    /// Exact received record count.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
    /// Exact homogeneous channel count.
    pub const fn channel_count(&self) -> usize {
        self.channels
    }
    /// Terminal completion classification from the shared lifecycle.
    pub const fn completion(&self) -> TimestampedFloat32SessionCompletion {
        TimestampedFloat32SessionCompletion::Complete
    }
    /// Consumes the report without copying or reordering records.
    pub fn into_records(self) -> Vec<TimestampedSample<f64>> {
        self.records
    }
}

/// Preflighted Double64 outlet facade over the sole session lifecycle.
pub struct TimestampedDouble64OutletSession<'a> {
    activation: FixedWidthNumericSampleActivation,
    listener: Option<TcpListener>,
    identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: TimestampedDouble64SessionIoLimits,
    records: &'a [TimestampedSample<f64>],
    channel_count: usize,
}
impl<'a> TimestampedDouble64OutletSession<'a> {
    /// Validates a bounded homogeneous Double64 shape before socket I/O.
    pub fn preflight_bounded(
        activation: FixedWidthNumericSampleActivation,
        listener: TcpListener,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        io_limits: TimestampedDouble64SessionIoLimits,
        session_limits: TimestampedDouble64SessionLimits,
        records: &'a [TimestampedSample<f64>],
    ) -> Result<Self, TimestampedDouble64SessionPreflightError> {
        let channel_count = require_outlet_shape_generic(session_limits, records)?;
        require_evidenced_double64_shape(channel_count, records.len())?;
        Ok(Self {
            activation,
            listener: Some(listener),
            identity,
            handshake_limits,
            io_limits,
            records,
            channel_count,
        })
    }
    /// Consumes the facade through the sole accept/handshake/initialize/records/close lifecycle.
    pub fn finish(
        mut self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedDouble64OutletSessionReport, TimestampedDouble64SessionError> {
        let listener = self
            .listener
            .take()
            .expect("preflighted listener is present");
        let _ = self.activation;
        let completed = finish_outlet::<Double64>(
            listener,
            self.identity,
            self.handshake_limits,
            self.io_limits,
            self.records,
            validated_session_shape(self.channel_count, self.records.len()),
            cancelled,
        )?;
        let shape = completed.shape();
        Ok(TimestampedDouble64OutletSessionReport {
            local: completed.local(),
            peer: completed.peer(),
            records: shape.records(),
            channels: shape.channels(),
        })
    }
}

/// Preflighted Double64 inlet facade over the sole session lifecycle.
pub struct TimestampedDouble64InletSession<'a> {
    activation: FixedWidthNumericSampleActivation,
    peer: SocketAddr,
    identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: TimestampedDouble64SessionIoLimits,
    record_count: usize,
    channel_count: usize,
}
impl<'a> TimestampedDouble64InletSession<'a> {
    /// Validates an exact bounded Double64 shape before connecting.
    pub fn preflight_bounded(
        activation: FixedWidthNumericSampleActivation,
        peer: SocketAddr,
        identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        io_limits: TimestampedDouble64SessionIoLimits,
        session_limits: TimestampedDouble64SessionLimits,
        channel_count: usize,
        record_count: usize,
    ) -> Result<Self, TimestampedDouble64SessionPreflightError> {
        require_shape(session_limits, channel_count, record_count)?;
        require_evidenced_double64_shape(channel_count, record_count)?;
        Ok(Self {
            activation,
            peer,
            identity,
            handshake_limits,
            io_limits,
            record_count,
            channel_count,
        })
    }
    /// Consumes the facade through the sole connect/handshake/initialize/records/close lifecycle.
    pub fn finish(
        self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedDouble64InletSessionReport, TimestampedDouble64SessionError> {
        let _ = self.activation;
        let completed = finish_inlet::<Double64>(
            self.peer,
            self.identity,
            self.handshake_limits,
            self.io_limits,
            validated_session_shape(self.channel_count, self.record_count),
            cancelled,
        )?;
        let shape = completed.shape();
        Ok(TimestampedDouble64InletSessionReport {
            peer: self.peer,
            records: completed.into_records(),
            channels: shape.channels(),
        })
    }
}

fn require_evidenced_double64_shape(
    channel_count: usize,
    record_count: usize,
) -> Result<(), TimestampedDouble64SessionPreflightError> {
    match (channel_count, record_count) {
        (1, 1) | (2, 3) => Ok(()),
        (_, 1 | 3) => Err(TimestampedDouble64SessionPreflightError::ChannelCount {
            index: 0,
            actual: channel_count,
        }),
        _ => Err(TimestampedDouble64SessionPreflightError::RecordCount {
            actual: record_count,
        }),
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
        let _ = self.activation;
        let completed = finish_outlet::<session_format::Float32>(
            listener,
            self.identity,
            self.handshake_limits,
            self.sample_limits,
            self.records,
            validated_session_shape(self.channel_count, self.records.len()),
            cancelled,
        )?;
        let shape = completed.shape();
        Ok(TimestampedFloat32OutletSessionReport {
            local: completed.local(),
            peer: completed.peer(),
            records: shape.records(),
            channels: shape.channels(),
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
        self,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedFloat32InletSessionReport, TimestampedFloat32SessionError> {
        let _ = self.activation;
        let completed = finish_inlet::<session_format::Float32>(
            self.peer,
            self.identity,
            self.handshake_limits,
            self.sample_limits,
            validated_session_shape(self.channel_count, self.record_count),
            cancelled,
        )?;
        let shape = completed.shape();
        Ok(TimestampedFloat32InletSessionReport {
            peer: self.peer,
            records: completed.into_records(),
            completion: TimestampedFloat32SessionCompletion::Complete,
            channels: shape.channels(),
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
    preflight_shape(
        limits.max_channels,
        limits.max_records,
        channel_count,
        record_count,
    )
    .map(|_| ())
    .map_err(map_shape_error)
}

fn require_outlet_shape(
    limits: TimestampedFloat32SessionLimits,
    records: &[TimestampedSample<f32>],
) -> Result<usize, TimestampedFloat32SessionPreflightError> {
    require_outlet_shape_generic(limits, records)
}

fn require_outlet_shape_generic<T>(
    limits: TimestampedFloat32SessionLimits,
    records: &[TimestampedSample<T>],
) -> Result<usize, TimestampedFloat32SessionPreflightError> {
    preflight_outlet_shape(limits.max_channels, limits.max_records, records, |record| {
        record.sample().declared_channels()
    })
    .map(|shape| shape.channels())
    .map_err(map_shape_error)
}

fn map_shape_error(error: SessionShapeError) -> TimestampedFloat32SessionPreflightError {
    match error {
        SessionShapeError::RecordCount { actual } => {
            TimestampedFloat32SessionPreflightError::RecordCount { actual }
        }
        SessionShapeError::ChannelCount { index, actual } => {
            TimestampedFloat32SessionPreflightError::ChannelCount { index, actual }
        }
        SessionShapeError::InconsistentChannelCount {
            index,
            expected,
            actual,
        } => TimestampedFloat32SessionPreflightError::InconsistentChannelCount {
            index,
            expected,
            actual,
        },
    }
}

#[cfg(test)]
fn terminal_read_timeout(
    elapsed: std::time::Duration,
    limits: TimestampedFloat32SampleLimits,
) -> Result<std::time::Duration, TimestampedFloat32SessionError> {
    let remaining = limits
        .total_deadline()
        .checked_sub(elapsed)
        .filter(|remaining| !remaining.is_zero())
        .ok_or(TimestampedFloat32SessionError::Record {
            index: None,
            error: TimestampedFloat32SampleError::Deadline,
        })?;
    Ok(remaining.min(limits.io_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_activation::test_capability, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, StreamHandshakeActivation,
    };
    use std::{thread, time::Duration};

    #[test]
    fn candidate_string_session_shape_is_closed_before_io() {
        assert_eq!(string_codec::require_shape(1, 1), Ok(()));
        assert_eq!(string_codec::require_shape(0, 1), Err(()));
        assert_eq!(string_codec::require_shape(2, 1), Err(()));
        assert_eq!(string_codec::require_shape(1, 0), Err(()));
        assert_eq!(string_codec::require_shape(1, 2), Err(()));
        assert_eq!(
            StringSampleRecord::new(1.0, "x".repeat(130)),
            Err(StringSampleError::ValueTooLong { actual: 130 })
        );
    }

    fn activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn double64_activation() -> FixedWidthNumericSampleActivation {
        FixedWidthNumericSampleActivation::new(
            test_capability(RuntimeModule::FixedWidthNumericSample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn string_activation() -> StringSampleActivation {
        StringSampleActivation::new(
            test_capability(RuntimeModule::StringSample),
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

    #[test]
    fn p5_integer_session_rejects_unsupported_shape_before_socket_io() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let result = finish_fixed_width_integer_outlet_session(
            listener,
            &identity(),
            handshake_limits(),
            FixedWidthNumericSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap(),
            FixedWidthNumericValue::Int32(0),
            &[],
            &AtomicBool::new(false),
        );
        assert_eq!(
            result,
            Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: 0,
            })
        );
        TcpListener::bind(address).expect("preflight released the untouched listener");
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

    fn double64_sample(timestamp_bits: u64, value_bits: &[u64]) -> TimestampedSample<f64> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(value_bits.len()).unwrap(),
                value_bits.len(),
                value_bits.iter().copied().map(f64::from_bits).collect(),
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(timestamp_bits)).unwrap(),
            None,
        )
    }

    #[test]
    fn lslc_007l_string_session_preserves_all_byte_boundaries_utf8_reports_and_port_reuse() {
        let values = [
            String::new(),
            String::from("x"),
            "x".repeat(128),
            "x".repeat(129),
            String::from("μ界🦀"),
            "μ".repeat(64),
            "μ".repeat(64) + "a",
        ];
        for (case, value) in values.into_iter().enumerate() {
            let timestamp = f64::from_bits(0x4093_4a00_0000_0001 + case as u64);
            assert!(value.len() <= 129);
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let expected = value.clone();
            let worker = thread::spawn(move || {
                let records = [StringSampleRecord::new(timestamp, value).unwrap()];
                TimestampedStringOutletSession::preflight_bounded(
                    string_activation(),
                    listener,
                    &identity(),
                    handshake_limits(),
                    StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                        .unwrap(),
                    TimestampedStringSessionLimits::new(1, 1).unwrap(),
                    &records,
                )
                .unwrap()
                .finish(&AtomicBool::new(false))
                .unwrap()
            });
            let received = TimestampedStringInletSession::preflight_bounded(
                string_activation(),
                address,
                &identity(),
                handshake_limits(),
                StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                TimestampedStringSessionLimits::new(1, 1).unwrap(),
                1,
                1,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap();
            assert_eq!(
                received.records()[0].timestamp().to_bits(),
                timestamp.to_bits()
            );
            assert_eq!(received.records()[0].value(), expected);
            let received_role = received.role();
            assert_eq!(
                received.completion(),
                TimestampedStringSessionCompletion::Complete
            );
            let allocation = received.records()[0].value().as_ptr();
            let records = received.into_records();
            assert_eq!(records[0].value().as_ptr(), allocation);
            let sent = worker.join().unwrap();
            assert_eq!(sent.local_address(), address);
            assert_eq!(sent.role(), TimestampedStringSessionRole::Outlet);
            assert_eq!(received_role, TimestampedStringSessionRole::Inlet);
            assert_eq!(sent.record_count(), 1);
            assert_eq!(sent.channel_count(), 1);
            assert_eq!(
                sent.completion(),
                TimestampedStringSessionCompletion::Complete
            );
            TcpListener::bind(address).expect("completed facade released listener port");
        }
    }

    #[test]
    fn lslc_007l_string_session_rejects_shape_value_and_nonfinite_timestamp_before_io() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let records: [StringSampleRecord; 0] = [];
        assert!(matches!(
            TimestampedStringOutletSession::preflight_bounded(
                string_activation(),
                listener,
                &identity(),
                handshake_limits(),
                StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                TimestampedStringSessionLimits::new(1, 1).unwrap(),
                &records,
            ),
            Err(TimestampedStringSessionPreflightError::RecordCount { actual: 0 })
        ));
        TcpListener::bind(address).expect("shape preflight performed no accept");
        assert_eq!(
            StringSampleRecord::new(1.0, "x".repeat(130)),
            Err(StringSampleError::ValueTooLong { actual: 130 })
        );
        assert_eq!(
            StringSampleRecord::new(f64::NAN, String::new()),
            Err(StringSampleError::InvalidTimestamp)
        );
        assert_eq!(
            TimestampedStringSessionLimits::new(0, 0),
            Err(TimestampedStringSessionLimitError::RecordCount { actual: 0 })
        );
        assert_eq!(
            TimestampedStringSessionLimits::new(0, 1),
            Err(TimestampedStringSessionLimitError::ChannelCount { actual: 0 })
        );
        assert_eq!(
            TimestampedStringSessionLimits::new(1, 2),
            Err(TimestampedStringSessionLimitError::RecordCount { actual: 2 })
        );
    }

    #[test]
    fn lslc_007l_string_session_preserves_indexed_trailing_and_legacy_error_projection() {
        let initialization = StringSampleError::InvalidInitialization { index: 1 };
        assert_eq!(
            map_string_session_error_legacy(TimestampedStringSessionError::Record {
                index: None,
                error: initialization,
            }),
            StringSampleError::InvalidInitialization { index: 1 }
        );
        assert_eq!(
            TimestampedStringSessionError::Record {
                index: Some(0),
                error: StringSampleError::InvalidUtf8,
            },
            TimestampedStringSessionError::Record {
                index: Some(0),
                error: StringSampleError::InvalidUtf8,
            }
        );
        assert_eq!(
            map_string_session_error_legacy(TimestampedStringSessionError::TrailingByte {
                actual: 0xa5,
            }),
            StringSampleError::Io(ErrorKind::InvalidData)
        );
    }

    macro_rules! integer_facade_host_test {
        ($name:ident, $value:ty, $outlet:ident, $inlet:ident, $limits:ident, $values:expr) => {
            #[test]
            fn $name() {
                let listener = TcpListener::bind("127.0.0.1:0").unwrap();
                let address = listener.local_addr().unwrap();
                let values: [[$value; 2]; 3] = $values;
                let records: Vec<_> = values
                    .into_iter()
                    .enumerate()
                    .map(|(index, values)| {
                        TimestampedSample::new(
                            Sample::new(SampleLimits::new(2).unwrap(), 2, values.to_vec()).unwrap(),
                            RawSourceTimestamp::new(1000.25 + index as f64).unwrap(),
                            None,
                        )
                    })
                    .collect();
                let worker = thread::spawn(move || {
                    $outlet::preflight_bounded(
                        double64_activation(),
                        listener,
                        &identity(),
                        handshake_limits(),
                        FixedWidthNumericSampleLimits::new(
                            Duration::from_millis(5),
                            Duration::from_secs(1),
                        )
                        .unwrap(),
                        $limits::new(2, 3).unwrap(),
                        &records,
                    )
                    .unwrap()
                    .finish(&AtomicBool::new(false))
                    .unwrap()
                });
                let received = $inlet::preflight_bounded(
                    double64_activation(),
                    address,
                    &identity(),
                    handshake_limits(),
                    FixedWidthNumericSampleLimits::new(
                        Duration::from_millis(5),
                        Duration::from_secs(1),
                    )
                    .unwrap(),
                    $limits::new(2, 3).unwrap(),
                    2,
                    3,
                )
                .unwrap()
                .finish(&AtomicBool::new(false))
                .unwrap();
                assert_eq!(received.record_count(), 3);
                assert_eq!(received.channel_count(), 2);
                for (index, (record, expected)) in
                    received.into_records().into_iter().zip(values).enumerate()
                {
                    assert_eq!(
                        record.raw_source_timestamp().value().to_bits(),
                        (1000.25 + index as f64).to_bits()
                    );
                    assert_eq!(record.sample().values(), &expected);
                }
                let sent = worker.join().unwrap();
                assert_eq!(sent.local_address(), address);
                assert_eq!(sent.record_count(), 3);
                TcpListener::bind(address).unwrap();

                let listener = TcpListener::bind("127.0.0.1:0").unwrap();
                let address = listener.local_addr().unwrap();
                let timestamp = 2000.125f64;
                let record = TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![values[0][0]]).unwrap(),
                    RawSourceTimestamp::new(timestamp).unwrap(),
                    None,
                );
                let worker = thread::spawn(move || {
                    $outlet::preflight_bounded(
                        double64_activation(),
                        listener,
                        &identity(),
                        handshake_limits(),
                        FixedWidthNumericSampleLimits::new(
                            Duration::from_millis(5),
                            Duration::from_secs(1),
                        )
                        .unwrap(),
                        $limits::new(1, 1).unwrap(),
                        &[record],
                    )
                    .unwrap()
                    .finish(&AtomicBool::new(false))
                    .unwrap()
                });
                let received = $inlet::preflight_bounded(
                    double64_activation(),
                    address,
                    &identity(),
                    handshake_limits(),
                    FixedWidthNumericSampleLimits::new(
                        Duration::from_millis(5),
                        Duration::from_secs(1),
                    )
                    .unwrap(),
                    $limits::new(1, 1).unwrap(),
                    1,
                    1,
                )
                .unwrap()
                .finish(&AtomicBool::new(false))
                .unwrap();
                assert_eq!(
                    received.records()[0]
                        .raw_source_timestamp()
                        .value()
                        .to_bits(),
                    timestamp.to_bits()
                );
                assert_eq!(received.records()[0].sample().values(), &[values[0][0]]);
                worker.join().unwrap();
                TcpListener::bind(address).unwrap();
            }
        };
    }

    integer_facade_host_test!(
        p10_integer_session_int32_preserves_typed_records_reports_bits_and_cleanup,
        i32,
        TimestampedInt32OutletSession,
        TimestampedInt32InletSession,
        TimestampedInt32SessionLimits,
        [[i32::MIN + 1, i32::MAX], [3, -4], [5, -6]]
    );
    integer_facade_host_test!(
        p10_integer_session_int16_preserves_typed_records_reports_bits_and_cleanup,
        i16,
        TimestampedInt16OutletSession,
        TimestampedInt16InletSession,
        TimestampedInt16SessionLimits,
        [[i16::MIN + 1, i16::MAX], [3, -4], [5, -6]]
    );
    integer_facade_host_test!(
        p10_integer_session_int8_preserves_typed_records_reports_bits_and_cleanup,
        i8,
        TimestampedInt8OutletSession,
        TimestampedInt8InletSession,
        TimestampedInt8SessionLimits,
        [[i8::MIN + 1, i8::MAX], [3, -4], [5, -6]]
    );

    #[test]
    fn p12_all_concrete_sessions_share_bounded_shape_preflight_without_widening() {
        let peer: SocketAddr = "127.0.0.1:9".parse().unwrap();
        let identity = identity();
        let handshake = handshake_limits();
        let numeric_io =
            FixedWidthNumericSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap();

        macro_rules! assert_integer_shape {
            ($inlet:ident, $limits:ident) => {{
                assert!($inlet::preflight_bounded(
                    double64_activation(),
                    peer,
                    &identity,
                    handshake,
                    numeric_io,
                    $limits::new(2, 3).unwrap(),
                    2,
                    3,
                )
                .is_ok());
                assert!(matches!(
                    $inlet::preflight_bounded(
                        double64_activation(),
                        peer,
                        &identity,
                        handshake,
                        numeric_io,
                        $limits::new(2, 3).unwrap(),
                        1,
                        2,
                    ),
                    Err(TimestampedFloat32SessionPreflightError::RecordCount { actual: 2 })
                ));
            }};
        }

        assert_integer_shape!(TimestampedInt32InletSession, TimestampedInt32SessionLimits);
        assert_integer_shape!(TimestampedInt16InletSession, TimestampedInt16SessionLimits);
        assert_integer_shape!(TimestampedInt8InletSession, TimestampedInt8SessionLimits);
        assert!(TimestampedDouble64InletSession::preflight_bounded(
            double64_activation(),
            peer,
            &identity,
            handshake,
            TimestampedDouble64SessionIoLimits::new(
                Duration::from_millis(5),
                Duration::from_secs(1),
            )
            .unwrap(),
            TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
            1,
            1,
        )
        .is_ok());
        assert!(TimestampedFloat32InletSession::preflight_bounded(
            activation(),
            peer,
            &identity,
            handshake,
            sample_limits(),
            TimestampedFloat32SessionLimits::new(2, 3).unwrap(),
            2,
            3,
        )
        .is_ok());
        assert!(TimestampedStringInletSession::preflight_bounded(
            string_activation(),
            peer,
            &identity,
            handshake,
            StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
            TimestampedStringSessionLimits::new(1, 1).unwrap(),
            1,
            1,
        )
        .is_ok());
        assert!(matches!(
            TimestampedStringInletSession::preflight_bounded(
                string_activation(),
                peer,
                &identity,
                handshake,
                StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap(),
                TimestampedStringSessionLimits::new(1, 1).unwrap(),
                1,
                2,
            ),
            Err(TimestampedStringSessionPreflightError::RecordCount { actual: 2 })
        ));
    }

    #[test]
    fn p2_double64_uses_shared_bounded_session_lifecycle_and_preserves_bits() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let records = [
            double64_sample(
                0x4092_5220_0000_0001,
                &[0x3ff0_0000_0000_0001, 0xbff0_0000_0000_0002],
            ),
            double64_sample(
                0x4092_5b80_0000_0002,
                &[0x7ff8_0000_0000_0042, 0x4000_0000_0000_0003],
            ),
            double64_sample(
                0x4092_64e0_0000_0003,
                &[0x8000_0000_0000_0000, 0x4010_0000_0000_0004],
            ),
        ];
        let worker = thread::spawn(move || {
            TimestampedDouble64OutletSession::preflight_bounded(
                double64_activation(),
                listener,
                &identity(),
                handshake_limits(),
                TimestampedDouble64SessionIoLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(1),
                )
                .unwrap(),
                TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let received = TimestampedDouble64InletSession::preflight_bounded(
            double64_activation(),
            address,
            &identity(),
            handshake_limits(),
            TimestampedDouble64SessionIoLimits::new(
                Duration::from_millis(5),
                Duration::from_secs(1),
            )
            .unwrap(),
            TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
            2,
            3,
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        assert_eq!(received.record_count(), 3);
        assert_eq!(received.channel_count(), 2);
        assert_eq!(
            received.records()[0].sample().values()[1].to_bits(),
            0xbff0_0000_0000_0002
        );
        assert_eq!(
            received.records()[1].sample().values()[0].to_bits(),
            0x7ff8_0000_0000_0042
        );
        assert_eq!(
            received.records()[2].sample().values()[0].to_bits(),
            0x8000_0000_0000_0000
        );
        let sent = worker.join().unwrap();
        assert_eq!(sent.record_count(), 3);
        assert_eq!(sent.local_address(), address);
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn p2_double64_rejects_unevidenced_shapes_before_socket_io() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let records = [double64_sample(
            0x4092_5220_0000_0001,
            &[1.0f64.to_bits(); 2],
        )];
        let identity = identity();
        let result = TimestampedDouble64OutletSession::preflight_bounded(
            double64_activation(),
            listener,
            &identity,
            handshake_limits(),
            TimestampedDouble64SessionIoLimits::new(
                Duration::from_millis(5),
                Duration::from_secs(1),
            )
            .unwrap(),
            TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
            &records,
        );
        assert!(matches!(
            result,
            Err(TimestampedDouble64SessionPreflightError::ChannelCount {
                index: 0,
                actual: 2
            })
        ));
        TcpListener::bind(address).unwrap();
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

    #[test]
    fn p2_zero_remaining_terminal_deadline_is_typed_before_io() {
        let limits = sample_limits();
        assert_eq!(
            terminal_read_timeout(limits.total_deadline(), limits),
            Err(TimestampedFloat32SessionError::Record {
                index: None,
                error: TimestampedFloat32SampleError::Deadline,
            })
        );
        assert_eq!(
            terminal_read_timeout(limits.total_deadline() + Duration::from_nanos(1), limits),
            Err(TimestampedFloat32SessionError::Record {
                index: None,
                error: TimestampedFloat32SampleError::Deadline,
            })
        );
    }
}
