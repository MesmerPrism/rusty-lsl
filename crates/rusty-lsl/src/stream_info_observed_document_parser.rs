// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::{fmt, ops::Range};

const DECLARATION_AND_ROOT: &str = "<?xml version=\"1.0\"?>\n<info>\n";
const EMPTY_DESCRIPTION_AND_ROOT_END: &str = "\t<desc />\n</info>\n";
const FIELD_NAMES: [&str; 17] = [
    "name",
    "type",
    "channel_count",
    "channel_format",
    "source_id",
    "nominal_srate",
    "version",
    "created_at",
    "uid",
    "session_id",
    "hostname",
    "v4address",
    "v4data_port",
    "v4service_port",
    "v6address",
    "v6data_port",
    "v6service_port",
];
const FIELD_END_TAGS: [&str; 17] = [
    "</name>",
    "</type>",
    "</channel_count>",
    "</channel_format>",
    "</source_id>",
    "</nominal_srate>",
    "</version>",
    "</created_at>",
    "</uid>",
    "</session_id>",
    "</hostname>",
    "</v4address>",
    "</v4data_port>",
    "</v4service_port>",
    "</v6address>",
    "</v6data_port>",
    "</v6service_port>",
];

/// A nonzero byte maximum for one borrowed observed-document parse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoObservedDocumentParseLimit {
    max_input_bytes: usize,
}

impl StreamInfoObservedDocumentParseLimit {
    /// Creates a limit accepting at least one input byte.
    pub fn new(max_input_bytes: usize) -> Result<Self, StreamInfoObservedDocumentParseError> {
        if max_input_bytes == 0 {
            return Err(StreamInfoObservedDocumentParseError::InvalidLimit {
                expected_min: 1,
                actual: 0,
            });
        }
        Ok(Self { max_input_bytes })
    }

    #[must_use]
    /// Returns the caller-selected maximum input length.
    pub const fn max_input_bytes(self) -> usize {
        self.max_input_bytes
    }
}

/// A borrowed, bounded parse of the fixed empty-description LSLC-001R shape.
///
/// This is deliberately not a general XML parser. It retains the source and
/// the seventeen value byte ranges without decoding or interpreting values.
#[derive(Debug, Eq, PartialEq)]
pub struct ParsedStreamInfoObservedDocument<'a> {
    limit: StreamInfoObservedDocumentParseLimit,
    source: &'a str,
    value_ranges: [Range<usize>; FIELD_NAMES.len()],
}

impl<'a> ParsedStreamInfoObservedDocument<'a> {
    /// Parses the fixed canonical empty-description document shape.
    pub fn parse(
        limit: StreamInfoObservedDocumentParseLimit,
        source: &'a str,
    ) -> Result<Self, StreamInfoObservedDocumentParseError> {
        if source.len() > limit.max_input_bytes {
            return Err(StreamInfoObservedDocumentParseError::InputLimitExceeded {
                expected_max: limit.max_input_bytes,
                actual: source.len(),
            });
        }
        let bytes = source.as_bytes();
        let mut offset = expect(
            bytes,
            0,
            DECLARATION_AND_ROOT.as_bytes(),
            ShapePart::DeclarationAndRoot,
        )?;
        let mut value_ranges = core::array::from_fn(|_| 0..0);
        for (field_index, name) in FIELD_NAMES.into_iter().enumerate() {
            offset = expect(bytes, offset, b"\t<", ShapePart::FieldStart { field_index })?;
            offset = expect(
                bytes,
                offset,
                name.as_bytes(),
                ShapePart::FieldName { field_index },
            )?;
            offset = expect(bytes, offset, b">", ShapePart::FieldStart { field_index })?;
            let value_start = offset;
            let value_end = validate_character_data(source, value_start, field_index)?;
            value_ranges[field_index] = value_start..value_end;
            offset = expect(
                bytes,
                value_end,
                FIELD_END_TAGS[field_index].as_bytes(),
                ShapePart::FieldEnd { field_index },
            )?;
            offset = expect(bytes, offset, b"\n", ShapePart::FieldEnd { field_index })?;
        }
        offset = expect(
            bytes,
            offset,
            EMPTY_DESCRIPTION_AND_ROOT_END.as_bytes(),
            ShapePart::EmptyDescriptionAndRootEnd,
        )?;
        if offset != bytes.len() {
            return Err(StreamInfoObservedDocumentParseError::TrailingInput {
                byte_offset: offset,
            });
        }
        Ok(Self {
            limit,
            source,
            value_ranges,
        })
    }

