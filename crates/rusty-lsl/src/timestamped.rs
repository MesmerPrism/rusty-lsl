// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::Sample;

/// Identifies the caller-provided role of a timestamp value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TimestampRole {
    /// The raw timestamp supplied with the source sample.
    RawSource,
    /// An optional, separately labelled derived timestamp value.
    Derived,
}

/// Classifies a rejected non-finite timestamp without retaining a NaN payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum NonFiniteTimestamp {
    /// A not-a-number value.
    NaN,
    /// Positive infinity.
    PositiveInfinity,
    /// Negative infinity.
    NegativeInfinity,
}

/// Deterministic rejection of a non-finite timestamp value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TimestampError {
    /// The caller supplied a value outside the finite timestamp domain.
    NonFinite {
        /// The timestamp role that was being constructed.
        role: TimestampRole,
        /// The stable classification of the rejected value.
        actual: NonFiniteTimestamp,
    },
}

impl fmt::Display for TimestampError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "timestamp rejected input: {self:?}")
    }
}

impl std::error::Error for TimestampError {}

fn classify_non_finite(value: f64) -> NonFiniteTimestamp {
    if value.is_nan() {
        NonFiniteTimestamp::NaN
    } else if value.is_sign_positive() {
        NonFiniteTimestamp::PositiveInfinity
    } else {
        NonFiniteTimestamp::NegativeInfinity
    }
}

/// A finite raw source timestamp retained exactly as supplied by the caller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawSourceTimestamp(f64);

impl RawSourceTimestamp {
    /// Validates finiteness without normalizing or otherwise changing the value.
    pub fn new(value: f64) -> Result<Self, TimestampError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(TimestampError::NonFinite {
                role: TimestampRole::RawSource,
                actual: classify_non_finite(value),
            })
        }
    }

    /// Returns the unchanged caller-provided floating-point value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Classifies a caller-provided derived timestamp without calculating it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DerivedTimestampKind {
    /// A value labelled by the caller as clock-corrected.
    ClockCorrected,
    /// A value labelled by the caller as smoothed.
    Smoothed,
}

/// A finite, explicitly classified derived timestamp kept distinct from raw time.
///
/// This type stores the caller's classification but does not calculate,
/// correct, smooth, interpolate, or otherwise interpret the value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DerivedTimestamp {
    kind: DerivedTimestampKind,
    value: f64,
}

impl DerivedTimestamp {
    /// Stores the explicit kind and validates finiteness without changing value.
    pub fn new(kind: DerivedTimestampKind, value: f64) -> Result<Self, TimestampError> {
        if value.is_finite() {
            Ok(Self { kind, value })
        } else {
            Err(TimestampError::NonFinite {
                role: TimestampRole::Derived,
                actual: classify_non_finite(value),
            })
        }
    }

    /// Returns the unchanged caller-provided derived timestamp classification.
    #[must_use]
    pub const fn kind(self) -> DerivedTimestampKind {
        self.kind
    }

    /// Returns the unchanged caller-provided floating-point value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

/// A validated sample paired with its mandatory raw and optional derived time.
#[derive(Clone, Debug, PartialEq)]
pub struct TimestampedSample<T> {
    sample: Sample<T>,
    raw_source_timestamp: RawSourceTimestamp,
    derived_timestamp: Option<DerivedTimestamp>,
}

impl<T> TimestampedSample<T> {
    /// Constructs a sample-time pairing from separately validated values.
    ///
    /// The mandatory raw value remains independently addressable whether or not
    /// a derived value is present.
    #[must_use]
    pub const fn new(
        sample: Sample<T>,
        raw_source_timestamp: RawSourceTimestamp,
        derived_timestamp: Option<DerivedTimestamp>,
    ) -> Self {
        Self {
            sample,
            raw_source_timestamp,
            derived_timestamp,
        }
    }

    /// Returns the validated sample without changing its value order.
    #[must_use]
    pub const fn sample(&self) -> &Sample<T> {
        &self.sample
    }

