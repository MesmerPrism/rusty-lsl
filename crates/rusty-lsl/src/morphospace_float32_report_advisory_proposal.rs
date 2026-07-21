// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic, effect-free advice over the exact P36 Float32 report observation.
//!
//! This owner only reads the sibling observation's ordered records, exact post-processing
//! facts, and terminal exact loss-health snapshot. It returns that observation intact and
//! owns no acceptance, route, lease, revision, authorization, application, or audit action.

use crate::exact_sequence_loss_health::ExactSequenceLossHealthSnapshot;
use crate::morphospace_float32_report_observation::MorphospaceFloat32ReportObservation;
use crate::requested_timestamp_post_processing::{
    RequestedEffectiveTimestampSource, RequestedTimestampPostProcessingDisposition,
};

/// Explicit caller bounds for an advisory classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisoryProposalConfig {
    maximum_records: usize,
    maximum_explicit_missing_sequences: u64,
    maximum_duplicates: u64,
    maximum_out_of_order: u64,
    maximum_retained_changed: u64,
    maximum_absolute_adjustment_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisoryProposalConfigError {
    ZeroMaximumRecords,
    MaximumRecordsUnrepresentable { requested: usize },
    NonFiniteMaximumAbsoluteAdjustment { bits: u64 },
    NegativeMaximumAbsoluteAdjustment { bits: u64 },
}

impl MorphospaceFloat32ReportAdvisoryProposalConfig {
    pub(crate) fn new(
        maximum_records: usize,
        maximum_explicit_missing_sequences: u64,
        maximum_duplicates: u64,
        maximum_out_of_order: u64,
        maximum_retained_changed: u64,
        maximum_absolute_adjustment: f64,
    ) -> Result<Self, MorphospaceFloat32ReportAdvisoryProposalConfigError> {
        if maximum_records == 0 {
            return Err(MorphospaceFloat32ReportAdvisoryProposalConfigError::ZeroMaximumRecords);
        }
        u64::try_from(maximum_records).map_err(|_| {
            MorphospaceFloat32ReportAdvisoryProposalConfigError::MaximumRecordsUnrepresentable {
                requested: maximum_records,
            }
        })?;
        if !maximum_absolute_adjustment.is_finite() {
            return Err(MorphospaceFloat32ReportAdvisoryProposalConfigError::
                NonFiniteMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                });
        }
        if maximum_absolute_adjustment < 0.0 {
            return Err(MorphospaceFloat32ReportAdvisoryProposalConfigError::
                NegativeMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                });
        }
        Ok(Self {
            maximum_records,
            maximum_explicit_missing_sequences,
            maximum_duplicates,
            maximum_out_of_order,
            maximum_retained_changed,
            maximum_absolute_adjustment_bits: maximum_absolute_adjustment.to_bits(),
        })
    }
}

/// Addressable evidence for the first ordered record having the largest magnitude.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportLargestAdjustmentEvidence {
    pub(crate) record_index: u64,
    pub(crate) sequence: u64,
    pub(crate) signed_adjustment_bits: u64,
    pub(crate) absolute_adjustment_bits: u64,
    pub(crate) effective_timestamp_value_bits: u64,
    pub(crate) effective_timestamp_source: RequestedEffectiveTimestampSource,
    pub(crate) disposition: RequestedTimestampPostProcessingDisposition,
}

/// Exact borrowed facts copied without reclassifying the sibling observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisoryEvidence {
    pub(crate) terminal_health: ExactSequenceLossHealthSnapshot,
    pub(crate) largest_adjustment: Option<MorphospaceFloat32ReportLargestAdjustmentEvidence>,
}

