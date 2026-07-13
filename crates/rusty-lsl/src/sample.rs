// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// Identifies one configured sample-shape bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SampleBound {
    /// Maximum channel count for one sample.
    Channels,
}

/// Limits applied atomically when constructing a [`Sample`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SampleLimits {
    max_channels: usize,
}

impl SampleLimits {
    /// Validates a sample limit configuration.
    pub fn new(max_channels: usize) -> Result<Self, SampleError> {
        if max_channels == 0 {
            return Err(SampleError::InvalidLimit {
                bound: SampleBound::Channels,
                expected_min: 1,
                actual: max_channels,
            });
        }
        Ok(Self { max_channels })
    }

    /// Returns the maximum accepted declared channel count.
    #[must_use]
    pub const fn max_channels(self) -> usize {
        self.max_channels
    }
}

/// One caller-owned sample whose value count matches its declared shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Sample<T> {
    declared_channels: usize,
    values: Vec<T>,
}

impl<T> Sample<T> {
    /// Validates the declared count and shape before returning a sample.
    ///
    /// Accepted values retain their original order and representation.
    pub fn new(
        limits: SampleLimits,
        declared_channels: usize,
        values: Vec<T>,
    ) -> Result<Self, SampleError> {
        if declared_channels == 0 || declared_channels > limits.max_channels {
            return Err(SampleError::ChannelCountOutOfBounds {
                expected_min: 1,
                expected_max: limits.max_channels,
                actual: declared_channels,
            });
        }
        if values.len() != declared_channels {
            return Err(SampleError::ChannelCountMismatch {
                expected: declared_channels,
                actual: values.len(),
            });
        }
        Ok(Self {
            declared_channels,
            values,
        })
    }

    /// Returns the validated declared channel count.
    #[must_use]
    pub const fn declared_channels(&self) -> usize {
        self.declared_channels
    }

    /// Returns the unchanged caller-provided sample values.
    #[must_use]
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// Returns the unchanged caller-provided sample values.
    #[must_use]
    pub fn into_values(self) -> Vec<T> {
        self.values
    }
}

/// Deterministic rejection from sample limit configuration or construction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SampleError {
    /// A limit configuration cannot accept any channel count.
    InvalidLimit {
        /// The malformed bound.
        bound: SampleBound,
        /// The smallest accepted configuration value.
        expected_min: usize,
        /// The caller-provided configuration value.
        actual: usize,
    },
    /// A declared channel count was zero or exceeded its configured maximum.
    ChannelCountOutOfBounds {
        /// The smallest accepted channel count.
        expected_min: usize,
        /// The configured maximum channel count.
        expected_max: usize,
        /// The caller-provided declared channel count.
        actual: usize,
    },
    /// The value count did not match the valid declared channel count.
    ChannelCountMismatch {
        /// The declared channel count.
        expected: usize,
        /// The caller-provided value count.
        actual: usize,
    },
}

impl fmt::Display for SampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "sample shape rejected input: {self:?}")
    }
}

impl std::error::Error for SampleError {}

#[cfg(test)]
mod tests {
    use super::{Sample, SampleBound, SampleError, SampleLimits};

    #[test]
    fn contract_sample_shape_exact_limit_and_values_unchanged() {
        let limits = SampleLimits::new(3).unwrap();
        let expected = vec![
            String::from(" left "),
            String::from("MIDDLE"),
            String::from("右"),
        ];

        let sample = Sample::new(limits, 3, expected.clone()).unwrap();

        assert_eq!(sample.declared_channels(), 3);
        assert_eq!(sample.values(), expected);
        assert_eq!(sample.into_values(), expected);
    }

    #[test]
    fn contract_sample_shape_one_past_limit_rejected() {
        let limits = SampleLimits::new(2).unwrap();
        assert_eq!(
            Sample::<u8>::new(limits, 3, vec![1, 2, 3]),
            Err(SampleError::ChannelCountOutOfBounds {
                expected_min: 1,
                expected_max: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn contract_sample_shape_zero_limit_rejected() {
        assert_eq!(
            SampleLimits::new(0),
            Err(SampleError::InvalidLimit {
                bound: SampleBound::Channels,
                expected_min: 1,
                actual: 0,
            })
        );
    }

    #[test]
    fn contract_sample_shape_zero_declared_channels_rejected() {
        assert_eq!(
            Sample::<u8>::new(SampleLimits::new(2).unwrap(), 0, vec![]),
            Err(SampleError::ChannelCountOutOfBounds {
                expected_min: 1,
                expected_max: 2,
                actual: 0,
            })
        );
    }

    #[test]
    fn contract_sample_shape_channel_mismatch_has_stable_payload() {
        assert_eq!(
            Sample::new(SampleLimits::new(8).unwrap(), 8, vec![0_u8; 7]),
            Err(SampleError::ChannelCountMismatch {
                expected: 8,
                actual: 7,
            })
        );
    }
}
