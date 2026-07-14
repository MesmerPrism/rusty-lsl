// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use crate::{
    StreamInfoVolatileFieldClass, StreamInfoVolatileFieldError, StreamInfoVolatileFieldInput,
    StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole, StreamInfoVolatileFields,
};

/// One caller-owned opaque value labelled with its volatile stream-info role.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileProviderValue {
    role: StreamInfoVolatileFieldRole,
    value: String,
}

impl StreamInfoVolatileProviderValue {
    /// Groups one role and value without inspecting or changing the value.
    #[must_use]
    pub const fn new(role: StreamInfoVolatileFieldRole, value: String) -> Self {
        Self { role, value }
    }

    /// Returns the labelled role.
    #[must_use]
    pub const fn role(&self) -> StreamInfoVolatileFieldRole {
        self.role
    }

    /// Returns the unchanged opaque value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the original role and value allocation.
    #[must_use]
    pub fn into_parts(self) -> (StreamInfoVolatileFieldRole, String) {
        (self.role, self.value)
    }
}

/// A caller-supplied candidate snapshot separated into three ownership lanes.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileProviderSnapshotInput {
    implementation_assigned: Vec<StreamInfoVolatileProviderValue>,
    runtime_assigned: Vec<StreamInfoVolatileProviderValue>,
    transport_owned: Vec<StreamInfoVolatileProviderValue>,
}

impl StreamInfoVolatileProviderSnapshotInput {
    /// Groups three explicit lanes without acquiring or validating their values.
    #[must_use]
    pub const fn new(
        implementation_assigned: Vec<StreamInfoVolatileProviderValue>,
        runtime_assigned: Vec<StreamInfoVolatileProviderValue>,
        transport_owned: Vec<StreamInfoVolatileProviderValue>,
    ) -> Self {
        Self {
            implementation_assigned,
            runtime_assigned,
            transport_owned,
        }
    }

    /// Returns the implementation-assigned candidate lane.
    #[must_use]
    pub fn implementation_assigned(&self) -> &[StreamInfoVolatileProviderValue] {
        &self.implementation_assigned
    }

    /// Returns the runtime-assigned candidate lane.
    #[must_use]
    pub fn runtime_assigned(&self) -> &[StreamInfoVolatileProviderValue] {
        &self.runtime_assigned
    }

    /// Returns the transport-owned candidate lane.
    #[must_use]
    pub fn transport_owned(&self) -> &[StreamInfoVolatileProviderValue] {
        &self.transport_owned
    }
}

/// One complete, internally consistent, bounded caller-supplied snapshot.
///
/// Acceptance is one-shot data admission. It does not establish freshness,
/// currentness, acquisition, provider selection, runtime application, or
/// authority because this contract has no clock, revision, epoch, or witness.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileProviderSnapshot {
    fields: StreamInfoVolatileFields,
}

impl StreamInfoVolatileProviderSnapshot {
    /// Validates lane shape and delegates unchanged complete data to LSLC-001O.
    pub fn new(
        limits: StreamInfoVolatileFieldLimits,
        input: StreamInfoVolatileProviderSnapshotInput,
    ) -> Result<Self, StreamInfoVolatileProviderSnapshotError> {
        const LANES: [(StreamInfoVolatileFieldClass, usize); 3] = [
            (StreamInfoVolatileFieldClass::ImplementationAssigned, 1),
            (StreamInfoVolatileFieldClass::RuntimeAssigned, 4),
            (StreamInfoVolatileFieldClass::TransportOwned, 6),
        ];
        let actuals = [
            input.implementation_assigned.len(),
            input.runtime_assigned.len(),
            input.transport_owned.len(),
        ];
        for ((lane, expected_max), actual) in LANES.into_iter().zip(actuals) {
            if actual > expected_max {
                return Err(StreamInfoVolatileProviderSnapshotError::LaneLimitExceeded {
                    lane,
                    expected_max,
                    actual,
                });
            }
        }

        let mut values: [Option<String>; 11] = core::array::from_fn(|_| None);
        ingest_lane(
            &mut values,
            StreamInfoVolatileFieldClass::ImplementationAssigned,
            input.implementation_assigned,
        )?;
        ingest_lane(
            &mut values,
            StreamInfoVolatileFieldClass::RuntimeAssigned,
            input.runtime_assigned,
        )?;
        ingest_lane(
            &mut values,
            StreamInfoVolatileFieldClass::TransportOwned,
            input.transport_owned,
        )?;

        for (index, role) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .enumerate()
        {
            if values[index].is_none() {
                return Err(StreamInfoVolatileProviderSnapshotError::MissingRole { role });
            }
        }
        let [version, created_at, uid, session_id, hostname, v4address, v4data_port, v4service_port, v6address, v6data_port, v6service_port] =
            values.map(Option::unwrap);
        let fields = StreamInfoVolatileFields::new(
            limits,
            StreamInfoVolatileFieldInput::new(
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
            ),
        )?;
        Ok(Self { fields })
    }

