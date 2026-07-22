// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete caller-named Float32 discovery/session/report-batch composition.

use crate::{
    run_float32_inlet_session_report_batch_recovery_clock_queue,
    run_typed_udp_discovery_float32_session_inlet, BoundedFloat32PipelineCancellation,
    BoundedSampleQueue, BoundedSampleQueueWait, ClockSource, FiniteSampleRecoveryActivation,
    FiniteSampleRecoveryPolicy, Float32SessionBatchHealth, Float32SessionReportBatchError,
    Float32SessionReportBatchOutcome, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, ShortInfoQueryWire, ShortInfoResponseEnvelopeLimits,
    StreamHandshakeIdentity, StreamHandshakeLimits, StreamInfoObservedAdmissionLimits,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    TimestampedFloat32SessionLimits, TypedUdpDiscoveryFloat32CompleteLifecycleError,
    TypedUdpDiscoveryRun, UdpDiscoveryActivation, UdpDiscoveryConfig,
};
use std::sync::atomic::AtomicBool;

/// Successful complete Float32 discovery/session/report-batch evidence.
#[derive(Debug)]
pub struct CompleteTypedUdpDiscoveryFloat32BatchOutcome {
    discovery: TypedUdpDiscoveryRun,
    response_index: usize,
    batch: Float32SessionReportBatchOutcome,
}

impl CompleteTypedUdpDiscoveryFloat32BatchOutcome {
    /// Borrows the unchanged completed discovery run.
    pub const fn discovery(&self) -> &TypedUdpDiscoveryRun {
        &self.discovery
    }

    /// Returns the exact receive-order response selected by the existing exact-name owner.
    pub const fn response_index(&self) -> usize {
        self.response_index
    }

    /// Borrows the canonical exact per-record batch completion evidence.
    pub const fn batch(&self) -> &Float32SessionReportBatchOutcome {
        &self.batch
    }

    /// Projects exact health without moving or duplicating retained evidence.
    pub fn health(&self) -> Float32SessionBatchHealth {
        Float32SessionBatchHealth::from_outcome(&self.batch)
    }

    /// Recovers every owned result without copying.
    pub fn into_parts(
        self,
    ) -> (
        TypedUdpDiscoveryRun,
        usize,
        Float32SessionReportBatchOutcome,
    ) {
        (self.discovery, self.response_index, self.batch)
    }
}

/// Stage-specific complete Float32 discovery/session/report-batch failure.
#[derive(Debug)]
pub enum CompleteTypedUdpDiscoveryFloat32BatchError {
    /// Discovery, exact-name selection, selected response, or canonical session completion failed.
    Lifecycle(TypedUdpDiscoveryFloat32CompleteLifecycleError),
    /// The completed canonical report failed in the existing report-batch owner.
    Batch {
        /// Unchanged completed discovery evidence.
        discovery: TypedUdpDiscoveryRun,
        /// Exact selected receive-order response index.
        response_index: usize,
        /// Existing owner-preserving report-batch failure.
        error: Float32SessionReportBatchError,
    },
}

impl CompleteTypedUdpDiscoveryFloat32BatchError {
    /// Projects exact batch health only when execution reached the batch owner.
    pub fn health(&self) -> Option<Float32SessionBatchHealth> {
        match self {
            Self::Lifecycle(_) => None,
            Self::Batch { error, .. } => Some(Float32SessionBatchHealth::from_error(error)),
        }
    }
}

