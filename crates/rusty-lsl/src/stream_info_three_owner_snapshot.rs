// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    StreamInfoImplementationVersionAcquisition, StreamInfoImplementationVersionWitness,
    StreamInfoRuntimeAcquisition, StreamInfoRuntimeWitness, StreamInfoTransportAcquisition,
    StreamInfoTransportWitness, StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole,
    StreamInfoVolatileProviderSnapshot, StreamInfoVolatileProviderSnapshotError,
    StreamInfoVolatileProviderSnapshotInput, StreamInfoVolatileProviderValue,
};

/// The three unchanged owner witnesses retained beside one admitted snapshot.
///
/// These witnesses remain independent. This type does not compare or combine
/// their identities, epochs, or revisions and grants no cross-owner atomicity.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoThreeOwnerEvidence {
    implementation: StreamInfoImplementationVersionWitness,
    runtime: StreamInfoRuntimeWitness,
    transport: StreamInfoTransportWitness,
}

impl StreamInfoThreeOwnerEvidence {
    /// Returns the implementation owner's exact accepted witness.
    #[must_use]
    pub const fn implementation(&self) -> &StreamInfoImplementationVersionWitness {
        &self.implementation
    }

    /// Returns the runtime owner's exact accepted witness.
    #[must_use]
    pub const fn runtime(&self) -> &StreamInfoRuntimeWitness {
        &self.runtime
    }

    /// Returns the transport owner's exact accepted witness.
    #[must_use]
    pub const fn transport(&self) -> &StreamInfoTransportWitness {
        &self.transport
    }

    /// Moves all three independent witnesses out without reallocating them.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        StreamInfoImplementationVersionWitness,
        StreamInfoRuntimeWitness,
        StreamInfoTransportWitness,
    ) {
        (self.implementation, self.runtime, self.transport)
    }
}

/// One complete LSLC-001S snapshot paired with its three separate witnesses.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoThreeOwnerSnapshot {
    evidence: StreamInfoThreeOwnerEvidence,
    snapshot: StreamInfoVolatileProviderSnapshot,
}

