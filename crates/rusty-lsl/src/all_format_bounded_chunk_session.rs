// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crate-private all-format bounded chunk projection into the sole session owner.

use crate::format_neutral_session_runtime::{
    project_bounded_chunk, BoundedChunkSessionProjection, SessionShapeError,
};
use crate::timestamped_float32_session_runtime::TimestampedFixedWidthIntegerSessionError;
use crate::{
    FixedWidthNumericSampleActivation, FixedWidthNumericSampleLimits, RawSourceTimestamp, Sample,
    SampleLimits, StreamHandshakeIdentity, StreamHandshakeLimits, StringSampleActivation,
    StringSampleLimits, StringSampleRecord, TimestampedDouble64InletSession,
    TimestampedDouble64OutletSession, TimestampedDouble64SessionError,
    TimestampedDouble64SessionIoLimits, TimestampedDouble64SessionLimits,
    TimestampedDouble64SessionPreflightError, TimestampedFloat32InletSession,
    TimestampedFloat32OutletSession, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits, TimestampedFloat32SessionError,
    TimestampedFloat32SessionLimits, TimestampedFloat32SessionPreflightError,
    TimestampedInt16InletSession, TimestampedInt16OutletSession, TimestampedInt16SessionLimits,
    TimestampedInt32InletSession, TimestampedInt32OutletSession, TimestampedInt32SessionLimits,
    TimestampedInt64InletSession, TimestampedInt64OutletSession, TimestampedInt64SessionLimits,
    TimestampedInt8InletSession, TimestampedInt8OutletSession, TimestampedInt8SessionLimits,
    TimestampedStringInletSession, TimestampedStringOutletSession, TimestampedStringSessionError,
    TimestampedStringSessionLimits, TimestampedStringSessionPreflightError,
};
use crate::{TimestampedChunk, TimestampedSample};
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::AtomicBool;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BoundedChunkFormat {
    Float32,
    Double64,
    String,
    Int32,
    Int64,
    Int16,
    Int8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum BoundedChunkProjectionError {
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
    UnsupportedShape {
        channels: usize,
        records: usize,
    },
    StringBytes {
        index: usize,
        actual: usize,
    },
}

impl From<SessionShapeError> for BoundedChunkProjectionError {
    fn from(error: SessionShapeError) -> Self {
        match error {
            SessionShapeError::RecordCount { actual } => Self::RecordCount { actual },
            SessionShapeError::ChannelCount { index, actual } => {
                Self::ChannelCount { index, actual }
            }
            SessionShapeError::InconsistentChannelCount {
                index,
                expected,
                actual,
            } => Self::InconsistentChannelCount {
                index,
                expected,
                actual,
            },
        }
    }
}

fn project<T>(
    format: BoundedChunkFormat,
    records: &[T],
    channels: impl Fn(&T) -> usize,
) -> Result<BoundedChunkSessionProjection, BoundedChunkProjectionError> {
    let (max_channels, max_records) = match format {
        BoundedChunkFormat::Float32 => (usize::MAX, usize::MAX),
        BoundedChunkFormat::String => (1, 1),
        BoundedChunkFormat::Double64
        | BoundedChunkFormat::Int32
        | BoundedChunkFormat::Int64
        | BoundedChunkFormat::Int16
        | BoundedChunkFormat::Int8 => (2, 3),
    };
    let projection = project_bounded_chunk(max_channels, max_records, records, channels)?;
    let shape = projection.shape();
    let accepted = match format {
        BoundedChunkFormat::Float32 => true,
        BoundedChunkFormat::String => shape.channels() == 1 && shape.records() == 1,
        _ => matches!((shape.channels(), shape.records()), (1, 1) | (2, 3)),
    };
    if !accepted {
        return Err(BoundedChunkProjectionError::UnsupportedShape {
            channels: shape.channels(),
            records: shape.records(),
        });
    }
    Ok(projection)
}

macro_rules! numeric_projection {
    ($name:ident, $value:ty, $format:ident) => {
        pub(crate) fn $name(
            chunk: &TimestampedChunk<$value>,
        ) -> Result<BoundedChunkSessionProjection, BoundedChunkProjectionError> {
            project(BoundedChunkFormat::$format, chunk.samples(), |record| {
                record.sample().declared_channels()
            })
        }
    };
}

numeric_projection!(project_float32_chunk, f32, Float32);
numeric_projection!(project_double64_chunk, f64, Double64);
numeric_projection!(project_int32_chunk, i32, Int32);
numeric_projection!(project_int64_chunk, i64, Int64);
numeric_projection!(project_int16_chunk, i16, Int16);
numeric_projection!(project_int8_chunk, i8, Int8);

pub(crate) fn project_string_chunk(
    chunk: &TimestampedChunk<String>,
) -> Result<BoundedChunkSessionProjection, BoundedChunkProjectionError> {
    let projection = project(BoundedChunkFormat::String, chunk.samples(), |record| {
        record.sample().declared_channels()
    })?;
    for (index, record) in chunk.samples().iter().enumerate() {
        let actual = record.sample().values()[0].len();
        if actual > 129 {
            return Err(BoundedChunkProjectionError::StringBytes { index, actual });
        }
    }
    Ok(projection)
}

/// Reconstitutes a received numeric chunk without copying its record allocation.
pub(crate) fn into_numeric_chunk<T>(
    records: Vec<TimestampedSample<T>>,
    channels: usize,
) -> TimestampedChunk<T> {
    let record_count = records.len();
    TimestampedChunk::new(
        crate::ChunkLimits::new(record_count, channels)
            .expect("a completed session has nonzero exact bounds"),
        records,
    )
    .expect("a completed homogeneous session is a valid exact chunk")
}

macro_rules! numeric_chunk_facade {
    (
        $value:ty, $variant:ident, $project:ident, $error:ident, $out_report:ident, $in_report:ident,
        $out_fn:ident, $in_fn:ident, $activation:ty, $io:ty, $limits:ty,
        $preflight:ty, $session:ty, $outlet:ty, $inlet:ty
    ) => {
        #[doc = concat!("Concrete bounded-chunk failure for ", stringify!($value), ".")]
        #[derive(Debug)]
        pub enum $error {
            /// The caller chunk failed the sole socket-free projection.
            Projection(BoundedChunkProjectionError),
            /// The existing concrete session rejected the projected shape.
            Preflight($preflight),
            /// The existing concrete session lifecycle failed unchanged.
            Session($session),
        }

        #[doc = concat!("Consuming bounded-chunk outlet report for ", stringify!($value), ".")]
        #[derive(Debug)]
        pub struct $out_report {
            local: SocketAddr,
            peer: SocketAddr,
            chunk: TimestampedChunk<$value>,
        }
        impl $out_report {
            /// Caller-bound listener address.
            pub const fn local_address(&self) -> SocketAddr {
                self.local
            }
            /// Accepted peer address.
            pub const fn peer(&self) -> SocketAddr {
                self.peer
            }
            /// Exact homogeneous channel count.
            pub fn channel_count(&self) -> usize {
                self.chunk.samples()[0].sample().declared_channels()
            }
            /// Exact caller-record count.
            pub fn record_count(&self) -> usize {
                self.chunk.samples().len()
            }
            /// Borrows the unchanged caller chunk.
            pub const fn chunk(&self) -> &TimestampedChunk<$value> {
                &self.chunk
            }
            /// Recovers the unchanged caller chunk and its allocations.
            pub fn into_chunk(self) -> TimestampedChunk<$value> {
                self.chunk
            }
        }

        #[doc = concat!("Consuming bounded-chunk inlet report for ", stringify!($value), ".")]
        #[derive(Debug)]
        pub struct $in_report {
            peer: SocketAddr,
            chunk: TimestampedChunk<$value>,
        }
        impl $in_report {
            /// Caller-selected peer address.
            pub const fn peer(&self) -> SocketAddr {
                self.peer
            }
            /// Exact homogeneous channel count.
            pub fn channel_count(&self) -> usize {
                self.chunk.samples()[0].sample().declared_channels()
            }
            /// Exact received record count.
            pub fn record_count(&self) -> usize {
                self.chunk.samples().len()
            }
            /// Borrows the ordered received chunk.
            pub const fn chunk(&self) -> &TimestampedChunk<$value> {
                &self.chunk
            }
            /// Recovers the received chunk without copying its record allocation.
            pub fn into_chunk(self) -> TimestampedChunk<$value> {
                self.chunk
            }
        }

        #[doc = concat!("Runs the concrete bounded-chunk outlet for ", stringify!($value), ".")]
        pub fn $out_fn(
            activation: $activation,
            listener: TcpListener,
            identity: &StreamHandshakeIdentity,
            handshake_limits: StreamHandshakeLimits,
            io_limits: $io,
            chunk: TimestampedChunk<$value>,
            cancelled: &AtomicBool,
        ) -> Result<$out_report, $error> {
            let projection = $project(&chunk).map_err($error::Projection)?;
            let shape = projection.shape();
            let limits = <$limits>::new(shape.channels(), shape.records())
                .expect("the sole projection admitted a concrete session shape");
            let report = <$outlet>::preflight_bounded(
                activation,
                listener,
                identity,
                handshake_limits,
                io_limits,
                limits,
                chunk.samples(),
            )
            .map_err($error::Preflight)?
            .finish(cancelled)
            .map_err($error::Session)?;
            Ok($out_report {
                local: report.local_address(),
                peer: report.peer(),
                chunk,
            })
        }

        #[doc = concat!("Runs the concrete bounded-chunk inlet for ", stringify!($value), ".")]
        pub fn $in_fn(
            activation: $activation,
            peer: SocketAddr,
            identity: &StreamHandshakeIdentity,
            handshake_limits: StreamHandshakeLimits,
            io_limits: $io,
            channel_count: usize,
            record_count: usize,
            cancelled: &AtomicBool,
        ) -> Result<$in_report, $error> {
            let limits = <$limits>::new(channel_count, record_count).map_err(|_| {
                $error::Projection(BoundedChunkProjectionError::UnsupportedShape {
                    channels: channel_count,
                    records: record_count,
                })
            })?;
            let report = <$inlet>::preflight_bounded(
                activation,
                peer,
                identity,
                handshake_limits,
                io_limits,
                limits,
                channel_count,
                record_count,
            )
            .map_err($error::Preflight)?
            .finish(cancelled)
            .map_err($error::Session)?;
            Ok($in_report {
                peer: report.peer(),
                chunk: into_numeric_chunk(report.into_records(), channel_count),
            })
        }
    };
}

numeric_chunk_facade!(
    f32,
    Float32,
    project_float32_chunk,
    TimestampedFloat32BoundedChunkError,
    TimestampedFloat32BoundedChunkOutletSessionReport,
    TimestampedFloat32BoundedChunkInletSessionReport,
    run_timestamped_float32_bounded_chunk_outlet,
    run_timestamped_float32_bounded_chunk_inlet,
    TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits,
    TimestampedFloat32SessionLimits,
    TimestampedFloat32SessionPreflightError,
    TimestampedFloat32SessionError,
    TimestampedFloat32OutletSession<'_>,
    TimestampedFloat32InletSession<'_>
);
numeric_chunk_facade!(
    f64,
    Double64,
    project_double64_chunk,
    TimestampedDouble64BoundedChunkError,
    TimestampedDouble64BoundedChunkOutletSessionReport,
    TimestampedDouble64BoundedChunkInletSessionReport,
    run_timestamped_double64_bounded_chunk_outlet,
    run_timestamped_double64_bounded_chunk_inlet,
    FixedWidthNumericSampleActivation,
    TimestampedDouble64SessionIoLimits,
    TimestampedDouble64SessionLimits,
    TimestampedDouble64SessionPreflightError,
    TimestampedDouble64SessionError,
    TimestampedDouble64OutletSession<'_>,
    TimestampedDouble64InletSession<'_>
);

macro_rules! integer_chunk_facade {
    ($value:ty, $variant:ident, $project:ident, $error:ident, $out_report:ident, $in_report:ident,
     $out_fn:ident, $in_fn:ident, $limits:ty, $outlet:ty, $inlet:ty) => {
        numeric_chunk_facade!(
            $value,
            $variant,
            $project,
            $error,
            $out_report,
            $in_report,
            $out_fn,
            $in_fn,
            FixedWidthNumericSampleActivation,
            FixedWidthNumericSampleLimits,
            $limits,
            TimestampedFloat32SessionPreflightError,
            TimestampedFixedWidthIntegerSessionError,
            $outlet,
            $inlet
        );
    };
}
integer_chunk_facade!(
    i64,
    Int64,
    project_int64_chunk,
    TimestampedInt64BoundedChunkError,
    TimestampedInt64BoundedChunkOutletSessionReport,
    TimestampedInt64BoundedChunkInletSessionReport,
    run_timestamped_int64_bounded_chunk_outlet,
    run_timestamped_int64_bounded_chunk_inlet,
    TimestampedInt64SessionLimits,
    TimestampedInt64OutletSession<'_>,
    TimestampedInt64InletSession<'_>
);
integer_chunk_facade!(
    i32,
    Int32,
    project_int32_chunk,
    TimestampedInt32BoundedChunkError,
    TimestampedInt32BoundedChunkOutletSessionReport,
    TimestampedInt32BoundedChunkInletSessionReport,
    run_timestamped_int32_bounded_chunk_outlet,
    run_timestamped_int32_bounded_chunk_inlet,
    TimestampedInt32SessionLimits,
    TimestampedInt32OutletSession<'_>,
    TimestampedInt32InletSession<'_>
);
integer_chunk_facade!(
    i16,
    Int16,
    project_int16_chunk,
    TimestampedInt16BoundedChunkError,
    TimestampedInt16BoundedChunkOutletSessionReport,
    TimestampedInt16BoundedChunkInletSessionReport,
    run_timestamped_int16_bounded_chunk_outlet,
    run_timestamped_int16_bounded_chunk_inlet,
    TimestampedInt16SessionLimits,
    TimestampedInt16OutletSession<'_>,
    TimestampedInt16InletSession<'_>
);
integer_chunk_facade!(
    i8,
    Int8,
    project_int8_chunk,
    TimestampedInt8BoundedChunkError,
    TimestampedInt8BoundedChunkOutletSessionReport,
    TimestampedInt8BoundedChunkInletSessionReport,
    run_timestamped_int8_bounded_chunk_outlet,
    run_timestamped_int8_bounded_chunk_inlet,
    TimestampedInt8SessionLimits,
    TimestampedInt8OutletSession<'_>,
    TimestampedInt8InletSession<'_>
);

/// Concrete bounded String chunk failure.
#[derive(Debug)]
pub enum TimestampedStringBoundedChunkError {
    /// The caller chunk failed the sole socket-free projection.
    Projection(BoundedChunkProjectionError),
    /// The existing concrete String session rejected the projected shape.
    Preflight(TimestampedStringSessionPreflightError),
    /// The existing concrete String lifecycle failed unchanged.
    Session(TimestampedStringSessionError),
}

/// Consuming String bounded-chunk outlet report.
#[derive(Debug)]
pub struct TimestampedStringBoundedChunkOutletSessionReport {
    local: SocketAddr,
    peer: SocketAddr,
    chunk: TimestampedChunk<String>,
}
impl TimestampedStringBoundedChunkOutletSessionReport {
    /// Caller-bound listener address.
    pub const fn local_address(&self) -> SocketAddr {
        self.local
    }
    /// Accepted peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Exact one-channel shape.
    pub const fn channel_count(&self) -> usize {
        1
    }
    /// Exact one-record shape.
    pub const fn record_count(&self) -> usize {
        1
    }
    /// Borrows the unchanged caller chunk.
    pub const fn chunk(&self) -> &TimestampedChunk<String> {
        &self.chunk
    }
    /// Recovers the unchanged caller chunk and String allocation.
    pub fn into_chunk(self) -> TimestampedChunk<String> {
        self.chunk
    }
}

/// Consuming String bounded-chunk inlet report.
#[derive(Debug)]
pub struct TimestampedStringBoundedChunkInletSessionReport {
    peer: SocketAddr,
    chunk: TimestampedChunk<String>,
}
impl TimestampedStringBoundedChunkInletSessionReport {
    /// Caller-selected peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Exact one-channel shape.
    pub const fn channel_count(&self) -> usize {
        1
    }
    /// Exact one-record shape.
    pub const fn record_count(&self) -> usize {
        1
    }
    /// Borrows the received chunk.
    pub const fn chunk(&self) -> &TimestampedChunk<String> {
        &self.chunk
    }
    /// Recovers the received chunk and String allocation.
    pub fn into_chunk(self) -> TimestampedChunk<String> {
        self.chunk
    }
}

/// Runs the concrete one-channel, one-record bounded String chunk outlet.
pub fn run_timestamped_string_bounded_chunk_outlet(
    activation: StringSampleActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: StringSampleLimits,
    chunk: TimestampedChunk<String>,
    cancelled: &AtomicBool,
) -> Result<TimestampedStringBoundedChunkOutletSessionReport, TimestampedStringBoundedChunkError> {
    project_string_chunk(&chunk).map_err(TimestampedStringBoundedChunkError::Projection)?;
    let mut samples = chunk.into_samples();
    let sample = samples
        .pop()
        .expect("the sole projection admitted one record");
    let (sample, timestamp, _) = sample.into_parts();
    let timestamp = timestamp.value();
    let value = sample.into_values().pop().expect("one String channel");
    let record = StringSampleRecord::new(timestamp, value).map_err(|error| {
        TimestampedStringBoundedChunkError::Session(TimestampedStringSessionError::Record {
            index: Some(0),
            error,
        })
    })?;
    let records = [record];
    let report = TimestampedStringOutletSession::preflight_bounded(
        activation,
        listener,
        identity,
        handshake_limits,
        io_limits,
        TimestampedStringSessionLimits::new(1, 1).expect("closed String shape"),
        &records,
    )
    .map_err(TimestampedStringBoundedChunkError::Preflight)?
    .finish(cancelled)
    .map_err(TimestampedStringBoundedChunkError::Session)?;
    let record = records.into_iter().next().expect("one String record");
    let chunk = string_record_into_chunk(record);
    Ok(TimestampedStringBoundedChunkOutletSessionReport {
        local: report.local_address(),
        peer: report.peer(),
        chunk,
    })
}

/// Runs the concrete one-channel, one-record bounded String chunk inlet.
pub fn run_timestamped_string_bounded_chunk_inlet(
    activation: StringSampleActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: StringSampleLimits,
    cancelled: &AtomicBool,
) -> Result<TimestampedStringBoundedChunkInletSessionReport, TimestampedStringBoundedChunkError> {
    let report = TimestampedStringInletSession::preflight_bounded(
        activation,
        peer,
        identity,
        handshake_limits,
        io_limits,
        TimestampedStringSessionLimits::new(1, 1).expect("closed String shape"),
        1,
        1,
    )
    .map_err(TimestampedStringBoundedChunkError::Preflight)?
    .finish(cancelled)
    .map_err(TimestampedStringBoundedChunkError::Session)?;
    let record = report
        .into_records()
        .pop()
        .expect("closed String shape returned one record");
    let chunk = string_record_into_chunk(record);
    Ok(TimestampedStringBoundedChunkInletSessionReport { peer, chunk })
}

fn string_record_into_chunk(record: StringSampleRecord) -> TimestampedChunk<String> {
    let timestamp =
        RawSourceTimestamp::new(record.timestamp()).expect("session validated timestamp");
    let sample = Sample::new(
        SampleLimits::new(1).expect("one channel"),
        1,
        vec![record.into_value()],
    )
    .expect("one declared String channel");
    into_numeric_chunk(vec![TimestampedSample::new(sample, timestamp, None)], 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChunkLimits, RawSourceTimestamp, Sample, SampleLimits};

    fn sample<T>(values: Vec<T>) -> TimestampedSample<T> {
        let channels = values.len();
        TimestampedSample::new(
            Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap(),
            RawSourceTimestamp::new(17.25).unwrap(),
            None,
        )
    }

    fn chunk<T>(records: Vec<TimestampedSample<T>>, channels: usize) -> TimestampedChunk<T> {
        TimestampedChunk::new(ChunkLimits::new(records.len(), channels).unwrap(), records).unwrap()
    }

    #[test]
    fn p31_all_numeric_formats_preserve_only_their_accepted_shapes() {
        let one = chunk(vec![sample(vec![1])], 1);
        let three = chunk(
            vec![sample(vec![1, 2]), sample(vec![3, 4]), sample(vec![5, 6])],
            2,
        );
        assert_eq!(project_int32_chunk(&one).unwrap().shape().records(), 1);
        assert_eq!(project_int64_chunk(&three).unwrap().shape().channels(), 2);
        let one_i16 = chunk(vec![sample(vec![1_i16])], 1);
        let three_i8 = chunk(
            vec![
                sample(vec![1_i8, 2]),
                sample(vec![3, 4]),
                sample(vec![5, 6]),
            ],
            2,
        );
        assert_eq!(project_int16_chunk(&one_i16).unwrap().shape().records(), 1);
        assert_eq!(project_int8_chunk(&three_i8).unwrap().shape().records(), 3);
        let doubles = chunk(vec![sample(vec![f64::from_bits(0x3ff0_0000_0000_0001)])], 1);
        assert_eq!(
            project_double64_chunk(&doubles).unwrap().shape().channels(),
            1
        );
        let floats = chunk(vec![sample(vec![f32::from_bits(0x3f80_0001)])], 1);
        assert_eq!(project_float32_chunk(&floats).unwrap().shape().records(), 1);
    }

    #[test]
    fn p31_fixed_formats_reject_crossed_shapes_before_session_entry() {
        let crossed = chunk(vec![sample(vec![1_i32, 2])], 2);
        assert_eq!(
            project_int32_chunk(&crossed),
            Err(BoundedChunkProjectionError::UnsupportedShape {
                channels: 2,
                records: 1,
            })
        );
    }

    #[test]
    fn p31_numeric_inlet_projection_preserves_record_allocation_and_bits() {
        let records = vec![sample(vec![f64::from_bits(0x4008_0000_0000_0001)])];
        let pointer = records.as_ptr();
        let chunk = into_numeric_chunk(records, 1);
        assert_eq!(chunk.samples().as_ptr(), pointer);
        assert_eq!(
            chunk.samples()[0].sample().values()[0].to_bits(),
            0x4008_0000_0000_0001
        );
    }

    #[test]
    fn p31_string_projection_keeps_exact_one_by_one_bound() {
        let strings = chunk(vec![sample(vec!["é".repeat(64) + "x"])], 1);
        assert_eq!(project_string_chunk(&strings).unwrap().shape().records(), 1);
        let oversized = chunk(vec![sample(vec!["é".repeat(65)])], 1);
        assert_eq!(
            project_string_chunk(&oversized),
            Err(BoundedChunkProjectionError::StringBytes {
                index: 0,
                actual: 130,
            })
        );
    }
}
