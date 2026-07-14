// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{
    StreamInfoVolatileFieldError, StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole,
    StreamInfoVolatileProviderValue,
};

/// Bounds the owner identity retained in implementation-version evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoImplementationVersionEvidenceLimit {
    max_provider_identity_code_points: usize,
}

impl StreamInfoImplementationVersionEvidenceLimit {
    /// Creates a nonzero provider-identity scalar-value bound.
    pub fn new(
        max_provider_identity_code_points: usize,
    ) -> Result<Self, StreamInfoImplementationVersionEvidenceError> {
        if max_provider_identity_code_points == 0 {
            return Err(StreamInfoImplementationVersionEvidenceError::InvalidProviderIdentityLimit);
        }
        Ok(Self {
            max_provider_identity_code_points,
        })
    }

    /// Returns the provider-identity scalar-value bound.
    #[must_use]
    pub const fn max_provider_identity_code_points(self) -> usize {
        self.max_provider_identity_code_points
    }
}

/// An explicit owner-issued witness naming one provider state.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoImplementationVersionWitness {
    provider_identity: String,
    epoch: u64,
    revision: u64,
}

impl StreamInfoImplementationVersionWitness {
    /// Validates and retains an owner-issued provider identity, epoch, and revision.
    pub fn new(
        limit: StreamInfoImplementationVersionEvidenceLimit,
        provider_identity: String,
        epoch: u64,
        revision: u64,
    ) -> Result<Self, StreamInfoImplementationVersionEvidenceError> {
        let actual = provider_identity.chars().count();
        if actual == 0 {
            return Err(StreamInfoImplementationVersionEvidenceError::EmptyProviderIdentity);
        }
        if actual > limit.max_provider_identity_code_points {
            return Err(
                StreamInfoImplementationVersionEvidenceError::ProviderIdentityLimitExceeded {
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
    /// Returns the owner-issued revision within that epoch.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

/// One provider-produced version value and its separate owner-issued witness.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoImplementationVersionProviderOutput {
    witness: StreamInfoImplementationVersionWitness,
    version: String,
}

impl StreamInfoImplementationVersionProviderOutput {
    /// Groups provider output without claiming that its witness is expected or current.
    #[must_use]
    pub const fn new(witness: StreamInfoImplementationVersionWitness, version: String) -> Self {
        Self { witness, version }
    }
    /// Returns the provider-produced witness.
    #[must_use]
    pub const fn witness(&self) -> &StreamInfoImplementationVersionWitness {
        &self.witness
    }
    /// Returns the opaque implementation version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// A caller-selected synchronous provider for one implementation version.
pub trait StreamInfoImplementationVersionProvider {
    /// Provider-owned failure returned unchanged by the adapter.
    type Error;
    /// Acquires one version and separate owner-issued evidence.
    fn acquire(&mut self) -> Result<StreamInfoImplementationVersionProviderOutput, Self::Error>;
}

/// One accepted version acquisition whose evidence exactly matched the expected owner witness.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoImplementationVersionAcquisition {
    witness: StreamInfoImplementationVersionWitness,
    version: String,
}

impl StreamInfoImplementationVersionAcquisition {
    /// Invokes one caller-selected provider once and validates exact owner evidence and the O implementation bound.
    pub fn acquire<P: StreamInfoImplementationVersionProvider>(
        provider: &mut P,
        expected: &StreamInfoImplementationVersionWitness,
        volatile_limits: StreamInfoVolatileFieldLimits,
    ) -> Result<Self, StreamInfoImplementationVersionAcquisitionError<P::Error>> {
        let output = provider
            .acquire()
            .map_err(StreamInfoImplementationVersionAcquisitionError::Provider)?;
        if output.witness.provider_identity != expected.provider_identity {
            return Err(StreamInfoImplementationVersionAcquisitionError::ProviderIdentityMismatch);
        }
        if output.witness.epoch != expected.epoch {
            return Err(
                StreamInfoImplementationVersionAcquisitionError::EpochMismatch {
                    expected: expected.epoch,
                    actual: output.witness.epoch,
                },
            );
        }
        if output.witness.revision != expected.revision {
            return Err(
                StreamInfoImplementationVersionAcquisitionError::RevisionMismatch {
                    expected: expected.revision,
                    actual: output.witness.revision,
                },
            );
        }
        let actual = output.version.chars().count();
        let expected_max = volatile_limits.max_implementation_code_points();
        if actual > expected_max {
            return Err(StreamInfoImplementationVersionAcquisitionError::Version(
                StreamInfoVolatileFieldError::TextLimitExceeded {
                    role: StreamInfoVolatileFieldRole::Version,
                    expected_max,
                    actual,
                },
            ));
        }
        Ok(Self {
            witness: output.witness,
            version: output.version,
        })
    }

    /// Returns the exact matched owner-issued witness.
    #[must_use]
    pub const fn witness(&self) -> &StreamInfoImplementationVersionWitness {
        &self.witness
    }
    /// Returns the opaque acquired version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
    /// Moves only the version allocation into the LSLC-001S implementation lane.
    #[must_use]
    pub fn into_provider_value(self) -> StreamInfoVolatileProviderValue {
        StreamInfoVolatileProviderValue::new(StreamInfoVolatileFieldRole::Version, self.version)
    }
    /// Moves the separately retained witness and original version allocation apart.
    #[must_use]
    pub fn into_parts(self) -> (StreamInfoImplementationVersionWitness, String) {
        (self.witness, self.version)
    }
}

/// Rejection from explicit implementation-version acquisition or evidence admission.
#[derive(Debug, Eq, PartialEq)]
pub enum StreamInfoImplementationVersionAcquisitionError<E> {
    /// The selected provider failed.
    Provider(E),
    /// Returned owner identity did not match the expected witness.
    ProviderIdentityMismatch,
    /// Returned owner epoch did not match the expected witness.
    EpochMismatch {
        /// Expected epoch.
        expected: u64,
        /// Returned epoch.
        actual: u64,
    },
    /// Returned owner revision did not match the expected witness.
    RevisionMismatch {
        /// Expected revision.
        expected: u64,
        /// Returned revision.
        actual: u64,
    },
    /// The opaque version exceeded the existing implementation-assigned bound.
    Version(StreamInfoVolatileFieldError),
}

/// Rejection while constructing bounded owner evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamInfoImplementationVersionEvidenceError {
    /// The identity maximum was zero.
    InvalidProviderIdentityLimit,
    /// The provider identity was empty.
    EmptyProviderIdentity,
    /// The identity exceeded its scalar-value maximum.
    ProviderIdentityLimitExceeded {
        /// Accepted maximum.
        expected_max: usize,
        /// Actual scalar count.
        actual: usize,
    },
}

impl<E: fmt::Debug> fmt::Display for StreamInfoImplementationVersionAcquisitionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "implementation version acquisition rejected: {self:?}")
    }
}
impl<E: fmt::Debug> std::error::Error for StreamInfoImplementationVersionAcquisitionError<E> {}
impl fmt::Display for StreamInfoImplementationVersionEvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "implementation version evidence rejected: {self:?}")
    }
}
impl std::error::Error for StreamInfoImplementationVersionEvidenceError {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Provider {
        calls: usize,
        output: Result<StreamInfoImplementationVersionProviderOutput, &'static str>,
    }
    impl StreamInfoImplementationVersionProvider for Provider {
        type Error = &'static str;
        fn acquire(
            &mut self,
        ) -> Result<StreamInfoImplementationVersionProviderOutput, Self::Error> {
            self.calls += 1;
            core::mem::replace(&mut self.output, Err("called twice"))
        }
    }
    fn witness(id: &str, epoch: u64, revision: u64) -> StreamInfoImplementationVersionWitness {
        StreamInfoImplementationVersionWitness::new(
            StreamInfoImplementationVersionEvidenceLimit::new(8).unwrap(),
            id.to_owned(),
            epoch,
            revision,
        )
        .unwrap()
    }
    fn limits(max: usize) -> StreamInfoVolatileFieldLimits {
        StreamInfoVolatileFieldLimits::new(max, 1, 1).unwrap()
    }

