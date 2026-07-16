// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
//! One-channel, one-record String runtime bound to LSLC-003Q.

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
pub const STRING_SAMPLE_FEATURE_ID: &str = "string-sample";
/// Explicit runtime marker.
pub const STRING_SAMPLE_EFFECTIVE_MARKER: &str = "rusty.lsl.string_sample.effective";
const INITIALIZATION_TIMESTAMP: f64 = 123_456.789;
const MAX_STRING_BYTES: usize = 127;

/// One finite raw timestamp beside one nonempty bounded UTF-8 String.
#[derive(Clone, Debug, PartialEq)]
pub struct StringSampleRecord {
    timestamp: f64,
    value: String,
}
impl StringSampleRecord {
    /// Validates the closed LSLC-003Q value envelope.
    pub fn new(timestamp: f64, value: String) -> Result<Self, StringSampleError> {
        if !timestamp.is_finite() {
            return Err(StringSampleError::InvalidTimestamp);
        }
        if value.is_empty() {
            return Err(StringSampleError::EmptyValue);
        }
        if value.len() > MAX_STRING_BYTES {
            return Err(StringSampleError::ValueTooLong {
                actual: value.len(),
            });
        }
        Ok(Self { timestamp, value })
    }
    /// Raw caller timestamp.
    pub const fn timestamp(&self) -> f64 {
        self.timestamp
    }
    /// Exact caller String.
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Recovers the caller allocation.
    pub fn into_value(self) -> String {
        self.value
    }
}

/// Closed activation composed from distinct StringSample and handshake capabilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StringSampleActivation {
    handshake: StreamHandshakeActivation,
}
impl StringSampleActivation {
    /// Requires the module-nominal StringSample capability.
    pub fn new(
        capability: RuntimeModuleCapability,
        handshake: StreamHandshakeActivation,
    ) -> Result<Self, StringSampleActivationError> {
        if !capability.matches(RuntimeModule::StringSample) {
            return Err(StringSampleActivationError::WrongModule);
        }
        Ok(Self { handshake })
    }
}
/// Rejected activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringSampleActivationError {
    /// Capability named another module.
    WrongModule,
}