    /// Returns the complete accepted LSLC-001O fields.
    #[must_use]
    pub const fn fields(&self) -> &StreamInfoVolatileFields {
        &self.fields
    }

    /// Returns the complete accepted LSLC-001O fields without reallocating.
    #[must_use]
    pub fn into_fields(self) -> StreamInfoVolatileFields {
        self.fields
    }
}

fn ingest_lane(
    values: &mut [Option<String>; 11],
    lane: StreamInfoVolatileFieldClass,
    entries: Vec<StreamInfoVolatileProviderValue>,
) -> Result<(), StreamInfoVolatileProviderSnapshotError> {
    for (entry_index, entry) in entries.into_iter().enumerate() {
        let (role, value) = entry.into_parts();
        let expected_lane = role.class();
        if expected_lane != lane {
            return Err(StreamInfoVolatileProviderSnapshotError::CrossLaneRole {
                lane,
                entry_index,
                role,
                expected_lane,
            });
        }
        let role_index = role_index(role);
        if values[role_index].is_some() {
            return Err(StreamInfoVolatileProviderSnapshotError::DuplicateRole {
                lane,
                entry_index,
                role,
            });
        }
        values[role_index] = Some(value);
    }
    Ok(())
}

const fn role_index(role: StreamInfoVolatileFieldRole) -> usize {
    match role {
        StreamInfoVolatileFieldRole::Version => 0,
        StreamInfoVolatileFieldRole::CreatedAt => 1,
        StreamInfoVolatileFieldRole::Uid => 2,
        StreamInfoVolatileFieldRole::SessionId => 3,
        StreamInfoVolatileFieldRole::Hostname => 4,
        StreamInfoVolatileFieldRole::V4Address => 5,
        StreamInfoVolatileFieldRole::V4DataPort => 6,
        StreamInfoVolatileFieldRole::V4ServicePort => 7,
        StreamInfoVolatileFieldRole::V6Address => 8,
        StreamInfoVolatileFieldRole::V6DataPort => 9,
        StreamInfoVolatileFieldRole::V6ServicePort => 10,
    }
}

/// Deterministic rejection from caller-supplied volatile snapshot admission.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoVolatileProviderSnapshotError {
    /// One lane exceeded the number of roles owned by that lane.
    LaneLimitExceeded {
        /// The first oversized lane in fixed ownership order.
        lane: StreamInfoVolatileFieldClass,
        /// The exact role count owned by the lane.
        expected_max: usize,
        /// The caller-provided item count.
        actual: usize,
    },
    /// An entry was supplied through a lane that does not own its role.
    CrossLaneRole {
        /// The lane containing the entry.
        lane: StreamInfoVolatileFieldClass,
        /// The zero-based entry index within that lane.
        entry_index: usize,
        /// The misplaced role.
        role: StreamInfoVolatileFieldRole,
        /// The role's fixed LSLC-001O ownership lane.
        expected_lane: StreamInfoVolatileFieldClass,
    },
    /// One role appeared twice in its owning lane.
    DuplicateRole {
        /// The owning lane.
        lane: StreamInfoVolatileFieldClass,
        /// The zero-based duplicate entry index.
        entry_index: usize,
        /// The repeated role.
        role: StreamInfoVolatileFieldRole,
    },
    /// One fixed-order LSLC-001O role was absent.
    MissingRole {
        /// The first absent role in observed order.
        role: StreamInfoVolatileFieldRole,
    },
    /// The complete snapshot failed the existing LSLC-001O text bounds.
    VolatileFields(StreamInfoVolatileFieldError),
}

impl From<StreamInfoVolatileFieldError> for StreamInfoVolatileProviderSnapshotError {
    fn from(error: StreamInfoVolatileFieldError) -> Self {
        Self::VolatileFields(error)
    }
}

