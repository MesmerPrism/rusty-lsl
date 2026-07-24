// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional composition of a completed P63 execution batch and per-cycle supervision.

use crate::{
    RequestedProcessingRecoveryQueueExecutionBatchOutcome,
    RequestedProcessingRecoveryQueueExecutionReport, RequestedProcessingRecoveryQueueSupervision,
    RequestedProcessingSupervisionError, RequestedProcessingSupervisionLimits,
};

/// A completed finite execution batch paired with exactly one validated supervision per cycle.
#[derive(Debug)]
pub struct CompleteRequestedProcessingRecoveryQueueExecutionBatch<'a> {
    batch: RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
    supervision: Vec<RequestedProcessingRecoveryQueueSupervision>,
}

impl<'a> CompleteRequestedProcessingRecoveryQueueExecutionBatch<'a> {
    /// Returns the unchanged completed execution batch.
    pub const fn batch(&self) -> &RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a> {
        &self.batch
    }

    /// Returns the ordered supervision values, one for each committed cycle.
    pub fn supervision(&self) -> &[RequestedProcessingRecoveryQueueSupervision] {
        &self.supervision
    }

    /// Consumes the composition without projecting either owner.
    pub fn into_parts(
        self,
    ) -> (
        RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
        Vec<RequestedProcessingRecoveryQueueSupervision>,
    ) {
        (self.batch, self.supervision)
    }
}

/// Transactional refusal retaining the unchanged completed batch.
#[derive(Debug)]
pub enum CompleteRequestedProcessingRecoveryQueueExecutionBatchError<'a> {
    /// The number of report series does not equal the exact committed cycle count.
    ReportSeriesCount {
        /// Unchanged completed batch.
        batch: RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
        /// Exact committed cycle count.
        expected: usize,
        /// Caller-supplied report-series count.
        actual: usize,
    },
    /// Bounded storage for the supervision values could not be reserved.
    Allocation {
        /// Unchanged completed batch.
        batch: RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
    },
    /// One cycle's immutable P62 report series was refused by the supervision owner.
    Supervision {
        /// Unchanged completed batch.
        batch: RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
        /// Zero-based committed cycle whose report series was refused.
        cycle: usize,
        /// Exact supervision refusal.
        error: RequestedProcessingSupervisionError,
    },
}

#[derive(Debug, Eq, PartialEq)]
enum SupervisionSeriesError {
    Allocation,
    Supervision {
        cycle: usize,
        error: RequestedProcessingSupervisionError,
    },
}

fn validate_supervision_series(
    limits: RequestedProcessingSupervisionLimits,
    report_series: &[&[RequestedProcessingRecoveryQueueExecutionReport]],
) -> Result<Vec<RequestedProcessingRecoveryQueueSupervision>, SupervisionSeriesError> {
    let mut supervision = Vec::new();
    supervision
        .try_reserve_exact(report_series.len())
        .map_err(|_| SupervisionSeriesError::Allocation)?;
    for (cycle, reports) in report_series.iter().enumerate() {
        let value = RequestedProcessingRecoveryQueueSupervision::new(limits, reports)
            .map_err(|error| SupervisionSeriesError::Supervision { cycle, error })?;
        supervision.push(value);
    }
    Ok(supervision)
}

/// Validates exactly one bounded P62 snapshot series for every committed batch cycle.
///
/// Distinct cycles are never treated as snapshots of one execution. The caller supplies the
/// already observed reports and remains responsible for associating each series with its cycle.
/// This data-only composition performs no execution, retry, scheduling, recovery, queue, clock,
/// processing, observation, or activation work.
pub fn complete_requested_processing_recovery_queue_execution_batch<'a>(
    batch: RequestedProcessingRecoveryQueueExecutionBatchOutcome<'a>,
    limits: RequestedProcessingSupervisionLimits,
    report_series: &[&[RequestedProcessingRecoveryQueueExecutionReport]],
) -> Result<
    CompleteRequestedProcessingRecoveryQueueExecutionBatch<'a>,
    CompleteRequestedProcessingRecoveryQueueExecutionBatchError<'a>,
> {
    let expected = batch.committed().len();
    if report_series.len() != expected {
        return Err(
            CompleteRequestedProcessingRecoveryQueueExecutionBatchError::ReportSeriesCount {
                batch,
                expected,
                actual: report_series.len(),
            },
        );
    }
    let supervision = match validate_supervision_series(limits, report_series) {
        Ok(value) => value,
        Err(SupervisionSeriesError::Allocation) => {
            return Err(
                CompleteRequestedProcessingRecoveryQueueExecutionBatchError::Allocation { batch },
            )
        }
        Err(SupervisionSeriesError::Supervision { cycle, error }) => {
            return Err(
                CompleteRequestedProcessingRecoveryQueueExecutionBatchError::Supervision {
                    batch,
                    cycle,
                    error,
                },
            )
        }
    };
    Ok(CompleteRequestedProcessingRecoveryQueueExecutionBatch { batch, supervision })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        RequestedProcessingExecutionReportLimits, RequestedProcessingExecutionStage,
        RequestedProcessingExecutionTermination,
    };

    fn report(
        completed: usize,
        termination: RequestedProcessingExecutionTermination,
    ) -> RequestedProcessingRecoveryQueueExecutionReport {
        RequestedProcessingRecoveryQueueExecutionReport::new(
            RequestedProcessingExecutionReportLimits::new(3, 2, 2).unwrap(),
            3,
            completed,
            termination,
        )
        .unwrap()
    }

    #[test]
    fn validates_each_cycle_as_its_own_ordered_snapshot_series() {
        let first = [report(
            0,
            RequestedProcessingExecutionTermination::Cancelled {
                stage: RequestedProcessingExecutionStage::Recovery,
                completed_recovery_attempts: 0,
                queue_len: None,
            },
        )];
        let second = [report(3, RequestedProcessingExecutionTermination::Complete)];
        let values = validate_supervision_series(
            RequestedProcessingSupervisionLimits::new(2).unwrap(),
            &[&first, &second],
        )
        .unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].last_completed_execution_count(), 0);
        assert_eq!(values[1].last_completed_execution_count(), 3);
    }

    #[test]
    fn reports_exact_cycle_for_invalid_series_without_reclassifying() {
        let valid = [report(3, RequestedProcessingExecutionTermination::Complete)];
        let empty = [];
        let error = validate_supervision_series(
            RequestedProcessingSupervisionLimits::new(1).unwrap(),
            &[&valid, &empty],
        )
        .unwrap_err();
        assert_eq!(
            error,
            SupervisionSeriesError::Supervision {
                cycle: 1,
                error: RequestedProcessingSupervisionError::EmptyReports,
            }
        );
    }
}