impl StreamInfoThreeOwnerSnapshot {
    /// Consumes accepted T, U, and V acquisitions into one S admission.
    ///
    /// The caller supplies the S bounds. No provider is called and no relation
    /// among the three witnesses is inferred.
    pub fn new(
        limits: StreamInfoVolatileFieldLimits,
        implementation: StreamInfoImplementationVersionAcquisition,
        runtime: StreamInfoRuntimeAcquisition,
        transport: StreamInfoTransportAcquisition,
    ) -> Result<Self, StreamInfoVolatileProviderSnapshotError> {
        let (implementation_witness, version) = implementation.into_parts();
        let (runtime_witness, runtime_values) = runtime.into_parts();
        let (created_at, uid, session_id, hostname) = runtime_values.into_parts();
        let (transport_witness, transport_values) = transport.into_parts();
        let (v4address, v4data_port, v4service_port, v6address, v6data_port, v6service_port) =
            transport_values.into_parts();

        let snapshot = StreamInfoVolatileProviderSnapshot::new(
            limits,
            StreamInfoVolatileProviderSnapshotInput::new(
                vec![StreamInfoVolatileProviderValue::new(
                    StreamInfoVolatileFieldRole::Version,
                    version,
                )],
                vec![
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::CreatedAt,
                        created_at,
                    ),
                    StreamInfoVolatileProviderValue::new(StreamInfoVolatileFieldRole::Uid, uid),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::SessionId,
                        session_id,
                    ),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::Hostname,
                        hostname,
                    ),
                ],
                vec![
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::V4Address,
                        v4address,
                    ),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::V4DataPort,
                        v4data_port,
                    ),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::V4ServicePort,
                        v4service_port,
                    ),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::V6Address,
                        v6address,
                    ),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::V6DataPort,
                        v6data_port,
                    ),
                    StreamInfoVolatileProviderValue::new(
                        StreamInfoVolatileFieldRole::V6ServicePort,
                        v6service_port,
                    ),
                ],
            ),
        )?;

        Ok(Self {
            evidence: StreamInfoThreeOwnerEvidence {
                implementation: implementation_witness,
                runtime: runtime_witness,
                transport: transport_witness,
            },
            snapshot,
        })
    }

    /// Returns the three separately retained owner witnesses.
    #[must_use]
    pub const fn evidence(&self) -> &StreamInfoThreeOwnerEvidence {
        &self.evidence
    }

    /// Returns the complete accepted LSLC-001S snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> &StreamInfoVolatileProviderSnapshot {
        &self.snapshot
    }

    /// Moves the separate evidence and admitted snapshot out unchanged.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        StreamInfoThreeOwnerEvidence,
        StreamInfoVolatileProviderSnapshot,
    ) {
        (self.evidence, self.snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        StreamInfoImplementationVersionAcquisitionError,
        StreamInfoImplementationVersionEvidenceLimit, StreamInfoImplementationVersionProvider,
        StreamInfoImplementationVersionProviderOutput, StreamInfoRuntimeAcquisitionError,
        StreamInfoRuntimeEvidenceLimit, StreamInfoRuntimeProvider, StreamInfoRuntimeProviderOutput,
        StreamInfoRuntimeValues, StreamInfoTransportAcquisitionError,
        StreamInfoTransportEvidenceLimit, StreamInfoTransportProvider,
        StreamInfoTransportProviderOutput, StreamInfoTransportValues, StreamInfoVolatileFieldError,
    };
    use std::{cell::RefCell, rc::Rc};

    struct ImplementationProvider(Option<StreamInfoImplementationVersionProviderOutput>);
    impl StreamInfoImplementationVersionProvider for ImplementationProvider {
        type Error = ();
        fn acquire(
            &mut self,
        ) -> Result<StreamInfoImplementationVersionProviderOutput, Self::Error> {
            self.0.take().ok_or(())
        }
    }
    struct RuntimeProvider(Option<StreamInfoRuntimeProviderOutput>);
    impl StreamInfoRuntimeProvider for RuntimeProvider {
        type Error = ();
        fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error> {
            self.0.take().ok_or(())
        }
    }
    struct TransportProvider(Option<StreamInfoTransportProviderOutput>);
    impl StreamInfoTransportProvider for TransportProvider {
        type Error = ();
        fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
            self.0.take().ok_or(())
        }
    }

    fn limits(max: usize) -> StreamInfoVolatileFieldLimits {
        StreamInfoVolatileFieldLimits::new(max, max, max).unwrap()
    }

    fn acquisitions() -> (
        StreamInfoImplementationVersionAcquisition,
        StreamInfoRuntimeAcquisition,
        StreamInfoTransportAcquisition,
        [*const u8; 11],
    ) {
        let implementation_witness = StreamInfoImplementationVersionWitness::new(
            StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
            "implementation-owner".into(),
            1,
            11,
        )
        .unwrap();
        let version = "version".to_owned();
        let version_pointer = version.as_ptr();
        let mut implementation_provider =
            ImplementationProvider(Some(StreamInfoImplementationVersionProviderOutput::new(
                StreamInfoImplementationVersionWitness::new(
                    StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
                    "implementation-owner".into(),
                    1,
                    11,
                )
                .unwrap(),
                version,
            )));
        let implementation = StreamInfoImplementationVersionAcquisition::acquire(
            &mut implementation_provider,
            &implementation_witness,
            limits(32),
        )
        .unwrap();

        let runtime_witness = StreamInfoRuntimeWitness::new(
            StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
            "runtime-owner".into(),
            2,
            22,
        )
        .unwrap();
        let runtime_values = StreamInfoRuntimeValues::new(
            "created".into(),
            "uid".into(),
            "session".into(),
            "host".into(),
        );
        let runtime_pointers = [
            runtime_values.created_at().as_ptr(),
            runtime_values.uid().as_ptr(),
            runtime_values.session_id().as_ptr(),
            runtime_values.hostname().as_ptr(),
        ];
        let mut runtime_provider = RuntimeProvider(Some(StreamInfoRuntimeProviderOutput::new(
            StreamInfoRuntimeWitness::new(
                StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
                "runtime-owner".into(),
                2,
                22,
            )
            .unwrap(),
            runtime_values,
        )));
        let runtime = StreamInfoRuntimeAcquisition::acquire(
            &mut runtime_provider,
            &runtime_witness,
            limits(32),
        )
        .unwrap();

        let transport_witness = StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(32).unwrap(),
            "transport-owner".into(),
            3,
            33,
        )
        .unwrap();
        let transport_values = StreamInfoTransportValues::new(
            "v4".into(),
            "v4data".into(),
            "v4service".into(),
            "v6".into(),
            "v6data".into(),
            "v6service".into(),
        );
        let transport_pointers = [
            transport_values.v4address().as_ptr(),
            transport_values.v4data_port().as_ptr(),
            transport_values.v4service_port().as_ptr(),
            transport_values.v6address().as_ptr(),
            transport_values.v6data_port().as_ptr(),
            transport_values.v6service_port().as_ptr(),
        ];
        let mut transport_provider =
            TransportProvider(Some(StreamInfoTransportProviderOutput::new(
                StreamInfoTransportWitness::new(
                    StreamInfoTransportEvidenceLimit::new(32).unwrap(),
                    "transport-owner".into(),
                    3,
                    33,
                )
                .unwrap(),
                transport_values,
            )));
        let transport = StreamInfoTransportAcquisition::acquire(
            &mut transport_provider,
            &transport_witness,
            limits(32),
        )
        .unwrap();

        let pointers = [
            version_pointer,
            runtime_pointers[0],
            runtime_pointers[1],
            runtime_pointers[2],
            runtime_pointers[3],
            transport_pointers[0],
            transport_pointers[1],
            transport_pointers[2],
            transport_pointers[3],
            transport_pointers[4],
            transport_pointers[5],
        ];
        (implementation, runtime, transport, pointers)
    }

    #[test]
    fn three_witnesses_remain_separate_without_cross_owner_matching() {
        let (implementation, runtime, transport, _) = acquisitions();
        let accepted =
            StreamInfoThreeOwnerSnapshot::new(limits(32), implementation, runtime, transport)
                .unwrap();
        assert_eq!(
            accepted.evidence().implementation().provider_identity(),
            "implementation-owner"
        );
        assert_eq!(accepted.evidence().implementation().epoch(), 1);
        assert_eq!(
            accepted.evidence().runtime().provider_identity(),
            "runtime-owner"
        );
        assert_eq!(accepted.evidence().runtime().epoch(), 2);
        assert_eq!(
            accepted.evidence().transport().provider_identity(),
            "transport-owner"
        );
        assert_eq!(accepted.evidence().transport().epoch(), 3);
    }

    #[test]
    fn all_value_allocations_move_into_complete_s_snapshot() {
        let (implementation, runtime, transport, pointers) = acquisitions();
        let accepted =
            StreamInfoThreeOwnerSnapshot::new(limits(32), implementation, runtime, transport)
                .unwrap();
        for (role, pointer) in crate::StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip(pointers)
        {
            assert_eq!(accepted.snapshot().fields().field(role).as_ptr(), pointer);
        }
    }

    #[test]
    fn tighter_s_limit_rejection_is_delegated_unchanged() {
        let (implementation, runtime, transport, _) = acquisitions();
        assert_eq!(
            StreamInfoThreeOwnerSnapshot::new(limits(1), implementation, runtime, transport),
            Err(StreamInfoVolatileProviderSnapshotError::VolatileFields(
                StreamInfoVolatileFieldError::TextLimitExceeded {
                    role: StreamInfoVolatileFieldRole::Version,
                    expected_max: 1,
                    actual: 7,
                }
            ))
        );
    }

    #[test]
    fn consuming_parts_preserve_evidence_and_snapshot() {
        let (implementation, runtime, transport, _) = acquisitions();
        let accepted =
            StreamInfoThreeOwnerSnapshot::new(limits(32), implementation, runtime, transport)
                .unwrap();
        let (evidence, snapshot) = accepted.into_parts();
        let (implementation, runtime, transport) = evidence.into_parts();
        assert_eq!(implementation.revision(), 11);
        assert_eq!(runtime.revision(), 22);
        assert_eq!(transport.revision(), 33);
        assert_eq!(
            snapshot.fields().field(StreamInfoVolatileFieldRole::Uid),
            "uid"
        );
    }

    #[test]
    fn caller_selected_acquisition_order_is_observed_once_per_owner() {
        struct OrderedImplementationProvider(Rc<RefCell<Vec<&'static str>>>);
        impl StreamInfoImplementationVersionProvider for OrderedImplementationProvider {
            type Error = ();
            fn acquire(
                &mut self,
            ) -> Result<StreamInfoImplementationVersionProviderOutput, Self::Error> {
                self.0.borrow_mut().push("implementation");
                Ok(StreamInfoImplementationVersionProviderOutput::new(
                    StreamInfoImplementationVersionWitness::new(
                        StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
                        "implementation-owner".into(),
                        1,
                        11,
                    )
                    .unwrap(),
                    "version".into(),
                ))
            }
        }
        struct OrderedRuntimeProvider(Rc<RefCell<Vec<&'static str>>>);
        impl StreamInfoRuntimeProvider for OrderedRuntimeProvider {
            type Error = ();
            fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error> {
                self.0.borrow_mut().push("runtime");
                Ok(StreamInfoRuntimeProviderOutput::new(
                    StreamInfoRuntimeWitness::new(
                        StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
                        "runtime-owner".into(),
                        2,
                        22,
                    )
                    .unwrap(),
                    StreamInfoRuntimeValues::new(
                        "created".into(),
                        "uid".into(),
                        "session".into(),
                        "host".into(),
                    ),
                ))
            }
        }
        struct OrderedTransportProvider(Rc<RefCell<Vec<&'static str>>>);
        impl StreamInfoTransportProvider for OrderedTransportProvider {
            type Error = ();
            fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
                self.0.borrow_mut().push("transport");
                Ok(StreamInfoTransportProviderOutput::new(
                    StreamInfoTransportWitness::new(
                        StreamInfoTransportEvidenceLimit::new(32).unwrap(),
                        "transport-owner".into(),
                        3,
                        33,
                    )
                    .unwrap(),
                    StreamInfoTransportValues::new(
                        "v4".into(),
                        "v4data".into(),
                        "v4service".into(),
                        "v6".into(),
                        "v6data".into(),
                        "v6service".into(),
                    ),
                ))
            }
        }

        let order = Rc::new(RefCell::new(Vec::new()));
        let mut transport_provider = OrderedTransportProvider(Rc::clone(&order));
        let mut implementation_provider = OrderedImplementationProvider(Rc::clone(&order));
        let mut runtime_provider = OrderedRuntimeProvider(Rc::clone(&order));
        let transport_expected = StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(32).unwrap(),
            "transport-owner".into(),
            3,
            33,
        )
        .unwrap();
        let implementation_expected = StreamInfoImplementationVersionWitness::new(
            StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
            "implementation-owner".into(),
            1,
            11,
        )
        .unwrap();
        let runtime_expected = StreamInfoRuntimeWitness::new(
            StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
            "runtime-owner".into(),
            2,
            22,
        )
        .unwrap();

        let transport = StreamInfoTransportAcquisition::acquire(
            &mut transport_provider,
            &transport_expected,
            limits(32),
        )
        .unwrap();
        let implementation = StreamInfoImplementationVersionAcquisition::acquire(
            &mut implementation_provider,
            &implementation_expected,
            limits(32),
        )
        .unwrap();
        let runtime = StreamInfoRuntimeAcquisition::acquire(
            &mut runtime_provider,
            &runtime_expected,
            limits(32),
        )
        .unwrap();
        let accepted =
            StreamInfoThreeOwnerSnapshot::new(limits(32), implementation, runtime, transport)
                .unwrap();

        assert_eq!(
            order.borrow().as_slice(),
            ["transport", "implementation", "runtime"]
        );
        assert_eq!(accepted.evidence().implementation().revision(), 11);
        assert_eq!(accepted.evidence().runtime().revision(), 22);
        assert_eq!(accepted.evidence().transport().revision(), 33);
    }

    #[test]
    fn provider_errors_remain_typed_by_their_separate_owner() {
        let mut implementation = ImplementationProvider(None);
        let mut runtime = RuntimeProvider(None);
        let mut transport = TransportProvider(None);
        let implementation_expected = StreamInfoImplementationVersionWitness::new(
            StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
            "implementation-owner".into(),
            1,
            11,
        )
        .unwrap();
        let runtime_expected = StreamInfoRuntimeWitness::new(
            StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
            "runtime-owner".into(),
            2,
            22,
        )
        .unwrap();
        let transport_expected = StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(32).unwrap(),
            "transport-owner".into(),
            3,
            33,
        )
        .unwrap();

        assert_eq!(
            StreamInfoImplementationVersionAcquisition::acquire(
                &mut implementation,
                &implementation_expected,
                limits(32),
            ),
            Err(StreamInfoImplementationVersionAcquisitionError::Provider(()))
        );
        assert_eq!(
            StreamInfoRuntimeAcquisition::acquire(&mut runtime, &runtime_expected, limits(32)),
            Err(StreamInfoRuntimeAcquisitionError::Provider(()))
        );
        assert_eq!(
            StreamInfoTransportAcquisition::acquire(
                &mut transport,
                &transport_expected,
                limits(32),
            ),
            Err(StreamInfoTransportAcquisitionError::Provider(()))
        );
    }

    #[test]
    fn evidence_and_snapshot_allocations_survive_both_consuming_layers() {
        let (implementation, runtime, transport, value_pointers) = acquisitions();
        let evidence_pointers = [
            implementation.witness().provider_identity().as_ptr(),
            runtime.witness().provider_identity().as_ptr(),
            transport.witness().provider_identity().as_ptr(),
        ];
        let accepted =
            StreamInfoThreeOwnerSnapshot::new(limits(32), implementation, runtime, transport)
                .unwrap();
        let (evidence, snapshot) = accepted.into_parts();
        let (implementation, runtime, transport) = evidence.into_parts();

        assert_eq!(
            implementation.provider_identity().as_ptr(),
            evidence_pointers[0]
        );
        assert_eq!(runtime.provider_identity().as_ptr(), evidence_pointers[1]);
        assert_eq!(transport.provider_identity().as_ptr(), evidence_pointers[2]);
        for (role, pointer) in crate::StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip(value_pointers)
        {
            assert_eq!(snapshot.fields().field(role).as_ptr(), pointer);
        }
    }

    #[test]
    fn repeated_composition_is_deterministic_and_never_cross_matches_owners() {
        for _ in 0..16 {
            let (implementation, runtime, transport, _) = acquisitions();
            let accepted =
                StreamInfoThreeOwnerSnapshot::new(limits(32), implementation, runtime, transport)
                    .unwrap();
            assert_eq!(
                (
                    accepted.evidence().implementation().provider_identity(),
                    accepted.evidence().implementation().epoch(),
                    accepted.evidence().implementation().revision(),
                ),
                ("implementation-owner", 1, 11)
            );
            assert_eq!(
                (
                    accepted.evidence().runtime().provider_identity(),
                    accepted.evidence().runtime().epoch(),
                    accepted.evidence().runtime().revision(),
                ),
                ("runtime-owner", 2, 22)
            );
            assert_eq!(
                (
                    accepted.evidence().transport().provider_identity(),
                    accepted.evidence().transport().epoch(),
                    accepted.evidence().transport().revision(),
                ),
                ("transport-owner", 3, 33)
            );
            assert_eq!(
                crate::StreamInfoVolatileFields::roles()
                    .iter()
                    .map(|role| accepted.snapshot().fields().field(*role))
                    .collect::<Vec<_>>(),
                [
                    "version",
                    "created",
                    "uid",
                    "session",
                    "host",
                    "v4",
                    "v4data",
                    "v4service",
                    "v6",
                    "v6data",
                    "v6service",
                ]
            );
        }
    }
}
