// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete caller-named Float32 discovery/session lifecycle followed by one
//! caller-requested transactional report post-processing operation.

use crate::caller_requested_float32_report_post_processing::{
    CallerRequestedFloat32ReportPostProcessing, CallerRequestedFloat32ReportPostProcessingError,
};
use crate::exact_sequence_loss_health::ExactSequenceLossHealthSnapshot;
use crate::float32_session_report_post_processing_batch::{
    Float32PostProcessingBatchConfigError, Float32PostProcessingBatchOutcome,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;
use crate::{
    run_typed_udp_discovery_float32_session_inlet, ShortInfoQueryWire,
    ShortInfoResponseEnvelopeLimits, StreamHandshakeIdentity, StreamHandshakeLimits,
    StreamInfoObservedAdmissionLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits, TimestampedFloat32SessionLimits,
    TypedUdpDiscoveryFloat32CompleteLifecycleError, TypedUdpDiscoveryRun, UdpDiscoveryActivation,
    UdpDiscoveryConfig,
};
use std::sync::atomic::AtomicBool;

/// Successful complete lifecycle evidence and its committed processing result.
#[derive(Debug)]
pub struct CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle {
    discovery: TypedUdpDiscoveryRun,
    response_index: usize,
    processing: Float32PostProcessingBatchOutcome,
}

impl CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle {
    /// Borrows the exact completed bounded discovery run.
    pub const fn discovery(&self) -> &TypedUdpDiscoveryRun {
        &self.discovery
    }

    /// Returns the exact receive-order index selected by the caller's name.
    pub const fn response_index(&self) -> usize {
        self.response_index
    }

    /// Borrows the unchanged canonical transactional processing outcome.
    pub const fn processing(&self) -> &Float32PostProcessingBatchOutcome {
        &self.processing
    }

    /// Projects the exact health committed by the subordinate transaction owner.
    pub const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.processing.health()
    }

    /// Recovers every owned success component without copying allocations.
    pub fn into_parts(
        self,
    ) -> (
        TypedUdpDiscoveryRun,
        usize,
        Float32PostProcessingBatchOutcome,
    ) {
        (self.discovery, self.response_index, self.processing)
    }
}

/// Stage-specific failure preserving caller inputs and existing owner evidence.
#[derive(Debug)]
pub enum CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError {
    /// Discovery, selection, connection, transfer, completion, or cleanup failed.
    Lifecycle {
        /// Exact caller-requested processing mode, not consumed by processing.
        request: RequestedTimestampPostProcessing,
        /// Exact caller-supplied sequences, in unchanged allocation and order.
        sequences: Vec<u64>,
        /// Existing stage-specific complete lifecycle error.
        error: TypedUdpDiscoveryFloat32CompleteLifecycleError,
    },
    /// The completed report was transactionally refused.
    Processing {
        /// Exact completed discovery evidence.
        discovery: TypedUdpDiscoveryRun,
        /// Exact receive-order selected response index.
        response_index: usize,
        /// Existing owner-preserving admission or processing refusal.
        error: CallerRequestedFloat32ReportPostProcessingError,
    },
}

/// Sole composition owner retaining the existing transactional processing owner.
#[derive(Debug, PartialEq)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing {
    processing: CallerRequestedFloat32ReportPostProcessing,
}

impl CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing {
    /// Constructs an explicitly bounded owner for exactly one requested mode.
    pub fn new(
        maximum_records: usize,
        request: RequestedTimestampPostProcessing,
    ) -> Result<Self, Float32PostProcessingBatchConfigError> {
        Ok(Self {
            processing: CallerRequestedFloat32ReportPostProcessing::new(maximum_records, request)?,
        })
    }

    /// Returns the caller-requested mode bound to this transaction owner.
    pub const fn request(&self) -> RequestedTimestampPostProcessing {
        self.processing.request()
    }

    /// Returns the exact per-transaction report bound.
    pub const fn maximum_records(&self) -> usize {
        self.processing.maximum_records()
    }

