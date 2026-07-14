// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded local byte shape for one protocol-110 short-info query candidate.

use core::fmt;

const HEADER: &[u8] = b"LSL:shortinfo\r\n";

/// Nonzero byte limits for one short-info query line and its complete payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortInfoQueryWireLimits {
    max_query_bytes: usize,
    max_payload_bytes: usize,
}

impl ShortInfoQueryWireLimits {
    /// Creates limits. Both maxima must be nonzero.
    pub fn new(
        max_query_bytes: usize,
        max_payload_bytes: usize,
    ) -> Result<Self, ShortInfoQueryWireLimitError> {
        if max_query_bytes == 0 {
            return Err(ShortInfoQueryWireLimitError::ZeroQueryBytes);
        }
        if max_payload_bytes == 0 {
            return Err(ShortInfoQueryWireLimitError::ZeroPayloadBytes);
        }
        Ok(Self {
            max_query_bytes,
            max_payload_bytes,
        })
    }

    /// Returns the maximum query-line byte count.
    pub const fn max_query_bytes(self) -> usize {
        self.max_query_bytes
    }

    /// Returns the maximum complete-payload byte count.
    pub const fn max_payload_bytes(self) -> usize {
        self.max_payload_bytes
    }
}

/// A malformed limit declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShortInfoQueryWireLimitError {
    /// The query-line maximum was zero.
    ZeroQueryBytes,
    /// The payload maximum was zero.
    ZeroPayloadBytes,
}

/// One owned, validated query line and its uninterpreted numeric fields.
#[derive(Debug, Eq, PartialEq)]
pub struct ShortInfoQuery {
    query: String,
    return_port: u16,
    query_id: u64,
}

impl ShortInfoQuery {
    /// Validates and retains one caller-owned query allocation.
    pub fn new(
        query: String,
        return_port: u16,
        query_id: u64,
        limits: ShortInfoQueryWireLimits,
    ) -> Result<Self, ShortInfoQueryValueError> {
        validate_query(query.as_bytes(), limits)?;
        if return_port == 0 {
            return Err(ShortInfoQueryValueError::ZeroReturnPort);
        }
        Ok(Self {
            query,
            return_port,
            query_id,
        })
    }

    /// Returns the unchanged query line.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the uninterpreted nonzero return-port number.
    pub const fn return_port(&self) -> u16 {
        self.return_port
    }

    /// Returns the uninterpreted query identifier.
    pub const fn query_id(&self) -> u64 {
        self.query_id
    }

    /// Recovers the original query allocation and numeric fields.
    pub fn into_parts(self) -> (String, u16, u64) {
        (self.query, self.return_port, self.query_id)
    }
}

/// A rejected owned query value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShortInfoQueryValueError {
    /// The query line was empty.
    EmptyQuery,
    /// The query line exceeded its selected maximum.
    QueryLimitExceeded {
        /// Selected maximum.
        expected: usize,
        /// Observed byte count.
        actual: usize,
    },
    /// A query byte was outside printable ASCII.
    InvalidQueryByte {
        /// Query-relative byte offset.
        offset: usize,
        /// Rejected byte.
        byte: u8,
    },
    /// The return port was zero.
    ZeroReturnPort,
}

fn validate_query(
    bytes: &[u8],
    limits: ShortInfoQueryWireLimits,
) -> Result<(), ShortInfoQueryValueError> {
    if bytes.is_empty() {
        return Err(ShortInfoQueryValueError::EmptyQuery);
    }
    if bytes.len() > limits.max_query_bytes {
        return Err(ShortInfoQueryValueError::QueryLimitExceeded {
            expected: limits.max_query_bytes,
            actual: bytes.len(),
        });
    }
    for (offset, byte) in bytes.iter().copied().enumerate() {
        if !(0x20..=0x7e).contains(&byte) {
            return Err(ShortInfoQueryValueError::InvalidQueryByte { offset, byte });
        }
    }
    Ok(())
}

/// An owned canonical short-info query payload.
#[derive(Debug, Eq, PartialEq)]
pub struct ShortInfoQueryWire {
    limits: ShortInfoQueryWireLimits,
    bytes: Vec<u8>,
}

