// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    BoundDescriptorSample, DerivedTimestamp, DescriptorSampleError, DescriptorSampleInput,
    DescriptorSampleLimits, RawSourceTimestamp, StreamDescriptor, TimestampedSample,
};

/// One unvalidated timestamped descriptor/sample composition input.
///
/// Each variant owns an already validated homogeneous [`TimestampedSample`]
/// and corresponds exactly to one existing data-only channel format. The
/// input is not accepted composition state until
/// [`BoundTimestampedDescriptorSample::new`] delegates sample validation to
/// [`BoundDescriptorSample::new`].
#[derive(Debug, PartialEq)]
pub enum TimestampedDescriptorSampleInput {
    /// A timestamped homogeneous sample of `f32` values.
    Float32(TimestampedSample<f32>),
    /// A timestamped homogeneous sample of `f64` values.
    Double64(TimestampedSample<f64>),
    /// A timestamped homogeneous sample of owned string values.
    String(TimestampedSample<String>),
    /// A timestamped homogeneous sample of `i32` values.
    Int32(TimestampedSample<i32>),
    /// A timestamped homogeneous sample of `i16` values.
    Int16(TimestampedSample<i16>),
    /// A timestamped homogeneous sample of `i8` values.
    Int8(TimestampedSample<i8>),
    /// A timestamped homogeneous sample of `i64` values.
    Int64(TimestampedSample<i64>),
}

impl TimestampedDescriptorSampleInput {
    fn into_binding_parts(
        self,
    ) -> (
        DescriptorSampleInput,
        RawSourceTimestamp,
        Option<DerivedTimestamp>,
    ) {
        match self {
            Self::Float32(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::Float32(sample), raw, derived)
            }
            Self::Double64(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::Double64(sample), raw, derived)
            }
            Self::String(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::String(sample), raw, derived)
            }
            Self::Int32(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::Int32(sample), raw, derived)
            }
            Self::Int16(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::Int16(sample), raw, derived)
            }
            Self::Int8(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::Int8(sample), raw, derived)
            }
            Self::Int64(timestamped) => {
                let (sample, raw, derived) = timestamped.into_parts();
                (DescriptorSampleInput::Int64(sample), raw, derived)
            }
        }
    }
}

/// One timestamped sample accepted against an exact descriptor shape.
///
/// Accepted state owns the unchanged CORE-005 binding and the timestamp
/// evidence moved from the same input sample. Its private fields prevent
/// callers from forging a different sample/timestamp pairing.
#[derive(Debug, PartialEq)]
pub struct BoundTimestampedDescriptorSample {
    bound_sample: BoundDescriptorSample,
    raw_source_timestamp: RawSourceTimestamp,
    derived_timestamp: Option<DerivedTimestamp>,
}

impl BoundTimestampedDescriptorSample {
    /// Composes one timestamped homogeneous sample with a validated descriptor.
    ///
    /// The timestamped input is moved apart once. Its sample is passed unchanged
    /// to [`BoundDescriptorSample::new`], which retains CORE-005 format, channel
    /// count, and String-value validation and error precedence. The raw and
    /// optional derived timestamp evidence is retained without recalculation or
    /// rewriting.
    pub fn new(
        limits: DescriptorSampleLimits,
        descriptor: &StreamDescriptor,
        input: TimestampedDescriptorSampleInput,
    ) -> Result<Self, DescriptorSampleError> {
        let (sample, raw_source_timestamp, derived_timestamp) = input.into_binding_parts();
        let bound_sample = BoundDescriptorSample::new(limits, descriptor, sample)?;

        Ok(Self {
            bound_sample,
            raw_source_timestamp,
            derived_timestamp,
        })
    }

    /// Returns the unchanged accepted descriptor/sample binding.
    #[must_use]
    pub const fn bound_sample(&self) -> &BoundDescriptorSample {
        &self.bound_sample
    }

