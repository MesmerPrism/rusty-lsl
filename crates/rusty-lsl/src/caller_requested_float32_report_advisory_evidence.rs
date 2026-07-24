// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested retention of actual P34 report and P40 advisory evidence.
//!
//! This crate-private composition consumes and retains both complete owners. It
//! creates only a bounded deterministic index over their existing facts; it
//! does not infer continuity or loss and cannot apply the advisory snapshot.

use crate::exact_sequence_loss_health::ExactSequenceClassification;
use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessingOutcome;
use crate::morphospace_float32_report_advisory_snapshot::{
    MorphospaceFloat32ReportAdvisorySnapshot, MorphospaceFloat32ReportAdvisorySnapshotEvidence,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessingDisposition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ReportAdvisoryEvidenceConfigError {
    ZeroMaximumAdvisoryEvidence,
    ZeroMaximumOrderedEvidence,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportAdvisoryEvidenceBounds {
    maximum_advisory_evidence: usize,
    maximum_ordered_evidence: usize,
}

impl CallerRequestedFloat32ReportAdvisoryEvidenceBounds {
    pub(crate) fn new(
        maximum_advisory_evidence: usize,
        maximum_ordered_evidence: usize,
    ) -> Result<Self, CallerRequestedFloat32ReportAdvisoryEvidenceConfigError> {
        use CallerRequestedFloat32ReportAdvisoryEvidenceConfigError::*;
        if maximum_advisory_evidence == 0 {
            return Err(ZeroMaximumAdvisoryEvidence);
        }
        if maximum_ordered_evidence == 0 {
            return Err(ZeroMaximumOrderedEvidence);
        }
        for requested in [maximum_advisory_evidence, maximum_ordered_evidence] {
            u64::try_from(requested).map_err(|_| BoundUnrepresentable { requested })?;
        }
        Ok(Self {
            maximum_advisory_evidence,
            maximum_ordered_evidence,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ReportAdvisoryEvidenceItem {
    TransactionalReport {
        sequence: u64,
        classification: ExactSequenceClassification,
        disposition: RequestedTimestampPostProcessingDisposition,
        raw_source_timestamp_bits: u64,
        effective_timestamp_bits: u64,
    },
    Advisory {
        advisory_index: u64,
        evidence: MorphospaceFloat32ReportAdvisorySnapshotEvidence,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportAdvisoryEvidence {
    report: Float32SessionReportRequestedPostProcessingOutcome,
    advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    ordered: Vec<CallerRequestedFloat32ReportAdvisoryEvidenceItem>,
}

impl CallerRequestedFloat32ReportAdvisoryEvidence {
    pub(crate) const fn report(&self) -> &Float32SessionReportRequestedPostProcessingOutcome {
        &self.report
    }

    pub(crate) const fn advisory(&self) -> &MorphospaceFloat32ReportAdvisorySnapshot {
        &self.advisory
    }

    pub(crate) fn ordered(&self) -> &[CallerRequestedFloat32ReportAdvisoryEvidenceItem] {
        &self.ordered
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Float32SessionReportRequestedPostProcessingOutcome,
        MorphospaceFloat32ReportAdvisorySnapshot,
    ) {
        (self.report, self.advisory)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ReportAdvisoryEvidenceError {
    AdvisoryEvidenceLimit {
        limit: usize,
        actual: usize,
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    IndexUnrepresentable {
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    EvidenceCountOverflow {
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    OrderedEvidenceLimit {
        limit: usize,
        required: usize,
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    Allocation {
        requested: usize,
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    },
}

impl CallerRequestedFloat32ReportAdvisoryEvidenceError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        Float32SessionReportRequestedPostProcessingOutcome,
        MorphospaceFloat32ReportAdvisorySnapshot,
    ) {
        use CallerRequestedFloat32ReportAdvisoryEvidenceError::*;
        match self {
            AdvisoryEvidenceLimit {
                report, advisory, ..
            }
            | IndexUnrepresentable { report, advisory }
            | EvidenceCountOverflow { report, advisory }
            | OrderedEvidenceLimit {
                report, advisory, ..
            }
            | Allocation {
                report, advisory, ..
            } => (report, advisory),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportAdvisoryEvidenceOwner {
    bounds: CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
}

impl CallerRequestedFloat32ReportAdvisoryEvidenceOwner {
    pub(crate) const fn new(bounds: CallerRequestedFloat32ReportAdvisoryEvidenceBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn compose(
        &self,
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
    ) -> Result<
        CallerRequestedFloat32ReportAdvisoryEvidence,
        CallerRequestedFloat32ReportAdvisoryEvidenceError,
    > {
        self.compose_with(
            report,
            advisory,
            |items, requested| items.try_reserve_exact(requested).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
            u64::try_from,
        )
    }

    fn compose_with<R, A, I>(
        &self,
        report: Float32SessionReportRequestedPostProcessingOutcome,
        advisory: MorphospaceFloat32ReportAdvisorySnapshot,
        reserve: R,
        add: A,
        mut index: I,
    ) -> Result<
        CallerRequestedFloat32ReportAdvisoryEvidence,
        CallerRequestedFloat32ReportAdvisoryEvidenceError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32ReportAdvisoryEvidenceItem>,
            usize,
        ) -> Result<(), ()>,
        A: FnOnce(usize, usize) -> Result<usize, ()>,
        I: FnMut(usize) -> Result<u64, std::num::TryFromIntError>,
    {
        use CallerRequestedFloat32ReportAdvisoryEvidenceError::*;
        let actual = advisory.evidence().len();
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* report, advisory }) }; }
        if actual > self.bounds.maximum_advisory_evidence {
            fail!(AdvisoryEvidenceLimit {
                limit: self.bounds.maximum_advisory_evidence,
                actual: actual
            });
        }
        if index(actual).is_err() {
            fail!(IndexUnrepresentable {});
        }
        let required = match add(1, actual) {
            Ok(value) => value,
            Err(_) => fail!(EvidenceCountOverflow {}),
        };
        if required > self.bounds.maximum_ordered_evidence {
            fail!(OrderedEvidenceLimit {
                limit: self.bounds.maximum_ordered_evidence,
                required: required
            });
        }
        for item_index in 0..actual {
            if index(item_index).is_err() {
                fail!(IndexUnrepresentable {});
            }
        }
        let mut ordered = Vec::new();
        if reserve(&mut ordered, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        ordered.push(
            CallerRequestedFloat32ReportAdvisoryEvidenceItem::TransactionalReport {
                sequence: report.sequence(),
                classification: report.classification(),
                disposition: report.facts().disposition(),
                raw_source_timestamp_bits: report.sample().raw_source_timestamp().value().to_bits(),
                effective_timestamp_bits: report.facts().effective_timestamp().value().to_bits(),
            },
        );
        ordered.extend(advisory.evidence().iter().copied().enumerate().map(
            |(advisory_index, evidence)| {
                CallerRequestedFloat32ReportAdvisoryEvidenceItem::Advisory {
                    advisory_index: u64::try_from(advisory_index)
                        .expect("validated advisory index"),
                    evidence,
                }
            },
        ));
        Ok(CallerRequestedFloat32ReportAdvisoryEvidence {
            report,
            advisory,
            ordered,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_advisory_snapshot_history::MorphospaceFloat32ReportAdvisorySnapshotHistory;
    use crate::morphospace_float32_report_observation::{
        tests::outcome, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
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

    fn actual_report() -> Float32SessionReportRequestedPostProcessingOutcome {
        let mut owner = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(4),
        );
        let sample = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![17.0]).unwrap(),
            RawSourceTimestamp::new(3.0).unwrap(),
            None,
        );
        owner.process_record(41, sample).unwrap()
    }

    fn actual_advisory() -> MorphospaceFloat32ReportAdvisorySnapshot {
        let observations = MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap();
        let deltas = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap();
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 1).unwrap(),
        )
        .snapshot(observations, deltas, stability)
        .unwrap()
    }

    fn actual_nonzero_advisory() -> MorphospaceFloat32ReportAdvisorySnapshot {
        let sample = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![23.0]).unwrap(),
            RawSourceTimestamp::new(5.0).unwrap(),
            None,
        );
        let observation = MorphospaceFloat32ReportObservationOwner::new(1)
            .unwrap()
            .observe(outcome(vec![23.0f32.to_bits() as u64], vec![sample]))
            .unwrap();
        let window = MorphospaceFloat32ReportObservationWindow::new(1, 1)
            .unwrap()
            .append(observation)
            .unwrap();
        let observations = MorphospaceFloat32ReportObservationHistory::new(1, 1)
            .unwrap()
            .append(window)
            .unwrap();
        let deltas = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap();
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 2).unwrap(),
        )
        .snapshot(observations, deltas, stability)
        .unwrap()
    }

    fn owner() -> CallerRequestedFloat32ReportAdvisoryEvidenceOwner {
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
    }

    fn pointers(report: &Float32SessionReportRequestedPostProcessingOutcome) -> *const f32 {
        report.sample().sample().values().as_ptr()
    }

    #[test]
    fn actual_p34_and_p40_owners_are_retained_with_deterministic_extraction() {
        let report = actual_report();
        let allocation = pointers(&report);
        let evidence = owner().compose(report, actual_advisory()).unwrap();
        assert_eq!(pointers(evidence.report()), allocation);
        assert!(evidence.advisory().evidence().is_empty());
        assert_eq!(
            evidence.ordered(),
            &[
                CallerRequestedFloat32ReportAdvisoryEvidenceItem::TransactionalReport {
                    sequence: 41,
                    classification: ExactSequenceClassification::First,
                    disposition: RequestedTimestampPostProcessingDisposition::RetainedUnchanged,
                    raw_source_timestamp_bits: 3.0f64.to_bits(),
                    effective_timestamp_bits: 3.0f64.to_bits(),
                }
            ]
        );
        let (report, advisory) = evidence.into_parts();
        assert_eq!(pointers(&report), allocation);
        assert!(advisory.evidence().is_empty());
    }

    #[test]
    fn actual_p34_p40_p41_nonzero_composition_preserves_both_allocations_and_order() {
        let report = actual_report();
        let report_allocation = pointers(&report);
        let advisory = actual_nonzero_advisory();
        let advisory_allocation = advisory.observation_history().windows()[0].observations()[0]
            .records()[0]
            .processed()
            .sample()
            .sample()
            .values()
            .as_ptr();
        let history = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(advisory)
            .unwrap();
        assert_eq!(history.totals().snapshot_count(), 1);
        assert_eq!(history.totals().evidence_count(), 1);
        let advisory = history.into_snapshots().into_iter().next().unwrap();
        let composed = CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 2).unwrap(),
        )
        .compose(report, advisory)
        .unwrap();
        assert_eq!(pointers(composed.report()), report_allocation);
        assert_eq!(composed.ordered().len(), 2);
        assert!(matches!(
            composed.ordered()[0],
            CallerRequestedFloat32ReportAdvisoryEvidenceItem::TransactionalReport { .. }
        ));
        assert!(matches!(
            composed.ordered()[1],
            CallerRequestedFloat32ReportAdvisoryEvidenceItem::Advisory {
                advisory_index: 0,
                ..
            }
        ));
        let (report, advisory) = composed.into_parts();
        assert_eq!(pointers(&report), report_allocation);
        assert_eq!(
            advisory.observation_history().windows()[0].observations()[0].records()[0]
                .processed()
                .sample()
                .sample()
                .values()
                .as_ptr(),
            advisory_allocation
        );
    }

    #[test]
    fn configuration_bounds_are_explicit() {
        use CallerRequestedFloat32ReportAdvisoryEvidenceConfigError::*;
        assert_eq!(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(0, 1),
            Err(ZeroMaximumAdvisoryEvidence)
        );
        assert_eq!(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 0),
            Err(ZeroMaximumOrderedEvidence)
        );
        assert!(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(usize::MAX, usize::MAX).is_ok()
        );
    }

    #[test]
    fn every_fallible_stage_returns_both_inputs_and_allocation_unchanged() {
        for kind in 0..4 {
            let report = actual_report();
            let allocation = pointers(&report);
            let advisory = actual_advisory();
            let error = match kind {
                0 => CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
                    CallerRequestedFloat32ReportAdvisoryEvidenceBounds {
                        maximum_advisory_evidence: 1,
                        maximum_ordered_evidence: 1,
                    },
                )
                .compose_with(
                    report,
                    advisory,
                    |_, _| Ok(()),
                    |_, _| Err(()),
                    u64::try_from,
                )
                .unwrap_err(),
                1 => owner()
                    .compose_with(
                        report,
                        advisory,
                        |_, _| Ok(()),
                        |a, b| a.checked_add(b).ok_or(()),
                        |_| u64::try_from(u128::MAX),
                    )
                    .unwrap_err(),
                2 => owner()
                    .compose_with(
                        report,
                        advisory,
                        |_, _| Err(()),
                        |a, b| a.checked_add(b).ok_or(()),
                        u64::try_from,
                    )
                    .unwrap_err(),
                _ => CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
                    CallerRequestedFloat32ReportAdvisoryEvidenceBounds {
                        maximum_advisory_evidence: 1,
                        maximum_ordered_evidence: 0,
                    },
                )
                .compose(report, advisory)
                .unwrap_err(),
            };
            let (returned_report, returned_advisory) = error.into_parts();
            assert_eq!(pointers(&returned_report), allocation);
            assert!(returned_advisory.evidence().is_empty());
        }
    }

    #[test]
    fn composition_has_no_applying_or_shared_surface() {
        let source = include_str!("caller_requested_float32_report_advisory_evidence.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(
            !include_str!("runtime.rs").contains("CallerRequestedFloat32ReportAdvisoryEvidence")
        );
        assert!(!include_str!("lib.rs")
            .contains("pub mod caller_requested_float32_report_advisory_evidence"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_report_advisory_evidence"));
    }
}
