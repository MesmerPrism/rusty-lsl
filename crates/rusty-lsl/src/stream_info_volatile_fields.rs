// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// Classifies which layer supplies one volatile stream-info value.
///
/// This classification records data ownership only. It does not acquire,
/// inspect, generate, validate semantically, or activate any value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamInfoVolatileFieldClass {
    /// Implementation-assigned compatibility-version data.
    ImplementationAssigned,
    /// Runtime-assigned creation, identity, session, or host data.
    RuntimeAssigned,
    /// Transport-owned address or port data.
    TransportOwned,
}

/// Identifies one accepted volatile stream-info field in fixed observed order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamInfoVolatileFieldRole {
    /// Implementation-assigned compatibility-version text.
    Version,
    /// Runtime-assigned creation-time text.
    CreatedAt,
    /// Runtime-assigned opaque identity text.
    Uid,
    /// Runtime-assigned session text.
    SessionId,
    /// Runtime-assigned host text.
    Hostname,
    /// Transport-owned IPv4-address-role text.
    V4Address,
    /// Transport-owned IPv4 data-port-role text.
    V4DataPort,
    /// Transport-owned IPv4 service-port-role text.
    V4ServicePort,
    /// Transport-owned IPv6-address-role text.
    V6Address,
    /// Transport-owned IPv6 data-port-role text.
    V6DataPort,
    /// Transport-owned IPv6 service-port-role text.
    V6ServicePort,
}

impl StreamInfoVolatileFieldRole {
    /// Returns the data-ownership class for this role.
    #[must_use]
    pub const fn class(self) -> StreamInfoVolatileFieldClass {
        match self {
            Self::Version => StreamInfoVolatileFieldClass::ImplementationAssigned,
            Self::CreatedAt | Self::Uid | Self::SessionId | Self::Hostname => {
                StreamInfoVolatileFieldClass::RuntimeAssigned
            }
            Self::V4Address
            | Self::V4DataPort
            | Self::V4ServicePort
            | Self::V6Address
            | Self::V6DataPort
            | Self::V6ServicePort => StreamInfoVolatileFieldClass::TransportOwned,
        }
    }
}

/// Three explicit nonzero maxima for volatile stream-info text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileFieldLimits {
    max_implementation_code_points: usize,
    max_runtime_code_points: usize,
    max_transport_code_points: usize,
}

impl StreamInfoVolatileFieldLimits {
    /// Validates maxima in implementation, runtime, then transport order.
    pub fn new(
        max_implementation_code_points: usize,
        max_runtime_code_points: usize,
        max_transport_code_points: usize,
    ) -> Result<Self, StreamInfoVolatileFieldError> {
        for (class, actual) in [
            (
                StreamInfoVolatileFieldClass::ImplementationAssigned,
                max_implementation_code_points,
            ),
            (
                StreamInfoVolatileFieldClass::RuntimeAssigned,
                max_runtime_code_points,
            ),
            (
                StreamInfoVolatileFieldClass::TransportOwned,
                max_transport_code_points,
            ),
        ] {
            if actual == 0 {
                return Err(StreamInfoVolatileFieldError::InvalidLimit {
                    class,
                    expected_min: 1,
                    actual,
                });
            }
        }

        Ok(Self {
            max_implementation_code_points,
            max_runtime_code_points,
            max_transport_code_points,
        })
    }

    /// Returns the maximum scalar count for implementation-assigned text.
    #[must_use]
    pub const fn max_implementation_code_points(self) -> usize {
        self.max_implementation_code_points
    }

    /// Returns the maximum scalar count for each runtime-assigned field.
    #[must_use]
    pub const fn max_runtime_code_points(self) -> usize {
        self.max_runtime_code_points
    }

    /// Returns the maximum scalar count for each transport-owned field.
    #[must_use]
    pub const fn max_transport_code_points(self) -> usize {
        self.max_transport_code_points
    }

    const fn maximum_for(self, class: StreamInfoVolatileFieldClass) -> usize {
        match class {
            StreamInfoVolatileFieldClass::ImplementationAssigned => {
                self.max_implementation_code_points
            }
            StreamInfoVolatileFieldClass::RuntimeAssigned => self.max_runtime_code_points,
            StreamInfoVolatileFieldClass::TransportOwned => self.max_transport_code_points,
        }
    }
}

/// Unvalidated owned values for the eleven volatile stream-info roles.
///
/// These strings are opaque data. This input performs no clock or host read,
/// identity generation, address or port parsing, XML handling, or provider
/// operation.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileFieldInput {
    version: String,
    created_at: String,
    uid: String,
    session_id: String,
    hostname: String,
    v4address: String,
    v4data_port: String,
    v4service_port: String,
    v6address: String,
    v6data_port: String,
    v6service_port: String,
}

