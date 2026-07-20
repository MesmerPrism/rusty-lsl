// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit caller-requested, bounded timestamp post-processing.

use crate::{DerivedTimestamp, DerivedTimestampKind, TimestampedSample};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RequestedTimestampPostProcessingFacts {
    kind: RequestedTimestampPostProcessingKind,
    input_timestamp: f64,
    output_timestamp: f64,
    adjustment: f64,
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
    pub(crate) const fn output_timestamp(&self) -> f64 {
        self.output_timestamp
    }
    pub(crate) const fn adjustment(&self) -> f64 {
        self.adjustment
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

#[derive(Debug, PartialEq)]
pub(crate) enum RequestedTimestampPostProcessingError<T> {
    NonFiniteResult {
        sample: TimestampedSample<T>,
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

#[derive(Debug)]
pub(crate) struct RequestedTimestampPostProcessor {
    request: RequestedTimestampPostProcessing,
    input_history: Vec<f64>,
    last_output: Option<f64>,
}

impl RequestedTimestampPostProcessor {
    pub(crate) fn new(
        request: RequestedTimestampPostProcessing,
    ) -> Result<Self, RequestedTimestampPostProcessingConfigError> {
        let capacity = match request {
            RequestedTimestampPostProcessing::PassThrough => 0,
            RequestedTimestampPostProcessing::Monotonic(config)
            | RequestedTimestampPostProcessing::DeJitter(config) => config.history_samples,
        };
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

    pub(crate) fn process<T>(
        &mut self,
        sample: TimestampedSample<T>,
    ) -> Result<RequestedTimestampPostProcessed<T>, RequestedTimestampPostProcessingError<T>> {
        let input = sample.derived_timestamp().map_or_else(
            || sample.raw_source_timestamp().value(),
            |timestamp| timestamp.value(),
        );
        let (kind, output, history_limit, advance) = match self.request {
            RequestedTimestampPostProcessing::PassThrough => {
                let facts = RequestedTimestampPostProcessingFacts {
                    kind: RequestedTimestampPostProcessingKind::PassThrough,
                    input_timestamp: input,
                    output_timestamp: input,
                    adjustment: 0.0,
                    retained_history_samples: self.input_history.len(),
                    state_advanced: false,
                };
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
                    config.history_samples,
                    true,
                )
            }
            RequestedTimestampPostProcessing::DeJitter(config) => {
                if let Some(&previous_input) = self.input_history.last() {
                    if input <= previous_input {
                        return Err(RequestedTimestampPostProcessingError::OrderInvalid {
                            sample,
                            previous_input_bits: previous_input.to_bits(),
                            input_bits: input.to_bits(),
                        });
                    }
                }
                let output = if self.input_history.is_empty() {
                    input
                } else {
                    let first = self.input_history[0];
                    let intervals = self.input_history.len() as f64;
                    let span = input - first;
                    let mean_step = span / intervals;
                    let step = mean_step.max(config.minimum_step);
                    let output = self
                        .last_output
                        .expect("history and output advance together")
                        + step;
                    if !span.is_finite() || !mean_step.is_finite() || !output.is_finite() {
                        return Err(RequestedTimestampPostProcessingError::ArithmeticOverflow {
                            sample,
                        });
                    }
                    output
                };
                (
                    RequestedTimestampPostProcessingKind::DeJitter,
                    output,
                    config.history_samples,
                    true,
                )
            }
        };

        if !output.is_finite() {
            return Err(RequestedTimestampPostProcessingError::NonFiniteResult { sample });
        }
        let adjustment = output - input;
        if !adjustment.is_finite() {
            return Err(RequestedTimestampPostProcessingError::ArithmeticOverflow { sample });
        }
        let config = match self.request {
            RequestedTimestampPostProcessing::Monotonic(config)
            | RequestedTimestampPostProcessing::DeJitter(config) => config,
            RequestedTimestampPostProcessing::PassThrough => unreachable!(),
        };
        if adjustment.abs() > config.maximum_adjustment {
            return Err(
                RequestedTimestampPostProcessingError::AdjustmentLimitExceeded {
                    sample,
                    limit_bits: config.maximum_adjustment.to_bits(),
                    required_bits: adjustment.abs().to_bits(),
                },
            );
        }
        let derived = match DerivedTimestamp::new(DerivedTimestampKind::Smoothed, output) {
            Ok(timestamp) => timestamp,
            Err(_) => {
                return Err(RequestedTimestampPostProcessingError::NonFiniteResult { sample })
            }
        };
        let (values, raw, _) = sample.into_parts();
        let sample = TimestampedSample::new(values, raw, Some(derived));

        if advance {
            if self.input_history.len() == history_limit {
                self.input_history.remove(0);
            }
            self.input_history.push(input);
            self.last_output = Some(output);
        }
        let facts = RequestedTimestampPostProcessingFacts {
            kind,
            input_timestamp: input,
            output_timestamp: output,
            adjustment,
            retained_history_samples: self.input_history.len(),
            state_advanced: advance,
        };
        Ok(RequestedTimestampPostProcessed { sample, facts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RawSourceTimestamp, Sample, SampleLimits};

    fn sample(timestamp: f64, values: Vec<String>) -> TimestampedSample<String> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(values.len()).unwrap(),
                values.len(),
                values,
            )
            .unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            None,
        )
    }

    fn config(
        history: usize,
        step: f64,
        adjustment: f64,
    ) -> RequestedTimestampPostProcessingConfig {
        RequestedTimestampPostProcessingConfig::new(history, step, adjustment).unwrap()
    }

    #[test]
    fn pass_through_is_inert_and_preserves_allocation() {
        let values = vec![String::from("owned")];
        let pointer = values[0].as_ptr();
        let mut owner =
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::PassThrough)
                .unwrap();
        let output = owner.process(sample(2.0, values)).unwrap();
        assert_eq!(output.sample().sample().values()[0].as_ptr(), pointer);
        assert_eq!(output.sample().derived_timestamp(), None);
        assert!(!output.facts().state_advanced());
    }

    #[test]
    fn monotonic_is_bounded_and_failure_does_not_advance() {
        let mut owner = RequestedTimestampPostProcessor::new(
            RequestedTimestampPostProcessing::Monotonic(config(3, 0.5, 1.0)),
        )
        .unwrap();
        assert_eq!(
            owner
                .process(sample(10.0, vec!["a".into()]))
                .unwrap()
                .facts()
                .output_timestamp(),
            10.0
        );
        let rejected = owner.process(sample(8.0, vec!["b".into()]));
        assert!(matches!(
            rejected,
            Err(RequestedTimestampPostProcessingError::AdjustmentLimitExceeded { .. })
        ));
        let accepted = owner.process(sample(10.25, vec!["c".into()])).unwrap();
        assert_eq!(accepted.facts().output_timestamp(), 10.5);
        assert_eq!(accepted.facts().retained_history_samples(), 2);
        assert_eq!(accepted.sample().raw_source_timestamp().value(), 10.25);
    }

    #[test]
    fn dejitter_uses_bounded_history_and_rejects_order_without_mutation() {
        let mut owner = RequestedTimestampPostProcessor::new(
            RequestedTimestampPostProcessing::DeJitter(config(2, 0.1, 2.0)),
        )
        .unwrap();
        owner.process(sample(1.0, vec!["a".into()])).unwrap();
        owner.process(sample(2.2, vec!["b".into()])).unwrap();
        assert!(matches!(
            owner.process(sample(2.2, vec!["x".into()])),
            Err(RequestedTimestampPostProcessingError::OrderInvalid { .. })
        ));
        let output = owner.process(sample(3.0, vec!["c".into()])).unwrap();
        assert_eq!(output.facts().retained_history_samples(), 2);
        assert_eq!(
            output.facts().kind(),
            RequestedTimestampPostProcessingKind::DeJitter
        );
        assert_eq!(output.sample().sample().values(), &[String::from("c")]);
        assert_eq!(output.sample().raw_source_timestamp().value(), 3.0);
    }

    #[test]
    fn configuration_rejects_unbounded_and_non_finite_values() {
        assert!(matches!(
            RequestedTimestampPostProcessingConfig::new(1, 1.0, 1.0),
            Err(RequestedTimestampPostProcessingConfigError::HistorySamples { .. })
        ));
        assert!(matches!(
            RequestedTimestampPostProcessingConfig::new(2, f64::NAN, 1.0),
            Err(RequestedTimestampPostProcessingConfigError::InvalidMinimumStep { .. })
        ));
        assert!(matches!(
            RequestedTimestampPostProcessingConfig::new(2, 1.0, f64::INFINITY),
            Err(RequestedTimestampPostProcessingConfigError::InvalidMaximumAdjustment { .. })
        ));
    }
}
