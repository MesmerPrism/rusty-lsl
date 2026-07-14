// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded source-only short-info response-envelope contract.

use crate::{
    ParsedStreamInfoObservedDocument, StreamInfoObservedDocumentParseError,
    StreamInfoObservedDocumentParseLimit,
};

/// Nonzero body and complete-envelope byte limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortInfoResponseEnvelopeLimits {
    max_body_bytes: usize,
    max_envelope_bytes: usize,
}

impl ShortInfoResponseEnvelopeLimits {
    /// Creates explicit nonzero limits.
    pub fn new(
        max_body_bytes: usize,
        max_envelope_bytes: usize,
    ) -> Result<Self, ShortInfoResponseEnvelopeLimitError> {
        if max_body_bytes == 0 {
            return Err(ShortInfoResponseEnvelopeLimitError::ZeroBodyBytes);
        }
        if max_envelope_bytes == 0 {
            return Err(ShortInfoResponseEnvelopeLimitError::ZeroEnvelopeBytes);
        }
        Ok(Self {
            max_body_bytes,
            max_envelope_bytes,
        })
    }
    /// Maximum body bytes.
    pub const fn max_body_bytes(self) -> usize {
        self.max_body_bytes
    }
    /// Maximum complete-envelope bytes.
    pub const fn max_envelope_bytes(self) -> usize {
        self.max_envelope_bytes
    }
}

/// Invalid response-envelope limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShortInfoResponseEnvelopeLimitError {
    /// Body maximum was zero.
    ZeroBodyBytes,
    /// Envelope maximum was zero.
    ZeroEnvelopeBytes,
}

/// Owned canonical envelope bytes.
#[derive(Debug, Eq, PartialEq)]
pub struct ShortInfoResponseEnvelope {
    limits: ShortInfoResponseEnvelopeLimits,
    bytes: Vec<u8>,
}

impl ShortInfoResponseEnvelope {
    /// Encodes an uninterpreted identifier, CRLF, and an unchanged accepted body.
    pub fn encode(
        query_id: u64,
        body: &ParsedStreamInfoObservedDocument<'_>,
        limits: ShortInfoResponseEnvelopeLimits,
    ) -> Result<Self, ShortInfoResponseEnvelopeEncodeError> {
        if body.source().len() > limits.max_body_bytes {
            return Err(ShortInfoResponseEnvelopeEncodeError::BodyLimitExceeded {
                expected: limits.max_body_bytes,
                actual: body.source().len(),
            });
        }
        let required = decimal_len(query_id)
            .checked_add(2)
            .and_then(|v| v.checked_add(body.source().len()))
            .ok_or(ShortInfoResponseEnvelopeEncodeError::LengthOverflow)?;
        if required > limits.max_envelope_bytes {
            return Err(
                ShortInfoResponseEnvelopeEncodeError::EnvelopeLimitExceeded {
                    expected: limits.max_envelope_bytes,
                    required,
                },
            );
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(required).map_err(|_| {
            ShortInfoResponseEnvelopeEncodeError::AllocationFailed {
                requested: required,
            }
        })?;
        write_decimal(&mut bytes, query_id);
        bytes.extend_from_slice(b"\r\n");
        bytes.extend_from_slice(body.source().as_bytes());
        Ok(Self { limits, bytes })
    }
    /// Selected limits.
    pub const fn limits(&self) -> ShortInfoResponseEnvelopeLimits {
        self.limits
    }
    /// Canonical bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Recovers the exact allocation.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Encoding rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShortInfoResponseEnvelopeEncodeError {
    /// Body exceeded its maximum.
    BodyLimitExceeded {
        /// Maximum.
        expected: usize,
        /// Actual.
        actual: usize,
    },
    /// Checked length overflowed.
    LengthOverflow,
    /// Complete envelope exceeded its maximum.
    EnvelopeLimitExceeded {
        /// Maximum.
        expected: usize,
        /// Required.
        required: usize,
    },
    /// Exact reserve failed.
    AllocationFailed {
        /// Requested capacity.
        requested: usize,
    },
}

/// Borrowed accepted envelope and its separately shape-admitted body.
#[derive(Debug, Eq, PartialEq)]
pub struct ParsedShortInfoResponseEnvelope<'a> {
    source: &'a str,
    query_id: u64,
    body: ParsedStreamInfoObservedDocument<'a>,
}

