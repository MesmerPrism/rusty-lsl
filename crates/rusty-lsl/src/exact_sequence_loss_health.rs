// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exact, caller-fed sequence and post-processing health observation.

/// A caller's explicit fact about requested post-processing of one observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExactPostProcessingFact {
    /// The observation was retained without post-processing changing it.
    RetainedUnchanged,
    /// The observation was retained after caller-owned post-processing changed it.
    RetainedChanged,
}

impl ExactPostProcessingFact {
    /// Maps the exact numerical result of successful requested post-processing.
    ///
    /// Callers must not invoke this for a post-processing error: an error produces
    /// no fact and therefore no loss-health observation.
    pub(crate) const fn from_successful_timestamp_change(changed: bool) -> Self {
        if changed {
            Self::RetainedChanged
        } else {
            Self::RetainedUnchanged
        }
    }
}

/// The exact relationship between one sequence number and prior admitted evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExactSequenceClassification {
    /// The first admitted sequence number establishes the high-water mark.
    First,
    /// The number is exactly one greater than the prior high-water mark.
    Contiguous,
    /// The number advances beyond the next number, with an exact intervening extent.
    Gap { missing_sequence_count: u64 },
    /// The number exactly repeats the current high-water mark.
    Duplicate,
    /// The number is below the current high-water mark by an exact distance.
    OutOfOrder { behind_high_water_by: u64 },
}

/// Immutable exact counters and sequence evidence from an observation owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExactSequenceLossHealthSnapshot {
    observation_count: u64,
    first_count: u64,
    contiguous_count: u64,
    gap_count: u64,
    explicit_missing_sequence_count: u64,
    duplicate_count: u64,
    out_of_order_count: u64,
    retained_unchanged_count: u64,
    retained_changed_count: u64,
    high_water_sequence: Option<u64>,
    last_classification: Option<ExactSequenceClassification>,
    last_post_processing_fact: Option<ExactPostProcessingFact>,
}

impl ExactSequenceLossHealthSnapshot {
    pub(crate) const fn observation_count(&self) -> u64 {
        self.observation_count
    }
    pub(crate) const fn first_count(&self) -> u64 {
        self.first_count
    }
    pub(crate) const fn contiguous_count(&self) -> u64 {
        self.contiguous_count
    }
    pub(crate) const fn gap_count(&self) -> u64 {
        self.gap_count
    }
    pub(crate) const fn explicit_missing_sequence_count(&self) -> u64 {
        self.explicit_missing_sequence_count
    }
    pub(crate) const fn duplicate_count(&self) -> u64 {
        self.duplicate_count
    }
    pub(crate) const fn out_of_order_count(&self) -> u64 {
        self.out_of_order_count
    }
    pub(crate) const fn retained_unchanged_count(&self) -> u64 {
        self.retained_unchanged_count
    }
    pub(crate) const fn retained_changed_count(&self) -> u64 {
        self.retained_changed_count
    }
    pub(crate) const fn high_water_sequence(&self) -> Option<u64> {
        self.high_water_sequence
    }
    pub(crate) const fn last_classification(&self) -> Option<ExactSequenceClassification> {
        self.last_classification
    }
    pub(crate) const fn last_post_processing_fact(&self) -> Option<ExactPostProcessingFact> {
        self.last_post_processing_fact
    }
}

/// Typed refusal that leaves the owner byte-for-byte unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExactSequenceLossHealthError {
    /// The caller-configured observation bound has already been reached.
    ObservationLimitReached { limit: u64 },
    /// An exact counter cannot represent the next accepted observation.
    CounterOverflow,
}

/// Bounded owner of exact caller-supplied sequence and post-processing facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExactSequenceLossHealth {
    observation_limit: u64,
    snapshot: ExactSequenceLossHealthSnapshot,
}

impl ExactSequenceLossHealth {
    pub(crate) const fn new(observation_limit: u64) -> Self {
        Self {
            observation_limit,
            snapshot: ExactSequenceLossHealthSnapshot {
                observation_count: 0,
                first_count: 0,
                contiguous_count: 0,
                gap_count: 0,
                explicit_missing_sequence_count: 0,
                duplicate_count: 0,
                out_of_order_count: 0,
                retained_unchanged_count: 0,
                retained_changed_count: 0,
                high_water_sequence: None,
                last_classification: None,
                last_post_processing_fact: None,
            },
        }
    }

