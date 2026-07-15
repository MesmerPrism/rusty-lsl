// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Dependency-free local contracts for Rusty LSL.
//!
//! This crate currently implements only bounded metadata, sample-shape,
//! timestamped-chunk, core stream-descriptor, and flat metadata-tree
//! construction, descriptor/sample shape and format binding, and timestamped
//! descriptor/sample, non-empty descriptor/chunk, and stream-definition
//! composition, a borrowed static stream-info semantic projection, plus
//! a bounded static numeric lexical projection for its channel count and
//! nominal sample rate, plus
//! bounded XML legal-text, element-name, and character-data
//! representation contracts, leaf-only XML value composition, and a bounded
//! parent-before-child XML container/leaf hierarchy, plus bounded borrowed
//! element-tree string serialization, bounded opaque volatile stream-info
//! accepted data, and its bounded local XML element projection.
//! A separate one-shot provider-input snapshot contract admits complete,
//! disjoint caller-supplied volatile lanes before producing that accepted data.
//! Accepted static-plus-description and volatile trees can also be consumed
//! into one bounded local static, volatile, then `desc` element hierarchy.
//! A separate bounded borrowed projection can apply the accepted observed
//! stream-info document envelope without changing compact serialization.
//! One separate bounded local contract encodes and parses only the canonical
//! three-line protocol-110 short-info query payload candidate.
//! Another bounded local contract encodes and parses only the
//! observed short-info response envelope around an accepted document body.
//! A closed allocation-free data contract also exposes only the documented
//! default discovery port and exact displayed destination spellings.
//! One separately locked and explicitly invoked synchronous runtime call owns
//! only bounded caller-configured UDP discovery with loopback evidence.
//! It does not implement endpoint selection, official interoperability,
//! clocks, inlet, outlet, FFI, or Morphospace authority behavior.

mod bounded_fixed_record_transport;
mod bounded_sample_queue_runtime;
mod clock_filter_selection;
mod clock_offset_application;
pub mod contract;
mod descriptor_sample;
mod documented_discovery_destination;
mod documented_discovery_query_proposal;
mod finite_sample_recovery_runtime;
mod fixed_width_numeric_sample_runtime;
mod integrated_clock_correction_runtime;
mod metadata;
mod metadata_tree;
mod metadata_xml_projection;
mod raw_clock_exchange;
pub mod runtime;
mod runtime_activation;
mod sample;
mod short_info_discovery_responder_runtime;
mod short_info_query_wire;
mod short_info_response_envelope;
mod stream_definition;
mod stream_descriptor;
mod stream_handshake;
mod stream_info_description_xml;
mod stream_info_implementation_version_provider;
mod stream_info_observed_document;
mod stream_info_observed_document_admission;
mod stream_info_observed_document_parser;
mod stream_info_ordered_xml;
mod stream_info_runtime_provider;
mod stream_info_static_fields;
mod stream_info_static_numeric_spellings;
mod stream_info_static_xml;
mod stream_info_three_owner_observed_document;
mod stream_info_three_owner_snapshot;
mod stream_info_transport_provider;
mod stream_info_volatile_fields;
mod stream_info_volatile_snapshot;
mod stream_info_volatile_xml;
mod timestamped;
mod timestamped_descriptor_chunk;
mod timestamped_descriptor_sample;
mod timestamped_float32_sample_runtime;
mod typed_short_info_response_observation;
mod udp_discovery;
mod xml_character_data;
mod xml_element_serialization;
mod xml_element_tree;
mod xml_leaf_element;
mod xml_value;

