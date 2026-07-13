// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// Identifies one configured stream-descriptor bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamDescriptorBound {
    /// Maximum Unicode scalar-value count in the required stream name.
    NameText,
    /// Maximum Unicode scalar-value count in the optional content type.
    ContentTypeText,
    /// Maximum Unicode scalar-value count in the optional source identifier.
    SourceIdText,
    /// Maximum channel count in the descriptor.
    Channels,
}

/// Identifies one bounded stream-descriptor text member.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamDescriptorTextRole {
    /// The required stream name.
    Name,
    /// The optional opaque content type.
    ContentType,
    /// The optional opaque source correlation identifier.
    SourceId,
}

/// Explicit maxima applied atomically when constructing a stream descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamDescriptorLimits {
    max_name_code_points: usize,
    max_content_type_code_points: usize,
    max_source_id_code_points: usize,
    max_channels: usize,
}

impl StreamDescriptorLimits {
    /// Validates all maxima in name, content-type, source-id, then channel order.
    pub fn new(
        max_name_code_points: usize,
        max_content_type_code_points: usize,
        max_source_id_code_points: usize,
        max_channels: usize,
    ) -> Result<Self, StreamDescriptorError> {
        for (bound, actual) in [
            (StreamDescriptorBound::NameText, max_name_code_points),
            (
                StreamDescriptorBound::ContentTypeText,
                max_content_type_code_points,
            ),
            (
                StreamDescriptorBound::SourceIdText,
                max_source_id_code_points,
            ),
            (StreamDescriptorBound::Channels, max_channels),
        ] {
            if actual == 0 {
                return Err(StreamDescriptorError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual,
                });
            }
        }

        Ok(Self {
            max_name_code_points,
            max_content_type_code_points,
            max_source_id_code_points,
            max_channels,
        })
    }

    /// Returns the maximum accepted stream-name Unicode scalar count.
    #[must_use]
    pub const fn max_name_code_points(self) -> usize {
        self.max_name_code_points
    }

    /// Returns the maximum accepted content-type Unicode scalar count.
    #[must_use]
    pub const fn max_content_type_code_points(self) -> usize {
        self.max_content_type_code_points
    }

    /// Returns the maximum accepted source-id Unicode scalar count.
    #[must_use]
    pub const fn max_source_id_code_points(self) -> usize {
        self.max_source_id_code_points
    }

    /// Returns the maximum accepted channel count.
    #[must_use]
    pub const fn max_channels(self) -> usize {
        self.max_channels
    }
}

/// Classifies a rejected regular nominal sample rate without retaining NaN.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvalidRegularSampleRate {
    /// A zero value, including negative zero.
    Zero,
    /// A finite negative value.
    Negative,
    /// A not-a-number value.
    NaN,
    /// Positive infinity.
    PositiveInfinity,
    /// Negative infinity.
    NegativeInfinity,
}

/// Deterministic rejection of a non-positive or non-finite regular rate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum NominalSampleRateError {
    /// The caller supplied a value outside the finite positive regular-Hz domain.
    InvalidRegularHz {
        /// The stable classification of the rejected value.
        actual: InvalidRegularSampleRate,
    },
}

impl fmt::Display for NominalSampleRateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "nominal sample rate rejected input: {self:?}")
    }
}

impl std::error::Error for NominalSampleRateError {}

/// A validated finite positive caller-provided regular rate in hertz.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularSampleRate(f64);

impl RegularSampleRate {
    /// Validates a finite positive hertz value without changing its bits.
    pub fn new(hz: f64) -> Result<Self, NominalSampleRateError> {
        let actual = if hz.is_nan() {
            Some(InvalidRegularSampleRate::NaN)
        } else if hz == 0.0 {
            Some(InvalidRegularSampleRate::Zero)
        } else if hz == f64::INFINITY {
            Some(InvalidRegularSampleRate::PositiveInfinity)
        } else if hz == f64::NEG_INFINITY {
            Some(InvalidRegularSampleRate::NegativeInfinity)
        } else if hz < 0.0 {
            Some(InvalidRegularSampleRate::Negative)
        } else {
            None
        };

        match actual {
            Some(actual) => Err(NominalSampleRateError::InvalidRegularHz { actual }),
            None => Ok(Self(hz)),
        }
    }

