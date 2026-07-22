// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default-inert production entrypoint for admitted Float32 report post-processing.

use crate::caller_requested_float32_report_post_processing_admission::{
    CallerRequestedFloat32ReportPostProcessingAdmission,
    CallerRequestedFloat32ReportPostProcessingAdmissionError,
    CallerRequestedFloat32ReportPostProcessingPlan,
};
use crate::float32_session_report_post_processing_batch::{
    Float32PostProcessingBatchConfigError, Float32PostProcessingBatchError,
    Float32PostProcessingBatchOutcome, Float32SessionReportPostProcessingBatch,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;
use crate::TimestampedFloat32InletSessionReport;

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
        RawSourceTimestamp, RuntimeModule, Sample, SampleLimits, StreamHandshakeActivation,
        StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedFloat32InletSession,
        TimestampedFloat32OutletSession, TimestampedFloat32SampleActivation,
        TimestampedFloat32SampleLimits, TimestampedSample,
    };
    use std::net::TcpListener;
    use std::sync::atomic::AtomicBool;
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
