// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later
//! One-channel, one-record String runtime bound to LSLC-003Q.

use crate::{
    timestamped_float32_session_runtime::{
        finish_string_inlet_session, finish_string_outlet_session,
    },
    RuntimeModule, RuntimeModuleCapability, StreamHandshakeActivation, StreamHandshakeError,
    StreamHandshakeIdentity, StreamHandshakeLimits,
};
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// Selected feature identity.
pub const STRING_SAMPLE_FEATURE_ID: &str = "string-sample";
/// Explicit runtime marker.
pub const STRING_SAMPLE_EFFECTIVE_MARKER: &str = "rusty.lsl.string_sample.effective";
const MAX_STRING_BYTES: usize = 129;

/// One finite raw timestamp beside one bounded UTF-8 String.
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
    pub(crate) const fn io_slice(self) -> Duration {
        self.io_slice
    }
    pub(crate) const fn total_deadline(self) -> Duration {
        self.total_deadline
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
    /// String exceeded 129 UTF-8 bytes.
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
    let _ = activation.handshake;
    finish_string_outlet_session(
        listener,
        identity,
        handshake_limits,
        limits,
        &record,
        cancelled,
    )
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
    let _ = activation.handshake;
    finish_string_inlet_session(peer, identity, handshake_limits, limits, cancelled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::timestamped_float32_session_runtime::string_codec::{
        read as codec_read, read_record, write as codec_write,
    };
    use std::io::Write;
    use std::net::TcpStream;
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
            StringSampleRecord::new(1.0, String::new()).unwrap().value(),
            ""
        );
        assert_eq!(
            StringSampleRecord::new(1.0, "x".repeat(130)),
            Err(StringSampleError::ValueTooLong { actual: 130 })
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
            let mut damaged = vec![2];
            damaged.extend_from_slice(&1.0f64.to_le_bytes());
            damaged.extend_from_slice(&[1, 1, b'x']);
            damaged[9] = 2;
            stream.write_all(&damaged).unwrap();
        });
        let (mut stream, _) = listener.accept().unwrap();
        assert_eq!(
            read_record(&mut stream, limits(), &AtomicBool::new(false)),
            Err(StringSampleError::InvalidLengthForm { actual: 2 })
        );
        peer.join().unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = thread::spawn(move || TcpStream::connect(address).unwrap());
        let (mut stream, _) = listener.accept().unwrap();
        assert_eq!(
            codec_write(&mut stream, b"x", limits(), &AtomicBool::new(true)),
            Err(StringSampleError::Cancelled)
        );
        let mut byte = [0];
        assert_eq!(
            codec_read(
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

    #[test]
    fn lslc_003v_observed_utf8_cases_conform_without_production_change() {
        for (timestamp, value, expected_bytes) in [
            (1234.5, "μ雪🙂".to_owned(), 9),
            (1235.5, format!("μ{}", "a".repeat(125)), 127),
        ] {
            assert_eq!(value.len(), expected_bytes);
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let expected = value.clone();
            let worker = thread::spawn(move || {
                run_string_sample_outlet(
                    activation(),
                    listener,
                    &identity(),
                    handshake_limits(),
                    limits(),
                    StringSampleRecord::new(timestamp, value).unwrap(),
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
            assert_eq!(received.timestamp().to_bits(), timestamp.to_bits());
            assert_eq!(received.value(), expected);
            assert_eq!(worker.join().unwrap().unwrap(), address);
            assert!(TcpListener::bind(address).is_ok());
        }
    }

    #[test]
    fn lslc_003x_empty_string_preserves_timestamp_and_cleanup() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            run_string_sample_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                limits(),
                StringSampleRecord::new(1234.5, String::new()).unwrap(),
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
        assert_eq!(received.value(), "");
        assert_eq!(worker.join().unwrap().unwrap(), address);
        assert!(TcpListener::bind(address).is_ok());
    }

    #[test]
    fn lslc_003z_exact_128_bytes_preserve_timestamp_and_cleanup() {
        let value = "a".repeat(128);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let expected = value.clone();
        let worker = thread::spawn(move || {
            run_string_sample_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                limits(),
                StringSampleRecord::new(1236.5, value).unwrap(),
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
        assert_eq!(received.timestamp().to_bits(), 1236.5f64.to_bits());
        assert_eq!(received.value(), expected);
        assert_eq!(worker.join().unwrap().unwrap(), address);
        assert!(TcpListener::bind(address).is_ok());
        assert_eq!(
            StringSampleRecord::new(1236.5, "a".repeat(130)),
            Err(StringSampleError::ValueTooLong { actual: 130 })
        );
    }

    #[test]
    fn lslc_004b_exact_129_bytes_preserve_timestamp_and_cleanup() {
        let value = "q".repeat(129);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let expected = value.clone();
        let worker = thread::spawn(move || {
            run_string_sample_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                limits(),
                StringSampleRecord::new(1237.5, value).unwrap(),
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
        assert_eq!(received.timestamp().to_bits(), 1237.5f64.to_bits());
        assert_eq!(received.value(), expected);
        assert_eq!(worker.join().unwrap().unwrap(), address);
        assert!(TcpListener::bind(address).is_ok());
        assert_eq!(
            StringSampleRecord::new(1237.5, "q".repeat(130)),
            Err(StringSampleError::ValueTooLong { actual: 130 })
        );
    }
}