    /// Admits one exact observation only after every affected counter is checked.
    pub(crate) fn observe(
        &mut self,
        sequence: u64,
        post_processing: ExactPostProcessingFact,
    ) -> Result<ExactSequenceClassification, ExactSequenceLossHealthError> {
        if self.snapshot.observation_count >= self.observation_limit {
            return Err(ExactSequenceLossHealthError::ObservationLimitReached {
                limit: self.observation_limit,
            });
        }

        let classification = classify(self.snapshot.high_water_sequence, sequence);
        let mut next = self.snapshot;
        next.observation_count = add(next.observation_count, 1)?;
        match classification {
            ExactSequenceClassification::First => next.first_count = add(next.first_count, 1)?,
            ExactSequenceClassification::Contiguous => {
                next.contiguous_count = add(next.contiguous_count, 1)?
            }
            ExactSequenceClassification::Gap {
                missing_sequence_count,
            } => {
                next.gap_count = add(next.gap_count, 1)?;
                next.explicit_missing_sequence_count =
                    add(next.explicit_missing_sequence_count, missing_sequence_count)?;
            }
            ExactSequenceClassification::Duplicate => {
                next.duplicate_count = add(next.duplicate_count, 1)?
            }
            ExactSequenceClassification::OutOfOrder { .. } => {
                next.out_of_order_count = add(next.out_of_order_count, 1)?
            }
        }
        match post_processing {
            ExactPostProcessingFact::RetainedUnchanged => {
                next.retained_unchanged_count = add(next.retained_unchanged_count, 1)?
            }
            ExactPostProcessingFact::RetainedChanged => {
                next.retained_changed_count = add(next.retained_changed_count, 1)?
            }
        }
        if next
            .high_water_sequence
            .map_or(true, |high| sequence > high)
        {
            next.high_water_sequence = Some(sequence);
        }
        next.last_classification = Some(classification);
        next.last_post_processing_fact = Some(post_processing);
        self.snapshot = next;
        Ok(classification)
    }

    /// Returns a deterministic immutable value copy; it borrows no caller evidence.
    pub(crate) const fn snapshot(&self) -> ExactSequenceLossHealthSnapshot {
        self.snapshot
    }
}

fn classify(high_water: Option<u64>, sequence: u64) -> ExactSequenceClassification {
    let Some(high_water) = high_water else {
        return ExactSequenceClassification::First;
    };
    if sequence == high_water {
        return ExactSequenceClassification::Duplicate;
    }
    if sequence < high_water {
        return ExactSequenceClassification::OutOfOrder {
            behind_high_water_by: high_water - sequence,
        };
    }
    if high_water.checked_add(1) == Some(sequence) {
        ExactSequenceClassification::Contiguous
    } else {
        ExactSequenceClassification::Gap {
            missing_sequence_count: sequence - high_water - 1,
        }
    }
}

