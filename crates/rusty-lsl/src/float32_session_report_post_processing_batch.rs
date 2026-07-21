// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded all-or-nothing post-processing and exact-health owner for Float32 reports.

use crate::exact_sequence_loss_health::{
    ExactSequenceClassification, ExactSequenceLossHealth, ExactSequenceLossHealthSnapshot,
};
use crate::float32_session_report_requested_post_processing::{
    Float32SessionReportRequestedPostProcessing, Float32SessionReportRequestedPostProcessingError,
};
use crate::requested_timestamp_post_processing::{
    RequestedTimestampPostProcessed, RequestedTimestampPostProcessing,
    RequestedTimestampPostProcessingConfigError, RequestedTimestampPostProcessor,
    RequestedTimestampPostProcessorCopyError,
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
        error: Float32SessionReportRequestedPostProcessingError,
        completed: Vec<Float32PostProcessingBatchRecordOutcome>,
        remaining_sequences: Float32PostProcessingBatchRemainder<u64>,
        remaining_records: Float32PostProcessingBatchRemainder<TimestampedSample<f32>>,
    },
}

/// Allocation-preserving unprocessed suffix of one caller-owned vector.
#[derive(Debug)]
pub(crate) struct Float32PostProcessingBatchRemainder<T> {
    values: std::vec::IntoIter<T>,
}

impl<T> Float32PostProcessingBatchRemainder<T> {
    fn new(values: std::vec::IntoIter<T>) -> Self {
        Self { values }
    }

    pub(crate) fn as_slice(&self) -> &[T] {
        self.values.as_slice()
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.len() == 0
    }
}

impl<T: PartialEq> PartialEq for Float32PostProcessingBatchRemainder<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

/// Sole bounded lifecycle owner for P33 post-processing plus exact health.
#[derive(Debug, PartialEq)]
pub(crate) struct Float32SessionReportPostProcessingBatch {
    maximum_records: usize,
    record_owner: Float32SessionReportRequestedPostProcessing,
}

