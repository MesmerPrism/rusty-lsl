// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exact-lock admission for inert-by-default runtime module capabilities.

/// Fingerprint of the complete accepted feature lock.
pub const ACCEPTED_FEATURE_LOCK_FINGERPRINT: &str =
    "6ca3a41d93d11085310fac12c085e7183448c4c7f38037921a95a00aedfcb68c";
/// Revision of the complete accepted feature lock.
pub const ACCEPTED_FEATURE_LOCK_REVISION: u64 = 16;

const MAX_CONSUMER_ID_BYTES: usize = 128;
const MODULE_COUNT: usize = 9;

/// A module selected by the accepted lock.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RuntimeModule {
    /// Caller-owned bounded Float32 sample queue.
    BoundedSampleQueue,
    /// Caller-invoked finite sample recovery.
    FiniteSampleRecovery,
    /// Fixed-width numeric sample transport family.
    FixedWidthNumericSample,
    /// Integrated clock correction runtime.
    IntegratedClockCorrection,
    /// Finite short-info discovery responder.
    ShortInfoDiscoveryResponder,
    /// Capability-only bounded String sample module.
    StringSample,
    /// Bounded stream handshake runtime.
    StreamHandshake,
    /// Timestamped Float32 sample runtime.
    TimestampedFloat32Sample,
    /// Bounded UDP discovery runtime.
    UdpDiscovery,
}

impl RuntimeModule {
    /// Returns the exact selected module identity.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::BoundedSampleQueue => "bounded-sample-queue",
            Self::FiniteSampleRecovery => "finite-sample-recovery",
            Self::FixedWidthNumericSample => "fixed-width-numeric-sample",
            Self::IntegratedClockCorrection => "integrated-clock-correction",
            Self::ShortInfoDiscoveryResponder => "short-info-discovery-responder",
            Self::StringSample => "string-sample",
            Self::StreamHandshake => "stream-handshake",
            Self::TimestampedFloat32Sample => "timestamped-float32-sample",
            Self::UdpDiscovery => "udp-discovery",
        }
    }

    /// Returns the exact consumer effective marker declared by the lock.
    #[must_use]
    pub const fn effective_marker(self) -> &'static str {
        match self {
            Self::BoundedSampleQueue => "rusty.lsl.bounded_sample_queue.effective",
            Self::FiniteSampleRecovery => "rusty.lsl.finite_sample_recovery.effective",
            Self::FixedWidthNumericSample => "rusty.lsl.fixed_width_numeric_sample.effective",
            Self::IntegratedClockCorrection => "rusty.lsl.integrated_clock_correction.effective",
            Self::ShortInfoDiscoveryResponder => {
                "rusty.lsl.short_info_discovery_responder.effective"
            }
            Self::StringSample => "rusty.lsl.string_sample.effective",
            Self::StreamHandshake => "rusty.lsl.stream_handshake.effective",
            Self::TimestampedFloat32Sample => "rusty.lsl.timestamped_float32_sample.effective",
            Self::UdpDiscovery => "rusty.lsl.udp_discovery.effective",
        }
    }

    const fn index(self) -> usize {
        self as usize
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "bounded-sample-queue" => Some(Self::BoundedSampleQueue),
            "finite-sample-recovery" => Some(Self::FiniteSampleRecovery),
            "fixed-width-numeric-sample" => Some(Self::FixedWidthNumericSample),
            "integrated-clock-correction" => Some(Self::IntegratedClockCorrection),
            "short-info-discovery-responder" => Some(Self::ShortInfoDiscoveryResponder),
            "string-sample" => Some(Self::StringSample),
            "stream-handshake" => Some(Self::StreamHandshake),
            "timestamped-float32-sample" => Some(Self::TimestampedFloat32Sample),
            "udp-discovery" => Some(Self::UdpDiscovery),
            _ => None,
        }
    }

    const fn dependency(self) -> Option<Self> {
        match self {
            Self::BoundedSampleQueue | Self::IntegratedClockCorrection => {
                Some(Self::TimestampedFloat32Sample)
            }
            Self::FiniteSampleRecovery => Some(Self::BoundedSampleQueue),
            Self::FixedWidthNumericSample | Self::StringSample | Self::TimestampedFloat32Sample => {
                Some(Self::StreamHandshake)
            }
            Self::ShortInfoDiscoveryResponder | Self::StreamHandshake | Self::UdpDiscovery => None,
        }
    }
}

