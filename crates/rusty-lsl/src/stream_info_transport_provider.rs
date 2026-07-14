// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{
    StreamInfoVolatileFieldError, StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole,
    StreamInfoVolatileProviderValue,
};

/// Bound for the transport provider identity retained as owner evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoTransportEvidenceLimit {
    max_provider_identity_code_points: usize,
}
impl StreamInfoTransportEvidenceLimit {
    /// Creates a nonzero provider-identity scalar bound.
    pub fn new(
        max_provider_identity_code_points: usize,
    ) -> Result<Self, StreamInfoTransportEvidenceError> {
        if max_provider_identity_code_points == 0 {
            return Err(StreamInfoTransportEvidenceError::InvalidProviderIdentityLimit);
        }
        Ok(Self {
            max_provider_identity_code_points,
        })
    }
    /// Returns the exact provider-identity scalar bound.
    #[must_use]
    pub const fn max_provider_identity_code_points(self) -> usize {
        self.max_provider_identity_code_points
    }
}

/// One owner-issued witness shared by all six transport-owned values.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoTransportWitness {
    provider_identity: String,
    epoch: u64,
    revision: u64,
}
impl StreamInfoTransportWitness {
    /// Validates and retains the provider identity and owner-issued epoch/revision.
    pub fn new(
        limit: StreamInfoTransportEvidenceLimit,
        provider_identity: String,
        epoch: u64,
        revision: u64,
    ) -> Result<Self, StreamInfoTransportEvidenceError> {
        let actual = provider_identity.chars().count();
        if actual == 0 {
            return Err(StreamInfoTransportEvidenceError::EmptyProviderIdentity);
        }
        if actual > limit.max_provider_identity_code_points {
            return Err(
                StreamInfoTransportEvidenceError::ProviderIdentityLimitExceeded {
                    expected_max: limit.max_provider_identity_code_points,
                    actual,
                },
            );
        }
        Ok(Self {
            provider_identity,
            epoch,
            revision,
        })
    }
    /// Returns the exact provider identity.
    #[must_use]
    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }
    /// Returns the owner-issued epoch.
    #[must_use]
    pub const fn epoch(&self) -> u64 {
        self.epoch
    }
    /// Returns the owner-issued revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

/// Six opaque values acquired together from one transport owner.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoTransportValues {
    v4address: String,
    v4data_port: String,
    v4service_port: String,
    v6address: String,
    v6data_port: String,
    v6service_port: String,
}
impl StreamInfoTransportValues {
    /// Groups the six transport values without interpreting them.
    #[must_use]
    pub const fn new(
        v4address: String,
        v4data_port: String,
        v4service_port: String,
        v6address: String,
        v6data_port: String,
        v6service_port: String,
    ) -> Self {
        Self {
            v4address,
            v4data_port,
            v4service_port,
            v6address,
            v6data_port,
            v6service_port,
        }
    }
    /// Returns opaque IPv4 address text.
    #[must_use]
    pub fn v4address(&self) -> &str {
        &self.v4address
    }
    /// Returns opaque IPv4 data-port text.
    #[must_use]
    pub fn v4data_port(&self) -> &str {
        &self.v4data_port
    }
    /// Returns opaque IPv4 service-port text.
    #[must_use]
    pub fn v4service_port(&self) -> &str {
        &self.v4service_port
    }
    /// Returns opaque IPv6 address text.
    #[must_use]
    pub fn v6address(&self) -> &str {
        &self.v6address
    }
    /// Returns opaque IPv6 data-port text.
    #[must_use]
    pub fn v6data_port(&self) -> &str {
        &self.v6data_port
    }
    /// Returns opaque IPv6 service-port text.
    #[must_use]
    pub fn v6service_port(&self) -> &str {
        &self.v6service_port
    }
    /// Moves all six original allocations out in fixed transport role order.
    #[must_use]
    pub fn into_parts(self) -> (String, String, String, String, String, String) {
        (
            self.v4address,
            self.v4data_port,
            self.v4service_port,
            self.v6address,
            self.v6data_port,
            self.v6service_port,
        )
    }
}