    /// Returns the unchanged mandatory raw source timestamp.
    #[must_use]
    pub const fn raw_source_timestamp(&self) -> RawSourceTimestamp {
        self.raw_source_timestamp
    }

    /// Returns the separately labelled optional derived timestamp.
    #[must_use]
    pub const fn derived_timestamp(&self) -> Option<DerivedTimestamp> {
        self.derived_timestamp
    }

    /// Returns the unchanged sample and its still-distinct timestamp values.
    #[must_use]
    pub fn into_parts(self) -> (Sample<T>, RawSourceTimestamp, Option<DerivedTimestamp>) {
        (
            self.sample,
            self.raw_source_timestamp,
            self.derived_timestamp,
        )
    }
}

/// Identifies one configured timestamped-chunk bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ChunkBound {
    /// Maximum sample count in one chunk.
    Samples,
    /// Maximum channel count in each sample.
    Channels,
}

/// Explicit maxima applied atomically when constructing a timestamped chunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkLimits {
    max_samples: usize,
    max_channels: usize,
}

impl ChunkLimits {
    /// Validates both maxima in sample-then-channel argument order.
    pub fn new(max_samples: usize, max_channels: usize) -> Result<Self, ChunkError> {
        for (bound, actual) in [
            (ChunkBound::Samples, max_samples),
            (ChunkBound::Channels, max_channels),
        ] {
            if actual == 0 {
                return Err(ChunkError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual,
                });
            }
        }
        Ok(Self {
            max_samples,
            max_channels,
        })
    }

    /// Returns the maximum accepted sample count.
    #[must_use]
    pub const fn max_samples(self) -> usize {
        self.max_samples
    }

    /// Returns the maximum accepted channel count for each sample.
    #[must_use]
    pub const fn max_channels(self) -> usize {
        self.max_channels
    }
}

/// A caller-ordered timestamped sample collection accepted under explicit bounds.
#[derive(Clone, Debug, PartialEq)]
pub struct TimestampedChunk<T> {
    limits: ChunkLimits,
    samples: Vec<TimestampedSample<T>>,
}

impl<T> TimestampedChunk<T> {
    /// Validates sample count, per-sample channel maxima, and common shape.
    ///
    /// Validation completes before the ordered input is exposed as an accepted
    /// chunk. Values and raw/derived timestamp pairings are not reordered.
    pub fn new(
        limits: ChunkLimits,
        samples: Vec<TimestampedSample<T>>,
    ) -> Result<Self, ChunkError> {
        if samples.len() > limits.max_samples {
            return Err(ChunkError::SampleCountExceeded {
                expected_max: limits.max_samples,
                actual: samples.len(),
            });
        }

        let mut expected_channels = None;
        for (sample_index, sample) in samples.iter().enumerate() {
            let actual = sample.sample.declared_channels();
            if actual > limits.max_channels {
                return Err(ChunkError::ChannelCountExceeded {
                    sample_index,
                    expected_max: limits.max_channels,
                    actual,
                });
            }
            if let Some(expected) = expected_channels {
                if actual != expected {
                    return Err(ChunkError::InconsistentChannelShape {
                        sample_index,
                        expected,
                        actual,
                    });
                }
            } else {
                expected_channels = Some(actual);
            }
        }

        Ok(Self { limits, samples })
    }

    /// Returns the explicit limits under which the chunk was accepted.
    #[must_use]
    pub const fn limits(&self) -> ChunkLimits {
        self.limits
    }

    /// Returns the unchanged ordered sample/time pairings.
    #[must_use]
    pub fn samples(&self) -> &[TimestampedSample<T>] {
        &self.samples
    }

    /// Returns the unchanged owned sample/time pairings.
    #[must_use]
    pub fn into_samples(self) -> Vec<TimestampedSample<T>> {
        self.samples
    }
}

