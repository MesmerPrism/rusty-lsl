// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded local observation evidence projected from one successful Float32 batch.
//!
//! This module produces local advisory evidence only. It proves no behavioral,
//! numerical, wire, or other equivalence with liblsl and grants no downstream
//! admission, route, lease, revision, authorization, application, or audit authority.

use crate::exact_sequence_loss_health::{
    ExactSequenceClassification, ExactSequenceLossHealthSnapshot,
};
use crate::float32_session_report_post_processing_batch::Float32PostProcessingBatchOutcome;
use crate::requested_timestamp_post_processing::{
    RequestedEffectiveTimestamp, RequestedTimestampPostProcessed,
    RequestedTimestampPostProcessingDisposition,
};

/// One immutable, ordered observation retaining the original processed sample owner.
#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportRecordObservation {
    index: u64,
    sequence: u64,
    processed: RequestedTimestampPostProcessed<f32>,
    classification: ExactSequenceClassification,
}

impl MorphospaceFloat32ReportRecordObservation {
    pub(crate) const fn index(&self) -> u64 {
        self.index
    }

    pub(crate) const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) const fn processed(&self) -> &RequestedTimestampPostProcessed<f32> {
        &self.processed
    }

    pub(crate) const fn effective_timestamp(&self) -> RequestedEffectiveTimestamp {
        self.processed.facts().effective_timestamp()
    }

    pub(crate) const fn disposition(&self) -> RequestedTimestampPostProcessingDisposition {
        self.processed.facts().disposition()
    }

    pub(crate) const fn classification(&self) -> ExactSequenceClassification {
        self.classification
    }
}

/// Complete immutable Morphospace-facing evidence for one successful batch.
#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportObservation {
    records: Vec<MorphospaceFloat32ReportRecordObservation>,
    terminal_health: ExactSequenceLossHealthSnapshot,
}

impl MorphospaceFloat32ReportObservation {
    pub(crate) fn records(&self) -> &[MorphospaceFloat32ReportRecordObservation] {
        &self.records
    }

    pub(crate) const fn terminal_health(&self) -> ExactSequenceLossHealthSnapshot {
        self.terminal_health
    }
}

/// Construction refusal before an inert observation owner exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportObservationConfigError {
    ZeroMaximumRecords,
    MaximumRecordsUnrepresentable { requested: usize },
}

/// Projection refusal retaining the complete, unchanged successful batch outcome.
#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportObservationError {
    Empty {
        outcome: Float32PostProcessingBatchOutcome,
    },
    RecordLimit {
        limit: usize,
        actual: usize,
        outcome: Float32PostProcessingBatchOutcome,
    },
    RecordCountUnrepresentable {
        actual: usize,
        outcome: Float32PostProcessingBatchOutcome,
    },
    IndexUnrepresentable {
        index: usize,
        outcome: Float32PostProcessingBatchOutcome,
    },
    Allocation {
        requested: usize,
        outcome: Float32PostProcessingBatchOutcome,
    },
}

impl MorphospaceFloat32ReportObservationError {
    pub(crate) fn into_outcome(self) -> Float32PostProcessingBatchOutcome {
        match self {
            Self::Empty { outcome }
            | Self::RecordLimit { outcome, .. }
            | Self::RecordCountUnrepresentable { outcome, .. }
            | Self::IndexUnrepresentable { outcome, .. }
            | Self::Allocation { outcome, .. } => outcome,
        }
    }
}

/// Default-inert bounded owner of the explicit P36 projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportObservationOwner {
    maximum_records: usize,
}

impl MorphospaceFloat32ReportObservationOwner {
    pub(crate) fn new(
        maximum_records: usize,
    ) -> Result<Self, MorphospaceFloat32ReportObservationConfigError> {
        if maximum_records == 0 {
            return Err(MorphospaceFloat32ReportObservationConfigError::ZeroMaximumRecords);
        }
        u64::try_from(maximum_records).map_err(|_| {
            MorphospaceFloat32ReportObservationConfigError::MaximumRecordsUnrepresentable {
                requested: maximum_records,
            }
        })?;
        Ok(Self { maximum_records })
    }

    pub(crate) const fn maximum_records(&self) -> usize {
        self.maximum_records
    }

