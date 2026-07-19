// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounded one-record runtime for four observed fixed-width numeric formats.

use crate::{
    bounded_fixed_record_transport::{
        read_exact_bounded, write_exact_bounded, BoundedFixedRecordError,
    },
    stream_handshake::{accept_handshake_stream_with_format, connect_handshake_stream_with_format},
    RawSourceTimestamp, RuntimeModule, RuntimeModuleCapability, Sample, SampleLimits,
    StreamHandshakeActivation, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedDouble64InletSession, TimestampedDouble64OutletSession,
    TimestampedDouble64SessionError, TimestampedDouble64SessionIoLimits,
    TimestampedDouble64SessionLimits, TimestampedSample,
};
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// Selected feature identity.
pub const FIXED_WIDTH_NUMERIC_SAMPLE_FEATURE_ID: &str = "fixed-width-numeric-sample";
/// Explicit runtime marker.
pub const FIXED_WIDTH_NUMERIC_SAMPLE_EFFECTIVE_MARKER: &str =
    "rusty.lsl.fixed_width_numeric_sample.effective";
const INIT_TIMESTAMP: f64 = 123_456.789;

/// Closed observed numeric value family.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FixedWidthNumericValue {
    /// IEEE-754 double.
    Double64(f64),
    /// Signed 32-bit integer.
    Int32(i32),
    /// Signed 16-bit integer.
    Int16(i16),
    /// Signed 8-bit integer.
    Int8(i8),
}
impl FixedWidthNumericValue {
    /// Canonical public format spelling.
    pub const fn format(self) -> &'static str {
        match self {
            Self::Double64(_) => "double64",
            Self::Int32(_) => "int32",
            Self::Int16(_) => "int16",
            Self::Int8(_) => "int8",
        }
    }
    /// Exact observed byte width.
    pub const fn width(self) -> usize {
        match self {
            Self::Double64(_) => 8,
            Self::Int32(_) => 4,
            Self::Int16(_) => 2,
            Self::Int8(_) => 1,
        }
    }
    const fn supports_subnormals(self) -> bool {
        matches!(self, Self::Double64(_))
    }
    fn bytes(self) -> Vec<u8> {
        match self {
            Self::Double64(v) => v.to_le_bytes().to_vec(),
            Self::Int32(v) => v.to_le_bytes().to_vec(),
            Self::Int16(v) => v.to_le_bytes().to_vec(),
            Self::Int8(v) => v.to_le_bytes().to_vec(),
        }
    }
    fn initialization(self) -> [Vec<u8>; 2] {
        match self {
            Self::Double64(_) => [
                hex(&[0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x70, 0x41]),
                hex(&[0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x70, 0x41]),
            ],
            Self::Int32(_) => [hex(&[5, 0, 1, 0]), hex(&[3, 0, 1, 0])],
            Self::Int16(_) => [hex(&[5, 1]), hex(&[3, 1])],
            Self::Int8(_) => [hex(&[5]), hex(&[3])],
        }
    }
    fn sequence_initialization(self) -> [[Self; 2]; 2] {
        match self {
            Self::Double64(_) => [
                [Self::Double64(16_777_221.0), Self::Double64(-16_777_222.0)],
                [Self::Double64(16_777_219.0), Self::Double64(-16_777_220.0)],
            ],
            Self::Int32(_) => [
                [Self::Int32(65_541), Self::Int32(-65_542)],
                [Self::Int32(65_539), Self::Int32(-65_540)],
            ],
            Self::Int16(_) => [
                [Self::Int16(261), Self::Int16(-262)],
                [Self::Int16(259), Self::Int16(-260)],
            ],
            Self::Int8(_) => [
                [Self::Int8(5), Self::Int8(-6)],
                [Self::Int8(3), Self::Int8(-4)],
            ],
        }
    }
    fn from_bytes(template: Self, b: &[u8]) -> Self {
        match template {
            Self::Double64(_) => Self::Double64(f64::from_le_bytes(b.try_into().unwrap())),
            Self::Int32(_) => Self::Int32(i32::from_le_bytes(b.try_into().unwrap())),
            Self::Int16(_) => Self::Int16(i16::from_le_bytes(b.try_into().unwrap())),
            Self::Int8(_) => Self::Int8(i8::from_le_bytes(b.try_into().unwrap())),
        }
    }
}
fn hex(v: &[u8]) -> Vec<u8> {
    v.to_vec()
}

/// One finite raw timestamp beside one value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedWidthNumericRecord {
    timestamp: f64,
    value: FixedWidthNumericValue,
}

