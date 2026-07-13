// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::XmlText;
use core::fmt;

const AMPERSAND: &str = "&amp;";
const LESS_THAN: &str = "&lt;";
const GREATER_THAN: &str = "&gt;";

/// A validated nonzero maximum encoded UTF-8 byte count for [`XmlCharacterData`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XmlCharacterDataLimit {
    max_encoded_bytes: usize,
}

impl XmlCharacterDataLimit {
    /// Creates a character-data limit that can accept at least one encoded byte.
    pub const fn new(max_encoded_bytes: usize) -> Result<Self, XmlCharacterDataError> {
        if max_encoded_bytes == 0 {
            return Err(XmlCharacterDataError::InvalidLimit {
                expected_min: 1,
                actual: max_encoded_bytes,
            });
        }
        Ok(Self { max_encoded_bytes })
    }

    /// Returns the maximum accepted encoded UTF-8 byte count.
    #[must_use]
    pub const fn max_encoded_bytes(self) -> usize {
        self.max_encoded_bytes
    }
}

/// A bounded XML character-data representation of an existing validated [`XmlText`].
///
/// Ampersand, less-than, and greater-than are represented by `&amp;`, `&lt;`,
/// and `&gt;`, respectively. Every other accepted scalar remains unchanged.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlCharacterData {
    limit: XmlCharacterDataLimit,
    encoded: String,
}

impl XmlCharacterData {
    /// Encodes borrowed validated text under the fixed local character-data policy.
    ///
    /// The exact encoded length is checked for arithmetic overflow and against
    /// `limit` before a fallible allocation is attempted. The source is neither
    /// consumed nor revalidated.
    pub fn encode(
        limit: XmlCharacterDataLimit,
        text: &XmlText,
    ) -> Result<Self, XmlCharacterDataError> {
        let required = encoded_length(text.as_str())?;
        if required > limit.max_encoded_bytes {
            return Err(XmlCharacterDataError::LimitExceeded {
                expected_max: limit.max_encoded_bytes,
                required,
            });
        }

        let mut encoded = String::new();
        reserve_exact(&mut encoded, required)?;

        for character in text.as_str().chars() {
            match character {
                '&' => encoded.push_str(AMPERSAND),
                '<' => encoded.push_str(LESS_THAN),
                '>' => encoded.push_str(GREATER_THAN),
                unchanged => encoded.push(unchanged),
            }
        }
        debug_assert_eq!(encoded.len(), required);

        Ok(Self { limit, encoded })
    }

    /// Returns the encoded-byte limit under which this value was accepted.
    #[must_use]
    pub const fn limit(&self) -> XmlCharacterDataLimit {
        self.limit
    }

    /// Returns the encoded character data.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.encoded
    }

    /// Returns the encoded string without replacing its allocation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.encoded
    }
}

/// Deterministic rejection from character-data limit configuration or encoding.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum XmlCharacterDataError {
    /// The configured maximum cannot accept any nonempty encoded output.
    InvalidLimit {
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// The exact encoded UTF-8 byte length could not be represented by `usize`.
    LengthOverflow,
    /// The encoded output exceeded its configured UTF-8 byte maximum.
    LimitExceeded {
        /// The configured maximum.
        expected_max: usize,
        /// The exact encoded byte count required by the caller text.
        required: usize,
    },
    /// The encoded output allocation could not be reserved.
    AllocationFailed {
        /// The exact encoded byte count requested from the allocator.
        requested: usize,
    },
}

impl fmt::Display for XmlCharacterDataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "XML character data rejected input: {self:?}")
    }
}

impl std::error::Error for XmlCharacterDataError {}

fn encoded_length(text: &str) -> Result<usize, XmlCharacterDataError> {
    checked_length_sum(text.chars().map(encoded_character_length))
        .ok_or(XmlCharacterDataError::LengthOverflow)
}

fn reserve_exact(encoded: &mut String, requested: usize) -> Result<(), XmlCharacterDataError> {
    encoded
        .try_reserve_exact(requested)
        .map_err(|_| XmlCharacterDataError::AllocationFailed { requested })
}

fn encoded_character_length(character: char) -> usize {
    match character {
        '&' => AMPERSAND.len(),
        '<' => LESS_THAN.len(),
        '>' => GREATER_THAN.len(),
        unchanged => unchanged.len_utf8(),
    }
}

