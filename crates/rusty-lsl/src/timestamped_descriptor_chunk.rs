// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{
    BoundTimestampedDescriptorSample, ChunkLimits, DescriptorSampleError, DescriptorSampleLimits,
    StreamDescriptor, TimestampedChunk, TimestampedDescriptorSampleInput, TimestampedSample,
};

/// One unvalidated timestamped descriptor/chunk composition input.
///
/// Each variant owns an already validated homogeneous [`TimestampedChunk`]
/// and maps transitively through CORE-006 to one existing data-only channel
/// format. The input is not accepted composition state until every ordered
/// sample has been delegated through [`BoundTimestampedDescriptorSample::new`].
#[derive(Debug, PartialEq)]
pub enum TimestampedDescriptorChunkInput {
    /// A timestamped homogeneous chunk of `f32` values.
    Float32(TimestampedChunk<f32>),
    /// A timestamped homogeneous chunk of `f64` values.
    Double64(TimestampedChunk<f64>),
    /// A timestamped homogeneous chunk of owned string values.
    String(TimestampedChunk<String>),
    /// A timestamped homogeneous chunk of `i32` values.
    Int32(TimestampedChunk<i32>),
    /// A timestamped homogeneous chunk of `i16` values.
    Int16(TimestampedChunk<i16>),
    /// A timestamped homogeneous chunk of `i8` values.
    Int8(TimestampedChunk<i8>),
    /// A timestamped homogeneous chunk of `i64` values.
    Int64(TimestampedChunk<i64>),
}

/// One non-empty timestamped chunk accepted against an exact descriptor shape.
///
/// Accepted state owns only the original chunk limits and the caller-ordered
/// CORE-006 bindings. Private fields prevent callers from forging acceptance
/// or changing sample/timestamp pairings.
#[derive(Debug, PartialEq)]
pub struct BoundTimestampedDescriptorChunk {
    chunk_limits: ChunkLimits,
    bound_samples: Vec<BoundTimestampedDescriptorSample>,
}

impl BoundTimestampedDescriptorChunk {
    /// Binds one non-empty timestamped homogeneous chunk to a descriptor.
    ///
    /// Empty input is rejected before sample delegation. Every other sample is
    /// moved in caller order exactly once through CORE-006, which retains all
    /// descriptor format, channel-count, String-bound, and timestamp evidence
    /// semantics. This constructor performs no rechunking or runtime action.
    pub fn new(
        limits: DescriptorSampleLimits,
        descriptor: &StreamDescriptor,
        input: TimestampedDescriptorChunkInput,
    ) -> Result<Self, TimestampedDescriptorChunkError> {
        match input {
            TimestampedDescriptorChunkInput::Float32(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::Float32,
            ),
            TimestampedDescriptorChunkInput::Double64(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::Double64,
            ),
            TimestampedDescriptorChunkInput::String(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::String,
            ),
            TimestampedDescriptorChunkInput::Int32(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::Int32,
            ),
            TimestampedDescriptorChunkInput::Int16(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::Int16,
            ),
            TimestampedDescriptorChunkInput::Int8(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::Int8,
            ),
            TimestampedDescriptorChunkInput::Int64(chunk) => Self::bind_chunk(
                limits,
                descriptor,
                chunk,
                TimestampedDescriptorSampleInput::Int64,
            ),
        }
    }

    fn bind_chunk<T>(
        limits: DescriptorSampleLimits,
        descriptor: &StreamDescriptor,
        chunk: TimestampedChunk<T>,
        wrap: fn(TimestampedSample<T>) -> TimestampedDescriptorSampleInput,
    ) -> Result<Self, TimestampedDescriptorChunkError> {
        let chunk_limits = chunk.limits();
        let samples = chunk.into_samples();
        if samples.is_empty() {
            return Err(TimestampedDescriptorChunkError::EmptyChunk);
        }

        let mut bound_samples = Vec::with_capacity(samples.len());
        for (sample_index, sample) in samples.into_iter().enumerate() {
            let bound = BoundTimestampedDescriptorSample::new(limits, descriptor, wrap(sample))
                .map_err(|error| TimestampedDescriptorChunkError::SampleRejected {
                    sample_index,
                    error,
                })?;
            bound_samples.push(bound);
        }

        Ok(Self {
            chunk_limits,
            bound_samples,
        })
    }

    /// Returns the original limits under which the input chunk was accepted.
    #[must_use]
    pub const fn chunk_limits(&self) -> ChunkLimits {
        self.chunk_limits
    }