impl fmt::Display for StreamInfoVolatileProviderSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "volatile provider snapshot rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoVolatileProviderSnapshotError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(role: StreamInfoVolatileFieldRole, text: &str) -> StreamInfoVolatileProviderValue {
        StreamInfoVolatileProviderValue::new(role, text.to_owned())
    }

    fn complete() -> StreamInfoVolatileProviderSnapshotInput {
        use StreamInfoVolatileFieldRole::*;
        StreamInfoVolatileProviderSnapshotInput::new(
            vec![value(Version, "v")],
            vec![
                value(CreatedAt, "t"),
                value(Uid, "u"),
                value(SessionId, "s"),
                value(Hostname, "h"),
            ],
            vec![
                value(V4Address, "opaque-a"),
                value(V4DataPort, "opaque-b"),
                value(V4ServicePort, "opaque-c"),
                value(V6Address, "opaque-d"),
                value(V6DataPort, "opaque-e"),
                value(V6ServicePort, "opaque-f"),
            ],
        )
    }

    #[test]
    fn lslc_001s_complete_snapshot_preserves_values_and_allocations() {
        let input = complete();
        let pointers: Vec<_> = input
            .implementation_assigned()
            .iter()
            .chain(input.runtime_assigned())
            .chain(input.transport_owned())
            .map(|entry| entry.value.as_ptr())
            .collect();
        let accepted = StreamInfoVolatileProviderSnapshot::new(
            StreamInfoVolatileFieldLimits::new(1, 1, 8).unwrap(),
            input,
        )
        .unwrap();
        for ((role, expected), pointer) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip([
                "v", "t", "u", "s", "h", "opaque-a", "opaque-b", "opaque-c", "opaque-d",
                "opaque-e", "opaque-f",
            ])
            .zip(pointers)
        {
            assert_eq!(accepted.fields().field(role), expected);
            assert_eq!(accepted.fields().field(role).as_ptr(), pointer);
        }
    }

    #[test]
    fn lslc_001s_lane_limits_reject_before_entry_inspection() {
        use StreamInfoVolatileFieldRole::{CreatedAt, Version};
        let input = StreamInfoVolatileProviderSnapshotInput::new(
            vec![value(CreatedAt, "x"), value(Version, "v")],
            vec![],
            vec![],
        );
        assert_eq!(
            StreamInfoVolatileProviderSnapshot::new(
                StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
                input
            ),
            Err(StreamInfoVolatileProviderSnapshotError::LaneLimitExceeded {
                lane: StreamInfoVolatileFieldClass::ImplementationAssigned,
                expected_max: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn lslc_001s_cross_lane_rejects_with_exact_location() {
        use StreamInfoVolatileFieldRole::CreatedAt;
        let input = StreamInfoVolatileProviderSnapshotInput::new(
            vec![value(CreatedAt, "t")],
            vec![],
            vec![],
        );
        assert_eq!(
            StreamInfoVolatileProviderSnapshot::new(
                StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
                input
            ),
            Err(StreamInfoVolatileProviderSnapshotError::CrossLaneRole {
                lane: StreamInfoVolatileFieldClass::ImplementationAssigned,
                entry_index: 0,
                role: CreatedAt,
                expected_lane: StreamInfoVolatileFieldClass::RuntimeAssigned
            })
        );
    }

    #[test]
    fn lslc_001s_duplicate_rejects_before_missing_role() {
        use StreamInfoVolatileFieldRole::{CreatedAt, Hostname, SessionId};
        let input = StreamInfoVolatileProviderSnapshotInput::new(
            vec![],
            vec![
                value(CreatedAt, "a"),
                value(CreatedAt, "b"),
                value(SessionId, "s"),
                value(Hostname, "h"),
            ],
            vec![],
        );
        assert_eq!(
            StreamInfoVolatileProviderSnapshot::new(
                StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
                input
            ),
            Err(StreamInfoVolatileProviderSnapshotError::DuplicateRole {
                lane: StreamInfoVolatileFieldClass::RuntimeAssigned,
                entry_index: 1,
                role: CreatedAt
            })
        );
    }

    #[test]
    fn lslc_001s_missing_role_uses_fixed_observed_order() {
        use StreamInfoVolatileFieldRole::Version;
        let input = StreamInfoVolatileProviderSnapshotInput::new(vec![], vec![], vec![]);
        assert_eq!(
            StreamInfoVolatileProviderSnapshot::new(
                StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
                input
            ),
            Err(StreamInfoVolatileProviderSnapshotError::MissingRole { role: Version })
        );
    }

    #[test]
    fn lslc_001s_complete_snapshot_delegates_unchanged_o_error() {
        use StreamInfoVolatileFieldRole::V4Address;
        assert_eq!(
            StreamInfoVolatileProviderSnapshot::new(
                StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
                complete()
            ),
            Err(StreamInfoVolatileProviderSnapshotError::VolatileFields(
                StreamInfoVolatileFieldError::TextLimitExceeded {
                    role: V4Address,
                    expected_max: 1,
                    actual: 8
                }
            ))
        );
    }
}