impl ShortInfoQueryWire {
    /// Encodes exactly three CRLF-terminated lines with canonical unsigned decimals.
    pub fn encode(
        value: &ShortInfoQuery,
        limits: ShortInfoQueryWireLimits,
    ) -> Result<Self, ShortInfoQueryEncodeError> {
        validate_query(value.query.as_bytes(), limits).map_err(ShortInfoQueryEncodeError::Query)?;
        let port_len = decimal_len(u64::from(value.return_port));
        let id_len = decimal_len(value.query_id);
        let required = HEADER
            .len()
            .checked_add(value.query.len())
            .and_then(|v| v.checked_add(2 + port_len + 1 + id_len + 2))
            .ok_or(ShortInfoQueryEncodeError::LengthOverflow)?;
        if required > limits.max_payload_bytes {
            return Err(ShortInfoQueryEncodeError::PayloadLimitExceeded {
                expected: limits.max_payload_bytes,
                required,
            });
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(required).map_err(|_| {
            ShortInfoQueryEncodeError::AllocationFailed {
                requested: required,
            }
        })?;
        bytes.extend_from_slice(HEADER);
        bytes.extend_from_slice(value.query.as_bytes());
        bytes.extend_from_slice(b"\r\n");
        write_decimal(&mut bytes, u64::from(value.return_port));
        bytes.push(b' ');
        write_decimal(&mut bytes, value.query_id);
        bytes.extend_from_slice(b"\r\n");
        debug_assert_eq!(bytes.len(), required);
        Ok(Self { limits, bytes })
    }

    /// Returns the selected limits.
    pub const fn limits(&self) -> ShortInfoQueryWireLimits {
        self.limits
    }

    /// Returns the canonical bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the wrapper without copying its allocation.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// A failed canonical encoding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShortInfoQueryEncodeError {
    /// Query validation failed.
    Query(ShortInfoQueryValueError),
    /// Checked payload length overflowed `usize`.
    LengthOverflow,
    /// The complete payload exceeded its maximum.
    PayloadLimitExceeded {
        /// Selected maximum.
        expected: usize,
        /// Required canonical byte count.
        required: usize,
    },
    /// The exact output allocation could not be reserved.
    AllocationFailed {
        /// Exact requested capacity.
        requested: usize,
    },
}

/// A borrowed accepted view of one canonical short-info query payload.
#[derive(Debug, Eq, PartialEq)]
pub struct ParsedShortInfoQuery<'a> {
    source: &'a [u8],
    query_range: core::ops::Range<usize>,
    return_port: u16,
    query_id: u64,
}

impl<'a> ParsedShortInfoQuery<'a> {
    /// Parses only the exact canonical three-line candidate shape.
    pub fn parse(
        source: &'a [u8],
        limits: ShortInfoQueryWireLimits,
    ) -> Result<Self, ShortInfoQueryParseError> {
        if source.len() > limits.max_payload_bytes {
            return Err(ShortInfoQueryParseError::PayloadLimitExceeded {
                expected: limits.max_payload_bytes,
                actual: source.len(),
            });
        }
        if source.len() < HEADER.len() || &source[..HEADER.len()] != HEADER {
            let offset = first_difference(source, HEADER);
            return Err(ShortInfoQueryParseError::Header { offset });
        }
        let query_start = HEADER.len();
        let query_end = find_crlf(source, query_start)?;
        validate_query(&source[query_start..query_end], limits).map_err(|error| {
            ShortInfoQueryParseError::Query {
                offset: query_start,
                error,
            }
        })?;
        let port_start = query_end + 2;
        let space =
            find_byte(source, port_start, b' ').ok_or(ShortInfoQueryParseError::Truncated {
                offset: source.len(),
            })?;
        let (port, port_len) =
            parse_canonical_decimal(&source[port_start..space], u64::from(u16::MAX), port_start)?;
        if port == 0 {
            return Err(ShortInfoQueryParseError::ZeroReturnPort { offset: port_start });
        }
        let id_start = space + 1;
        let final_cr = find_crlf(source, id_start)?;
        let (query_id, _) =
            parse_canonical_decimal(&source[id_start..final_cr], u64::MAX, id_start)?;
        if final_cr + 2 != source.len() {
            return Err(ShortInfoQueryParseError::TrailingBytes {
                offset: final_cr + 2,
            });
        }
        debug_assert_eq!(port_len, space - port_start);
        Ok(Self {
            source,
            query_range: query_start..query_end,
            return_port: port as u16,
            query_id,
        })
    }

