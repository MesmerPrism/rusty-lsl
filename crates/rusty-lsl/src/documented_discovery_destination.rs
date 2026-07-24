// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

/// The UDP port stated by the pinned public documentation for default discovery.
pub const DOCUMENTED_DEFAULT_DISCOVERY_PORT: u16 = 16_571;

/// One exact destination spelling displayed by the pinned public documentation.
///
/// Variants are closed data labels. They do not validate, parse, normalize, or
/// operationally classify an address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentedDiscoveryDestination {
    /// First displayed destination.
    First,
    /// Second displayed destination.
    Second,
    /// Third displayed destination, retained despite its unusual spelling.
    Third,
    /// Fourth displayed destination.
    Fourth,
    /// Fifth displayed destination.
    Fifth,
    /// Sixth displayed destination.
    Sixth,
    /// Seventh displayed destination.
    Seventh,
}

/// The complete destination inventory in documented display order.
pub const DOCUMENTED_DISCOVERY_DESTINATIONS: [DocumentedDiscoveryDestination; 7] = [
    DocumentedDiscoveryDestination::First,
    DocumentedDiscoveryDestination::Second,
    DocumentedDiscoveryDestination::Third,
    DocumentedDiscoveryDestination::Fourth,
    DocumentedDiscoveryDestination::Fifth,
    DocumentedDiscoveryDestination::Sixth,
    DocumentedDiscoveryDestination::Seventh,
];

impl DocumentedDiscoveryDestination {
    /// Returns the exact spelling displayed by the pinned documentation.
    #[must_use]
    pub const fn as_documented_str(self) -> &'static str {
        match self {
            Self::First => "FF02:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2",
            Self::Second => "FF05:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2",
            Self::Third => "FF08113D:6FDD:2C17:A643:FFE2:1BD1:3CD2",
            Self::Fourth => "FF0E:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2",
            Self::Fifth => "224.0.0.1",
            Self::Sixth => "224.0.0.183",
            Self::Seventh => "239.255.172.215",
        }
    }

    /// Reports whether the source displayed this spelling in parentheses.
    #[must_use]
    pub const fn source_parenthesized(self) -> bool {
        matches!(self, Self::First | Self::Third | Self::Fourth)
    }
}

#[cfg(test)]
mod tests {
    use super::{DOCUMENTED_DEFAULT_DISCOVERY_PORT, DOCUMENTED_DISCOVERY_DESTINATIONS};

    #[test]
    fn lslc_002j_exact_inventory_and_presentation_are_closed() {
        assert_eq!(DOCUMENTED_DEFAULT_DISCOVERY_PORT, 16_571);
        let actual = DOCUMENTED_DISCOVERY_DESTINATIONS
            .map(|item| (item.as_documented_str(), item.source_parenthesized()));
        assert_eq!(
            actual,
            [
                ("FF02:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", true),
                ("FF05:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", false),
                ("FF08113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", true),
                ("FF0E:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", true),
                ("224.0.0.1", false),
                ("224.0.0.183", false),
                ("239.255.172.215", false),
            ]
        );
    }

    #[test]
    fn lslc_002j_access_reuses_static_spelling_storage() {
        for destination in DOCUMENTED_DISCOVERY_DESTINATIONS {
            assert!(core::ptr::eq(
                destination.as_documented_str().as_ptr(),
                destination.as_documented_str().as_ptr()
            ));
        }
    }
}