impl<'a> ParsedShortInfoResponseEnvelope<'a> {
    /// Parses only canonical u64 decimal + CRLF + LSLC-002A body.
    pub fn parse(
        source: &'a str,
        limits: ShortInfoResponseEnvelopeLimits,
    ) -> Result<Self, ShortInfoResponseEnvelopeParseError> {
        if source.len() > limits.max_envelope_bytes {
            return Err(ShortInfoResponseEnvelopeParseError::EnvelopeLimitExceeded {
                expected: limits.max_envelope_bytes,
                actual: source.len(),
            });
        }
        let bytes = source.as_bytes();
        let cr = bytes
            .iter()
            .position(|b| *b == b'\r' || *b == b'\n')
            .ok_or(ShortInfoResponseEnvelopeParseError::Truncated {
                offset: source.len(),
            })?;
        if bytes[cr] != b'\r' || bytes.get(cr + 1) != Some(&b'\n') {
            return Err(ShortInfoResponseEnvelopeParseError::InvalidDelimiter { offset: cr });
        }
        let query_id = parse_decimal(&bytes[..cr])?;
        let body_start = cr + 2;
        let body_source = &source[body_start..];
        let body_limit = StreamInfoObservedDocumentParseLimit::new(limits.max_body_bytes)
            .expect("nonzero response body limit");
        let body =
            ParsedStreamInfoObservedDocument::parse(body_limit, body_source).map_err(|error| {
                ShortInfoResponseEnvelopeParseError::Body {
                    offset: body_start + document_error_offset(&error),
                    error,
                }
            })?;
        Ok(Self {
            source,
            query_id,
            body,
        })
    }
    /// Unchanged complete source.
    pub const fn source(&self) -> &'a str {
        self.source
    }
    /// Uninterpreted identifier.
    pub const fn query_id(&self) -> u64 {
        self.query_id
    }
    /// Separately shape-admitted unchanged body.
    pub const fn body(&self) -> &ParsedStreamInfoObservedDocument<'a> {
        &self.body
    }
}

/// Parsing rejection with the complete-envelope first failing offset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShortInfoResponseEnvelopeParseError {
    /// Complete input exceeded its maximum.
    EnvelopeLimitExceeded {
        /// Maximum.
        expected: usize,
        /// Actual.
        actual: usize,
    },
    /// Delimiter or body was missing.
    Truncated {
        /// End offset.
        offset: usize,
    },
    /// Identifier was empty.
    EmptyIdentifier {
        /// Field start.
        offset: usize,
    },
    /// Leading zero made the identifier noncanonical.
    NonCanonicalIdentifier {
        /// First offending offset.
        offset: usize,
    },
    /// Identifier contained a nondigit.
    InvalidIdentifierByte {
        /// Offset.
        offset: usize,
        /// Byte.
        byte: u8,
    },
    /// Identifier overflowed u64.
    IdentifierOverflow {
        /// Digit offset.
        offset: usize,
    },
    /// Delimiter was not exact CRLF.
    InvalidDelimiter {
        /// First invalid offset.
        offset: usize,
    },
    /// LSLC-002A body admission failed.
    Body {
        /// Complete-envelope offset.
        offset: usize,
        /// Unchanged delegated error.
        error: StreamInfoObservedDocumentParseError,
    },
}

