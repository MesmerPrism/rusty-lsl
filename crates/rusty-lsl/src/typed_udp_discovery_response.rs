// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded local projection from one accepted UDP response into typed observation state.

use crate::{
    ParsedShortInfoResponseEnvelope, ShortInfoResponseEnvelopeLimits,
    ShortInfoResponseEnvelopeParseError, StreamInfoObservedAdmissionLimits,
    TypedShortInfoResponseObservation, TypedShortInfoResponseObservationError,
    UdpDiscoveryResponse,
};
use std::net::SocketAddr;

/// One typed local observation paired with its unchanged observed UDP source.
#[derive(Debug, PartialEq)]
pub struct TypedUdpDiscoveryResponse {
    source: SocketAddr,
    observation: TypedShortInfoResponseObservation,
}

impl TypedUdpDiscoveryResponse {
    /// Borrows one already accepted response and delegates all existing parsing and admission.
    pub fn project(
        response: &UdpDiscoveryResponse,
        envelope_limits: ShortInfoResponseEnvelopeLimits,
        admission_limits: StreamInfoObservedAdmissionLimits,
    ) -> Result<Self, TypedUdpDiscoveryResponseError> {
        project_parts(
            response.source(),
            response.as_bytes(),
            envelope_limits,
            admission_limits,
        )
    }

    /// Returns the unchanged observed datagram source.
    pub const fn source(&self) -> SocketAddr {
        self.source
    }

    /// Returns the existing typed observation.
    pub const fn observation(&self) -> &TypedShortInfoResponseObservation {
        &self.observation
    }

    /// Recovers both accepted artifacts.
    pub fn into_parts(self) -> (SocketAddr, TypedShortInfoResponseObservation) {
        (self.source, self.observation)
    }
}

/// Stable delegated failure from the local projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypedUdpDiscoveryResponseError {
    /// Accepted response bytes were not UTF-8.
    InvalidUtf8 {
        /// First byte not known to be valid UTF-8.
        valid_up_to: usize,
    },
    /// Existing bounded envelope/document parsing rejected the response.
    Envelope(ShortInfoResponseEnvelopeParseError),
    /// Existing typed observation admission rejected the parsed response.
    Typed(TypedShortInfoResponseObservationError),
}

