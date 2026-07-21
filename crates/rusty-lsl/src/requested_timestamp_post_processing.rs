// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit caller-requested, bounded timestamp post-processing.
//!
//! `Monotonic` and `DeJitter` are bounded, project-owned candidate algorithms.
//! Their results are not evidence of behavioral, numerical, or protocol
//! equivalence with liblsl. They acquire no clocks and infer no policy or loss.

use crate::{DerivedTimestampKind, TimestampedSample};

const MAX_HISTORY_SAMPLES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessing {
    PassThrough,
    Monotonic(RequestedTimestampPostProcessingConfig),
    DeJitter(RequestedTimestampPostProcessingConfig),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RequestedTimestampPostProcessingConfig {
    history_samples: usize,
    minimum_step: f64,
    maximum_adjustment: f64,
}

impl RequestedTimestampPostProcessingConfig {
    pub(crate) fn new(
        history_samples: usize,
        minimum_step: f64,
        maximum_adjustment: f64,
    ) -> Result<Self, RequestedTimestampPostProcessingConfigError> {
        if !(2..=MAX_HISTORY_SAMPLES).contains(&history_samples) {
            return Err(
                RequestedTimestampPostProcessingConfigError::HistorySamples {
                    expected_min: 2,
                    expected_max: MAX_HISTORY_SAMPLES,
                    actual: history_samples,
                },
            );
        }
        if !minimum_step.is_finite() || minimum_step <= 0.0 {
            return Err(
                RequestedTimestampPostProcessingConfigError::InvalidMinimumStep {
                    bits: minimum_step.to_bits(),
                },
            );
        }
        if !maximum_adjustment.is_finite() || maximum_adjustment < 0.0 {
            return Err(
                RequestedTimestampPostProcessingConfigError::InvalidMaximumAdjustment {
                    bits: maximum_adjustment.to_bits(),
                },
            );
        }
        Ok(Self {
            history_samples,
            minimum_step,
            maximum_adjustment,
        })
    }

    pub(crate) const fn history_samples(self) -> usize {
        self.history_samples
    }
    pub(crate) const fn minimum_step(self) -> f64 {
        self.minimum_step
    }
    pub(crate) const fn maximum_adjustment(self) -> f64 {
        self.maximum_adjustment
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingConfigError {
    HistorySamples {
        expected_min: usize,
        expected_max: usize,
        actual: usize,
    },
    InvalidMinimumStep {
        bits: u64,
    },
    InvalidMaximumAdjustment {
        bits: u64,
    },
    AllocationFailed {
        requested: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingKind {
    PassThrough,
    Monotonic,
    DeJitter,
}

/// The only successful dispositions produced by this non-discarding owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingDisposition {
    RetainedUnchanged,
    RetainedChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestedEffectiveTimestampSource {
    RawSource,
    ExistingDerived(DerivedTimestampKind),
    ProjectPostProcessed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RequestedEffectiveTimestamp {
    value: f64,
    source: RequestedEffectiveTimestampSource,
}

impl RequestedEffectiveTimestamp {
    pub(crate) const fn value(self) -> f64 {
        self.value
    }
    pub(crate) const fn source(self) -> RequestedEffectiveTimestampSource {
        self.source
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RequestedTimestampPostProcessingFacts {
    kind: RequestedTimestampPostProcessingKind,
    input_timestamp: f64,
    effective_timestamp: RequestedEffectiveTimestamp,
    adjustment: f64,
    disposition: RequestedTimestampPostProcessingDisposition,
    retained_history_samples: usize,
    state_advanced: bool,
}

impl RequestedTimestampPostProcessingFacts {
    pub(crate) const fn kind(&self) -> RequestedTimestampPostProcessingKind {
        self.kind
    }
    pub(crate) const fn input_timestamp(&self) -> f64 {
        self.input_timestamp
    }
    pub(crate) const fn effective_timestamp(&self) -> RequestedEffectiveTimestamp {
        self.effective_timestamp
    }
    pub(crate) const fn adjustment(&self) -> f64 {
        self.adjustment
    }
    pub(crate) const fn disposition(&self) -> RequestedTimestampPostProcessingDisposition {
        self.disposition
    }
    pub(crate) const fn retained_history_samples(&self) -> usize {
        self.retained_history_samples
    }
    pub(crate) const fn state_advanced(&self) -> bool {
        self.state_advanced
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RequestedTimestampPostProcessed<T> {
    sample: TimestampedSample<T>,
    facts: RequestedTimestampPostProcessingFacts,
}

impl<T> RequestedTimestampPostProcessed<T> {
    pub(crate) const fn sample(&self) -> &TimestampedSample<T> {
        &self.sample
    }
    pub(crate) const fn facts(&self) -> &RequestedTimestampPostProcessingFacts {
        &self.facts
    }
    pub(crate) fn into_parts(
        self,
    ) -> (TimestampedSample<T>, RequestedTimestampPostProcessingFacts) {
        (self.sample, self.facts)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingInvariant {
    PassThroughRetainedState,
    HistoryExceedsConfiguredBound {
        configured: usize,
        actual: usize,
    },
    MissingLastOutput,
    UnexpectedLastOutput,
    NonFiniteLastOutput {
        bits: u64,
    },
    NonFiniteHistory {
        index: usize,
        bits: u64,
    },
    DeJitterHistoryOrder {
        index: usize,
        previous_bits: u64,
        actual_bits: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingError<T> {
    InvariantViolation {
        sample: TimestampedSample<T>,
        invariant: RequestedTimestampPostProcessingInvariant,
    },
    ArithmeticOverflow {
        sample: TimestampedSample<T>,
    },
    OrderInvalid {
        sample: TimestampedSample<T>,
        previous_input_bits: u64,
        input_bits: u64,
    },
    AdjustmentLimitExceeded {
        sample: TimestampedSample<T>,
        limit_bits: u64,
        required_bits: u64,
    },
}

impl<T> RequestedTimestampPostProcessingError<T> {
    pub(crate) fn into_sample(self) -> TimestampedSample<T> {
        match self {
            Self::InvariantViolation { sample, .. }
            | Self::ArithmeticOverflow { sample }
            | Self::OrderInvalid { sample, .. }
            | Self::AdjustmentLimitExceeded { sample, .. } => sample,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RequestedTimestampPostProcessor {
    request: RequestedTimestampPostProcessing,
    input_history: Vec<f64>,
    last_output: Option<f64>,
}

impl RequestedTimestampPostProcessor {
    pub(crate) fn new(
        request: RequestedTimestampPostProcessing,
    ) -> Result<Self, RequestedTimestampPostProcessingConfigError> {
        let capacity = configured_history(request).unwrap_or(0);
        let mut input_history = Vec::new();
        input_history.try_reserve_exact(capacity).map_err(|_| {
            RequestedTimestampPostProcessingConfigError::AllocationFailed {
                requested: capacity,
            }
        })?;
        Ok(Self {
            request,
            input_history,
            last_output: None,
        })
    }

    /// Rehydrates retained state supplied by a crate-internal persistence owner.
    /// Invalid combinations remain representable so `process` can fail closed,
    /// return the sample, and preserve the retained evidence unchanged.
    pub(crate) fn from_retained_state(
        request: RequestedTimestampPostProcessing,
        mut input_history: Vec<f64>,
        last_output: Option<f64>,
    ) -> Result<Self, RequestedTimestampPostProcessingConfigError> {
        let configured = configured_history(request).unwrap_or(0);
        let additional = configured.saturating_sub(input_history.len());
        input_history.try_reserve_exact(additional).map_err(|_| {
            RequestedTimestampPostProcessingConfigError::AllocationFailed {
                requested: configured,
            }
        })?;
        Ok(Self {
            request,
            input_history,
            last_output,
        })
    }

    pub(crate) fn process<T>(
        &mut self,
        sample: TimestampedSample<T>,
    ) -> Result<RequestedTimestampPostProcessed<T>, RequestedTimestampPostProcessingError<T>> {
        if let Err(invariant) = self.validate_state() {
            return Err(RequestedTimestampPostProcessingError::InvariantViolation {
                sample,
                invariant,
            });
        }
        let existing_derived = sample.derived_timestamp();
        let (input, input_source) = existing_derived.map_or_else(
            || {
                (
                    sample.raw_source_timestamp().value(),
                    RequestedEffectiveTimestampSource::RawSource,
                )
            },
            |timestamp| {
                (
                    timestamp.value(),
                    RequestedEffectiveTimestampSource::ExistingDerived(timestamp.kind()),
                )
            },
        );

        let (kind, output, config) = match self.request {
            RequestedTimestampPostProcessing::PassThrough => {
                let facts = facts(
                    RequestedTimestampPostProcessingKind::PassThrough,
                    input,
                    input,
                    input_source,
                    self.input_history.len(),
                    false,
                );
                return Ok(RequestedTimestampPostProcessed { sample, facts });
            }
            RequestedTimestampPostProcessing::Monotonic(config) => {
                let output = match self.last_output {
                    None => input,
                    Some(previous) => {
                        let floor = previous + config.minimum_step;
                        if !floor.is_finite() {
                            return Err(
                                RequestedTimestampPostProcessingError::ArithmeticOverflow {
                                    sample,
                                },
                            );
                        }
                        input.max(floor)
                    }
                };
                (
                    RequestedTimestampPostProcessingKind::Monotonic,
                    output,
                    config,
                )
            }
            RequestedTimestampPostProcessing::DeJitter(config) => {
                let output = match (
                    self.input_history.first(),
                    self.input_history.last(),
                    self.last_output,
                ) {
                    (None, None, None) => input,
                    (Some(&first), Some(&previous_input), Some(previous_output)) => {
                        if input <= previous_input {
                            return Err(RequestedTimestampPostProcessingError::OrderInvalid {
                                sample,
                                previous_input_bits: previous_input.to_bits(),
                                input_bits: input.to_bits(),
                            });
                        }
                        let span = input - first;
                        let mean_step = span / self.input_history.len() as f64;
                        let output = previous_output + mean_step.max(config.minimum_step);
                        if !span.is_finite() || !mean_step.is_finite() || !output.is_finite() {
                            return Err(
                                RequestedTimestampPostProcessingError::ArithmeticOverflow {
                                    sample,
                                },
                            );
                        }
                        output
                    }
                    _ => {
                        return Err(RequestedTimestampPostProcessingError::InvariantViolation {
                            sample,
                            invariant: RequestedTimestampPostProcessingInvariant::MissingLastOutput,
                        });
                    }
                };
                (
                    RequestedTimestampPostProcessingKind::DeJitter,
                    output,
                    config,
                )
            }
        };

        let adjustment = output - input;
        if !adjustment.is_finite() {
            return Err(RequestedTimestampPostProcessingError::ArithmeticOverflow { sample });
        }
        if adjustment.abs() > config.maximum_adjustment {
            return Err(
                RequestedTimestampPostProcessingError::AdjustmentLimitExceeded {
                    sample,
                    limit_bits: config.maximum_adjustment.to_bits(),
                    required_bits: adjustment.abs().to_bits(),
                },
            );
        }

        if self.input_history.len() == config.history_samples {
            self.input_history.remove(0);
        }
        self.input_history.push(input);
        self.last_output = Some(output);
        let facts = facts(
            kind,
            input,
            output,
            RequestedEffectiveTimestampSource::ProjectPostProcessed,
            self.input_history.len(),
            true,
        );
        Ok(RequestedTimestampPostProcessed { sample, facts })
    }

    fn validate_state(&self) -> Result<(), RequestedTimestampPostProcessingInvariant> {
        let Some(limit) = configured_history(self.request) else {
            return if self.input_history.is_empty() && self.last_output.is_none() {
                Ok(())
            } else {
                Err(RequestedTimestampPostProcessingInvariant::PassThroughRetainedState)
            };
        };
        if self.input_history.len() > limit {
            return Err(
                RequestedTimestampPostProcessingInvariant::HistoryExceedsConfiguredBound {
                    configured: limit,
                    actual: self.input_history.len(),
                },
            );
        }
        match (self.input_history.is_empty(), self.last_output) {
            (true, Some(_)) => {
                return Err(RequestedTimestampPostProcessingInvariant::UnexpectedLastOutput)
            }
            (false, None) => {
                return Err(RequestedTimestampPostProcessingInvariant::MissingLastOutput)
            }
            (_, Some(value)) if !value.is_finite() => {
                return Err(
                    RequestedTimestampPostProcessingInvariant::NonFiniteLastOutput {
                        bits: value.to_bits(),
                    },
                );
            }
            _ => {}
        }
        for (index, value) in self.input_history.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(
                    RequestedTimestampPostProcessingInvariant::NonFiniteHistory {
                        index,
                        bits: value.to_bits(),
                    },
                );
            }
            if matches!(self.request, RequestedTimestampPostProcessing::DeJitter(_)) && index > 0 {
                let previous = self.input_history[index - 1];
                if value <= previous {
                    return Err(
                        RequestedTimestampPostProcessingInvariant::DeJitterHistoryOrder {
                            index,
                            previous_bits: previous.to_bits(),
                            actual_bits: value.to_bits(),
                        },
                    );
                }
            }
        }
        Ok(())
    }
}

fn configured_history(request: RequestedTimestampPostProcessing) -> Option<usize> {
    match request {
        RequestedTimestampPostProcessing::PassThrough => None,
        RequestedTimestampPostProcessing::Monotonic(config)
        | RequestedTimestampPostProcessing::DeJitter(config) => Some(config.history_samples),
    }
}

fn facts(
    kind: RequestedTimestampPostProcessingKind,
    input: f64,
    output: f64,
    source: RequestedEffectiveTimestampSource,
    retained_history_samples: usize,
    state_advanced: bool,
) -> RequestedTimestampPostProcessingFacts {
    let disposition = if input.to_bits() == output.to_bits() {
        RequestedTimestampPostProcessingDisposition::RetainedUnchanged
    } else {
        RequestedTimestampPostProcessingDisposition::RetainedChanged
    };
    RequestedTimestampPostProcessingFacts {
        kind,
        input_timestamp: input,
        effective_timestamp: RequestedEffectiveTimestamp {
            value: output,
            source,
        },
        adjustment: output - input,
        disposition,
        retained_history_samples,
        state_advanced,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DerivedTimestamp, RawSourceTimestamp, Sample, SampleLimits};

    fn sample(
        timestamp: f64,
        value: &str,
        derived: Option<DerivedTimestamp>,
    ) -> TimestampedSample<String> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value.to_owned()]).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            derived,
        )
    }

    fn corrected(value: f64) -> DerivedTimestamp {
        DerivedTimestamp::new(DerivedTimestampKind::ClockCorrected, value).unwrap()
    }

    fn config(
        history: usize,
        step: f64,
        adjustment: f64,
    ) -> RequestedTimestampPostProcessingConfig {
        RequestedTimestampPostProcessingConfig::new(history, step, adjustment).unwrap()
    }

    fn assert_preserved(
        sample: &TimestampedSample<String>,
        pointer: *const u8,
        raw: f64,
        derived: f64,
    ) {
        assert_eq!(sample.sample().values()[0].as_ptr(), pointer);
        assert_eq!(
            sample.raw_source_timestamp().value().to_bits(),
            raw.to_bits()
        );
        let retained = sample.derived_timestamp().unwrap();
        assert_eq!(retained.kind(), DerivedTimestampKind::ClockCorrected);
        assert_eq!(retained.value().to_bits(), derived.to_bits());
    }

    #[test]
    fn clock_corrected_sample_and_allocation_survive_every_success_disposition() {
        for request in [
            RequestedTimestampPostProcessing::PassThrough,
            RequestedTimestampPostProcessing::Monotonic(config(2, 0.5, 2.0)),
            RequestedTimestampPostProcessing::DeJitter(config(2, 0.5, 2.0)),
        ] {
            let mut owner = RequestedTimestampPostProcessor::new(request).unwrap();
            let input = sample(1.0, "owned", Some(corrected(10.0)));
            let pointer = input.sample().values()[0].as_ptr();
            let first = owner.process(input).unwrap();
            assert_preserved(first.sample(), pointer, 1.0, 10.0);
            assert_eq!(
                first.facts().disposition(),
                RequestedTimestampPostProcessingDisposition::RetainedUnchanged
            );
            if !matches!(request, RequestedTimestampPostProcessing::PassThrough) {
                let changed = owner
                    .process(sample(2.0, "next", Some(corrected(10.1))))
                    .unwrap();
                assert_eq!(
                    changed.facts().disposition(),
                    RequestedTimestampPostProcessingDisposition::RetainedChanged
                );
                assert_eq!(
                    changed.sample().derived_timestamp().unwrap(),
                    corrected(10.1)
                );
            }
        }
    }

    #[test]
    fn exact_adjustment_limit_edge_succeeds_and_one_bit_over_rejects_without_state_change() {
        let request = RequestedTimestampPostProcessing::Monotonic(config(2, 1.0, 0.25));
        let mut owner = RequestedTimestampPostProcessor::new(request).unwrap();
        owner.process(sample(0.0, "first", None)).unwrap();
        let edge = owner.process(sample(0.75, "edge", None)).unwrap();
        assert_eq!(edge.facts().adjustment(), 0.25);

        let mut owner = RequestedTimestampPostProcessor::new(request).unwrap();
        owner.process(sample(0.0, "first", None)).unwrap();
        let before = (owner.input_history.clone(), owner.last_output);
        let just_over = f64::from_bits(0.75_f64.to_bits() - 1);
        let rejected = owner.process(sample(just_over, "rejected", Some(corrected(just_over))));
        let returned = rejected.unwrap_err().into_sample();
        assert_eq!(returned.derived_timestamp().unwrap(), corrected(just_over));
        assert_eq!((owner.input_history.clone(), owner.last_output), before);
    }

    #[test]
    fn order_rejection_preserves_clock_evidence_allocation_and_state() {
        let mut owner = RequestedTimestampPostProcessor::new(
            RequestedTimestampPostProcessing::DeJitter(config(2, 0.1, 2.0)),
        )
        .unwrap();
        owner
            .process(sample(1.0, "first", Some(corrected(1.0))))
            .unwrap();
        let before = (owner.input_history.clone(), owner.last_output);
        let input = sample(2.0, "rejected", Some(corrected(1.0)));
        let pointer = input.sample().values()[0].as_ptr();
        let returned = owner.process(input).unwrap_err().into_sample();
        assert_preserved(&returned, pointer, 2.0, 1.0);
        assert_eq!((owner.input_history.clone(), owner.last_output), before);
    }

    #[test]
    fn typed_invariants_are_constructible_and_do_not_mutate_or_observe() {
        let request = RequestedTimestampPostProcessing::DeJitter(config(2, 0.1, 2.0));
        let mut owner =
            RequestedTimestampPostProcessor::from_retained_state(request, vec![1.0], None).unwrap();
        let before = (owner.input_history.clone(), owner.last_output);
        let input = sample(3.0, "invariant", Some(corrected(2.0)));
        let pointer = input.sample().values()[0].as_ptr();
        let error = owner.process(input).unwrap_err();
        assert!(matches!(
            error,
            RequestedTimestampPostProcessingError::InvariantViolation {
                invariant: RequestedTimestampPostProcessingInvariant::MissingLastOutput,
                ..
            }
        ));
        let returned = error.into_sample();
        assert_preserved(&returned, pointer, 3.0, 2.0);
        assert_eq!((owner.input_history.clone(), owner.last_output), before);

        let mut pass_through = RequestedTimestampPostProcessor::from_retained_state(
            RequestedTimestampPostProcessing::PassThrough,
            vec![1.0],
            Some(1.0),
        )
        .unwrap();
        let before = (pass_through.input_history.clone(), pass_through.last_output);
        let error = pass_through
            .process(sample(4.0, "pass invariant", Some(corrected(3.0))))
            .unwrap_err();
        assert!(matches!(
            error,
            RequestedTimestampPostProcessingError::InvariantViolation {
                invariant: RequestedTimestampPostProcessingInvariant::PassThroughRetainedState,
                ..
            }
        ));
        assert_eq!(
            (pass_through.input_history.clone(), pass_through.last_output),
            before
        );
    }

    #[test]
    fn arithmetic_rejection_preserves_sample_and_state() {
        let request = RequestedTimestampPostProcessing::Monotonic(config(2, 1.0e308, f64::MAX));
        let mut owner = RequestedTimestampPostProcessor::from_retained_state(
            request,
            vec![f64::MAX],
            Some(f64::MAX),
        )
        .unwrap();
        let before = (owner.input_history.clone(), owner.last_output);
        let input = sample(4.0, "overflow", Some(corrected(4.0)));
        let pointer = input.sample().values()[0].as_ptr();
        let returned = owner.process(input).unwrap_err().into_sample();
        assert_preserved(&returned, pointer, 4.0, 4.0);
        assert_eq!((owner.input_history.clone(), owner.last_output), before);
    }

    #[test]
    fn history_never_exceeds_the_configured_bound() {
        for request in [
            RequestedTimestampPostProcessing::Monotonic(config(2, 0.1, 10.0)),
            RequestedTimestampPostProcessing::DeJitter(config(2, 0.1, 10.0)),
        ] {
            let mut owner = RequestedTimestampPostProcessor::new(request).unwrap();
            for index in 1..=8 {
                let result = owner.process(sample(index as f64, "value", None)).unwrap();
                assert!(result.facts().retained_history_samples() <= 2);
            }
            assert_eq!(owner.input_history.len(), 2);
        }
    }

    #[test]
    fn pass_through_effective_source_retains_existing_classification() {
        let mut owner =
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::PassThrough)
                .unwrap();
        let result = owner
            .process(sample(1.0, "value", Some(corrected(7.0))))
            .unwrap();
        assert_eq!(result.facts().effective_timestamp().value(), 7.0);
        assert_eq!(
            result.facts().effective_timestamp().source(),
            RequestedEffectiveTimestampSource::ExistingDerived(
                DerivedTimestampKind::ClockCorrected
            ),
        );
    }

    #[test]
    fn candidate_non_equivalence_boundary_is_explicit() {
        let source = include_str!("requested_timestamp_post_processing.rs");
        assert!(source.contains("not evidence of behavioral, numerical, or protocol"));
        assert!(source.contains("equivalence with liblsl"));
    }
}
