// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded all-or-nothing post-processing and exact-health owner for Float32 reports.

use crate::exact_sequence_loss_health::{
    ExactSequenceClassification, ExactSequenceLossHealth, ExactSequenceLossHealthSnapshot,
};
use crate::requested_timestamp_post_processing::{
    RequestedTimestampPostProcessed, RequestedTimestampPostProcessing,
    RequestedTimestampPostProcessingConfigError, RequestedTimestampPostProcessor,
};
use crate::requested_timestamp_post_processing_loss_health::{
    process_requested_timestamp_and_observe_exact_health,
    RequestedTimestampPostProcessingLossHealthError,
};
use crate::{TimestampedFloat32InletSessionReport, TimestampedSample};

/// One ordered retained successful record and its exact caller-sequence observation.
#[derive(Debug, PartialEq)]
pub(crate) struct Float32PostProcessingBatchRecordOutcome {
    index: usize,
    sequence: u64,
    processed: RequestedTimestampPostProcessed<f32>,
    classification: ExactSequenceClassification,
    health: ExactSequenceLossHealthSnapshot,
}

impl Float32PostProcessingBatchRecordOutcome {
    pub(crate) const fn index(&self) -> usize {
        self.index
    }
    pub(crate) const fn sequence(&self) -> u64 {
        self.sequence
    }
    pub(crate) const fn processed(&self) -> &RequestedTimestampPostProcessed<f32> {
        &self.processed
    }
    pub(crate) const fn classification(&self) -> ExactSequenceClassification {
        self.classification
    }
    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.health
    }
    pub(crate) fn into_processed(self) -> RequestedTimestampPostProcessed<f32> {
        self.processed
    }
}

/// Complete ordered result after both candidate owners commit exactly once.
#[derive(Debug, PartialEq)]
pub(crate) struct Float32PostProcessingBatchOutcome {
    records: Vec<Float32PostProcessingBatchRecordOutcome>,
    health: ExactSequenceLossHealthSnapshot,
}

impl Float32PostProcessingBatchOutcome {
    pub(crate) fn records(&self) -> &[Float32PostProcessingBatchRecordOutcome] {
        &self.records
    }
    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.health
    }
    pub(crate) fn into_records(self) -> Vec<Float32PostProcessingBatchRecordOutcome> {
        self.records
    }
}

/// Construction refusal before a batch owner exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32PostProcessingBatchConfigError {
    ZeroMaximumRecords,
    MaximumRecordsUnrepresentable { requested: usize },
    PostProcessing(RequestedTimestampPostProcessingConfigError),
}

/// Owner-preserving batch refusal.
#[derive(Debug, PartialEq)]
pub(crate) enum Float32PostProcessingBatchError {
    Empty {
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
    },
    SequenceCount {
        sequence_count: usize,
        record_count: usize,
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
    },
    RecordLimit {
        limit: usize,
        actual: usize,
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
    },
    Allocation {
        requested: usize,
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
    },
    Record {
        index: usize,
        sequence: u64,
        error: RequestedTimestampPostProcessingLossHealthError<f32>,
        completed: Vec<Float32PostProcessingBatchRecordOutcome>,
        remaining_sequences: Vec<u64>,
        remaining_records: Vec<TimestampedSample<f32>>,
    },
    CounterOverflow {
        index: usize,
        sequence: u64,
        current: RequestedTimestampPostProcessed<f32>,
        completed: Vec<Float32PostProcessingBatchRecordOutcome>,
        remaining_sequences: Vec<u64>,
        remaining_records: Vec<TimestampedSample<f32>>,
    },
}

/// Sole bounded lifecycle owner for P33 post-processing plus exact health.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Float32SessionReportPostProcessingBatch {
    maximum_records: usize,
    processor: RequestedTimestampPostProcessor,
    health: ExactSequenceLossHealth,
}

impl Float32SessionReportPostProcessingBatch {
    pub(crate) fn new(
        maximum_records: usize,
        request: RequestedTimestampPostProcessing,
    ) -> Result<Self, Float32PostProcessingBatchConfigError> {
        if maximum_records == 0 {
            return Err(Float32PostProcessingBatchConfigError::ZeroMaximumRecords);
        }
        let health_limit = u64::try_from(maximum_records).map_err(|_| {
            Float32PostProcessingBatchConfigError::MaximumRecordsUnrepresentable {
                requested: maximum_records,
            }
        })?;
        let processor = RequestedTimestampPostProcessor::new(request)
            .map_err(Float32PostProcessingBatchConfigError::PostProcessing)?;
        Ok(Self {
            maximum_records,
            processor,
            health: ExactSequenceLossHealth::new(health_limit),
        })
    }

