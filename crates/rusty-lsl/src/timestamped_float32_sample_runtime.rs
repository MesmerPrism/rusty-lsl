// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! One bounded protocol-110 timestamped single-channel `float32` record.

#[cfg(test)]
use crate::stream_handshake::accept_handshake_stream;
#[cfg(test)]
use crate::timestamped_float32_session_runtime::codec::{
    initialization_sample, read_initialization, write_record, INITIALIZATION_VALUE_BITS,
    RECORD_BYTES, RECORD_MARKER,
};
use crate::{
    timestamped_float32_session_runtime::{
        TimestampedFloat32InletSession, TimestampedFloat32OutletSession,
        TimestampedFloat32SessionError, TimestampedFloat32SessionPreflightError,
    },
    RuntimeModule, RuntimeModuleCapability, StreamHandshakeActivation, StreamHandshakeError,
    StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedSample,
};
#[cfg(test)]
use crate::{RawSourceTimestamp, Sample, SampleLimits};
use std::io::ErrorKind;
#[cfg(test)]
use std::net::TcpStream;
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// Feature selected for the one-record sample effect.
pub const TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID: &str = "timestamped-float32-sample";
/// Exact effective marker required at runtime.
pub const TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER: &str =
    "rusty.lsl.timestamped_float32_sample.effective";

/// Closed activation composed with accepted handshake activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32SampleActivation {
    handshake: StreamHandshakeActivation,
}

impl TimestampedFloat32SampleActivation {
    /// Admits the exact selected feature and marker beside handshake activation.
    pub fn new(
        capability: RuntimeModuleCapability,
        handshake: StreamHandshakeActivation,
    ) -> Result<Self, TimestampedFloat32SampleActivationError> {
        if !capability.matches(RuntimeModule::TimestampedFloat32Sample) {
            return Err(TimestampedFloat32SampleActivationError::WrongModule);
        }
        Ok(Self { handshake })
    }
}

/// Rejected sample activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SampleActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}

/// Finite I/O limits for the one fixed-size record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampedFloat32SampleLimits {
    io_slice: Duration,
    total_deadline: Duration,
}

