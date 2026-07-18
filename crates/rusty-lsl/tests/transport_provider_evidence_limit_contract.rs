// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External conformance for transport-provider evidence-limit contracts.

use rusty_lsl::{
    StreamInfoTransportAcquisition, StreamInfoTransportAcquisitionError,
    StreamInfoTransportEvidenceError, StreamInfoTransportEvidenceLimit,
    StreamInfoTransportProvider, StreamInfoTransportProviderOutput, StreamInfoTransportValues,
    StreamInfoTransportWitness, StreamInfoVolatileFieldLimits,
};

struct OneShotProvider(Option<StreamInfoTransportProviderOutput>);

impl StreamInfoTransportProvider for OneShotProvider {
    type Error = &'static str;

    fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
        self.0.take().ok_or("provider called more than once")
    }
}

fn witness(identity: &str, epoch: u64, revision: u64) -> StreamInfoTransportWitness {
    StreamInfoTransportWitness::new(
        StreamInfoTransportEvidenceLimit::new(identity.chars().count()).unwrap(),
        identity.to_owned(),
        epoch,
        revision,
    )
    .unwrap()
}

#[test]
fn evidence_limit_construction_rejects_zero_and_retains_the_exact_nonzero_bound() {
    assert_eq!(
        StreamInfoTransportEvidenceLimit::new(0),
        Err(StreamInfoTransportEvidenceError::InvalidProviderIdentityLimit)
    );

    let limit = StreamInfoTransportEvidenceLimit::new(17).unwrap();
    assert_eq!(limit.max_provider_identity_code_points(), 17);
}

#[test]
fn provider_identity_bound_counts_unicode_scalars_instead_of_utf8_bytes() {
    let limit = StreamInfoTransportEvidenceLimit::new(3).unwrap();
    let accepted = StreamInfoTransportWitness::new(limit, "Aé🦀".to_owned(), 5, 8).unwrap();
    assert_eq!(accepted.provider_identity(), "Aé🦀");
    assert_eq!((accepted.epoch(), accepted.revision()), (5, 8));

    assert_eq!(
        StreamInfoTransportWitness::new(limit, "Aé🦀Z".to_owned(), 13, 21),
        Err(
            StreamInfoTransportEvidenceError::ProviderIdentityLimitExceeded {
                expected_max: 3,
                actual: 4,
            }
        )
    );
}

#[test]
fn empty_identity_rejection_has_its_exact_payload() {
    assert_eq!(
        StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(1).unwrap(),
            String::new(),
            u64::MAX,
            u64::MAX,
        ),
        Err(StreamInfoTransportEvidenceError::EmptyProviderIdentity)
    );
}

#[test]
fn acquisition_rejects_identity_before_epoch_revision_and_value_damage() {
    let returned = StreamInfoTransportProviderOutput::new(
        witness("other", 89, 144),
        StreamInfoTransportValues::new(
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
            "oversized".into(),
        ),
    );
    let mut provider = OneShotProvider(Some(returned));

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(
            &mut provider,
            &witness("owner", 34, 55),
            StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
        ),
        Err(StreamInfoTransportAcquisitionError::ProviderIdentityMismatch)
    );
}