pub use bounded_sample_queue_runtime::{
    BoundedSampleQueue, BoundedSampleQueueActivation, BoundedSampleQueueActivationError,
    BoundedSampleQueueCloseError, BoundedSampleQueueCreateError, BoundedSampleQueuePopError,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, BoundedSampleQueueWaitError,
    BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER, BOUNDED_SAMPLE_QUEUE_FEATURE_ID,
};
pub use clock_filter_selection::{
    ClockFilterSelection, ClockFilterSelectionError, ClockFilterSelectionLimit,
    ClockFilterSelectionLimitError,
};
pub use clock_offset_application::{
    ClockOffset, ClockOffsetApplication, ClockOffsetApplicationError, ClockOffsetError,
};
pub use descriptor_sample::{
    BoundDescriptorSample, DescriptorSampleBound, DescriptorSampleError, DescriptorSampleInput,
    DescriptorSampleLimits,
};
pub use documented_discovery_destination::{
    DocumentedDiscoveryDestination, DOCUMENTED_DEFAULT_DISCOVERY_PORT,
    DOCUMENTED_DISCOVERY_DESTINATIONS,
};
pub use documented_discovery_query_proposal::DocumentedDiscoveryQueryProposal;
pub use finite_sample_recovery_runtime::{
    run_finite_sample_recovery, FiniteSampleRecoveryActivation,
    FiniteSampleRecoveryActivationError, FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryPolicyError, FiniteSampleRecoveryState,
    RecoveryAttemptFailure, RecoveryFailureClass, FINITE_SAMPLE_RECOVERY_EFFECTIVE_MARKER,
    FINITE_SAMPLE_RECOVERY_FEATURE_ID,
};
pub use fixed_width_numeric_sample_runtime::{
    run_fixed_width_numeric_inlet, run_fixed_width_numeric_outlet, FixedWidthNumericRecord,
    FixedWidthNumericSampleActivation, FixedWidthNumericSampleActivationError,
    FixedWidthNumericSampleError, FixedWidthNumericSampleLimitError, FixedWidthNumericSampleLimits,
    FixedWidthNumericValue, FIXED_WIDTH_NUMERIC_SAMPLE_EFFECTIVE_MARKER,
    FIXED_WIDTH_NUMERIC_SAMPLE_FEATURE_ID,
};
pub use integrated_clock_correction_runtime::{
    run_integrated_clock_correction, ClockSource, IntegratedClockCorrection,
    IntegratedClockCorrectionActivation, IntegratedClockCorrectionActivationError,
    IntegratedClockCorrectionConfig, IntegratedClockCorrectionConfigError,
    IntegratedClockCorrectionError, INTEGRATED_CLOCK_CORRECTION_EFFECTIVE_MARKER,
    INTEGRATED_CLOCK_CORRECTION_FEATURE_ID,
};
pub use metadata::{
    BoundedMetadata, MetadataBound, MetadataDescription, MetadataError, MetadataField,
    MetadataLimits, MetadataTextRole,
};
pub use metadata_tree::{
    MetadataNode, MetadataNodeInput, MetadataTree, MetadataTreeBound, MetadataTreeError,
    MetadataTreeLimits, MetadataTreeTextRole,
};
pub use metadata_xml_projection::{
    project_metadata_tree_to_xml_element_tree, MetadataXmlProjectionError,
    MetadataXmlProjectionLimits,
};
pub use raw_clock_exchange::{
    RawClockExchange, RawClockExchangeFormulaError, RawClockExchangeFormulaResult,
    RawClockExchangeFormulaStage, RawClockExchangeInputError, RawClockExchangeTimestampRole,
};
pub use runtime_activation::{
    admit_runtime_activation, RuntimeActivationAdmission, RuntimeActivationError,
    RuntimeActivationOutcome, RuntimeActivationReceipt, RuntimeActivationSelection, RuntimeModule,
    RuntimeModuleCapability, ACCEPTED_FEATURE_LOCK_FINGERPRINT, ACCEPTED_FEATURE_LOCK_REVISION,
};
pub use sample::{Sample, SampleBound, SampleError, SampleLimits};
pub use short_info_discovery_responder_runtime::{
    run_short_info_responder, ShortInfoResponderActivation, ShortInfoResponderActivationError,
    ShortInfoResponderError, ShortInfoResponderLimitError, ShortInfoResponderLimits,
    ShortInfoResponderRun, ShortInfoResponderTermination, SHORT_INFO_RESPONDER_EFFECTIVE_MARKER,
    SHORT_INFO_RESPONDER_FEATURE_ID,
};
pub use short_info_query_wire::{
    ParsedShortInfoQuery, ShortInfoQuery, ShortInfoQueryEncodeError, ShortInfoQueryParseError,
    ShortInfoQueryValueError, ShortInfoQueryWire, ShortInfoQueryWireLimitError,
    ShortInfoQueryWireLimits,
};
pub use short_info_response_envelope::{
    ParsedShortInfoResponseEnvelope, ShortInfoResponseEnvelope,
    ShortInfoResponseEnvelopeEncodeError, ShortInfoResponseEnvelopeLimitError,
    ShortInfoResponseEnvelopeLimits, ShortInfoResponseEnvelopeParseError,
};
pub use stream_definition::StreamDefinition;
pub use stream_descriptor::{
    ChannelFormat, InvalidRegularSampleRate, NominalSampleRate, NominalSampleRateError,
    RegularSampleRate, StreamDescriptor, StreamDescriptorBound, StreamDescriptorError,
    StreamDescriptorLimits, StreamDescriptorTextRole,
};
pub use stream_handshake::{
    run_stream_inlet_handshake, run_stream_outlet_handshake, StreamHandshakeActivation,
    StreamHandshakeActivationError, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeIdentityError, StreamHandshakeIdentityRole, StreamHandshakeLimitError,
    StreamHandshakeLimits, StreamInletHandshake, StreamOutletHandshake,
    STREAM_HANDSHAKE_EFFECTIVE_MARKER, STREAM_HANDSHAKE_FEATURE_ID,
};
pub use stream_info_description_xml::{StreamInfoDescriptionXml, StreamInfoDescriptionXmlError};
pub use stream_info_implementation_version_provider::{
    StreamInfoImplementationVersionAcquisition, StreamInfoImplementationVersionAcquisitionError,
    StreamInfoImplementationVersionEvidenceError, StreamInfoImplementationVersionEvidenceLimit,
    StreamInfoImplementationVersionProvider, StreamInfoImplementationVersionProviderOutput,
    StreamInfoImplementationVersionWitness,
};
pub use stream_info_observed_document::{
    StreamInfoObservedDocument, StreamInfoObservedDocumentError, StreamInfoObservedDocumentLimit,
};
pub use stream_info_observed_document_admission::{
    StreamInfoObservedAdmissionError, StreamInfoObservedAdmissionLimits, StreamInfoObservedFields,
};
pub use stream_info_observed_document_parser::{
    ParsedStreamInfoObservedDocument, StreamInfoObservedDocumentParseError,
    StreamInfoObservedDocumentParseLimit,
};
pub use stream_info_ordered_xml::{StreamInfoOrderedXml, StreamInfoOrderedXmlError};
pub use stream_info_runtime_provider::{
    StreamInfoRuntimeAcquisition, StreamInfoRuntimeAcquisitionError,
    StreamInfoRuntimeEvidenceError, StreamInfoRuntimeEvidenceLimit, StreamInfoRuntimeProvider,
    StreamInfoRuntimeProviderOutput, StreamInfoRuntimeValues, StreamInfoRuntimeWitness,
};
pub use stream_info_static_fields::{StreamInfoStaticFieldRole, StreamInfoStaticFields};
pub use stream_info_static_numeric_spellings::{
    StreamInfoStaticNumericSpellingError, StreamInfoStaticNumericSpellings,
};
pub use stream_info_static_xml::{
    StreamInfoStaticXml, StreamInfoStaticXmlError, StreamInfoStaticXmlLimits,
};
pub use stream_info_three_owner_observed_document::{
    StreamInfoThreeOwnerObservedDocument, StreamInfoThreeOwnerObservedDocumentError,
    StreamInfoThreeOwnerObservedDocumentLimits,
};
pub use stream_info_three_owner_snapshot::{
    StreamInfoThreeOwnerEvidence, StreamInfoThreeOwnerSnapshot,
};
pub use stream_info_transport_provider::{
    StreamInfoTransportAcquisition, StreamInfoTransportAcquisitionError,
    StreamInfoTransportEvidenceError, StreamInfoTransportEvidenceLimit,
    StreamInfoTransportProvider, StreamInfoTransportProviderOutput, StreamInfoTransportValues,
    StreamInfoTransportWitness,
};
pub use stream_info_volatile_fields::{
    StreamInfoVolatileFieldClass, StreamInfoVolatileFieldError, StreamInfoVolatileFieldInput,
    StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole, StreamInfoVolatileFields,
};
pub use stream_info_volatile_snapshot::{
    StreamInfoVolatileProviderSnapshot, StreamInfoVolatileProviderSnapshotError,
    StreamInfoVolatileProviderSnapshotInput, StreamInfoVolatileProviderValue,
};
pub use stream_info_volatile_xml::{
    StreamInfoVolatileXml, StreamInfoVolatileXmlError, StreamInfoVolatileXmlLimits,
};
pub use timestamped::{
    ChunkBound, ChunkError, ChunkLimits, DerivedTimestamp, DerivedTimestampKind,
    NonFiniteTimestamp, RawSourceTimestamp, TimestampError, TimestampRole, TimestampedChunk,
    TimestampedSample,
};
pub use timestamped_descriptor_chunk::{
    BoundTimestampedDescriptorChunk, TimestampedDescriptorChunkError,
    TimestampedDescriptorChunkInput,
};
pub use timestamped_descriptor_sample::{
    BoundTimestampedDescriptorSample, TimestampedDescriptorSampleInput,
};
pub use timestamped_float32_sample_runtime::{
    run_timestamped_float32_inlet, run_timestamped_float32_outlet,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleActivationError,
    TimestampedFloat32SampleError, TimestampedFloat32SampleLimitError,
    TimestampedFloat32SampleLimits, TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER,
    TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID,
};
pub use typed_short_info_response_observation::{
    TypedShortInfoResponseObservation, TypedShortInfoResponseObservationError,
};
pub use udp_discovery::{
    run_udp_discovery, UdpDiscoveryActivation, UdpDiscoveryActivationError, UdpDiscoveryConfig,
    UdpDiscoveryError, UdpDiscoveryLimitError, UdpDiscoveryLimits, UdpDiscoveryResponse,
    UdpDiscoveryRun, UdpDiscoveryTermination, UDP_DISCOVERY_EFFECTIVE_MARKER,
    UDP_DISCOVERY_FEATURE_ID,
};
pub use xml_character_data::{XmlCharacterData, XmlCharacterDataError, XmlCharacterDataLimit};
pub use xml_element_serialization::{
    XmlElementSerialization, XmlElementSerializationError, XmlElementSerializationLimit,
};
pub use xml_element_tree::{
    XmlElementNodeInput, XmlElementNodeValue, XmlElementTree, XmlElementTreeBound,
    XmlElementTreeError, XmlElementTreeLimits,
};
pub use xml_leaf_element::XmlLeafElement;
pub use xml_value::{
    XmlElementName, XmlNameError, XmlNameLimit, XmlText, XmlTextError, XmlTextLimit,
};