impl TimestampedFloat32SampleLimits {
    /// Creates explicit nonzero I/O slice and total deadline limits.
    pub fn new(
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, TimestampedFloat32SampleLimitError> {
        if io_slice.is_zero() {
            return Err(TimestampedFloat32SampleLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(TimestampedFloat32SampleLimitError::ZeroTotalDeadline);
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

/// Invalid one-record limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SampleLimitError {
    /// I/O slice was zero.
    ZeroIoSlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
}

/// Stable failure from connection setup or one-record transfer.
#[derive(Debug, Eq, PartialEq)]
pub enum TimestampedFloat32SampleError {
    /// Accepted handshake setup failed.
    Handshake(StreamHandshakeError),
    /// Caller supplied a shape other than one channel.
    ChannelCount {
        /// Actual channel count.
        actual: usize,
    },
    /// Caller cancellation was observed.
    Cancelled,
    /// Sample-stage deadline elapsed.
    Deadline,
    /// Sample-stage socket I/O failed.
    Io(ErrorKind),
    /// Peer closed before the fixed record completed.
    Truncated {
        /// Bytes received before close.
        actual: usize,
    },
    /// Fixed record marker differed.
    InvalidMarker {
        /// Observed marker.
        actual: u8,
    },
    /// Timestamp bytes decoded outside the finite raw timestamp domain.
    InvalidTimestamp,
    /// A required post-handshake initialization record differed.
    InvalidInitialization {
        /// Zero-based initialization record index.
        index: usize,
    },
}

/// Opens the accepted outlet handshake, sends exactly one record, and closes on return.
pub fn run_timestamped_float32_outlet(
    activation: TimestampedFloat32SampleActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    sample: &TimestampedSample<f32>,
    cancelled: &AtomicBool,
) -> Result<SocketAddr, TimestampedFloat32SampleError> {
    TimestampedFloat32OutletSession::preflight(
        activation,
        listener,
        identity,
        handshake_limits,
        sample_limits,
        std::slice::from_ref(sample),
    )
    .map_err(map_session_preflight_error)?
    .finish(cancelled)
    .map(|report| report.local_address())
    .map_err(map_session_error)
}

/// Opens the accepted inlet handshake, receives exactly one record, and closes on return.
pub fn run_timestamped_float32_inlet(
    activation: TimestampedFloat32SampleActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    cancelled: &AtomicBool,
) -> Result<TimestampedSample<f32>, TimestampedFloat32SampleError> {
    let mut records = TimestampedFloat32InletSession::preflight(
        activation,
        peer,
        identity,
        handshake_limits,
        sample_limits,
        1,
    )
    .expect("one record always passes session preflight")
    .finish(cancelled)
    .map_err(map_session_error)?
    .into_records();
    Ok(records
        .pop()
        .expect("one-record session reports one record"))
}

fn map_session_preflight_error(
    error: TimestampedFloat32SessionPreflightError,
) -> TimestampedFloat32SampleError {
    match error {
        TimestampedFloat32SessionPreflightError::ChannelCount { actual, .. } => {
            TimestampedFloat32SampleError::ChannelCount { actual }
        }
        TimestampedFloat32SessionPreflightError::RecordCount { .. } => {
            unreachable!("the legacy adapter always supplies exactly one record")
        }
        TimestampedFloat32SessionPreflightError::InconsistentChannelCount { .. } => {
            unreachable!("the legacy adapter always supplies exactly one record")
        }
    }
}

fn map_session_error(error: TimestampedFloat32SessionError) -> TimestampedFloat32SampleError {
    match error {
        TimestampedFloat32SessionError::Handshake(error) => {
            TimestampedFloat32SampleError::Handshake(error)
        }
        TimestampedFloat32SessionError::Record { error, .. } => error,
        TimestampedFloat32SessionError::TrailingByte { actual } => {
            TimestampedFloat32SampleError::InvalidMarker { actual }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use std::io::Write;
    use std::thread;

    fn handshake() -> StreamHandshakeActivation {
        StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake)).unwrap()
    }
    fn activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            handshake(),
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
    fn lslc_002t_one_loopback_sample_preserves_exact_bits_and_releases_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let sent = sample(-0.0, f32::from_bits(0x7fc0_1234));
        let worker = thread::spawn(move || {
            run_timestamped_float32_outlet(
                activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sent,
                &AtomicBool::new(false),
            )
        });
        let received = run_timestamped_float32_inlet(
            activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(
            received.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(received.sample().values()[0].to_bits(), 0x7fc0_1234);
        assert_eq!(worker.join().unwrap().unwrap(), address);
        TcpListener::bind(address).unwrap();
    }

    #[test]
    fn lslc_002t_activation_limits_shape_and_cancellation_fail_closed() {
        assert_eq!(
            TimestampedFloat32SampleActivation::new(
                test_capability(RuntimeModule::UdpDiscovery),
                handshake()
            ),
            Err(TimestampedFloat32SampleActivationError::WrongModule)
        );
        assert_eq!(
            TimestampedFloat32SampleLimits::new(Duration::ZERO, Duration::ZERO),
            Err(TimestampedFloat32SampleLimitError::ZeroIoSlice)
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (_reader, _) = listener.accept().unwrap();
        let two = TimestampedSample::new(
            Sample::new(SampleLimits::new(2).unwrap(), 2, vec![1.0, 2.0]).unwrap(),
            RawSourceTimestamp::new(1.0).unwrap(),
            None,
        );
        assert_eq!(
            write_record(&mut writer, &two, sample_limits(), &AtomicBool::new(false)),
            Err(TimestampedFloat32SampleError::ChannelCount { actual: 2 })
        );
        let cancelled = AtomicBool::new(true);
        assert_eq!(
            run_timestamped_float32_inlet(
                activation(),
                "127.0.0.1:9".parse().unwrap(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                &cancelled,
            ),
            Err(TimestampedFloat32SampleError::Handshake(
                StreamHandshakeError::Cancelled
            ))
        );
    }

    #[test]
    fn lslc_002t_truncated_and_damaged_records_are_typed() {
        let mut nonfinite = vec![RECORD_MARKER];
        nonfinite.extend_from_slice(&f64::INFINITY.to_le_bytes());
        nonfinite.extend_from_slice(&1.0f32.to_le_bytes());
        for (record, expected) in [
            (
                vec![2, 0],
                TimestampedFloat32SampleError::Truncated { actual: 2 },
            ),
            (
                vec![9; RECORD_BYTES],
                TimestampedFloat32SampleError::InvalidMarker { actual: 9 },
            ),
            (nonfinite, TimestampedFloat32SampleError::InvalidTimestamp),
        ] {
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
                stream.write_all(&record).unwrap();
            });
            assert_eq!(
                run_timestamped_float32_inlet(
                    activation(),
                    address,
                    &identity(),
                    handshake_limits(),
                    sample_limits(),
                    &AtomicBool::new(false),
                ),
                Err(expected)
            );
            worker.join().unwrap();
        }
    }

    #[test]
    fn lslc_002y_initialization_sequence_is_exact_and_ordered() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (mut reader, _) = listener.accept().unwrap();
        write_record(
            &mut writer,
            &initialization_sample(INITIALIZATION_VALUE_BITS[0]),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        write_record(
            &mut writer,
            &initialization_sample(INITIALIZATION_VALUE_BITS[1]),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        read_initialization(&mut reader, sample_limits(), &AtomicBool::new(false)).unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (mut reader, _) = listener.accept().unwrap();
        write_record(
            &mut writer,
            &initialization_sample(INITIALIZATION_VALUE_BITS[0]),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        write_record(
            &mut writer,
            &initialization_sample(3.0f32.to_bits()),
            sample_limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(
            read_initialization(&mut reader, sample_limits(), &AtomicBool::new(false)),
            Err(TimestampedFloat32SampleError::InvalidInitialization { index: 1 })
        );
    }
}