    #[must_use]
    /// Returns the selected input bound.
    pub const fn limit(&self) -> StreamInfoObservedDocumentParseLimit {
        self.limit
    }
    #[must_use]
    /// Returns the unchanged borrowed input.
    pub const fn source(&self) -> &'a str {
        self.source
    }
    #[must_use]
    /// Returns one represented field value by its fixed zero-based role index.
    pub fn value(&self, field_index: usize) -> Option<&'a str> {
        self.value_ranges
            .get(field_index)
            .map(|range| &self.source[range.clone()])
    }
    #[must_use]
    /// Returns all fixed-role value ranges in document order.
    pub fn value_ranges(&self) -> &[Range<usize>] {
        &self.value_ranges
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShapePart {
    DeclarationAndRoot,
    FieldStart { field_index: usize },
    FieldName { field_index: usize },
    FieldEnd { field_index: usize },
    EmptyDescriptionAndRootEnd,
}

/// Deterministic rejection from bounded observed-document shape parsing.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoObservedDocumentParseError {
    InvalidLimit {
        expected_min: usize,
        actual: usize,
    },
    InputLimitExceeded {
        expected_max: usize,
        actual: usize,
    },
    Truncated {
        byte_offset: usize,
        expected: ShapePart,
    },
    NonCanonical {
        byte_offset: usize,
        expected: ShapePart,
    },
    NonCanonicalCharacterData {
        byte_offset: usize,
    },
    TrailingInput {
        byte_offset: usize,
    },
}

impl fmt::Display for StreamInfoObservedDocumentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "observed stream-info document parse rejected input: {self:?}"
        )
    }
}
impl std::error::Error for StreamInfoObservedDocumentParseError {}

fn expect(
    bytes: &[u8],
    offset: usize,
    expected: &[u8],
    part: ShapePart,
) -> Result<usize, StreamInfoObservedDocumentParseError> {
    let available = bytes.len().saturating_sub(offset);
    let compared = available.min(expected.len());
    if bytes.get(offset..offset + compared) != Some(&expected[..compared]) {
        let mismatch = (0..compared)
            .find(|index| bytes[offset + index] != expected[*index])
            .unwrap_or(compared);
        return Err(StreamInfoObservedDocumentParseError::NonCanonical {
            byte_offset: offset + mismatch,
            expected: part,
        });
    }
    if available < expected.len() {
        return Err(StreamInfoObservedDocumentParseError::Truncated {
            byte_offset: bytes.len(),
            expected: part,
        });
    }
    Ok(offset + expected.len())
}

fn validate_character_data(
    source: &str,
    value_start: usize,
    field_index: usize,
) -> Result<usize, StreamInfoObservedDocumentParseError> {
    let bytes = source.as_bytes();
    let mut offset = value_start;
    while offset < bytes.len() {
        match bytes[offset] {
            b'<' => return Ok(offset),
            b'>' => {
                return Err(
                    StreamInfoObservedDocumentParseError::NonCanonicalCharacterData {
                        byte_offset: offset,
                    },
                )
            }
            b'&' => {
                let accepted = ["&amp;", "&lt;", "&gt;"]
                    .into_iter()
                    .find(|entity| source[offset..].starts_with(entity));
                if let Some(entity) = accepted {
                    offset += entity.len();
                } else {
                    return Err(
                        StreamInfoObservedDocumentParseError::NonCanonicalCharacterData {
                            byte_offset: offset,
                        },
                    );
                }
            }
            _ => {
                let character = source[offset..].chars().next().ok_or(
                    StreamInfoObservedDocumentParseError::Truncated {
                        byte_offset: bytes.len(),
                        expected: ShapePart::FieldEnd { field_index },
                    },
                )?;
                if !is_xml_char(character) {
                    return Err(
                        StreamInfoObservedDocumentParseError::NonCanonicalCharacterData {
                            byte_offset: offset,
                        },
                    );
                }
                offset += character.len_utf8();
            }
        }
    }
    Err(StreamInfoObservedDocumentParseError::Truncated {
        byte_offset: bytes.len(),
        expected: ShapePart::FieldEnd { field_index },
    })
}