/// One provider result containing one shared witness and all six values.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoTransportProviderOutput {
    witness: StreamInfoTransportWitness,
    values: StreamInfoTransportValues,
}
impl StreamInfoTransportProviderOutput {
    /// Groups provider output without claiming its evidence is expected.
    #[must_use]
    pub const fn new(
        witness: StreamInfoTransportWitness,
        values: StreamInfoTransportValues,
    ) -> Self {
        Self { witness, values }
    }
}

/// A caller-selected synchronous provider for all transport-owned values.
pub trait StreamInfoTransportProvider {
    /// Provider-owned failure returned unchanged.
    type Error;
    /// Acquires all six values and their one shared owner witness.
    fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error>;
}

/// Accepted transport-lane acquisition with separately inspectable evidence.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoTransportAcquisition {
    witness: StreamInfoTransportWitness,
    values: StreamInfoTransportValues,
}
impl StreamInfoTransportAcquisition {
    /// Calls one selected provider once, matches its witness, then applies O bounds in role order.
    pub fn acquire<P: StreamInfoTransportProvider>(
        provider: &mut P,
        expected: &StreamInfoTransportWitness,
        limits: StreamInfoVolatileFieldLimits,
    ) -> Result<Self, StreamInfoTransportAcquisitionError<P::Error>> {
        let output = provider
            .acquire()
            .map_err(StreamInfoTransportAcquisitionError::Provider)?;
        if output.witness.provider_identity != expected.provider_identity {
            return Err(StreamInfoTransportAcquisitionError::ProviderIdentityMismatch);
        }
        if output.witness.epoch != expected.epoch {
            return Err(StreamInfoTransportAcquisitionError::EpochMismatch {
                expected: expected.epoch,
                actual: output.witness.epoch,
            });
        }
        if output.witness.revision != expected.revision {
            return Err(StreamInfoTransportAcquisitionError::RevisionMismatch {
                expected: expected.revision,
                actual: output.witness.revision,
            });
        }
        let maximum = limits.max_transport_code_points();
        for (role, value) in [
            (
                StreamInfoVolatileFieldRole::V4Address,
                output.values.v4address.as_str(),
            ),
            (
                StreamInfoVolatileFieldRole::V4DataPort,
                output.values.v4data_port.as_str(),
            ),
            (
                StreamInfoVolatileFieldRole::V4ServicePort,
                output.values.v4service_port.as_str(),
            ),
            (
                StreamInfoVolatileFieldRole::V6Address,
                output.values.v6address.as_str(),
            ),
            (
                StreamInfoVolatileFieldRole::V6DataPort,
                output.values.v6data_port.as_str(),
            ),
            (
                StreamInfoVolatileFieldRole::V6ServicePort,
                output.values.v6service_port.as_str(),
            ),
        ] {
            let actual = value.chars().count();
            if actual > maximum {
                return Err(StreamInfoTransportAcquisitionError::Value(
                    StreamInfoVolatileFieldError::TextLimitExceeded {
                        role,
                        expected_max: maximum,
                        actual,
                    },
                ));
            }
        }
        Ok(Self {
            witness: output.witness,
            values: output.values,
        })
    }
    /// Returns the exact matched shared owner witness.
    #[must_use]
    pub const fn witness(&self) -> &StreamInfoTransportWitness {
        &self.witness
    }
    /// Returns all six opaque values.
    #[must_use]
    pub const fn values(&self) -> &StreamInfoTransportValues {
        &self.values
    }
    /// Moves the separately retained witness and grouped transport values apart.
    #[must_use]
    pub fn into_parts(self) -> (StreamInfoTransportWitness, StreamInfoTransportValues) {
        (self.witness, self.values)
    }
    /// Moves the six original allocations into exactly the LSLC-001S transport lane.
    #[must_use]
    pub fn into_provider_values(self) -> [StreamInfoVolatileProviderValue; 6] {
        [
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::V4Address,
                self.values.v4address,
            ),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::V4DataPort,
                self.values.v4data_port,
            ),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::V4ServicePort,
                self.values.v4service_port,
            ),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::V6Address,
                self.values.v6address,
            ),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::V6DataPort,
                self.values.v6data_port,
            ),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::V6ServicePort,
                self.values.v6service_port,
            ),
        ]
    }
}

