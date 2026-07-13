// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// A validated nonzero Unicode scalar-value maximum for [`XmlText`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XmlTextLimit {
    max_code_points: usize,
}

impl XmlTextLimit {
    /// Creates a text limit that can accept at least one Unicode scalar value.
    pub const fn new(max_code_points: usize) -> Result<Self, XmlTextError> {
        if max_code_points == 0 {
            return Err(XmlTextError::InvalidLimit {
                expected_min: 1,
                actual: max_code_points,
            });
        }
        Ok(Self { max_code_points })
    }

    /// Returns the maximum accepted Unicode scalar-value count.
    #[must_use]
    pub const fn max_code_points(self) -> usize {
        self.max_code_points
    }
}

/// Text accepted under the XML 1.0 Fifth Edition `Char` production.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlText {
    limit: XmlTextLimit,
    text: String,
}

impl XmlText {
    /// Validates a complete caller-owned string without changing its allocation.
    ///
    /// Length is checked before scalar legality. Empty text is accepted.
    pub fn new(limit: XmlTextLimit, text: String) -> Result<Self, XmlTextError> {
        let actual = text.chars().count();
        if actual > limit.max_code_points {
            return Err(XmlTextError::LimitExceeded {
                expected_max: limit.max_code_points,
                actual,
            });
        }

        for (index, character) in text.chars().enumerate() {
            if !is_xml_char(character) {
                return Err(XmlTextError::IllegalCharacter {
                    index,
                    code_point: character as u32,
                });
            }
        }

        Ok(Self { limit, text })
    }

    /// Returns the limit under which this text was accepted.
    #[must_use]
    pub const fn limit(&self) -> XmlTextLimit {
        self.limit
    }

    /// Returns the unchanged accepted text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Returns the unchanged caller-owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.text
    }
}

/// Deterministic rejection from XML text limit configuration or validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum XmlTextError {
    /// The configured maximum cannot accept any nonempty text.
    InvalidLimit {
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// The text exceeded its configured Unicode scalar-value maximum.
    LimitExceeded {
        /// The configured maximum.
        expected_max: usize,
        /// The input's Unicode scalar-value count.
        actual: usize,
    },
    /// The first scalar outside the XML `Char` production.
    IllegalCharacter {
        /// Zero-based Unicode scalar-value index.
        index: usize,
        /// Rejected Unicode code point.
        code_point: u32,
    },
}

impl fmt::Display for XmlTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "XML text rejected input: {self:?}")
    }
}

impl std::error::Error for XmlTextError {}

/// A validated nonzero Unicode scalar-value maximum for [`XmlElementName`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XmlNameLimit {
    max_code_points: usize,
}

impl XmlNameLimit {
    /// Creates an element-name limit that can accept at least one scalar value.
    pub const fn new(max_code_points: usize) -> Result<Self, XmlNameError> {
        if max_code_points == 0 {
            return Err(XmlNameError::InvalidLimit {
                expected_min: 1,
                actual: max_code_points,
            });
        }
        Ok(Self { max_code_points })
    }

    /// Returns the maximum accepted Unicode scalar-value count.
    #[must_use]
    pub const fn max_code_points(self) -> usize {
        self.max_code_points
    }
}

/// An element name accepted under the XML 1.0 Fifth Edition `Name` production.
///
/// A colon is accepted only as name syntax. This value assigns no namespace
/// interpretation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlElementName {
    limit: XmlNameLimit,
    name: String,
}