/// One caller-requested module and consumer-observed effective marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeActivationSelection<'a> {
    module_id: &'a str,
    effective_marker: &'a str,
}

impl<'a> RuntimeActivationSelection<'a> {
    /// Creates an untrusted selection proposal for admission.
    #[must_use]
    pub const fn new(module_id: &'a str, effective_marker: &'a str) -> Self {
        Self {
            module_id,
            effective_marker,
        }
    }
}

/// Opaque proof that one exact module survived accepted-lock admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeModuleCapability {
    module: RuntimeModule,
    _private: (),
}

impl RuntimeModuleCapability {
    /// Returns the admitted module identity without exposing construction.
    #[must_use]
    pub const fn module(self) -> RuntimeModule {
        self.module
    }

    pub(crate) const fn matches(self, expected: RuntimeModule) -> bool {
        self.module as usize == expected as usize
    }
}

/// Accepted activation set and its distinct consumer-issued receipt.
#[derive(Debug, Eq, PartialEq)]
pub struct RuntimeActivationAdmission {
    capabilities: [Option<RuntimeModuleCapability>; MODULE_COUNT],
    receipt: RuntimeActivationReceipt,
}

impl RuntimeActivationAdmission {
    /// Returns a capability only when that module was explicitly admitted.
    #[must_use]
    pub const fn capability(&self, module: RuntimeModule) -> Option<RuntimeModuleCapability> {
        self.capabilities[module.index()]
    }

    /// Borrows the separate consumer-issued activation receipt.
    #[must_use]
    pub const fn receipt(&self) -> &RuntimeActivationReceipt {
        &self.receipt
    }

    /// Consumes the admission into its separate receipt.
    #[must_use]
    pub fn into_receipt(self) -> RuntimeActivationReceipt {
        self.receipt
    }
}

/// Consumer evidence for the exact admitted lock and selected marker set.
#[derive(Debug, Eq, PartialEq)]
pub struct RuntimeActivationReceipt {
    consumer_id: String,
    lock_fingerprint: &'static str,
    lock_revision: u64,
    selected_modules: Vec<RuntimeModule>,
    outcome: RuntimeActivationOutcome,
}

/// Closed consumer-issued activation outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeActivationOutcome {
    /// The exact lock, markers, and dependency closure were admitted.
    Accepted,
}

impl RuntimeActivationReceipt {
    /// Returns the consumer that issued effective-marker evidence.
    #[must_use]
    pub fn consumer_id(&self) -> &str {
        &self.consumer_id
    }

    /// Returns the exact complete-lock fingerprint admitted.
    #[must_use]
    pub const fn lock_fingerprint(&self) -> &'static str {
        self.lock_fingerprint
    }

    /// Returns the exact complete-lock revision admitted.
    #[must_use]
    pub const fn lock_revision(&self) -> u64 {
        self.lock_revision
    }

    /// Returns selected modules in accepted-lock order.
    #[must_use]
    pub fn selected_modules(&self) -> &[RuntimeModule] {
        &self.selected_modules
    }

    /// Returns the closed admission outcome.
    #[must_use]
    pub const fn outcome(&self) -> RuntimeActivationOutcome {
        self.outcome
    }

    /// Returns the marker bound to a selected module.
    #[must_use]
    pub fn effective_marker(&self, module: RuntimeModule) -> Option<&'static str> {
        self.selected_modules
            .contains(&module)
            .then(|| module.effective_marker())
    }
}

