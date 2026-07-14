// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{
    StreamInfoVolatileFieldError, StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole,
    StreamInfoVolatileProviderValue,
};

/// Bound for the runtime provider identity retained as owner evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoRuntimeEvidenceLimit {
    max_provider_identity_code_points: usize,
}
impl StreamInfoRuntimeEvidenceLimit {
    /// Creates a nonzero provider-identity scalar bound.
    pub fn new(
        max_provider_identity_code_points: usize,
    ) -> Result<Self, StreamInfoRuntimeEvidenceError> {
        if max_provider_identity_code_points == 0 {
            return Err(StreamInfoRuntimeEvidenceError::InvalidProviderIdentityLimit);
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

/// One owner-issued witness shared by all four runtime-assigned values.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoRuntimeWitness {
    provider_identity: String,
    epoch: u64,
    revision: u64,
}
impl StreamInfoRuntimeWitness {
    /// Validates and retains the provider identity and owner-issued epoch/revision.
    pub fn new(
        limit: StreamInfoRuntimeEvidenceLimit,
        provider_identity: String,
        epoch: u64,
        revision: u64,
    ) -> Result<Self, StreamInfoRuntimeEvidenceError> {
        let actual = provider_identity.chars().count();
        if actual == 0 {
            return Err(StreamInfoRuntimeEvidenceError::EmptyProviderIdentity);
        }
        if actual > limit.max_provider_identity_code_points {
            return Err(
                StreamInfoRuntimeEvidenceError::ProviderIdentityLimitExceeded {
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

/// Four opaque values acquired together from one runtime owner.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoRuntimeValues {
    created_at: String,
    uid: String,
    session_id: String,
    hostname: String,
}
impl StreamInfoRuntimeValues {
    /// Groups the four runtime values without interpreting them.
    #[must_use]
    pub const fn new(
        created_at: String,
        uid: String,
        session_id: String,
        hostname: String,
    ) -> Self {
        Self {
            created_at,
            uid,
            session_id,
            hostname,
        }
    }
    /// Returns opaque creation text.
    #[must_use]
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    /// Returns opaque identity text.
    #[must_use]
    pub fn uid(&self) -> &str {
        &self.uid
    }
    /// Returns opaque session text.
    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    /// Returns opaque host text.
    #[must_use]
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    /// Moves all four original allocations out in fixed runtime role order.
    #[must_use]
    pub fn into_parts(self) -> (String, String, String, String) {
        (self.created_at, self.uid, self.session_id, self.hostname)
    }
}

/// One provider result containing one shared witness and all four values.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoRuntimeProviderOutput {
    witness: StreamInfoRuntimeWitness,
    values: StreamInfoRuntimeValues,
}
impl StreamInfoRuntimeProviderOutput {
    /// Groups provider output without claiming its evidence is expected.
    #[must_use]
    pub const fn new(witness: StreamInfoRuntimeWitness, values: StreamInfoRuntimeValues) -> Self {
        Self { witness, values }
    }
}

/// A caller-selected synchronous provider for all runtime-assigned values.
pub trait StreamInfoRuntimeProvider {
    /// Provider-owned failure returned unchanged.
    type Error;
    /// Acquires all four values and their one shared owner witness.
    fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error>;
}

/// Accepted runtime-lane acquisition with separately inspectable evidence.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoRuntimeAcquisition {
    witness: StreamInfoRuntimeWitness,
    values: StreamInfoRuntimeValues,
}
impl StreamInfoRuntimeAcquisition {
    /// Calls one selected provider once, matches its witness, then applies O bounds in role order.
    pub fn acquire<P: StreamInfoRuntimeProvider>(
        provider: &mut P,
        expected: &StreamInfoRuntimeWitness,
        limits: StreamInfoVolatileFieldLimits,
    ) -> Result<Self, StreamInfoRuntimeAcquisitionError<P::Error>> {
        let output = provider
            .acquire()
            .map_err(StreamInfoRuntimeAcquisitionError::Provider)?;
        if output.witness.provider_identity != expected.provider_identity {
            return Err(StreamInfoRuntimeAcquisitionError::ProviderIdentityMismatch);
        }
        if output.witness.epoch != expected.epoch {
            return Err(StreamInfoRuntimeAcquisitionError::EpochMismatch {
                expected: expected.epoch,
                actual: output.witness.epoch,
            });
        }
        if output.witness.revision != expected.revision {
            return Err(StreamInfoRuntimeAcquisitionError::RevisionMismatch {
                expected: expected.revision,
                actual: output.witness.revision,
            });
        }
        let maximum = limits.max_runtime_code_points();
        for (role, value) in [
            (
                StreamInfoVolatileFieldRole::CreatedAt,
                output.values.created_at.as_str(),
            ),
            (StreamInfoVolatileFieldRole::Uid, output.values.uid.as_str()),
            (
                StreamInfoVolatileFieldRole::SessionId,
                output.values.session_id.as_str(),
            ),
            (
                StreamInfoVolatileFieldRole::Hostname,
                output.values.hostname.as_str(),
            ),
        ] {
            let actual = value.chars().count();
            if actual > maximum {
                return Err(StreamInfoRuntimeAcquisitionError::Value(
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
    pub const fn witness(&self) -> &StreamInfoRuntimeWitness {
        &self.witness
    }
    /// Returns all four opaque values.
    #[must_use]
    pub const fn values(&self) -> &StreamInfoRuntimeValues {
        &self.values
    }
    /// Moves the separately retained witness and grouped runtime values apart.
    #[must_use]
    pub fn into_parts(self) -> (StreamInfoRuntimeWitness, StreamInfoRuntimeValues) {
        (self.witness, self.values)
    }
    /// Moves the four original allocations into exactly the LSLC-001S runtime lane.
    #[must_use]
    pub fn into_provider_values(self) -> [StreamInfoVolatileProviderValue; 4] {
        [
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::CreatedAt,
                self.values.created_at,
            ),
            StreamInfoVolatileProviderValue::new(StreamInfoVolatileFieldRole::Uid, self.values.uid),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::SessionId,
                self.values.session_id,
            ),
            StreamInfoVolatileProviderValue::new(
                StreamInfoVolatileFieldRole::Hostname,
                self.values.hostname,
            ),
        ]
    }
}

/// Typed runtime acquisition rejection.
#[derive(Debug, Eq, PartialEq)]
pub enum StreamInfoRuntimeAcquisitionError<E> {
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
    /// First oversized value in fixed runtime role order.
    Value(StreamInfoVolatileFieldError),
}

/// Typed runtime witness construction rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamInfoRuntimeEvidenceError {
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
impl<E: fmt::Debug> fmt::Display for StreamInfoRuntimeAcquisitionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "runtime acquisition rejected: {self:?}")
    }
}
impl<E: fmt::Debug> std::error::Error for StreamInfoRuntimeAcquisitionError<E> {}
impl fmt::Display for StreamInfoRuntimeEvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "runtime evidence rejected: {self:?}")
    }
}
impl std::error::Error for StreamInfoRuntimeEvidenceError {}

#[cfg(test)]
mod tests {
    use super::*;
    struct Provider {
        calls: usize,
        output: Result<StreamInfoRuntimeProviderOutput, &'static str>,
    }
    impl StreamInfoRuntimeProvider for Provider {
        type Error = &'static str;
        fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error> {
            self.calls += 1;
            core::mem::replace(&mut self.output, Err("called twice"))
        }
    }
    fn witness(id: &str, epoch: u64, revision: u64) -> StreamInfoRuntimeWitness {
        StreamInfoRuntimeWitness::new(
            StreamInfoRuntimeEvidenceLimit::new(8).unwrap(),
            id.into(),
            epoch,
            revision,
        )
        .unwrap()
    }
    fn values(items: [&str; 4]) -> StreamInfoRuntimeValues {
        StreamInfoRuntimeValues::new(
            items[0].into(),
            items[1].into(),
            items[2].into(),
            items[3].into(),
        )
    }
    fn limits(max: usize) -> StreamInfoVolatileFieldLimits {
        StreamInfoVolatileFieldLimits::new(1, max, 1).unwrap()
    }
    #[test]
    fn one_call_shared_witness_and_allocations_are_preserved() {
        let v = values(["t", "u", "s", "h"]);
        let pointers = [
            v.created_at.as_ptr(),
            v.uid.as_ptr(),
            v.session_id.as_ptr(),
            v.hostname.as_ptr(),
        ];
        let mut provider = Provider {
            calls: 0,
            output: Ok(StreamInfoRuntimeProviderOutput::new(
                witness("owner", 2, 3),
                v,
            )),
        };
        let accepted = StreamInfoRuntimeAcquisition::acquire(
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
                StreamInfoVolatileFieldRole::CreatedAt,
                StreamInfoVolatileFieldRole::Uid,
                StreamInfoVolatileFieldRole::SessionId,
                StreamInfoVolatileFieldRole::Hostname
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
            StreamInfoRuntimeAcquisition::acquire(&mut failed, &witness("owner", 1, 2), limits(1)),
            Err(StreamInfoRuntimeAcquisitionError::Provider("offline"))
        );
        for (returned, error) in [
            (
                witness("other", 1, 2),
                StreamInfoRuntimeAcquisitionError::ProviderIdentityMismatch,
            ),
            (
                witness("owner", 3, 2),
                StreamInfoRuntimeAcquisitionError::EpochMismatch {
                    expected: 1,
                    actual: 3,
                },
            ),
            (
                witness("owner", 1, 4),
                StreamInfoRuntimeAcquisitionError::RevisionMismatch {
                    expected: 2,
                    actual: 4,
                },
            ),
        ] {
            let mut provider = Provider {
                calls: 0,
                output: Ok(StreamInfoRuntimeProviderOutput::new(
                    returned,
                    values(["", "", "", ""]),
                )),
            };
            assert_eq!(
                StreamInfoRuntimeAcquisition::acquire(
                    &mut provider,
                    &witness("owner", 1, 2),
                    limits(1)
                ),
                Err(error)
            );
        }
    }
    #[test]
    fn runtime_value_bounds_reject_in_fixed_role_order() {
        for (items, role) in [
            (
                ["xx", "xx", "xx", "xx"],
                StreamInfoVolatileFieldRole::CreatedAt,
            ),
            (["x", "xx", "xx", "xx"], StreamInfoVolatileFieldRole::Uid),
            (
                ["x", "x", "xx", "xx"],
                StreamInfoVolatileFieldRole::SessionId,
            ),
            (["x", "x", "x", "éx"], StreamInfoVolatileFieldRole::Hostname),
        ] {
            let mut provider = Provider {
                calls: 0,
                output: Ok(StreamInfoRuntimeProviderOutput::new(
                    witness("owner", 1, 2),
                    values(items),
                )),
            };
            assert_eq!(
                StreamInfoRuntimeAcquisition::acquire(
                    &mut provider,
                    &witness("owner", 1, 2),
                    limits(1)
                ),
                Err(StreamInfoRuntimeAcquisitionError::Value(
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
            StreamInfoRuntimeEvidenceLimit::new(0),
            Err(StreamInfoRuntimeEvidenceError::InvalidProviderIdentityLimit)
        );
        let limit = StreamInfoRuntimeEvidenceLimit::new(1).unwrap();
        assert_eq!(
            StreamInfoRuntimeWitness::new(limit, "".into(), 0, 0),
            Err(StreamInfoRuntimeEvidenceError::EmptyProviderIdentity)
        );
        assert_eq!(
            StreamInfoRuntimeWitness::new(limit, "éx".into(), 0, 0),
            Err(
                StreamInfoRuntimeEvidenceError::ProviderIdentityLimitExceeded {
                    expected_max: 1,
                    actual: 2
                }
            )
        );
    }
}
