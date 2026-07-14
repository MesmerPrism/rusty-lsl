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
//! It does not
//! implement or claim LSL protocol,
//! runtime, wire, discovery, clock, inlet, outlet, FFI, or Morphospace adapter
//! behavior.

mod descriptor_sample;
mod metadata;
mod metadata_tree;
mod metadata_xml_projection;
mod sample;
mod stream_definition;
mod stream_descriptor;
mod stream_info_description_xml;
mod stream_info_implementation_version_provider;
mod stream_info_observed_document;
mod stream_info_ordered_xml;
mod stream_info_runtime_provider;
mod stream_info_static_fields;
mod stream_info_static_numeric_spellings;
mod stream_info_static_xml;
mod stream_info_three_owner_snapshot;
mod stream_info_transport_provider;
mod stream_info_volatile_fields;
mod stream_info_volatile_snapshot;
mod stream_info_volatile_xml;
mod timestamped;
mod timestamped_descriptor_chunk;
mod timestamped_descriptor_sample;
mod xml_character_data;
mod xml_element_serialization;
mod xml_element_tree;
mod xml_leaf_element;
mod xml_value;

pub use descriptor_sample::{
    BoundDescriptorSample, DescriptorSampleBound, DescriptorSampleError, DescriptorSampleInput,
    DescriptorSampleLimits,
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
pub use sample::{Sample, SampleBound, SampleError, SampleLimits};
pub use stream_definition::StreamDefinition;
pub use stream_descriptor::{
    ChannelFormat, InvalidRegularSampleRate, NominalSampleRate, NominalSampleRateError,
    RegularSampleRate, StreamDescriptor, StreamDescriptorBound, StreamDescriptorError,
    StreamDescriptorLimits, StreamDescriptorTextRole,
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
    /// Only local bounded metadata, sample, timestamped-chunk, descriptor, tree, and binding contracts exist.
    BoundedLocalContracts,
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
    ImplementationStatus::BoundedLocalContracts
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
            ImplementationStatus::BoundedLocalContracts
        );
    }

    #[test]
    fn authority_remains_outside_the_repository() {
        let declaration = ownership_declaration();
        assert!(declaration.does_not_own.contains(&"Manifold authority"));
        assert!(declaration
            .does_not_own
            .contains(&"commands derived from inbound samples"));
    }
}