fn project_parts(
    source: SocketAddr,
    bytes: &[u8],
    envelope_limits: ShortInfoResponseEnvelopeLimits,
    admission_limits: StreamInfoObservedAdmissionLimits,
) -> Result<TypedUdpDiscoveryResponse, TypedUdpDiscoveryResponseError> {
    let text = std::str::from_utf8(bytes).map_err(|error| {
        TypedUdpDiscoveryResponseError::InvalidUtf8 {
            valid_up_to: error.valid_up_to(),
        }
    })?;
    let envelope = ParsedShortInfoResponseEnvelope::parse(text, envelope_limits)
        .map_err(TypedUdpDiscoveryResponseError::Envelope)?;
    let observation = TypedShortInfoResponseObservation::admit(envelope, admission_limits)
        .map_err(TypedUdpDiscoveryResponseError::Typed)?;
    Ok(TypedUdpDiscoveryResponse {
        source,
        observation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MetadataTreeLimits, StreamDescriptorLimits, StreamInfoObservedAdmissionError,
        StreamInfoVolatileFieldLimits,
    };

    fn document(channel_count: &str) -> String {
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
        let values = [
            "projected",
            "independent",
            channel_count,
            "float32",
            "fresh-source",
            "100.0000000000000",
            "110",
            "1",
            "fresh-uid",
            "fresh-session",
            "fresh-host",
            "203.0.113.9",
            "42001",
            "42002",
            "2001:db8::9",
            "42003",
            "42004",
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in names.into_iter().zip(values) {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn admission_limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(64, 64, 64, 4).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
        )
    }

    #[test]
    fn lslc_004u_projects_existing_typed_state_and_source() {
        let source: SocketAddr = "127.0.0.1:42000".parse().unwrap();
        let text = format!("19\r\n{}", document("1"));
        let limits = ShortInfoResponseEnvelopeLimits::new(text.len() - 4, text.len()).unwrap();
        let projected = project_parts(source, text.as_bytes(), limits, admission_limits()).unwrap();
        assert_eq!(projected.source(), source);
        assert_eq!(projected.observation().query_id(), 19);
        assert_eq!(
            projected
                .observation()
                .fields()
                .definition()
                .descriptor()
                .name(),
            "projected"
        );
        let (recovered_source, observation) = projected.into_parts();
        assert_eq!(recovered_source, source);
        assert_eq!(
            observation
                .fields()
                .definition()
                .descriptor()
                .channel_count(),
            1
        );
    }

    #[test]
    fn lslc_004u_delegates_utf8_envelope_and_typed_errors() {
        let source: SocketAddr = "127.0.0.1:42000".parse().unwrap();
        assert_eq!(
            project_parts(
                source,
                &[0xff],
                ShortInfoResponseEnvelopeLimits::new(1, 1).unwrap(),
                admission_limits(),
            ),
            Err(TypedUdpDiscoveryResponseError::InvalidUtf8 { valid_up_to: 0 })
        );

        let invalid_envelope = format!("19\n{}", document("1"));
        assert!(matches!(
            project_parts(
                source,
                invalid_envelope.as_bytes(),
                ShortInfoResponseEnvelopeLimits::new(
                    invalid_envelope.len() - 3,
                    invalid_envelope.len(),
                )
                .unwrap(),
                admission_limits(),
            ),
            Err(TypedUdpDiscoveryResponseError::Envelope(_))
        ));

        let invalid_typed = format!("19\r\n{}", document("01"));
        assert_eq!(
            project_parts(
                source,
                invalid_typed.as_bytes(),
                ShortInfoResponseEnvelopeLimits::new(invalid_typed.len() - 4, invalid_typed.len(),)
                    .unwrap(),
                admission_limits(),
            ),
            Err(TypedUdpDiscoveryResponseError::Typed(
                TypedShortInfoResponseObservationError::Admission(
                    StreamInfoObservedAdmissionError::InvalidChannelCount
                )
            ))
        );
    }

    #[test]
    fn lslc_005n_projection_preserves_utf8_position_and_exact_envelope_errors() {
        let source: SocketAddr = "127.0.0.1:42000".parse().unwrap();
        assert_eq!(
            project_parts(
                source,
                b"19\r\nvalid\xff",
                ShortInfoResponseEnvelopeLimits::new(6, 10).unwrap(),
                admission_limits(),
            ),
            Err(TypedUdpDiscoveryResponseError::InvalidUtf8 { valid_up_to: 9 })
        );

        let text = format!("19\r\n{}", document("1"));
        assert_eq!(
            project_parts(
                source,
                text.as_bytes(),
                ShortInfoResponseEnvelopeLimits::new(text.len() - 4, text.len() - 1).unwrap(),
                admission_limits(),
            ),
            Err(TypedUdpDiscoveryResponseError::Envelope(
                ShortInfoResponseEnvelopeParseError::EnvelopeLimitExceeded {
                    expected: text.len() - 1,
                    actual: text.len(),
                }
            ))
        );

        let malformed = format!("19\n{}", document("1"));
        assert_eq!(
            project_parts(
                source,
                malformed.as_bytes(),
                ShortInfoResponseEnvelopeLimits::new(malformed.len() - 3, malformed.len(),)
                    .unwrap(),
                admission_limits(),
            ),
            Err(TypedUdpDiscoveryResponseError::Envelope(
                ShortInfoResponseEnvelopeParseError::InvalidDelimiter { offset: 2 }
            ))
        );
    }

    #[test]
    fn lslc_005n_projection_repeatedly_accepts_exact_bounds_without_widening_source() {
        let source: SocketAddr = "[fe80::9%7]:42000".parse().unwrap();
        let text = format!("19\r\n{}", document("1"));
        let limits = ShortInfoResponseEnvelopeLimits::new(text.len() - 4, text.len()).unwrap();

        for _ in 0..12 {
            let projected =
                project_parts(source, text.as_bytes(), limits, admission_limits()).unwrap();
            let (recovered_source, observation) = projected.into_parts();
            assert_eq!(recovered_source, source);
            assert_eq!(observation.query_id(), 19);
            assert_eq!(
                observation
                    .fields()
                    .definition()
                    .descriptor()
                    .channel_count(),
                1
            );
        }
    }
}
