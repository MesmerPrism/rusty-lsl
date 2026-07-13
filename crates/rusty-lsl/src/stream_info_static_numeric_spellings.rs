// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{NominalSampleRate, StreamInfoStaticFields};
use core::fmt;

const MAX_CHANNEL_COUNT_BYTES: usize = 20;
const NUMERIC_SPELLING_BYTES: usize = 17;
const IRREGULAR_SPELLING: &str = "0.000000000000000";
const DECIMAL_DIGITS: &[u8; 10] = b"0123456789";

/// A bounded owned lexical view of two numeric static stream-info fields.
///
/// The view borrows an accepted [`StreamInfoStaticFields`] value and exposes
/// only its channel-count and nominal-rate text. It does not mutate or consume
/// the source and assigns no XML, document, locale, protocol, or runtime
/// meaning to either string.
pub struct StreamInfoStaticNumericSpellings<'fields, 'definition> {
    _fields: &'fields StreamInfoStaticFields<'definition>,
    channel_count: String,
    nominal_srate: String,
}

impl<'fields, 'definition> StreamInfoStaticNumericSpellings<'fields, 'definition> {
    /// Projects the two numeric spellings under the exact LSLC-001L policy.
    ///
    /// Irregular rates use the observed positive-zero spelling. Regular rates
    /// are accepted only when their `f64` bits equal one of the five values
    /// observed by LSLC-001H. Each exact output length is known before one
    /// fallible exact reserve for that output string.
    ///
    /// # Errors
    ///
    /// Returns a typed error before allocation when a regular rate is outside
    /// the accepted five-value domain, or when either exact reserve fails.
    pub fn new(
        fields: &'fields StreamInfoStaticFields<'definition>,
    ) -> Result<Self, StreamInfoStaticNumericSpellingError> {
        let nominal_source = nominal_srate_spelling(fields.nominal_sample_rate())?;
        let channel_count = spell_channel_count(fields.channel_count())?;
        let nominal_srate = copy_nominal_srate(nominal_source)?;

        Ok(Self {
            _fields: fields,
            channel_count,
            nominal_srate,
        })
    }

    /// Returns the bounded channel-count text.
    #[must_use]
    pub fn channel_count(&self) -> &str {
        &self.channel_count
    }

    /// Returns the bounded nominal-sample-rate text.
    #[must_use]
    pub fn nominal_srate(&self) -> &str {
        &self.nominal_srate
    }
}

/// Deterministic rejection from the bounded LSLC-001L lexical policy.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoStaticNumericSpellingError {
    /// The channel-count output allocation could not be reserved.
    ChannelCountAllocationFailed {
        /// The exact byte count requested from the allocator.
        requested: usize,
    },
    /// A regular rate had no accepted observed fixed-decimal spelling.
    UnsupportedRegularNominalSrate {
        /// The unchanged bits of the rejected regular `f64` value.
        actual_bits: u64,
    },
    /// The nominal-rate output allocation could not be reserved.
    NominalSrateAllocationFailed {
        /// The exact byte count requested from the allocator.
        requested: usize,
    },
}

impl fmt::Display for StreamInfoStaticNumericSpellingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "static numeric spelling rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoStaticNumericSpellingError {}

fn spell_channel_count(
    channel_count: usize,
) -> Result<String, StreamInfoStaticNumericSpellingError> {
    let mut reversed_digits = [0_u8; MAX_CHANNEL_COUNT_BYTES];
    let mut remaining = channel_count;
    let mut digits = 0;
    loop {
        reversed_digits[digits] = DECIMAL_DIGITS[remaining % 10];
        digits += 1;
        remaining /= 10;
        if remaining == 0 {
            break;
        }
    }

    let mut spelling = String::new();
    spelling.try_reserve_exact(digits).map_err(|_| {
        StreamInfoStaticNumericSpellingError::ChannelCountAllocationFailed { requested: digits }
    })?;
    for digit in reversed_digits[..digits].iter().rev() {
        spelling.push(char::from(*digit));
    }
    debug_assert_eq!(spelling.len(), digits);
    Ok(spelling)
}

fn nominal_srate_spelling(
    rate: NominalSampleRate,
) -> Result<&'static str, StreamInfoStaticNumericSpellingError> {
    let NominalSampleRate::RegularHz(rate) = rate else {
        return Ok(IRREGULAR_SPELLING);
    };
    let bits = rate.hz().to_bits();
    let spelling = if bits == 100.0_f64.to_bits() {
        "100.0000000000000"
    } else if bits == 59.94_f64.to_bits() {
        "59.94000000000000"
    } else if bits == 1.0_f64.to_bits() {
        "1.000000000000000"
    } else if bits == 256.5_f64.to_bits() {
        "256.5000000000000"
    } else if bits == 1_000_000.25_f64.to_bits() {
        "1000000.250000000"
    } else {
        return Err(
            StreamInfoStaticNumericSpellingError::UnsupportedRegularNominalSrate {
                actual_bits: bits,
            },
        );
    };
    Ok(spelling)
}

