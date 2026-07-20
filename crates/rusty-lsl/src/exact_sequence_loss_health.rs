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
    /// Caller-owned post-processing discarded the observation.
    Discarded,
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
    discarded_count: u64,
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
    pub(crate) const fn discarded_count(&self) -> u64 {
        self.discarded_count
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
                discarded_count: 0,
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
            ExactPostProcessingFact::Discarded => {
                next.discarded_count = add(next.discarded_count, 1)?
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
            assert_eq!(
                health.observe(sequence, ExactPostProcessingFact::RetainedUnchanged),
                Ok(expected)
            );
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
    fn explicit_post_processing_facts_are_counted_without_inference() {
        let mut health = ExactSequenceLossHealth::new(3);
        health
            .observe(0, ExactPostProcessingFact::RetainedUnchanged)
            .unwrap();
        health
            .observe(1, ExactPostProcessingFact::RetainedChanged)
            .unwrap();
        health
            .observe(2, ExactPostProcessingFact::Discarded)
            .unwrap();
        let snapshot = health.snapshot();
        assert_eq!(snapshot.retained_unchanged_count(), 1);
        assert_eq!(snapshot.retained_changed_count(), 1);
        assert_eq!(snapshot.discarded_count(), 1);
        assert_eq!(
            snapshot.last_post_processing_fact(),
            Some(ExactPostProcessingFact::Discarded)
        );
    }

    #[test]
    fn bound_rejection_has_no_partial_mutation() {
        let mut health = ExactSequenceLossHealth::new(1);
        health
            .observe(7, ExactPostProcessingFact::RetainedChanged)
            .unwrap();
        let before = health.clone();
        assert_eq!(
            health.observe(9, ExactPostProcessingFact::Discarded),
            Err(ExactSequenceLossHealthError::ObservationLimitReached { limit: 1 })
        );
        assert_eq!(health, before);
    }

    #[test]
    fn checked_counter_failure_has_no_partial_mutation() {
        let mut health = ExactSequenceLossHealth::new(u64::MAX);
        health.snapshot.observation_count = u64::MAX - 1;
        health.snapshot.explicit_missing_sequence_count = u64::MAX;
        health.snapshot.high_water_sequence = Some(0);
        let before = health.clone();
        assert_eq!(
            health.observe(2, ExactPostProcessingFact::RetainedUnchanged),
            Err(ExactSequenceLossHealthError::CounterOverflow)
        );
        assert_eq!(health, before);
    }

    #[test]
    fn maximum_sequence_never_wraps() {
        let mut health = ExactSequenceLossHealth::new(3);
        assert_eq!(
            health.observe(u64::MAX, ExactPostProcessingFact::RetainedUnchanged),
            Ok(ExactSequenceClassification::First)
        );
        assert_eq!(
            health.observe(u64::MAX, ExactPostProcessingFact::RetainedUnchanged),
            Ok(ExactSequenceClassification::Duplicate)
        );
        assert_eq!(
            health.observe(u64::MAX - 1, ExactPostProcessingFact::RetainedUnchanged),
            Ok(ExactSequenceClassification::OutOfOrder {
                behind_high_water_by: 1
            })
        );
    }
}
