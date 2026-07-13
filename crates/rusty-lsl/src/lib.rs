// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Dependency-free local contracts for Rusty LSL.
//!
//! This crate currently implements only bounded metadata and sample-shape
//! construction. It does not implement or claim LSL protocol, runtime, wire,
//! discovery, clock, inlet, outlet, FFI, or Morphospace adapter behavior.

mod metadata;
mod sample;

pub use metadata::{
    BoundedMetadata, MetadataBound, MetadataDescription, MetadataError, MetadataField,
    MetadataLimits, MetadataTextRole,
};
pub use sample::{Sample, SampleBound, SampleError, SampleLimits};

/// The implementation state exposed by the crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImplementationStatus {
    /// Only local bounded metadata and sample-shape contracts are implemented.
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