impl XmlElementName {
    /// Validates a complete caller-owned name without changing its allocation.
    ///
    /// Rejection order is empty, length, invalid start, then the first invalid
    /// continuation.
    pub fn new(limit: XmlNameLimit, name: String) -> Result<Self, XmlNameError> {
        if name.is_empty() {
            return Err(XmlNameError::Empty);
        }

        let actual = name.chars().count();
        if actual > limit.max_code_points {
            return Err(XmlNameError::LimitExceeded {
                expected_max: limit.max_code_points,
                actual,
            });
        }

        let mut characters = name.chars();
        let Some(first) = characters.next() else {
            return Err(XmlNameError::Empty);
        };
        if !is_name_start_char(first) {
            return Err(XmlNameError::InvalidStart {
                index: 0,
                code_point: first as u32,
            });
        }

        for (offset, character) in characters.enumerate() {
            if !is_name_char(character) {
                return Err(XmlNameError::InvalidContinuation {
                    index: offset + 1,
                    code_point: character as u32,
                });
            }
        }

        Ok(Self { limit, name })
    }

    /// Returns the limit under which this name was accepted.
    #[must_use]
    pub const fn limit(&self) -> XmlNameLimit {
        self.limit
    }

    /// Returns the unchanged accepted name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Returns the unchanged caller-owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.name
    }
}

/// Deterministic rejection from XML element-name limit or grammar validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum XmlNameError {
    /// The configured maximum cannot accept any name.
    InvalidLimit {
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// An element name must contain at least one scalar value.
    Empty,
    /// The name exceeded its configured Unicode scalar-value maximum.
    LimitExceeded {
        /// The configured maximum.
        expected_max: usize,
        /// The input's Unicode scalar-value count.
        actual: usize,
    },
    /// The first scalar was not a `NameStartChar`.
    InvalidStart {
        /// Zero-based Unicode scalar-value index, always zero.
        index: usize,
        /// Rejected Unicode code point.
        code_point: u32,
    },
    /// A later scalar was not a `NameChar`.
    InvalidContinuation {
        /// Zero-based Unicode scalar-value index.
        index: usize,
        /// Rejected Unicode code point.
        code_point: u32,
    },
}

impl fmt::Display for XmlNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "XML element name rejected input: {self:?}")
    }
}

impl std::error::Error for XmlNameError {}

const fn is_xml_char(character: char) -> bool {
    matches!(
        character as u32,
        0x9 | 0xA | 0xD | 0x20..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF
    )
}

const fn is_name_start_char(character: char) -> bool {
    matches!(
        character as u32,
        0x3A
            | 0x41..=0x5A
            | 0x5F
            | 0x61..=0x7A
            | 0xC0..=0xD6
            | 0xD8..=0xF6
            | 0xF8..=0x2FF
            | 0x370..=0x37D
            | 0x37F..=0x1FFF
            | 0x200C..=0x200D
            | 0x2070..=0x218F
            | 0x2C00..=0x2FEF
            | 0x3001..=0xD7FF
            | 0xF900..=0xFDCF
            | 0xFDF0..=0xFFFD
            | 0x10000..=0xEFFFF
    )
}

const fn is_name_char(character: char) -> bool {
    is_name_start_char(character)
        || matches!(
            character as u32,
            0x2D | 0x2E | 0x30..=0x39 | 0xB7 | 0x300..=0x36F | 0x203F..=0x2040
        )
}

#[cfg(test)]
mod tests {
    use super::{XmlElementName, XmlNameError, XmlNameLimit, XmlText, XmlTextError, XmlTextLimit};

    fn scalar(code_point: u32) -> char {
        char::from_u32(code_point).expect("test code point must be a Unicode scalar value")
    }

    fn text_of(code_point: u32) -> String {
        scalar(code_point).to_string()
    }

    #[test]
    fn lslc_001b_text_char_production_boundaries() {
        let accepted = [
            0x9, 0xA, 0xD, 0x20, 0xD7FF, 0xE000, 0xFFFD, 0x10000, 0x10FFFF,
        ];
        for code_point in accepted {
            let value = text_of(code_point);
            assert_eq!(
                XmlText::new(XmlTextLimit::new(1).unwrap(), value.clone())
                    .unwrap()
                    .into_string(),
                value
            );
        }

        for code_point in [0x0, 0x8, 0xB, 0xC, 0xE, 0x1F, 0xFFFE, 0xFFFF] {
            assert_eq!(
                XmlText::new(XmlTextLimit::new(1).unwrap(), text_of(code_point)),
                Err(XmlTextError::IllegalCharacter {
                    index: 0,
                    code_point,
                })
            );
        }
        assert!(char::from_u32(0xD800).is_none());
        assert!(char::from_u32(0xDFFF).is_none());
    }