    pub(crate) const fn maximum_records(&self) -> usize {
        self.maximum_records
    }
    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.health.snapshot()
    }

    /// Consumes the report allocation and commits owner state only on total success.
    pub(crate) fn process_report(
        &mut self,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    ) -> Result<Float32PostProcessingBatchOutcome, Float32PostProcessingBatchError> {
        self.process_records(sequences, report.into_records())
    }

    fn process_records(
        &mut self,
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
    ) -> Result<Float32PostProcessingBatchOutcome, Float32PostProcessingBatchError> {
        let actual = records.len();
        if actual == 0 {
            return Err(Float32PostProcessingBatchError::Empty { sequences, records });
        }
        if sequences.len() != actual {
            return Err(Float32PostProcessingBatchError::SequenceCount {
                sequence_count: sequences.len(),
                record_count: actual,
                sequences,
                records,
            });
        }
        if actual > self.maximum_records {
            return Err(Float32PostProcessingBatchError::RecordLimit {
                limit: self.maximum_records,
                actual,
                sequences,
                records,
            });
        }

        let mut completed = Vec::new();
        if completed.try_reserve_exact(actual).is_err() {
            return Err(Float32PostProcessingBatchError::Allocation {
                requested: actual,
                sequences,
                records,
            });
        }
        let mut next_processor = self.processor.clone();
        let mut next_health = self.health.clone();
        let mut sequence_iter = sequences.into_iter();
        let mut record_iter = records.into_iter();
        let mut completed_count = 0usize;

        while let (Some(sequence), Some(record)) = (sequence_iter.next(), record_iter.next()) {
            let index = completed_count;
            let observed = match process_requested_timestamp_and_observe_exact_health(
                &mut next_processor,
                &mut next_health,
                sequence,
                record,
            ) {
                Ok(observed) => observed,
                Err(error) => {
                    return Err(Float32PostProcessingBatchError::Record {
                        index,
                        sequence,
                        error,
                        completed,
                        remaining_sequences: sequence_iter.collect(),
                        remaining_records: record_iter.collect(),
                    });
                }
            };
            let classification = observed.classification();
            let health = observed.health();
            let processed = observed.into_processed();
            completed_count = match completed_count.checked_add(1) {
                Some(count) => count,
                None => {
                    return Err(Float32PostProcessingBatchError::CounterOverflow {
                        index,
                        sequence,
                        current: processed,
                        completed,
                        remaining_sequences: sequence_iter.collect(),
                        remaining_records: record_iter.collect(),
                    });
                }
            };
            completed.push(Float32PostProcessingBatchRecordOutcome {
                index,
                sequence,
                processed,
                classification,
                health,
            });
        }

        debug_assert_eq!(completed_count, actual);
        let health = next_health.snapshot();
        self.processor = next_processor;
        self.health = next_health;
        Ok(Float32PostProcessingBatchOutcome {
            records: completed,
            health,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exact_sequence_loss_health::ExactPostProcessingFact;
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessingConfig, RequestedTimestampPostProcessingDisposition,
    };
    use crate::{DerivedTimestamp, DerivedTimestampKind, RawSourceTimestamp, Sample, SampleLimits};

    fn sample(timestamp: f64, value: f32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(timestamp - 100.0).unwrap(),
            Some(DerivedTimestamp::new(DerivedTimestampKind::ClockCorrected, timestamp).unwrap()),
        )
    }

    fn monotonic() -> RequestedTimestampPostProcessing {
        RequestedTimestampPostProcessing::Monotonic(
            RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
        )
    }

    fn de_jitter() -> RequestedTimestampPostProcessing {
        RequestedTimestampPostProcessing::DeJitter(
            RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
        )
    }

    #[test]
    fn ordered_success_maps_only_unchanged_or_changed_and_commits_once() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(3, monotonic()).unwrap();
        let records = vec![sample(10.0, 1.0), sample(9.0, 2.0), sample(12.0, 3.0)];
        let pointers: Vec<_> = records
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let outcome = owner.process_records(vec![7, 9, 8], records).unwrap();
        assert_eq!(outcome.records().len(), 3);
        assert_eq!(
            outcome
                .records()
                .iter()
                .map(|record| record.index())
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        assert_eq!(
            outcome
                .records()
                .iter()
                .map(|record| record.sequence())
                .collect::<Vec<_>>(),
            vec![7, 9, 8]
        );
        assert_eq!(
            outcome
                .records()
                .iter()
                .map(|record| record.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>(),
            pointers
        );
        assert_eq!(
            outcome.records()[0].processed().facts().disposition(),
            RequestedTimestampPostProcessingDisposition::RetainedUnchanged
        );
        assert_eq!(
            outcome.records()[1].processed().facts().disposition(),
            RequestedTimestampPostProcessingDisposition::RetainedChanged
        );
        assert_eq!(
            outcome.records()[2].processed().facts().disposition(),
            RequestedTimestampPostProcessingDisposition::RetainedUnchanged
        );
        assert_eq!(
            outcome.records()[1].classification(),
            ExactSequenceClassification::Gap {
                missing_sequence_count: 1
            }
        );
        assert_eq!(
            outcome.records()[2].classification(),
            ExactSequenceClassification::OutOfOrder {
                behind_high_water_by: 1
            }
        );
        assert_eq!(owner.health(), outcome.health());
        assert_eq!(owner.health().observation_count(), 3);
    }

    #[test]
    fn zero_empty_mismatch_and_upper_bound_reject_before_owner_mutation() {
        assert_eq!(
            Float32SessionReportPostProcessingBatch::new(
                0,
                RequestedTimestampPostProcessing::PassThrough
            ),
            Err(Float32PostProcessingBatchConfigError::ZeroMaximumRecords)
        );
        let mut owner = Float32SessionReportPostProcessingBatch::new(
            1,
            RequestedTimestampPostProcessing::PassThrough,
        )
        .unwrap();
        let before = owner.clone();
        assert!(matches!(
            owner.process_records(vec![], vec![]),
            Err(Float32PostProcessingBatchError::Empty { .. })
        ));
        assert_eq!(owner, before);
        assert!(matches!(
            owner.process_records(vec![], vec![sample(1.0, 1.0)]),
            Err(Float32PostProcessingBatchError::SequenceCount {
                sequence_count: 0,
                record_count: 1,
                ..
            })
        ));
        assert_eq!(owner, before);
        assert!(matches!(
            owner.process_records(vec![1, 2], vec![sample(1.0, 1.0), sample(2.0, 2.0)]),
            Err(Float32PostProcessingBatchError::RecordLimit {
                limit: 1,
                actual: 2,
                ..
            })
        ));
        assert_eq!(owner, before);
    }

    #[test]
    fn middle_processing_failure_retains_exact_evidence_and_commits_nothing() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(3, de_jitter()).unwrap();
        let before = owner.clone();
        let records = vec![sample(10.0, 1.0), sample(10.0, 2.0), sample(12.0, 3.0)];
        let pointers: Vec<_> = records
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let error = owner
            .process_records(vec![20, 21, 22], records)
            .unwrap_err();
        match error {
            Float32PostProcessingBatchError::Record {
                index,
                sequence,
                error,
                completed,
                remaining_sequences,
                remaining_records,
            } => {
                assert_eq!((index, sequence), (1, 21));
                assert_eq!(completed.len(), 1);
                assert_eq!(
                    completed[0].processed().sample().sample().values().as_ptr(),
                    pointers[0]
                );
                let current = match error {
                    RequestedTimestampPostProcessingLossHealthError::PostProcessing(error) => {
                        error.into_sample()
                    }
                    other => panic!("unexpected error: {other:?}"),
                };
                assert_eq!(current.sample().values().as_ptr(), pointers[1]);
                assert_eq!(remaining_sequences, vec![22]);
                assert_eq!(remaining_records[0].sample().values().as_ptr(), pointers[2]);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(owner, before);
    }

    #[test]
    fn health_failure_observes_no_partial_batch_and_retains_current_processed_record() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(
            2,
            RequestedTimestampPostProcessing::PassThrough,
        )
        .unwrap();
        owner.health = ExactSequenceLossHealth::new(1);
        let before = owner.clone();
        let error = owner
            .process_records(vec![1, 2], vec![sample(1.0, 1.0), sample(2.0, 2.0)])
            .unwrap_err();
        assert!(matches!(error, Float32PostProcessingBatchError::Record {
            index: 1,
            error: RequestedTimestampPostProcessingLossHealthError::Health { .. },
            ref completed,
            ref remaining_sequences,
            ref remaining_records,
            ..
        } if completed.len() == 1 && remaining_sequences.is_empty() && remaining_records.is_empty()));
        assert_eq!(owner, before);
    }

    #[test]
    fn source_contains_no_discarded_mapping_or_equivalence_claim() {
        let source = include_str!("float32_session_report_post_processing_batch.rs");
        assert!(!source.contains(&["::", "Discarded"].concat()));
        assert!(source.contains("process_requested_timestamp_and_observe_exact_health"));
        assert_eq!(
            ExactPostProcessingFact::from_successful_timestamp_change(false),
            ExactPostProcessingFact::RetainedUnchanged
        );
    }
}
