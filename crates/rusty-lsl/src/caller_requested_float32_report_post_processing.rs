// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default-inert production entrypoint for admitted Float32 report post-processing.

use crate::caller_requested_float32_report_post_processing_admission::{
    CallerRequestedFloat32ReportPostProcessingAdmission,
    CallerRequestedFloat32ReportPostProcessingAdmissionError,
    CallerRequestedFloat32ReportPostProcessingPlan,
};
use crate::exact_sequence_loss_health::ExactSequenceLossHealthSnapshot;
use crate::float32_session_report_post_processing_batch::{
    Float32PostProcessingBatchConfigError, Float32PostProcessingBatchError,
    Float32PostProcessingBatchOutcome, Float32SessionReportPostProcessingBatch,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;
use crate::{
    TimestampedFloat32InletSessionReport, TypedUdpDiscoveryFloat32SessionConnectionError,
    TypedUdpDiscoveryRun,
};

/// Successful selected-discovery/session report processing result.
#[derive(Debug)]
pub(crate) struct SelectedCallerRequestedFloat32ReportPostProcessingOutcome<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    processing: Float32PostProcessingBatchOutcome,
}

impl<'a> SelectedCallerRequestedFloat32ReportPostProcessingOutcome<'a> {
    pub(crate) const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }

    pub(crate) const fn response_index(&self) -> usize {
        self.response_index
    }

    pub(crate) const fn processing(&self) -> &Float32PostProcessingBatchOutcome {
        &self.processing
    }

    /// Borrows the canonical processing result to project its exact committed health.
    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.processing.health()
    }

    pub(crate) fn into_processing(self) -> Float32PostProcessingBatchOutcome {
        self.processing
    }
}

/// Existing selected-session or transactional processing evidence.
#[derive(Debug)]
pub(crate) enum SelectedCallerRequestedFloat32ReportPostProcessingErrorKind {
    Session {
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
        error: TypedUdpDiscoveryFloat32SessionConnectionError,
    },
    Processing(CallerRequestedFloat32ReportPostProcessingError),
}

/// Selected discovery identity and the unchanged failing owner evidence.
#[derive(Debug)]
pub(crate) struct SelectedCallerRequestedFloat32ReportPostProcessingError<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    kind: SelectedCallerRequestedFloat32ReportPostProcessingErrorKind,
}

impl<'a> SelectedCallerRequestedFloat32ReportPostProcessingError<'a> {
    pub(crate) const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }

    pub(crate) const fn response_index(&self) -> usize {
        self.response_index
    }

    pub(crate) const fn kind(
        &self,
    ) -> &SelectedCallerRequestedFloat32ReportPostProcessingErrorKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> SelectedCallerRequestedFloat32ReportPostProcessingErrorKind {
        self.kind
    }
}

/// Pre-delegation or owner-preserving transactional refusal.
#[derive(Debug)]
pub(crate) enum CallerRequestedFloat32ReportPostProcessingError {
    Admission(CallerRequestedFloat32ReportPostProcessingAdmissionError),
    AdmissionMismatch {
        expected_request: RequestedTimestampPostProcessing,
        actual_request: RequestedTimestampPostProcessing,
        expected_maximum_records: usize,
        actual_maximum_records: usize,
        plan: CallerRequestedFloat32ReportPostProcessingPlan,
    },
    PostProcessing(Float32PostProcessingBatchError),
}

/// Sole production entrypoint owning one bounded P34 report transaction owner.
#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportPostProcessing {
    request: RequestedTimestampPostProcessing,
    batch: Float32SessionReportPostProcessingBatch,
}

impl CallerRequestedFloat32ReportPostProcessing {
    pub(crate) fn new(
        maximum_records: usize,
        request: RequestedTimestampPostProcessing,
    ) -> Result<Self, Float32PostProcessingBatchConfigError> {
        Ok(Self {
            request,
            batch: Float32SessionReportPostProcessingBatch::new(maximum_records, request)?,
        })
    }

    pub(crate) const fn request(&self) -> RequestedTimestampPostProcessing {
        self.request
    }