impl StreamInfoVolatileFieldInput {
    /// Groups caller-owned opaque values without validating or changing them.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        version: String,
        created_at: String,
        uid: String,
        session_id: String,
        hostname: String,
        v4address: String,
        v4data_port: String,
        v4service_port: String,
        v6address: String,
        v6data_port: String,
        v6service_port: String,
    ) -> Self {
        Self {
            version,
            created_at,
            uid,
            session_id,
            hostname,
            v4address,
            v4data_port,
            v4service_port,
            v6address,
            v6data_port,
            v6service_port,
        }
    }

    fn field(&self, role: StreamInfoVolatileFieldRole) -> &str {
        match role {
            StreamInfoVolatileFieldRole::Version => &self.version,
            StreamInfoVolatileFieldRole::CreatedAt => &self.created_at,
            StreamInfoVolatileFieldRole::Uid => &self.uid,
            StreamInfoVolatileFieldRole::SessionId => &self.session_id,
            StreamInfoVolatileFieldRole::Hostname => &self.hostname,
            StreamInfoVolatileFieldRole::V4Address => &self.v4address,
            StreamInfoVolatileFieldRole::V4DataPort => &self.v4data_port,
            StreamInfoVolatileFieldRole::V4ServicePort => &self.v4service_port,
            StreamInfoVolatileFieldRole::V6Address => &self.v6address,
            StreamInfoVolatileFieldRole::V6DataPort => &self.v6data_port,
            StreamInfoVolatileFieldRole::V6ServicePort => &self.v6service_port,
        }
    }

    /// Returns all eleven original allocations in fixed role order.
    #[allow(clippy::type_complexity)]
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) {
        (
            self.version,
            self.created_at,
            self.uid,
            self.session_id,
            self.hostname,
            self.v4address,
            self.v4data_port,
            self.v4service_port,
            self.v6address,
            self.v6data_port,
            self.v6service_port,
        )
    }
}

/// A bounded accepted-data view of eleven opaque volatile stream-info fields.
///
/// Accepted state owns only its limits and the original input. It does not
/// assert that any value is current, generated, parsed, reachable, authorized,
/// represented as XML, or accepted by a runtime or Manifold.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileFields {
    limits: StreamInfoVolatileFieldLimits,
    input: StreamInfoVolatileFieldInput,
}

impl StreamInfoVolatileFields {
    const ROLES: [StreamInfoVolatileFieldRole; 11] = [
        StreamInfoVolatileFieldRole::Version,
        StreamInfoVolatileFieldRole::CreatedAt,
        StreamInfoVolatileFieldRole::Uid,
        StreamInfoVolatileFieldRole::SessionId,
        StreamInfoVolatileFieldRole::Hostname,
        StreamInfoVolatileFieldRole::V4Address,
        StreamInfoVolatileFieldRole::V4DataPort,
        StreamInfoVolatileFieldRole::V4ServicePort,
        StreamInfoVolatileFieldRole::V6Address,
        StreamInfoVolatileFieldRole::V6DataPort,
        StreamInfoVolatileFieldRole::V6ServicePort,
    ];

    /// Validates every field in fixed role order and retains the input unchanged.
    pub fn new(
        limits: StreamInfoVolatileFieldLimits,
        input: StreamInfoVolatileFieldInput,
    ) -> Result<Self, StreamInfoVolatileFieldError> {
        for role in Self::ROLES {
            let actual = input.field(role).chars().count();
            let expected_max = limits.maximum_for(role.class());
            if actual > expected_max {
                return Err(StreamInfoVolatileFieldError::TextLimitExceeded {
                    role,
                    expected_max,
                    actual,
                });
            }
        }

        Ok(Self { limits, input })
    }

    /// Returns the fixed eleven-role order observed by LSLC-001H.
    #[must_use]
    pub const fn roles() -> &'static [StreamInfoVolatileFieldRole; 11] {
        &Self::ROLES
    }

    /// Returns the limits under which all fields were accepted.
    #[must_use]
    pub const fn limits(&self) -> StreamInfoVolatileFieldLimits {
        self.limits
    }

    /// Returns one unchanged opaque field by role.
    #[must_use]
    pub fn field(&self, role: StreamInfoVolatileFieldRole) -> &str {
        self.input.field(role)
    }

    /// Returns the original accepted input without reallocating its strings.
    #[must_use]
    pub fn into_input(self) -> StreamInfoVolatileFieldInput {
        self.input
    }
}

/// Deterministic rejection from volatile stream-info data bounds.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoVolatileFieldError {
    /// One ownership-class maximum was zero.
    InvalidLimit {
        /// The first malformed ownership class.
        class: StreamInfoVolatileFieldClass,
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// One opaque field exceeded its ownership-class maximum.
    TextLimitExceeded {
        /// The first rejected field in fixed role order.
        role: StreamInfoVolatileFieldRole,
        /// The maximum scalar-value count for its ownership class.
        expected_max: usize,
        /// The field's actual scalar-value count.
        actual: usize,
    },
}

impl fmt::Display for StreamInfoVolatileFieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "volatile stream-info data rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoVolatileFieldError {}

#[cfg(test)]
mod tests {
    use super::{
        StreamInfoVolatileFieldClass, StreamInfoVolatileFieldError, StreamInfoVolatileFieldInput,
        StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole, StreamInfoVolatileFields,
    };