    #[test]
    fn exact_owner_witness_acquires_once_and_preserves_allocation() {
        let version = String::from("1.2.3");
        let pointer = version.as_ptr();
        let mut provider = Provider {
            calls: 0,
            output: Ok(StreamInfoImplementationVersionProviderOutput::new(
                witness("owner", 7, 9),
                version,
            )),
        };
        let accepted = StreamInfoImplementationVersionAcquisition::acquire(
            &mut provider,
            &witness("owner", 7, 9),
            limits(5),
        )
        .unwrap();
        assert_eq!(provider.calls, 1);
        assert_eq!(accepted.version(), "1.2.3");
        assert_eq!(accepted.version.as_ptr(), pointer);
        let value = accepted.into_provider_value();
        assert_eq!(value.role(), StreamInfoVolatileFieldRole::Version);
        assert_eq!(value.value().as_ptr(), pointer);
    }

    #[test]
    fn provider_failure_and_each_witness_mismatch_are_typed() {
        let mut failed = Provider {
            calls: 0,
            output: Err("unavailable"),
        };
        assert_eq!(
            StreamInfoImplementationVersionAcquisition::acquire(
                &mut failed,
                &witness("owner", 1, 2),
                limits(1)
            ),
            Err(StreamInfoImplementationVersionAcquisitionError::Provider(
                "unavailable"
            ))
        );
        for (returned, error) in [
            (
                witness("other", 1, 2),
                StreamInfoImplementationVersionAcquisitionError::ProviderIdentityMismatch,
            ),
            (
                witness("owner", 3, 2),
                StreamInfoImplementationVersionAcquisitionError::EpochMismatch {
                    expected: 1,
                    actual: 3,
                },
            ),
            (
                witness("owner", 1, 4),
                StreamInfoImplementationVersionAcquisitionError::RevisionMismatch {
                    expected: 2,
                    actual: 4,
                },
            ),
        ] {
            let mut provider = Provider {
                calls: 0,
                output: Ok(StreamInfoImplementationVersionProviderOutput::new(
                    returned,
                    "v".into(),
                )),
            };
            assert_eq!(
                StreamInfoImplementationVersionAcquisition::acquire(
                    &mut provider,
                    &witness("owner", 1, 2),
                    limits(1)
                ),
                Err(error)
            );
        }
    }