/// Exactly two homogeneous channel values beside one finite raw timestamp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedWidthNumericPairRecord {
    timestamp: f64,
    values: [FixedWidthNumericValue; 2],
}
impl FixedWidthNumericPairRecord {
    /// Admits only a finite timestamp and one homogeneous format.
    pub fn new(
        timestamp: f64,
        values: [FixedWidthNumericValue; 2],
    ) -> Result<Self, FixedWidthNumericSampleError> {
        if !timestamp.is_finite() {
            return Err(FixedWidthNumericSampleError::InvalidTimestamp);
        }
        if values[0].format() != values[1].format() {
            return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: 1,
            });
        }
        Ok(Self { timestamp, values })
    }
    /// Timestamp.
    pub const fn timestamp(self) -> f64 {
        self.timestamp
    }
    /// Two channel values in caller order.
    pub const fn values(self) -> [FixedWidthNumericValue; 2] {
        self.values
    }
}

/// Exactly three ordered, homogeneous two-channel records.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedWidthNumericRecordSequence {
    records: [FixedWidthNumericPairRecord; 3],
}
impl FixedWidthNumericRecordSequence {
    /// Validates one closed three-record format family atomically.
    pub fn new(
        records: [FixedWidthNumericPairRecord; 3],
    ) -> Result<Self, FixedWidthNumericSampleError> {
        let format = records[0].values[0].format();
        for (record, candidate) in records.iter().enumerate() {
            for (channel, value) in candidate.values.iter().enumerate() {
                if value.format() != format {
                    return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                        record,
                        channel,
                    });
                }
            }
        }
        Ok(Self { records })
    }
    /// Three records in caller order.
    pub const fn records(self) -> [FixedWidthNumericPairRecord; 3] {
        self.records
    }
}
impl FixedWidthNumericRecord {
    /// Admits only a finite timestamp.
    pub fn new(
        timestamp: f64,
        value: FixedWidthNumericValue,
    ) -> Result<Self, FixedWidthNumericSampleError> {
        if !timestamp.is_finite() {
            return Err(FixedWidthNumericSampleError::InvalidTimestamp);
        }
        Ok(Self { timestamp, value })
    }
    /// Timestamp.
    pub const fn timestamp(self) -> f64 {
        self.timestamp
    }
    /// Value.
    pub const fn value(self) -> FixedWidthNumericValue {
        self.value
    }
}

