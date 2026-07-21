// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default-inert production entrypoint for admitted Float32 report post-processing.

use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingPlan;
use crate::float32_session_report_post_processing_batch::{
    Float32PostProcessingBatchConfigError, Float32PostProcessingBatchError,
    Float32PostProcessingBatchOutcome, Float32SessionReportPostProcessingBatch,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;

/// Pre-delegation or owner-preserving transactional refusal.
#[derive(Debug)]
pub(crate) enum CallerRequestedFloat32ReportPostProcessingError {
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