/// Review reasons always appear in this declaration order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisoryReviewReason {
    ExplicitMissingSequences {
        observed: u64,
        maximum: u64,
    },
    Duplicates {
        observed: u64,
        maximum: u64,
    },
    OutOfOrder {
        observed: u64,
        maximum: u64,
    },
    RetainedChanged {
        observed: u64,
        maximum: u64,
    },
    AbsoluteAdjustment {
        record_index: u64,
        sequence: u64,
        signed_adjustment_bits: u64,
        observed_absolute_bits: u64,
        maximum_absolute_bits: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisoryProposal {
    RecommendRetain {
        observation: MorphospaceFloat32ReportObservation,
        evidence: MorphospaceFloat32ReportAdvisoryEvidence,
    },
    RecommendReview {
        observation: MorphospaceFloat32ReportObservation,
        evidence: MorphospaceFloat32ReportAdvisoryEvidence,
        reasons: Vec<MorphospaceFloat32ReportAdvisoryReviewReason>,
    },
}

/// Every refusal returns the sole observation, including its original sample allocations.
#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisoryProposalError {
    RecordLimit {
        limit: usize,
        actual: usize,
        observation: MorphospaceFloat32ReportObservation,
    },
    RecordCountUnrepresentable {
        actual: usize,
        observation: MorphospaceFloat32ReportObservation,
    },
    TerminalHealthExtentMismatch {
        records: u64,
        health_observations: u64,
        observation: MorphospaceFloat32ReportObservation,
    },
    NonFiniteAdjustment {
        record_index: u64,
        sequence: u64,
        bits: u64,
        observation: MorphospaceFloat32ReportObservation,
    },
    Allocation {
        requested_reasons: usize,
        observation: MorphospaceFloat32ReportObservation,
    },
}