fn checked_length_sum(lengths: impl IntoIterator<Item = usize>) -> Option<usize> {
    lengths
        .into_iter()
        .try_fold(0usize, |total, length| total.checked_add(length))
}

#[cfg(test)]
mod tests {
    use super::{
        checked_length_sum, reserve_exact, XmlCharacterData, XmlCharacterDataError,
        XmlCharacterDataLimit,
    };
    use crate::{XmlText, XmlTextLimit};

    fn text(value: &str) -> XmlText {
        XmlText::new(
            XmlTextLimit::new(value.chars().count().max(1)).unwrap(),
            String::from(value),
        )
        .unwrap()
    }

    fn encode(value: &str) -> XmlCharacterData {
        XmlCharacterData::encode(XmlCharacterDataLimit::new(256).unwrap(), &text(value)).unwrap()
    }

    #[test]
    fn lslc_001c_empty_and_each_fixed_escape_are_exact() {
        assert_eq!(encode("").as_str(), "");
        assert_eq!(encode("&").as_str(), "&amp;");
        assert_eq!(encode("<").as_str(), "&lt;");
        assert_eq!(encode(">").as_str(), "&gt;");
        assert_eq!(encode("&<>").as_str(), "&amp;&lt;&gt;");
    }

    #[test]
    fn lslc_001c_close_delimiter_and_reference_like_text_are_represented_literally() {
        assert_eq!(encode("&<>]]>").as_str(), "&amp;&lt;&gt;]]&gt;");
        assert!(!encode("]]>").as_str().contains("]]>"));
        assert_eq!(encode("&amp;").as_str(), "&amp;amp;");
    }

    #[test]
    fn lslc_001c_quotes_apostrophes_whitespace_and_unicode_are_preserved() {
        let value = "\"'\t\n\ré中𐀀\u{FDD0}\u{1FFFE}";
        assert_eq!(encode(value).as_str(), value);
    }

    #[test]
    fn lslc_001c_encoded_byte_bounds_are_exact() {
        let source = text("é&𐀀>");
        let required = "é&amp;𐀀&gt;".len();
        let exact = XmlCharacterDataLimit::new(required).unwrap();
        let encoded = XmlCharacterData::encode(exact, &source).unwrap();
        assert_eq!(encoded.limit(), exact);
        assert_eq!(encoded.as_str().len(), required);
        assert_eq!(
            XmlCharacterData::encode(XmlCharacterDataLimit::new(required - 1).unwrap(), &source),
            Err(XmlCharacterDataError::LimitExceeded {
                expected_max: required - 1,
                required,
            })
        );
    }

    #[test]
    fn lslc_001c_source_is_unchanged_and_reusable() {
        let source = text("&<>]]>");
        let source_pointer = source.as_str().as_ptr();
        let first =
            XmlCharacterData::encode(XmlCharacterDataLimit::new(64).unwrap(), &source).unwrap();
        let second =
            XmlCharacterData::encode(XmlCharacterDataLimit::new(64).unwrap(), &source).unwrap();
        assert_eq!(source.as_str(), "&<>]]>");
        assert_eq!(source.as_str().as_ptr(), source_pointer);
        assert_eq!(first, second);
    }

    #[test]
    fn lslc_001c_consuming_access_preserves_output_allocation() {
        let encoded = encode("A&B");
        let pointer = encoded.as_str().as_ptr();
        let returned = encoded.into_string();
        assert_eq!(returned.as_ptr(), pointer);
        assert_eq!(returned, "A&amp;B");
    }

    #[test]
    fn lslc_001c_invalid_limit_and_error_precedence_are_stable() {
        assert_eq!(
            XmlCharacterDataLimit::new(0),
            Err(XmlCharacterDataError::InvalidLimit {
                expected_min: 1,
                actual: 0,
            })
        );
        assert_eq!(checked_length_sum([usize::MAX, 1]), None);

        let mut encoded = String::new();
        assert_eq!(
            reserve_exact(&mut encoded, usize::MAX),
            Err(XmlCharacterDataError::AllocationFailed {
                requested: usize::MAX,
            })
        );

        let source = text("&");
        assert_eq!(
            XmlCharacterData::encode(XmlCharacterDataLimit::new(4).unwrap(), &source),
            Err(XmlCharacterDataError::LimitExceeded {
                expected_max: 4,
                required: 5,
            })
        );
    }
}