/// Deterministic rejection from chunk-limit configuration or construction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ChunkError {
    /// A maximum cannot accept any value for the named bound.
    InvalidLimit {
        /// The malformed bound.
        bound: ChunkBound,
        /// The smallest accepted configuration value.
        expected_min: usize,
        /// The caller-provided configuration value.
        actual: usize,
    },
    /// The chunk contained more samples than its explicit maximum.
    SampleCountExceeded {
        /// The configured maximum sample count.
        expected_max: usize,
        /// The caller-provided sample count.
        actual: usize,
    },
    /// One sample exceeded the chunk's explicit channel maximum.
    ChannelCountExceeded {
        /// The zero-based location of the rejected sample.
        sample_index: usize,
        /// The configured maximum channel count.
        expected_max: usize,
        /// The sample's validated channel count.
        actual: usize,
    },
    /// One sample shape differed from the first accepted shape in the chunk.
    InconsistentChannelShape {
        /// The zero-based location of the rejected sample.
        sample_index: usize,
        /// The first sample's validated channel count.
        expected: usize,
        /// The mismatching sample's validated channel count.
        actual: usize,
    },
}

impl fmt::Display for ChunkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "timestamped chunk rejected input: {self:?}")
    }
}

impl std::error::Error for ChunkError {}

#[cfg(test)]
mod tests {
    use super::{
        ChunkBound, ChunkError, ChunkLimits, DerivedTimestamp, DerivedTimestampKind,
        NonFiniteTimestamp, RawSourceTimestamp, TimestampError, TimestampRole, TimestampedChunk,
        TimestampedSample,
    };
    use crate::{Sample, SampleLimits};

    fn timestamped(
        values: Vec<u32>,
        raw: f64,
        derived: Option<(DerivedTimestampKind, f64)>,
    ) -> TimestampedSample<u32> {
        let sample = Sample::new(
            SampleLimits::new(values.len()).unwrap(),
            values.len(),
            values,
        )
        .unwrap();
        TimestampedSample::new(
            sample,
            RawSourceTimestamp::new(raw).unwrap(),
            derived.map(|(kind, value)| DerivedTimestamp::new(kind, value).unwrap()),
        )
    }

