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
//! composition. It does not
//! implement or claim LSL protocol,
//! runtime, wire, discovery, clock, inlet, outlet, FFI, or Morphospace adapter
//! behavior.

mod descriptor_sample;
mod metadata;
mod metadata_tree;
mod sample;
mod stream_definition;
mod stream_descriptor;
mod timestamped;
mod timestamped_descriptor_chunk;
mod timestamped_descriptor_sample;

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
pub use sample::{Sample, SampleBound, SampleError, SampleLimits};
pub use stream_definition::StreamDefinition;
pub use stream_descriptor::{
    ChannelFormat, InvalidRegularSampleRate, NominalSampleRate, NominalSampleRateError,
    RegularSampleRate, StreamDescriptor, StreamDescriptorBound, StreamDescriptorError,
    StreamDescriptorLimits, StreamDescriptorTextRole,
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