    /// Returns the unchanged caller-provided hertz value.
    #[must_use]
    pub const fn hz(self) -> f64 {
        self.0
    }
}

/// A caller-declared nominal sample-rate form.
///
/// This value does not read clocks, measure, schedule, enforce, interpolate,
/// or derive sample rates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NominalSampleRate {
    /// Samples do not have a declared regular nominal cadence.
    Irregular,
    /// A separately validated finite positive regular rate in hertz.
    RegularHz(RegularSampleRate),
}

impl NominalSampleRate {
    /// Constructs the explicit irregular form.
    #[must_use]
    pub const fn irregular() -> Self {
        Self::Irregular
    }

    /// Validates and constructs the regular-hertz form.
    pub fn regular_hz(hz: f64) -> Result<Self, NominalSampleRateError> {
        RegularSampleRate::new(hz).map(Self::RegularHz)
    }
}

/// The caller-declared data format for every channel in one stream.
///
/// These variants are data-only names. They have no protocol numeric
/// discriminants and do not encode, decode, size, or convert sample values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelFormat {
    /// 32-bit floating-point channel values.
    Float32,
    /// 64-bit floating-point channel values.
    Double64,
    /// String channel values.
    String,
    /// 32-bit signed integer channel values.
    Int32,
    /// 16-bit signed integer channel values.
    Int16,
    /// 8-bit signed integer channel values.
    Int8,
    /// 64-bit signed integer channel values.
    Int64,
}

/// A bounded caller-provided core stream descriptor.
///
/// The optional source identifier is opaque correlation data only. It is not
/// global identity, discovery, recovery, authorization, routing, permission,
/// admission, or accepted Morphospace state.
#[derive(Clone, Debug, PartialEq)]
pub struct StreamDescriptor {
    limits: StreamDescriptorLimits,
    name: String,
    content_type: Option<String>,
    source_id: Option<String>,
    channel_count: usize,
    nominal_sample_rate: NominalSampleRate,
    channel_format: ChannelFormat,
}

impl StreamDescriptor {
    /// Validates all text and channel bounds before returning a descriptor.
    ///
    /// Text is counted in Unicode scalar values and retained exactly. The
    /// constructor does not trim, normalize, case-fold, infer, or reorder data.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        limits: StreamDescriptorLimits,
        name: String,
        content_type: Option<String>,
        source_id: Option<String>,
        channel_count: usize,
        nominal_sample_rate: NominalSampleRate,
        channel_format: ChannelFormat,
    ) -> Result<Self, StreamDescriptorError> {
        if name.is_empty() {
            return Err(StreamDescriptorError::EmptyName);
        }

        for (role, text, expected_max) in [
            (
                StreamDescriptorTextRole::Name,
                Some(name.as_str()),
                limits.max_name_code_points,
            ),
            (
                StreamDescriptorTextRole::ContentType,
                content_type.as_deref(),
                limits.max_content_type_code_points,
            ),
            (
                StreamDescriptorTextRole::SourceId,
                source_id.as_deref(),
                limits.max_source_id_code_points,
            ),
        ] {
            if let Some(text) = text {
                let actual = text.chars().count();
                if actual > expected_max {
                    return Err(StreamDescriptorError::TextLimitExceeded {
                        role,
                        expected_max,
                        actual,
                    });
                }
            }
        }

        if channel_count == 0 || channel_count > limits.max_channels {
            return Err(StreamDescriptorError::ChannelCountOutOfBounds {
                expected_min: 1,
                expected_max: limits.max_channels,
                actual: channel_count,
            });
        }

        Ok(Self {
            limits,
            name,
            content_type,
            source_id,
            channel_count,
            nominal_sample_rate,
            channel_format,
        })
    }

    /// Returns the limits under which this descriptor was accepted.
    #[must_use]
    pub const fn limits(&self) -> StreamDescriptorLimits {
        self.limits
    }

    /// Returns the unchanged nonempty stream name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the unchanged optional opaque content type.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Returns the unchanged optional opaque source correlation identifier.
    #[must_use]
    pub fn source_id(&self) -> Option<&str> {
        self.source_id.as_deref()
    }

    /// Returns the validated channel count.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.channel_count
    }

    /// Returns the caller-declared nominal sample-rate form.
    #[must_use]
    pub const fn nominal_sample_rate(&self) -> NominalSampleRate {
        self.nominal_sample_rate
    }

    /// Returns the caller-declared channel data format.
    #[must_use]
    pub const fn channel_format(&self) -> ChannelFormat {
        self.channel_format
    }

    /// Returns all unchanged owned descriptor members.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        String,
        Option<String>,
        Option<String>,
        usize,
        NominalSampleRate,
        ChannelFormat,
    ) {
        (
            self.name,
            self.content_type,
            self.source_id,
            self.channel_count,
            self.nominal_sample_rate,
            self.channel_format,
        )
    }
}

