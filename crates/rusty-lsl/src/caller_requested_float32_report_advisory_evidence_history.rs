// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded history of complete actual P41 caller-requested advisory evidence.
//!
//! This crate-private, default-inert owner retains caller-supplied values in
//! insertion order. It does not infer loss or continuity, apply evidence, or
//! grant Manifold, session, stream, transport, control, or application
//! authority, and it makes no claim of liblsl equivalence.

use crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ReportAdvisoryEvidenceHistoryConfigError {
    ZeroMaximumValues,
    MaximumValuesUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportAdvisoryEvidenceHistoryTotals {
    value_count: u64,
    ordered_evidence_count: u64,
}

impl CallerRequestedFloat32ReportAdvisoryEvidenceHistoryTotals {
    pub(crate) const fn value_count(&self) -> u64 {
        self.value_count
    }

    pub(crate) const fn ordered_evidence_count(&self) -> u64 {
        self.ordered_evidence_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportAdvisoryEvidenceHistory {
    maximum_values: usize,
    maximum_values_u64: u64,
    values: Vec<CallerRequestedFloat32ReportAdvisoryEvidence>,
    totals: CallerRequestedFloat32ReportAdvisoryEvidenceHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ReportAdvisoryEvidenceHistoryAppendError {
    CollectionLengthOverflow {
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    },
    HistoryLimit {
        limit: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    },
    CountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    },
    OrderedEvidenceCountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    },
    CounterOverflow {
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    },
    Allocation {
        requested_values: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    },
}

impl CallerRequestedFloat32ReportAdvisoryEvidenceHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        CallerRequestedFloat32ReportAdvisoryEvidence,
    ) {
        use CallerRequestedFloat32ReportAdvisoryEvidenceHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, candidate }
            | HistoryLimit {
                history, candidate, ..
            }
            | CountUnrepresentable {
                history, candidate, ..
            }
            | OrderedEvidenceCountUnrepresentable {
                history, candidate, ..
            }
            | CounterOverflow { history, candidate }
            | Allocation {
                history, candidate, ..
            } => (history, candidate),
        }
    }
}

impl CallerRequestedFloat32ReportAdvisoryEvidenceHistory {
    pub(crate) fn new(
        maximum_values: usize,
    ) -> Result<Self, CallerRequestedFloat32ReportAdvisoryEvidenceHistoryConfigError> {
        if maximum_values == 0 {
            return Err(
                CallerRequestedFloat32ReportAdvisoryEvidenceHistoryConfigError::ZeroMaximumValues,
            );
        }
        let maximum_values_u64 = u64::try_from(maximum_values).map_err(|_| {
            CallerRequestedFloat32ReportAdvisoryEvidenceHistoryConfigError::MaximumValuesUnrepresentable {
                requested: maximum_values,
            }
        })?;
        Ok(Self {
            maximum_values,
            maximum_values_u64,
            values: Vec::new(),
            totals: Default::default(),
        })
    }

    pub(crate) const fn maximum_values(&self) -> usize {
        self.maximum_values
    }

    pub(crate) fn values(&self) -> &[CallerRequestedFloat32ReportAdvisoryEvidence] {
        &self.values
    }

    pub(crate) const fn totals(&self) -> CallerRequestedFloat32ReportAdvisoryEvidenceHistoryTotals {
        self.totals
    }

    pub(crate) fn into_values(self) -> Vec<CallerRequestedFloat32ReportAdvisoryEvidence> {
        self.values
    }

