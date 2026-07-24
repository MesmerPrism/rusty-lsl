// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Consumer-facing data and protocol contracts.
//!
//! This is an additive role facade over the crate-root compatibility surface.
//! It defines no types and owns no implementation or runtime authority.

pub use crate::{
    BoundDescriptorSample, BoundTimestampedDescriptorChunk, BoundTimestampedDescriptorSample,
    BoundedMetadata, ChannelFormat, ChunkBound, ChunkError, ChunkLimits, ClockFilterSelection,
    ClockFilterSelectionError, ClockFilterSelectionLimit, ClockFilterSelectionLimitError,
    ClockOffset, ClockOffsetApplication, ClockOffsetApplicationError, ClockOffsetError,
    DerivedTimestamp, DerivedTimestampKind, DescriptorSampleBound, DescriptorSampleError,
    DescriptorSampleInput, DescriptorSampleLimits, DocumentedDiscoveryDestination,
    DocumentedDiscoveryQueryProposal, MetadataBound, MetadataDescription, MetadataError,
    MetadataField, MetadataLimits, MetadataNode, MetadataNodeInput, MetadataTextRole, MetadataTree,
    MetadataTreeBound, MetadataTreeError, MetadataTreeLimits, MetadataTreeTextRole,
    MetadataXmlProjectionError, MetadataXmlProjectionLimits, NominalSampleRate,
    NominalSampleRateError, NonFiniteTimestamp, ParsedShortInfoQuery,
    ParsedShortInfoResponseEnvelope, ParsedStreamInfoObservedDocument, RawClockExchange,
    RawClockExchangeFormulaError, RawClockExchangeFormulaResult, RawClockExchangeFormulaStage,
    RawClockExchangeInputError, RawClockExchangeTimestampRole, RawSourceTimestamp,
    RegularSampleRate, Sample, SampleBound, SampleError, SampleLimits, ShortInfoQuery,
    ShortInfoQueryEncodeError, ShortInfoQueryParseError, ShortInfoQueryValueError,
    ShortInfoQueryWire, ShortInfoQueryWireLimitError, ShortInfoQueryWireLimits,
    ShortInfoResponseEnvelope, ShortInfoResponseEnvelopeEncodeError,
    ShortInfoResponseEnvelopeLimitError, ShortInfoResponseEnvelopeLimits,
    ShortInfoResponseEnvelopeParseError, StreamDefinition, StreamDescriptor, StreamDescriptorBound,
    StreamDescriptorError, StreamDescriptorLimits, StreamDescriptorTextRole,
    StreamInfoDescriptionXml, StreamInfoDescriptionXmlError, StreamInfoObservedAdmissionError,
    StreamInfoObservedAdmissionLimits, StreamInfoObservedDocument, StreamInfoObservedDocumentError,
    StreamInfoObservedDocumentLimit, StreamInfoObservedDocumentParseError,
    StreamInfoObservedDocumentParseLimit, StreamInfoObservedFields, StreamInfoOrderedXml,
    StreamInfoOrderedXmlError, StreamInfoStaticFieldRole, StreamInfoStaticFields,
    StreamInfoStaticNumericSpellingError, StreamInfoStaticNumericSpellings, StreamInfoStaticXml,
    StreamInfoStaticXmlError, StreamInfoStaticXmlLimits, StreamInfoVolatileFieldClass,
    StreamInfoVolatileFieldError, StreamInfoVolatileFieldInput, StreamInfoVolatileFieldLimits,
    StreamInfoVolatileFieldRole, StreamInfoVolatileFields, StreamInfoVolatileProviderSnapshot,
    StreamInfoVolatileProviderSnapshotError, StreamInfoVolatileProviderSnapshotInput,
    StreamInfoVolatileProviderValue, StreamInfoVolatileXml, StreamInfoVolatileXmlError,
    StreamInfoVolatileXmlLimits, TimestampError, TimestampRole, TimestampedChunk,
    TimestampedDescriptorChunkError, TimestampedDescriptorChunkInput,
    TimestampedDescriptorSampleInput, TimestampedSample, TypedShortInfoResponseObservation,
    TypedShortInfoResponseObservationError, XmlCharacterData, XmlCharacterDataError,
    XmlCharacterDataLimit, XmlElementName, XmlElementNodeInput, XmlElementNodeValue,
    XmlElementSerialization, XmlElementSerializationError, XmlElementSerializationLimit,
    XmlElementTree, XmlElementTreeBound, XmlElementTreeError, XmlElementTreeLimits, XmlLeafElement,
    XmlNameError, XmlNameLimit, XmlText, XmlTextError, XmlTextLimit,
    DOCUMENTED_DEFAULT_DISCOVERY_PORT, DOCUMENTED_DISCOVERY_DESTINATIONS,
};

pub use crate::project_metadata_tree_to_xml_element_tree;