/// Rejection from exact-lock runtime activation admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeActivationError {
    /// The supplied complete-lock fingerprint was stale or different.
    LockFingerprintMismatch,
    /// The consumer identity was empty.
    EmptyConsumerId,
    /// The consumer identity exceeded its public bound.
    ConsumerIdTooLong {
        /// Observed UTF-8 byte count.
        observed: usize,
        /// Accepted UTF-8 byte maximum.
        maximum: usize,
    },
    /// A requested module is not in the accepted lock.
    UnknownModule {
        /// Selection position in caller order.
        selection: usize,
    },
    /// A module appeared more than once.
    DuplicateModule {
        /// Duplicated accepted module.
        module: RuntimeModule,
        /// Second selection position in caller order.
        selection: usize,
    },
    /// The consumer marker did not match the selected lock.
    EffectiveMarkerMismatch {
        /// Module whose marker differed.
        module: RuntimeModule,
        /// Selection position in caller order.
        selection: usize,
    },
    /// A requested module omitted its declared dependency.
    MissingDependency {
        /// Requested module with incomplete closure.
        module: RuntimeModule,
        /// Required module absent from the selection.
        dependency: RuntimeModule,
    },
    /// Memory could not be reserved for accepted receipt state.
    Allocation {
        /// Requested element or byte count.
        requested: usize,
    },
}