    /// Returns the unchanged source payload.
    pub fn source(&self) -> &'a [u8] {
        self.source
    }

    /// Returns the borrowed canonical query line.
    pub fn query(&self) -> &'a str {
        core::str::from_utf8(&self.source[self.query_range.clone()])
            .expect("validated printable ASCII")
    }

    /// Returns the uninterpreted nonzero return-port number.
    pub const fn return_port(&self) -> u16 {
        self.return_port
    }

    /// Returns the uninterpreted query identifier.
    pub const fn query_id(&self) -> u64 {
        self.query_id
    }
}

/// A rejected input payload with its first failing byte offset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShortInfoQueryParseError {
    /// The complete input exceeded its selected maximum.
    PayloadLimitExceeded {
        /// Selected maximum.
        expected: usize,
        /// Observed byte count.
        actual: usize,
    },
    /// The fixed header differed or was truncated.
    Header {
        /// First differing or missing byte.
        offset: usize,
    },
    /// A required delimiter or field was absent.
    Truncated {
        /// End offset where a delimiter or field was required.
        offset: usize,
    },
    /// A line ending was not exactly CRLF.
    InvalidLineEnding {
        /// First byte that cannot belong to the required CRLF delimiter.
        offset: usize,
    },
    /// Query validation failed; `offset` is the query's source start.
    Query {
        /// Source offset where the query begins.
        offset: usize,
        /// Unchanged query validation error.
        error: ShortInfoQueryValueError,
    },
    /// A decimal field was empty.
    EmptyDecimal {
        /// Decimal-field start offset.
        offset: usize,
    },
    /// A decimal field used a noncanonical spelling.
    NonCanonicalDecimal {
        /// First noncanonical byte offset.
        offset: usize,
    },
    /// A decimal field contained a nondigit.
    InvalidDecimalByte {
        /// Rejected source offset.
        offset: usize,
        /// Rejected byte.
        byte: u8,
    },
    /// A decimal field exceeded its target domain.
    DecimalOverflow {
        /// Source offset where the domain was exceeded.
        offset: usize,
    },
    /// The return port was zero.
    ZeroReturnPort {
        /// Return-port field start offset.
        offset: usize,
    },
    /// Bytes followed the one required final CRLF.
    TrailingBytes {
        /// First byte after the required final LF.
        offset: usize,
    },
}

impl fmt::Display for ShortInfoQueryParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid short-info query payload: {self:?}")
    }
}

fn first_difference(source: &[u8], expected: &[u8]) -> usize {
    source
        .iter()
        .zip(expected)
        .position(|(a, b)| a != b)
        .unwrap_or(source.len().min(expected.len()))
}

fn find_byte(source: &[u8], start: usize, needle: u8) -> Option<usize> {
    source
        .get(start..)?
        .iter()
        .position(|byte| *byte == needle)
        .map(|offset| start + offset)
}

fn find_crlf(source: &[u8], start: usize) -> Result<usize, ShortInfoQueryParseError> {
    let mut offset = start;
    while offset < source.len() {
        match source[offset] {
            b'\r' => {
                if source.get(offset + 1) == Some(&b'\n') {
                    return Ok(offset);
                }
                return Err(ShortInfoQueryParseError::InvalidLineEnding { offset });
            }
            b'\n' => return Err(ShortInfoQueryParseError::InvalidLineEnding { offset }),
            _ => offset += 1,
        }
    }
    Err(ShortInfoQueryParseError::Truncated {
        offset: source.len(),
    })
}

fn parse_canonical_decimal(
    bytes: &[u8],
    maximum: u64,
    start: usize,
) -> Result<(u64, usize), ShortInfoQueryParseError> {
    if bytes.is_empty() {
        return Err(ShortInfoQueryParseError::EmptyDecimal { offset: start });
    }
    if bytes.len() > 1 && bytes[0] == b'0' {
        return Err(ShortInfoQueryParseError::NonCanonicalDecimal { offset: start });
    }
    let mut value = 0u64;
    for (index, byte) in bytes.iter().copied().enumerate() {
        if !byte.is_ascii_digit() {
            return Err(ShortInfoQueryParseError::InvalidDecimalByte {
                offset: start + index,
                byte,
            });
        }
        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(u64::from(byte - b'0')))
            .ok_or(ShortInfoQueryParseError::DecimalOverflow {
                offset: start + index,
            })?;
        if value > maximum {
            return Err(ShortInfoQueryParseError::DecimalOverflow {
                offset: start + index,
            });
        }
    }
    Ok((value, bytes.len()))
}