fn add(value: u64, addition: u64) -> Result<u64, ExactSequenceLossHealthError> {
    value
        .checked_add(addition)
        .ok_or(ExactSequenceLossHealthError::CounterOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unchanged() -> ExactPostProcessingFact {
        ExactPostProcessingFact::RetainedUnchanged
    }

    fn assert_counter_overflow_without_mutation(
        mut health: ExactSequenceLossHealth,
        sequence: u64,
        fact: ExactPostProcessingFact,
    ) {
        let before = health.clone();
        assert_eq!(
            health.observe(sequence, fact),
            Err(ExactSequenceLossHealthError::CounterOverflow)
        );
        assert_eq!(health, before);
    }

    #[test]
    fn exact_relationships_use_only_caller_sequence_evidence() {
        let mut health = ExactSequenceLossHealth::new(8);
        let cases = [
            (10, ExactSequenceClassification::First),
            (11, ExactSequenceClassification::Contiguous),
            (
                14,
                ExactSequenceClassification::Gap {
                    missing_sequence_count: 2,
                },
            ),
            (14, ExactSequenceClassification::Duplicate),
            (
                12,
                ExactSequenceClassification::OutOfOrder {
                    behind_high_water_by: 2,
                },
            ),
            (15, ExactSequenceClassification::Contiguous),
        ];
        for (sequence, expected) in cases {
            assert_eq!(health.observe(sequence, unchanged()), Ok(expected));
        }
        let snapshot = health.snapshot();
        assert_eq!(snapshot.observation_count(), 6);
        assert_eq!(snapshot.first_count(), 1);
        assert_eq!(snapshot.contiguous_count(), 2);
        assert_eq!(snapshot.gap_count(), 1);
        assert_eq!(snapshot.explicit_missing_sequence_count(), 2);
        assert_eq!(snapshot.duplicate_count(), 1);
        assert_eq!(snapshot.out_of_order_count(), 1);
        assert_eq!(snapshot.high_water_sequence(), Some(15));
        assert_eq!(
            snapshot.last_classification(),
            Some(ExactSequenceClassification::Contiguous)
        );
    }

    #[test]
    fn successful_post_processing_mapping_is_deterministic_and_non_discarding() {
        assert_eq!(
            ExactPostProcessingFact::from_successful_timestamp_change(false),
            ExactPostProcessingFact::RetainedUnchanged
        );
        assert_eq!(
            ExactPostProcessingFact::from_successful_timestamp_change(true),
            ExactPostProcessingFact::RetainedChanged
        );

        let mut health = ExactSequenceLossHealth::new(2);
        health
            .observe(
                0,
                ExactPostProcessingFact::from_successful_timestamp_change(false),
            )
            .unwrap();
        health
            .observe(
                1,
                ExactPostProcessingFact::from_successful_timestamp_change(true),
            )
            .unwrap();
        let snapshot = health.snapshot();
        assert_eq!(snapshot.retained_unchanged_count(), 1);
        assert_eq!(snapshot.retained_changed_count(), 1);
        assert_eq!(
            snapshot.last_post_processing_fact(),
            Some(ExactPostProcessingFact::RetainedChanged)
        );
    }

    #[test]
    fn post_processing_error_produces_no_fact_observation_or_mutation() {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct ProcessingError;

        let mut health = ExactSequenceLossHealth::new(1);
        let before = health.clone();
        let processing: Result<bool, ProcessingError> = Err(ProcessingError);
        let fact = processing
            .ok()
            .map(ExactPostProcessingFact::from_successful_timestamp_change);
        assert_eq!(fact, None);
        if let Some(fact) = fact {
            health.observe(7, fact).unwrap();
        }
        assert_eq!(health, before);
    }

    #[test]
    fn zero_and_reached_bounds_reject_without_partial_mutation() {
        let mut zero = ExactSequenceLossHealth::new(0);
        let zero_before = zero.clone();
        assert_eq!(
            zero.observe(0, unchanged()),
            Err(ExactSequenceLossHealthError::ObservationLimitReached { limit: 0 })
        );
        assert_eq!(zero, zero_before);

        let mut health = ExactSequenceLossHealth::new(1);
        health
            .observe(7, ExactPostProcessingFact::RetainedChanged)
            .unwrap();
        let before = health.clone();
        assert_eq!(
            health.observe(9, unchanged()),
            Err(ExactSequenceLossHealthError::ObservationLimitReached { limit: 1 })
        );
        assert_eq!(health, before);
    }

    #[test]
    fn every_exercisable_affected_counter_overflow_has_no_partial_mutation() {
        let base = ExactSequenceLossHealth::new(u64::MAX);

        let mut first = base.clone();
        first.snapshot.first_count = u64::MAX;
        assert_counter_overflow_without_mutation(first, 4, unchanged());

        let mut contiguous = base.clone();
        contiguous.snapshot.high_water_sequence = Some(4);
        contiguous.snapshot.contiguous_count = u64::MAX;
        assert_counter_overflow_without_mutation(contiguous, 5, unchanged());

        let mut gap = base.clone();
        gap.snapshot.high_water_sequence = Some(4);
        gap.snapshot.gap_count = u64::MAX;
        assert_counter_overflow_without_mutation(gap, 6, unchanged());

        let mut missing = base.clone();
        missing.snapshot.high_water_sequence = Some(4);
        missing.snapshot.explicit_missing_sequence_count = u64::MAX;
        assert_counter_overflow_without_mutation(missing, 6, unchanged());

        let mut duplicate = base.clone();
        duplicate.snapshot.high_water_sequence = Some(4);
        duplicate.snapshot.duplicate_count = u64::MAX;
        assert_counter_overflow_without_mutation(duplicate, 4, unchanged());

        let mut out_of_order = base.clone();
        out_of_order.snapshot.high_water_sequence = Some(4);
        out_of_order.snapshot.out_of_order_count = u64::MAX;
        assert_counter_overflow_without_mutation(out_of_order, 3, unchanged());

        let mut retained_unchanged = base.clone();
        retained_unchanged.snapshot.retained_unchanged_count = u64::MAX;
        assert_counter_overflow_without_mutation(retained_unchanged, 4, unchanged());

        let mut retained_changed = base;
        retained_changed.snapshot.retained_changed_count = u64::MAX;
        assert_counter_overflow_without_mutation(
            retained_changed,
            4,
            ExactPostProcessingFact::RetainedChanged,
        );
    }

    #[test]
    fn observation_count_max_uses_bound_precedence_without_mutation() {
        let mut health = ExactSequenceLossHealth::new(u64::MAX);
        health.snapshot.observation_count = u64::MAX;
        let before = health.clone();
        assert_eq!(
            health.observe(0, unchanged()),
            Err(ExactSequenceLossHealthError::ObservationLimitReached { limit: u64::MAX })
        );
        assert_eq!(health, before);
    }

    #[test]
    fn u64_sequence_extremes_and_maximum_gap_are_exact_without_wrap() {
        let mut health = ExactSequenceLossHealth::new(4);
        assert_eq!(
            health.observe(0, unchanged()),
            Ok(ExactSequenceClassification::First)
        );
        assert_eq!(
            health.observe(u64::MAX, unchanged()),
            Ok(ExactSequenceClassification::Gap {
                missing_sequence_count: u64::MAX - 1
            })
        );
        assert_eq!(
            health.observe(u64::MAX, unchanged()),
            Ok(ExactSequenceClassification::Duplicate)
        );
        assert_eq!(
            health.observe(0, unchanged()),
            Ok(ExactSequenceClassification::OutOfOrder {
                behind_high_water_by: u64::MAX
            })
        );
        let snapshot = health.snapshot();
        assert_eq!(snapshot.high_water_sequence(), Some(u64::MAX));
        assert_eq!(snapshot.explicit_missing_sequence_count(), u64::MAX - 1);
    }
}
