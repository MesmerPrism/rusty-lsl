// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic, effect-free advisory proposals over a completed Float32 report observation.
//!
//! This candidate is deliberately crate-private and unwired. It reads only exact facts supplied
//! by the sibling observation interface, returns the observation unchanged, and owns no action,
//! acceptance, routing, lease, revision, authorization, application, or audit mechanism.

/// Exact terminal health supplied by the completed-report observation owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportObservedTerminalHealth {
    Complete,
    EmptyReport,
    Cancelled,
    Deadline,
    Terminal,
    Exhausted,
    RecoveryError,
    PipelineError,
    Invariant,
}

/// Exact sequence classification supplied for one ordered observed record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportObservedSequenceClassification {
    First,
    Contiguous,
    Gap { missing: u64 },
    Duplicate,
    OutOfOrder { distance: u64 },
}

/// Exact successful disposition supplied by post-processing observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportObservedDisposition {
    RetainedUnchanged,
    RetainedChanged,
}

/// Borrowed record view required from the frozen sibling observation.
pub(crate) trait Float32ReportAdvisoryObservedRecord {
    fn sequence(&self) -> u64;
    fn classification(&self) -> Float32ReportObservedSequenceClassification;
    fn disposition(&self) -> Float32ReportObservedDisposition;
    fn adjustment_bits(&self) -> u64;
    fn effective_timestamp_bits(&self) -> u64;
}

/// Borrowed, ordered view required from the frozen sibling observation.
pub(crate) trait Float32ReportAdvisoryObservation {
    type Record: Float32ReportAdvisoryObservedRecord;

    fn records(&self) -> &[Self::Record];
    fn terminal_health(&self) -> Float32ReportObservedTerminalHealth;
}

/// Caller-owned bounded policy for producing advice, never applying it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportAdvisoryProposalConfig {
    maximum_records: usize,
    maximum_explicit_missing: u64,
    maximum_duplicates: u64,
    maximum_out_of_order: u64,
    maximum_adjusted: u64,
    maximum_absolute_adjustment_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportAdvisoryProposalConfigError {
    ZeroMaximumRecords,
    MaximumRecordsUnrepresentable { requested: usize },
    NonFiniteMaximumAbsoluteAdjustment { bits: u64 },
    NegativeMaximumAbsoluteAdjustment { bits: u64 },
}

impl Float32ReportAdvisoryProposalConfig {
    pub(crate) fn new(
        maximum_records: usize,
        maximum_explicit_missing: u64,
        maximum_duplicates: u64,
        maximum_out_of_order: u64,
        maximum_adjusted: u64,
        maximum_absolute_adjustment: f64,
    ) -> Result<Self, Float32ReportAdvisoryProposalConfigError> {
        if maximum_records == 0 {
            return Err(Float32ReportAdvisoryProposalConfigError::ZeroMaximumRecords);
        }
        if u64::try_from(maximum_records).is_err() {
            return Err(
                Float32ReportAdvisoryProposalConfigError::MaximumRecordsUnrepresentable {
                    requested: maximum_records,
                },
            );
        }
        if !maximum_absolute_adjustment.is_finite() {
            return Err(
                Float32ReportAdvisoryProposalConfigError::NonFiniteMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                },
            );
        }
        if maximum_absolute_adjustment < 0.0 {
            return Err(
                Float32ReportAdvisoryProposalConfigError::NegativeMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                },
            );
        }
        Ok(Self {
            maximum_records,
            maximum_explicit_missing,
            maximum_duplicates,
            maximum_out_of_order,
            maximum_adjusted,
            maximum_absolute_adjustment_bits: maximum_absolute_adjustment.to_bits(),
        })
    }
}

/// Exact checked evidence copied from the observation; it contains no inferred loss.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportAdvisoryEvidence {
    pub(crate) record_count: u64,
    pub(crate) explicit_missing: u64,
    pub(crate) gaps: u64,
    pub(crate) duplicates: u64,
    pub(crate) out_of_order: u64,
    pub(crate) adjusted: u64,
    pub(crate) largest_absolute_adjustment_bits: u64,
    pub(crate) largest_absolute_adjustment_sequence: Option<u64>,
    pub(crate) last_effective_timestamp_bits: Option<u64>,
    pub(crate) terminal_health: Float32ReportObservedTerminalHealth,
}