    /// Fallibly projects only after all bounds, conversions, and allocation succeed.
    pub(crate) fn observe(
        &self,
        outcome: Float32PostProcessingBatchOutcome,
    ) -> Result<MorphospaceFloat32ReportObservation, MorphospaceFloat32ReportObservationError> {
        self.observe_with(
            outcome,
            |value| u64::try_from(value).map_err(|_| ()),
            |records, requested| records.try_reserve_exact(requested).map_err(|_| ()),
        )
    }

    fn observe_with<C, R>(
        &self,
        outcome: Float32PostProcessingBatchOutcome,
        mut convert: C,
        reserve: R,
    ) -> Result<MorphospaceFloat32ReportObservation, MorphospaceFloat32ReportObservationError>
    where
        C: FnMut(usize) -> Result<u64, ()>,
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportRecordObservation>, usize) -> Result<(), ()>,
    {
        let actual = outcome.records().len();
        if actual == 0 {
            return Err(MorphospaceFloat32ReportObservationError::Empty { outcome });
        }
        if actual > self.maximum_records {
            return Err(MorphospaceFloat32ReportObservationError::RecordLimit {
                limit: self.maximum_records,
                actual,
                outcome,
            });
        }
        if convert(actual).is_err() {
            return Err(
                MorphospaceFloat32ReportObservationError::RecordCountUnrepresentable {
                    actual,
                    outcome,
                },
            );
        }
        for record in outcome.records() {
            if convert(record.index()).is_err() {
                return Err(
                    MorphospaceFloat32ReportObservationError::IndexUnrepresentable {
                        index: record.index(),
                        outcome,
                    },
                );
            }
        }

        let terminal_health = outcome.health();
        let mut records = Vec::new();
        if reserve(&mut records, actual).is_err() {
            return Err(MorphospaceFloat32ReportObservationError::Allocation {
                requested: actual,
                outcome,
            });
        }
        for record in outcome.into_records() {
            let index = u64::try_from(record.index())
                .expect("record index was checked before outcome consumption");
            let sequence = record.sequence();
            let classification = record.classification();
            let processed = record.into_processed();
            records.push(MorphospaceFloat32ReportRecordObservation {
                index,
                sequence,
                processed,
                classification,
            });
        }
        Ok(MorphospaceFloat32ReportObservation {
            records,
            terminal_health,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requested_timestamp_post_processing::{
        RequestedEffectiveTimestampSource, RequestedTimestampPostProcessing,
        RequestedTimestampPostProcessingConfig,
    };
    use crate::{
        DerivedTimestamp, DerivedTimestampKind, RawSourceTimestamp, Sample, SampleLimits,
        TimestampedSample,
    };

    fn sample(timestamp: f64, value: f32, derived: bool) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            derived.then(|| {
                DerivedTimestamp::new(DerivedTimestampKind::ClockCorrected, timestamp + 100.0)
                    .unwrap()
            }),
        )
    }

    fn outcome(
        sequences: Vec<u64>,
        records: Vec<TimestampedSample<f32>>,
    ) -> Float32PostProcessingBatchOutcome {
        let maximum = records.len().max(1);
        let mut owner = super::super::Float32SessionReportPostProcessingBatch::new(
            maximum,
            RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 200.0).unwrap(),
            ),
        )
        .unwrap();
        owner.process_records(sequences, records).unwrap()
    }

    #[test]
    fn zero_is_rejected_and_the_upper_representable_bound_is_inert() {
        assert_eq!(
            MorphospaceFloat32ReportObservationOwner::new(0),
            Err(MorphospaceFloat32ReportObservationConfigError::ZeroMaximumRecords)
        );
        let owner = MorphospaceFloat32ReportObservationOwner::new(usize::MAX).unwrap();
        assert_eq!(owner.maximum_records(), usize::MAX);
    }

    #[test]
    fn ordered_projection_preserves_extreme_sequences_samples_clock_facts_and_health() {
        let records = vec![
            sample(10.0, f32::from_bits(0x3f80_0001), true),
            sample(9.0, f32::from_bits(0xc020_0001), false),
            sample(12.0, f32::from_bits(0x4040_0001), false),
            sample(13.0, f32::from_bits(0x4080_0001), false),
        ];
        let pointers: Vec<_> = records
            .iter()
            .map(|record| record.sample().values().as_ptr())
            .collect();
        let batch = outcome(vec![0, u64::MAX, u64::MAX, 0], records);
        let expected_health = batch.health();
        let observation = MorphospaceFloat32ReportObservationOwner::new(4)
            .unwrap()
            .observe(batch)
            .unwrap();

        assert_eq!(
            observation
                .records()
                .iter()
                .map(|record| (record.index(), record.sequence()))
                .collect::<Vec<_>>(),
            vec![(0, 0), (1, u64::MAX), (2, u64::MAX), (3, 0)]
        );
        assert_eq!(
            observation
                .records()
                .iter()
                .map(|record| record.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>(),
            pointers
        );
        assert_eq!(
            observation.records()[0].effective_timestamp().source(),
            RequestedEffectiveTimestampSource::ProjectPostProcessed
        );
        assert_eq!(
            observation.records()[0].effective_timestamp().value(),
            110.0
        );
        assert_eq!(
            observation.records()[0]
                .processed()
                .sample()
                .derived_timestamp()
                .unwrap()
                .kind(),
            DerivedTimestampKind::ClockCorrected
        );
        assert_eq!(
            observation.records()[0].disposition(),
            RequestedTimestampPostProcessingDisposition::RetainedUnchanged
        );
        assert_eq!(
            observation.records()[1].disposition(),
            RequestedTimestampPostProcessingDisposition::RetainedChanged
        );
        assert_eq!(
            observation.records()[0].classification(),
            ExactSequenceClassification::First
        );
        assert_eq!(
            observation.records()[1].classification(),
            ExactSequenceClassification::Gap {
                missing_sequence_count: u64::MAX - 1
            }
        );
        assert_eq!(
            observation.records()[2].classification(),
            ExactSequenceClassification::Duplicate
        );
        assert_eq!(
            observation.records()[3].classification(),
            ExactSequenceClassification::OutOfOrder {
                behind_high_water_by: u64::MAX
            }
        );
        assert_eq!(observation.terminal_health(), expected_health);
    }

    #[test]
    fn exact_upper_bound_succeeds_and_limit_refusal_returns_the_unchanged_owner() {
        let batch = outcome(
            vec![4, 5],
            vec![sample(1.0, 1.0, false), sample(2.0, 2.0, false)],
        );
        assert_eq!(
            MorphospaceFloat32ReportObservationOwner::new(2)
                .unwrap()
                .observe(batch)
                .unwrap()
                .records()
                .len(),
            2
        );

        let batch = outcome(
            vec![4, 5],
            vec![sample(1.0, 1.0, false), sample(2.0, 2.0, false)],
        );
        let pointers: Vec<_> = batch
            .records()
            .iter()
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect();
        let error = MorphospaceFloat32ReportObservationOwner::new(1)
            .unwrap()
            .observe(batch)
            .unwrap_err();
        let returned = error.into_outcome();
        assert_eq!(
            returned
                .records()
                .iter()
                .map(|record| record.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>(),
            pointers
        );
    }

    #[test]
    fn conversion_and_allocation_refusals_create_no_partial_observation() {
        let owner = MorphospaceFloat32ReportObservationOwner::new(2).unwrap();
        let batch = outcome(
            vec![8, 9],
            vec![sample(1.0, 1.0, false), sample(2.0, 2.0, false)],
        );
        let health = batch.health();
        let returned = owner
            .observe_with(batch, |_| Err(()), |_, _| Ok(()))
            .unwrap_err()
            .into_outcome();
        assert_eq!(returned.records().len(), 2);
        assert_eq!(returned.health(), health);

        let batch = outcome(
            vec![8, 9],
            vec![sample(1.0, 1.0, false), sample(2.0, 2.0, false)],
        );
        let pointers: Vec<_> = batch
            .records()
            .iter()
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect();
        let returned = owner
            .observe_with(batch, |value| Ok(value as u64), |_, _| Err(()))
            .unwrap_err()
            .into_outcome();
        assert_eq!(
            returned
                .records()
                .iter()
                .map(|record| record.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>(),
            pointers
        );
    }
}
