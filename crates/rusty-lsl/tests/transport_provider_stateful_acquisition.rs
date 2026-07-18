// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Public-API conformance for sequential use of one stateful transport provider.

use std::collections::VecDeque;

use rusty_lsl::{
    StreamInfoTransportAcquisition, StreamInfoTransportAcquisitionError,
    StreamInfoTransportEvidenceLimit, StreamInfoTransportProvider,
    StreamInfoTransportProviderOutput, StreamInfoTransportValues, StreamInfoTransportWitness,
    StreamInfoVolatileFieldError, StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole,
};

#[derive(Debug, Eq, PartialEq)]
enum StatefulProviderError {
    TemporarilyUnavailable(u64),
    Exhausted,
}

struct StatefulProvider {
    calls: usize,
    outputs: VecDeque<Result<StreamInfoTransportProviderOutput, StatefulProviderError>>,
}

impl StreamInfoTransportProvider for StatefulProvider {
    type Error = StatefulProviderError;

    fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
        self.calls += 1;
        self.outputs
            .pop_front()
            .unwrap_or(Err(StatefulProviderError::Exhausted))
    }
}

fn witness(epoch: u64, revision: u64) -> StreamInfoTransportWitness {
    StreamInfoTransportWitness::new(
        StreamInfoTransportEvidenceLimit::new(16).unwrap(),
        "stateful-owner".into(),
        epoch,
        revision,
    )
    .unwrap()
}

fn output(epoch: u64, revision: u64, values: [&str; 6]) -> StreamInfoTransportProviderOutput {
    StreamInfoTransportProviderOutput::new(
        witness(epoch, revision),
        StreamInfoTransportValues::new(
            values[0].into(),
            values[1].into(),
            values[2].into(),
            values[3].into(),
            values[4].into(),
            values[5].into(),
        ),
    )
}

fn limits(max_transport_code_points: usize) -> StreamInfoVolatileFieldLimits {
    StreamInfoVolatileFieldLimits::new(1, 1, max_transport_code_points).unwrap()
}

#[test]
fn sequential_stateful_acquisitions_are_call_isolated_and_recover_after_typed_failures() {
    let mut provider = StatefulProvider {
        calls: 0,
        outputs: VecDeque::from([
            Ok(output(7, 11, ["a1", "d1", "s1", "A1", "D1", "S1"])),
            Err(StatefulProviderError::TemporarilyUnavailable(23)),
            Ok(output(7, 11, ["oversized", "d3", "s3", "A3", "D3", "S3"])),
            Ok(output(7, 11, ["a4", "d4", "s4", "A4", "D4", "S4"])),
        ]),
    };
    let expected = witness(7, 11);

    let first = StreamInfoTransportAcquisition::acquire(&mut provider, &expected, limits(2))
        .expect("the first queued acquisition should be accepted");
    assert_eq!(provider.calls, 1);
    assert_eq!(first.values().v4address(), "a1");
    assert_eq!(first.values().v6service_port(), "S1");

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, limits(2)),
        Err(StreamInfoTransportAcquisitionError::Provider(
            StatefulProviderError::TemporarilyUnavailable(23)
        ))
    );
    assert_eq!(provider.calls, 2);
    assert_eq!(first.values().v4address(), "a1");

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, limits(2)),
        Err(StreamInfoTransportAcquisitionError::Value(
            StreamInfoVolatileFieldError::TextLimitExceeded {
                role: StreamInfoVolatileFieldRole::V4Address,
                expected_max: 2,
                actual: 9,
            }
        ))
    );
    assert_eq!(provider.calls, 3);
    assert_eq!(first.values().v6service_port(), "S1");

    let recovered = StreamInfoTransportAcquisition::acquire(&mut provider, &expected, limits(2))
        .expect("a later valid provider output should recover independently");
    assert_eq!(provider.calls, 4);
    assert_eq!(recovered.witness(), &expected);
    assert_eq!(recovered.values().v4address(), "a4");
    assert_eq!(recovered.values().v6service_port(), "S4");
    assert_eq!(first.values().v4address(), "a1");

    assert_eq!(
        StreamInfoTransportAcquisition::acquire(&mut provider, &expected, limits(2)),
        Err(StreamInfoTransportAcquisitionError::Provider(
            StatefulProviderError::Exhausted
        ))
    );
    assert_eq!(provider.calls, 5);
}
