// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crate-private all-format bounded chunk projection into the sole session owner.

use crate::format_neutral_session_runtime::{
    project_bounded_chunk, BoundedChunkSessionProjection, SessionShapeError,
};
use crate::{TimestampedChunk, TimestampedSample};

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
pub(crate) enum BoundedChunkProjectionError {
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
        format: BoundedChunkFormat,
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
            format,
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
                format: BoundedChunkFormat::Int32,
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