    #[test]
    fn lslc_001b_text_noncharacter_distinction_is_exact() {
        for code_point in [0xFDD0, 0x1FFFE] {
            assert!(XmlText::new(XmlTextLimit::new(1).unwrap(), text_of(code_point)).is_ok());
        }
        for code_point in [0xFFFE, 0xFFFF] {
            assert_eq!(
                XmlText::new(XmlTextLimit::new(1).unwrap(), text_of(code_point)),
                Err(XmlTextError::IllegalCharacter {
                    index: 0,
                    code_point,
                })
            );
        }
    }

    #[test]
    fn lslc_001b_text_empty_delimiters_and_allocation_are_preserved() {
        let empty = XmlText::new(XmlTextLimit::new(1).unwrap(), String::new()).unwrap();
        assert_eq!(empty.as_str(), "");

        let value = String::from("&<>]]>");
        let pointer = value.as_ptr();
        let limit = XmlTextLimit::new(6).unwrap();
        let accepted = XmlText::new(limit, value).unwrap();
        assert_eq!(accepted.limit(), limit);
        assert_eq!(accepted.as_str(), "&<>]]>");
        let returned = accepted.into_string();
        assert_eq!(returned.as_ptr(), pointer);
        assert_eq!(returned, "&<>]]>");
    }

    #[test]
    fn lslc_001b_text_limits_indexes_and_precedence_are_scalar_based() {
        assert_eq!(
            XmlTextLimit::new(0),
            Err(XmlTextError::InvalidLimit {
                expected_min: 1,
                actual: 0,
            })
        );

        let exact = String::from("Aé𐀀");
        assert!(XmlText::new(XmlTextLimit::new(3).unwrap(), exact).is_ok());
        assert_eq!(
            XmlText::new(XmlTextLimit::new(2).unwrap(), String::from("Aé𐀀")),
            Err(XmlTextError::LimitExceeded {
                expected_max: 2,
                actual: 3,
            })
        );
        assert_eq!(
            XmlText::new(XmlTextLimit::new(3).unwrap(), String::from("é𐀀\u{0}")),
            Err(XmlTextError::IllegalCharacter {
                index: 2,
                code_point: 0,
            })
        );
        assert_eq!(
            XmlText::new(XmlTextLimit::new(1).unwrap(), String::from("A\u{0}")),
            Err(XmlTextError::LimitExceeded {
                expected_max: 1,
                actual: 2,
            })
        );
    }

    #[test]
    fn lslc_001b_name_start_range_boundaries() {
        let boundaries = [
            0x3A, 0x41, 0x5A, 0x5F, 0x61, 0x7A, 0xC0, 0xD6, 0xD8, 0xF6, 0xF8, 0x2FF, 0x370, 0x37D,
            0x37F, 0x1FFF, 0x200C, 0x200D, 0x2070, 0x218F, 0x2C00, 0x2FEF, 0x3001, 0xD7FF, 0xF900,
            0xFDCF, 0xFDF0, 0xFFFD, 0x10000, 0xEFFFF,
        ];
        for code_point in boundaries {
            let value = text_of(code_point);
            assert_eq!(
                XmlElementName::new(XmlNameLimit::new(1).unwrap(), value.clone())
                    .unwrap()
                    .into_string(),
                value
            );
        }
    }

