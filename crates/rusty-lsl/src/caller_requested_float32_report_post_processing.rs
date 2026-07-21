// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default-inert production entrypoint for admitted Float32 report post-processing.

use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmission;
use crate::float32_session_report_post_processing_batch::{
    Float32PostProcessingBatchConfigError, Float32PostProcessingBatchError,
    Float32PostProcessingBatchOutcome, Float32SessionReportPostProcessingBatch,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;
use crate::TimestampedFloat32InletSessionReport;

/// Pre-delegation or owner-preserving transactional refusal.
#[derive(Debug)]
pub(crate) enum CallerRequestedFloat32ReportPostProcessingError {
    AdmissionMismatch {
        expected_request: RequestedTimestampPostProcessing,
        actual_request: RequestedTimestampPostProcessing,
        expected_maximum_records: usize,
        actual_maximum_records: usize,
        admission: CallerRequestedFloat32ReportPostProcessingAdmission,
        report: TimestampedFloat32InletSessionReport,
    },
    ExtentMismatch {
        admitted_record_count: usize,
        report_record_count: usize,
        admission: CallerRequestedFloat32ReportPostProcessingAdmission,
        report: TimestampedFloat32InletSessionReport,
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

    /// Validates the admitted identity and exact report extent before one P34 delegation.
    pub(crate) fn process_report(
        &mut self,
        admission: CallerRequestedFloat32ReportPostProcessingAdmission,
        report: TimestampedFloat32InletSessionReport,
    ) -> Result<Float32PostProcessingBatchOutcome, CallerRequestedFloat32ReportPostProcessingError>
    {
        let actual_request = admission.request();
        let actual_maximum_records = admission.maximum_records();
        if actual_request != self.request || actual_maximum_records != self.batch.maximum_records()
        {
            return Err(
                CallerRequestedFloat32ReportPostProcessingError::AdmissionMismatch {
                    expected_request: self.request,
                    actual_request,
                    expected_maximum_records: self.batch.maximum_records(),
                    actual_maximum_records,
                    admission,
                    report,
                },
            );
        }

        let admitted_record_count = admission.record_count();
        let report_record_count = report.record_count();
        if admitted_record_count != report_record_count {
            return Err(
                CallerRequestedFloat32ReportPostProcessingError::ExtentMismatch {
                    admitted_record_count,
                    report_record_count,
                    admission,
                    report,
                },
            );
        }

        let (_, _, _, sequences) = admission.into_parts();
        self.batch
            .process_report(sequences, report)
            .map_err(CallerRequestedFloat32ReportPostProcessingError::PostProcessing)
    }
}
