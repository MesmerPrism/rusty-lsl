// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Strict non-I/O IPv4 service-endpoint proposal from accepted typed discovery state.

use crate::{StreamInfoVolatileFieldRole, TypedUdpDiscoveryRun};
use std::net::{Ipv4Addr, SocketAddrV4};

/// Stable local rejection from selected-response endpoint projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypedUdpDiscoveryEndpointError {
    /// The caller-selected receive-order response does not exist.
    ResponseUnavailable {
        /// Requested zero-based receive-order index.
        index: usize,
        /// Number of accepted responses available in the run.
        response_count: usize,
    },
    /// The accepted `v4address` text is not an IPv4 address.
    InvalidAddress,
    /// The accepted `v4address` text is valid but not in canonical dotted-decimal form.
    NonCanonicalAddress,
    /// The accepted `v4address` is unspecified, multicast, or broadcast.
    NonConcreteUnicastAddress,
    /// The accepted `v4service_port` text is not an unsigned 16-bit integer.
    InvalidServicePort,
    /// The accepted `v4service_port` text is valid but not in canonical decimal form.
    NonCanonicalServicePort,
    /// The accepted `v4service_port` is zero.
    ZeroServicePort,
}

/// Proposes a concrete IPv4 service address from one caller-selected accepted response.
///
/// This performs no I/O or selection and grants no connection, routing, admission, or authority.
pub fn propose_typed_udp_discovery_ipv4_service_endpoint(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
) -> Result<SocketAddrV4, TypedUdpDiscoveryEndpointError> {
    let response = run.responses().get(response_index).ok_or(
        TypedUdpDiscoveryEndpointError::ResponseUnavailable {
            index: response_index,
            response_count: run.responses().len(),
        },
    )?;
    let fields = response.observation().fields().volatile_fields();
    parse_endpoint(
        fields.field(StreamInfoVolatileFieldRole::V4Address),
        fields.field(StreamInfoVolatileFieldRole::V4ServicePort),
    )
}

fn parse_endpoint(
    address_text: &str,
    service_port_text: &str,
) -> Result<SocketAddrV4, TypedUdpDiscoveryEndpointError> {
    let address = address_text
        .parse::<Ipv4Addr>()
        .map_err(|_| TypedUdpDiscoveryEndpointError::InvalidAddress)?;
    if address.to_string() != address_text {
        return Err(TypedUdpDiscoveryEndpointError::NonCanonicalAddress);
    }
    if address.is_unspecified() || address.is_multicast() || address == Ipv4Addr::BROADCAST {
        return Err(TypedUdpDiscoveryEndpointError::NonConcreteUnicastAddress);
    }

    let service_port = service_port_text
        .parse::<u16>()
        .map_err(|_| TypedUdpDiscoveryEndpointError::InvalidServicePort)?;
    if service_port.to_string() != service_port_text {
        return Err(TypedUdpDiscoveryEndpointError::NonCanonicalServicePort);
    }
    if service_port == 0 {
        return Err(TypedUdpDiscoveryEndpointError::ZeroServicePort);
    }
    Ok(SocketAddrV4::new(address, service_port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lslc_004x_accepts_only_canonical_concrete_unicast_endpoint_text() {
        assert_eq!(
            parse_endpoint("127.0.0.1", "16572"),
            Ok(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 16572))
        );
        assert_eq!(
            parse_endpoint("192.168.1.7", "443"),
            Ok(SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 7), 443))
        );
        assert_eq!(
            parse_endpoint("01.2.3.4", "443"),
            Err(TypedUdpDiscoveryEndpointError::InvalidAddress)
        );
        assert_eq!(
            parse_endpoint("127.0.0.1", "0443"),
            Err(TypedUdpDiscoveryEndpointError::NonCanonicalServicePort)
        );
    }

    #[test]
    fn lslc_004x_rejects_unavailable_or_non_service_values_without_io() {
        for address in ["0.0.0.0", "239.255.172.215", "255.255.255.255"] {
            assert_eq!(
                parse_endpoint(address, "16572"),
                Err(TypedUdpDiscoveryEndpointError::NonConcreteUnicastAddress)
            );
        }
        assert_eq!(
            parse_endpoint("not-an-address", "16572"),
            Err(TypedUdpDiscoveryEndpointError::InvalidAddress)
        );
        assert_eq!(
            parse_endpoint("127.0.0.1", "not-a-port"),
            Err(TypedUdpDiscoveryEndpointError::InvalidServicePort)
        );
        assert_eq!(
            parse_endpoint("127.0.0.1", "0"),
            Err(TypedUdpDiscoveryEndpointError::ZeroServicePort)
        );
    }
}
