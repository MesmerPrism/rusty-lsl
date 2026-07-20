// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected Float32 discovery/session/report-batch production composition.

use crate::{
    run_float32_inlet_session_report_batch_recovery_clock_queue,
    run_selected_typed_udp_discovery_float32_session_inlet, BoundedFloat32PipelineCancellation,
    BoundedSampleQueue, BoundedSampleQueueWait, ClockSource, FiniteSampleRecoveryActivation,
    FiniteSampleRecoveryPolicy, Float32SessionBatchHealth, Float32SessionReportBatchError,
    Float32SessionReportBatchOutcome, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, StreamHandshakeIdentity, StreamHandshakeLimits,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    TimestampedFloat32SessionLimits, TypedUdpDiscoveryFloat32SessionConnectionError,
    TypedUdpDiscoveryRun,
};
use std::sync::atomic::AtomicBool;

/// Successful selected-discovery Float32 session-batch production result.
#[derive(Debug)]
pub struct SelectedTypedUdpDiscoveryFloat32SessionBatchOutcome<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    batch: Float32SessionReportBatchOutcome,
}

impl<'a> SelectedTypedUdpDiscoveryFloat32SessionBatchOutcome<'a> {
    /// Returns the unchanged caller-owned completed discovery run.
    pub const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }

    /// Returns the caller-selected receive-order response index.
    pub const fn response_index(&self) -> usize {
        self.response_index
    }

    /// Borrows the canonical exact per-record completion evidence.
    pub const fn batch(&self) -> &Float32SessionReportBatchOutcome {
        &self.batch
    }

    /// Borrows this result to project exact health without moving its evidence.
    pub fn health(&self) -> Float32SessionBatchHealth {
        Float32SessionBatchHealth::from_outcome(&self.batch)
    }

    /// Consumes only this adapter and returns the canonical batch outcome unchanged.
    pub fn into_batch(self) -> Float32SessionReportBatchOutcome {
        self.batch
    }
}

/// Stage-specific failure preserving the existing session or batch owner unchanged.
#[derive(Debug)]
pub enum SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind {
    /// Selection, endpoint, contract, preflight, connect, transfer, or finish failed.
    Session(TypedUdpDiscoveryFloat32SessionConnectionError),
    /// The completed canonical report failed in the existing report-batch owner.
    Batch(Float32SessionReportBatchError),
}

/// Owner-preserving selected-discovery Float32 session-batch production failure.
#[derive(Debug)]
pub struct SelectedTypedUdpDiscoveryFloat32SessionBatchError<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    kind: SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind,
}

impl<'a> SelectedTypedUdpDiscoveryFloat32SessionBatchError<'a> {
    /// Returns the unchanged caller-owned completed discovery run.
    pub const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }

    /// Returns the caller-selected receive-order response index.
    pub const fn response_index(&self) -> usize {
        self.response_index
    }

    /// Borrows the existing stage-specific failure owner.
    pub const fn kind(&self) -> &SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind {
        &self.kind
    }

    /// Projects exact batch health when session completion reached the batch owner.
    pub fn health(&self) -> Option<Float32SessionBatchHealth> {
        match &self.kind {
            SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Session(_) => None,
            SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Batch(error) => {
                Some(Float32SessionBatchHealth::from_error(error))
            }
        }
    }

    /// Consumes only this adapter and returns the stage-specific failure unchanged.
    pub fn into_kind(self) -> SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind {
        self.kind
    }
}

/// Runs one caller-selected discovery response through the canonical Float32 production owners.
///
/// Selection and endpoint projection precede socket-free session preflight and the sole phased
/// session lifecycle. Only a canonically completed report reaches the existing actual-extent
/// report-batch recovery/clock/queue owner. This adapter borrows discovery identity and selection,
/// moves no record except through those existing owners, and adds no selection or retry policy.
#[allow(clippy::too_many_arguments)]
pub fn run_selected_typed_udp_discovery_float32_inlet_session_batch_recovery_clock_queue<'a, C>(
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: TimestampedFloat32SampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    session_limits: TimestampedFloat32SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    cancellation: BoundedFloat32PipelineCancellation<'_>,
) -> Result<
    SelectedTypedUdpDiscoveryFloat32SessionBatchOutcome<'a>,
    SelectedTypedUdpDiscoveryFloat32SessionBatchError<'a>,
