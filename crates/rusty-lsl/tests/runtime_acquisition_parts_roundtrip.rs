// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External-consumer ownership checks for accepted runtime acquisition parts.

use rusty_lsl::{
    StreamInfoRuntimeAcquisition, StreamInfoRuntimeEvidenceLimit, StreamInfoRuntimeProvider,
    StreamInfoRuntimeProviderOutput, StreamInfoRuntimeValues, StreamInfoRuntimeWitness,
    StreamInfoVolatileFieldLimits,
};

struct OneShotProvider(Option<StreamInfoRuntimeProviderOutput>);

impl StreamInfoRuntimeProvider for OneShotProvider {
    type Error = ();

    fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error> {
        self.0.take().ok_or(())
    }
}

fn witness() -> StreamInfoRuntimeWitness {
    StreamInfoRuntimeWitness::new(
        StreamInfoRuntimeEvidenceLimit::new(32).expect("nonzero evidence limit"),
        "runtime-owner".into(),
        17,
        23,
    )
    .expect("bounded runtime witness")
}

#[test]
fn accepted_runtime_parts_preserve_borrowed_witness_and_all_four_value_allocations() {
    let created_at = String::from("created-at-value");
    let uid = String::from("uid-value");
    let session_id = String::from("session-id-value");
    let hostname = String::from("hostname-value");
    let original_pointers = [
        created_at.as_ptr(),
        uid.as_ptr(),
        session_id.as_ptr(),
        hostname.as_ptr(),
    ];

    let mut provider = OneShotProvider(Some(StreamInfoRuntimeProviderOutput::new(
        witness(),
        StreamInfoRuntimeValues::new(created_at, uid, session_id, hostname),
    )));
    let accepted = StreamInfoRuntimeAcquisition::acquire(
        &mut provider,
        &witness(),
        StreamInfoVolatileFieldLimits::new(32, 32, 32).expect("nonzero field limits"),
    )
    .expect("matching bounded runtime acquisition");

    assert_eq!(accepted.witness().provider_identity(), "runtime-owner");
    assert_eq!(accepted.witness().epoch(), 17);
    assert_eq!(accepted.witness().revision(), 23);
    assert_eq!(accepted.values().created_at(), "created-at-value");
    assert_eq!(accepted.values().uid(), "uid-value");
    assert_eq!(accepted.values().session_id(), "session-id-value");
    assert_eq!(accepted.values().hostname(), "hostname-value");
    assert_eq!(
        [
            accepted.values().created_at().as_ptr(),
            accepted.values().uid().as_ptr(),
            accepted.values().session_id().as_ptr(),
            accepted.values().hostname().as_ptr(),
        ],
        original_pointers
    );

    let (accepted_witness, accepted_values) = accepted.into_parts();
    assert_eq!(accepted_witness.provider_identity(), "runtime-owner");
    assert_eq!(
        (accepted_witness.epoch(), accepted_witness.revision()),
        (17, 23)
    );

    let parts = accepted_values.into_parts();
    assert_eq!(
        [
            parts.0.as_ptr(),
            parts.1.as_ptr(),
            parts.2.as_ptr(),
            parts.3.as_ptr()
        ],
        original_pointers
    );
    assert_eq!(
        [
            parts.0.as_str(),
            parts.1.as_str(),
            parts.2.as_str(),
            parts.3.as_str()
        ],
        [
            "created-at-value",
            "uid-value",
            "session-id-value",
            "hostname-value",
        ]
    );
}