    /// Returns the unchanged mandatory raw source timestamp.
    #[must_use]
    pub const fn raw_source_timestamp(&self) -> RawSourceTimestamp {
        self.raw_source_timestamp
    }

    /// Returns the unchanged optional derived timestamp evidence.
    #[must_use]
    pub const fn derived_timestamp(&self) -> Option<DerivedTimestamp> {
        self.derived_timestamp
    }

    /// Returns the unchanged owned binding and its still-distinct timestamps.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        BoundDescriptorSample,
        RawSourceTimestamp,
        Option<DerivedTimestamp>,
    ) {
        (
            self.bound_sample,
            self.raw_source_timestamp,
            self.derived_timestamp,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundTimestampedDescriptorSample, TimestampedDescriptorSampleInput};
    use crate::{
        ChannelFormat, DerivedTimestamp, DerivedTimestampKind, DescriptorSampleBound,
        DescriptorSampleError, DescriptorSampleInput, DescriptorSampleLimits, NominalSampleRate,
        RawSourceTimestamp, Sample, SampleLimits, StreamDescriptor, StreamDescriptorLimits,
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
        let sample = Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap();
        TimestampedSample::new(
            sample,
            RawSourceTimestamp::new(f64::from_bits(raw_bits)).unwrap(),
            derived.map(|(kind, bits)| DerivedTimestamp::new(kind, f64::from_bits(bits)).unwrap()),
        )
    }

    fn bind(
        descriptor: &StreamDescriptor,
        input: TimestampedDescriptorSampleInput,
    ) -> Result<BoundTimestampedDescriptorSample, DescriptorSampleError> {
        BoundTimestampedDescriptorSample::new(
            DescriptorSampleLimits::new(8).unwrap(),
            descriptor,
            input,
        )
    }

    #[test]
    fn core_006_all_seven_timestamped_format_mappings_bind_exactly() {
        let raw_bits = 0x3ff0_0000_0000_0001;
        let cases = [
            (
                ChannelFormat::Float32,
                TimestampedDescriptorSampleInput::Float32(timestamped(
                    vec![1.0_f32],
                    raw_bits,
                    None,
                )),
            ),
            (
                ChannelFormat::Double64,
                TimestampedDescriptorSampleInput::Double64(timestamped(
                    vec![1.0_f64],
                    raw_bits,
                    None,
                )),
            ),
            (
                ChannelFormat::String,
                TimestampedDescriptorSampleInput::String(timestamped(
                    vec!["x".to_owned()],
                    raw_bits,
                    None,
                )),
            ),
            (
                ChannelFormat::Int32,
                TimestampedDescriptorSampleInput::Int32(timestamped(vec![1_i32], raw_bits, None)),
            ),
            (
                ChannelFormat::Int16,
                TimestampedDescriptorSampleInput::Int16(timestamped(vec![1_i16], raw_bits, None)),
            ),
            (
                ChannelFormat::Int8,
                TimestampedDescriptorSampleInput::Int8(timestamped(vec![1_i8], raw_bits, None)),
            ),
            (
                ChannelFormat::Int64,
                TimestampedDescriptorSampleInput::Int64(timestamped(vec![1_i64], raw_bits, None)),
            ),
        ];

        for (format, input) in cases {
            let accepted = bind(&descriptor(1, format), input).unwrap();
            assert_eq!(accepted.bound_sample().channel_format(), format);
            assert_eq!(accepted.bound_sample().channel_count(), 1);
            assert_eq!(accepted.raw_source_timestamp().value().to_bits(), raw_bits);
            assert_eq!(accepted.derived_timestamp(), None);
        }
    }

    #[test]
    fn core_006_raw_only_and_both_derived_kinds_preserve_exact_bits() {
        let cases = [
            (0x8000_0000_0000_0000, None),
            (
                0x4009_21fb_5444_2d18,
                Some((DerivedTimestampKind::ClockCorrected, 0x8000_0000_0000_0000)),
            ),
            (
                0x3ff0_0000_0000_0001,
                Some((DerivedTimestampKind::Smoothed, 0x4000_0000_0000_0001)),
            ),
        ];

        for (raw_bits, derived) in cases {
            let accepted = bind(
                &descriptor(1, ChannelFormat::Int32),
                TimestampedDescriptorSampleInput::Int32(timestamped(
                    vec![7_i32],
                    raw_bits,
                    derived,
                )),
            )
            .unwrap();

            assert_eq!(accepted.raw_source_timestamp().value().to_bits(), raw_bits);
            match (accepted.derived_timestamp(), derived) {
                (None, None) => {}
                (Some(actual), Some((kind, bits))) => {
                    assert_eq!(actual.kind(), kind);
                    assert_eq!(actual.value().to_bits(), bits);
                }
                _ => panic!("derived timestamp presence changed"),
            }
        }
    }

    #[test]
    fn core_006_float_nan_payloads_and_timestamp_pairing_survive_consumption() {
        let raw_bits = 0x4009_21fb_5444_2d18;
        let derived_bits = 0x3ff0_0000_0000_0001;
        let f32_bits = [0x8000_0000, 0x7fc1_2345, 0x3f80_0001];
        let accepted = bind(
            &descriptor(3, ChannelFormat::Float32),
            TimestampedDescriptorSampleInput::Float32(timestamped(
                f32_bits.into_iter().map(f32::from_bits).collect(),
                raw_bits,
                Some((DerivedTimestampKind::Smoothed, derived_bits)),
            )),
        )
        .unwrap();

        let (bound, raw, derived) = accepted.into_parts();
        assert_eq!(raw.value().to_bits(), raw_bits);
        assert_eq!(derived.unwrap().value().to_bits(), derived_bits);
        match bound.into_sample() {
            DescriptorSampleInput::Float32(sample) => assert_eq!(
                sample
                    .into_values()
                    .into_iter()
                    .map(f32::to_bits)
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
        let accepted = bind(
            &descriptor(3, ChannelFormat::Double64),
            TimestampedDescriptorSampleInput::Double64(timestamped(
                f64_bits.into_iter().map(f64::from_bits).collect(),
                raw_bits,
                None,
            )),
        )
        .unwrap();
        match accepted.into_parts().0.into_sample() {
            DescriptorSampleInput::Double64(sample) => assert_eq!(
                sample
                    .into_values()
                    .into_iter()
                    .map(f64::to_bits)
                    .collect::<Vec<_>>(),
                f64_bits
            ),
            _ => panic!("f64 sample changed family"),
        }
    }

    #[test]
    fn core_006_integer_edges_and_order_are_preserved() {
        let inputs = [
            TimestampedDescriptorSampleInput::Int32(timestamped(
                vec![i32::MIN, 0, i32::MAX],
                1.0_f64.to_bits(),
                None,
            )),
            TimestampedDescriptorSampleInput::Int16(timestamped(
                vec![i16::MIN, 0, i16::MAX],
                2.0_f64.to_bits(),
                None,
            )),
            TimestampedDescriptorSampleInput::Int8(timestamped(
                vec![i8::MIN, 0, i8::MAX],
                3.0_f64.to_bits(),
                None,
            )),
            TimestampedDescriptorSampleInput::Int64(timestamped(
                vec![i64::MIN, 0, i64::MAX],
                4.0_f64.to_bits(),
                None,
            )),
        ];

        for input in inputs {
            let format = match &input {
                TimestampedDescriptorSampleInput::Int32(_) => ChannelFormat::Int32,
                TimestampedDescriptorSampleInput::Int16(_) => ChannelFormat::Int16,
                TimestampedDescriptorSampleInput::Int8(_) => ChannelFormat::Int8,
                TimestampedDescriptorSampleInput::Int64(_) => ChannelFormat::Int64,
                _ => unreachable!(),
            };
            match bind(&descriptor(3, format), input)
                .unwrap()
                .into_parts()
                .0
                .into_sample()
            {
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

    #[test]
    fn core_006_string_exact_bound_and_accessors_preserve_pairing() {
        let raw_bits = 0x3ff0_0000_0000_0001;
        let derived_bits = 0x8000_0000_0000_0000;
        let accepted = BoundTimestampedDescriptorSample::new(
            DescriptorSampleLimits::new(3).unwrap(),
            &descriptor(3, ChannelFormat::String),
            TimestampedDescriptorSampleInput::String(timestamped(
                vec!["".to_owned(), "é中🦀".to_owned(), " x ".to_owned()],
                raw_bits,
                Some((DerivedTimestampKind::ClockCorrected, derived_bits)),
            )),
        )
        .unwrap();

        assert_eq!(
            accepted
                .bound_sample()
                .limits()
                .max_string_value_code_points(),
            3
        );
        assert_eq!(accepted.raw_source_timestamp().value().to_bits(), raw_bits);
        let derived = accepted.derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value().to_bits(), derived_bits);
        match accepted.bound_sample().sample() {
            DescriptorSampleInput::String(sample) => {
                assert_eq!(sample.values(), ["", "é中🦀", " x "]);
            }
            _ => panic!("string sample changed family"),
        }
    }

    #[test]
    fn core_006_delegates_format_mismatch_without_rewriting_error() {
        assert_eq!(
            bind(
                &descriptor(1, ChannelFormat::Double64),
                TimestampedDescriptorSampleInput::Float32(timestamped(
                    vec![1.0_f32],
                    1.0_f64.to_bits(),
                    None,
                )),
            ),
            Err(DescriptorSampleError::ChannelFormatMismatch {
                expected: ChannelFormat::Double64,
                actual: ChannelFormat::Float32,
            })
        );
    }

    #[test]
    fn core_006_delegates_channel_mismatch_without_rewriting_error() {
        assert_eq!(
            bind(
                &descriptor(2, ChannelFormat::Int32),
                TimestampedDescriptorSampleInput::Int32(timestamped(
                    vec![1_i32],
                    1.0_f64.to_bits(),
                    None,
                )),
            ),
            Err(DescriptorSampleError::ChannelCountMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn core_006_delegates_first_one_past_string_error_exactly() {
        let error = BoundTimestampedDescriptorSample::new(
            DescriptorSampleLimits::new(2).unwrap(),
            &descriptor(3, ChannelFormat::String),
            TimestampedDescriptorSampleInput::String(timestamped(
                vec![
                    "ok".to_owned(),
                    "é中🦀".to_owned(),
                    "also too long".to_owned(),
                ],
                1.0_f64.to_bits(),
                None,
            )),
        );

        assert_eq!(
            error,
            Err(DescriptorSampleError::StringValueLimitExceeded {
                channel_index: 1,
                expected_max: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn core_006_delegated_validation_precedence_is_unchanged() {
        let oversized = || {
            TimestampedDescriptorSampleInput::String(timestamped(
                vec!["too".to_owned()],
                1.0_f64.to_bits(),
                None,
            ))
        };

        assert_eq!(
            BoundTimestampedDescriptorSample::new(
                DescriptorSampleLimits::new(1).unwrap(),
                &descriptor(2, ChannelFormat::Int32),
                oversized(),
            ),
            Err(DescriptorSampleError::ChannelFormatMismatch {
                expected: ChannelFormat::Int32,
                actual: ChannelFormat::String,
            })
        );
        assert_eq!(
            BoundTimestampedDescriptorSample::new(
                DescriptorSampleLimits::new(1).unwrap(),
                &descriptor(2, ChannelFormat::String),
                oversized(),
            ),
            Err(DescriptorSampleError::ChannelCountMismatch {
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn core_006_zero_string_limit_retains_delegated_typed_error() {
        assert_eq!(
            DescriptorSampleLimits::new(0),
            Err(DescriptorSampleError::InvalidLimit {
                bound: DescriptorSampleBound::StringValueCodePoints,
                expected_min: 1,
                actual: 0,
            })
        );
    }
}
