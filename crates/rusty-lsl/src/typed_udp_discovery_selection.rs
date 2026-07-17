// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Allocation-free caller-explicit suggestion over accepted typed discovery responses.

use crate::TypedUdpDiscoveryRun;

/// Invalid local discovery-selection input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypedUdpDiscoverySelectionError {
    /// The caller supplied no stream-name value.
    EmptyStreamName,
}

/// Suggests the first receive-order response whose accepted stream name exactly matches.
///
/// The returned index is local advisory evidence only. It performs no I/O and grants no
/// endpoint, connection, admission, routing, or Manifold authority.
pub fn suggest_typed_udp_discovery_response(
    run: &TypedUdpDiscoveryRun,
    stream_name: &str,
) -> Result<Option<usize>, TypedUdpDiscoverySelectionError> {
    if stream_name.is_empty() {
        return Err(TypedUdpDiscoverySelectionError::EmptyStreamName);
    }
    Ok(run.responses().iter().position(|response| {
        response
            .observation()
            .fields()
            .definition()
            .descriptor()
            .name()
            == stream_name
    }))
}