    fn input(values: [&str; 11]) -> StreamInfoVolatileFieldInput {
        let [version, created_at, uid, session_id, hostname, v4address, v4data_port, v4service_port, v6address, v6data_port, v6service_port] =
            values;
        StreamInfoVolatileFieldInput::new(
            version.to_owned(),
            created_at.to_owned(),
            uid.to_owned(),
            session_id.to_owned(),
            hostname.to_owned(),
            v4address.to_owned(),
            v4data_port.to_owned(),
            v4service_port.to_owned(),
            v6address.to_owned(),
            v6data_port.to_owned(),
            v6service_port.to_owned(),
        )
    }

    #[test]
    fn lslc_001o_role_order_and_classes_are_exact_and_disjoint() {
        use StreamInfoVolatileFieldClass::{
            ImplementationAssigned, RuntimeAssigned, TransportOwned,
        };
        use StreamInfoVolatileFieldRole::{
            CreatedAt, Hostname, SessionId, Uid, V4Address, V4DataPort, V4ServicePort, V6Address,
            V6DataPort, V6ServicePort, Version,
        };

        assert_eq!(
            StreamInfoVolatileFields::roles(),
            &[
                Version,
                CreatedAt,
                Uid,
                SessionId,
                Hostname,
                V4Address,
                V4DataPort,
                V4ServicePort,
                V6Address,
                V6DataPort,
                V6ServicePort,
            ]
        );
        assert_eq!(Version.class(), ImplementationAssigned);
        for role in [CreatedAt, Uid, SessionId, Hostname] {
            assert_eq!(role.class(), RuntimeAssigned);
        }
        for role in [
            V4Address,
            V4DataPort,
            V4ServicePort,
            V6Address,
            V6DataPort,
            V6ServicePort,
        ] {
            assert_eq!(role.class(), TransportOwned);
        }
    }

    #[test]
    fn lslc_001o_exact_limits_preserve_empty_unicode_and_original_allocations() {
        let values = ["Ω", "中", "", "s", "h", "雪", "1", "2", "λ", "3", "4"];
        let source = input(values);
        let pointers = StreamInfoVolatileFields::roles().map(|role| source.field(role).as_ptr());
        let accepted = StreamInfoVolatileFields::new(
            StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
            source,
        )
        .unwrap();

        assert_eq!(
            accepted.limits(),
            StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap()
        );
        for ((role, expected), pointer) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip(values)
            .zip(pointers)
        {
            assert_eq!(accepted.field(role), expected);
            assert_eq!(accepted.field(role).as_ptr(), pointer);
        }

        let parts = accepted.into_input().into_parts();
        assert_eq!(parts.0.as_ptr(), pointers[0]);
        assert_eq!(parts.1.as_ptr(), pointers[1]);
        assert_eq!(parts.2.as_ptr(), pointers[2]);
        assert_eq!(parts.3.as_ptr(), pointers[3]);
        assert_eq!(parts.4.as_ptr(), pointers[4]);
        assert_eq!(parts.5.as_ptr(), pointers[5]);
        assert_eq!(parts.6.as_ptr(), pointers[6]);
        assert_eq!(parts.7.as_ptr(), pointers[7]);
        assert_eq!(parts.8.as_ptr(), pointers[8]);
        assert_eq!(parts.9.as_ptr(), pointers[9]);
        assert_eq!(parts.10.as_ptr(), pointers[10]);
    }

    #[test]
    fn lslc_001o_zero_limits_reject_in_class_order() {
        use StreamInfoVolatileFieldClass::{
            ImplementationAssigned, RuntimeAssigned, TransportOwned,
        };

        for (arguments, class) in [
            ((0, 0, 0), ImplementationAssigned),
            ((1, 0, 0), RuntimeAssigned),
            ((1, 1, 0), TransportOwned),
        ] {
            assert_eq!(
                StreamInfoVolatileFieldLimits::new(arguments.0, arguments.1, arguments.2),
                Err(StreamInfoVolatileFieldError::InvalidLimit {
                    class,
                    expected_min: 1,
                    actual: 0,
                })
            );
        }
    }

    #[test]
    fn lslc_001o_one_past_values_reject_in_fixed_role_order() {
        let limits = StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap();
        for (index, role) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .enumerate()
        {
            let mut values = [""; 11];
            values[index] = "ab";
            assert_eq!(
                StreamInfoVolatileFields::new(limits, input(values)),
                Err(StreamInfoVolatileFieldError::TextLimitExceeded {
                    role,
                    expected_max: 1,
                    actual: 2,
                })
            );
        }
    }

    #[test]
    fn lslc_001o_values_remain_opaque_without_provider_or_semantic_interpretation() {
        let values = [
            "not-a-version",
            "not-a-time",
            "not-a-uid",
            "not-a-session",
            "not-a-host",
            "not-an-ipv4-address",
            "not-a-port",
            "-1",
            "not-an-ipv6-address",
            "70000",
            "<&>\"'",
        ];
        let accepted = StreamInfoVolatileFields::new(
            StreamInfoVolatileFieldLimits::new(32, 32, 32).unwrap(),
            input(values),
        )
        .unwrap();

        for (role, expected) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip(values)
        {
            assert_eq!(accepted.field(role), expected);
        }
    }
}
