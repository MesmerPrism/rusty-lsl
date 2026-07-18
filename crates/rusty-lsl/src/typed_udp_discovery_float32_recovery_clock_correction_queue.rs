// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Finite inlet recovery followed by explicit clock correction and queue admission.

use crate::{
    run_finite_sample_recovery, run_integrated_clock_correction,
    run_selected_typed_udp_discovery_float32_inlet, BoundedSampleQueue,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, ClockSource,
    FiniteSampleRecoveryActivation, FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryState, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, IntegratedClockCorrectionError, RecoveryAttemptFailure,
    StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits, TimestampedSample, TypedUdpDiscoveryFloat32Error,
    TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Terminal result of the combined bounded composition.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome {
    /// One recovered, corrected record entered the queue.
    Queued {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The caller classified one inlet failure as terminal.
    Terminal {
        /// Unchanged failure.
        failure: RecoveryAttemptFailure,
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Every permitted attempt failed retryably.
    Exhausted {
        /// Last unchanged failure.
        failure: RecoveryAttemptFailure,
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Recovery cancellation was observed.
    Cancelled {
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The recovery deadline elapsed.
    Deadline {
        /// Ordered states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Stable owner-preserving setup, correction, or queue failure.
#[derive(Debug)]
pub enum TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError {
    /// Recovery trace setup failed.
    Recovery(FiniteSampleRecoveryError),
    /// Correction failed and returns the unchanged recovered record.
    Clock {
        /// Existing correction failure.
        error: IntegratedClockCorrectionError,
        /// Unchanged recovered record.
        sample: TimestampedSample<f32>,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Queue admission failed and retains the corrected record.
    Queue {
        /// Existing queue failure.
        error: BoundedSampleQueuePushError,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Recovers one inlet record, corrects only that record, then queues it.
#[allow(clippy::too_many_arguments)]
pub fn run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue<
    C,
    K,
>(
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
    mut classify: K,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    clock_cancelled: &AtomicBool,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    queue_cancelled: &AtomicBool,
) -> Result<
    TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome,
    TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError,
>
where
    C: ClockSource,
    K: FnMut(usize, &TypedUdpDiscoveryFloat32Error) -> RecoveryAttemptFailure,
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
    .map_err(TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Recovery)?;
    let (sample, states) = match recovery {
        FiniteSampleRecoveryOutcome::Recovered { sample, states } => (sample, states),
        FiniteSampleRecoveryOutcome::Terminal { failure, states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Terminal {
                    failure,
                    states,
                },
            )
        }
        FiniteSampleRecoveryOutcome::Exhausted { failure, states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Exhausted {
                    failure,
                    states,
                },
            )
        }
        FiniteSampleRecoveryOutcome::Cancelled { states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Cancelled { states },
            )
        }
        FiniteSampleRecoveryOutcome::Deadline { states } => {
            return Ok(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Deadline { states },
            )
        }
    };
    let correction = match run_integrated_clock_correction(
        clock_activation,
        clock_config,
        clock,
        sample.raw_source_timestamp(),
        clock_cancelled,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Err(
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Clock {
                    error,
                    sample,
                    states,
                },
            )
        }
    };
    let (values, raw, _) = sample.into_parts();
    let corrected = TimestampedSample::new(values, raw, Some(correction.application().derived()));
    match queue.push(corrected, queue_wait, queue_cancelled) {
        Ok(()) => {
            Ok(TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Queued { states })
        }
        Err(error) => {
            Err(TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Queue { error, states })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_timestamped_float32_outlet, run_typed_udp_discovery, BoundedSampleQueueActivation,
        ClockOffset, ClockOffsetApplication, DerivedTimestampKind, MetadataTreeLimits,
        RawSourceTimestamp, RecoveryFailureClass, RuntimeModule, Sample, SampleLimits,
        ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamInfoObservedAdmissionLimits, StreamInfoVolatileFieldLimits, UdpDiscoveryActivation,
        UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{SocketAddr, TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    struct SequenceClock {
        values: Vec<f64>,
        index: usize,
    }

    impl ClockSource for SequenceClock {
        fn now(&mut self) -> f64 {
            let value = self.values[self.index];
            self.index += 1;
            value
        }
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
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

    fn clock_activation() -> IntegratedClockCorrectionActivation {
        IntegratedClockCorrectionActivation::new(
            test_capability(RuntimeModule::IntegratedClockCorrection),
            sample_activation(),
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
        let document = format!(
            "<?xml version=\"1.0\"?>\n<info>\n\t<name>selected</name>\n\t<type>independent</type>\n\t<channel_count>1</channel_count>\n\t<channel_format>float32</channel_format>\n\t<source_id>source</source_id>\n\t<nominal_srate>100.0000000000000</nominal_srate>\n\t<version>110</version>\n\t<created_at>1</created_at>\n\t<uid>11111111-2222-4333-8444-555555555555</uid>\n\t<session_id>session</session_id>\n\t<hostname>host</hostname>\n\t<v4address>127.0.0.1</v4address>\n\t<v4data_port>43001</v4data_port>\n\t<v4service_port>{service_port}</v4service_port>\n\t<v6address>2001:db8::10</v6address>\n\t<v6data_port>43003</v6data_port>\n\t<v6service_port>43004</v6service_port>\n\t<desc />\n</info>\n"
        );
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

    fn correction_config(peer: SocketAddr) -> IntegratedClockCorrectionConfig {
        IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            peer,
            70,
            1,
            256,
            Duration::from_millis(5),
            Duration::from_secs(1),
        )
        .unwrap()
    }
    #[test]
    fn lslc_005d_reconstruction_preserves_raw_and_value_bits() {
        let raw = RawSourceTimestamp::new(-0.0).unwrap();
        let sample = TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(0x7fc0_5d5d)],
            )
            .unwrap(),
            raw,
            None,
        );
        let application =
            ClockOffsetApplication::apply(raw, ClockOffset::new(3.0).unwrap()).unwrap();
        let (values, retained_raw, _) = sample.into_parts();
        let corrected = TimestampedSample::new(values, retained_raw, Some(application.derived()));
        assert_eq!(
            corrected.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(corrected.sample().values()[0].to_bits(), 0x7fc0_5d5d);
        assert_eq!(
            corrected.derived_timestamp().unwrap().kind(),
            DerivedTimestampKind::ClockCorrected
        );
        assert_eq!(corrected.derived_timestamp().unwrap().value(), 3.0);
    }

    #[test]
    fn lslc_005e_retries_then_corrects_once_and_queues_exact_record() {
        let first = TcpListener::bind("127.0.0.1:0").unwrap();
        let inlet_address = first.local_addr().unwrap();
        let inlet_worker = thread::spawn(move || {
            let (stream, _) = first.accept().unwrap();
            drop(stream);
            drop(first);
            let second = TcpListener::bind(inlet_address).unwrap();
            run_timestamped_float32_outlet(
                sample_activation(),
                second,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sample(-0.0, f32::from_bits(0x7fc0_5e5e)),
                &AtomicBool::new(false),
            )
            .unwrap();
        });
        let correction_peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let correction_address = correction_peer.local_addr().unwrap();
        let correction_worker = thread::spawn(move || {
            let mut request = [0_u8; 256];
            let (length, source) = correction_peer.recv_from(&mut request).unwrap();
            let text = std::str::from_utf8(&request[..length]).unwrap();
            let mut fields = text.split("\r\n").nth(1).unwrap().split(' ');
            let id = fields.next().unwrap();
            let t0 = fields.next().unwrap();
            correction_peer
                .send_to(format!(" {id} {t0} 4.0 4.0").as_bytes(), source)
                .unwrap();
        });
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let mut clock = SequenceClock {
            values: vec![0.0, 2.0],
            index: 0,
        };
        let mut classified = Vec::new();
        let outcome = run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue(
            &typed_run(inlet_address.port()),
            0,
            sample_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            &AtomicBool::new(false),
            recovery_activation(),
            FiniteSampleRecoveryPolicy::new(2, 5, Duration::from_millis(20), Duration::from_millis(1), Duration::from_secs(1)).unwrap(),
            &AtomicBool::new(false),
            |attempt, _| {
                classified.push(attempt);
                RecoveryAttemptFailure::new(RecoveryFailureClass::Retryable, 23)
            },
            clock_activation(),
            correction_config(correction_address),
            &mut clock,
            &AtomicBool::new(false),
            &queue,
            BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(100)).unwrap(),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(
            matches!(outcome, TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Queued { ref states } if states.len() == 4)
        );
        assert_eq!(classified, vec![1]);
        assert_eq!(clock.index, 2);
        let drained = queue.try_pop().unwrap();
        assert_eq!(
            drained.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(drained.sample().values()[0].to_bits(), 0x7fc0_5e5e);
        let derived = drained.derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value(), 3.0);
        inlet_worker.join().unwrap();
        correction_worker.join().unwrap();
    }

    #[test]
    fn lslc_005f_clock_cancellation_returns_recovered_record_and_states() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let inlet_worker = thread::spawn(move || {
            run_timestamped_float32_outlet(
                sample_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sample(-0.0, f32::from_bits(0x7fc0_5f01)),
                &AtomicBool::new(false),
            )
            .unwrap();
        });
        let unused_peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let mut clock = SequenceClock {
            values: vec![],
            index: 0,
        };
        let error = run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue(
            &typed_run(address.port()), 0, sample_activation(), &identity(), handshake_limits(),
            sample_limits(), &AtomicBool::new(false), recovery_activation(),
            FiniteSampleRecoveryPolicy::new(1, 3, Duration::from_millis(20), Duration::from_millis(1), Duration::from_secs(1)).unwrap(),
            &AtomicBool::new(false), |_, _| RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, 1),
            clock_activation(), correction_config(unused_peer.local_addr().unwrap()), &mut clock,
            &AtomicBool::new(true), &queue,
            BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(100)).unwrap(),
            &AtomicBool::new(false),
        ).unwrap_err();
        match error {
            TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Clock {
                error: IntegratedClockCorrectionError::Cancelled,
                sample,
                states,
            } => {
                assert_eq!(states.len(), 2);
                assert_eq!(
                    sample.raw_source_timestamp().value().to_bits(),
                    (-0.0f64).to_bits()
                );
                assert_eq!(sample.sample().values()[0].to_bits(), 0x7fc0_5f01);
                assert!(sample.derived_timestamp().is_none());
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert!(queue.try_pop().is_err());
        assert_eq!(clock.index, 0);
        inlet_worker.join().unwrap();
    }

    #[test]
    fn lslc_005f_queue_cancellation_retains_corrected_record_and_states() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let inlet_worker = thread::spawn(move || {
            run_timestamped_float32_outlet(
                sample_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &sample(10.0, 5.25),
                &AtomicBool::new(false),
            )
            .unwrap();
        });
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer_address = peer.local_addr().unwrap();
        let correction_worker = thread::spawn(move || {
            let mut request = [0_u8; 256];
            let (length, source) = peer.recv_from(&mut request).unwrap();
            let text = std::str::from_utf8(&request[..length]).unwrap();
            let mut fields = text.split("\r\n").nth(1).unwrap().split(' ');
            let id = fields.next().unwrap();
            let t0 = fields.next().unwrap();
            peer.send_to(format!(" {id} {t0} 4.0 4.0").as_bytes(), source)
                .unwrap();
        });
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let mut clock = SequenceClock {
            values: vec![0.0, 2.0],
            index: 0,
        };
        let error = run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue(
            &typed_run(address.port()), 0, sample_activation(), &identity(), handshake_limits(),
            sample_limits(), &AtomicBool::new(false), recovery_activation(),
            FiniteSampleRecoveryPolicy::new(1, 3, Duration::from_millis(20), Duration::from_millis(1), Duration::from_secs(1)).unwrap(),
            &AtomicBool::new(false), |_, _| RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, 1),
            clock_activation(), correction_config(peer_address), &mut clock, &AtomicBool::new(false),
            &queue, BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(100)).unwrap(),
            &AtomicBool::new(true),
        ).unwrap_err();
        match error {
            TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError::Queue {
                error: BoundedSampleQueuePushError::Cancelled(sample),
                states,
            } => {
                assert_eq!(states.len(), 2);
                assert_eq!(sample.raw_source_timestamp().value(), 10.0);
                assert_eq!(sample.sample().values(), &[5.25]);
                assert_eq!(sample.derived_timestamp().unwrap().value(), 13.0);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert!(queue.try_pop().is_err());
        inlet_worker.join().unwrap();
        correction_worker.join().unwrap();
    }

    #[test]
    fn lslc_005g_terminal_and_exhausted_bypass_clock_and_queue() {
        for (class, expected_code, expected_states) in [
            (RecoveryFailureClass::Terminal, 71, 2),
            (RecoveryFailureClass::Retryable, 72, 3),
        ] {
            let unused_inlet = TcpListener::bind("127.0.0.1:0").unwrap();
            let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
            let mut clock = SequenceClock {
                values: vec![],
                index: 0,
            };
            let outcome = run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue(
                &typed_run(unused_inlet.local_addr().unwrap().port()), 0, sample_activation(),
                &identity(), handshake_limits(), sample_limits(), &AtomicBool::new(false),
                recovery_activation(),
                FiniteSampleRecoveryPolicy::new(1, 3, Duration::from_millis(1), Duration::from_millis(1), Duration::from_secs(1)).unwrap(),
                &AtomicBool::new(false),
                |_, _| RecoveryAttemptFailure::new(class, expected_code),
                clock_activation(), correction_config("127.0.0.1:9".parse().unwrap()),
                &mut clock, &AtomicBool::new(false), &queue,
                BoundedSampleQueueWait::new(Duration::from_millis(1), Duration::from_millis(5)).unwrap(),
                &AtomicBool::new(false),
            ).unwrap();
            match outcome {
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Terminal {
                    failure,
                    states,
                }
                | TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Exhausted {
                    failure,
                    states,
                } => {
                    assert_eq!(failure.code(), expected_code);
                    assert_eq!(states.len(), expected_states);
                }
                other => panic!("unexpected outcome: {other:?}"),
            }
            assert_eq!(clock.index, 0);
            assert!(queue.try_pop().is_err());
        }
    }

    #[test]
    fn lslc_005g_recovery_cancel_and_deadline_bypass_clock_and_queue() {
        for (cancelled, deadline) in [
            (true, Duration::from_secs(1)),
            (false, Duration::from_nanos(1)),
        ] {
            let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
            let mut clock = SequenceClock {
                values: vec![],
                index: 0,
            };
            let outcome = run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue(
                &typed_run(9), 0, sample_activation(), &identity(), handshake_limits(), sample_limits(),
                &AtomicBool::new(false), recovery_activation(),
                FiniteSampleRecoveryPolicy::new(1, 3, Duration::from_millis(1), Duration::from_millis(1), deadline).unwrap(),
                &AtomicBool::new(cancelled),
                |_, _| RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, 73),
                clock_activation(), correction_config("127.0.0.1:9".parse().unwrap()),
                &mut clock, &AtomicBool::new(false), &queue,
                BoundedSampleQueueWait::new(Duration::from_millis(1), Duration::from_millis(5)).unwrap(),
                &AtomicBool::new(false),
            ).unwrap();
            assert!(matches!(
                outcome,
                TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Cancelled { ref states }
                    | TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome::Deadline { ref states }
                    if states.len() == 1
            ));
            assert_eq!(clock.index, 0);
            assert!(queue.try_pop().is_err());
        }
    }
}