/// Review reasons are emitted in this declaration order, independent of record order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportAdvisoryReviewReason {
    TerminalHealth {
        observed: Float32ReportObservedTerminalHealth,
    },
    ExplicitMissingThreshold {
        observed: u64,
        maximum: u64,
    },
    DuplicateThreshold {
        observed: u64,
        maximum: u64,
    },
    OutOfOrderThreshold {
        observed: u64,
        maximum: u64,
    },
    AdjustedThreshold {
        observed: u64,
        maximum: u64,
    },
    AbsoluteAdjustmentThreshold {
        observed_bits: u64,
        maximum_bits: u64,
        sequence: u64,
    },
}

#[derive(Debug)]
pub(crate) enum Float32ReportAdvisoryProposal<O> {
    RecommendRetain {
        observation: O,
        evidence: Float32ReportAdvisoryEvidence,
    },
    RecommendReview {
        observation: O,
        evidence: Float32ReportAdvisoryEvidence,
        reasons: Vec<Float32ReportAdvisoryReviewReason>,
    },
}

/// Every refusal returns the exact caller observation without mutation.
#[derive(Debug)]
pub(crate) enum Float32ReportAdvisoryProposalError<O> {
    RecordBoundExceeded {
        maximum: usize,
        observed: usize,
        observation: O,
    },
    CounterOverflow {
        counter: Float32ReportAdvisoryCounter,
        observation: O,
    },
    NonFiniteAdjustment {
        sequence: u64,
        bits: u64,
        observation: O,
    },
    AllocationFailure {
        requested_reasons: usize,
        observation: O,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportAdvisoryCounter {
    RecordCount,
    ExplicitMissing,
    Gaps,
    Duplicates,
    OutOfOrder,
    Adjusted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportAdvisoryProposalOwner {
    config: Float32ReportAdvisoryProposalConfig,
}

impl Float32ReportAdvisoryProposalOwner {
    pub(crate) const fn new(config: Float32ReportAdvisoryProposalConfig) -> Self {
        Self { config }
    }

    pub(crate) fn propose<O: Float32ReportAdvisoryObservation>(
        self,
        observation: O,
    ) -> Result<Float32ReportAdvisoryProposal<O>, Float32ReportAdvisoryProposalError<O>> {
        let evidence = match collect_evidence(&observation, self.config.maximum_records) {
            Ok(evidence) => evidence,
            Err(CollectError::Bound { observed }) => {
                return Err(Float32ReportAdvisoryProposalError::RecordBoundExceeded {
                    maximum: self.config.maximum_records,
                    observed,
                    observation,
                });
            }
            Err(CollectError::Counter(counter)) => {
                return Err(Float32ReportAdvisoryProposalError::CounterOverflow {
                    counter,
                    observation,
                });
            }
            Err(CollectError::NonFinite { sequence, bits }) => {
                return Err(Float32ReportAdvisoryProposalError::NonFiniteAdjustment {
                    sequence,
                    bits,
                    observation,
                });
            }
        };
        let terminal_health = evidence.terminal_health;

        let mut reasons = Vec::new();
        if reasons.try_reserve_exact(6).is_err() {
            return Err(Float32ReportAdvisoryProposalError::AllocationFailure {
                requested_reasons: 6,
                observation,
            });
        }
        if terminal_health != Float32ReportObservedTerminalHealth::Complete {
            reasons.push(Float32ReportAdvisoryReviewReason::TerminalHealth {
                observed: terminal_health,
            });
        }
        push_threshold_reason(
            &mut reasons,
            evidence.explicit_missing,
            self.config.maximum_explicit_missing,
            |observed, maximum| Float32ReportAdvisoryReviewReason::ExplicitMissingThreshold {
                observed,
                maximum,
            },
        );
        push_threshold_reason(
            &mut reasons,
            evidence.duplicates,
            self.config.maximum_duplicates,
            |observed, maximum| Float32ReportAdvisoryReviewReason::DuplicateThreshold {
                observed,
                maximum,
            },
        );
        push_threshold_reason(
            &mut reasons,
            evidence.out_of_order,
            self.config.maximum_out_of_order,
            |observed, maximum| Float32ReportAdvisoryReviewReason::OutOfOrderThreshold {
                observed,
                maximum,
            },
        );
        push_threshold_reason(
            &mut reasons,
            evidence.adjusted,
            self.config.maximum_adjusted,
            |observed, maximum| Float32ReportAdvisoryReviewReason::AdjustedThreshold {
                observed,
                maximum,
            },
        );
        let observed_adjustment = f64::from_bits(evidence.largest_absolute_adjustment_bits);
        let maximum_adjustment = f64::from_bits(self.config.maximum_absolute_adjustment_bits);
        if observed_adjustment > maximum_adjustment {
            reasons.push(
                Float32ReportAdvisoryReviewReason::AbsoluteAdjustmentThreshold {
                    observed_bits: evidence.largest_absolute_adjustment_bits,
                    maximum_bits: self.config.maximum_absolute_adjustment_bits,
                    sequence: evidence
                        .largest_absolute_adjustment_sequence
                        .expect("positive maximum has a record"),
                },
            );
        }
        if reasons.is_empty() {
            Ok(Float32ReportAdvisoryProposal::RecommendRetain {
                observation,
                evidence,
            })
        } else {
            Ok(Float32ReportAdvisoryProposal::RecommendReview {
                observation,
                evidence,
                reasons,
            })
        }
    }
}

enum CollectError {
    Bound { observed: usize },
    Counter(Float32ReportAdvisoryCounter),
    NonFinite { sequence: u64, bits: u64 },
}

fn collect_evidence<O: Float32ReportAdvisoryObservation>(
    observation: &O,
    maximum_records: usize,
) -> Result<Float32ReportAdvisoryEvidence, CollectError> {
    let records = observation.records();
    if records.len() > maximum_records {
        return Err(CollectError::Bound {
            observed: records.len(),
        });
    }
    let mut evidence = Float32ReportAdvisoryEvidence {
        record_count: 0,
        explicit_missing: 0,
        gaps: 0,
        duplicates: 0,
        out_of_order: 0,
        adjusted: 0,
        largest_absolute_adjustment_bits: 0.0f64.to_bits(),
        largest_absolute_adjustment_sequence: None,
        last_effective_timestamp_bits: None,
        terminal_health: observation.terminal_health(),
    };
    for record in records {
        evidence.record_count =
            evidence
                .record_count
                .checked_add(1)
                .ok_or(CollectError::Counter(
                    Float32ReportAdvisoryCounter::RecordCount,
                ))?;
        match record.classification() {
            Float32ReportObservedSequenceClassification::First
            | Float32ReportObservedSequenceClassification::Contiguous => {}
            Float32ReportObservedSequenceClassification::Gap { missing } => {
                evidence.explicit_missing =
                    evidence
                        .explicit_missing
                        .checked_add(missing)
                        .ok_or(CollectError::Counter(
                            Float32ReportAdvisoryCounter::ExplicitMissing,
                        ))?;
                evidence.gaps = evidence
                    .gaps
                    .checked_add(1)
                    .ok_or(CollectError::Counter(Float32ReportAdvisoryCounter::Gaps))?;
            }
            Float32ReportObservedSequenceClassification::Duplicate => {
                evidence.duplicates =
                    evidence
                        .duplicates
                        .checked_add(1)
                        .ok_or(CollectError::Counter(
                            Float32ReportAdvisoryCounter::Duplicates,
                        ))?;
            }
            Float32ReportObservedSequenceClassification::OutOfOrder { .. } => {
                evidence.out_of_order =
                    evidence
                        .out_of_order
                        .checked_add(1)
                        .ok_or(CollectError::Counter(
                            Float32ReportAdvisoryCounter::OutOfOrder,
                        ))?;
            }
        }
        if record.disposition() == Float32ReportObservedDisposition::RetainedChanged {
            evidence.adjusted = evidence
                .adjusted
                .checked_add(1)
                .ok_or(CollectError::Counter(
                    Float32ReportAdvisoryCounter::Adjusted,
                ))?;
        }
        let adjustment = f64::from_bits(record.adjustment_bits());
        if !adjustment.is_finite() {
            return Err(CollectError::NonFinite {
                sequence: record.sequence(),
                bits: record.adjustment_bits(),
            });
        }
        let absolute = adjustment.abs();
        if absolute > f64::from_bits(evidence.largest_absolute_adjustment_bits) {
            evidence.largest_absolute_adjustment_bits = absolute.to_bits();
            evidence.largest_absolute_adjustment_sequence = Some(record.sequence());
        }
        evidence.last_effective_timestamp_bits = Some(record.effective_timestamp_bits());
    }
    Ok(evidence)
}

fn push_threshold_reason(
    reasons: &mut Vec<Float32ReportAdvisoryReviewReason>,
    observed: u64,
    maximum: u64,
    make: fn(u64, u64) -> Float32ReportAdvisoryReviewReason,
) {
    if observed > maximum {
        reasons.push(make(observed, maximum));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct StubRecord {
        sequence: u64,
        classification: Float32ReportObservedSequenceClassification,
        disposition: Float32ReportObservedDisposition,
        adjustment_bits: u64,
        effective_bits: u64,
    }
    impl Float32ReportAdvisoryObservedRecord for StubRecord {
        fn sequence(&self) -> u64 {
            self.sequence
        }
        fn classification(&self) -> Float32ReportObservedSequenceClassification {
            self.classification
        }
        fn disposition(&self) -> Float32ReportObservedDisposition {
            self.disposition
        }
        fn adjustment_bits(&self) -> u64 {
            self.adjustment_bits
        }
        fn effective_timestamp_bits(&self) -> u64 {
            self.effective_bits
        }
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct StubObservation {
        marker: Vec<u8>,
        records: Vec<StubRecord>,
        health: Float32ReportObservedTerminalHealth,
    }
    impl Float32ReportAdvisoryObservation for StubObservation {
        type Record = StubRecord;
        fn records(&self) -> &[Self::Record] {
            &self.records
        }
        fn terminal_health(&self) -> Float32ReportObservedTerminalHealth {
            self.health
        }
    }
    fn record(
        sequence: u64,
        classification: Float32ReportObservedSequenceClassification,
        disposition: Float32ReportObservedDisposition,
        adjustment: f64,
    ) -> StubRecord {
        StubRecord {
            sequence,
            classification,
            disposition,
            adjustment_bits: adjustment.to_bits(),
            effective_bits: (100.0 + sequence as f64).to_bits(),
        }
    }
    fn config(
        maximum_records: usize,
        thresholds: u64,
        adjustment: f64,
    ) -> Float32ReportAdvisoryProposalConfig {
        Float32ReportAdvisoryProposalConfig::new(
            maximum_records,
            thresholds,
            thresholds,
            thresholds,
            thresholds,
            adjustment,
        )
        .unwrap()
    }

    #[test]
    fn zero_and_extreme_configuration_boundaries_are_exact() {
        assert_eq!(
            Float32ReportAdvisoryProposalConfig::new(0, 0, 0, 0, 0, 0.0),
            Err(Float32ReportAdvisoryProposalConfigError::ZeroMaximumRecords)
        );
        assert!(Float32ReportAdvisoryProposalConfig::new(
            usize::try_from(u64::MAX).unwrap_or(usize::MAX),
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            f64::MAX
        )
        .is_ok());
        assert!(matches!(
            Float32ReportAdvisoryProposalConfig::new(1, 0, 0, 0, 0, f64::INFINITY),
            Err(
                Float32ReportAdvisoryProposalConfigError::NonFiniteMaximumAbsoluteAdjustment { .. }
            )
        ));
    }

    #[test]
    fn gaps_duplicates_out_of_order_adjustments_and_reason_order_are_exact() {
        let observation = StubObservation {
            marker: vec![7, 8, 9],
            health: Float32ReportObservedTerminalHealth::Deadline,
            records: vec![
                record(
                    4,
                    Float32ReportObservedSequenceClassification::First,
                    Float32ReportObservedDisposition::RetainedUnchanged,
                    0.0,
                ),
                record(
                    8,
                    Float32ReportObservedSequenceClassification::Gap { missing: 3 },
                    Float32ReportObservedDisposition::RetainedChanged,
                    -2.0,
                ),
                record(
                    8,
                    Float32ReportObservedSequenceClassification::Duplicate,
                    Float32ReportObservedDisposition::RetainedChanged,
                    1.0,
                ),
                record(
                    6,
                    Float32ReportObservedSequenceClassification::OutOfOrder { distance: 2 },
                    Float32ReportObservedDisposition::RetainedUnchanged,
                    0.0,
                ),
            ],
        };
        let original = observation.clone();
        let result = Float32ReportAdvisoryProposalOwner::new(config(4, 0, 0.5))
            .propose(observation)
            .unwrap();
        let Float32ReportAdvisoryProposal::RecommendReview {
            observation,
            evidence,
            reasons,
        } = result
        else {
            panic!()
        };
        assert_eq!(observation, original);
        assert_eq!(
            (
                evidence.record_count,
                evidence.explicit_missing,
                evidence.gaps,
                evidence.duplicates,
                evidence.out_of_order,
                evidence.adjusted
            ),
            (4, 3, 1, 1, 1, 2)
        );
        assert_eq!(
            reasons,
            vec![
                Float32ReportAdvisoryReviewReason::TerminalHealth {
                    observed: Float32ReportObservedTerminalHealth::Deadline
                },
                Float32ReportAdvisoryReviewReason::ExplicitMissingThreshold {
                    observed: 3,
                    maximum: 0
                },
                Float32ReportAdvisoryReviewReason::DuplicateThreshold {
                    observed: 1,
                    maximum: 0
                },
                Float32ReportAdvisoryReviewReason::OutOfOrderThreshold {
                    observed: 1,
                    maximum: 0
                },
                Float32ReportAdvisoryReviewReason::AdjustedThreshold {
                    observed: 2,
                    maximum: 0
                },
                Float32ReportAdvisoryReviewReason::AbsoluteAdjustmentThreshold {
                    observed_bits: 2.0f64.to_bits(),
                    maximum_bits: 0.5f64.to_bits(),
                    sequence: 8
                },
            ]
        );
    }

    #[test]
    fn exact_thresholds_retain_and_return_the_same_observation_allocation() {
        let marker = vec![1, 2, 3, 4];
        let pointer = marker.as_ptr();
        let observation = StubObservation {
            marker,
            health: Float32ReportObservedTerminalHealth::Complete,
            records: vec![record(
                0,
                Float32ReportObservedSequenceClassification::First,
                Float32ReportObservedDisposition::RetainedChanged,
                1.0,
            )],
        };
        let result = Float32ReportAdvisoryProposalOwner::new(config(1, 1, 1.0))
            .propose(observation)
            .unwrap();
        let Float32ReportAdvisoryProposal::RecommendRetain {
            observation,
            evidence,
        } = result
        else {
            panic!()
        };
        assert_eq!(observation.marker.as_ptr(), pointer);
        assert_eq!(evidence.adjusted, 1);
    }

    #[test]
    fn bound_and_nonfinite_failures_preserve_observation() {
        let observation = StubObservation {
            marker: vec![9],
            health: Float32ReportObservedTerminalHealth::Complete,
            records: vec![record(
                1,
                Float32ReportObservedSequenceClassification::First,
                Float32ReportObservedDisposition::RetainedUnchanged,
                f64::NAN,
            )],
        };
        let original = observation.clone();
        assert!(
            matches!(Float32ReportAdvisoryProposalOwner::new(config(1, 0, 0.0)).propose(observation),
            Err(Float32ReportAdvisoryProposalError::NonFiniteAdjustment { observation, .. }) if observation == original)
        );

        let over_bound = StubObservation {
            marker: vec![5],
            health: Float32ReportObservedTerminalHealth::Complete,
            records: vec![
                record(
                    0,
                    Float32ReportObservedSequenceClassification::First,
                    Float32ReportObservedDisposition::RetainedUnchanged,
                    0.0,
                ),
                record(
                    1,
                    Float32ReportObservedSequenceClassification::Contiguous,
                    Float32ReportObservedDisposition::RetainedUnchanged,
                    0.0,
                ),
            ],
        };
        let original = over_bound.clone();
        assert!(
            matches!(Float32ReportAdvisoryProposalOwner::new(config(1, 0, 0.0)).propose(over_bound),
            Err(Float32ReportAdvisoryProposalError::RecordBoundExceeded { maximum: 1, observed: 2, observation }) if observation == original)
        );

        let overflow = StubObservation {
            marker: vec![6],
            health: Float32ReportObservedTerminalHealth::Complete,
            records: vec![
                record(
                    2,
                    Float32ReportObservedSequenceClassification::Gap { missing: u64::MAX },
                    Float32ReportObservedDisposition::RetainedUnchanged,
                    0.0,
                ),
                record(
                    4,
                    Float32ReportObservedSequenceClassification::Gap { missing: 1 },
                    Float32ReportObservedDisposition::RetainedUnchanged,
                    0.0,
                ),
            ],
        };
        let original = overflow.clone();
        assert!(
            matches!(Float32ReportAdvisoryProposalOwner::new(config(2, u64::MAX, f64::MAX)).propose(overflow),
            Err(Float32ReportAdvisoryProposalError::CounterOverflow { counter: Float32ReportAdvisoryCounter::ExplicitMissing, observation }) if observation == original)
        );
    }
}