/// Deterministic rejection from stream-descriptor limits or construction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamDescriptorError {
    /// A maximum cannot accept any value for the named bound.
    InvalidLimit {
        /// The malformed bound.
        bound: StreamDescriptorBound,
        /// The smallest accepted configuration value.
        expected_min: usize,
        /// The caller-provided configuration value.
        actual: usize,
    },
    /// The required stream name was empty.
    EmptyName,
    /// One text member exceeded its explicit Unicode scalar-value maximum.
    TextLimitExceeded {
        /// The rejected text member.
        role: StreamDescriptorTextRole,
        /// The configured Unicode scalar-value maximum.
        expected_max: usize,
        /// The observed Unicode scalar-value count.
        actual: usize,
    },
    /// The channel count was zero or exceeded its explicit maximum.
    ChannelCountOutOfBounds {
        /// The smallest accepted channel count.
        expected_min: usize,
        /// The configured maximum channel count.
        expected_max: usize,
        /// The caller-provided channel count.
        actual: usize,
    },
}

impl fmt::Display for StreamDescriptorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "stream descriptor rejected input: {self:?}")
    }
}

impl std::error::Error for StreamDescriptorError {}

#[cfg(test)]
mod tests {
    use super::{
        ChannelFormat, InvalidRegularSampleRate, NominalSampleRate, NominalSampleRateError,
        StreamDescriptor, StreamDescriptorBound, StreamDescriptorError, StreamDescriptorLimits,
        StreamDescriptorTextRole,
    };

    fn descriptor(
        limits: StreamDescriptorLimits,
        name: &str,
        content_type: Option<&str>,
        source_id: Option<&str>,
        channel_count: usize,
    ) -> Result<StreamDescriptor, StreamDescriptorError> {
        StreamDescriptor::new(
            limits,
            name.to_owned(),
            content_type.map(str::to_owned),
            source_id.map(str::to_owned),
            channel_count,
            NominalSampleRate::Irregular,
            ChannelFormat::Float32,
        )
    }

    #[test]
    fn core_003_descriptor_exact_limits_preserve_all_text_and_values() {
        let limits = StreamDescriptorLimits::new(4, 4, 4, 3).unwrap();
        let rate_bits = 0x4009_21fb_5444_2d18;
        let expected_rate = NominalSampleRate::regular_hz(f64::from_bits(rate_bits)).unwrap();
        let accepted = StreamDescriptor::new(
            limits,
            " Aß中".to_owned(),
            Some(" Eeg".to_owned()),
            Some("ß-中 ".to_owned()),
            3,
            expected_rate,
            ChannelFormat::Double64,
        )
        .unwrap();

        assert_eq!(accepted.limits(), limits);
        assert_eq!(accepted.name(), " Aß中");
        assert_eq!(accepted.content_type(), Some(" Eeg"));
        assert_eq!(accepted.source_id(), Some("ß-中 "));
        assert_eq!(accepted.channel_count(), 3);
        assert_eq!(accepted.channel_format(), ChannelFormat::Double64);
        match accepted.nominal_sample_rate() {
            NominalSampleRate::RegularHz(rate) => assert_eq!(rate.hz().to_bits(), rate_bits),
            NominalSampleRate::Irregular => panic!("regular rate changed form"),
        }
        assert_eq!(
            accepted.into_parts(),
            (
                " Aß中".to_owned(),
                Some(" Eeg".to_owned()),
                Some("ß-中 ".to_owned()),
                3,
                expected_rate,
                ChannelFormat::Double64,
            )
        );
    }

