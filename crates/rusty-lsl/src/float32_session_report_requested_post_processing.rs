// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional requested timestamp processing for one retained Float32 report record.
//!
//! Sequence numbers are explicit caller evidence. The owner records only their exact
//! relationships and makes no estimate or inference about packet loss. Requested
//! timestamp algorithms remain project-owned candidates, not evidence of equivalence
//! with liblsl.

use crate::exact_sequence_loss_health::{
    ExactPostProcessingFact, ExactSequenceClassification, ExactSequenceLossHealth,
    ExactSequenceLossHealthError, ExactSequenceLossHealthSnapshot,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessingDisposition;
use crate::requested_timestamp_post_processing::{
    RequestedTimestampPostProcessingError, RequestedTimestampPostProcessingFacts,
    RequestedTimestampPostProcessor, RequestedTimestampPostProcessorCopyError,
};
use crate::TimestampedSample;

/// Successful ownership of one retained report record and all exact derived facts.
#[derive(Debug, PartialEq)]
pub(crate) struct Float32SessionReportRequestedPostProcessingOutcome {
    sequence: u64,
    sample: TimestampedSample<f32>,
    facts: RequestedTimestampPostProcessingFacts,
    classification: ExactSequenceClassification,
    health: ExactSequenceLossHealthSnapshot,
}

impl Float32SessionReportRequestedPostProcessingOutcome {
    pub(crate) const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) const fn sample(&self) -> &TimestampedSample<f32> {
        &self.sample
    }

    pub(crate) const fn facts(&self) -> &RequestedTimestampPostProcessingFacts {
        &self.facts
    }

    pub(crate) const fn classification(&self) -> ExactSequenceClassification {
        self.classification
    }

    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.health
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        u64,
        TimestampedSample<f32>,
        RequestedTimestampPostProcessingFacts,
        ExactSequenceClassification,
        ExactSequenceLossHealthSnapshot,
    ) {
        (
            self.sequence,
            self.sample,
            self.facts,
            self.classification,
            self.health,
        )
    }
}

/// Typed refusal retaining the exact Float32 report record.
#[derive(Debug, PartialEq)]
pub(crate) enum Float32SessionReportRequestedPostProcessingError {
    Allocation {
        sample: TimestampedSample<f32>,
        error: RequestedTimestampPostProcessorCopyError,
    },
    PostProcessing(RequestedTimestampPostProcessingError<f32>),
    Health {
        sample: TimestampedSample<f32>,
        facts: RequestedTimestampPostProcessingFacts,
        error: ExactSequenceLossHealthError,
    },
}

impl Float32SessionReportRequestedPostProcessingError {
    pub(crate) fn into_sample(self) -> TimestampedSample<f32> {
        match self {
            Self::Allocation { sample, .. } => sample,
            Self::PostProcessing(error) => error.into_sample(),
            Self::Health { sample, .. } => sample,
        }
    }
}

/// Sole per-record owner of requested processing state and exact sequence health.
#[derive(Debug, PartialEq)]
pub(crate) struct Float32SessionReportRequestedPostProcessing {
    processor: RequestedTimestampPostProcessor,
    health: ExactSequenceLossHealth,
}

impl Float32SessionReportRequestedPostProcessing {
    pub(crate) fn try_candidate_copy(
        &self,
    ) -> Result<Self, RequestedTimestampPostProcessorCopyError> {
        Ok(Self {
            processor: self.processor.try_candidate_copy()?,
            health: self.health.clone(),
        })
    }

    pub(crate) fn try_candidate_copy_with<F>(
        &self,
        reserve: F,
    ) -> Result<Self, RequestedTimestampPostProcessorCopyError>
    where
        F: FnOnce(&mut Vec<f64>, usize) -> Result<(), ()>,
    {
        Ok(Self {
            processor: self.processor.try_candidate_copy_with(reserve)?,
            health: self.health.clone(),
        })
    }

