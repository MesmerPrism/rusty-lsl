// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{
    ChannelFormat, MetadataNodeInput, MetadataTree, MetadataTreeError, MetadataTreeLimits,
    NominalSampleRate, ParsedStreamInfoObservedDocument, StreamDefinition, StreamDescriptor,
    StreamDescriptorError, StreamDescriptorLimits, StreamInfoVolatileFieldError,
    StreamInfoVolatileFieldInput, StreamInfoVolatileFieldLimits, StreamInfoVolatileFields,
};

/// Existing contract limits applied by observed-document typed admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoObservedAdmissionLimits {
    descriptor: StreamDescriptorLimits,
    metadata: MetadataTreeLimits,
    volatile: StreamInfoVolatileFieldLimits,
}

impl StreamInfoObservedAdmissionLimits {
    /// Groups already validated descriptor, metadata, and volatile-field limits.
    #[must_use]
    pub const fn new(
        descriptor: StreamDescriptorLimits,
        metadata: MetadataTreeLimits,
        volatile: StreamInfoVolatileFieldLimits,
    ) -> Self {
        Self {
            descriptor,
            metadata,
            volatile,
        }
    }
}

/// A typed local observation admitted from one canonical parsed document.
#[derive(Debug, PartialEq)]
pub struct StreamInfoObservedFields {
    definition: StreamDefinition,
    volatile: StreamInfoVolatileFields,
}

impl StreamInfoObservedFields {
    /// Decodes the parsed character data and delegates to existing typed contracts.
    pub fn admit(
        limits: StreamInfoObservedAdmissionLimits,
        parsed: ParsedStreamInfoObservedDocument<'_>,
    ) -> Result<Self, StreamInfoObservedAdmissionError> {
        let mut values: [String; 17] = core::array::from_fn(|_| String::new());
        for (index, value) in values.iter_mut().enumerate() {
            *value = decode(
                index,
                parsed
                    .value(index)
                    .expect("LSLC-002A always indexes 17 fields"),
            )?;
        }
        let [name, content_type, channel_count, channel_format, source_id, nominal_srate, version, created_at, uid, session_id, hostname, v4address, v4data_port, v4service_port, v6address, v6data_port, v6service_port] =
            values;

        let channel_count = parse_channel_count(&channel_count)?;
        let channel_format = parse_channel_format(&channel_format)?;
        let nominal_sample_rate = parse_nominal_rate(&nominal_srate)?;
        let descriptor = StreamDescriptor::new(
            limits.descriptor,
            name,
            Some(content_type),
            Some(source_id),
            channel_count,
            nominal_sample_rate,
            channel_format,
        )
        .map_err(StreamInfoObservedAdmissionError::Descriptor)?;
        let metadata = MetadataTree::new(
            limits.metadata,
            vec![MetadataNodeInput::new(None, "desc".to_owned(), None)],
        )
        .map_err(StreamInfoObservedAdmissionError::Metadata)?;
        let volatile = StreamInfoVolatileFields::new(
            limits.volatile,
            StreamInfoVolatileFieldInput::new(
                version,
                created_at,
                uid,
                session_id,
                hostname,
                v4address,
                v4data_port,
                v4service_port,
                v6address,
                v6data_port,
                v6service_port,
            ),
        )
        .map_err(StreamInfoObservedAdmissionError::Volatile)?;
        Ok(Self {
            definition: StreamDefinition::new(descriptor, metadata),
            volatile,
        })
    }

    /// Returns the admitted static definition and empty description tree.
    #[must_use]
    pub const fn definition(&self) -> &StreamDefinition {
        &self.definition
    }

    /// Returns the admitted opaque volatile observation.
    #[must_use]
    pub const fn volatile_fields(&self) -> &StreamInfoVolatileFields {
        &self.volatile
    }

    /// Returns both owned typed observations without reallocating their values.
    #[must_use]
    pub fn into_parts(self) -> (StreamDefinition, StreamInfoVolatileFields) {
        (self.definition, self.volatile)
    }
}

/// Deterministic rejection from typed observed-document admission.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoObservedAdmissionError {
    /// A decoded field allocation could not be reserved.
    AllocationFailed {
        /// Fixed zero-based field role whose decoded allocation failed.
        field_index: usize,
        /// Exact represented byte capacity requested.
        requested: usize,
    },
    /// The channel count was not the canonical accepted decimal spelling.
    InvalidChannelCount,
    /// The channel format was not one of the seven accepted names.
    InvalidChannelFormat,
    /// The nominal rate was not one of the six accepted LSLC-001L spellings.
    InvalidNominalSampleRate,
    /// Existing descriptor admission rejected the decoded static fields.
    Descriptor(StreamDescriptorError),
    /// Existing metadata admission rejected the fixed empty desc tree.
    Metadata(MetadataTreeError),
    /// Existing volatile admission rejected a decoded opaque field.
    Volatile(StreamInfoVolatileFieldError),
}

impl fmt::Display for StreamInfoObservedAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "observed stream-info typed admission rejected input: {self:?}"
        )
    }
}
impl std::error::Error for StreamInfoObservedAdmissionError {}

