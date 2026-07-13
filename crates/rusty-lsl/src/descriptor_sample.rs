// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{ChannelFormat, Sample, StreamDescriptor};

/// Identifies one configured descriptor/sample binding bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DescriptorSampleBound {
    /// Maximum Unicode scalar-value count in each string channel value.
    StringValueCodePoints,
}

/// Explicit limits applied while binding a sample to a descriptor shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DescriptorSampleLimits {
    max_string_value_code_points: usize,
}

impl DescriptorSampleLimits {
    /// Validates the per-string Unicode scalar-value maximum.
    pub fn new(max_string_value_code_points: usize) -> Result<Self, DescriptorSampleError> {
        if max_string_value_code_points == 0 {
            return Err(DescriptorSampleError::InvalidLimit {
                bound: DescriptorSampleBound::StringValueCodePoints,
                expected_min: 1,
                actual: max_string_value_code_points,
            });
        }

        Ok(Self {
            max_string_value_code_points,
        })
    }

    /// Returns the maximum Unicode scalar-value count for each string channel.
    #[must_use]
    pub const fn max_string_value_code_points(self) -> usize {
        self.max_string_value_code_points
    }
}

/// One unvalidated descriptor/sample binding input.
///
/// Each variant owns an already validated homogeneous [`Sample`] and names its
/// exact data-only [`ChannelFormat`]. The input is not accepted binding state
/// until [`BoundDescriptorSample::new`] checks it against a descriptor.
#[derive(Clone, Debug, PartialEq)]
pub enum DescriptorSampleInput {
    /// A homogeneous sample of `f32` values.
    Float32(Sample<f32>),
    /// A homogeneous sample of `f64` values.
    Double64(Sample<f64>),
    /// A homogeneous sample of owned string values.
    String(Sample<String>),
    /// A homogeneous sample of `i32` values.
    Int32(Sample<i32>),
    /// A homogeneous sample of `i16` values.
    Int16(Sample<i16>),
    /// A homogeneous sample of `i8` values.
    Int8(Sample<i8>),
    /// A homogeneous sample of `i64` values.
    Int64(Sample<i64>),
}

impl DescriptorSampleInput {
    /// Returns the exact data-only format named by this input family.
    #[must_use]
    pub const fn channel_format(&self) -> ChannelFormat {
        match self {
            Self::Float32(_) => ChannelFormat::Float32,
            Self::Double64(_) => ChannelFormat::Double64,
            Self::String(_) => ChannelFormat::String,
            Self::Int32(_) => ChannelFormat::Int32,
            Self::Int16(_) => ChannelFormat::Int16,
            Self::Int8(_) => ChannelFormat::Int8,
            Self::Int64(_) => ChannelFormat::Int64,
        }
    }

    /// Returns the validated declared channel count of the contained sample.
    #[must_use]
    pub const fn declared_channels(&self) -> usize {
        match self {
            Self::Float32(sample) => sample.declared_channels(),
            Self::Double64(sample) => sample.declared_channels(),
            Self::String(sample) => sample.declared_channels(),
            Self::Int32(sample) => sample.declared_channels(),
            Self::Int16(sample) => sample.declared_channels(),
            Self::Int8(sample) => sample.declared_channels(),
            Self::Int64(sample) => sample.declared_channels(),
        }
    }
}

/// One sample accepted against an exact validated descriptor shape.
///
/// The binding retains a compact snapshot of the descriptor's channel count
/// and data-only format. It does not clone or retain the full descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundDescriptorSample {
    limits: DescriptorSampleLimits,
    channel_count: usize,
    channel_format: ChannelFormat,
    sample: DescriptorSampleInput,
}