    #[test]
    fn core_002_timestamp_values_preserve_finite_bits() {
        let raw_bits = 0x8000_0000_0000_0000;
        let derived_bits = 0x3ff0_0000_0000_0001;
        let raw = RawSourceTimestamp::new(f64::from_bits(raw_bits)).unwrap();
        let derived = DerivedTimestamp::new(
            DerivedTimestampKind::ClockCorrected,
            f64::from_bits(derived_bits),
        )
        .unwrap();

        assert_eq!(raw.value().to_bits(), raw_bits);
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value().to_bits(), derived_bits);
    }

    #[test]
    fn core_002_non_finite_timestamps_have_stable_typed_errors() {
        for (value, actual) in [
            (f64::NAN, NonFiniteTimestamp::NaN),
            (f64::INFINITY, NonFiniteTimestamp::PositiveInfinity),
            (f64::NEG_INFINITY, NonFiniteTimestamp::NegativeInfinity),
        ] {
            assert_eq!(
                RawSourceTimestamp::new(value),
                Err(TimestampError::NonFinite {
                    role: TimestampRole::RawSource,
                    actual,
                })
            );
            for kind in [
                DerivedTimestampKind::ClockCorrected,
                DerivedTimestampKind::Smoothed,
            ] {
                assert_eq!(
                    DerivedTimestamp::new(kind, value),
                    Err(TimestampError::NonFinite {
                        role: TimestampRole::Derived,
                        actual,
                    })
                );
            }
        }
    }

    #[test]
    fn core_002_raw_and_optional_derived_timestamps_remain_distinct() {
        let sample = timestamped(
            vec![10, 20],
            -0.0,
            Some((DerivedTimestampKind::ClockCorrected, 8.25)),
        );

        assert_eq!(
            sample.raw_source_timestamp().value().to_bits(),
            (-0.0_f64).to_bits()
        );
        let derived = sample.derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value(), 8.25);
        assert_eq!(sample.sample().values(), [10, 20]);

        let without_derived = timestamped(vec![30, 40], 1.5, None);
        assert_eq!(without_derived.raw_source_timestamp().value(), 1.5);
        assert_eq!(without_derived.derived_timestamp(), None);
    }

    #[test]
    fn core_002_raw_remains_unchanged_beside_each_derived_kind() {
        let raw_bits = 0x4009_21fb_5444_2d18;
        for (kind, derived_value) in [
            (DerivedTimestampKind::ClockCorrected, 4.0),
            (DerivedTimestampKind::Smoothed, 5.0),
        ] {
            let sample = timestamped(
                vec![10, 20],
                f64::from_bits(raw_bits),
                Some((kind, derived_value)),
            );

            assert_eq!(sample.raw_source_timestamp().value().to_bits(), raw_bits);
            assert_eq!(sample.derived_timestamp().unwrap().kind(), kind);
            assert_eq!(sample.derived_timestamp().unwrap().value(), derived_value);
        }
    }

    #[test]
    fn core_002_chunk_exact_limits_preserve_order_values_and_timestamp_pairing() {
        let expected = vec![
            timestamped(
                vec![1, 2],
                -0.0,
                Some((DerivedTimestampKind::Smoothed, 10.0)),
            ),
            timestamped(vec![3, 4], 2.5, None),
        ];

        let chunk =
            TimestampedChunk::new(ChunkLimits::new(2, 2).unwrap(), expected.clone()).unwrap();

        assert_eq!(chunk.limits().max_samples(), 2);
        assert_eq!(chunk.limits().max_channels(), 2);
        assert_eq!(chunk.samples(), expected);
        assert_eq!(chunk.into_samples(), expected);
    }

    #[test]
    fn core_002_empty_chunk_accepts_and_retains_valid_nonzero_limits() {
        let limits = ChunkLimits::new(3, 4).unwrap();
        let chunk = TimestampedChunk::<u8>::new(limits, vec![]).unwrap();

        assert_eq!(chunk.limits(), limits);
        assert!(chunk.samples().is_empty());
        assert!(chunk.into_samples().is_empty());
    }

    #[test]
    fn core_002_chunk_one_past_sample_limit_has_stable_error() {
        let samples = vec![
            timestamped(vec![1], 1.0, None),
            timestamped(vec![2], 2.0, None),
        ];

        assert_eq!(
            TimestampedChunk::new(ChunkLimits::new(1, 1).unwrap(), samples),
            Err(ChunkError::SampleCountExceeded {
                expected_max: 1,
                actual: 2,
            })
        );
    }

    #[test]
    fn core_002_chunk_one_past_channel_limit_has_stable_error() {
        assert_eq!(
            TimestampedChunk::new(
                ChunkLimits::new(1, 2).unwrap(),
                vec![timestamped(vec![1, 2, 3], 1.0, None)],
            ),
            Err(ChunkError::ChannelCountExceeded {
                sample_index: 0,
                expected_max: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn core_002_chunk_inconsistent_channel_shape_has_stable_error() {
        assert_eq!(
            TimestampedChunk::new(
                ChunkLimits::new(2, 3).unwrap(),
                vec![
                    timestamped(vec![1, 2], 1.0, None),
                    timestamped(vec![3, 4, 5], 2.0, None),
                ],
            ),
            Err(ChunkError::InconsistentChannelShape {
                sample_index: 1,
                expected: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn core_002_chunk_zero_limits_reject_in_argument_order() {
        assert_eq!(
            ChunkLimits::new(0, 0),
            Err(ChunkError::InvalidLimit {
                bound: ChunkBound::Samples,
                expected_min: 1,
                actual: 0,
            })
        );
        assert_eq!(
            ChunkLimits::new(1, 0),
            Err(ChunkError::InvalidLimit {
                bound: ChunkBound::Channels,
                expected_min: 1,
                actual: 0,
            })
        );
    }
}
