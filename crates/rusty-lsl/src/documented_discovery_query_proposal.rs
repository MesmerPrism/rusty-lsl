// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    DocumentedDiscoveryDestination, ShortInfoQueryWire, DOCUMENTED_DEFAULT_DISCOVERY_PORT,
};

/// An inert local composition of one documented destination and one accepted query payload.
///
/// Construction performs no I/O and grants no address, selection, send, or
/// discovery-runtime authority.
#[derive(Debug, Eq, PartialEq)]
pub struct DocumentedDiscoveryQueryProposal {
    destination: DocumentedDiscoveryDestination,
    query: ShortInfoQueryWire,
}

impl DocumentedDiscoveryQueryProposal {
    /// Moves one caller-selected documented destination beside one accepted query payload.
    #[must_use]
    pub const fn new(
        destination: DocumentedDiscoveryDestination,
        query: ShortInfoQueryWire,
    ) -> Self {
        Self { destination, query }
    }

    /// Returns the unchanged caller-selected destination label.
    #[must_use]
    pub const fn destination(&self) -> DocumentedDiscoveryDestination {
        self.destination
    }

    /// Returns the unchanged accepted query payload.
    #[must_use]
    pub const fn query(&self) -> &ShortInfoQueryWire {
        &self.query
    }

    /// Returns the documented port as data, without endpoint semantics.
    #[must_use]
    pub const fn documented_port(&self) -> u16 {
        DOCUMENTED_DEFAULT_DISCOVERY_PORT
    }

    /// Recovers both original components unchanged.
    #[must_use]
    pub fn into_parts(self) -> (DocumentedDiscoveryDestination, ShortInfoQueryWire) {
        (self.destination, self.query)
    }
}

#[cfg(test)]
mod tests {
    use super::DocumentedDiscoveryQueryProposal;
    use crate::{
        DocumentedDiscoveryDestination, ShortInfoQuery, ShortInfoQueryWire,
        ShortInfoQueryWireLimits,
    };

    fn wire() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(16, 64).unwrap();
        let query = ShortInfoQuery::new("name='alpha'".into(), 1, 0, limits).unwrap();
        ShortInfoQueryWire::encode(&query, limits).unwrap()
    }

    #[test]
    fn lslc_002k_borrowed_proposal_preserves_components() {
        let wire = wire();
        let pointer = wire.as_bytes().as_ptr();
        let proposal =
            DocumentedDiscoveryQueryProposal::new(DocumentedDiscoveryDestination::Third, wire);
        assert_eq!(proposal.documented_port(), 16_571);
        assert_eq!(
            proposal.destination().as_documented_str(),
            "FF08113D:6FDD:2C17:A643:FFE2:1BD1:3CD2"
        );
        assert!(proposal.destination().source_parenthesized());
        assert_eq!(proposal.query().as_bytes().as_ptr(), pointer);
    }

    #[test]
    fn lslc_002k_consuming_access_preserves_query_allocation() {
        let wire = wire();
        let pointer = wire.as_bytes().as_ptr();
        let proposal =
            DocumentedDiscoveryQueryProposal::new(DocumentedDiscoveryDestination::Seventh, wire);
        let (destination, wire) = proposal.into_parts();
        assert_eq!(destination, DocumentedDiscoveryDestination::Seventh);
        assert_eq!(wire.as_bytes().as_ptr(), pointer);
    }
}