/// The implementation state exposed by the crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImplementationStatus {
    /// Bounded local contracts plus one explicitly activated UDP discovery call exist.
    FiniteSampleRecoveryRuntime,
}

/// A stable declaration of one side of the repository ownership boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OwnershipDeclaration {
    /// Capabilities and evidence owned by Rusty LSL.
    pub owns: &'static [&'static str],
    /// Authorities and adapters explicitly owned elsewhere.
    pub does_not_own: &'static [&'static str],
}

/// Returns the current implementation status.
#[must_use]
pub const fn implementation_status() -> ImplementationStatus {
    ImplementationStatus::FiniteSampleRecoveryRuntime
}

/// Returns the repository's current ownership declaration.
#[must_use]
pub const fn ownership_declaration() -> OwnershipDeclaration {
    OwnershipDeclaration {
        owns: &[
            "bounded local metadata construction",
            "bounded local sample-shape construction",
            "bounded local timestamped-sample and chunk construction",
            "bounded local core stream-descriptor construction",
            "bounded local flat metadata-tree construction",
            "bounded local descriptor/sample binding",
            "bounded local timestamped descriptor/sample composition",
            "bounded local non-empty timestamped descriptor/chunk composition",
            "bounded local stream-definition composition",
            "borrowed static stream-info semantic projection",
            "bounded static stream-info numeric lexical projection",
            "bounded local XML legal-text and element-name values",
            "bounded local XML character-data representation",
            "bounded local XML leaf-element composition",
            "bounded local XML container/leaf hierarchy",
            "bounded local metadata-to-XML-element-tree projection",
            "bounded local XML element-tree serialization",
            "bounded volatile stream-info accepted data",
            "bounded volatile stream-info XML element composition",
            "bounded local short-info query byte-shape contract",
            "bounded local short-info response-envelope contract",
            "documented discovery-destination data contract",
            "inert documented discovery-query proposal composition",
            "finite raw clock-exchange formula contract",
            "bounded minimum-RTT selection contract",
            "explicit finite clock-offset application contract",
            "bounded caller-configured UDP discovery runtime",
            "bounded caller-configured TCP stream-handshake runtime",
            "bounded one-record timestamped float32 sample runtime",
            "bounded integrated clock-correction runtime",
            "bounded caller-owned sample queue runtime",
            "finite caller-invoked sample recovery runtime",
            "future backend-neutral Rust LSL API",
            "compatibility evidence",
            "typed observations and proposals for downstream adapters",
        ],
        does_not_own: &[
            "Manifold authority",
            "Morphospace-native sample transport",
            "topology, identity, permission, or platform lifecycle",
            "Quest or Hostess adapters",
            "commands derived from inbound samples",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{implementation_status, ownership_declaration, ImplementationStatus};

    #[test]
    fn status_names_only_the_implemented_local_contracts() {
        assert_eq!(
            implementation_status(),
            ImplementationStatus::FiniteSampleRecoveryRuntime
        );
    }

    #[test]
    fn authority_remains_outside_the_repository() {
        let declaration = ownership_declaration();
        assert!(declaration.does_not_own.contains(&"Manifold authority"));
        assert!(declaration
            .does_not_own
            .contains(&"commands derived from inbound samples"));
        assert!(declaration
            .owns
            .contains(&"bounded local short-info response-envelope contract"));
        assert!(declaration
            .owns
            .contains(&"documented discovery-destination data contract"));
        assert!(declaration
            .owns
            .contains(&"inert documented discovery-query proposal composition"));
        assert!(declaration
            .owns
            .contains(&"finite raw clock-exchange formula contract"));
        assert!(declaration
            .owns
            .contains(&"bounded minimum-RTT selection contract"));
        assert!(declaration
            .owns
            .contains(&"bounded caller-configured UDP discovery runtime"));
        assert!(declaration
            .owns
            .contains(&"explicit finite clock-offset application contract"));
        assert!(declaration
            .owns
            .contains(&"bounded caller-configured TCP stream-handshake runtime"));
        assert!(declaration
            .owns
            .contains(&"bounded one-record timestamped float32 sample runtime"));
        assert!(declaration
            .owns
            .contains(&"bounded integrated clock-correction runtime"));
        assert!(declaration
            .owns
            .contains(&"bounded caller-owned sample queue runtime"));
        assert!(declaration
            .owns
            .contains(&"finite caller-invoked sample recovery runtime"));
    }
}