    pub(crate) fn append(
        self,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
    ) -> Result<Self, CallerRequestedFloat32ReportAdvisoryEvidenceHistoryAppendError> {
        self.append_with(
            candidate,
            |values, requested| values.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        candidate: CallerRequestedFloat32ReportAdvisoryEvidence,
        reserve: R,
        mut to_u64: C,
        mut add_u64: U,
        mut add_usize: Z,
    ) -> Result<Self, CallerRequestedFloat32ReportAdvisoryEvidenceHistoryAppendError>
    where
        R: FnOnce(&mut Vec<CallerRequestedFloat32ReportAdvisoryEvidence>, usize) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        U: FnMut(u64, u64) -> Result<u64, ()>,
        Z: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32ReportAdvisoryEvidenceHistoryAppendError::*;
        let next_len = match add_usize(self.values.len(), 1) {
            Ok(value) => value,
            Err(()) => {
                return Err(CollectionLengthOverflow {
                    history: self,
                    candidate,
                })
            }
        };
        if next_len > self.maximum_values {
            return Err(HistoryLimit {
                limit: self.maximum_values,
                history: self,
                candidate,
            });
        }
        let next_count = match to_u64(next_len) {
            Ok(value) => value,
            Err(()) => {
                return Err(CountUnrepresentable {
                    actual: next_len,
                    history: self,
                    candidate,
                })
            }
        };
        if next_count > self.maximum_values_u64 {
            return Err(HistoryLimit {
                limit: self.maximum_values,
                history: self,
                candidate,
            });
        }
        let ordered_len = candidate.ordered().len();
        let ordered_count = match to_u64(ordered_len) {
            Ok(value) => value,
            Err(()) => {
                return Err(OrderedEvidenceCountUnrepresentable {
                    actual: ordered_len,
                    history: self,
                    candidate,
                })
            }
        };
        let value_count = match add_u64(self.totals.value_count, 1) {
            Ok(value) => value,
            Err(()) => {
                return Err(CounterOverflow {
                    history: self,
                    candidate,
                })
            }
        };
        let ordered_evidence_count =
            match add_u64(self.totals.ordered_evidence_count, ordered_count) {
                Ok(value) => value,
                Err(()) => {
                    return Err(CounterOverflow {
                        history: self,
                        candidate,
                    })
                }
            };
        if reserve(&mut self.values, 1).is_err() {
            return Err(Allocation {
                requested_values: 1,
                history: self,
                candidate,
            });
        }
        self.values.push(candidate);
        self.totals = CallerRequestedFloat32ReportAdvisoryEvidenceHistoryTotals {
            value_count,
            ordered_evidence_count,
        };
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessor,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn value(sequence: u64, sample_value: f32) -> CallerRequestedFloat32ReportAdvisoryEvidence {
        let mut report_owner = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(4),
        );
        let report = report_owner
            .process_record(
                sequence,
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![sample_value]).unwrap(),
                    RawSourceTimestamp::new(3.0).unwrap(),
                    None,
                ),
            )
            .unwrap();
        let observations = MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap();
        let deltas = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap();
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        let advisory = MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 1).unwrap(),
        )
        .snapshot(observations, deltas, stability)
        .unwrap();
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
        .compose(report, advisory)
        .unwrap()
    }

    fn sample_pointer(value: &CallerRequestedFloat32ReportAdvisoryEvidence) -> *const f32 {
        value.report().sample().sample().values().as_ptr()
    }

    #[test]
    fn zero_bound_is_rejected_and_explicit_bound_is_retained() {
        assert_eq!(
            CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(0),
            Err(CallerRequestedFloat32ReportAdvisoryEvidenceHistoryConfigError::ZeroMaximumValues)
        );
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(usize::MAX).unwrap();
        assert_eq!(history.maximum_values(), usize::MAX);
        assert!(history.values().is_empty());
    }

    #[test]
    fn actual_p41_values_retain_caller_order_and_allocations() {
        let candidates = [value(41, 17.0), value(43, 19.0), value(41, 17.0)];
        let pointers: Vec<_> = candidates.iter().map(sample_pointer).collect();
        let history = candidates.into_iter().fold(
            CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(3).unwrap(),
            |history, candidate| history.append(candidate).unwrap(),
        );
        assert_eq!(
            history
                .values()
                .iter()
                .map(|value| value.report().sequence())
                .collect::<Vec<_>>(),
            vec![41, 43, 41]
        );
        assert_eq!(
            history
                .values()
                .iter()
                .map(sample_pointer)
                .collect::<Vec<_>>(),
            pointers
        );
        assert_eq!(history.totals().value_count(), 3);
        assert_eq!(history.totals().ordered_evidence_count(), 3);
    }

    #[test]
    fn capacity_and_allocation_failures_return_unchanged_owners() {
        let kept = value(1, 1.0);
        let candidate = value(2, 2.0);
        let kept_pointer = sample_pointer(&kept);
        let candidate_pointer = sample_pointer(&candidate);
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(kept)
            .unwrap();
        let before = history.totals();
        let (history, candidate) = history.append(candidate).unwrap_err().into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(sample_pointer(&history.values()[0]), kept_pointer);
        assert_eq!(sample_pointer(&candidate), candidate_pointer);

        let pointer = sample_pointer(&candidate);
        let error = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append_with(
                candidate,
                |_, requested| {
                    assert_eq!(requested, 1);
                    Err(())
                },
                |value| u64::try_from(value).map_err(|_| ()),
                |left, right| left.checked_add(right).ok_or(()),
                |left, right| left.checked_add(right).ok_or(()),
            )
            .unwrap_err();
        let (history, candidate) = error.into_parts();
        assert!(history.values().is_empty());
        assert_eq!(history.totals(), Default::default());
        assert_eq!(sample_pointer(&candidate), pointer);
    }

    #[test]
    fn usize_conversion_and_u64_overflow_are_atomic() {
        for failure in 0..4 {
            let candidate = value(10 + failure, failure as f32 + 3.0);
            let pointer = sample_pointer(&candidate);
            let mut history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(2).unwrap();
            if failure == 2 {
                history.totals.value_count = u64::MAX;
            }
            if failure == 3 {
                history.totals.ordered_evidence_count = u64::MAX;
            }
            let before = history.totals();
            let mut conversions = 0;
            let error = history
                .append_with(
                    candidate,
                    |_, _| Ok(()),
                    |value| {
                        conversions += 1;
                        if conversions == failure + 1 && failure < 2 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |left, right| left.checked_add(right).ok_or(()),
                    |left, right| left.checked_add(right).ok_or(()),
                )
                .unwrap_err();
            let (history, candidate) = error.into_parts();
            assert_eq!(history.totals(), before);
            assert!(history.values().is_empty());
            assert_eq!(sample_pointer(&candidate), pointer);
        }
        let candidate = value(20, 20.0);
        let pointer = sample_pointer(&candidate);
        let error = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append_with(
                candidate,
                |_, _| Ok(()),
                |value| Ok(value as u64),
                |left, right| left.checked_add(right).ok_or(()),
                |_, _| Err(()),
            )
            .unwrap_err();
        let (_, candidate) = error.into_parts();
        assert_eq!(sample_pointer(&candidate), pointer);
    }

    #[test]
    fn consuming_extraction_preserves_order_and_allocations() {
        let candidates = [value(7, 7.0), value(8, 8.0)];
        let pointers: Vec<_> = candidates.iter().map(sample_pointer).collect();
        let values = candidates
            .into_iter()
            .fold(
                CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(2).unwrap(),
                |history, candidate| history.append(candidate).unwrap(),
            )
            .into_values();
        assert_eq!(
            values
                .iter()
                .map(|value| value.report().sequence())
                .collect::<Vec<_>>(),
            vec![7, 8]
        );
        assert_eq!(
            values.iter().map(sample_pointer).collect::<Vec<_>>(),
            pointers
        );
    }

    #[test]
    fn owner_is_private_default_inert_and_non_applying() {
        let source = include_str!("caller_requested_float32_report_advisory_evidence_history.rs");
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(!include_str!("runtime.rs")
            .contains("CallerRequestedFloat32ReportAdvisoryEvidenceHistory"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod caller_requested_float32_report_advisory_evidence_history"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_report_advisory_evidence_history"));
    }
}