/// Closed activation composed with handshake activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedWidthNumericSampleActivation {
    handshake: StreamHandshakeActivation,
}
impl FixedWidthNumericSampleActivation {
    /// Validates feature and marker.
    pub fn new(
        capability: RuntimeModuleCapability,
        handshake: StreamHandshakeActivation,
    ) -> Result<Self, FixedWidthNumericSampleActivationError> {
        if !capability.matches(RuntimeModule::FixedWidthNumericSample) {
            return Err(FixedWidthNumericSampleActivationError::WrongModule);
        }
        Ok(Self { handshake })
    }
}
/// Rejected activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixedWidthNumericSampleActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}
/// Finite I/O limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedWidthNumericSampleLimits {
    io_slice: Duration,
    total_deadline: Duration,
}
impl FixedWidthNumericSampleLimits {
    /// Creates nonzero limits.
    pub fn new(
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, FixedWidthNumericSampleLimitError> {
        if io_slice.is_zero() {
            return Err(FixedWidthNumericSampleLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(FixedWidthNumericSampleLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            io_slice,
            total_deadline,
        })
    }
}
/// Invalid limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixedWidthNumericSampleLimitError {
    /// Slice was zero.
    ZeroIoSlice,
    /// Deadline was zero.
    ZeroTotalDeadline,
}
/// Stable runtime failure.
#[derive(Debug, Eq, PartialEq)]
pub enum FixedWidthNumericSampleError {
    /// Handshake failed.
    Handshake(StreamHandshakeError),
    /// Timestamp was non-finite.
    InvalidTimestamp,
    /// Cancellation observed.
    Cancelled,
    /// Deadline elapsed.
    Deadline,
    /// Socket I/O failed.
    Io(ErrorKind),
    /// Peer closed early.
    Truncated {
        /// Bytes read.
        actual: usize,
    },
    /// Marker differed.
    InvalidMarker {
        /// Marker byte.
        actual: u8,
    },
    /// Initialization record differed.
    InvalidInitialization {
        /// Record index.
        index: usize,
    },
    /// A sequence channel did not use the first channel's format.
    SequenceFormatMismatch {
        /// Record index.
        record: usize,
        /// Channel index.
        channel: usize,
    },
}

fn transfer(
    stream: &mut TcpStream,
    mut bytes: Option<&mut [u8]>,
    write: &[u8],
    limits: FixedWidthNumericSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), FixedWidthNumericSampleError> {
    if let Some(bytes) = bytes.as_deref_mut() {
        read_exact_bounded(
            stream,
            bytes,
            limits.io_slice,
            limits.total_deadline,
            cancelled,
        )
        .map_err(map_transport_error)
    } else {
        write_exact_bounded(
            stream,
            write,
            limits.io_slice,
            limits.total_deadline,
            cancelled,
        )
        .map_err(map_transport_error)
    }
}

fn map_transport_error(error: BoundedFixedRecordError) -> FixedWidthNumericSampleError {
    match error {
        BoundedFixedRecordError::Cancelled => FixedWidthNumericSampleError::Cancelled,
        BoundedFixedRecordError::Deadline => FixedWidthNumericSampleError::Deadline,
        BoundedFixedRecordError::Truncated { actual } => {
            FixedWidthNumericSampleError::Truncated { actual }
        }
        BoundedFixedRecordError::Io(kind) => FixedWidthNumericSampleError::Io(kind),
    }
}
fn encode(timestamp: f64, value: &[u8]) -> Vec<u8> {
    let mut b = Vec::with_capacity(9 + value.len());
    b.push(2);
    b.extend_from_slice(&timestamp.to_le_bytes());
    b.extend_from_slice(value);
    b
}
fn write_record(
    stream: &mut TcpStream,
    timestamp: f64,
    value: &[u8],
    limits: FixedWidthNumericSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), FixedWidthNumericSampleError> {
    let b = encode(timestamp, value);
    transfer(stream, None, &b, limits, cancelled)
}
fn read_record(
    stream: &mut TcpStream,
    template: FixedWidthNumericValue,
    limits: FixedWidthNumericSampleLimits,
    cancelled: &AtomicBool,
) -> Result<FixedWidthNumericRecord, FixedWidthNumericSampleError> {
    let mut b = vec![0; 9 + template.width()];
    transfer(stream, Some(&mut b), &[], limits, cancelled)?;
    if b[0] != 2 {
        return Err(FixedWidthNumericSampleError::InvalidMarker { actual: b[0] });
    }
    FixedWidthNumericRecord::new(
        f64::from_le_bytes(b[1..9].try_into().unwrap()),
        FixedWidthNumericValue::from_bytes(template, &b[9..]),
    )
}
fn write_initialization(
    s: &mut TcpStream,
    v: FixedWidthNumericValue,
    l: FixedWidthNumericSampleLimits,
    c: &AtomicBool,
) -> Result<(), FixedWidthNumericSampleError> {
    for b in v.initialization() {
        write_record(s, INIT_TIMESTAMP, &b, l, c)?
    }
    Ok(())
}
fn read_initialization(
    s: &mut TcpStream,
    v: FixedWidthNumericValue,
    l: FixedWidthNumericSampleLimits,
    c: &AtomicBool,
) -> Result<(), FixedWidthNumericSampleError> {
    for (index, expected) in v.initialization().into_iter().enumerate() {
        let mut b = vec![0; 9 + v.width()];
        transfer(s, Some(&mut b), &[], l, c)?;
        if b[0] != 2 || b[1..9] != INIT_TIMESTAMP.to_le_bytes() || b[9..] != expected {
            return Err(FixedWidthNumericSampleError::InvalidInitialization { index });
        }
    }
    Ok(())
}

fn pair_bytes(values: [FixedWidthNumericValue; 2]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(values[0].width() * 2);
    bytes.extend_from_slice(&values[0].bytes());
    bytes.extend_from_slice(&values[1].bytes());
    bytes
}

fn write_sequence_initialization(
    stream: &mut TcpStream,
    template: FixedWidthNumericValue,
    limits: FixedWidthNumericSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), FixedWidthNumericSampleError> {
    for values in template.sequence_initialization() {
        write_record(
            stream,
            INIT_TIMESTAMP,
            &pair_bytes(values),
            limits,
            cancelled,
        )?;
    }
    Ok(())
}

fn read_sequence_initialization(
    stream: &mut TcpStream,
    template: FixedWidthNumericValue,
    limits: FixedWidthNumericSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), FixedWidthNumericSampleError> {
    for (index, values) in template.sequence_initialization().into_iter().enumerate() {
        let expected = encode(INIT_TIMESTAMP, &pair_bytes(values));
        let mut actual = vec![0; expected.len()];
        transfer(stream, Some(&mut actual), &[], limits, cancelled)?;
        if actual != expected {
            return Err(FixedWidthNumericSampleError::InvalidInitialization { index });
        }
    }
    Ok(())
}

fn read_pair_record(
    stream: &mut TcpStream,
    template: FixedWidthNumericValue,
    limits: FixedWidthNumericSampleLimits,
    cancelled: &AtomicBool,
) -> Result<FixedWidthNumericPairRecord, FixedWidthNumericSampleError> {
    let width = template.width();
    let mut bytes = vec![0; 9 + width * 2];
    transfer(stream, Some(&mut bytes), &[], limits, cancelled)?;
    if bytes[0] != 2 {
        return Err(FixedWidthNumericSampleError::InvalidMarker { actual: bytes[0] });
    }
    FixedWidthNumericPairRecord::new(
        f64::from_le_bytes(bytes[1..9].try_into().unwrap()),
        [
            FixedWidthNumericValue::from_bytes(template, &bytes[9..9 + width]),
            FixedWidthNumericValue::from_bytes(template, &bytes[9 + width..]),
        ],
    )
}

/// Sends observed two-channel initialization and exactly three caller records.
pub fn run_fixed_width_numeric_sequence_outlet(
    activation: FixedWidthNumericSampleActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    limits: FixedWidthNumericSampleLimits,
    sequence: FixedWidthNumericRecordSequence,
    cancelled: &AtomicBool,
) -> Result<SocketAddr, FixedWidthNumericSampleError> {
    let template = sequence.records[0].values[0];
    if matches!(template, FixedWidthNumericValue::Double64(_)) {
        let records = sequence_double64_records(sequence)?;
        let report = TimestampedDouble64OutletSession::preflight_bounded(
            activation,
            listener,
            identity,
            handshake_limits,
            double64_io_limits(limits),
            double64_session_limits(2, 3),
            &records,
        )
        .map_err(map_double64_preflight_error)?
        .finish(cancelled)
        .map_err(map_double64_session_error)?;
        return Ok(report.local_address());
    }
    let (mut stream, local, _) = accept_handshake_stream_with_format(
        listener,
        identity,
        handshake_limits,
        cancelled,
        template.width(),
        template.supports_subnormals(),
    )
    .map_err(FixedWidthNumericSampleError::Handshake)?;
    let _ = activation.handshake;
    write_sequence_initialization(&mut stream, template, limits, cancelled)?;
    for record in sequence.records {
        write_record(
            &mut stream,
            record.timestamp,
            &pair_bytes(record.values),
            limits,
            cancelled,
        )?;
    }
    Ok(local)
}

/// Receives observed two-channel initialization and exactly three caller records.
pub fn run_fixed_width_numeric_sequence_inlet(
    activation: FixedWidthNumericSampleActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    limits: FixedWidthNumericSampleLimits,
    template: FixedWidthNumericValue,
    cancelled: &AtomicBool,
) -> Result<FixedWidthNumericRecordSequence, FixedWidthNumericSampleError> {
    if matches!(template, FixedWidthNumericValue::Double64(_)) {
        let report = TimestampedDouble64InletSession::preflight_bounded(
            activation,
            peer,
            identity,
            handshake_limits,
            double64_io_limits(limits),
            double64_session_limits(2, 3),
            2,
            3,
        )
        .map_err(map_double64_preflight_error)?
        .finish(cancelled)
        .map_err(map_double64_session_error)?;
        return double64_sequence_from_records(report.into_records());
    }
    let mut stream = connect_handshake_stream_with_format(
        peer,
        identity,
        handshake_limits,
        cancelled,
        template.width(),
        template.supports_subnormals(),
    )
    .map_err(FixedWidthNumericSampleError::Handshake)?;
    let _ = activation.handshake;
    read_sequence_initialization(&mut stream, template, limits, cancelled)?;
    FixedWidthNumericRecordSequence::new([
        read_pair_record(&mut stream, template, limits, cancelled)?,
        read_pair_record(&mut stream, template, limits, cancelled)?,
        read_pair_record(&mut stream, template, limits, cancelled)?,
    ])
}
/// Sends initialization and one caller record.
pub fn run_fixed_width_numeric_outlet(
    a: FixedWidthNumericSampleActivation,
    listener: TcpListener,
    id: &StreamHandshakeIdentity,
    hl: StreamHandshakeLimits,
    l: FixedWidthNumericSampleLimits,
    r: FixedWidthNumericRecord,
    c: &AtomicBool,
) -> Result<SocketAddr, FixedWidthNumericSampleError> {
    if let FixedWidthNumericValue::Double64(value) = r.value {
        let records = [double64_sample(r.timestamp, [value])?];
        let report = TimestampedDouble64OutletSession::preflight_bounded(
            a,
            listener,
            id,
            hl,
            double64_io_limits(l),
            double64_session_limits(1, 1),
            &records,
        )
        .map_err(map_double64_preflight_error)?
        .finish(c)
        .map_err(map_double64_session_error)?;
        return Ok(report.local_address());
    }
    let (mut s, local, _) = accept_handshake_stream_with_format(
        listener,
        id,
        hl,
        c,
        r.value.width(),
        r.value.supports_subnormals(),
    )
    .map_err(FixedWidthNumericSampleError::Handshake)?;
    let _ = a.handshake;
    write_initialization(&mut s, r.value, l, c)?;
    write_record(&mut s, r.timestamp, &r.value.bytes(), l, c)?;
    Ok(local)
}
/// Receives initialization and one caller record of the selected template format.
pub fn run_fixed_width_numeric_inlet(
    a: FixedWidthNumericSampleActivation,
    peer: SocketAddr,
    id: &StreamHandshakeIdentity,
    hl: StreamHandshakeLimits,
    l: FixedWidthNumericSampleLimits,
    template: FixedWidthNumericValue,
    c: &AtomicBool,
) -> Result<FixedWidthNumericRecord, FixedWidthNumericSampleError> {
    if matches!(template, FixedWidthNumericValue::Double64(_)) {
        let report = TimestampedDouble64InletSession::preflight_bounded(
            a,
            peer,
            id,
            hl,
            double64_io_limits(l),
            double64_session_limits(1, 1),
            1,
            1,
        )
        .map_err(map_double64_preflight_error)?
        .finish(c)
        .map_err(map_double64_session_error)?;
        let record = report.into_records().pop().expect("one admitted record");
        return FixedWidthNumericRecord::new(
            record.raw_source_timestamp().value(),
            FixedWidthNumericValue::Double64(record.sample().values()[0]),
        );
    }
    let mut s = connect_handshake_stream_with_format(
        peer,
        id,
        hl,
        c,
        template.width(),
        template.supports_subnormals(),
    )
    .map_err(FixedWidthNumericSampleError::Handshake)?;
    let _ = a.handshake;
    read_initialization(&mut s, template, l, c)?;
    read_record(&mut s, template, l, c)
}

fn double64_io_limits(limits: FixedWidthNumericSampleLimits) -> TimestampedDouble64SessionIoLimits {
    TimestampedDouble64SessionIoLimits::new(limits.io_slice, limits.total_deadline)
        .expect("fixed-width limits are nonzero")
}

fn double64_session_limits(channels: usize, records: usize) -> TimestampedDouble64SessionLimits {
    TimestampedDouble64SessionLimits::new(channels, records)
        .expect("accepted Double64 shapes are nonzero")
}

fn double64_sample<const N: usize>(
    timestamp: f64,
    values: [f64; N],
) -> Result<TimestampedSample<f64>, FixedWidthNumericSampleError> {
    let timestamp = RawSourceTimestamp::new(timestamp)
        .map_err(|_| FixedWidthNumericSampleError::InvalidTimestamp)?;
    let sample = Sample::new(
        SampleLimits::new(N).expect("accepted Double64 shapes are nonzero"),
        N,
        values.into_iter().collect(),
    )
    .expect("array length equals declared channel count");
    Ok(TimestampedSample::new(sample, timestamp, None))
}

fn sequence_double64_records(
    sequence: FixedWidthNumericRecordSequence,
) -> Result<[TimestampedSample<f64>; 3], FixedWidthNumericSampleError> {
    sequence
        .records
        .map(|record| {
            let [first, second] = record.values;
            match (first, second) {
                (
                    FixedWidthNumericValue::Double64(first),
                    FixedWidthNumericValue::Double64(second),
                ) => double64_sample(record.timestamp, [first, second]),
                _ => Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                    record: 0,
                    channel: 0,
                }),
            }
        })
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| FixedWidthNumericSampleError::SequenceFormatMismatch {
            record: 0,
            channel: 0,
        })
}