/// Finite I/O bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StringSampleLimits {
    io_slice: Duration,
    total_deadline: Duration,
}
impl StringSampleLimits {
    /// Creates nonzero finite bounds.
    pub fn new(
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, StringSampleLimitError> {
        if io_slice.is_zero() {
            return Err(StringSampleLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(StringSampleLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            io_slice,
            total_deadline,
        })
    }
}
/// Invalid limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StringSampleLimitError {
    /// I/O slice was zero.
    ZeroIoSlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
}

/// Stable bounded runtime failure.
#[derive(Debug, Eq, PartialEq)]
pub enum StringSampleError {
    /// Handshake failed.
    Handshake(StreamHandshakeError),
    /// Timestamp was non-finite.
    InvalidTimestamp,
    /// String was empty.
    EmptyValue,
    /// String exceeded 127 UTF-8 bytes.
    ValueTooLong {
        /// Observed byte count.
        actual: usize,
    },
    /// Cancellation was observed.
    Cancelled,
    /// Deadline elapsed.
    Deadline,
    /// Socket I/O failed.
    Io(ErrorKind),
    /// Peer closed before the expected record completed.
    Truncated {
        /// Bytes received in the failing fixed segment.
        actual: usize,
    },
    /// Record marker was not two.
    InvalidMarker {
        /// Received marker.
        actual: u8,
    },
    /// Initialization record differed.
    InvalidInitialization {
        /// Zero-based record index.
        index: usize,
    },
    /// The observed one-byte length form was not used.
    InvalidLengthForm {
        /// Received form byte.
        actual: u8,
    },
    /// String payload was not UTF-8.
    InvalidUtf8,
}

fn map_transport(error: BoundedFixedRecordError) -> StringSampleError {
    match error {
        BoundedFixedRecordError::Cancelled => StringSampleError::Cancelled,
        BoundedFixedRecordError::Deadline => StringSampleError::Deadline,
        BoundedFixedRecordError::Truncated { actual } => StringSampleError::Truncated { actual },
        BoundedFixedRecordError::Io(kind) => StringSampleError::Io(kind),
    }
}
fn write(
    stream: &mut TcpStream,
    bytes: &[u8],
    limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), StringSampleError> {
    write_exact_bounded(
        stream,
        bytes,
        limits.io_slice,
        limits.total_deadline,
        cancelled,
    )
    .map_err(map_transport)
}
fn read(
    stream: &mut TcpStream,
    bytes: &mut [u8],
    limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), StringSampleError> {
    read_exact_bounded(
        stream,
        bytes,
        limits.io_slice,
        limits.total_deadline,
        cancelled,
    )
    .map_err(map_transport)
}
fn encode(timestamp: f64, value: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(11 + value.len());
    bytes.push(2);
    bytes.extend_from_slice(&timestamp.to_le_bytes());
    bytes.push(1);
    bytes.push(value.len() as u8);
    bytes.extend_from_slice(value);
    bytes
}
fn write_initialization(
    stream: &mut TcpStream,
    limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), StringSampleError> {
    let record = encode(INITIALIZATION_TIMESTAMP, b"10");
    write(stream, &record, limits, cancelled)?;
    write(stream, &record, limits, cancelled)
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
    if length == 0 {
        return Err(StringSampleError::EmptyValue);
    }
    if length > MAX_STRING_BYTES {
        return Err(StringSampleError::ValueTooLong { actual: length });
    }
    Ok((f64::from_le_bytes(header[1..9].try_into().unwrap()), length))
}
fn read_value(
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
fn read_initialization(
    stream: &mut TcpStream,
    limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<(), StringSampleError> {
    for index in 0..2 {
        let record = read_value(stream, limits, cancelled)?;
        if record.timestamp.to_bits() != INITIALIZATION_TIMESTAMP.to_bits() || record.value != "10"
        {
            return Err(StringSampleError::InvalidInitialization { index });
        }
    }
    Ok(())
}

/// Sends two observed initialization records and exactly one caller record.
pub fn run_string_sample_outlet(
    activation: StringSampleActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    limits: StringSampleLimits,
    record: StringSampleRecord,
    cancelled: &AtomicBool,
) -> Result<SocketAddr, StringSampleError> {
    let (mut stream, local, _) = accept_handshake_stream_with_format(
        listener,
        identity,
        handshake_limits,
        cancelled,
        0,
        false,
    )
    .map_err(StringSampleError::Handshake)?;
    let _ = activation.handshake;
    write_initialization(&mut stream, limits, cancelled)?;
    write(
        &mut stream,
        &encode(record.timestamp, record.value.as_bytes()),
        limits,
        cancelled,
    )?;
    Ok(local)
}
/// Receives two observed initialization records and exactly one caller record.
pub fn run_string_sample_inlet(
    activation: StringSampleActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<StringSampleRecord, StringSampleError> {
    let mut stream =
        connect_handshake_stream_with_format(peer, identity, handshake_limits, cancelled, 0, false)
            .map_err(StringSampleError::Handshake)?;
    let _ = activation.handshake;
    read_initialization(&mut stream, limits, cancelled)?;
    read_value(&mut stream, limits, cancelled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use std::io::Write;
    use std::sync::atomic::AtomicBool;
    use std::thread;
    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(4096, 4096, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }
    fn limits() -> StringSampleLimits {
        StringSampleLimits::new(Duration::from_millis(5), Duration::from_secs(1)).unwrap()
    }
    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            handshake_limits(),
        )
        .unwrap()
    }
    fn activation() -> StringSampleActivation {
        StringSampleActivation::new(
            test_capability(RuntimeModule::StringSample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn lslc_003t_one_string_preserves_timestamp_utf8_and_cleanup() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            run_string_sample_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                limits(),
                StringSampleRecord::new(1234.5, "Rusty-mu-snow".into()).unwrap(),
                &AtomicBool::new(false),
            )
        });
        let received = run_string_sample_inlet(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(received.timestamp().to_bits(), 1234.5f64.to_bits());
        assert_eq!(received.value(), "Rusty-mu-snow");
        assert_eq!(worker.join().unwrap().unwrap(), address);
        assert!(TcpListener::bind(address).is_ok());
    }

    #[test]
    fn lslc_003t_capability_bounds_and_limits_fail_closed() {
        let handshake =
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap();
        assert_eq!(
            StringSampleActivation::new(
                test_capability(RuntimeModule::FixedWidthNumericSample),
                handshake
            ),
            Err(StringSampleActivationError::WrongModule)
        );
        assert_eq!(
            StringSampleRecord::new(1.0, String::new()),
            Err(StringSampleError::EmptyValue)
        );
        assert_eq!(
            StringSampleRecord::new(1.0, "x".repeat(128)),
            Err(StringSampleError::ValueTooLong { actual: 128 })
        );
        assert_eq!(
            StringSampleRecord::new(f64::NAN, "x".into()),
            Err(StringSampleError::InvalidTimestamp)
        );
        assert_eq!(
            StringSampleLimits::new(Duration::ZERO, Duration::from_secs(1)),
            Err(StringSampleLimitError::ZeroIoSlice)
        );
    }

    #[test]
    fn lslc_003t_damage_cancellation_and_deadline_are_typed() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = thread::spawn(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            let mut damaged = encode(1.0, b"x");
            damaged[9] = 2;
            stream.write_all(&damaged).unwrap();
        });
        let (mut stream, _) = listener.accept().unwrap();
        assert_eq!(
            read_value(&mut stream, limits(), &AtomicBool::new(false)),
            Err(StringSampleError::InvalidLengthForm { actual: 2 })
        );
        peer.join().unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = thread::spawn(move || TcpStream::connect(address).unwrap());
        let (mut stream, _) = listener.accept().unwrap();
        assert_eq!(
            write(&mut stream, b"x", limits(), &AtomicBool::new(true)),
            Err(StringSampleError::Cancelled)
        );
        let mut byte = [0];
        assert_eq!(
            read(
                &mut stream,
                &mut byte,
                StringSampleLimits::new(Duration::from_millis(1), Duration::from_millis(2))
                    .unwrap(),
                &AtomicBool::new(false)
            ),
            Err(StringSampleError::Deadline)
        );
        drop(peer.join().unwrap());
    }
}