impl Float32SessionReportPostProcessingBatch {
    fn try_candidate_copy(&self) -> Result<Self, RequestedTimestampPostProcessorCopyError> {
        let record_owner = self.record_owner.try_candidate_copy()?;
        Ok(Self {
            maximum_records: self.maximum_records,
            record_owner,
        })
    }
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
            record_owner: Float32SessionReportRequestedPostProcessing::new(
                processor,
                ExactSequenceLossHealth::new(health_limit),
            ),
        })
    }

    pub(crate) const fn maximum_records(&self) -> usize {
        self.maximum_records
    }
    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.record_owner.health()
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
        self.process_records_with_reservations(
            sequences,
            records,
            |completed, requested| completed.try_reserve_exact(requested).map_err(|_| ()),
            |history, requested| history.try_reserve_exact(requested).map_err(|_| ()),
        )
    }

    fn process_records_with_candidate_copy<F>(
        &mut self,
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
        reserve: F,
    ) -> Result<Float32PostProcessingBatchOutcome, Float32PostProcessingBatchError>
    where
        F: FnOnce(&mut Vec<f64>, usize) -> Result<(), ()>,
    {
        self.process_records_with_reservations(
            sequences,
            records,
            |completed, requested| completed.try_reserve_exact(requested).map_err(|_| ()),
            reserve,
        )
    }

    fn process_records_with_reservations<F, G>(
        &mut self,
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
        reserve_completed: F,
        reserve_candidate: G,
    ) -> Result<Float32PostProcessingBatchOutcome, Float32PostProcessingBatchError>
    where
        F: FnOnce(&mut Vec<Float32PostProcessingBatchRecordOutcome>, usize) -> Result<(), ()>,
        G: FnOnce(&mut Vec<f64>, usize) -> Result<(), ()>,
    {
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
        if reserve_completed(&mut completed, actual).is_err() {
            return Err(Float32PostProcessingBatchError::Allocation {
                requested: actual,
                sequences,
                records,
            });
        }
        let mut next_record_owner =
            match self.record_owner.try_candidate_copy_with(reserve_candidate) {
                Ok(candidate) => candidate,
                Err(_) => {
                    return Err(Float32PostProcessingBatchError::Allocation {
                        requested: actual,
                        sequences,
                        records,
                    });
                }
            };
        let mut sequence_iter = sequences.into_iter();
        let mut record_iter = records.into_iter();

        while let (Some(sequence), Some(record)) = (sequence_iter.next(), record_iter.next()) {
            let index = completed.len();
            let observed = match next_record_owner.process_record_in_candidate(sequence, record) {
                Ok(observed) => observed,
                Err(error) => {
                    return Err(Float32PostProcessingBatchError::Record {
                        index,
                        sequence,
                        error,
                        completed,
                        remaining_sequences: Float32PostProcessingBatchRemainder::new(
                            sequence_iter,
                        ),
                        remaining_records: Float32PostProcessingBatchRemainder::new(record_iter),
                    });
                }
            };
            let classification = observed.classification();
            let health = observed.health();
            let (_, sample, facts, _, _) = observed.into_parts();
            let processed = RequestedTimestampPostProcessed::from_parts(sample, facts);
            completed.push(Float32PostProcessingBatchRecordOutcome {
                index,
                sequence,
                processed,
                classification,
                health,
            });
        }

        let health = next_record_owner.health();
        self.record_owner = next_record_owner;
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
    fn candidate_copy_refusal_preserves_every_input_and_live_state() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(2, monotonic()).unwrap();
        let before = owner.try_candidate_copy().unwrap();
        let records = vec![sample(10.0, 1.0), sample(11.0, 2.0)];
        let pointers: Vec<_> = records
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let error = owner
            .process_records_with_candidate_copy(vec![4, 5], records, |_, _| Err(()))
            .unwrap_err();
        match error {
            Float32PostProcessingBatchError::Allocation {
                sequences, records, ..
            } => {
                assert_eq!(sequences, vec![4, 5]);
                assert_eq!(records[0].sample().values().as_ptr(), pointers[0]);
                assert_eq!(records[1].sample().values().as_ptr(), pointers[1]);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(owner, before);
    }

    #[test]
    fn outcome_reserve_refusal_preserves_exact_input_allocations_and_live_state() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(2, monotonic()).unwrap();
        let before = owner.try_candidate_copy().unwrap();
        let sequences = vec![u64::MIN, u64::MAX];
        let sequence_pointer = sequences.as_ptr();
        let records = vec![sample(10.0, 1.0), sample(11.0, 2.0)];
        let record_pointers: Vec<_> = records
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let error = owner
            .process_records_with_reservations(
                sequences,
                records,
                |_, requested| {
                    assert_eq!(requested, 2);
                    Err(())
                },
                |_, _| panic!("candidate copy must follow outcome storage"),
            )
            .unwrap_err();
        match error {
            Float32PostProcessingBatchError::Allocation {
                requested,
                sequences,
                records,
            } => {
                assert_eq!(requested, 2);
                assert_eq!(sequences, [u64::MIN, u64::MAX]);
                assert_eq!(sequences.as_ptr(), sequence_pointer);
                assert_eq!(records[0].sample().values().as_ptr(), record_pointers[0]);
                assert_eq!(records[1].sample().values().as_ptr(), record_pointers[1]);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(owner, before);
    }

    #[test]
    fn maximum_gap_duplicate_out_of_order_and_u64_max_are_exact() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(
            4,
            RequestedTimestampPostProcessing::PassThrough,
        )
        .unwrap();
        let outcome = owner
            .process_records(
                vec![0, u64::MAX, u64::MAX, 0],
                vec![
                    sample(1.0, 1.0),
                    sample(2.0, 2.0),
                    sample(3.0, 3.0),
                    sample(4.0, 4.0),
                ],
            )
            .unwrap();
        assert_eq!(
            outcome.records()[1].classification(),
            ExactSequenceClassification::Gap {
                missing_sequence_count: u64::MAX - 1
            }
        );
        assert_eq!(
            outcome.records()[2].classification(),
            ExactSequenceClassification::Duplicate
        );
        assert_eq!(
            outcome.records()[3].classification(),
            ExactSequenceClassification::OutOfOrder {
                behind_high_water_by: u64::MAX
            }
        );
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
        let before = owner.try_candidate_copy().unwrap();
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
        let before = owner.try_candidate_copy().unwrap();
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
                    Float32SessionReportRequestedPostProcessingError::PostProcessing(error) => {
                        error.into_sample()
                    }
                    other => panic!("unexpected error: {other:?}"),
                };
                assert_eq!(current.sample().values().as_ptr(), pointers[1]);
                assert_eq!(remaining_sequences.as_slice(), [22]);
                assert_eq!(
                    remaining_records.as_slice()[0].sample().values().as_ptr(),
                    pointers[2]
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(owner, before);
    }

    #[test]
    fn first_middle_and_final_processing_failures_partition_exact_evidence() {
        let config = RequestedTimestampPostProcessingConfig::new(4, 1.0, 10.0).unwrap();
        for failing in 0..3 {
            let processor = RequestedTimestampPostProcessor::from_retained_state(
                RequestedTimestampPostProcessing::DeJitter(config),
                vec![10.0],
                Some(10.0),
            )
            .unwrap();
            let mut owner = Float32SessionReportPostProcessingBatch {
                maximum_records: 3,
                record_owner: Float32SessionReportRequestedPostProcessing::new(
                    processor,
                    ExactSequenceLossHealth::new(3),
                ),
            };
            let before = owner.try_candidate_copy().unwrap();
            let timestamps = match failing {
                0 => [10.0, 12.0, 13.0],
                1 => [11.0, 11.0, 13.0],
                _ => [11.0, 12.0, 12.0],
            };
            let records: Vec<_> = timestamps
                .into_iter()
                .enumerate()
                .map(|(index, timestamp)| sample(timestamp, index as f32))
                .collect();
            let pointers: Vec<_> = records
                .iter()
                .map(|record| record.sample().values().as_ptr())
                .collect();
            let sequences = vec![20, 21, 22];
            let sequence_pointer = sequences.as_ptr();
            match owner.process_records(sequences, records).unwrap_err() {
                Float32PostProcessingBatchError::Record {
                    index,
                    sequence,
                    error,
                    completed,
                    remaining_sequences,
                    remaining_records,
                } => {
                    assert_eq!((index, sequence), (failing, 20 + failing as u64));
                    assert_eq!(completed.len(), failing);
                    for (index, outcome) in completed.iter().enumerate() {
                        assert_eq!(
                            outcome.processed().sample().sample().values().as_ptr(),
                            pointers[index]
                        );
                    }
                    assert_eq!(
                        error.into_sample().sample().values().as_ptr(),
                        pointers[failing]
                    );
                    assert_eq!(
                        remaining_sequences.as_slice(),
                        &vec![20, 21, 22][failing + 1..]
                    );
                    if !remaining_sequences.is_empty() {
                        assert_eq!(
                            remaining_sequences.as_slice().as_ptr(),
                            sequence_pointer.wrapping_add(failing + 1)
                        );
                    }
                    for (offset, record) in remaining_records.as_slice().iter().enumerate() {
                        assert_eq!(
                            record.sample().values().as_ptr(),
                            pointers[failing + 1 + offset]
                        );
                    }
                }
                other => panic!("unexpected error: {other:?}"),
            }
            assert_eq!(owner, before);
        }
    }

    #[test]
    fn first_middle_and_final_health_failures_are_single_transaction_refusals() {
        for failing in 0..3 {
            let mut owner = Float32SessionReportPostProcessingBatch {
                maximum_records: 3,
                record_owner: Float32SessionReportRequestedPostProcessing::new(
                    RequestedTimestampPostProcessor::new(
                        RequestedTimestampPostProcessing::PassThrough,
                    )
                    .unwrap(),
                    ExactSequenceLossHealth::new(failing as u64),
                ),
            };
            let before = owner.try_candidate_copy().unwrap();
            let records = vec![sample(1.0, 1.0), sample(2.0, 2.0), sample(3.0, 3.0)];
            let pointers: Vec<_> = records
                .iter()
                .map(|record| record.sample().values().as_ptr())
                .collect();
            match owner.process_records(vec![7, 8, 9], records).unwrap_err() {
                Float32PostProcessingBatchError::Record {
                    index,
                    error,
                    completed,
                    remaining_sequences,
                    remaining_records,
                    ..
                } => {
                    assert_eq!(index, failing);
                    assert_eq!(completed.len(), failing);
                    match error {
                        Float32SessionReportRequestedPostProcessingError::Health {
                            sample,
                            facts,
                            ..
                        } => {
                            assert_eq!(sample.sample().values().as_ptr(), pointers[failing]);
                            assert_eq!(
                                facts.disposition(),
                                RequestedTimestampPostProcessingDisposition::RetainedUnchanged
                            );
                        }
                        other => panic!("unexpected error: {other:?}"),
                    }
                    assert_eq!(remaining_sequences.len(), 2 - failing);
                    assert_eq!(remaining_records.len(), 2 - failing);
                }
                other => panic!("unexpected error: {other:?}"),
            }
            assert_eq!(owner, before);
        }
    }

    #[test]
    fn health_failure_observes_no_partial_batch_and_retains_current_processed_record() {
        let mut owner = Float32SessionReportPostProcessingBatch::new(
            2,
            RequestedTimestampPostProcessing::PassThrough,
        )
        .unwrap();
        owner
            .process_records(vec![0], vec![sample(0.0, 0.0)])
            .unwrap();
        let before = owner.try_candidate_copy().unwrap();
        let error = owner
            .process_records(vec![1, 2], vec![sample(1.0, 1.0), sample(2.0, 2.0)])
            .unwrap_err();
        assert!(matches!(error, Float32PostProcessingBatchError::Record {
            index: 1,
            error: Float32SessionReportRequestedPostProcessingError::Health { .. },
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
        assert!(!source.contains(&["sequence_iter", ".", "collect()"].concat()));
        assert!(!source.contains(&["record_iter", ".", "collect()"].concat()));
        assert!(source.contains("process_record_in_candidate"));
        assert_eq!(
            ExactPostProcessingFact::from_successful_timestamp_change(false),
            ExactPostProcessingFact::RetainedUnchanged
        );
    }
}