fn decimal_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 10 {
        value /= 10;
        len += 1;
    }
    len
}

fn write_decimal(output: &mut Vec<u8>, mut value: u64) {
    let mut digits = [0u8; 20];
    let mut cursor = digits.len();
    loop {
        cursor -= 1;
        digits[cursor] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    output.extend_from_slice(&digits[cursor..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> ShortInfoQueryWireLimits {
        ShortInfoQueryWireLimits::new(64, 128).unwrap()
    }

    #[test]
    fn public_documentation_example_round_trips_without_source_copy() {
        let source = b"LSL:shortinfo\r\nsession_id='default'\r\n16577 11973266323178842010\r\n";
        assert_eq!(source.len(), 65);
        let parsed = ParsedShortInfoQuery::parse(source, limits()).unwrap();
        assert!(core::ptr::eq(parsed.source().as_ptr(), source.as_ptr()));
        assert_eq!(parsed.query(), "session_id='default'");
        assert_eq!(parsed.return_port(), 16577);
        assert_eq!(parsed.query_id(), 11973266323178842010);
    }

    #[test]
    fn canonical_encoder_preserves_query_allocation_on_recovery() {
        let query = String::from("name='alpha'");
        let pointer = query.as_ptr();
        let value = ShortInfoQuery::new(query, 1, 0, limits()).unwrap();
        let wire = ShortInfoQueryWire::encode(&value, limits()).unwrap();
        assert_eq!(wire.as_bytes(), b"LSL:shortinfo\r\nname='alpha'\r\n1 0\r\n");
        let (query, _, _) = value.into_parts();
        assert_eq!(query.as_ptr(), pointer);
    }

    #[test]
    fn damaged_truncated_oversized_and_noncanonical_inputs_reject() {
        assert_eq!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\r\nq\r\n01 2\r\n", limits()),
            Err(ShortInfoQueryParseError::NonCanonicalDecimal { offset: 18 })
        );
        assert!(matches!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\r\nq\r\n1", limits()),
            Err(ShortInfoQueryParseError::Truncated { .. })
        ));
        assert!(matches!(
            ParsedShortInfoQuery::parse(b"bad", limits()),
            Err(ShortInfoQueryParseError::Header { offset: 0 })
        ));
        let tight = ShortInfoQueryWireLimits::new(1, 4).unwrap();
        assert!(matches!(
            ParsedShortInfoQuery::parse(b"12345", tight),
            Err(ShortInfoQueryParseError::PayloadLimitExceeded { .. })
        ));
    }

    #[test]
    fn value_and_parser_reject_forbidden_query_bytes_and_zero_port() {
        assert_eq!(
            ShortInfoQuery::new("a\nb".into(), 1, 1, limits()),
            Err(ShortInfoQueryValueError::InvalidQueryByte {
                offset: 1,
                byte: b'\n'
            })
        );
        assert_eq!(
            ShortInfoQuery::new("q".into(), 0, 1, limits()),
            Err(ShortInfoQueryValueError::ZeroReturnPort)
        );
        assert!(matches!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\r\nq\r\n0 1\r\n", limits()),
            Err(ShortInfoQueryParseError::ZeroReturnPort { .. })
        ));
    }

    #[test]
    fn lslc_002d_rejects_lf_only_mixed_missing_and_extra_delimiters_at_first_offset() {
        assert_eq!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\nq\n1 1\n", limits()),
            Err(ShortInfoQueryParseError::Header { offset: 13 })
        );
        assert_eq!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\r\nq\n1 1\r\n", limits()),
            Err(ShortInfoQueryParseError::InvalidLineEnding { offset: 16 })
        );
        assert_eq!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\r\nq\r\r\n1 1\r\n", limits()),
            Err(ShortInfoQueryParseError::InvalidLineEnding { offset: 16 })
        );
        assert_eq!(
            ParsedShortInfoQuery::parse(b"LSL:shortinfo\r\nq\r\n1 1\r\n\n", limits()),
            Err(ShortInfoQueryParseError::TrailingBytes { offset: 23 })
        );
    }
}
