// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-classified finite recovery from one selected response into an existing queue.

use crate::{
    run_finite_sample_recovery, run_selected_typed_udp_discovery_float32_inlet, BoundedSampleQueue,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, FiniteSampleRecoveryActivation,
    FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome, FiniteSampleRecoveryPolicy,
    FiniteSampleRecoveryState, RecoveryAttemptFailure, StreamHandshakeIdentity,
    StreamHandshakeLimits, TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    TypedUdpDiscoveryFloat32Error, TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Terminal result of caller-classified finite recovery and queue admission.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32RecoveryQueueOutcome {
    /// One recovered sample entered the caller-owned queue.
    Queued {
        /// Ordered finite recovery states ending in recovery.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The caller classified one inlet failure as terminal.
    Terminal {
        /// Unchanged caller-classified failure.
        failure: RecoveryAttemptFailure,
        /// Ordered finite recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Every permitted attempt produced a caller-classified retryable failure.
    Exhausted {
        /// Last unchanged caller-classified failure.
        failure: RecoveryAttemptFailure,
        /// Ordered finite recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Recovery cancellation was observed independently of inlet and queue cancellation.
    Cancelled {
        /// Ordered finite recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The recovery policy deadline elapsed.
    Deadline {
        /// Ordered finite recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Stable setup or post-recovery queue failure.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32RecoveryQueueError {
    /// The existing finite recovery owner could not reserve its bounded trace.
    Recovery(FiniteSampleRecoveryError),
    /// The existing queue rejected the unchanged recovered sample.
    Queue {
        /// Existing queue error retaining rejected sample ownership.
        error: BoundedSampleQueuePushError,
        /// Ordered recovery states ending in recovery.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Runs caller-classified finite inlet recovery, then queues only a recovered sample.
///
/// The caller separately owns recovery activation and policy, typed inlet-error
/// classification, the already activated queue, wait bounds, and recovery, inlet,
/// and queue cancellation inputs. No endpoint reselection or rediscovery occurs.
#[allow(clippy::too_many_arguments)]
pub fn run_recovering_selected_typed_udp_discovery_float32_inlet_into_queue<C>(
    run: &TypedUdpDiscoveryRun,
    response_index: usize,
    sample_activation: TimestampedFloat32SampleActivation,
    identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    inlet_cancelled: &AtomicBool,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    recovery_cancelled: &AtomicBool,
    mut classify: C,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    queue_cancelled: &AtomicBool,
) -> Result<TypedUdpDiscoveryFloat32RecoveryQueueOutcome, TypedUdpDiscoveryFloat32RecoveryQueueError>
where
    C: FnMut(usize, &TypedUdpDiscoveryFloat32Error) -> RecoveryAttemptFailure,
{
    let recovery = run_finite_sample_recovery(
        recovery_activation,
        recovery_policy,
        recovery_cancelled,
        |attempt| {
            run_selected_typed_udp_discovery_float32_inlet(
                run,
                response_index,
                sample_activation,
                identity,
                handshake_limits,
                sample_limits,
                inlet_cancelled,
            )
            .map_err(|error| classify(attempt, &error))
        },
    )
    .map_err(TypedUdpDiscoveryFloat32RecoveryQueueError::Recovery)?;

    match recovery {
        FiniteSampleRecoveryOutcome::Recovered { sample, states } => {
            match queue.push(sample, queue_wait, queue_cancelled) {
                Ok(()) => Ok(TypedUdpDiscoveryFloat32RecoveryQueueOutcome::Queued { states }),
                Err(error) => {
                    Err(TypedUdpDiscoveryFloat32RecoveryQueueError::Queue { error, states })
                }
            }
        }
        FiniteSampleRecoveryOutcome::Terminal { failure, states } => {
            Ok(TypedUdpDiscoveryFloat32RecoveryQueueOutcome::Terminal { failure, states })
        }
        FiniteSampleRecoveryOutcome::Exhausted { failure, states } => {
            Ok(TypedUdpDiscoveryFloat32RecoveryQueueOutcome::Exhausted { failure, states })
        }
        FiniteSampleRecoveryOutcome::Cancelled { states } => {
            Ok(TypedUdpDiscoveryFloat32RecoveryQueueOutcome::Cancelled { states })
        }
        FiniteSampleRecoveryOutcome::Deadline { states } => {
            Ok(TypedUdpDiscoveryFloat32RecoveryQueueOutcome::Deadline { states })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::test_network_harness::tcp::SpawnedTcpPeer;
    use crate::{
        run_timestamped_float32_outlet, run_typed_udp_discovery, BoundedSampleQueueActivation,
        MetadataTreeLimits, RawSourceTimestamp, RecoveryFailureClass, RuntimeModule, Sample,
        SampleLimits, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamInfoObservedAdmissionLimits, StreamInfoVolatileFieldLimits, TimestampedSample,
        UdpDiscoveryActivation, UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }
    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }
    fn queue_wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(100)).unwrap()
    }
    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            handshake_limits(),
        )
        .unwrap()
    }
    fn sample_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }
    fn queue_activation() -> BoundedSampleQueueActivation {
        BoundedSampleQueueActivation::new(
            test_capability(RuntimeModule::BoundedSampleQueue),
            sample_activation(),
        )
        .unwrap()
    }
    fn recovery_activation() -> FiniteSampleRecoveryActivation {
        FiniteSampleRecoveryActivation::new(
            test_capability(RuntimeModule::FiniteSampleRecovery),
            queue_activation(),
        )
        .unwrap()
    }
    fn recovery_policy(attempts: usize) -> FiniteSampleRecoveryPolicy {
        FiniteSampleRecoveryPolicy::new(
            attempts,
            attempts * 2 + 1,
            Duration::from_millis(20),
            Duration::from_millis(1),
            Duration::from_secs(1),
        )
        .unwrap()
    }
    fn sample(timestamp: f64, value: f32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            None,
        )
    }

    fn typed_run(service_port: u16) -> TypedUdpDiscoveryRun {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = peer.local_addr().unwrap();
        let roles = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", "1".to_owned()),
            ("channel_format", "float32".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "11111111-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "session".to_owned()),
            ("hostname", "host".to_owned()),
            ("v4address", "127.0.0.1".to_owned()),
            ("v4data_port", "43001".to_owned()),
            ("v4service_port", service_port.to_string()),
            ("v6address", "2001:db8::10".to_owned()),
            ("v6data_port", "43003".to_owned()),
            ("v6service_port", "43004".to_owned()),
        ];
        let mut document = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in roles {
            document.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        document.push_str("\t<desc />\n</info>\n");
        let response = format!("19\r\n{document}").into_bytes();
        let document_bytes = document.len();
        let worker = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = peer.recv_from(&mut query).unwrap();
            peer.send_to(&response, source).unwrap();
        });
        let query_limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, query_limits).unwrap(),
            query_limits,
        )
        .unwrap();
        let envelope_limits =
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap();
        let run = run_typed_udp_discovery(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    document_bytes + 32,
                    1,
                    Duration::from_millis(10),
                    Duration::from_secs(1),
                )
                .unwrap(),
                envelope_limits,
            ),
            &query,
            &AtomicBool::new(false),
            envelope_limits,
            StreamInfoObservedAdmissionLimits::new(
                StreamDescriptorLimits::new(64, 64, 64, 4).unwrap(),
                MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
                StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
            ),
        )
        .unwrap();
        worker.join().unwrap();
        run
    }

    #[test]
    fn lslc_005b_caller_classifies_retry_then_queues_exact_record() {
        let first = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = first.local_addr().unwrap();
        let worker = SpawnedTcpPeer::spawn("lslc-005b-retry-outlet", first, move |first| {
            drop(first.accept().unwrap().0);
            drop(first);
            let second = TcpListener::bind(address).unwrap();
            run_timestamped_float32_outlet(
                sample_activation(),
                second,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sample(-0.0, f32::from_bits(0x7fc0_2468)),
                &AtomicBool::new(false),
            )
            .unwrap();
        });
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let mut classified = Vec::new();
        let outcome = run_recovering_selected_typed_udp_discovery_float32_inlet_into_queue(
            &typed_run(address.port()),
            0,
            sample_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
            recovery_activation(),
            recovery_policy(2),
            &AtomicBool::new(false),
            |attempt, _| {
                classified.push(attempt);
                RecoveryAttemptFailure::new(RecoveryFailureClass::Retryable, 17)
            },
            &queue,
            queue_wait(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(
            matches!(outcome, TypedUdpDiscoveryFloat32RecoveryQueueOutcome::Queued { ref states } if states.len() == 4)
        );
        assert_eq!(classified, vec![1]);
        let drained = queue.try_pop().unwrap();
        assert_eq!(
            drained.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(drained.sample().values()[0].to_bits(), 0x7fc0_2468);
        worker.complete(Duration::from_secs(2)).unwrap();
    }

    #[test]
    fn lslc_005b_queue_cancellation_retains_record_and_recovery_states() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = SpawnedTcpPeer::spawn(
            "lslc-005b-cancelled-queue-outlet",
            listener,
            move |listener| {
                run_timestamped_float32_outlet(
                    sample_activation(),
                    listener,
                    &identity(),
                    handshake_limits(),
                    sample_limits(),
                    &sample(1234.5, 7.25),
                    &AtomicBool::new(false),
                )
                .unwrap();
            },
        );
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let error = run_recovering_selected_typed_udp_discovery_float32_inlet_into_queue(
            &typed_run(address.port()),
            0,
            sample_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
            recovery_activation(),
            recovery_policy(1),
            &AtomicBool::new(false),
            |_, _| RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, 1),
            &queue,
            queue_wait(),
            &AtomicBool::new(true),
        )
        .unwrap_err();
        match error {
            TypedUdpDiscoveryFloat32RecoveryQueueError::Queue {
                error: BoundedSampleQueuePushError::Cancelled(sample),
                states,
            } => {
                assert_eq!(
                    states,
                    vec![
                        FiniteSampleRecoveryState::Attempting { attempt: 1 },
                        FiniteSampleRecoveryState::Recovered { attempt: 1 }
                    ]
                );
                assert_eq!(
                    sample.raw_source_timestamp().value().to_bits(),
                    1234.5f64.to_bits()
                );
                assert_eq!(sample.sample().values()[0].to_bits(), 7.25f32.to_bits());
            }
            other => panic!("expected queue cancellation, got {other:?}"),
        }
        worker.complete(Duration::from_secs(2)).unwrap();
    }
}