impl BoundDescriptorSample {
    /// Binds one homogeneous sample to a validated descriptor shape.
    ///
    /// Construction checks format, then channel count, then string values in
    /// zero-based channel order. Accepted values are moved unchanged; this
    /// function performs no conversion, inference, or runtime action.
    pub fn new(
        limits: DescriptorSampleLimits,
        descriptor: &StreamDescriptor,
        sample: DescriptorSampleInput,
    ) -> Result<Self, DescriptorSampleError> {
        let expected_format = descriptor.channel_format();
        let actual_format = sample.channel_format();
        if actual_format != expected_format {
            return Err(DescriptorSampleError::ChannelFormatMismatch {
                expected: expected_format,
                actual: actual_format,
            });
        }

        let expected_channels = descriptor.channel_count();
        let actual_channels = sample.declared_channels();
        if actual_channels != expected_channels {
            return Err(DescriptorSampleError::ChannelCountMismatch {
                expected: expected_channels,
                actual: actual_channels,
            });
        }

        if let DescriptorSampleInput::String(string_sample) = &sample {
            for (channel_index, value) in string_sample.values().iter().enumerate() {
                let actual = value.chars().count();
                if actual > limits.max_string_value_code_points {
                    return Err(DescriptorSampleError::StringValueLimitExceeded {
                        channel_index,
                        expected_max: limits.max_string_value_code_points,
                        actual,
                    });
                }
            }
        }

        Ok(Self {
            limits,
            channel_count: expected_channels,
            channel_format: expected_format,
            sample,
        })
    }

    /// Returns the limits under which this binding was accepted.
    #[must_use]
    pub const fn limits(&self) -> DescriptorSampleLimits {
        self.limits
    }

    /// Returns the descriptor channel count retained by the accepted binding.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.channel_count
    }

    /// Returns the descriptor data-only format retained by the accepted binding.
    #[must_use]
    pub const fn channel_format(&self) -> ChannelFormat {
        self.channel_format
    }

    /// Returns the unchanged homogeneous sample input.
    #[must_use]
    pub const fn sample(&self) -> &DescriptorSampleInput {
        &self.sample
    }

    /// Returns the unchanged owned homogeneous sample input.
    #[must_use]
    pub fn into_sample(self) -> DescriptorSampleInput {
        self.sample
    }
}

/// Deterministic rejection from binding limits or descriptor-shape checks.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DescriptorSampleError {
    /// A configured maximum cannot accept any string channel value.
    InvalidLimit {
        /// The malformed bound.
        bound: DescriptorSampleBound,
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// The input family's exact data-only format differed from the descriptor.
    ChannelFormatMismatch {
        /// The descriptor's required format.
        expected: ChannelFormat,
        /// The input family's actual format.
        actual: ChannelFormat,
    },
    /// The validated sample shape differed from the descriptor channel count.
    ChannelCountMismatch {
        /// The descriptor's required channel count.
        expected: usize,
        /// The sample's validated declared channel count.
        actual: usize,
    },
    /// One string channel exceeded its Unicode scalar-value maximum.
    StringValueLimitExceeded {
        /// The zero-based channel location of the first oversized value.
        channel_index: usize,
        /// The configured Unicode scalar-value maximum.
        expected_max: usize,
        /// The observed Unicode scalar-value count.
        actual: usize,
    },
}

impl fmt::Display for DescriptorSampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "descriptor/sample binding rejected input: {self:?}"
        )
    }
}

impl std::error::Error for DescriptorSampleError {}