    pub(crate) const fn maximum_records(&self) -> usize {
        self.batch.maximum_records()
    }

    /// Borrows this owner to project its exact committed health without moving evidence.
    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.batch.health()
    }

    /// Processes only a canonically completed caller-selected session report.
    ///
    /// Discovery and its caller-selected index remain borrowed and unchanged. A session failure
    /// bypasses admission and processing; a completed report enters the existing transactional
    /// owner with the caller's explicit request and sequences.
    pub(crate) fn process_selected_session_report<'a>(
        &mut self,
        discovery: &'a TypedUdpDiscoveryRun,
        response_index: usize,
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
        session: Result<
            TimestampedFloat32InletSessionReport,
            TypedUdpDiscoveryFloat32SessionConnectionError,
        >,
    ) -> Result<
        SelectedCallerRequestedFloat32ReportPostProcessingOutcome<'a>,
        SelectedCallerRequestedFloat32ReportPostProcessingError<'a>,
    > {
        let report = match session {
            Ok(report) => report,
            Err(error) => {
                return Err(SelectedCallerRequestedFloat32ReportPostProcessingError {
                    discovery,
                    response_index,
                    kind: SelectedCallerRequestedFloat32ReportPostProcessingErrorKind::Session {
                        request,
                        sequences,
                        error,
                    },
                });
            }
        };
        let processing = self
            .process_requested_report(request, sequences, report)
            .map_err(
                |error| SelectedCallerRequestedFloat32ReportPostProcessingError {
                    discovery,
                    response_index,
                    kind: SelectedCallerRequestedFloat32ReportPostProcessingErrorKind::Processing(
                        error,
                    ),
                },
            )?;
        Ok(SelectedCallerRequestedFloat32ReportPostProcessingOutcome {
            discovery,
            response_index,
            processing,
        })
    }

    /// Admits one canonical report against this owner's bound, then processes it transactionally.
    pub(crate) fn process_requested_report(
        &mut self,
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    ) -> Result<Float32PostProcessingBatchOutcome, CallerRequestedFloat32ReportPostProcessingError>
    {
        let admission =
            CallerRequestedFloat32ReportPostProcessingAdmission::new(self.batch.maximum_records())
                .expect("a constructed post-processing owner has a representable nonzero bound");
        let plan = admission
            .admit(request, sequences, report)
            .map_err(CallerRequestedFloat32ReportPostProcessingError::Admission)?;
        self.process_report(plan)
    }

    /// Validates the admitted owner identity before consuming one plan in one P34 delegation.
    pub(crate) fn process_report(
        &mut self,
        plan: CallerRequestedFloat32ReportPostProcessingPlan,
    ) -> Result<Float32PostProcessingBatchOutcome, CallerRequestedFloat32ReportPostProcessingError>
    {
        let actual_request = plan.request();
        let actual_maximum_records = plan.maximum_records();
        if actual_request != self.request || actual_maximum_records != self.batch.maximum_records()
        {
            return Err(
                CallerRequestedFloat32ReportPostProcessingError::AdmissionMismatch {
                    expected_request: self.request,
                    actual_request,
                    expected_maximum_records: self.batch.maximum_records(),
                    actual_maximum_records,
                    plan,
                },
            );
        }

        let (_, _, sequences, report) = plan.into_parts();
        self.batch
            .process_report(sequences, report)
            .map_err(CallerRequestedFloat32ReportPostProcessingError::PostProcessing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmission;
    use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessingConfig;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamHandshakeIdentity, StreamHandshakeLimits, StreamInfoObservedAdmissionLimits,
        StreamInfoVolatileFieldLimits, TimestampedFloat32InletSession,
        TimestampedFloat32OutletSession, TimestampedFloat32SampleActivation,
        TimestampedFloat32SampleLimits, TimestampedSample, TypedUdpDiscoveryEndpointError,
        UdpDiscoveryActivation, UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::atomic::AtomicBool;
    use std::sync::mpsc::sync_channel;
    use std::thread;
    use std::time::Duration;

    fn request() -> RequestedTimestampPostProcessing {
        RequestedTimestampPostProcessing::DeJitter(
            RequestedTimestampPostProcessingConfig::new(4, 1.0, 10.0).unwrap(),
        )
    }

    fn report(timestamps: &[f64]) -> crate::TimestampedFloat32InletSessionReport {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let limits =
            StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
                .unwrap();
        let identity = StreamHandshakeIdentity::new(
            "35353535-2222-4333-8444-555555555555".into(),
            "p35-host".into(),
            "p35-source".into(),
            "p35-session".into(),
            limits,
        )
        .unwrap();
        let activation = TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap();
        let sample_limits =
            TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap();
        let records: Vec<_> = timestamps
            .iter()
            .enumerate()
            .map(|(index, timestamp)| {
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![index as f32]).unwrap(),
                    RawSourceTimestamp::new(*timestamp).unwrap(),
                    None,
                )
            })
            .collect();
        let worker_identity = identity.clone();
        let worker = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight(
                activation,
                listener,
                &worker_identity,
                limits,
                sample_limits,
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let report = TimestampedFloat32InletSession::preflight(
            activation,
            address,
            &identity,
            limits,
            sample_limits,
            timestamps.len(),
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        worker.join().unwrap();
        TcpListener::bind(address).unwrap();
        report
    }

    fn plan(
        maximum: usize,
        sequences: Vec<u64>,
        timestamps: &[f64],
    ) -> CallerRequestedFloat32ReportPostProcessingPlan {
        CallerRequestedFloat32ReportPostProcessingAdmission::new(maximum)
            .unwrap()
            .admit(request(), sequences, report(timestamps))
            .unwrap()
    }

    fn discovery() -> TypedUdpDiscoveryRun {
        let fields = [
            ("name", "selected"),
            ("type", "independent"),
            ("channel_count", "1"),
            ("channel_format", "float32"),
            ("source_id", "p55-source"),
            ("nominal_srate", "100.0000000000000"),
            ("version", "110"),
            ("created_at", "1"),
            ("uid", "35353535-2222-4333-8444-555555555555"),
            ("session_id", "p55-session"),
            ("hostname", "p55-host"),
            ("v4address", "127.0.0.1"),
            ("v4data_port", "43001"),
            ("v4service_port", "43002"),
            ("v6address", "2001:db8::10"),
            ("v6data_port", "43003"),
            ("v6service_port", "43004"),
        ];
        let mut document = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in fields {
            document.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        document.push_str("\t<desc />\n</info>\n");
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let bytes = document.len();
        let (ready_sender, ready_receiver) = sync_channel(0);
        let responder = thread::spawn(move || {
            ready_sender.send(()).unwrap();
            let mut query = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut query).unwrap();
            socket
                .send_to(format!("19\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        ready_receiver.recv().unwrap();
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

    #[test]
    fn p55r_selected_success_preserves_context_order_bits_allocations_and_health() {
        let discovery = discovery();
        let report = report(&[f64::from_bits(0x4024_0000_0000_0001), 11.0]);
        let pointers: Vec<_> = report
            .records()
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(2, request()).unwrap();
        let outcome = owner
            .process_selected_session_report(
                &discovery,
                0,
                request(),
                vec![u64::MIN, u64::MAX],
                Ok(report),
            )
            .unwrap();

        assert!(std::ptr::eq(outcome.discovery(), &discovery));
        assert_eq!(outcome.response_index(), 0);
        assert_eq!(outcome.health(), owner.health());
        for ((record, pointer), sequence) in outcome
            .processing()
            .records()
            .iter()
            .zip(pointers)
            .zip([u64::MIN, u64::MAX])
        {
            assert_eq!(record.sequence(), sequence);
            assert_eq!(
                record.processed().sample().sample().values().as_ptr(),
                pointer
            );
        }
        let processing = outcome.into_processing();
        assert_eq!(
            processing.records()[0]
                .processed()
                .facts()
                .effective_timestamp()
                .value()
                .to_bits(),
            0x4024_0000_0000_0001
        );
    }

    #[test]
    fn p55r_session_and_processing_mismatch_retain_context_inputs_and_rollback() {
        let discovery = discovery();
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(2, request()).unwrap();
        let before = owner.health();
        let session_error = TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(
            TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                index: 7,
                response_count: 1,
            },
        );
        let sequences = vec![91];
        let sequence_pointer = sequences.as_ptr();
        let error = owner
            .process_selected_session_report(
                &discovery,
                7,
                request(),
                sequences,
                Err(session_error),
            )
            .unwrap_err();
        assert!(std::ptr::eq(error.discovery(), &discovery));
        assert_eq!(error.response_index(), 7);
        assert!(matches!(
            error.into_kind(),
            SelectedCallerRequestedFloat32ReportPostProcessingErrorKind::Session {
                request: returned_request,
                sequences,
                error: TypedUdpDiscoveryFloat32SessionConnectionError::Endpoint(
                    TypedUdpDiscoveryEndpointError::ResponseUnavailable { index: 7, .. }
                ),
            } if returned_request == request() && sequences.as_ptr() == sequence_pointer
        ));
        assert_eq!(owner.health(), before);

        let mismatched_report = report(&[10.0, 11.0]);
        let record_pointer = mismatched_report.records()[0].sample().values().as_ptr();
        let mismatched_sequences = vec![91];
        let mismatched_sequence_pointer = mismatched_sequences.as_ptr();
        let error = owner
            .process_selected_session_report(
                &discovery,
                0,
                request(),
                mismatched_sequences,
                Ok(mismatched_report),
            )
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            SelectedCallerRequestedFloat32ReportPostProcessingErrorKind::Processing(
                CallerRequestedFloat32ReportPostProcessingError::Admission(
                    CallerRequestedFloat32ReportPostProcessingAdmissionError::SequenceExtentMismatch {
                        sequences,
                        report,
                        ..
                    }
                )
            ) if sequences.as_ptr() == mismatched_sequence_pointer
                && report.records()[0].sample().values().as_ptr() == record_pointer
        ));
        assert_eq!(owner.health(), before);

        let error = owner
            .process_selected_session_report(
                &discovery,
                0,
                request(),
                vec![20, 21],
                Ok(report(&[10.0, 10.0])),
            )
            .unwrap_err();
        assert!(matches!(
            error.kind(),
            SelectedCallerRequestedFloat32ReportPostProcessingErrorKind::Processing(
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(
                    Float32PostProcessingBatchError::Record { index: 1, .. }
                )
            )
        ));
        assert_eq!(owner.health(), before);
    }

    #[test]
    fn real_plan_success_preserves_order_extremes_and_record_allocations() {
        let plan = plan(2, vec![u64::MIN, u64::MAX], &[10.0, 11.0]);
        let sequence_pointer = plan.sequences().as_ptr();
        let record_pointers: Vec<_> = plan
            .report()
            .records()
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        assert_eq!(plan.sequences().as_ptr(), sequence_pointer);
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(2, request()).unwrap();
        let outcome = owner.process_report(plan).unwrap();
        assert_eq!(
            outcome
                .records()
                .iter()
                .map(|record| record.sequence())
                .collect::<Vec<_>>(),
            vec![u64::MIN, u64::MAX]
        );
        for (record, pointer) in outcome.records().iter().zip(record_pointers) {
            assert_eq!(
                record.processed().sample().sample().values().as_ptr(),
                pointer
            );
        }
    }

    #[test]
    fn requested_report_pipeline_admits_and_processes_the_canonical_report() {
        let report = report(&[10.0, 11.0]);
        let record_pointers: Vec<_> = report
            .records()
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let sequences = vec![40, 43];
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(2, request()).unwrap();

        let outcome = owner
            .process_requested_report(request(), sequences, report)
            .unwrap();

        assert_eq!(
            outcome
                .records()
                .iter()
                .map(|record| record.sequence())
                .collect::<Vec<_>>(),
            vec![40, 43]
        );
        for (record, pointer) in outcome.records().iter().zip(record_pointers) {
            assert_eq!(
                record.processed().sample().sample().values().as_ptr(),
                pointer
            );
        }
    }

    #[test]
    fn requested_report_pipeline_admission_refusal_retains_all_inputs() {
        let report = report(&[10.0, 11.0]);
        let record_pointer = report.records()[0].sample().values().as_ptr();
        let sequences = vec![40];
        let sequence_pointer = sequences.as_ptr();
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(2, request()).unwrap();

        match owner
            .process_requested_report(request(), sequences, report)
            .unwrap_err()
        {
            CallerRequestedFloat32ReportPostProcessingError::Admission(
                CallerRequestedFloat32ReportPostProcessingAdmissionError::SequenceExtentMismatch {
                    request: returned_request,
                    sequences,
                    report,
                    sequence_count: 1,
                    report_record_count: 2,
                },
            ) => {
                assert_eq!(returned_request, request());
                assert_eq!(sequences.as_ptr(), sequence_pointer);
                assert_eq!(
                    report.records()[0].sample().values().as_ptr(),
                    record_pointer
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(owner.batch.health().observation_count(), 0);
    }

    #[test]
    fn request_and_maximum_mismatch_return_the_intact_opaque_plan() {
        for mismatch_request in [false, true] {
            let plan = plan(2, vec![4, u64::MAX], &[10.0, 11.0]);
            let sequence_pointer = plan.sequences().as_ptr();
            let record_pointer = plan.report().records()[0].sample().values().as_ptr();
            let owner_request = if mismatch_request {
                RequestedTimestampPostProcessing::PassThrough
            } else {
                request()
            };
            let owner_maximum = if mismatch_request { 2 } else { 3 };
            let mut owner =
                CallerRequestedFloat32ReportPostProcessing::new(owner_maximum, owner_request)
                    .unwrap();
            match owner.process_report(plan).unwrap_err() {
                CallerRequestedFloat32ReportPostProcessingError::AdmissionMismatch {
                    plan, ..
                } => {
                    assert_eq!(plan.maximum_records(), 2);
                    assert_eq!(plan.request(), request());
                    assert_eq!(plan.sequences(), [4, u64::MAX]);
                    assert_eq!(plan.sequences().as_ptr(), sequence_pointer);
                    assert_eq!(
                        plan.report().records()[0].sample().values().as_ptr(),
                        record_pointer
                    );
                }
                other => panic!("unexpected error: {other:?}"),
            }
        }
    }

    #[test]
    fn failed_transaction_rolls_back_and_the_same_owner_then_succeeds() {
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(2, request()).unwrap();
        let before = owner.batch.health();
        match owner
            .process_report(plan(2, vec![20, 21], &[10.0, 10.0]))
            .unwrap_err()
        {
            CallerRequestedFloat32ReportPostProcessingError::PostProcessing(
                Float32PostProcessingBatchError::Record {
                    index,
                    sequence,
                    completed,
                    remaining_sequences,
                    remaining_records,
                    ..
                },
            ) => {
                assert_eq!((index, sequence), (1, 21));
                assert_eq!(completed.len(), 1);
                assert!(remaining_sequences.is_empty());
                assert!(remaining_records.is_empty());
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(owner.batch.health(), before);
        assert_eq!(
            owner
                .process_report(plan(2, vec![30, 31], &[10.0, 11.0]))
                .unwrap()
                .records()
                .len(),
            2
        );
    }

    #[test]
    fn repeated_real_reports_commit_in_order_without_ambient_activation() {
        let mut owner = CallerRequestedFloat32ReportPostProcessing::new(
            2,
            RequestedTimestampPostProcessing::PassThrough,
        )
        .unwrap();
        for sequence in [u64::MIN, u64::MAX] {
            let plan = CallerRequestedFloat32ReportPostProcessingAdmission::new(2)
                .unwrap()
                .admit(
                    RequestedTimestampPostProcessing::PassThrough,
                    vec![sequence],
                    report(&[sequence as f64 + 1.0]),
                )
                .unwrap();
            assert_eq!(
                owner.process_report(plan).unwrap().records()[0].sequence(),
                sequence
            );
        }
        assert_eq!(owner.batch.health().observation_count(), 2);
    }
}
