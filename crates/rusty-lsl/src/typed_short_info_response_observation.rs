// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit composition of an accepted response envelope into typed observed fields.

use crate::{
    ParsedShortInfoResponseEnvelope, StreamInfoObservedAdmissionError,
    StreamInfoObservedAdmissionLimits, StreamInfoObservedFields,
};

/// A typed local short-info response observation.
#[derive(Debug, PartialEq)]
pub struct TypedShortInfoResponseObservation {
    query_id: u64,
    fields: StreamInfoObservedFields,
}

impl TypedShortInfoResponseObservation {
    /// Consumes accepted envelope state and delegates its body to LSLC-002B.
    pub fn admit(
        envelope: ParsedShortInfoResponseEnvelope<'_>,
        limits: StreamInfoObservedAdmissionLimits,
    ) -> Result<Self, TypedShortInfoResponseObservationError> {
        let (query_id, body) = envelope.into_parts();
        let fields = StreamInfoObservedFields::admit(limits, body)
            .map_err(TypedShortInfoResponseObservationError::Admission)?;
        Ok(Self { query_id, fields })
    }

    /// Returns the unchanged uninterpreted identifier.
    pub const fn query_id(&self) -> u64 {
        self.query_id
    }

    /// Returns the existing typed observed fields.
    pub const fn fields(&self) -> &StreamInfoObservedFields {
        &self.fields
    }

    /// Recovers both accepted artifacts without reallocating their values.
    pub fn into_parts(self) -> (u64, StreamInfoObservedFields) {
        (self.query_id, self.fields)
    }
}

/// Typed response-observation rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypedShortInfoResponseObservationError {
    /// Existing LSLC-002B admission rejected the unchanged parsed body.
    Admission(StreamInfoObservedAdmissionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MetadataTreeLimits, ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits,
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
            "name",
            "type",
            channel_count,
            "float32",
            "source",
            "100.0000000000000",
            "110",
            "1",
            "uid",
            "session",
            "host",
            "127.0.0.1",
            "1",
            "2",
            "::1",
            "3",
            "4",
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in names.into_iter().zip(values) {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(32, 32, 32, 64).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 4, 1).unwrap(),
            StreamInfoVolatileFieldLimits::new(32, 32, 32).unwrap(),
        )
    }

    fn envelope(source: &str) -> ParsedShortInfoResponseEnvelope<'_> {
        ParsedShortInfoResponseEnvelope::parse(
            source,
            ShortInfoResponseEnvelopeLimits::new(source.len(), source.len()).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn lslc_002h_identifier_boundaries_remain_uninterpreted_beside_typed_fields() {
        for query_id in [0, u64::MAX] {
            let source = format!("{query_id}\r\n{}", document("2"));
            let admitted =
                TypedShortInfoResponseObservation::admit(envelope(&source), limits()).unwrap();
            assert_eq!(admitted.query_id(), query_id);
            assert_eq!(
                admitted.fields().definition().descriptor().channel_count(),
                2
            );
        }
    }

    #[test]
    fn lslc_002h_admission_error_is_delegated_unchanged() {
        let source = format!("7\r\n{}", document("02"));
        assert_eq!(
            TypedShortInfoResponseObservation::admit(envelope(&source), limits()),
            Err(TypedShortInfoResponseObservationError::Admission(
                StreamInfoObservedAdmissionError::InvalidChannelCount
            ))
        );
    }
}
