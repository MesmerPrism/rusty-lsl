// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Borrowed exact-health projection for Float32 session-report batches.

use crate::{
    BoundedFloat32PipelineError, Float32SessionReportBatchError, Float32SessionReportBatchOutcome,
    Float32SessionReportBatchTermination,
};

/// Existing terminal class of one completed or owner-preserving Float32 batch result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Float32SessionBatchHealthClassification {
    /// Every retained report record completed the pipeline.
    Complete,
    /// The completed inlet report retained no records.
    EmptyReport,
    /// Recovery cancellation preceded acquisition of the current record.
    Cancelled,
    /// The recovery deadline preceded acquisition of the current record.
    Deadline,
    /// Acquisition classified the current record failure as terminal.
    Terminal,
    /// Acquisition exhausted its bounded attempts for the current record.
    Exhausted,
    /// Recovery setup failed for the current record.
    RecoveryError,
    /// Clock or queue work failed for the current record.
    PipelineError,
    /// The existing batch adapter observed an impossible pipeline result.
    Invariant,
}

/// Immutable exact counts and terminal class borrowed from an existing Float32 batch result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Float32SessionBatchHealth {
    total_record_count: usize,
    completed_record_count: usize,
    remaining_record_count: usize,
    current_record_index: Option<usize>,
    classification: Float32SessionBatchHealthClassification,
}

impl Float32SessionBatchHealth {
    /// Projects successful batch completion without retaining or cloning its evidence.
    pub fn from_outcome(outcome: &Float32SessionReportBatchOutcome) -> Self {
        let completed = outcome.completed.len();
        Self::new(
            completed,
            completed,
            0,
            None,
            Float32SessionBatchHealthClassification::Complete,
        )
    }

    /// Projects an owner-preserving batch error without retaining or cloning its evidence.
    pub fn from_error(error: &Float32SessionReportBatchError) -> Self {
        match error {
            Float32SessionReportBatchError::EmptyReport { report } => Self::new(
                report.record_count(),
                0,
                report.record_count(),
                None,
                Float32SessionBatchHealthClassification::EmptyReport,
            ),
            Float32SessionReportBatchError::NotAcquired {
                index,
                termination,
                completed,
                remaining,
                ..
            } => Self::indexed(
                *index,
                completed.len(),
                remaining.len(),
                match termination {
                    Float32SessionReportBatchTermination::Cancelled => {
                        Float32SessionBatchHealthClassification::Cancelled
                    }
                    Float32SessionReportBatchTermination::Deadline => {
                        Float32SessionBatchHealthClassification::Deadline
                    }
                    Float32SessionReportBatchTermination::Terminal { .. } => {
                        Float32SessionBatchHealthClassification::Terminal
                    }
                    Float32SessionReportBatchTermination::Exhausted { .. } => {
                        Float32SessionBatchHealthClassification::Exhausted
                    }
                },
            ),
            Float32SessionReportBatchError::Recovery {
                index,
                completed,
                remaining,
                ..
            } => Self::indexed(
                *index,
                completed.len(),
                remaining.len(),
                Float32SessionBatchHealthClassification::RecoveryError,
            ),
            Float32SessionReportBatchError::Pipeline {
                index,
                error,
                completed,
                remaining,
                ..
            } => Self::indexed(
                *index,
                completed.len(),
                remaining.len() + usize::from(pipeline_error_retains_current(error)),
                Float32SessionBatchHealthClassification::PipelineError,
            ),
            Float32SessionReportBatchError::Invariant {
                index,
                completed,
                remaining,
                ..
            } => Self::indexed(
                *index,
                completed.len(),
                remaining.len(),
                Float32SessionBatchHealthClassification::Invariant,
            ),
        }
    }

    const fn indexed(
        index: usize,
        completed: usize,
        remaining: usize,
        classification: Float32SessionBatchHealthClassification,
    ) -> Self {
        Self::new(
            completed + remaining,
            completed,
            remaining,
            Some(index),
            classification,
        )
    }

    const fn new(
        total_record_count: usize,
        completed_record_count: usize,
        remaining_record_count: usize,
        current_record_index: Option<usize>,
        classification: Float32SessionBatchHealthClassification,
    ) -> Self {
        Self {
            total_record_count,
            completed_record_count,
            remaining_record_count,
            current_record_index,
            classification,
        }
    }