    /// Borrows the currently committed exact sequence/processing health.
    pub const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.processing.health()
    }

    /// Runs bounded exact-name discovery and the canonical Float32 session, then
    /// delegates the completed report to the existing all-or-nothing owner.
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &mut self,
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
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
    ) -> Result<
        CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError,
    > {
        let completed = match run_typed_udp_discovery_float32_session_inlet(
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
        ) {
            Ok(completed) => completed,
            Err(error) => {
                return Err(
                    CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError::Lifecycle {
                        request,
                        sequences,
                        error,
                    },
                );
            }
        };
        let (discovery, response_index, report) = completed.into_parts();
        let processing = match self
            .processing
            .process_requested_report(request, sequences, report)
        {
            Ok(processing) => processing,
            Err(error) => {
                return Err(
                    CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError::Processing {
                        discovery,
                        response_index,
                        error,
                    },
                );
            }
        };
        Ok(
            CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle {
                discovery,
                response_index,
                processing,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmissionError;
    use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessingConfig;
    use crate::runtime_activation::test_capability;
    use crate::{
        MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample, SampleLimits,
        ShortInfoQuery, ShortInfoQueryWireLimits, StreamDescriptorLimits,
        StreamHandshakeActivation, StreamInfoVolatileFieldLimits, TimestampedFloat32OutletSession,
        TimestampedSample, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn request() -> RequestedTimestampPostProcessing {
        RequestedTimestampPostProcessing::DeJitter(
            RequestedTimestampPostProcessingConfig::new(4, 1.0, 10.0).unwrap(),
        )
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(5))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "59595959-2222-4333-8444-555555555555".into(),
            "p59-host".into(),
            "p59-source".into(),
            "p59-session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn discovery_activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap()
    }

    fn session_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, limits).unwrap(),
            limits,
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

    fn document(port: u16) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", "1".to_owned()),
            ("channel_format", "float32".to_owned()),
            ("source_id", "p59-source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "59595959-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "p59-session".to_owned()),
            ("hostname", "p59-host".to_owned()),
            ("v4address", "127.0.0.1".to_owned()),
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

    fn run(
        owner: &mut CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing,
        sequences: Vec<u64>,
    ) -> Result<
        CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError,
    > {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let records = vec![
            TimestampedSample::new(
                Sample::new(
                    SampleLimits::new(1).unwrap(),
                    1,
                    vec![f32::from_bits(0x3fc0_0001)],
                )
                .unwrap(),
                RawSourceTimestamp::new(10.0).unwrap(),
                None,
            ),
            TimestampedSample::new(
                Sample::new(
                    SampleLimits::new(1).unwrap(),
                    1,
                    vec![f32::from_bits(0xc020_0001)],
                )
                .unwrap(),
                RawSourceTimestamp::new(11.0).unwrap(),
                None,
            ),
        ];
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                TimestampedFloat32SampleLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(1),
                )
                .unwrap(),
                TimestampedFloat32SessionLimits::new(1, 2).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let document = document(endpoint.port());
        let bytes = document.len();
        let responder = thread::spawn(move || {
            let mut buffer = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut buffer).unwrap();
            socket
                .send_to(format!("19\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let result = owner.run(
            discovery_activation(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    bytes + 32,
                    1,
                    Duration::from_millis(5),
                    Duration::from_millis(250),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            ),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            admission_limits(),
            "selected",
            session_activation(),
            &identity(),
            handshake_limits(),
            TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap(),
            TimestampedFloat32SessionLimits::new(1, 2).unwrap(),
            1,
            2,
            &AtomicBool::new(false),
            request(),
            sequences,
        );
        responder.join().unwrap();
        outlet.join().unwrap();
        TcpListener::bind(endpoint).unwrap();
        result
    }

    #[test]
    fn p59_success_retains_discovery_selection_bits_sequences_and_committed_health() {
        let mut owner =
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing::new(2, request()).unwrap();
        let outcome = run(&mut owner, vec![u64::MIN, u64::MAX]).unwrap();
        assert_eq!(outcome.response_index(), 0);
        assert_eq!(outcome.discovery().responses().len(), 1);
        assert_eq!(outcome.health(), owner.health());
        assert_eq!(
            outcome
                .processing()
                .records()
                .iter()
                .map(|record| record.sequence())
                .collect::<Vec<_>>(),
            vec![u64::MIN, u64::MAX]
        );
        assert_eq!(
            outcome.processing().records()[0]
                .processed()
                .sample()
                .sample()
                .values()[0]
                .to_bits(),
            0x3fc0_0001
        );
    }

    #[test]
    fn p59_sequence_refusal_retains_transaction_evidence_and_prior_health() {
        let mut owner =
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing::new(2, request()).unwrap();
        let before = owner.health();
        let error = run(&mut owner, vec![77]).unwrap_err();
        match error {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError::Processing {
                discovery, response_index: 0,
                error: CallerRequestedFloat32ReportPostProcessingError::Admission(
                    CallerRequestedFloat32ReportPostProcessingAdmissionError::SequenceExtentMismatch {
                        sequence_count: 1, report_record_count: 2, sequences, report, ..
                    }),
            } => {
                assert_eq!(discovery.responses().len(), 1);
                assert_eq!(sequences, [77]);
                assert_eq!(report.record_count(), 2);
                assert_eq!(report.records()[0].sample().values()[0].to_bits(), 0x3fc0_0001);
            }
            other => panic!("unexpected refusal: {other:?}"),
        }
        assert_eq!(owner.health(), before);
    }
}