    /// Returns the unchanged caller-ordered CORE-006 bindings.
    #[must_use]
    pub fn bound_samples(&self) -> &[BoundTimestampedDescriptorSample] {
        &self.bound_samples
    }

    /// Returns the original chunk limits and ordered accepted bindings.
    #[must_use]
    pub fn into_parts(self) -> (ChunkLimits, Vec<BoundTimestampedDescriptorSample>) {
        (self.chunk_limits, self.bound_samples)
    }
}

/// Deterministic rejection from non-empty chunk composition or delegation.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TimestampedDescriptorChunkError {
    /// The existing validated chunk contained no samples.
    EmptyChunk,
    /// The first rejected sample failed unchanged CORE-006/CORE-005 validation.
    SampleRejected {
        /// The zero-based caller-order location of the rejected sample.
        sample_index: usize,
        /// The unchanged delegated descriptor/sample error.
        error: DescriptorSampleError,
    },
}

impl fmt::Display for TimestampedDescriptorChunkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "timestamped descriptor/chunk composition rejected input: {self:?}"
        )
    }
}

impl std::error::Error for TimestampedDescriptorChunkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmptyChunk => None,
            Self::SampleRejected { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoundTimestampedDescriptorChunk, TimestampedDescriptorChunkError,
        TimestampedDescriptorChunkInput,
    };
    use crate::{
        ChannelFormat, DerivedTimestamp, DerivedTimestampKind, DescriptorSampleError,
        DescriptorSampleInput, DescriptorSampleLimits, NominalSampleRate, RawSourceTimestamp,
        Sample, SampleLimits, StreamDescriptor, StreamDescriptorLimits, TimestampedChunk,
        TimestampedSample,
    };

    fn descriptor(channel_count: usize, channel_format: ChannelFormat) -> StreamDescriptor {
        StreamDescriptor::new(
            StreamDescriptorLimits::new(1, 1, 1, channel_count).unwrap(),
            "s".to_owned(),
            None,
            None,
            channel_count,
            NominalSampleRate::Irregular,
            channel_format,
        )
        .unwrap()
    }

    fn timestamped<T>(
        values: Vec<T>,
        raw_bits: u64,
        derived: Option<(DerivedTimestampKind, u64)>,
    ) -> TimestampedSample<T> {
        let channels = values.len();
        TimestampedSample::new(
            Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap(),
            RawSourceTimestamp::new(f64::from_bits(raw_bits)).unwrap(),
            derived.map(|(kind, bits)| DerivedTimestamp::new(kind, f64::from_bits(bits)).unwrap()),
        )
    }

    fn chunk<T>(samples: Vec<TimestampedSample<T>>, max_channels: usize) -> TimestampedChunk<T> {
        let limits = ChunkLimits::new(samples.len().max(1) + 2, max_channels + 2).unwrap();
        TimestampedChunk::new(limits, samples).unwrap()
    }

    use crate::ChunkLimits;

    fn bind(
        descriptor: &StreamDescriptor,
        input: TimestampedDescriptorChunkInput,
    ) -> Result<BoundTimestampedDescriptorChunk, TimestampedDescriptorChunkError> {
        BoundTimestampedDescriptorChunk::new(
            DescriptorSampleLimits::new(8).unwrap(),
            descriptor,
            input,
        )
    }

    #[test]
    fn core_007_all_seven_timestamped_chunk_mappings_bind_exactly() {
        let raw = 1.0_f64.to_bits();
        let cases = [
            (
                ChannelFormat::Float32,
                TimestampedDescriptorChunkInput::Float32(chunk(
                    vec![timestamped(vec![1.0_f32], raw, None)],
                    1,
                )),
            ),
            (
                ChannelFormat::Double64,
                TimestampedDescriptorChunkInput::Double64(chunk(
                    vec![timestamped(vec![1.0_f64], raw, None)],
                    1,
                )),
            ),
            (
                ChannelFormat::String,
                TimestampedDescriptorChunkInput::String(chunk(
                    vec![timestamped(vec!["x".to_owned()], raw, None)],
                    1,
                )),
            ),
            (
                ChannelFormat::Int32,
                TimestampedDescriptorChunkInput::Int32(chunk(
                    vec![timestamped(vec![1_i32], raw, None)],
                    1,
                )),
            ),
            (
                ChannelFormat::Int16,
                TimestampedDescriptorChunkInput::Int16(chunk(
                    vec![timestamped(vec![1_i16], raw, None)],
                    1,
                )),
            ),
            (
                ChannelFormat::Int8,
                TimestampedDescriptorChunkInput::Int8(chunk(
                    vec![timestamped(vec![1_i8], raw, None)],
                    1,
                )),
            ),
            (
                ChannelFormat::Int64,
                TimestampedDescriptorChunkInput::Int64(chunk(
                    vec![timestamped(vec![1_i64], raw, None)],
                    1,
                )),
            ),
        ];

        for (format, input) in cases {
            let accepted = bind(&descriptor(1, format), input).unwrap();
            assert_eq!(accepted.bound_samples().len(), 1);
            assert_eq!(
                accepted.bound_samples()[0].bound_sample().channel_format(),
                format
            );
        }
    }

    #[test]
    fn core_007_original_chunk_limits_survive_read_only_and_consuming_accessors() {
        let limits = ChunkLimits::new(7, 5).unwrap();
        let input = TimestampedChunk::new(
            limits,
            vec![timestamped(vec![1_i32, 2], 3.0_f64.to_bits(), None)],
        )
        .unwrap();
        let accepted = bind(
            &descriptor(2, ChannelFormat::Int32),
            TimestampedDescriptorChunkInput::Int32(input),
        )
        .unwrap();

        assert_eq!(accepted.chunk_limits(), limits);
        let (returned_limits, samples) = accepted.into_parts();
        assert_eq!(returned_limits, limits);
        assert_eq!(samples.len(), 1);
    }

    #[test]
    fn core_007_multi_sample_order_and_timestamp_pairing_are_exact() {
        let raw_negative_zero = (-0.0_f64).to_bits();
        let raw_finite = 0x4009_21fb_5444_2d18;
        let corrected_negative_zero = (-0.0_f64).to_bits();
        let smoothed_finite = 0x4014_0000_0000_0001;
        let input = chunk(
            vec![
                timestamped(vec![10_i32, 11], raw_negative_zero, None),
                timestamped(
                    vec![20, 21],
                    raw_finite,
                    Some((
                        DerivedTimestampKind::ClockCorrected,
                        corrected_negative_zero,
                    )),
                ),
                timestamped(
                    vec![30, 31],
                    6.0_f64.to_bits(),
                    Some((DerivedTimestampKind::Smoothed, smoothed_finite)),
                ),
            ],
            2,
        );
        let accepted = bind(
            &descriptor(2, ChannelFormat::Int32),
            TimestampedDescriptorChunkInput::Int32(input),
        )
        .unwrap();

        let samples = accepted.bound_samples();
        assert_eq!(
            samples[0].raw_source_timestamp().value().to_bits(),
            raw_negative_zero
        );
        assert_eq!(samples[0].derived_timestamp(), None);
        assert_eq!(
            samples[1].raw_source_timestamp().value().to_bits(),
            raw_finite
        );
        assert_eq!(
            samples[1].derived_timestamp().unwrap().kind(),
            DerivedTimestampKind::ClockCorrected
        );
        assert_eq!(
            samples[1].derived_timestamp().unwrap().value().to_bits(),
            corrected_negative_zero
        );
        assert_eq!(
            samples[2].derived_timestamp().unwrap().kind(),
            DerivedTimestampKind::Smoothed
        );
        assert_eq!(
            samples[2].derived_timestamp().unwrap().value().to_bits(),
            smoothed_finite
        );
        for (sample, expected) in samples.iter().zip([[10, 11], [20, 21], [30, 31]]) {
            match sample.bound_sample().sample() {
                DescriptorSampleInput::Int32(value) => assert_eq!(value.values(), expected),
                other => panic!("unexpected accepted sample: {other:?}"),
            }
        }
    }

    #[test]
    fn core_007_float_signed_zero_and_nan_payload_bits_are_preserved() {
        let f32_bits = [(-0.0_f32).to_bits(), 0x7fc0_1234];
        let f64_bits = [(-0.0_f64).to_bits(), 0x7ff8_0000_0000_1234];
        let accepted_f32 = bind(
            &descriptor(2, ChannelFormat::Float32),
            TimestampedDescriptorChunkInput::Float32(chunk(
                vec![timestamped(
                    f32_bits.map(f32::from_bits).to_vec(),
                    1.0_f64.to_bits(),
                    None,
                )],
                2,
            )),
        )
        .unwrap();
        let accepted_f64 = bind(
            &descriptor(2, ChannelFormat::Double64),
            TimestampedDescriptorChunkInput::Double64(chunk(
                vec![timestamped(
                    f64_bits.map(f64::from_bits).to_vec(),
                    2.0_f64.to_bits(),
                    None,
                )],
                2,
            )),
        )
        .unwrap();

        match accepted_f32.bound_samples()[0].bound_sample().sample() {
            DescriptorSampleInput::Float32(sample) => assert_eq!(
                sample
                    .values()
                    .iter()
                    .map(|v| v.to_bits())
                    .collect::<Vec<_>>(),
                f32_bits
            ),
            other => panic!("unexpected accepted sample: {other:?}"),
        }
        match accepted_f64.bound_samples()[0].bound_sample().sample() {
            DescriptorSampleInput::Double64(sample) => assert_eq!(
                sample
                    .values()
                    .iter()
                    .map(|v| v.to_bits())
                    .collect::<Vec<_>>(),
                f64_bits
            ),
            other => panic!("unexpected accepted sample: {other:?}"),
        }
    }

    #[test]
    fn core_007_integer_edges_and_order_are_preserved() {
        let raw = 1.0_f64.to_bits();
        let cases = [
            TimestampedDescriptorChunkInput::Int32(chunk(
                vec![timestamped(vec![i32::MIN, 0, i32::MAX], raw, None)],
                3,
            )),
            TimestampedDescriptorChunkInput::Int16(chunk(
                vec![timestamped(vec![i16::MIN, 0, i16::MAX], raw, None)],
                3,
            )),
            TimestampedDescriptorChunkInput::Int8(chunk(
                vec![timestamped(vec![i8::MIN, 0, i8::MAX], raw, None)],
                3,
            )),
            TimestampedDescriptorChunkInput::Int64(chunk(
                vec![timestamped(vec![i64::MIN, 0, i64::MAX], raw, None)],
                3,
            )),
        ];
        let formats = [
            ChannelFormat::Int32,
            ChannelFormat::Int16,
            ChannelFormat::Int8,
            ChannelFormat::Int64,
        ];

        for (input, format) in cases.into_iter().zip(formats) {
            let accepted = bind(&descriptor(3, format), input).unwrap();
            assert_eq!(
                accepted.bound_samples()[0].bound_sample().channel_format(),
                format
            );
            match accepted.bound_samples()[0].bound_sample().sample() {
                DescriptorSampleInput::Int32(sample) => {
                    assert_eq!(sample.values(), [i32::MIN, 0, i32::MAX]);
                }
                DescriptorSampleInput::Int16(sample) => {
                    assert_eq!(sample.values(), [i16::MIN, 0, i16::MAX]);
                }
                DescriptorSampleInput::Int8(sample) => {
                    assert_eq!(sample.values(), [i8::MIN, 0, i8::MAX]);
                }
                DescriptorSampleInput::Int64(sample) => {
                    assert_eq!(sample.values(), [i64::MIN, 0, i64::MAX]);
                }
                other => panic!("unexpected accepted sample: {other:?}"),
            }
        }
    }

    #[test]
    fn core_007_string_values_order_and_allocations_are_moved_unchanged() {
        let values = vec!["".to_owned(), "é🙂".to_owned(), "  exact  ".to_owned()];
        let pointers = values
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let input = chunk(vec![timestamped(values, 1.0_f64.to_bits(), None)], 3);
        let accepted = BoundTimestampedDescriptorChunk::new(
            DescriptorSampleLimits::new(9).unwrap(),
            &descriptor(3, ChannelFormat::String),
            TimestampedDescriptorChunkInput::String(input),
        )
        .unwrap();

        match accepted.bound_samples()[0].bound_sample().sample() {
            DescriptorSampleInput::String(sample) => {
                assert_eq!(sample.values(), ["", "é🙂", "  exact  "]);
                assert_eq!(
                    sample
                        .values()
                        .iter()
                        .map(|value| value.as_ptr())
                        .collect::<Vec<_>>(),
                    pointers
                );
            }
            other => panic!("unexpected accepted sample: {other:?}"),
        }
    }

    #[test]
    fn core_007_empty_chunk_rejects_before_sample_delegation() {
        let empty =
            TimestampedChunk::<String>::new(ChunkLimits::new(4, 3).unwrap(), vec![]).unwrap();
        assert_eq!(
            BoundTimestampedDescriptorChunk::new(
                DescriptorSampleLimits::new(1).unwrap(),
                &descriptor(1, ChannelFormat::Int32),
                TimestampedDescriptorChunkInput::String(empty),
            ),
            Err(TimestampedDescriptorChunkError::EmptyChunk)
        );
    }

    #[test]
    fn core_007_format_mismatch_at_sample_zero_is_unchanged() {
        let input = chunk(vec![timestamped(vec![1_i32], 1.0_f64.to_bits(), None)], 1);
        assert_eq!(
            bind(
                &descriptor(1, ChannelFormat::Int16),
                TimestampedDescriptorChunkInput::Int32(input)
            ),
            Err(TimestampedDescriptorChunkError::SampleRejected {
                sample_index: 0,
                error: DescriptorSampleError::ChannelFormatMismatch {
                    expected: ChannelFormat::Int16,
                    actual: ChannelFormat::Int32
                },
            })
        );
    }

    #[test]
    fn core_007_channel_mismatch_at_sample_zero_is_unchanged() {
        let input = chunk(
            vec![timestamped(vec![1_i32, 2], 1.0_f64.to_bits(), None)],
            2,
        );
        assert_eq!(
            bind(
                &descriptor(1, ChannelFormat::Int32),
                TimestampedDescriptorChunkInput::Int32(input)
            ),
            Err(TimestampedDescriptorChunkError::SampleRejected {
                sample_index: 0,
                error: DescriptorSampleError::ChannelCountMismatch {
                    expected: 1,
                    actual: 2
                },
            })
        );
    }

    #[test]
    fn core_007_later_string_failure_retains_sample_and_channel_indexes() {
        let input = chunk(
            vec![
                timestamped(
                    vec!["ok".to_owned(), "ok".to_owned()],
                    1.0_f64.to_bits(),
                    None,
                ),
                timestamped(
                    vec!["ok".to_owned(), "é🙂x".to_owned()],
                    2.0_f64.to_bits(),
                    None,
                ),
            ],
            2,
        );
        assert_eq!(
            BoundTimestampedDescriptorChunk::new(
                DescriptorSampleLimits::new(2).unwrap(),
                &descriptor(2, ChannelFormat::String),
                TimestampedDescriptorChunkInput::String(input),
            ),
            Err(TimestampedDescriptorChunkError::SampleRejected {
                sample_index: 1,
                error: DescriptorSampleError::StringValueLimitExceeded {
                    channel_index: 1,
                    expected_max: 2,
                    actual: 3
                },
            })
        );
    }

    #[test]
    fn core_007_first_failure_order_and_delegated_precedence_are_unchanged() {
        let input = chunk(
            vec![
                timestamped(
                    vec!["ok".to_owned(), "long".to_owned()],
                    1.0_f64.to_bits(),
                    None,
                ),
                timestamped(
                    vec!["later".to_owned(), "ok".to_owned()],
                    2.0_f64.to_bits(),
                    None,
                ),
            ],
            2,
        );
        assert_eq!(
            BoundTimestampedDescriptorChunk::new(
                DescriptorSampleLimits::new(2).unwrap(),
                &descriptor(1, ChannelFormat::Int32),
                TimestampedDescriptorChunkInput::String(input),
            ),
            Err(TimestampedDescriptorChunkError::SampleRejected {
                sample_index: 0,
                error: DescriptorSampleError::ChannelFormatMismatch {
                    expected: ChannelFormat::Int32,
                    actual: ChannelFormat::String
                },
            })
        );

        let input = chunk(
            vec![
                timestamped(
                    vec!["ok".to_owned(), "long".to_owned()],
                    1.0_f64.to_bits(),
                    None,
                ),
                timestamped(
                    vec!["later".to_owned(), "ok".to_owned()],
                    2.0_f64.to_bits(),
                    None,
                ),
            ],
            2,
        );
        assert_eq!(
            BoundTimestampedDescriptorChunk::new(
                DescriptorSampleLimits::new(2).unwrap(),
                &descriptor(2, ChannelFormat::String),
                TimestampedDescriptorChunkInput::String(input),
            ),
            Err(TimestampedDescriptorChunkError::SampleRejected {
                sample_index: 0,
                error: DescriptorSampleError::StringValueLimitExceeded {
                    channel_index: 1,
                    expected_max: 2,
                    actual: 4
                },
            })
        );
    }
}