impl MorphospaceFloat32ReportAdvisoryProposalError {
    pub(crate) fn into_observation(self) -> MorphospaceFloat32ReportObservation {
        match self {
            Self::RecordLimit { observation, .. }
            | Self::RecordCountUnrepresentable { observation, .. }
            | Self::TerminalHealthExtentMismatch { observation, .. }
            | Self::NonFiniteAdjustment { observation, .. }
            | Self::Allocation { observation, .. } => observation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisoryProposalOwner {
    config: MorphospaceFloat32ReportAdvisoryProposalConfig,
}

impl MorphospaceFloat32ReportAdvisoryProposalOwner {
    pub(crate) const fn new(config: MorphospaceFloat32ReportAdvisoryProposalConfig) -> Self {
        Self { config }
    }

    pub(crate) fn propose(
        &self,
        observation: MorphospaceFloat32ReportObservation,
    ) -> Result<
        MorphospaceFloat32ReportAdvisoryProposal,
        MorphospaceFloat32ReportAdvisoryProposalError,
    > {
        self.propose_with(observation, |reasons, requested| {
            reasons.try_reserve_exact(requested).map_err(|_| ())
        })
    }

    fn propose_with<R>(
        &self,
        observation: MorphospaceFloat32ReportObservation,
        reserve: R,
    ) -> Result<
        MorphospaceFloat32ReportAdvisoryProposal,
        MorphospaceFloat32ReportAdvisoryProposalError,
    >
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportAdvisoryReviewReason>, usize) -> Result<(), ()>,
    {
        let evidence = match collect_evidence(&observation, self.config.maximum_records) {
            Ok(evidence) => evidence,
            Err(CollectError::RecordLimit { actual }) => {
                return Err(MorphospaceFloat32ReportAdvisoryProposalError::RecordLimit {
                    limit: self.config.maximum_records,
                    actual,
                    observation,
                });
            }
            Err(CollectError::RecordCountUnrepresentable { actual }) => {
                return Err(
                    MorphospaceFloat32ReportAdvisoryProposalError::RecordCountUnrepresentable {
                        actual,
                        observation,
                    },
                );
            }
            Err(CollectError::TerminalHealthExtentMismatch {
                records,
                health_observations,
            }) => {
                return Err(
                    MorphospaceFloat32ReportAdvisoryProposalError::TerminalHealthExtentMismatch {
                        records,
                        health_observations,
                        observation,
                    },
                );
            }
            Err(CollectError::NonFiniteAdjustment {
                record_index,
                sequence,
                bits,
            }) => {
                return Err(
                    MorphospaceFloat32ReportAdvisoryProposalError::NonFiniteAdjustment {
                        record_index,
                        sequence,
                        bits,
                        observation,
                    },
                );
            }
        };

        const MAXIMUM_REASON_COUNT: usize = 5;
        let mut reasons = Vec::new();
        if reserve(&mut reasons, MAXIMUM_REASON_COUNT).is_err() {
            return Err(MorphospaceFloat32ReportAdvisoryProposalError::Allocation {
                requested_reasons: MAXIMUM_REASON_COUNT,
                observation,
            });
        }
        let health = evidence.terminal_health;
        push_threshold(
            &mut reasons,
            health.explicit_missing_sequence_count(),
            self.config.maximum_explicit_missing_sequences,
            |observed, maximum| {
                MorphospaceFloat32ReportAdvisoryReviewReason::ExplicitMissingSequences {
                    observed,
                    maximum,
                }
            },
        );
        push_threshold(
            &mut reasons,
            health.duplicate_count(),
            self.config.maximum_duplicates,
            |observed, maximum| MorphospaceFloat32ReportAdvisoryReviewReason::Duplicates {
                observed,
                maximum,
            },
        );
        push_threshold(
            &mut reasons,
            health.out_of_order_count(),
            self.config.maximum_out_of_order,
            |observed, maximum| MorphospaceFloat32ReportAdvisoryReviewReason::OutOfOrder {
                observed,
                maximum,
            },
        );
        push_threshold(
            &mut reasons,
            health.retained_changed_count(),
            self.config.maximum_retained_changed,
            |observed, maximum| MorphospaceFloat32ReportAdvisoryReviewReason::RetainedChanged {
                observed,
                maximum,
            },
        );
        if let Some(largest) = evidence.largest_adjustment {
            if f64::from_bits(largest.absolute_adjustment_bits)
                > f64::from_bits(self.config.maximum_absolute_adjustment_bits)
            {
                reasons.push(
                    MorphospaceFloat32ReportAdvisoryReviewReason::AbsoluteAdjustment {
                        record_index: largest.record_index,
                        sequence: largest.sequence,
                        signed_adjustment_bits: largest.signed_adjustment_bits,
                        observed_absolute_bits: largest.absolute_adjustment_bits,
                        maximum_absolute_bits: self.config.maximum_absolute_adjustment_bits,
                    },
                );
            }
        }

        if reasons.is_empty() {
            Ok(MorphospaceFloat32ReportAdvisoryProposal::RecommendRetain {
                observation,
                evidence,
            })
        } else {
            Ok(MorphospaceFloat32ReportAdvisoryProposal::RecommendReview {
                observation,
                evidence,
                reasons,
            })
        }
    }
}

enum CollectError {
    RecordLimit {
        actual: usize,
    },
    RecordCountUnrepresentable {
        actual: usize,
    },
    TerminalHealthExtentMismatch {
        records: u64,
        health_observations: u64,
    },
    NonFiniteAdjustment {
        record_index: u64,
        sequence: u64,
        bits: u64,
    },
}

fn collect_evidence(
    observation: &MorphospaceFloat32ReportObservation,
    maximum_records: usize,
) -> Result<MorphospaceFloat32ReportAdvisoryEvidence, CollectError> {
    let records = observation.records();
    if records.len() > maximum_records {
        return Err(CollectError::RecordLimit {
            actual: records.len(),
        });
    }
    let record_count =
        u64::try_from(records.len()).map_err(|_| CollectError::RecordCountUnrepresentable {
            actual: records.len(),
        })?;
    let terminal_health = observation.terminal_health();
    if record_count != terminal_health.observation_count() {
        return Err(CollectError::TerminalHealthExtentMismatch {
            records: record_count,
            health_observations: terminal_health.observation_count(),
        });
    }

    let mut largest_adjustment = None;
    for record in records {
        let facts = record.processed().facts();
        let adjustment = facts.adjustment();
        if !adjustment.is_finite() {
            return Err(CollectError::NonFiniteAdjustment {
                record_index: record.index(),
                sequence: record.sequence(),
                bits: adjustment.to_bits(),
            });
        }
        let absolute = adjustment.abs();
        let replace = largest_adjustment
            .map(
                |current: MorphospaceFloat32ReportLargestAdjustmentEvidence| {
                    absolute > f64::from_bits(current.absolute_adjustment_bits)
                },
            )
            .unwrap_or(true);
        // Strictly greater replacement makes the earliest ordered record win exact ties.
        if replace {
            let effective = record.effective_timestamp();
            largest_adjustment = Some(MorphospaceFloat32ReportLargestAdjustmentEvidence {
                record_index: record.index(),
                sequence: record.sequence(),
                signed_adjustment_bits: adjustment.to_bits(),
                absolute_adjustment_bits: absolute.to_bits(),
                effective_timestamp_value_bits: effective.value().to_bits(),
                effective_timestamp_source: effective.source(),
                disposition: record.disposition(),
            });
        }
    }
    Ok(MorphospaceFloat32ReportAdvisoryEvidence {
        terminal_health,
        largest_adjustment,
    })
}

fn push_threshold(
    reasons: &mut Vec<MorphospaceFloat32ReportAdvisoryReviewReason>,
    observed: u64,
    maximum: u64,
    make: fn(u64, u64) -> MorphospaceFloat32ReportAdvisoryReviewReason,
) {
    if observed > maximum {
        reasons.push(make(observed, maximum));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exact_sequence_loss_health::{ExactPostProcessingFact, ExactSequenceClassification};
    use crate::morphospace_float32_report_observation::{
        tests::outcome_with, MorphospaceFloat32ReportObservationOwner,
    };
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

    fn build_observation(
        sequences: Vec<u64>,
        timestamps: Vec<f64>,
    ) -> MorphospaceFloat32ReportObservation {
        let maximum = timestamps.len();
        let records = timestamps
            .into_iter()
            .enumerate()
            .map(|(index, timestamp)| sample(timestamp, index as f32, index == 0))
            .collect();
        MorphospaceFloat32ReportObservationOwner::new(maximum)
            .unwrap()
            .observe(outcome_with(
                sequences,
                records,
                RequestedTimestampPostProcessing::Monotonic(
                    RequestedTimestampPostProcessingConfig::new(8, 1.0, f64::MAX).unwrap(),
                ),
            ))
            .unwrap()
    }

    fn config(
        maximum_records: usize,
        threshold: u64,
        adjustment: f64,
    ) -> MorphospaceFloat32ReportAdvisoryProposalConfig {
        MorphospaceFloat32ReportAdvisoryProposalConfig::new(
            maximum_records,
            threshold,
            threshold,
            threshold,
            threshold,
            adjustment,
        )
        .unwrap()
    }

    fn outcome_parts(
        proposal: MorphospaceFloat32ReportAdvisoryProposal,
    ) -> (
        MorphospaceFloat32ReportObservation,
        MorphospaceFloat32ReportAdvisoryEvidence,
        Vec<MorphospaceFloat32ReportAdvisoryReviewReason>,
    ) {
        match proposal {
            MorphospaceFloat32ReportAdvisoryProposal::RecommendRetain {
                observation,
                evidence,
            } => (observation, evidence, Vec::new()),
            MorphospaceFloat32ReportAdvisoryProposal::RecommendReview {
                observation,
                evidence,
                reasons,
            } => (observation, evidence, reasons),
        }
    }

    #[test]
    fn config_zero_extreme_and_float_edges_are_exact() {
        assert_eq!(
            MorphospaceFloat32ReportAdvisoryProposalConfig::new(0, 0, 0, 0, 0, 0.0),
            Err(MorphospaceFloat32ReportAdvisoryProposalConfigError::ZeroMaximumRecords)
        );
        assert!(MorphospaceFloat32ReportAdvisoryProposalConfig::new(
            usize::try_from(u64::MAX).unwrap_or(usize::MAX),
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            f64::MAX,
        )
        .is_ok());
        assert!(matches!(
            MorphospaceFloat32ReportAdvisoryProposalConfig::new(1, 0, 0, 0, 0, f64::INFINITY),
            Err(MorphospaceFloat32ReportAdvisoryProposalConfigError::
                NonFiniteMaximumAbsoluteAdjustment { .. })
        ));
        assert!(matches!(
            MorphospaceFloat32ReportAdvisoryProposalConfig::new(1, 0, 0, 0, 0, -f64::MIN_POSITIVE),
            Err(MorphospaceFloat32ReportAdvisoryProposalConfigError::
                NegativeMaximumAbsoluteAdjustment { .. })
        ));
    }

    #[test]
    fn actual_interface_covers_every_classification_snapshot_and_reason_order() {
        let observation = build_observation(vec![4, 8], vec![10.0, 9.0]);
        let pointers: Vec<_> = observation
            .records()
            .iter()
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect();
        assert_eq!(
            observation
                .records()
                .iter()
                .map(|record| record.classification())
                .collect::<Vec<_>>(),
            vec![
                ExactSequenceClassification::First,
                ExactSequenceClassification::Gap {
                    missing_sequence_count: 3
                },
            ]
        );
        let expected_health = observation.terminal_health();
        let (observation, evidence, reasons) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, 0, 0.5))
                .propose(observation)
                .unwrap(),
        );
        assert_eq!(evidence.terminal_health, expected_health);
        assert_eq!(
            (
                expected_health.observation_count(),
                expected_health.first_count(),
                expected_health.contiguous_count(),
                expected_health.gap_count(),
                expected_health.explicit_missing_sequence_count(),
                expected_health.duplicate_count(),
                expected_health.out_of_order_count()
            ),
            (2, 1, 0, 1, 3, 0, 0)
        );
        assert_eq!(
            (
                expected_health.retained_unchanged_count(),
                expected_health.retained_changed_count()
            ),
            (1, 1)
        );
        assert_eq!(expected_health.high_water_sequence(), Some(8));
        assert_eq!(
            expected_health.last_classification(),
            Some(ExactSequenceClassification::Gap {
                missing_sequence_count: 3
            })
        );
        assert_eq!(
            expected_health.last_post_processing_fact(),
            Some(ExactPostProcessingFact::RetainedChanged)
        );
        assert!(matches!(
            reasons.as_slice(),
            [
                MorphospaceFloat32ReportAdvisoryReviewReason::ExplicitMissingSequences { .. },
                MorphospaceFloat32ReportAdvisoryReviewReason::RetainedChanged { .. },
                MorphospaceFloat32ReportAdvisoryReviewReason::AbsoluteAdjustment { .. },
            ]
        ));
        assert_eq!(
            observation
                .records()
                .iter()
                .map(|record| record.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>(),
            pointers
        );

        let (_, _, reasons) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, 0, 0.5))
                .propose(build_observation(vec![8, 8], vec![10.0, 9.0]))
                .unwrap(),
        );
        assert!(matches!(
            reasons.as_slice(),
            [
                MorphospaceFloat32ReportAdvisoryReviewReason::Duplicates { .. },
                MorphospaceFloat32ReportAdvisoryReviewReason::RetainedChanged { .. },
                MorphospaceFloat32ReportAdvisoryReviewReason::AbsoluteAdjustment { .. },
            ]
        ));

        let contiguous = build_observation(vec![4, 5], vec![10.0, 11.0]);
        assert_eq!(
            contiguous.records()[1].classification(),
            ExactSequenceClassification::Contiguous
        );

        let (_, _, reasons) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, 0, 0.5))
                .propose(build_observation(vec![8, 6], vec![10.0, 9.0]))
                .unwrap(),
        );
        assert!(matches!(
            reasons.as_slice(),
            [
                MorphospaceFloat32ReportAdvisoryReviewReason::OutOfOrder { .. },
                MorphospaceFloat32ReportAdvisoryReviewReason::RetainedChanged { .. },
                MorphospaceFloat32ReportAdvisoryReviewReason::AbsoluteAdjustment { .. },
            ]
        ));
    }

    #[test]
    fn largest_adjustment_uses_actual_effective_fact_and_first_equal_magnitude_record() {
        let observation = build_observation(vec![7, 7], vec![10.0, 9.0]);
        let expected = observation.records()[1].effective_timestamp();
        let (observation, evidence, _) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, u64::MAX, f64::MAX))
                .propose(observation)
                .unwrap(),
        );
        let largest = evidence.largest_adjustment.unwrap();
        assert_eq!((largest.record_index, largest.sequence), (1, 7));
        assert!(f64::from_bits(largest.signed_adjustment_bits) > 0.0);
        assert_eq!(
            largest.effective_timestamp_value_bits,
            expected.value().to_bits()
        );
        assert_eq!(largest.effective_timestamp_source, expected.source());
        assert_eq!(largest.disposition, observation.records()[1].disposition());
        assert_eq!(
            expected.source(),
            RequestedEffectiveTimestampSource::ProjectPostProcessed
        );
        assert_eq!(
            observation.records()[0].sequence(),
            observation.records()[1].sequence()
        );

        let (_, evidence, _) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, u64::MAX, f64::MAX))
                .propose(build_observation(vec![7, 8], vec![10.0, 11.0]))
                .unwrap(),
        );
        assert_eq!(evidence.largest_adjustment.unwrap().record_index, 0);
    }

    #[test]
    fn signed_zero_positive_and_exact_adjustment_threshold_edges_are_deterministic() {
        let observation = build_observation(vec![0], vec![1.0]);
        let (_, evidence, reasons) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(1, 1, 0.0))
                .propose(observation)
                .unwrap(),
        );
        assert!(reasons.is_empty());
        assert_eq!(
            evidence.largest_adjustment.unwrap().signed_adjustment_bits,
            0.0f64.to_bits()
        );

        let observation = build_observation(vec![0, 1], vec![4.0, 2.0]);
        let adjustment = observation.records()[1].processed().facts().adjustment();
        let (_, _, reasons) = outcome_parts(
            MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, 2, adjustment.abs()))
                .propose(observation)
                .unwrap(),
        );
        assert!(reasons.is_empty());
        assert!(adjustment > 0.0);
    }

    #[test]
    fn record_bound_and_reason_allocation_refusals_preserve_actual_allocations() {
        let observation = build_observation(vec![1, 3], vec![3.0, 1.0]);
        let pointers: Vec<_> = observation
            .records()
            .iter()
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect();
        let returned = MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(1, 0, 0.0))
            .propose(observation)
            .unwrap_err()
            .into_observation();
        assert_eq!(
            returned
                .records()
                .iter()
                .map(|record| record.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>(),
            pointers
        );

        let observation = build_observation(vec![1, 3], vec![3.0, 1.0]);
        let pointers: Vec<_> = observation
            .records()
            .iter()
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect();
        let returned = MorphospaceFloat32ReportAdvisoryProposalOwner::new(config(2, 0, 0.0))
            .propose_with(observation, |_, _| Err(()))
            .unwrap_err()
            .into_observation();
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
    fn authority_denials_remain_explicit() {
        let source = include_str!("morphospace_float32_report_advisory_proposal.rs");
        for denied in [
            "acceptance",
            "route",
            "lease",
            "revision",
            "authorization",
            "application",
            "audit",
        ] {
            assert!(source.contains(denied));
        }
        assert!(
            source.contains("no behavioral, numerical, or protocol equivalence with liblsl")
                || include_str!("../../../docs/p5-float32-report-advisory-observation-proposal.md")
                    .contains("does not claim")
        );
    }
}