fn double64_sequence_from_records(
    records: Vec<TimestampedSample<f64>>,
) -> Result<FixedWidthNumericRecordSequence, FixedWidthNumericSampleError> {
    let records: [TimestampedSample<f64>; 3] =
        records
            .try_into()
            .map_err(|_| FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: 0,
            })?;
    let mut converted = Vec::with_capacity(3);
    for record in records {
        let values = record.sample().values();
        if values.len() != 2 {
            return Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: converted.len(),
                channel: values.len(),
            });
        }
        converted.push(FixedWidthNumericPairRecord::new(
            record.raw_source_timestamp().value(),
            [
                FixedWidthNumericValue::Double64(values[0]),
                FixedWidthNumericValue::Double64(values[1]),
            ],
        )?);
    }
    FixedWidthNumericRecordSequence::new(converted.try_into().expect("three records retained"))
}

fn map_double64_preflight_error(
    error: crate::TimestampedDouble64SessionPreflightError,
) -> FixedWidthNumericSampleError {
    match error {
        crate::TimestampedDouble64SessionPreflightError::RecordCount { actual } => {
            FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: actual,
                channel: 0,
            }
        }
        crate::TimestampedDouble64SessionPreflightError::ChannelCount { index, actual } => {
            FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: index,
                channel: actual,
            }
        }
        crate::TimestampedDouble64SessionPreflightError::InconsistentChannelCount {
            index, ..
        } => FixedWidthNumericSampleError::SequenceFormatMismatch {
            record: index,
            channel: 0,
        },
    }
}