fn copy_nominal_srate(source: &str) -> Result<String, StreamInfoStaticNumericSpellingError> {
    debug_assert_eq!(source.len(), NUMERIC_SPELLING_BYTES);
    let mut spelling = String::new();
    spelling.try_reserve_exact(source.len()).map_err(|_| {
        StreamInfoStaticNumericSpellingError::NominalSrateAllocationFailed {
            requested: source.len(),
        }
    })?;
    spelling.push_str(source);
    Ok(spelling)
}

#[cfg(test)]
mod tests {
    use super::{
        StreamInfoStaticNumericSpellingError, StreamInfoStaticNumericSpellings,
        MAX_CHANNEL_COUNT_BYTES, NUMERIC_SPELLING_BYTES,
    };
    use crate::{
        ChannelFormat, MetadataNodeInput, MetadataTree, MetadataTreeLimits, NominalSampleRate,
        StreamDefinition, StreamDescriptor, StreamDescriptorLimits, StreamInfoStaticFields,
    };

    fn definition(channel_count: usize, rate: NominalSampleRate) -> StreamDefinition {
        let descriptor = StreamDescriptor::new(
            StreamDescriptorLimits::new(16, 16, 16, usize::MAX).unwrap(),
            "numeric-case".to_owned(),
            None,
            None,
            channel_count,
            rate,
            ChannelFormat::Float32,
        )
        .unwrap();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 16, 16).unwrap(),
            vec![MetadataNodeInput::new(
                None,
                "generic-root".to_owned(),
                None,
            )],
        )
        .unwrap();
        StreamDefinition::new(descriptor, metadata)
    }

    #[test]
    fn lslc_001l_seven_observed_numeric_cases_execute_exactly() {
        let cases = [
            (1, NominalSampleRate::irregular(), "0.000000000000000"),
            (
                2,
                NominalSampleRate::regular_hz(100.0).unwrap(),
                "100.0000000000000",
            ),
            (
                3,
                NominalSampleRate::regular_hz(59.94).unwrap(),
                "59.94000000000000",
            ),
            (
                4,
                NominalSampleRate::regular_hz(1.0).unwrap(),
                "1.000000000000000",
            ),
            (
                5,
                NominalSampleRate::regular_hz(256.5).unwrap(),
                "256.5000000000000",
            ),
            (6, NominalSampleRate::irregular(), "0.000000000000000"),
            (
                7,
                NominalSampleRate::regular_hz(1_000_000.25).unwrap(),
                "1000000.250000000",
            ),
        ];

        for (channel_count, rate, expected_rate) in cases {
            let definition = definition(channel_count, rate);
            let fields = StreamInfoStaticFields::new(&definition);
            let spellings = StreamInfoStaticNumericSpellings::new(&fields).unwrap();
            assert_eq!(spellings.channel_count(), channel_count.to_string());
            assert_eq!(spellings.nominal_srate(), expected_rate);
            assert_eq!(spellings.nominal_srate().len(), NUMERIC_SPELLING_BYTES);
        }
    }

    #[test]
    fn lslc_001l_unsupported_regular_rates_fail_closed_with_exact_bits() {
        for value in [
            2.0,
            f64::from_bits(59.94_f64.to_bits() + 1),
            f64::from_bits(100.0_f64.to_bits() - 1),
        ] {
            let rate = NominalSampleRate::regular_hz(value).unwrap();
            let definition = definition(1, rate);
            let fields = StreamInfoStaticFields::new(&definition);
            let result = StreamInfoStaticNumericSpellings::new(&fields);
            assert!(matches!(
                result,
                Err(StreamInfoStaticNumericSpellingError::UnsupportedRegularNominalSrate {
                    actual_bits,
                }) if actual_bits == value.to_bits()
            ));
        }
    }

    #[test]
    fn lslc_001l_source_remains_borrowed_unchanged_and_reusable() {
        let rate = NominalSampleRate::regular_hz(256.5).unwrap();
        let definition = definition(usize::MAX, rate);
        let fields = StreamInfoStaticFields::new(&definition);
        let spellings = StreamInfoStaticNumericSpellings::new(&fields).unwrap();

        assert_eq!(spellings.channel_count(), usize::MAX.to_string());
        assert!(spellings.channel_count().len() <= MAX_CHANNEL_COUNT_BYTES);
        assert_eq!(fields.channel_count(), usize::MAX);
        assert_eq!(fields.nominal_sample_rate(), rate);
        assert_eq!(definition.descriptor().channel_count(), usize::MAX);
    }
}