    /// Exact number of records represented by completed and remaining evidence.
    pub const fn total_record_count(&self) -> usize {
        self.total_record_count
    }

    /// Exact length of the completed prefix.
    pub const fn completed_record_count(&self) -> usize {
        self.completed_record_count
    }

    /// Exact number of records not present in the completed prefix.
    pub const fn remaining_record_count(&self) -> usize {
        self.remaining_record_count
    }

    /// Existing zero-based current record index for an indexed error.
    pub const fn current_record_index(&self) -> Option<usize> {
        self.current_record_index
    }

    /// Existing terminal classification, with no copied failure payload.
    pub const fn classification(&self) -> Float32SessionBatchHealthClassification {
        self.classification
    }
}

const fn pipeline_error_retains_current(error: &BoundedFloat32PipelineError) -> bool {
    matches!(
        error,
        BoundedFloat32PipelineError::Clock { .. } | BoundedFloat32PipelineError::Queue { .. }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_activation::test_capability, BoundedFloat32PipelineOutcome,
        BoundedSampleQueuePushError, FiniteSampleRecoveryError, FiniteSampleRecoveryState,
        Float32SessionReportRecordOutcome, RawSourceTimestamp, RecoveryAttemptFailure,
        RecoveryFailureClass, RuntimeModule, Sample, SampleLimits, StreamHandshakeActivation,
        StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedFloat32InletSession,
        TimestampedFloat32OutletSession, TimestampedFloat32SampleActivation,
        TimestampedFloat32SampleLimits, TimestampedFloat32SessionLimits, TimestampedSample,
    };
    use std::net::TcpListener;
    use std::sync::atomic::AtomicBool;
    use std::thread;
    use std::time::Duration;

    fn assert_health(
        health: Float32SessionBatchHealth,
        total: usize,
        completed: usize,
        remaining: usize,
        current: Option<usize>,
        classification: Float32SessionBatchHealthClassification,
    ) {
        assert_eq!(health.total_record_count(), total);
        assert_eq!(health.completed_record_count(), completed);
        assert_eq!(health.remaining_record_count(), remaining);
        assert_eq!(health.current_record_index(), current);
        assert_eq!(health.classification(), classification);
        assert_eq!(completed + remaining, total);
    }

    fn completed(indices: &[usize]) -> Vec<Float32SessionReportRecordOutcome> {
        indices
            .iter()
            .map(|index| Float32SessionReportRecordOutcome {
                index: *index,
                states: vec![FiniteSampleRecoveryState::Recovered { attempt: 1 }],
            })
            .collect()
    }

    fn record(index: usize) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![index as f32]).unwrap(),
            RawSourceTimestamp::new(index as f64).unwrap(),
            None,
        )
    }

    fn sample_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(2))
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

    fn completed_report(
        record: TimestampedSample<f32>,
    ) -> crate::TimestampedFloat32InletSessionReport {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                sample_activation(),
                listener,
                &identity(),
                handshake_limits(),
                TimestampedFloat32SampleLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(2),
                )
                .unwrap(),
                TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
                &[record],
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let report = TimestampedFloat32InletSession::preflight_bounded(
            sample_activation(),
            address,
            &identity(),
            handshake_limits(),
            TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(2))
                .unwrap(),
            TimestampedFloat32SessionLimits::new(1, 1).unwrap(),
            1,
            1,
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        worker.join().unwrap();
        report
    }

    #[test]
    fn successful_outcomes_are_complete_for_the_exact_completed_extent() {
        for count in [0, 1, 3, 7] {
            let outcome = Float32SessionReportBatchOutcome {
                completed: completed(&(0..count).collect::<Vec<_>>()),
            };
            assert_health(
                Float32SessionBatchHealth::from_outcome(&outcome),
                count,
                count,
                0,
                None,
                Float32SessionBatchHealthClassification::Complete,
            );
        }
    }

    #[test]
    fn every_pre_acquisition_termination_keeps_exact_prefix_and_current_suffix_counts() {
        let cases = [
            (
                Float32SessionReportBatchTermination::Cancelled,
                Float32SessionBatchHealthClassification::Cancelled,
            ),
            (
                Float32SessionReportBatchTermination::Deadline,
                Float32SessionBatchHealthClassification::Deadline,
            ),
            (
                Float32SessionReportBatchTermination::Terminal {
                    failure: RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, 71),
                },
                Float32SessionBatchHealthClassification::Terminal,
            ),
            (
                Float32SessionReportBatchTermination::Exhausted {
                    failure: RecoveryAttemptFailure::new(RecoveryFailureClass::Retryable, 72),
                },
                Float32SessionBatchHealthClassification::Exhausted,
            ),
        ];
        for (termination, classification) in cases {
            let remaining = vec![record(2), record(3), record(4)];
            let pointers: Vec<_> = remaining
                .iter()
                .map(|record| record.sample().values().as_ptr())
                .collect();
            let error = Float32SessionReportBatchError::NotAcquired {
                index: 2,
                termination,
                states: Vec::new(),
                completed: completed(&[0, 1]),
                remaining,
            };
            assert_health(
                Float32SessionBatchHealth::from_error(&error),
                5,
                2,
                3,
                Some(2),
                classification,
            );
            match error {
                Float32SessionReportBatchError::NotAcquired { remaining, .. } => {
                    assert_eq!(
                        remaining
                            .iter()
                            .map(|record| record.sample().values().as_ptr())
                            .collect::<Vec<_>>(),
                        pointers
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn error_classes_use_only_evidence_retained_by_real_variants() {
        let empty = Float32SessionReportBatchError::EmptyReport {
            report: completed_report(record(0)),
        };
        assert_health(
            Float32SessionBatchHealth::from_error(&empty),
            1,
            0,
            1,
            None,
            Float32SessionBatchHealthClassification::EmptyReport,
        );

        let recovery = Float32SessionReportBatchError::Recovery {
            index: 1,
            error: FiniteSampleRecoveryError::Allocation { requested: 3 },
            completed: completed(&[0]),
            remaining: vec![record(1), record(2)],
        };
        assert_health(
            Float32SessionBatchHealth::from_error(&recovery),
            3,
            1,
            2,
            Some(1),
            Float32SessionBatchHealthClassification::RecoveryError,
        );

        let pipeline_recovery = Float32SessionReportBatchError::Pipeline {
            index: 1,
            error: BoundedFloat32PipelineError::Recovery(FiniteSampleRecoveryError::Allocation {
                requested: 3,
            }),
            completed: completed(&[0]),
            remaining: vec![record(2)],
        };
        assert_health(
            Float32SessionBatchHealth::from_error(&pipeline_recovery),
            2,
            1,
            1,
            Some(1),
            Float32SessionBatchHealthClassification::PipelineError,
        );

        let retained = record(1);
        let retained_pointer = retained.sample().values().as_ptr();
        let pipeline_queue = Float32SessionReportBatchError::Pipeline {
            index: 1,
            error: BoundedFloat32PipelineError::Queue {
                error: BoundedSampleQueuePushError::Full(retained),
                states: Vec::new(),
            },
            completed: completed(&[0]),
            remaining: vec![record(2)],
        };
        assert_health(
            Float32SessionBatchHealth::from_error(&pipeline_queue),
            3,
            1,
            2,
            Some(1),
            Float32SessionBatchHealthClassification::PipelineError,
        );
        match pipeline_queue {
            Float32SessionReportBatchError::Pipeline {
                error:
                    BoundedFloat32PipelineError::Queue {
                        error: BoundedSampleQueuePushError::Full(sample),
                        ..
                    },
                ..
            } => assert_eq!(sample.sample().values().as_ptr(), retained_pointer),
            _ => unreachable!(),
        }
    }

    #[test]
    fn invariant_projection_borrows_without_changing_retained_allocations() {
        let completed = completed(&[0]);
        let states_pointer = completed[0].states.as_ptr();
        let error = Float32SessionReportBatchError::Invariant {
            index: 1,
            outcome: BoundedFloat32PipelineOutcome::Cancelled {
                states: Vec::with_capacity(4),
            },
            completed,
            remaining: Vec::new(),
        };
        assert_health(
            Float32SessionBatchHealth::from_error(&error),
            1,
            1,
            0,
            Some(1),
            Float32SessionBatchHealthClassification::Invariant,
        );
        match error {
            Float32SessionReportBatchError::Invariant { completed, .. } => {
                assert_eq!(completed[0].states.as_ptr(), states_pointer);
            }
            _ => unreachable!(),
        }
    }
}
