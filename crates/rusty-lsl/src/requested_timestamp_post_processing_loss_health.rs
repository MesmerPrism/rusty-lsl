// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private transactional composition of requested timestamp processing and exact health.

use crate::exact_sequence_loss_health::{
    ExactPostProcessingFact, ExactSequenceClassification, ExactSequenceLossHealth,
    ExactSequenceLossHealthError, ExactSequenceLossHealthSnapshot,
};
use crate::requested_timestamp_post_processing::{
    RequestedTimestampPostProcessed, RequestedTimestampPostProcessingDisposition,
    RequestedTimestampPostProcessingError, RequestedTimestampPostProcessor,
};
use crate::TimestampedSample;

/// Successful processing plus the exact observation it committed.
#[derive(Debug, PartialEq)]
pub(crate) struct RequestedTimestampPostProcessingLossHealthOutcome<T> {
    processed: RequestedTimestampPostProcessed<T>,
    classification: ExactSequenceClassification,
    health: ExactSequenceLossHealthSnapshot,
}

impl<T> RequestedTimestampPostProcessingLossHealthOutcome<T> {
    pub(crate) const fn processed(&self) -> &RequestedTimestampPostProcessed<T> {
        &self.processed
    }

    pub(crate) const fn classification(&self) -> ExactSequenceClassification {
        self.classification
    }

    pub(crate) const fn health(&self) -> ExactSequenceLossHealthSnapshot {
        self.health
    }

    pub(crate) fn into_processed(self) -> RequestedTimestampPostProcessed<T> {
        self.processed
    }
}

/// Typed transactional refusal. Neither owner commits on either variant.
#[derive(Debug, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingLossHealthError<T> {
    PostProcessing(RequestedTimestampPostProcessingError<T>),
    Health {
        processed: RequestedTimestampPostProcessed<T>,
        error: ExactSequenceLossHealthError,
    },
}

/// Processes one sample and observes only its exact successful disposition.
///
/// Both private owners are cloned as bounded candidate state. They replace the
/// caller's owners only after processing and health observation both succeed.
/// A processing error therefore creates no health fact or mutation; a health
/// refusal returns the processed sample while leaving both caller owners intact.
pub(crate) fn process_requested_timestamp_and_observe_exact_health<T>(
    processor: &mut RequestedTimestampPostProcessor,
    health: &mut ExactSequenceLossHealth,
    sequence: u64,
    sample: TimestampedSample<T>,
) -> Result<
    RequestedTimestampPostProcessingLossHealthOutcome<T>,
    RequestedTimestampPostProcessingLossHealthError<T>,