/// Typed transport acquisition rejection.
#[derive(Debug, Eq, PartialEq)]
pub enum StreamInfoTransportAcquisitionError<E> {
    /// The selected provider failed.
    Provider(E),
    /// Provider identity differed from the expected witness.
    ProviderIdentityMismatch,
    /// Epoch differed from the expected witness.
    EpochMismatch {
        /// Expected epoch.
        expected: u64,
        /// Returned epoch.
        actual: u64,
    },
    /// Revision differed from the expected witness.
    RevisionMismatch {
        /// Expected revision.
        expected: u64,
        /// Returned revision.
        actual: u64,
    },
    /// First oversized value in fixed transport role order.
    Value(StreamInfoVolatileFieldError),
}

/// Typed transport witness construction rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamInfoTransportEvidenceError {
    /// Provider identity bound was zero.
    InvalidProviderIdentityLimit,
    /// Provider identity was empty.
    EmptyProviderIdentity,
    /// Provider identity exceeded its scalar bound.
    ProviderIdentityLimitExceeded {
        /// Accepted maximum.
        expected_max: usize,
        /// Actual scalar count.
        actual: usize,
    },
}
impl<E: fmt::Debug> fmt::Display for StreamInfoTransportAcquisitionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transport acquisition rejected: {self:?}")
    }
}
impl<E: fmt::Debug> std::error::Error for StreamInfoTransportAcquisitionError<E> {}
impl fmt::Display for StreamInfoTransportEvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transport evidence rejected: {self:?}")
    }
}
impl std::error::Error for StreamInfoTransportEvidenceError {}