fn parse_decimal(bytes: &[u8]) -> Result<u64, ShortInfoResponseEnvelopeParseError> {
    if bytes.is_empty() {
        return Err(ShortInfoResponseEnvelopeParseError::EmptyIdentifier { offset: 0 });
    }
    if bytes.len() > 1 && bytes[0] == b'0' {
        return Err(ShortInfoResponseEnvelopeParseError::NonCanonicalIdentifier { offset: 0 });
    }
    let mut value = 0u64;
    for (offset, byte) in bytes.iter().copied().enumerate() {
        if !byte.is_ascii_digit() {
            return Err(ShortInfoResponseEnvelopeParseError::InvalidIdentifierByte {
                offset,
                byte,
            });
        }
        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(u64::from(byte - b'0')))
            .ok_or(ShortInfoResponseEnvelopeParseError::IdentifierOverflow { offset })?;
    }
    Ok(value)
}
fn document_error_offset(error: &StreamInfoObservedDocumentParseError) -> usize {
    match error {
        StreamInfoObservedDocumentParseError::InvalidLimit { .. }
        | StreamInfoObservedDocumentParseError::InputLimitExceeded { .. } => 0,
        StreamInfoObservedDocumentParseError::Truncated { byte_offset, .. }
        | StreamInfoObservedDocumentParseError::NonCanonical { byte_offset, .. }
        | StreamInfoObservedDocumentParseError::NonCanonicalCharacterData { byte_offset }
        | StreamInfoObservedDocumentParseError::TrailingInput { byte_offset } => *byte_offset,
    }
}
fn decimal_len(mut value: u64) -> usize {
    let mut n = 1;
    while value >= 10 {
        value /= 10;
        n += 1;
    }
    n
}
fn write_decimal(out: &mut Vec<u8>, mut value: u64) {
    let mut d = [0u8; 20];
    let mut i = 20;
    loop {
        i -= 1;
        d[i] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    out.extend_from_slice(&d[i..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn body() -> String {
        let names = [
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
        let mut s = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for n in names {
            s.push_str(&format!("\t<{n}>x</{n}>\n"));
        }
        s.push_str("\t<desc />\n</info>\n");
        s
    }
    fn limits(n: usize) -> ShortInfoResponseEnvelopeLimits {
        ShortInfoResponseEnvelopeLimits::new(n, n + 32).unwrap()
    }
    #[test]
    fn boundaries_round_trip_and_borrow() {
        let b = body();
        let l = limits(b.len());
        let p = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(b.len()).unwrap(),
            &b,
        )
        .unwrap();
        for id in [0, u64::MAX] {
            let e = ShortInfoResponseEnvelope::encode(id, &p, l).unwrap();
            let text = core::str::from_utf8(e.as_bytes()).unwrap();
            let q = ParsedShortInfoResponseEnvelope::parse(text, l).unwrap();
            assert_eq!(q.query_id(), id);
            assert_eq!(q.body().source(), b);
            assert_eq!(q.source().as_ptr(), text.as_ptr());
        }
    }
    #[test]
    fn damaged_identifiers_and_delimiters_reject_exactly() {
        let b = body();
        let l = limits(b.len());
        for (prefix, expected) in [
            (
                "01\r\n",
                ShortInfoResponseEnvelopeParseError::NonCanonicalIdentifier { offset: 0 },
            ),
            (
                "1x\r\n",
                ShortInfoResponseEnvelopeParseError::InvalidIdentifierByte {
                    offset: 1,
                    byte: b'x',
                },
            ),
            (
                "18446744073709551616\r\n",
                ShortInfoResponseEnvelopeParseError::IdentifierOverflow { offset: 19 },
            ),
            (
                "1\n",
                ShortInfoResponseEnvelopeParseError::InvalidDelimiter { offset: 1 },
            ),
            (
                "1\rX",
                ShortInfoResponseEnvelopeParseError::InvalidDelimiter { offset: 1 },
            ),
        ] {
            assert_eq!(
                ParsedShortInfoResponseEnvelope::parse(&(prefix.to_owned() + &b), l),
                Err(expected)
            );
        }
    }
    #[test]
    fn truncation_oversize_and_bad_body_reject() {
        let b = body();
        let l = limits(b.len());
        assert_eq!(
            ParsedShortInfoResponseEnvelope::parse("1", l),
            Err(ShortInfoResponseEnvelopeParseError::Truncated { offset: 1 })
        );
        let bad = "1\r\n".to_owned() + &b.replacen("<info>", "<inx>", 1);
        assert!(matches!(
            ParsedShortInfoResponseEnvelope::parse(&bad, l),
            Err(ShortInfoResponseEnvelopeParseError::Body { offset: _, .. })
        ));
        let tight = ShortInfoResponseEnvelopeLimits::new(b.len(), 1).unwrap();
        assert!(matches!(
            ParsedShortInfoResponseEnvelope::parse("12", tight),
            Err(ShortInfoResponseEnvelopeParseError::EnvelopeLimitExceeded { .. })
        ));
    }

    #[test]
    fn lslc_002g_extra_crlf_rejects_at_body_start_with_unchanged_document_error() {
        use crate::stream_info_observed_document_parser::ShapePart;

        let body = body();
        let source = "7\r\n\r\n".to_owned() + &body;
        assert_eq!(
            ParsedShortInfoResponseEnvelope::parse(&source, limits(body.len() + 2)),
            Err(ShortInfoResponseEnvelopeParseError::Body {
                offset: 3,
                error: StreamInfoObservedDocumentParseError::NonCanonical {
                    byte_offset: 0,
                    expected: ShapePart::DeclarationAndRoot,
                },
            })
        );
    }
}