> {
    let mut next_processor = processor.clone();
    let mut next_health = health.clone();
    let processed = next_processor
        .process(sample)
        .map_err(RequestedTimestampPostProcessingLossHealthError::PostProcessing)?;
    let fact = match processed.facts().disposition() {
        RequestedTimestampPostProcessingDisposition::RetainedUnchanged => {
            ExactPostProcessingFact::RetainedUnchanged
        }
        RequestedTimestampPostProcessingDisposition::RetainedChanged => {
            ExactPostProcessingFact::RetainedChanged
        }
    };
    let classification = match next_health.observe(sequence, fact) {
        Ok(classification) => classification,
        Err(error) => {
            return Err(RequestedTimestampPostProcessingLossHealthError::Health {
                processed,
                error,
            });
        }
    };
    let snapshot = next_health.snapshot();
    *processor = next_processor;
    *health = next_health;
    Ok(RequestedTimestampPostProcessingLossHealthOutcome {
        processed,
        classification,
        health: snapshot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessingError,
    };
    use crate::{DerivedTimestamp, DerivedTimestampKind, RawSourceTimestamp, Sample, SampleLimits};

    fn sample(timestamp: f64, value: &str) -> TimestampedSample<String> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value.to_owned()]).unwrap(),
            RawSourceTimestamp::new(timestamp - 100.0).unwrap(),
            Some(DerivedTimestamp::new(DerivedTimestampKind::ClockCorrected, timestamp).unwrap()),
        )
    }

    fn config() -> RequestedTimestampPostProcessingConfig {
        RequestedTimestampPostProcessingConfig::new(2, 1.0, 5.0).unwrap()
    }

    #[test]
    fn unchanged_success_maps_once_and_preserves_clock_evidence_and_allocation() {
        let mut processor =
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::PassThrough)
                .unwrap();
        let mut health = ExactSequenceLossHealth::new(1);
        let input = sample(10.0, "unchanged");
        let allocation = input.sample().values()[0].as_ptr();
        let outcome = process_requested_timestamp_and_observe_exact_health(
            &mut processor,
            &mut health,
            7,
            input,
        )
        .unwrap();
        assert_eq!(outcome.classification(), ExactSequenceClassification::First);
        assert_eq!(outcome.health().retained_unchanged_count(), 1);
        assert_eq!(outcome.health().retained_changed_count(), 0);
        assert_eq!(
            outcome.processed().sample().sample().values()[0].as_ptr(),
            allocation
        );
        let derived = outcome.processed().sample().derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value().to_bits(), 10.0f64.to_bits());
    }

    #[test]
    fn changed_success_maps_once_without_replacing_clock_evidence() {
        let mut processor = RequestedTimestampPostProcessor::new(
            RequestedTimestampPostProcessing::Monotonic(config()),
        )
        .unwrap();
        let mut health = ExactSequenceLossHealth::new(2);
        process_requested_timestamp_and_observe_exact_health(
            &mut processor,
            &mut health,
            0,
            sample(10.0, "first"),
        )
        .unwrap();
        let outcome = process_requested_timestamp_and_observe_exact_health(
            &mut processor,
            &mut health,
            1,
            sample(9.0, "changed"),
        )
        .unwrap();
        assert_eq!(
            outcome.classification(),
            ExactSequenceClassification::Contiguous
        );
        assert_eq!(outcome.health().retained_unchanged_count(), 1);
        assert_eq!(outcome.health().retained_changed_count(), 1);
        let processed = outcome.into_processed();
        assert_eq!(
            processed.facts().effective_timestamp().value().to_bits(),
            11.0f64.to_bits()
        );
        let derived = processed.sample().derived_timestamp().unwrap();
        assert_eq!(derived.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(derived.value().to_bits(), 9.0f64.to_bits());
    }

    #[test]
    fn post_processing_error_observes_nothing_and_commits_neither_owner() {
        let mut processor = RequestedTimestampPostProcessor::new(
            RequestedTimestampPostProcessing::DeJitter(config()),
        )
        .unwrap();
        let mut health = ExactSequenceLossHealth::new(2);
        process_requested_timestamp_and_observe_exact_health(
            &mut processor,
            &mut health,
            0,
            sample(10.0, "first"),
        )
        .unwrap();
        let processor_before = processor.clone();
        let health_before = health.clone();
        let error = process_requested_timestamp_and_observe_exact_health(
            &mut processor,
            &mut health,
            1,
            sample(10.0, "rejected"),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            RequestedTimestampPostProcessingLossHealthError::PostProcessing(
                RequestedTimestampPostProcessingError::OrderInvalid { .. }
            )
        ));
        assert_eq!(processor, processor_before);
        assert_eq!(health, health_before);
    }

    #[test]
    fn health_refusal_returns_processed_sample_and_commits_neither_owner() {
        let mut processor = RequestedTimestampPostProcessor::new(
            RequestedTimestampPostProcessing::Monotonic(config()),
        )
        .unwrap();
        let mut health = ExactSequenceLossHealth::new(0);
        let processor_before = processor.clone();
        let health_before = health.clone();
        let input = sample(10.0, "retained");
        let allocation = input.sample().values()[0].as_ptr();
        let error = process_requested_timestamp_and_observe_exact_health(
            &mut processor,
            &mut health,
            0,
            input,
        )
        .unwrap_err();
        match error {
            RequestedTimestampPostProcessingLossHealthError::Health { processed, error } => {
                assert_eq!(
                    error,
                    ExactSequenceLossHealthError::ObservationLimitReached { limit: 0 }
                );
                assert_eq!(processed.sample().sample().values()[0].as_ptr(), allocation);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(processor, processor_before);
        assert_eq!(health, health_before);
    }
}