/// Admits explicit runtime selection against the complete accepted feature lock.
pub fn admit_runtime_activation(
    lock_fingerprint: &str,
    consumer_id: &str,
    selections: &[RuntimeActivationSelection<'_>],
) -> Result<RuntimeActivationAdmission, RuntimeActivationError> {
    if lock_fingerprint != ACCEPTED_FEATURE_LOCK_FINGERPRINT {
        return Err(RuntimeActivationError::LockFingerprintMismatch);
    }
    if consumer_id.is_empty() {
        return Err(RuntimeActivationError::EmptyConsumerId);
    }
    if consumer_id.len() > MAX_CONSUMER_ID_BYTES {
        return Err(RuntimeActivationError::ConsumerIdTooLong {
            observed: consumer_id.len(),
            maximum: MAX_CONSUMER_ID_BYTES,
        });
    }

    let mut capabilities = [None; MODULE_COUNT];
    for (selection_index, selection) in selections.iter().enumerate() {
        let module = RuntimeModule::from_id(selection.module_id).ok_or(
            RuntimeActivationError::UnknownModule {
                selection: selection_index,
            },
        )?;
        if capabilities[module.index()].is_some() {
            return Err(RuntimeActivationError::DuplicateModule {
                module,
                selection: selection_index,
            });
        }
        if selection.effective_marker != module.effective_marker() {
            return Err(RuntimeActivationError::EffectiveMarkerMismatch {
                module,
                selection: selection_index,
            });
        }
        capabilities[module.index()] = Some(RuntimeModuleCapability {
            module,
            _private: (),
        });
    }

    for capability in capabilities.iter().flatten() {
        if let Some(dependency) = capability.module.dependency() {
            if capabilities[dependency.index()].is_none() {
                return Err(RuntimeActivationError::MissingDependency {
                    module: capability.module,
                    dependency,
                });
            }
        }
    }

    let mut selected_modules = Vec::new();
    selected_modules
        .try_reserve_exact(selections.len())
        .map_err(|_| RuntimeActivationError::Allocation {
            requested: selections.len(),
        })?;
    for capability in capabilities.iter().flatten() {
        selected_modules.push(capability.module);
    }

    let mut accepted_consumer = String::new();
    accepted_consumer
        .try_reserve_exact(consumer_id.len())
        .map_err(|_| RuntimeActivationError::Allocation {
            requested: consumer_id.len(),
        })?;
    accepted_consumer.push_str(consumer_id);

    Ok(RuntimeActivationAdmission {
        capabilities,
        receipt: RuntimeActivationReceipt {
            consumer_id: accepted_consumer,
            lock_fingerprint: ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            lock_revision: ACCEPTED_FEATURE_LOCK_REVISION,
            selected_modules,
            outcome: RuntimeActivationOutcome::Accepted,
        },
    })
}

#[cfg(test)]
pub(crate) const fn test_capability(module: RuntimeModule) -> RuntimeModuleCapability {
    RuntimeModuleCapability {
        module,
        _private: (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BoundedSampleQueueActivation, FiniteSampleRecoveryActivation,
        FixedWidthNumericSampleActivation, IntegratedClockCorrectionActivation,
        ShortInfoResponderActivation, StreamHandshakeActivation,
        TimestampedFloat32SampleActivation, UdpDiscoveryActivation,
    };

    fn selection(module: RuntimeModule) -> RuntimeActivationSelection<'static> {
        RuntimeActivationSelection::new(module.id(), module.effective_marker())
    }

    #[test]
    fn lslc_003c_exact_lock_issues_module_capabilities_and_receipt() {
        let selections = [
            selection(RuntimeModule::StreamHandshake),
            selection(RuntimeModule::TimestampedFloat32Sample),
            selection(RuntimeModule::BoundedSampleQueue),
            selection(RuntimeModule::FiniteSampleRecovery),
        ];
        let admitted = admit_runtime_activation(
            ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            "synthetic-consumer",
            &selections,
        )
        .unwrap();

        assert_eq!(
            admitted
                .capability(RuntimeModule::FiniteSampleRecovery)
                .unwrap()
                .module(),
            RuntimeModule::FiniteSampleRecovery
        );
        assert_eq!(admitted.capability(RuntimeModule::UdpDiscovery), None);
        assert_eq!(admitted.receipt().consumer_id(), "synthetic-consumer");
        assert_eq!(
            admitted.receipt().lock_fingerprint(),
            ACCEPTED_FEATURE_LOCK_FINGERPRINT
        );
        assert_eq!(
            admitted.receipt().lock_revision(),
            ACCEPTED_FEATURE_LOCK_REVISION
        );
        assert_eq!(
            admitted.receipt().outcome(),
            RuntimeActivationOutcome::Accepted
        );
        assert_eq!(
            admitted.receipt().selected_modules(),
            &[
                RuntimeModule::BoundedSampleQueue,
                RuntimeModule::FiniteSampleRecovery,
                RuntimeModule::StreamHandshake,
                RuntimeModule::TimestampedFloat32Sample,
            ]
        );
        assert_eq!(
            admitted
                .receipt()
                .effective_marker(RuntimeModule::BoundedSampleQueue),
            Some("rusty.lsl.bounded_sample_queue.effective")
        );
        assert_eq!(
            admitted
                .receipt()
                .effective_marker(RuntimeModule::UdpDiscovery),
            None
        );
    }

    #[test]
    fn current_revision_16_admits_and_revision_14_fingerprint_rejects() {
        let current =
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, "lslc-004i-current", &[])
                .unwrap();
        assert_eq!(current.receipt().lock_revision(), 16);
        assert_eq!(current.receipt().selected_modules(), &[]);
        assert_eq!(
            admit_runtime_activation(
                "787827cfe80bd7ff856a304a4f1389070db264d8726a744cd967bbd0948c2c0c",
                "lslc-004i-stale",
                &[],
            ),
            Err(RuntimeActivationError::LockFingerprintMismatch)
        );
    }

    #[test]
    fn lslc_003c_stale_unknown_duplicate_and_marker_damage_fail_closed() {
        assert_eq!(
            admit_runtime_activation("stale", "consumer", &[]),
            Err(RuntimeActivationError::LockFingerprintMismatch)
        );
        assert_eq!(
            admit_runtime_activation(
                ACCEPTED_FEATURE_LOCK_FINGERPRINT,
                "consumer",
                &[RuntimeActivationSelection::new("unknown", "marker")],
            ),
            Err(RuntimeActivationError::UnknownModule { selection: 0 })
        );
        let udp = selection(RuntimeModule::UdpDiscovery);
        assert_eq!(
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, "consumer", &[udp, udp]),
            Err(RuntimeActivationError::DuplicateModule {
                module: RuntimeModule::UdpDiscovery,
                selection: 1,
            })
        );
        assert_eq!(
            admit_runtime_activation(
                ACCEPTED_FEATURE_LOCK_FINGERPRINT,
                "consumer",
                &[RuntimeActivationSelection::new(
                    RuntimeModule::UdpDiscovery.id(),
                    "damaged",
                )],
            ),
            Err(RuntimeActivationError::EffectiveMarkerMismatch {
                module: RuntimeModule::UdpDiscovery,
                selection: 0,
            })
        );
    }

    #[test]
    fn lslc_003c_dependency_closure_and_absence_are_explicit() {
        assert_eq!(
            admit_runtime_activation(
                ACCEPTED_FEATURE_LOCK_FINGERPRINT,
                "consumer",
                &[selection(RuntimeModule::FiniteSampleRecovery)],
            ),
            Err(RuntimeActivationError::MissingDependency {
                module: RuntimeModule::FiniteSampleRecovery,
                dependency: RuntimeModule::BoundedSampleQueue,
            })
        );

        let empty =
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, "consumer", &[]).unwrap();
        assert!(empty.receipt().selected_modules().is_empty());
        assert_eq!(empty.capability(RuntimeModule::StreamHandshake), None);
    }

    #[test]
    fn lslc_006a_accepted_receipts_are_identity_exact_and_canonical_across_selection_order() {
        let forward = [
            selection(RuntimeModule::StreamHandshake),
            selection(RuntimeModule::TimestampedFloat32Sample),
            selection(RuntimeModule::BoundedSampleQueue),
            selection(RuntimeModule::FiniteSampleRecovery),
        ];
        let reverse = [
            selection(RuntimeModule::FiniteSampleRecovery),
            selection(RuntimeModule::BoundedSampleQueue),
            selection(RuntimeModule::TimestampedFloat32Sample),
            selection(RuntimeModule::StreamHandshake),
        ];

        let forward = admit_runtime_activation(
            ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            "authority-consumer",
            &forward,
        )
        .unwrap();
        let reverse = admit_runtime_activation(
            ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            "authority-consumer",
            &reverse,
        )
        .unwrap();

        assert_eq!(forward.receipt(), reverse.receipt());
        assert_eq!(forward.receipt().consumer_id(), "authority-consumer");
        assert_eq!(
            forward.receipt().lock_fingerprint(),
            ACCEPTED_FEATURE_LOCK_FINGERPRINT
        );
        assert_eq!(
            forward.receipt().lock_revision(),
            ACCEPTED_FEATURE_LOCK_REVISION
        );
        assert_eq!(
            forward.receipt().selected_modules(),
            &[
                RuntimeModule::BoundedSampleQueue,
                RuntimeModule::FiniteSampleRecovery,
                RuntimeModule::StreamHandshake,
                RuntimeModule::TimestampedFloat32Sample,
            ]
        );
        for module in forward.receipt().selected_modules() {
            assert_eq!(forward.capability(*module).unwrap().module(), *module);
            assert_eq!(
                forward.receipt().effective_marker(*module),
                Some(module.effective_marker())
            );
        }
    }

    #[test]
    fn lslc_006a_malformed_selection_precedence_rejects_without_partial_authority() {
        let malformed = [
            selection(RuntimeModule::BoundedSampleQueue),
            selection(RuntimeModule::BoundedSampleQueue),
            RuntimeActivationSelection::new("unknown", "damaged"),
        ];

        assert_eq!(
            admit_runtime_activation("stale", "", &malformed),
            Err(RuntimeActivationError::LockFingerprintMismatch)
        );
        assert_eq!(
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, "", &malformed),
            Err(RuntimeActivationError::EmptyConsumerId)
        );
        let rejected = admit_runtime_activation(
            ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            "authority-consumer",
            &malformed,
        );
        assert_eq!(
            rejected,
            Err(RuntimeActivationError::DuplicateModule {
                module: RuntimeModule::BoundedSampleQueue,
                selection: 1,
            })
        );
        assert!(rejected.is_err());
    }

    #[test]
    fn lslc_003c_consumer_identity_is_bounded_before_allocation() {
        assert_eq!(
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, "", &[]),
            Err(RuntimeActivationError::EmptyConsumerId)
        );
        let oversized = "x".repeat(129);
        assert_eq!(
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, &oversized, &[]),
            Err(RuntimeActivationError::ConsumerIdTooLong {
                observed: 129,
                maximum: 128,
            })
        );
    }

    #[test]
    fn lslc_003d_runtime_facades_require_module_and_dependency_capabilities() {
        let selections = [
            selection(RuntimeModule::BoundedSampleQueue),
            selection(RuntimeModule::FiniteSampleRecovery),
            selection(RuntimeModule::FixedWidthNumericSample),
            selection(RuntimeModule::IntegratedClockCorrection),
            selection(RuntimeModule::ShortInfoDiscoveryResponder),
            selection(RuntimeModule::StreamHandshake),
            selection(RuntimeModule::TimestampedFloat32Sample),
            selection(RuntimeModule::UdpDiscovery),
        ];
        let admitted = admit_runtime_activation(
            ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            "synthetic-composition-consumer",
            &selections,
        )
        .unwrap();
        let capability = |module| admitted.capability(module).unwrap();

        let handshake =
            StreamHandshakeActivation::new(capability(RuntimeModule::StreamHandshake)).unwrap();
        let float_sample = TimestampedFloat32SampleActivation::new(
            capability(RuntimeModule::TimestampedFloat32Sample),
            handshake,
        )
        .unwrap();
        let queue = BoundedSampleQueueActivation::new(
            capability(RuntimeModule::BoundedSampleQueue),
            float_sample,
        )
        .unwrap();
        FiniteSampleRecoveryActivation::new(capability(RuntimeModule::FiniteSampleRecovery), queue)
            .unwrap();

        let handshake =
            StreamHandshakeActivation::new(capability(RuntimeModule::StreamHandshake)).unwrap();
        FixedWidthNumericSampleActivation::new(
            capability(RuntimeModule::FixedWidthNumericSample),
            handshake,
        )
        .unwrap();
        let handshake =
            StreamHandshakeActivation::new(capability(RuntimeModule::StreamHandshake)).unwrap();
        let float_sample = TimestampedFloat32SampleActivation::new(
            capability(RuntimeModule::TimestampedFloat32Sample),
            handshake,
        )
        .unwrap();
        IntegratedClockCorrectionActivation::new(
            capability(RuntimeModule::IntegratedClockCorrection),
            float_sample,
        )
        .unwrap();
        ShortInfoResponderActivation::new(capability(RuntimeModule::ShortInfoDiscoveryResponder))
            .unwrap();
        UdpDiscoveryActivation::new(capability(RuntimeModule::UdpDiscovery)).unwrap();
    }

    #[test]
    fn lslc_003s_string_sample_is_distinct_dependency_closed_and_inert_when_absent() {
        assert_eq!(RuntimeModule::StringSample.id(), "string-sample");
        assert_eq!(
            RuntimeModule::StringSample.effective_marker(),
            "rusty.lsl.string_sample.effective"
        );
        assert_eq!(
            admit_runtime_activation(
                ACCEPTED_FEATURE_LOCK_FINGERPRINT,
                "consumer",
                &[selection(RuntimeModule::StringSample)],
            ),
            Err(RuntimeActivationError::MissingDependency {
                module: RuntimeModule::StringSample,
                dependency: RuntimeModule::StreamHandshake,
            })
        );
        let admitted = admit_runtime_activation(
            ACCEPTED_FEATURE_LOCK_FINGERPRINT,
            "consumer",
            &[
                selection(RuntimeModule::StreamHandshake),
                selection(RuntimeModule::StringSample),
            ],
        )
        .unwrap();
        assert_eq!(
            admitted
                .capability(RuntimeModule::StringSample)
                .unwrap()
                .module(),
            RuntimeModule::StringSample
        );
        assert_eq!(
            admitted.capability(RuntimeModule::FixedWidthNumericSample),
            None
        );
        let empty =
            admit_runtime_activation(ACCEPTED_FEATURE_LOCK_FINGERPRINT, "consumer", &[]).unwrap();
        assert_eq!(empty.capability(RuntimeModule::StringSample), None);
    }
}