/// Runs exact caller-named bounded discovery through the canonical Float32 batch production owner.
///
/// All limits, cancellation inputs, activations, identities, shape, clock, recovery policy, and
/// queue policy remain caller-explicit. This adapter delegates discovery through canonical session
/// completion first, then moves only that report into the existing actual-extent batch owner.
#[allow(clippy::too_many_arguments)]
pub fn run_complete_typed_udp_discovery_float32_batch_lifecycle<C>(
    discovery_activation: UdpDiscoveryActivation,
    discovery_config: UdpDiscoveryConfig,
    query: &ShortInfoQueryWire,
    discovery_cancelled: &AtomicBool,
    envelope_limits: ShortInfoResponseEnvelopeLimits,
    admission_limits: StreamInfoObservedAdmissionLimits,
    stream_name: &str,
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
) -> Result<CompleteTypedUdpDiscoveryFloat32BatchOutcome, CompleteTypedUdpDiscoveryFloat32BatchError>
where
    C: ClockSource,
{
    let completed = run_typed_udp_discovery_float32_session_inlet(
        discovery_activation,
        discovery_config,
        query,
        discovery_cancelled,
        envelope_limits,
        admission_limits,
        stream_name,
        session_activation,
        expected_identity,
        handshake_limits,
        sample_limits,
        session_limits,
        channel_count,
        record_count,
        session_cancelled,
    )
    .map_err(CompleteTypedUdpDiscoveryFloat32BatchError::Lifecycle)?;
    let (discovery, response_index, report) = completed.into_parts();
    let batch = match run_float32_inlet_session_report_batch_recovery_clock_queue(
        report,
        recovery_activation,
        recovery_policy,
        clock_activation,
        clock_config,
        clock,
        queue,
        queue_wait,
        cancellation,
    ) {
        Ok(batch) => batch,
        Err(error) => {
            return Err(CompleteTypedUdpDiscoveryFloat32BatchError::Batch {
                discovery,
                response_index,
                error,
            });
        }
    };

    Ok(CompleteTypedUdpDiscoveryFloat32BatchOutcome {
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
        BoundedSampleQueueActivation, Float32SessionBatchHealthClassification, MetadataTreeLimits,
        RawSourceTimestamp, RuntimeModule, Sample, SampleLimits, ShortInfoQuery,
        ShortInfoQueryWireLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamInfoVolatileFieldLimits, TimestampedFloat32OutletSession, TimestampedSample,
        TypedUdpDiscoveryFloat32CompleteLifecycleError, UdpDiscoveryLimits,
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

    fn admission_limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
        )
    }

    fn document(name: &str, port: u16) -> String {
        format!(
            "<?xml version=\"1.0\"?>\n<info>\n\t<name>{name}</name>\n\t<type>independent</type>\n\t<channel_count>1</channel_count>\n\t<channel_format>float32</channel_format>\n\t<source_id>source</source_id>\n\t<nominal_srate>100.0000000000000</nominal_srate>\n\t<version>110</version>\n\t<created_at>1</created_at>\n\t<uid>11111111-2222-4333-8444-555555555555</uid>\n\t<session_id>session</session_id>\n\t<hostname>host</hostname>\n\t<v4address>127.0.0.1</v4address>\n\t<v4data_port>43001</v4data_port>\n\t<v4service_port>{port}</v4service_port>\n\t<v6address>2001:db8::10</v6address>\n\t<v6data_port>43003</v6data_port>\n\t<v6service_port>43004</v6service_port>\n\t<desc />\n</info>\n"
        )
    }

    fn discovery(
        local_address: SocketAddr,
        document: String,
    ) -> (
        UdpDiscoveryConfig,
        ShortInfoQueryWire,
        ShortInfoResponseEnvelopeLimits,
        thread::JoinHandle<()>,
    ) {
        let responder = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = responder.local_addr().unwrap();
        let bytes = document.len();
        let worker = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = responder.recv_from(&mut query).unwrap();
            responder
                .send_to(format!("19\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let wire_limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, wire_limits).unwrap(),
            wire_limits,
        )
        .unwrap();
        let envelope = ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap();
        (
            UdpDiscoveryConfig::new(
                local_address,
                destination,
                UdpDiscoveryLimits::new(
                    bytes + 32,
                    1,
                    Duration::from_millis(5),
                    Duration::from_secs(2),
                )
                .unwrap(),
                envelope,
            ),
            query,
            envelope,
            worker,
        )
    }

    fn correction() -> (IntegratedClockCorrectionConfig, thread::JoinHandle<()>) {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer = socket.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (length, source) = socket.recv_from(&mut bytes).unwrap();
            let text = std::str::from_utf8(&bytes[..length]).unwrap();
            let mut fields = text.split("\r\n").nth(1).unwrap().split(' ');
            let id = fields.next().unwrap();
            let t0 = fields.next().unwrap();
            socket
                .send_to(format!(" {id} {t0} 4.0 4.0").as_bytes(), source)
                .unwrap();
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
            worker,
        )
    }

    #[test]
    fn p59_complete_success_retains_selection_batch_health_and_exact_queued_bits() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            let record = TimestampedSample::new(
                Sample::new(
                    SampleLimits::new(1).unwrap(),
                    1,
                    vec![f32::from_bits(0x7fc0_5901)],
                )
                .unwrap(),
                RawSourceTimestamp::new(f64::from_bits(0x4008_0000_0000_5901)).unwrap(),
                None,
            );
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
                &[record],
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let discovery_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let discovery_address = discovery_socket.local_addr().unwrap();
        drop(discovery_socket);
        let (config, query, envelope, discovery_worker) =
            discovery(discovery_address, document("selected", endpoint.port()));
        let (clock_config, correction_worker) = correction();
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let off = AtomicBool::new(false);
        let mut clock = SequenceClock {
            values: vec![0.0, 1.0],
            index: 0,
        };
        let result = run_complete_typed_udp_discovery_float32_batch_lifecycle(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap(),
            config,
            &query,
            &off,
            envelope,
            admission_limits(),
            "selected",
            session_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
            1,
            1,
            &off,
            recovery_activation(),
            FiniteSampleRecoveryPolicy::new(
                1,
                3,
                Duration::ZERO,
                Duration::from_millis(2),
                Duration::from_secs(2),
            )
            .unwrap(),
            clock_activation(),
            clock_config,
            &mut clock,
            &queue,
            BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(30))
                .unwrap(),
            BoundedFloat32PipelineCancellation::new(&off, &off, &off),
        )
        .unwrap();
        assert_eq!(result.discovery().responses().len(), 1);
        assert_eq!(result.response_index(), 0);
        assert_eq!(result.batch().completed.len(), 1);
        assert_eq!(
            result.health().classification(),
            Float32SessionBatchHealthClassification::Complete
        );
        let queued = queue.try_pop().unwrap();
        assert_eq!(queued.sample().values()[0].to_bits(), 0x7fc0_5901);
        assert_eq!(
            queued.raw_source_timestamp().value().to_bits(),
            0x4008_0000_0000_5901
        );
        discovery_worker.join().unwrap();
        correction_worker.join().unwrap();
        outlet.join().unwrap();
        TcpListener::bind(endpoint).unwrap();
        UdpSocket::bind(discovery_address).unwrap();
    }

    #[test]
    fn p59_exact_name_refusal_retains_discovery_and_releases_udp_port() {
        let discovery_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let discovery_address = discovery_socket.local_addr().unwrap();
        drop(discovery_socket);
        let (config, query, envelope, worker) = discovery(discovery_address, document("other", 9));
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let off = AtomicBool::new(false);
        let mut clock = SequenceClock {
            values: Vec::new(),
            index: 0,
        };
        let error = run_complete_typed_udp_discovery_float32_batch_lifecycle(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap(),
            config,
            &query,
            &off,
            envelope,
            admission_limits(),
            "selected",
            session_activation(),
            &identity(),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
            1,
            1,
            &off,
            recovery_activation(),
            FiniteSampleRecoveryPolicy::new(
                1,
                3,
                Duration::ZERO,
                Duration::from_millis(2),
                Duration::from_secs(2),
            )
            .unwrap(),
            clock_activation(),
            IntegratedClockCorrectionConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                "127.0.0.1:9".parse().unwrap(),
                91,
                1,
                256,
                Duration::from_millis(5),
                Duration::from_millis(20),
            )
            .unwrap(),
            &mut clock,
            &queue,
            BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(20))
                .unwrap(),
            BoundedFloat32PipelineCancellation::new(&off, &off, &off),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            CompleteTypedUdpDiscoveryFloat32BatchError::Lifecycle(
                TypedUdpDiscoveryFloat32CompleteLifecycleError::NoMatchingStreamName {
                    ref stream_name,
                    ref discovery,
                }
            ) if stream_name == "selected" && discovery.responses().len() == 1
        ));
        assert!(error.health().is_none());
        assert_eq!(clock.index, 0);
        assert!(queue.try_pop().is_err());
        worker.join().unwrap();
        UdpSocket::bind(discovery_address).unwrap();
    }
}