fn map_double64_session_error(
    error: TimestampedDouble64SessionError,
) -> FixedWidthNumericSampleError {
    match error {
        TimestampedDouble64SessionError::Handshake(error) => {
            FixedWidthNumericSampleError::Handshake(error)
        }
        TimestampedDouble64SessionError::Record { error, .. } => error,
        TimestampedDouble64SessionError::TrailingByte { actual } => {
            FixedWidthNumericSampleError::InvalidMarker { actual }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::stream_handshake::{
        accept_handshake_stream_with_value_size, connect_handshake_stream_with_value_size,
    };
    use std::thread;
    fn hl() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(4096, 4096, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }
    fn sl() -> FixedWidthNumericSampleLimits {
        FixedWidthNumericSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }
    fn id() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            hl(),
        )
        .unwrap()
    }
    fn activation() -> FixedWidthNumericSampleActivation {
        FixedWidthNumericSampleActivation::new(
            test_capability(RuntimeModule::FixedWidthNumericSample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn lslc_003b_four_formats_preserve_timestamp_value_and_cleanup() {
        for value in [
            FixedWidthNumericValue::Double64(f64::from_bits(0x4009_21fb_5444_2d18)),
            FixedWidthNumericValue::Int32(i32::MIN + 7),
            FixedWidthNumericValue::Int16(i16::MIN + 7),
            FixedWidthNumericValue::Int8(i8::MIN + 7),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let sent = FixedWidthNumericRecord::new(-0.0, value).unwrap();
            let worker = thread::spawn(move || {
                run_fixed_width_numeric_outlet(
                    activation(),
                    listener,
                    &id(),
                    hl(),
                    sl(),
                    sent,
                    &AtomicBool::new(false),
                )
            });
            let received = run_fixed_width_numeric_inlet(
                activation(),
                address,
                &id(),
                hl(),
                sl(),
                value,
                &AtomicBool::new(false),
            )
            .unwrap();
            assert_eq!(received.timestamp().to_bits(), (-0.0f64).to_bits());
            assert_eq!(received.value(), value);
            assert_eq!(worker.join().unwrap().unwrap(), address);
            assert!(TcpListener::bind(address).is_ok());
        }
    }
    #[test]
    fn lslc_003b_activation_limits_and_nonfinite_timestamp_fail_closed() {
        let handshake =
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap();
        assert_eq!(
            FixedWidthNumericSampleActivation::new(
                test_capability(RuntimeModule::UdpDiscovery),
                handshake
            ),
            Err(FixedWidthNumericSampleActivationError::WrongModule)
        );
        assert_eq!(
            FixedWidthNumericSampleLimits::new(Duration::ZERO, Duration::ZERO),
            Err(FixedWidthNumericSampleLimitError::ZeroIoSlice)
        );
        assert_eq!(
            FixedWidthNumericRecord::new(f64::INFINITY, FixedWidthNumericValue::Int8(1)),
            Err(FixedWidthNumericSampleError::InvalidTimestamp)
        );
    }
    #[test]
    fn lslc_003b_cross_format_initialization_rejects_typed() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let (mut s, _, _) = accept_handshake_stream_with_value_size(
                listener,
                &id(),
                hl(),
                &AtomicBool::new(false),
                2,
            )
            .unwrap();
            write_initialization(
                &mut s,
                FixedWidthNumericValue::Int16(0),
                sl(),
                &AtomicBool::new(false),
            )
            .unwrap();
        });
        let mut s = connect_handshake_stream_with_value_size(
            address,
            &id(),
            hl(),
            &AtomicBool::new(false),
            2,
        )
        .unwrap();
        assert_eq!(
            read_initialization(
                &mut s,
                FixedWidthNumericValue::Int8(0),
                sl(),
                &AtomicBool::new(false)
            ),
            Err(FixedWidthNumericSampleError::InvalidInitialization { index: 1 })
        );
        worker.join().unwrap();
    }

    fn sequence(template: FixedWidthNumericValue) -> FixedWidthNumericRecordSequence {
        let values = match template {
            FixedWidthNumericValue::Double64(_) => [
                [
                    FixedWidthNumericValue::Double64(-0.0),
                    FixedWidthNumericValue::Double64(f64::from_bits(0x7ff8_0000_0000_0042)),
                ],
                [
                    FixedWidthNumericValue::Double64(3.5),
                    FixedWidthNumericValue::Double64(-4.5),
                ],
                [
                    FixedWidthNumericValue::Double64(5.5),
                    FixedWidthNumericValue::Double64(-6.5),
                ],
            ],
            FixedWidthNumericValue::Int32(_) => [
                [
                    FixedWidthNumericValue::Int32(i32::MIN + 1),
                    FixedWidthNumericValue::Int32(i32::MAX),
                ],
                [
                    FixedWidthNumericValue::Int32(3),
                    FixedWidthNumericValue::Int32(-4),
                ],
                [
                    FixedWidthNumericValue::Int32(5),
                    FixedWidthNumericValue::Int32(-6),
                ],
            ],
            FixedWidthNumericValue::Int16(_) => [
                [
                    FixedWidthNumericValue::Int16(i16::MIN + 1),
                    FixedWidthNumericValue::Int16(i16::MAX),
                ],
                [
                    FixedWidthNumericValue::Int16(3),
                    FixedWidthNumericValue::Int16(-4),
                ],
                [
                    FixedWidthNumericValue::Int16(5),
                    FixedWidthNumericValue::Int16(-6),
                ],
            ],
            FixedWidthNumericValue::Int8(_) => [
                [
                    FixedWidthNumericValue::Int8(i8::MIN + 1),
                    FixedWidthNumericValue::Int8(i8::MAX),
                ],
                [
                    FixedWidthNumericValue::Int8(3),
                    FixedWidthNumericValue::Int8(-4),
                ],
                [
                    FixedWidthNumericValue::Int8(5),
                    FixedWidthNumericValue::Int8(-6),
                ],
            ],
        };
        FixedWidthNumericRecordSequence::new([
            FixedWidthNumericPairRecord::new(1234.5, values[0]).unwrap(),
            FixedWidthNumericPairRecord::new(1235.5, values[1]).unwrap(),
            FixedWidthNumericPairRecord::new(1236.5, values[2]).unwrap(),
        ])
        .unwrap()
    }

    #[test]
    fn lslc_003p_four_formats_preserve_two_channels_three_records_and_cleanup() {
        for template in [
            FixedWidthNumericValue::Double64(0.0),
            FixedWidthNumericValue::Int32(0),
            FixedWidthNumericValue::Int16(0),
            FixedWidthNumericValue::Int8(0),
        ] {
            let expected = sequence(template);
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let worker = thread::spawn(move || {
                run_fixed_width_numeric_sequence_outlet(
                    activation(),
                    listener,
                    &id(),
                    hl(),
                    sl(),
                    expected,
                    &AtomicBool::new(false),
                )
            });
            let actual = run_fixed_width_numeric_sequence_inlet(
                activation(),
                address,
                &id(),
                hl(),
                sl(),
                template,
                &AtomicBool::new(false),
            )
            .unwrap();
            for (actual, expected) in actual.records().into_iter().zip(expected.records()) {
                assert_eq!(actual.timestamp().to_bits(), expected.timestamp().to_bits());
                for (actual, expected) in actual.values().into_iter().zip(expected.values()) {
                    match (actual, expected) {
                        (
                            FixedWidthNumericValue::Double64(a),
                            FixedWidthNumericValue::Double64(e),
                        ) => assert_eq!(a.to_bits(), e.to_bits()),
                        _ => assert_eq!(actual, expected),
                    }
                }
            }
            assert_eq!(worker.join().unwrap().unwrap(), address);
            assert!(TcpListener::bind(address).is_ok());
        }
    }

    #[test]
    fn lslc_003p_shape_format_timestamp_and_truncation_fail_closed() {
        assert_eq!(
            FixedWidthNumericPairRecord::new(
                1.0,
                [
                    FixedWidthNumericValue::Int16(1),
                    FixedWidthNumericValue::Int8(2)
                ],
            ),
            Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 0,
                channel: 1
            })
        );
        assert_eq!(
            FixedWidthNumericPairRecord::new(
                f64::NAN,
                [
                    FixedWidthNumericValue::Int8(1),
                    FixedWidthNumericValue::Int8(2)
                ],
            ),
            Err(FixedWidthNumericSampleError::InvalidTimestamp)
        );
        let mismatched = [
            FixedWidthNumericPairRecord::new(
                1.0,
                [
                    FixedWidthNumericValue::Int8(1),
                    FixedWidthNumericValue::Int8(2),
                ],
            )
            .unwrap(),
            FixedWidthNumericPairRecord::new(
                2.0,
                [
                    FixedWidthNumericValue::Int16(3),
                    FixedWidthNumericValue::Int16(4),
                ],
            )
            .unwrap(),
            FixedWidthNumericPairRecord::new(
                3.0,
                [
                    FixedWidthNumericValue::Int8(5),
                    FixedWidthNumericValue::Int8(6),
                ],
            )
            .unwrap(),
        ];
        assert_eq!(
            FixedWidthNumericRecordSequence::new(mismatched),
            Err(FixedWidthNumericSampleError::SequenceFormatMismatch {
                record: 1,
                channel: 0
            })
        );

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let (mut stream, _, _) = accept_handshake_stream_with_format(
                listener,
                &id(),
                hl(),
                &AtomicBool::new(false),
                1,
                false,
            )
            .unwrap();
            write_sequence_initialization(
                &mut stream,
                FixedWidthNumericValue::Int8(0),
                sl(),
                &AtomicBool::new(false),
            )
            .unwrap();
            write_record(&mut stream, 1.0, &[1, 2], sl(), &AtomicBool::new(false)).unwrap();
        });
        assert!(matches!(
            run_fixed_width_numeric_sequence_inlet(
                activation(),
                address,
                &id(),
                hl(),
                sl(),
                FixedWidthNumericValue::Int8(0),
                &AtomicBool::new(false),
            ),
            Err(FixedWidthNumericSampleError::Truncated { .. })
        ));
        worker.join().unwrap();
    }

    #[test]
    fn lslc_003p_truncation_is_addressable_for_every_accepted_width() {
        for template in [
            FixedWidthNumericValue::Double64(0.0),
            FixedWidthNumericValue::Int32(0),
            FixedWidthNumericValue::Int16(0),
            FixedWidthNumericValue::Int8(0),
        ] {
            let record = sequence(template).records()[0];
            let encoded = encode(record.timestamp(), &pair_bytes(record.values()));
            for retained in [0, 1, encoded.len() - 1] {
                let listener = TcpListener::bind("127.0.0.1:0").unwrap();
                let address = listener.local_addr().unwrap();
                let encoded = encoded.clone();
                let worker = thread::spawn(move || {
                    let (mut stream, _, _) = accept_handshake_stream_with_format(
                        listener,
                        &id(),
                        hl(),
                        &AtomicBool::new(false),
                        template.width(),
                        template.supports_subnormals(),
                    )
                    .unwrap();
                    write_sequence_initialization(
                        &mut stream,
                        template,
                        sl(),
                        &AtomicBool::new(false),
                    )
                    .unwrap();
                    transfer(
                        &mut stream,
                        None,
                        &encoded[..retained],
                        sl(),
                        &AtomicBool::new(false),
                    )
                    .unwrap();
                });
                assert_eq!(
                    run_fixed_width_numeric_sequence_inlet(
                        activation(),
                        address,
                        &id(),
                        hl(),
                        sl(),
                        template,
                        &AtomicBool::new(false),
                    ),
                    Err(FixedWidthNumericSampleError::Truncated { actual: retained })
                );
                worker.join().unwrap();
                assert!(TcpListener::bind(address).is_ok());
            }
        }
    }

    #[test]
    fn lslc_003p_width_shift_retains_marker_error_ownership_and_cleanup() {
        for template in [
            FixedWidthNumericValue::Double64(0.0),
            FixedWidthNumericValue::Int32(0),
            FixedWidthNumericValue::Int16(0),
            FixedWidthNumericValue::Int8(0),
        ] {
            let expected = sequence(template);
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let worker = thread::spawn(move || {
                let (mut stream, _, _) = accept_handshake_stream_with_format(
                    listener,
                    &id(),
                    hl(),
                    &AtomicBool::new(false),
                    template.width(),
                    template.supports_subnormals(),
                )
                .unwrap();
                write_sequence_initialization(&mut stream, template, sl(), &AtomicBool::new(false))
                    .unwrap();
                let records = expected.records();
                write_record(
                    &mut stream,
                    records[0].timestamp(),
                    &pair_bytes(records[0].values()),
                    sl(),
                    &AtomicBool::new(false),
                )
                .unwrap();
                transfer(&mut stream, None, &[0x7f], sl(), &AtomicBool::new(false)).unwrap();
                write_record(
                    &mut stream,
                    records[1].timestamp(),
                    &pair_bytes(records[1].values()),
                    sl(),
                    &AtomicBool::new(false),
                )
                .unwrap();
            });
            assert_eq!(
                run_fixed_width_numeric_sequence_inlet(
                    activation(),
                    address,
                    &id(),
                    hl(),
                    sl(),
                    template,
                    &AtomicBool::new(false),
                ),
                Err(FixedWidthNumericSampleError::InvalidMarker { actual: 0x7f })
            );
            worker.join().unwrap();
            assert!(TcpListener::bind(address).is_ok());
        }
    }

    #[test]
    fn lslc_003p_caller_cancellation_precedes_deadline_and_teardown_repeats() {
        let limits =
            FixedWidthNumericSampleLimits::new(Duration::from_millis(2), Duration::from_millis(20))
                .unwrap();
        for cancelled_before_read in [true, false, true, false] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let worker = thread::spawn(move || {
                let (_stream, _) = listener.accept().unwrap();
                thread::sleep(Duration::from_millis(40));
            });
            let mut stream = TcpStream::connect(address).unwrap();
            let cancelled = AtomicBool::new(cancelled_before_read);
            let actual = read_pair_record(
                &mut stream,
                FixedWidthNumericValue::Int8(0),
                limits,
                &cancelled,
            );
            let expected = if cancelled_before_read {
                FixedWidthNumericSampleError::Cancelled
            } else {
                FixedWidthNumericSampleError::Deadline
            };
            assert_eq!(actual, Err(expected));
            drop(stream);
            worker.join().unwrap();
            assert!(TcpListener::bind(address).is_ok());
        }
    }
}
