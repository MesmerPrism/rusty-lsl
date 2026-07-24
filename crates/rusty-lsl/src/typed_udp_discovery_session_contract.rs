// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private validation of one caller-selected discovery response against a session contract.

use crate::{
    ChannelFormat, StreamHandshakeIdentity, StreamHandshakeIdentityRole,
    StreamInfoVolatileFieldRole, TypedUdpDiscoveryResponse,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TypedUdpDiscoverySessionContractMismatch<'a> {
    Format {
        expected: ChannelFormat,
        actual: ChannelFormat,
    },
    ChannelCount {
        expected: usize,
        actual: usize,
    },
    Identity {
        role: StreamHandshakeIdentityRole,
        expected: &'a str,
        actual: &'a str,
    },
}

pub(crate) fn validate_selected_typed_udp_discovery_session_contract<'a>(
    response: &'a TypedUdpDiscoveryResponse,
    expected_format: ChannelFormat,
    expected_channel_count: usize,
    expected_identity: &'a StreamHandshakeIdentity,
) -> Result<(), TypedUdpDiscoverySessionContractMismatch<'a>> {
    let fields = response.observation().fields();
    let descriptor = fields.definition().descriptor();
    let volatile = fields.volatile_fields();

    validate_contract_fields(
        descriptor.channel_format(),
        descriptor.channel_count(),
        volatile.field(StreamInfoVolatileFieldRole::Uid),
        volatile.field(StreamInfoVolatileFieldRole::Hostname),
        descriptor.source_id().unwrap_or(""),
        volatile.field(StreamInfoVolatileFieldRole::SessionId),
        expected_format,
        expected_channel_count,
        expected_identity,
    )
}