fn decode(
    field_index: usize,
    represented: &str,
) -> Result<String, StreamInfoObservedAdmissionError> {
    let mut decoded = String::new();
    decoded.try_reserve_exact(represented.len()).map_err(|_| {
        StreamInfoObservedAdmissionError::AllocationFailed {
            field_index,
            requested: represented.len(),
        }
    })?;
    let mut rest = represented;
    while let Some(offset) = rest.find('&') {
        decoded.push_str(&rest[..offset]);
        rest = &rest[offset..];
        for (entity, character) in [("&amp;", '&'), ("&lt;", '<'), ("&gt;", '>')] {
            if rest.starts_with(entity) {
                decoded.push(character);
                rest = &rest[entity.len()..];
                break;
            }
        }
    }
    decoded.push_str(rest);
    Ok(decoded)
}

fn parse_channel_count(text: &str) -> Result<usize, StreamInfoObservedAdmissionError> {
    let value = text
        .parse::<usize>()
        .map_err(|_| StreamInfoObservedAdmissionError::InvalidChannelCount)?;
    if value.to_string() == text {
        Ok(value)
    } else {
        Err(StreamInfoObservedAdmissionError::InvalidChannelCount)
    }
}

fn parse_channel_format(text: &str) -> Result<ChannelFormat, StreamInfoObservedAdmissionError> {
    match text {
        "float32" => Ok(ChannelFormat::Float32),
        "double64" => Ok(ChannelFormat::Double64),
        "string" => Ok(ChannelFormat::String),
        "int32" => Ok(ChannelFormat::Int32),
        "int16" => Ok(ChannelFormat::Int16),
        "int8" => Ok(ChannelFormat::Int8),
        "int64" => Ok(ChannelFormat::Int64),
        _ => Err(StreamInfoObservedAdmissionError::InvalidChannelFormat),
    }
}

fn parse_nominal_rate(text: &str) -> Result<NominalSampleRate, StreamInfoObservedAdmissionError> {
    match text {
        "0.000000000000000" => Ok(NominalSampleRate::irregular()),
        "100.0000000000000" => NominalSampleRate::regular_hz(100.0),
        "59.94000000000000" => NominalSampleRate::regular_hz(59.94),
        "1.000000000000000" => NominalSampleRate::regular_hz(1.0),
        "256.5000000000000" => NominalSampleRate::regular_hz(256.5),
        "1000000.250000000" => NominalSampleRate::regular_hz(1_000_000.25),
        _ => return Err(StreamInfoObservedAdmissionError::InvalidNominalSampleRate),
    }
    .map_err(|_| StreamInfoObservedAdmissionError::InvalidNominalSampleRate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StreamInfoObservedDocumentParseLimit, StreamInfoVolatileFieldRole};

    fn document(channel_count: &str, format: &str, rate: &str) -> String {
        let values = [
            "name&amp;&lt;",
            "type",
            channel_count,
            format,
            "source",
            rate,
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
        let mut result = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in names.into_iter().zip(values) {
            result.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        result.push_str("\t<desc />\n</info>\n");
        result
    }
    fn limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(32, 32, 32, 64).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 4, 1).unwrap(),
            StreamInfoVolatileFieldLimits::new(32, 32, 32).unwrap(),
        )
    }
    fn parse(source: &str) -> ParsedStreamInfoObservedDocument<'_> {
        ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(source.len()).unwrap(),
            source,
        )
        .unwrap()
    }
    #[test]
    fn lslc_002b_closes_parsed_document_into_existing_typed_contracts() {
        let source = document("2", "float32", "100.0000000000000");
        let admitted = StreamInfoObservedFields::admit(limits(), parse(&source)).unwrap();
        assert_eq!(admitted.definition().descriptor().name(), "name&<");
        assert_eq!(admitted.definition().descriptor().channel_count(), 2);
        assert_eq!(
            admitted.definition().extended_metadata().nodes()[0].name(),
            "desc"
        );
        assert_eq!(
            admitted
                .volatile_fields()
                .field(StreamInfoVolatileFieldRole::Version),
            "110"
        );
    }
    #[test]
    fn lslc_002b_rejects_noncanonical_static_lexemes() {
        for source in [
            document("02", "float32", "100.0000000000000"),
            document("2", "FLOAT32", "100.0000000000000"),
            document("2", "float32", "100.0"),
        ] {
            assert!(StreamInfoObservedFields::admit(limits(), parse(&source)).is_err());
        }
    }
    #[test]
    fn lslc_002b_all_closed_formats_and_rates_admit() {
        for format in [
            "float32", "double64", "string", "int32", "int16", "int8", "int64",
        ] {
            for rate in [
                "0.000000000000000",
                "100.0000000000000",
                "59.94000000000000",
                "1.000000000000000",
                "256.5000000000000",
                "1000000.250000000",
            ] {
                let source = document("1", format, rate);
                StreamInfoObservedFields::admit(limits(), parse(&source)).unwrap();
            }
        }
    }
}