    #[test]
    fn evidence_and_version_bounds_fail_closed() {
        assert_eq!(
            StreamInfoImplementationVersionEvidenceLimit::new(0),
            Err(StreamInfoImplementationVersionEvidenceError::InvalidProviderIdentityLimit)
        );
        let limit = StreamInfoImplementationVersionEvidenceLimit::new(1).unwrap();
        assert_eq!(
            StreamInfoImplementationVersionWitness::new(limit, String::new(), 0, 0),
            Err(StreamInfoImplementationVersionEvidenceError::EmptyProviderIdentity)
        );
        assert_eq!(
            StreamInfoImplementationVersionWitness::new(limit, "éx".into(), 0, 0),
            Err(
                StreamInfoImplementationVersionEvidenceError::ProviderIdentityLimitExceeded {
                    expected_max: 1,
                    actual: 2
                }
            )
        );
        let mut provider = Provider {
            calls: 0,
            output: Ok(StreamInfoImplementationVersionProviderOutput::new(
                witness("owner", 1, 2),
                "éx".into(),
            )),
        };
        assert_eq!(
            StreamInfoImplementationVersionAcquisition::acquire(
                &mut provider,
                &witness("owner", 1, 2),
                limits(1)
            ),
            Err(StreamInfoImplementationVersionAcquisitionError::Version(
                StreamInfoVolatileFieldError::TextLimitExceeded {
                    role: StreamInfoVolatileFieldRole::Version,
                    expected_max: 1,
                    actual: 2
                }
            ))
        );
    }
}