    #[test]
    fn core_003_optional_text_is_explicit_and_empty_opaque_text_is_preserved() {
        let limits = StreamDescriptorLimits::new(1, 1, 1, 1).unwrap();
        let absent = descriptor(limits, "n", None, None, 1).unwrap();
        let empty = descriptor(limits, "n", Some(""), Some(""), 1).unwrap();

        assert_eq!(absent.content_type(), None);
        assert_eq!(absent.source_id(), None);
        assert_eq!(empty.content_type(), Some(""));
        assert_eq!(empty.source_id(), Some(""));
    }

    #[test]
    fn core_003_empty_name_has_stable_typed_error() {
        assert_eq!(
            descriptor(
                StreamDescriptorLimits::new(1, 1, 1, 1).unwrap(),
                "",
                None,
                None,
                1,
            ),
            Err(StreamDescriptorError::EmptyName)
        );
    }

    #[test]
    fn core_003_one_past_each_text_limit_has_stable_typed_error() {
        let limits = StreamDescriptorLimits::new(1, 1, 1, 1).unwrap();
        for (role, name, content_type, source_id) in [
            (StreamDescriptorTextRole::Name, "ab", None, None),
            (StreamDescriptorTextRole::ContentType, "n", Some("ab"), None),
            (StreamDescriptorTextRole::SourceId, "n", None, Some("ab")),
        ] {
            assert_eq!(
                descriptor(limits, name, content_type, source_id, 1),
                Err(StreamDescriptorError::TextLimitExceeded {
                    role,
                    expected_max: 1,
                    actual: 2,
                })
            );
        }
    }

    #[test]
    fn core_003_zero_limits_reject_in_argument_order() {
        for (arguments, bound) in [
            ((0, 0, 0, 0), StreamDescriptorBound::NameText),
            ((1, 0, 0, 0), StreamDescriptorBound::ContentTypeText),
            ((1, 1, 0, 0), StreamDescriptorBound::SourceIdText),
            ((1, 1, 1, 0), StreamDescriptorBound::Channels),
        ] {
            assert_eq!(
                StreamDescriptorLimits::new(arguments.0, arguments.1, arguments.2, arguments.3),
                Err(StreamDescriptorError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual: 0,
                })
            );
        }
    }

    #[test]
    fn core_003_zero_and_one_past_channels_have_stable_typed_errors() {
        let limits = StreamDescriptorLimits::new(1, 1, 1, 2).unwrap();
        for actual in [0, 3] {
            assert_eq!(
                descriptor(limits, "n", None, None, actual),
                Err(StreamDescriptorError::ChannelCountOutOfBounds {
                    expected_min: 1,
                    expected_max: 2,
                    actual,
                })
            );
        }
    }

    #[test]
    fn core_003_regular_rate_preserves_bits_and_irregular_is_explicit() {
        let bits = 0x3ff0_0000_0000_0001;
        match NominalSampleRate::regular_hz(f64::from_bits(bits)).unwrap() {
            NominalSampleRate::RegularHz(rate) => assert_eq!(rate.hz().to_bits(), bits),
            NominalSampleRate::Irregular => panic!("regular rate changed form"),
        }
        assert_eq!(NominalSampleRate::irregular(), NominalSampleRate::Irregular);
    }

    #[test]
    fn core_003_invalid_regular_rates_have_stable_typed_errors() {
        for (value, actual) in [
            (0.0, InvalidRegularSampleRate::Zero),
            (-0.0, InvalidRegularSampleRate::Zero),
            (-1.0, InvalidRegularSampleRate::Negative),
            (f64::NAN, InvalidRegularSampleRate::NaN),
            (f64::INFINITY, InvalidRegularSampleRate::PositiveInfinity),
            (
                f64::NEG_INFINITY,
                InvalidRegularSampleRate::NegativeInfinity,
            ),
        ] {
            assert_eq!(
                NominalSampleRate::regular_hz(value),
                Err(NominalSampleRateError::InvalidRegularHz { actual })
            );
        }
    }

    #[test]
    fn core_003_all_seven_channel_formats_are_independent_data_values() {
        let formats = [
            ChannelFormat::Float32,
            ChannelFormat::Double64,
            ChannelFormat::String,
            ChannelFormat::Int32,
            ChannelFormat::Int16,
            ChannelFormat::Int8,
            ChannelFormat::Int64,
        ];

        assert_eq!(formats.len(), 7);
        for (index, format) in formats.into_iter().enumerate() {
            assert!(!formats[..index].contains(&format));
        }
    }
}
