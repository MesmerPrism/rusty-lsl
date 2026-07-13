// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Inert facade for the Rusty LSL repository scaffold.
//!
//! No LSL protocol, runtime, wire, discovery, clock, inlet, outlet, FFI, or
//! Morphospace adapter behavior is implemented or claimed by this crate.

/// The implementation state exposed by this scaffold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImplementationStatus {
    /// The repository contains documentation and an inert facade only.
    ScaffoldOnly,
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
    ImplementationStatus::ScaffoldOnly
}

/// Returns the repository's current ownership declaration.
#[must_use]
pub const fn ownership_declaration() -> OwnershipDeclaration {
    OwnershipDeclaration {
        owns: &[
            "backend-neutral Rust LSL API",
            "LSL metadata and sample types",
            "future discovery, inlet, outlet, clock, recovery, and provider health behavior",
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
    fn status_does_not_claim_an_implementation() {
        assert_eq!(implementation_status(), ImplementationStatus::ScaffoldOnly);
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