const fn is_xml_char(character: char) -> bool {
    matches!(
        character as u32,
        0x9 | 0xA | 0xD | 0x20..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> String {
        let mut text = String::from(DECLARATION_AND_ROOT);
        for (index, name) in FIELD_NAMES.into_iter().enumerate() {
            text.push_str(&format!("\t<{name}>value-{index}-&amp;-雪</{name}>\n"));
        }
        text.push_str(EMPTY_DESCRIPTION_AND_ROOT_END);
        text
    }

    #[test]
    fn lslc_002a_valid_shape_borrows_source_and_indexes_values() {
        let source = valid();
        let parsed = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(source.len()).unwrap(),
            &source,
        )
        .unwrap();
        assert_eq!(parsed.source().as_ptr(), source.as_ptr());
        assert_eq!(parsed.value_ranges().len(), 17);
        assert_eq!(parsed.value(0), Some("value-0-&amp;-雪"));
        assert_eq!(parsed.value(16), Some("value-16-&amp;-雪"));
    }

    #[test]
    fn lslc_002a_damaged_and_truncated_inputs_report_first_offset() {
        let source = valid();
        let damaged = source.replacen("<channel_count>", "<channel-count>", 1);
        let changed_offset = damaged.find("channel-count").unwrap() + "channel".len();
        assert_eq!(
            ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(damaged.len()).unwrap(),
                &damaged
            ),
            Err(StreamInfoObservedDocumentParseError::NonCanonical {
                byte_offset: changed_offset,
                expected: ShapePart::FieldName { field_index: 2 },
            })
        );
        let truncated = &source[..source.len() - 4];
        assert_eq!(
            ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(source.len()).unwrap(),
                truncated
            ),
            Err(StreamInfoObservedDocumentParseError::Truncated {
                byte_offset: truncated.len(),
                expected: ShapePart::EmptyDescriptionAndRootEnd,
            })
        );
    }

    #[test]
    fn lslc_002a_malformed_closing_tag_fails_at_its_first_changed_byte() {
        let source = valid().replacen("</uid>", "</uix>", 1);
        let changed_offset = source.find("</uix>").unwrap() + 4;
        assert_eq!(
            ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(source.len()).unwrap(),
                &source,
            ),
            Err(StreamInfoObservedDocumentParseError::NonCanonical {
                byte_offset: changed_offset,
                expected: ShapePart::FieldEnd { field_index: 8 },
            })
        );
    }

    #[test]
    fn lslc_002a_oversized_and_zero_limits_fail_before_shape() {
        let source = valid();
        assert_eq!(
            StreamInfoObservedDocumentParseLimit::new(0),
            Err(StreamInfoObservedDocumentParseError::InvalidLimit {
                expected_min: 1,
                actual: 0
            })
        );
        assert_eq!(
            ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(source.len() - 1).unwrap(),
                &source
            ),
            Err(StreamInfoObservedDocumentParseError::InputLimitExceeded {
                expected_max: source.len() - 1,
                actual: source.len()
            })
        );
    }

    #[test]
    fn lslc_002a_noncanonical_forms_and_character_data_fail_closed() {
        for changed in [
            valid().replacen("\n<info>", "\r\n<info>", 1),
            valid().replacen("\t<name>", "  <name>", 1),
            valid().replacen("&amp;", "&quot;", 1),
            valid().replacen("<desc />", "<desc></desc>", 1),
        ] {
            assert!(ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(changed.len()).unwrap(),
                &changed
            )
            .is_err());
        }
    }

    #[test]
    fn lslc_002a_character_data_is_utf8_scalar_checked_without_decoding() {
        let accepted = valid().replacen("value-0-&amp;-雪", "\t\n\r&amp;&lt;&gt;'\"雪", 1);
        let parsed = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(accepted.len()).unwrap(),
            &accepted,
        )
        .unwrap();
        assert_eq!(parsed.value(0), Some("\t\n\r&amp;&lt;&gt;'\"雪"));

        let damaged = valid().replacen("value-0", "value\0-0", 1);
        let nul_offset = damaged.find('\0').unwrap();
        assert_eq!(
            ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(damaged.len()).unwrap(),
                &damaged,
            ),
            Err(
                StreamInfoObservedDocumentParseError::NonCanonicalCharacterData {
                    byte_offset: nul_offset,
                }
            )
        );
    }
}