#[cfg(test)]
mod tests {
    use super::{
        BoundDescriptorSample, DescriptorSampleBound, DescriptorSampleError, DescriptorSampleInput,
        DescriptorSampleLimits,
    };
    use crate::{
        ChannelFormat, NominalSampleRate, Sample, SampleLimits, StreamDescriptor,
        StreamDescriptorLimits,
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

    fn sample<T>(values: Vec<T>) -> Sample<T> {
        let channels = values.len();
        Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap()
    }

    fn bind(
        descriptor: &StreamDescriptor,
        input: DescriptorSampleInput,
    ) -> Result<BoundDescriptorSample, DescriptorSampleError> {
        BoundDescriptorSample::new(DescriptorSampleLimits::new(8).unwrap(), descriptor, input)
    }

    #[test]
    fn core_005_all_seven_exact_format_mappings_bind() {
        let cases = [
            (
                ChannelFormat::Float32,
                DescriptorSampleInput::Float32(sample(vec![1.0_f32])),
            ),
            (
                ChannelFormat::Double64,
                DescriptorSampleInput::Double64(sample(vec![1.0_f64])),
            ),
            (
                ChannelFormat::String,
                DescriptorSampleInput::String(sample(vec!["x".to_owned()])),
            ),
            (
                ChannelFormat::Int32,
                DescriptorSampleInput::Int32(sample(vec![1_i32])),
            ),
            (
                ChannelFormat::Int16,
                DescriptorSampleInput::Int16(sample(vec![1_i16])),
            ),
            (
                ChannelFormat::Int8,
                DescriptorSampleInput::Int8(sample(vec![1_i8])),
            ),
            (
                ChannelFormat::Int64,
                DescriptorSampleInput::Int64(sample(vec![1_i64])),
            ),
        ];

        for (format, input) in cases {
            let accepted = bind(&descriptor(1, format), input).unwrap();
            assert_eq!(accepted.channel_count(), 1);
            assert_eq!(accepted.channel_format(), format);
            assert_eq!(accepted.sample().channel_format(), format);
        }
    }

    #[test]
    fn core_005_each_input_family_has_stable_format_mismatch() {
        let cases = [
            (
                ChannelFormat::Double64,
                ChannelFormat::Float32,
                DescriptorSampleInput::Float32(sample(vec![1.0_f32])),
            ),
            (
                ChannelFormat::String,
                ChannelFormat::Double64,
                DescriptorSampleInput::Double64(sample(vec![1.0_f64])),
            ),
            (
                ChannelFormat::Int32,
                ChannelFormat::String,
                DescriptorSampleInput::String(sample(vec!["x".to_owned()])),
            ),
            (
                ChannelFormat::Int16,
                ChannelFormat::Int32,
                DescriptorSampleInput::Int32(sample(vec![1_i32])),
            ),
            (
                ChannelFormat::Int8,
                ChannelFormat::Int16,
                DescriptorSampleInput::Int16(sample(vec![1_i16])),
            ),
            (
                ChannelFormat::Int64,
                ChannelFormat::Int8,
                DescriptorSampleInput::Int8(sample(vec![1_i8])),
            ),
            (
                ChannelFormat::Float32,
                ChannelFormat::Int64,
                DescriptorSampleInput::Int64(sample(vec![1_i64])),
            ),
        ];

        for (expected, actual, input) in cases {
            assert_eq!(
                bind(&descriptor(1, expected), input),
                Err(DescriptorSampleError::ChannelFormatMismatch { expected, actual })
            );
        }
    }

    #[test]
    fn core_005_descriptor_sample_channel_mismatch_has_stable_error() {
        assert_eq!(
            bind(
                &descriptor(2, ChannelFormat::Int32),
                DescriptorSampleInput::Int32(sample(vec![10_i32])),
            ),
            Err(DescriptorSampleError::ChannelCountMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn core_005_validation_order_is_format_then_channel_then_string_bound() {
        let wrong_family = DescriptorSampleInput::String(sample(vec!["too".to_owned()]));
        assert_eq!(
            BoundDescriptorSample::new(
                DescriptorSampleLimits::new(1).unwrap(),
                &descriptor(2, ChannelFormat::Int32),
                wrong_family,
            ),
            Err(DescriptorSampleError::ChannelFormatMismatch {
                expected: ChannelFormat::Int32,
                actual: ChannelFormat::String,
            })
        );

        let wrong_shape = DescriptorSampleInput::String(sample(vec!["too".to_owned()]));
        assert_eq!(
            BoundDescriptorSample::new(
                DescriptorSampleLimits::new(1).unwrap(),
                &descriptor(2, ChannelFormat::String),
                wrong_shape,
            ),
            Err(DescriptorSampleError::ChannelCountMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn core_005_zero_string_limit_has_stable_error() {
        assert_eq!(
            DescriptorSampleLimits::new(0),
            Err(DescriptorSampleError::InvalidLimit {
                bound: DescriptorSampleBound::StringValueCodePoints,
                expected_min: 1,
                actual: 0,
            })
        );
    }

    #[test]
    fn core_005_string_exact_unicode_scalar_limit_and_empty_value_preserved() {
        let limits = DescriptorSampleLimits::new(3).unwrap();
        let input = DescriptorSampleInput::String(sample(vec![
            "".to_owned(),
            "é中🦀".to_owned(),
            " x ".to_owned(),
        ]));
        let accepted =
            BoundDescriptorSample::new(limits, &descriptor(3, ChannelFormat::String), input)
                .unwrap();

        assert_eq!(accepted.limits(), limits);
        match accepted.into_sample() {
            DescriptorSampleInput::String(sample) => {
                assert_eq!(sample.into_values(), ["", "é中🦀", " x "]);
            }
            _ => panic!("string sample changed family"),
        }
    }

    #[test]
    fn core_005_first_one_past_string_channel_has_stable_indexed_error() {
        let input = DescriptorSampleInput::String(sample(vec![
            "ok".to_owned(),
            "é中🦀".to_owned(),
            "also too long".to_owned(),
        ]));

        assert_eq!(
            BoundDescriptorSample::new(
                DescriptorSampleLimits::new(2).unwrap(),
                &descriptor(3, ChannelFormat::String),
                input,
            ),
            Err(DescriptorSampleError::StringValueLimitExceeded {
                channel_index: 1,
                expected_max: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn core_005_float_bits_and_channel_order_are_preserved() {
        let f32_bits = [0x8000_0000, 0x7fc1_2345, 0x3f80_0001];
        let f32_input = DescriptorSampleInput::Float32(sample(
            f32_bits.into_iter().map(f32::from_bits).collect(),
        ));
        let f32_accepted = bind(&descriptor(3, ChannelFormat::Float32), f32_input).unwrap();
        match f32_accepted.into_sample() {
            DescriptorSampleInput::Float32(sample) => assert_eq!(
                sample
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                f32_bits
            ),
            _ => panic!("f32 sample changed family"),
        }

        let f64_bits = [
            0x8000_0000_0000_0000,
            0x7ff8_0000_0001_2345,
            0x3ff0_0000_0000_0001,
        ];
        let f64_input = DescriptorSampleInput::Double64(sample(
            f64_bits.into_iter().map(f64::from_bits).collect(),
        ));
        let f64_accepted = bind(&descriptor(3, ChannelFormat::Double64), f64_input).unwrap();
        match f64_accepted.into_sample() {
            DescriptorSampleInput::Double64(sample) => assert_eq!(
                sample
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                f64_bits
            ),
            _ => panic!("f64 sample changed family"),
        }
    }

    #[test]
    fn core_005_integer_edges_and_channel_order_are_preserved() {
        let cases = [
            DescriptorSampleInput::Int32(sample(vec![i32::MIN, 0, i32::MAX])),
            DescriptorSampleInput::Int16(sample(vec![i16::MIN, 0, i16::MAX])),
            DescriptorSampleInput::Int8(sample(vec![i8::MIN, 0, i8::MAX])),
            DescriptorSampleInput::Int64(sample(vec![i64::MIN, 0, i64::MAX])),
        ];

        for input in cases {
            let format = input.channel_format();
            let accepted = bind(&descriptor(3, format), input).unwrap();
            match accepted.into_sample() {
                DescriptorSampleInput::Int32(sample) => {
                    assert_eq!(sample.into_values(), [i32::MIN, 0, i32::MAX]);
                }
                DescriptorSampleInput::Int16(sample) => {
                    assert_eq!(sample.into_values(), [i16::MIN, 0, i16::MAX]);
                }
                DescriptorSampleInput::Int8(sample) => {
                    assert_eq!(sample.into_values(), [i8::MIN, 0, i8::MAX]);
                }
                DescriptorSampleInput::Int64(sample) => {
                    assert_eq!(sample.into_values(), [i64::MIN, 0, i64::MAX]);
                }
                _ => panic!("integer sample changed family"),
            }
        }
    }
}