    #[test]
    fn lslc_001b_name_start_adjacent_failures() {
        for code_point in [
            0x39, 0x3B, 0x40, 0x5B, 0x5E, 0x60, 0x7B, 0xBF, 0xD7, 0xF7, 0x300, 0x36F, 0x37E,
            0x2000, 0x200B, 0x200E, 0x206F, 0x2190, 0x2BFF, 0x2FF0, 0x3000, 0xE000, 0xF8FF, 0xFDD0,
            0xFDEF, 0xFFFE, 0xF0000,
        ] {
            assert_eq!(
                XmlElementName::new(XmlNameLimit::new(1).unwrap(), text_of(code_point)),
                Err(XmlNameError::InvalidStart {
                    index: 0,
                    code_point,
                })
            );
        }
    }

    #[test]
    fn lslc_001b_name_char_additional_boundaries_and_adjacent_failures() {
        for code_point in [0x2D, 0x2E, 0x30, 0x39, 0xB7, 0x300, 0x36F, 0x203F, 0x2040] {
            let value = format!("A{}", scalar(code_point));
            assert!(XmlElementName::new(XmlNameLimit::new(2).unwrap(), value).is_ok());
            assert_eq!(
                XmlElementName::new(XmlNameLimit::new(1).unwrap(), text_of(code_point)),
                Err(XmlNameError::InvalidStart {
                    index: 0,
                    code_point,
                })
            );
        }

        for code_point in [0x2C, 0x2F, 0x3B, 0xB6, 0xB8, 0x203E, 0x2041, 0xF0000] {
            assert_eq!(
                XmlElementName::new(
                    XmlNameLimit::new(2).unwrap(),
                    format!("A{}", scalar(code_point)),
                ),
                Err(XmlNameError::InvalidContinuation {
                    index: 1,
                    code_point,
                })
            );
        }
    }

    #[test]
    fn lslc_001b_name_limits_indexes_precedence_and_allocation_are_exact() {
        assert_eq!(
            XmlNameLimit::new(0),
            Err(XmlNameError::InvalidLimit {
                expected_min: 1,
                actual: 0,
            })
        );
        assert_eq!(
            XmlElementName::new(XmlNameLimit::new(1).unwrap(), String::new()),
            Err(XmlNameError::Empty)
        );

        let value = String::from(":é𐀀-9");
        let pointer = value.as_ptr();
        let limit = XmlNameLimit::new(5).unwrap();
        let accepted = XmlElementName::new(limit, value).unwrap();
        assert_eq!(accepted.limit(), limit);
        assert_eq!(accepted.as_str(), ":é𐀀-9");
        let returned = accepted.into_string();
        assert_eq!(returned.as_ptr(), pointer);
        assert_eq!(returned, ":é𐀀-9");

        assert_eq!(
            XmlElementName::new(XmlNameLimit::new(4).unwrap(), String::from(":é𐀀-9")),
            Err(XmlNameError::LimitExceeded {
                expected_max: 4,
                actual: 5,
            })
        );
        assert_eq!(
            XmlElementName::new(XmlNameLimit::new(1).unwrap(), String::from("9?")),
            Err(XmlNameError::LimitExceeded {
                expected_max: 1,
                actual: 2,
            })
        );
        assert_eq!(
            XmlElementName::new(XmlNameLimit::new(2).unwrap(), String::from("9?")),
            Err(XmlNameError::InvalidStart {
                index: 0,
                code_point: 0x39,
            })
        );
        assert_eq!(
            XmlElementName::new(XmlNameLimit::new(4).unwrap(), String::from("Aé𐀀?")),
            Err(XmlNameError::InvalidContinuation {
                index: 3,
                code_point: 0x3F,
            })
        );
    }

    #[test]
    fn lslc_001b_colon_is_accepted_as_syntax_only() {
        let accepted =
            XmlElementName::new(XmlNameLimit::new(12).unwrap(), String::from(":alpha:beta:"))
                .unwrap();
        assert_eq!(accepted.as_str(), ":alpha:beta:");
    }
}