#[allow(clippy::too_many_arguments)]
fn validate_contract_fields<'a>(
    actual_format: ChannelFormat,
    actual_channel_count: usize,
    actual_uid: &'a str,
    actual_hostname: &'a str,
    actual_source_id: &'a str,
    actual_session_id: &'a str,
    expected_format: ChannelFormat,
    expected_channel_count: usize,
    expected_identity: &'a StreamHandshakeIdentity,
) -> Result<(), TypedUdpDiscoverySessionContractMismatch<'a>> {
    if actual_format != expected_format {
        return Err(TypedUdpDiscoverySessionContractMismatch::Format {
            expected: expected_format,
            actual: actual_format,
        });
    }
    if actual_channel_count != expected_channel_count {
        return Err(TypedUdpDiscoverySessionContractMismatch::ChannelCount {
            expected: expected_channel_count,
            actual: actual_channel_count,
        });
    }

    for (role, actual) in [
        (StreamHandshakeIdentityRole::Uid, actual_uid),
        (StreamHandshakeIdentityRole::Hostname, actual_hostname),
        (StreamHandshakeIdentityRole::SourceId, actual_source_id),
        (StreamHandshakeIdentityRole::SessionId, actual_session_id),
    ] {
        let expected = expected_identity.field(role);
        if actual != expected {
            return Err(TypedUdpDiscoverySessionContractMismatch::Identity {
                role,
                expected,
                actual,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StreamHandshakeLimits;
    use std::time::Duration;

    const UID: &str = "synthetic-uid";
    const HOSTNAME: &str = "synthetic-host";
    const SOURCE_ID: &str = "synthetic-source";
    const SESSION_ID: &str = "synthetic-session";

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            UID.into(),
            HOSTNAME.into(),
            SOURCE_ID.into(),
            SESSION_ID.into(),
            StreamHandshakeLimits::new(
                1024,
                64,
                Duration::from_millis(1),
                Duration::from_millis(1),
            )
            .unwrap(),
        )
        .unwrap()
    }

    fn validate<'a>(
        actual_format: ChannelFormat,
        actual_channel_count: usize,
        actual_uid: &'a str,
        actual_hostname: &'a str,
        actual_source_id: &'a str,
        actual_session_id: &'a str,
        expected_identity: &'a StreamHandshakeIdentity,
    ) -> Result<(), TypedUdpDiscoverySessionContractMismatch<'a>> {
        validate_contract_fields(
            actual_format,
            actual_channel_count,
            actual_uid,
            actual_hostname,
            actual_source_id,
            actual_session_id,
            ChannelFormat::Float32,
            2,
            expected_identity,
        )
    }

    #[test]
    fn selected_resolution_exact_contract_succeeds_without_sockets() {
        assert_eq!(
            validate(
                ChannelFormat::Float32,
                2,
                UID,
                HOSTNAME,
                SOURCE_ID,
                SESSION_ID,
                &identity(),
            ),
            Ok(())
        );
    }

    #[test]
    fn every_mismatch_returns_borrowed_exact_evidence() {
        let expected_identity = identity();
        let cases = [
            (
                validate(
                    ChannelFormat::Double64,
                    2,
                    UID,
                    HOSTNAME,
                    SOURCE_ID,
                    SESSION_ID,
                    &expected_identity,
                ),
                TypedUdpDiscoverySessionContractMismatch::Format {
                    expected: ChannelFormat::Float32,
                    actual: ChannelFormat::Double64,
                },
            ),
            (
                validate(
                    ChannelFormat::Float32,
                    3,
                    UID,
                    HOSTNAME,
                    SOURCE_ID,
                    SESSION_ID,
                    &expected_identity,
                ),
                TypedUdpDiscoverySessionContractMismatch::ChannelCount {
                    expected: 2,
                    actual: 3,
                },
            ),
            (
                validate(
                    ChannelFormat::Float32,
                    2,
                    "actual-uid",
                    HOSTNAME,
                    SOURCE_ID,
                    SESSION_ID,
                    &expected_identity,
                ),
                TypedUdpDiscoverySessionContractMismatch::Identity {
                    role: StreamHandshakeIdentityRole::Uid,
                    expected: UID,
                    actual: "actual-uid",
                },
            ),
            (
                validate(
                    ChannelFormat::Float32,
                    2,
                    UID,
                    "actual-host",
                    SOURCE_ID,
                    SESSION_ID,
                    &expected_identity,
                ),
                TypedUdpDiscoverySessionContractMismatch::Identity {
                    role: StreamHandshakeIdentityRole::Hostname,
                    expected: HOSTNAME,
                    actual: "actual-host",
                },
            ),
            (
                validate(
                    ChannelFormat::Float32,
                    2,
                    UID,
                    HOSTNAME,
                    "actual-source",
                    SESSION_ID,
                    &expected_identity,
                ),
                TypedUdpDiscoverySessionContractMismatch::Identity {
                    role: StreamHandshakeIdentityRole::SourceId,
                    expected: SOURCE_ID,
                    actual: "actual-source",
                },
            ),
            (
                validate(
                    ChannelFormat::Float32,
                    2,
                    UID,
                    HOSTNAME,
                    SOURCE_ID,
                    "actual-session",
                    &expected_identity,
                ),
                TypedUdpDiscoverySessionContractMismatch::Identity {
                    role: StreamHandshakeIdentityRole::SessionId,
                    expected: SESSION_ID,
                    actual: "actual-session",
                },
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, Err(expected));
        }
    }

    #[test]
    fn mismatches_follow_the_frozen_precedence() {
        let expected_identity = identity();
        assert!(matches!(
            validate(
                ChannelFormat::Int16,
                3,
                "uid-x",
                "host-x",
                "source-x",
                "session-x",
                &expected_identity
            ),
            Err(TypedUdpDiscoverySessionContractMismatch::Format { .. })
        ));
        assert!(matches!(
            validate(
                ChannelFormat::Float32,
                3,
                "uid-x",
                "host-x",
                "source-x",
                "session-x",
                &expected_identity
            ),
            Err(TypedUdpDiscoverySessionContractMismatch::ChannelCount { .. })
        ));

        for (actual_uid, actual_hostname, actual_source_id, actual_session_id, first_role) in [
            (
                "uid-x",
                "host-x",
                "source-x",
                "session-x",
                StreamHandshakeIdentityRole::Uid,
            ),
            (
                UID,
                "host-x",
                "source-x",
                "session-x",
                StreamHandshakeIdentityRole::Hostname,
            ),
            (
                UID,
                HOSTNAME,
                "source-x",
                "session-x",
                StreamHandshakeIdentityRole::SourceId,
            ),
            (
                UID,
                HOSTNAME,
                SOURCE_ID,
                "session-x",
                StreamHandshakeIdentityRole::SessionId,
            ),
        ] {
            assert!(matches!(
                validate(ChannelFormat::Float32, 2, actual_uid, actual_hostname, actual_source_id, actual_session_id, &expected_identity),
                Err(TypedUdpDiscoverySessionContractMismatch::Identity { role, .. }) if role == first_role
            ));
        }
    }
}