#[cfg(test)]
mod tests {
    use super::*;
    struct Provider {
        calls: usize,
        output: Result<StreamInfoTransportProviderOutput, &'static str>,
    }
    impl StreamInfoTransportProvider for Provider {
        type Error = &'static str;
        fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
            self.calls += 1;
            core::mem::replace(&mut self.output, Err("called twice"))
        }
    }
    fn witness(id: &str, epoch: u64, revision: u64) -> StreamInfoTransportWitness {
        StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(8).unwrap(),
            id.into(),
            epoch,
            revision,
        )
        .unwrap()
    }
    fn values(items: [&str; 6]) -> StreamInfoTransportValues {
        StreamInfoTransportValues::new(
            items[0].into(),
            items[1].into(),
            items[2].into(),
            items[3].into(),
            items[4].into(),
            items[5].into(),
        )
    }
    fn limits(max: usize) -> StreamInfoVolatileFieldLimits {
        StreamInfoVolatileFieldLimits::new(1, 1, max).unwrap()
    }
    #[test]
    fn one_call_shared_witness_and_allocations_are_preserved() {
        let v = values(["a", "b", "c", "d", "e", "f"]);
        let pointers = [
            v.v4address.as_ptr(),
            v.v4data_port.as_ptr(),
            v.v4service_port.as_ptr(),
            v.v6address.as_ptr(),
            v.v6data_port.as_ptr(),
            v.v6service_port.as_ptr(),
        ];
        let mut provider = Provider {
            calls: 0,
            output: Ok(StreamInfoTransportProviderOutput::new(
                witness("owner", 2, 3),
                v,
            )),
        };
        let accepted = StreamInfoTransportAcquisition::acquire(
            &mut provider,
            &witness("owner", 2, 3),
            limits(1),
        )
        .unwrap();
        assert_eq!(provider.calls, 1);
        assert_eq!(accepted.witness().revision(), 3);
        let lane = accepted.into_provider_values();
        assert_eq!(
            lane.iter()
                .map(StreamInfoVolatileProviderValue::role)
                .collect::<Vec<_>>(),
            [
                StreamInfoVolatileFieldRole::V4Address,
                StreamInfoVolatileFieldRole::V4DataPort,
                StreamInfoVolatileFieldRole::V4ServicePort,
                StreamInfoVolatileFieldRole::V6Address,
                StreamInfoVolatileFieldRole::V6DataPort,
                StreamInfoVolatileFieldRole::V6ServicePort
            ]
        );
        assert_eq!(
            lane.iter()
                .zip(pointers)
                .all(|(value, pointer)| value.value().as_ptr() == pointer),
            true
        );
    }
    #[test]
    fn provider_and_witness_failures_are_typed() {
        let mut failed = Provider {
            calls: 0,
            output: Err("offline"),
        };
        assert_eq!(
            StreamInfoTransportAcquisition::acquire(
                &mut failed,
                &witness("owner", 1, 2),
                limits(1)
            ),
            Err(StreamInfoTransportAcquisitionError::Provider("offline"))
        );
        for (returned, error) in [
            (
                witness("other", 1, 2),
                StreamInfoTransportAcquisitionError::ProviderIdentityMismatch,
            ),
            (
                witness("owner", 3, 2),
                StreamInfoTransportAcquisitionError::EpochMismatch {
                    expected: 1,
                    actual: 3,
                },
            ),
            (
                witness("owner", 1, 4),
                StreamInfoTransportAcquisitionError::RevisionMismatch {
                    expected: 2,
                    actual: 4,
                },
            ),
        ] {
            let mut provider = Provider {
                calls: 0,
                output: Ok(StreamInfoTransportProviderOutput::new(
                    returned,
                    values(["", "", "", "", "", ""]),
                )),
            };
            assert_eq!(
                StreamInfoTransportAcquisition::acquire(
                    &mut provider,
                    &witness("owner", 1, 2),
                    limits(1)
                ),
                Err(error)
            );
        }
    }
    #[test]
    fn transport_value_bounds_reject_in_fixed_role_order() {
        for (items, role) in [
            (
                ["xx", "xx", "xx", "xx", "xx", "xx"],
                StreamInfoVolatileFieldRole::V4Address,
            ),
            (
                ["x", "xx", "xx", "xx", "xx", "xx"],
                StreamInfoVolatileFieldRole::V4DataPort,
            ),
            (
                ["x", "x", "xx", "xx", "xx", "xx"],
                StreamInfoVolatileFieldRole::V4ServicePort,
            ),
            (
                ["x", "x", "x", "xx", "xx", "xx"],
                StreamInfoVolatileFieldRole::V6Address,
            ),
            (
                ["x", "x", "x", "x", "xx", "xx"],
                StreamInfoVolatileFieldRole::V6DataPort,
            ),
            (
                ["x", "x", "x", "x", "x", "éx"],
                StreamInfoVolatileFieldRole::V6ServicePort,
            ),
        ] {
            let mut provider = Provider {
                calls: 0,
                output: Ok(StreamInfoTransportProviderOutput::new(
                    witness("owner", 1, 2),
                    values(items),
                )),
            };
            assert_eq!(
                StreamInfoTransportAcquisition::acquire(
                    &mut provider,
                    &witness("owner", 1, 2),
                    limits(1)
                ),
                Err(StreamInfoTransportAcquisitionError::Value(
                    StreamInfoVolatileFieldError::TextLimitExceeded {
                        role,
                        expected_max: 1,
                        actual: 2
                    }
                ))
            );
        }
    }
    #[test]
    fn evidence_limits_are_explicit() {
        assert_eq!(
            StreamInfoTransportEvidenceLimit::new(0),
            Err(StreamInfoTransportEvidenceError::InvalidProviderIdentityLimit)
        );
        let limit = StreamInfoTransportEvidenceLimit::new(1).unwrap();
        assert_eq!(
            StreamInfoTransportWitness::new(limit, "".into(), 0, 0),
            Err(StreamInfoTransportEvidenceError::EmptyProviderIdentity)
        );
        assert_eq!(
            StreamInfoTransportWitness::new(limit, "éx".into(), 0, 0),
            Err(
                StreamInfoTransportEvidenceError::ProviderIdentityLimitExceeded {
                    expected_max: 1,
                    actual: 2
                }
            )
        );
    }
}