>
where
    C: ClockSource,
{
    let report = run_selected_typed_udp_discovery_float32_session_inlet(
        discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        sample_limits,
        session_limits,
        channel_count,
        record_count,
        session_cancelled,
    )
    .map_err(|error| SelectedTypedUdpDiscoveryFloat32SessionBatchError {
        discovery,
        response_index,
        kind: SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Session(error),
    })?;

    let batch = run_float32_inlet_session_report_batch_recovery_clock_queue(
        report,
        recovery_activation,
        recovery_policy,
        clock_activation,
        clock_config,
        clock,
        queue,
        queue_wait,
        cancellation,
    )
    .map_err(|error| SelectedTypedUdpDiscoveryFloat32SessionBatchError {
        discovery,
        response_index,
        kind: SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Batch(error),
    })?;

    Ok(SelectedTypedUdpDiscoveryFloat32SessionBatchOutcome {
        discovery,
        response_index,
        batch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, BoundedSampleQueueActivation, BoundedSampleQueuePopError,
        Float32SessionBatchHealthClassification, MetadataTreeLimits, RawSourceTimestamp,
        RuntimeModule, Sample, SampleLimits, ShortInfoQuery, ShortInfoQueryWire,
        ShortInfoQueryWireLimits, ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits,
        StreamHandshakeActivation, StreamInfoObservedAdmissionLimits,
        StreamInfoVolatileFieldLimits, TimestampedFloat32OutletSession, TimestampedSample,
        TypedUdpDiscoveryEndpointError, UdpDiscoveryActivation, UdpDiscoveryConfig,
        UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::atomic::AtomicBool;
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
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(2))
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

    fn session_activation() -> TimestampedFloat32SampleActivation {
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
            session_activation(),
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
            session_activation(),
        )
        .unwrap()
    }

    fn recovery_policy() -> FiniteSampleRecoveryPolicy {
        FiniteSampleRecoveryPolicy::new(
            1,
            3,
            Duration::ZERO,
            Duration::from_millis(2),
            Duration::from_secs(2),
        )
        .unwrap()
    }

    fn queue_wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(30)).unwrap()
    }

    fn cancellation<'a>(
        recovery: &'a AtomicBool,
        clock: &'a AtomicBool,
        queue: &'a AtomicBool,
    ) -> BoundedFloat32PipelineCancellation<'a> {
        BoundedFloat32PipelineCancellation::new(recovery, clock, queue)
    }

    fn record(index: usize, channel_count: usize) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(channel_count).unwrap(),
                channel_count,
                (0..channel_count)
                    .map(|channel| {
                        f32::from_bits(0x3f80_0100 + (index * channel_count + channel) as u32)
                    })
                    .collect(),
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(0x4008_0000_0000_0100 + index as u64)).unwrap(),
            None,
        )
    }

    fn document(address: &str, port: u16, channel_count: usize) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", channel_count.to_string()),
            ("channel_format", "float32".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "11111111-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "session".to_owned()),
            ("hostname", "host".to_owned()),
            ("v4address", address.to_owned()),
            ("v4data_port", "43001".to_owned()),
            ("v4service_port", port.to_string()),
            ("v6address", "2001:db8::10".to_owned()),
            ("v6data_port", "43003".to_owned()),
            ("v6service_port", "43004".to_owned()),
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in fields {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn completed_discovery(document: String) -> TypedUdpDiscoveryRun {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let bytes = document.len();
        let responder = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut query).unwrap();
            socket
                .send_to(format!("19\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let query_limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, query_limits).unwrap(),
            query_limits,
        )
        .unwrap();
        let run = run_typed_udp_discovery(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    bytes + 32,
                    1,
                    Duration::from_millis(5),
                    Duration::from_secs(2),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            ),
            &query,
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            StreamInfoObservedAdmissionLimits::new(
                StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
                MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
                StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
            ),
        )
        .unwrap();
        responder.join().unwrap();
        run
    }

    fn correction(count: usize) -> (IntegratedClockCorrectionConfig, thread::JoinHandle<()>) {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer = socket.local_addr().unwrap();
        let responder = thread::spawn(move || {
            for _ in 0..count {
                let mut bytes = [0_u8; 256];
                let (length, source) = socket.recv_from(&mut bytes).unwrap();
                let text = std::str::from_utf8(&bytes[..length]).unwrap();
                let mut fields = text.split("\r\n").nth(1).unwrap().split(' ');
                let id = fields.next().unwrap();
                let t0 = fields.next().unwrap();
                socket
                    .send_to(format!(" {id} {t0} 4.0 4.0").as_bytes(), source)
                    .unwrap();
            }
        });
        (
            IntegratedClockCorrectionConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                peer,
                91,
                1,
                256,
                Duration::from_millis(5),
                Duration::from_secs(2),
            )
            .unwrap(),
            responder,
        )
    }

    fn unused_clock_config() -> IntegratedClockCorrectionConfig {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer = socket.local_addr().unwrap();
        drop(socket);
        IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            peer,
            91,
            1,
            256,
            Duration::from_millis(5),
            Duration::from_millis(30),
        )
        .unwrap()
    }

    #[test]
    fn p32_selection_and_endpoint_rejection_precede_session_and_pipeline_work() {
        let discovery = completed_discovery(document("0.0.0.0", 9, 1));
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let off = AtomicBool::new(false);
        let mut clock = SequenceClock {
            values: Vec::new(),
            index: 0,
        };
        let error =
            run_selected_typed_udp_discovery_float32_inlet_session_batch_recovery_clock_queue(
                &discovery,
                1,
                session_activation(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
                1,
                1,
                &off,
                recovery_activation(),
                recovery_policy(),
                clock_activation(),
                unused_clock_config(),
                &mut clock,
                &queue,
                queue_wait(),
                cancellation(&off, &off, &off),
            )
            .unwrap_err();
        assert!(std::ptr::eq(error.discovery(), &discovery));
        assert_eq!(error.response_index(), 1);
        assert!(error.health().is_none());
        assert!(matches!(
            error.kind(),
            SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Session(
                TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(
                    TypedUdpDiscoveryEndpointError::ResponseUnavailable { .. }
                )
            )
        ));

        let error =
            run_selected_typed_udp_discovery_float32_inlet_session_batch_recovery_clock_queue(
                &discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
                1,
                1,
                &off,
                recovery_activation(),
                recovery_policy(),
                clock_activation(),
                unused_clock_config(),
                &mut clock,
                &queue,
                queue_wait(),
                cancellation(&off, &off, &off),
            )
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Session(
                TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(
                    TypedUdpDiscoveryEndpointError::NonConcreteUnicastAddress
                )
            )
        ));
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }

    #[test]
    fn p32_actual_multi_record_extent_transfers_exact_bits_order_and_allocations_once() {
        let count = 3;
        let records: Vec<_> = (0..count).map(|index| record(index, 2)).collect();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, count).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port(), 2));
        let queue = BoundedSampleQueue::new(queue_activation(), count).unwrap();
        let (clock_config, correction) = correction(count);
        let mut clock = SequenceClock {
            values: (0..count)
                .flat_map(|index| [index as f64 * 2.0, index as f64 * 2.0 + 1.0])
                .collect(),
            index: 0,
        };
        let off = AtomicBool::new(false);
        let outcome =
            run_selected_typed_udp_discovery_float32_inlet_session_batch_recovery_clock_queue(
                &discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(2, count).unwrap(),
                2,
                count,
                &off,
                recovery_activation(),
                recovery_policy(),
                clock_activation(),
                clock_config,
                &mut clock,
                &queue,
                queue_wait(),
                cancellation(&off, &off, &off),
            )
            .unwrap();
        assert!(std::ptr::eq(outcome.discovery(), &discovery));
        assert_eq!(outcome.response_index(), 0);
        assert_eq!(outcome.batch().completed.len(), count);
        let health = outcome.health();
        assert_eq!(health.total_record_count(), count);
        assert_eq!(health.completed_record_count(), count);
        assert_eq!(health.remaining_record_count(), 0);
        assert_eq!(health.current_record_index(), None);
        assert_eq!(
            health.classification(),
            Float32SessionBatchHealthClassification::Complete
        );
        for index in 0..count {
            let queued = queue.try_pop().unwrap();
            assert_eq!(
                queued.raw_source_timestamp().value().to_bits(),
                0x4008_0000_0000_0100 + index as u64
            );
            assert_eq!(
                queued.sample().values()[0].to_bits(),
                0x3f80_0100 + (index * 2) as u32
            );
        }
        correction.join().unwrap();
        assert_eq!(outlet.join().unwrap().record_count(), count);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p32_recovery_cancellation_retains_current_suffix_exact_health_and_releases_session_endpoint()
    {
        let count = 2;
        let records: Vec<_> = (0..count).map(|index| record(index, 1)).collect();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(1, count).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port(), 1));
        let queue = BoundedSampleQueue::new(queue_activation(), count).unwrap();
        let off = AtomicBool::new(false);
        let recovery_cancelled = AtomicBool::new(true);
        let mut clock = SequenceClock {
            values: Vec::new(),
            index: 0,
        };
        let error =
            run_selected_typed_udp_discovery_float32_inlet_session_batch_recovery_clock_queue(
                &discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(1, count).unwrap(),
                1,
                count,
                &off,
                recovery_activation(),
                recovery_policy(),
                clock_activation(),
                unused_clock_config(),
                &mut clock,
                &queue,
                queue_wait(),
                cancellation(&recovery_cancelled, &off, &off),
            )
            .unwrap_err();
        let retained_pointers = match error.kind() {
            SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Batch(
                Float32SessionReportBatchError::NotAcquired { remaining, .. },
            ) => remaining
                .iter()
                .map(|record| record.sample().values().as_ptr())
                .collect::<Vec<_>>(),
            other => panic!("unexpected error: {other:?}"),
        };
        let health = error.health().unwrap();
        assert_eq!(health.total_record_count(), count);
        assert_eq!(health.completed_record_count(), 0);
        assert_eq!(health.remaining_record_count(), count);
        assert_eq!(health.current_record_index(), Some(0));
        assert_eq!(
            health.classification(),
            Float32SessionBatchHealthClassification::Cancelled
        );
        match error.into_kind() {
            SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind::Batch(
                Float32SessionReportBatchError::NotAcquired { remaining, .. },
            ) => assert_eq!(
                remaining
                    .iter()
                    .map(|record| record.sample().values().as_ptr())
                    .collect::<Vec<_>>(),
                retained_pointers
            ),
            other => panic!("unexpected error: {other:?}"),
        }
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
        assert_eq!(outlet.join().unwrap().record_count(), count);
        TcpListener::bind(endpoint).unwrap();
    }
}