    pub(crate) const fn new(
        processor: RequestedTimestampPostProcessor,
        health: ExactSequenceLossHealth,
    ) -> Self {
        Self { processor, health }
    }

    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.health.snapshot()
    }

    /// Processes one report-owned record and commits both subordinate owners atomically.
    pub(crate) fn process_record(
        &mut self,
        sequence: u64,
        sample: TimestampedSample<f32>,
    ) -> Result<
        Float32SessionReportRequestedPostProcessingOutcome,
        Float32SessionReportRequestedPostProcessingError,
    > {
        let mut candidate = match self.try_candidate_copy() {
            Ok(candidate) => candidate,
            Err(error) => {
                return Err(
                    Float32SessionReportRequestedPostProcessingError::Allocation { sample, error },
                );
            }
        };
        let outcome = candidate.process_record_in_candidate(sequence, sample)?;
        *self = candidate;
        Ok(outcome)
    }

    pub(crate) fn process_record_in_candidate(
        &mut self,
        sequence: u64,
        sample: TimestampedSample<f32>,
    ) -> Result<
        Float32SessionReportRequestedPostProcessingOutcome,
        Float32SessionReportRequestedPostProcessingError,
    > {
        let processed = self
            .processor
            .process(sample)
            .map_err(Float32SessionReportRequestedPostProcessingError::PostProcessing)?;
        let fact = match processed.facts().disposition() {
            RequestedTimestampPostProcessingDisposition::RetainedUnchanged => {
                ExactPostProcessingFact::RetainedUnchanged
            }
            RequestedTimestampPostProcessingDisposition::RetainedChanged => {
                ExactPostProcessingFact::RetainedChanged
            }
        };
        let classification = match self.health.observe(sequence, fact) {
            Ok(classification) => classification,
            Err(error) => {
                let (sample, facts) = processed.into_parts();
                return Err(Float32SessionReportRequestedPostProcessingError::Health {
                    sample,
                    facts,
                    error,
                });
            }
        };
        let health = self.health.snapshot();
        let (sample, facts) = processed.into_parts();
        Ok(Float32SessionReportRequestedPostProcessingOutcome {
            sequence,
            sample,
            facts,
            classification,
            health,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessingDisposition,
    };
    use crate::{DerivedTimestamp, DerivedTimestampKind, RawSourceTimestamp, Sample, SampleLimits};

    fn record(raw: f64, effective: f64, value: f32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(raw).unwrap(),
            Some(DerivedTimestamp::new(DerivedTimestampKind::ClockCorrected, effective).unwrap()),
        )
    }

    fn monotonic_owner(limit: u64) -> Float32SessionReportRequestedPostProcessing {
        let config = RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap();
        Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                config,
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(limit),
        )
    }

    #[test]
    fn report_record_allocation_and_timestamp_evidence_are_retained() {
        let mut owner = monotonic_owner(2);
        owner.process_record(40, record(1.0, 10.0, 1.0)).unwrap();
        let input = record(2.0, 9.0, 2.0);
        let allocation = input.sample().values().as_ptr();
        let outcome = owner.process_record(43, input).unwrap();

        assert_eq!(outcome.sequence(), 43);
        assert_eq!(outcome.sample().sample().values().as_ptr(), allocation);
        assert_eq!(
            outcome.sample().raw_source_timestamp().value().to_bits(),
            2.0f64.to_bits()
        );
        let derived = outcome.sample().derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value().to_bits(), 9.0f64.to_bits());
        assert_eq!(
            outcome.facts().effective_timestamp().value().to_bits(),
            11.0f64.to_bits()
        );
        assert_eq!(
            outcome.facts().disposition(),
            RequestedTimestampPostProcessingDisposition::RetainedChanged
        );
        assert_eq!(
            outcome.classification(),
            ExactSequenceClassification::Gap {
                missing_sequence_count: 2
            }
        );
        assert_eq!(outcome.health().explicit_missing_sequence_count(), 2);
    }

    #[test]
    fn processing_error_returns_record_and_changes_neither_owner() {
        let config = RequestedTimestampPostProcessingConfig::new(2, 0.5, 5.0).unwrap();
        let mut owner = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::DeJitter(
                config,
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(2),
        );
        owner.process_record(7, record(1.0, 10.0, 1.0)).unwrap();
        let before = owner.try_candidate_copy().unwrap();
        let input = record(2.0, 10.0, 2.0);
        let allocation = input.sample().values().as_ptr();
        let returned = owner.process_record(9, input).unwrap_err().into_sample();
        assert_eq!(returned.sample().values().as_ptr(), allocation);
        assert_eq!(owner, before);
    }

    #[test]
    fn health_error_returns_processed_record_and_changes_neither_owner() {
        let mut owner = monotonic_owner(0);
        let before = owner.try_candidate_copy().unwrap();
        let input = record(3.0, 12.0, 3.0);
        let allocation = input.sample().values().as_ptr();
        let error = owner.process_record(u64::MAX, input).unwrap_err();
        match &error {
            Float32SessionReportRequestedPostProcessingError::Health { facts, error, .. } => {
                assert_eq!(
                    facts.effective_timestamp().value().to_bits(),
                    12.0f64.to_bits()
                );
                assert_eq!(
                    *error,
                    ExactSequenceLossHealthError::ObservationLimitReached { limit: 0 }
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(error.into_sample().sample().values().as_ptr(), allocation);
        assert_eq!(owner, before);
    }
}
