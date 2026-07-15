// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
//! Bounded one-record runtime for four observed fixed-width numeric formats.

use crate::{
    bounded_fixed_record_transport::{
        read_exact_bounded, write_exact_bounded, BoundedFixedRecordError,
    },
    stream_handshake::{accept_handshake_stream_with_format, connect_handshake_stream_with_format},
    RuntimeModule, RuntimeModuleCapability, StreamHandshakeActivation, StreamHandshakeError,
    StreamHandshakeIdentity, StreamHandshakeLimits,
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
}
