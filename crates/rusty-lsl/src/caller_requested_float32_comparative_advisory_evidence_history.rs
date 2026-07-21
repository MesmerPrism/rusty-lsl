// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded transactional history of actual P45 comparative evidence.
//!
//! This crate-private, default-inert container retains evidence in exact caller
//! order without cloning or reconstructing any nested allocation. It infers
//! neither loss nor continuity, applies no advisory result, and grants no
//! runtime, liblsl, device, Manifold, session, stream, or control authority.

use crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryConfigError {
    ZeroMaximumEvidence,
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds {
    maximum_evidence: usize,
    maximum_evidence_u64: u64,
    maximum_facts: u64,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds {
    pub(crate) fn new(
        maximum_evidence: usize,
        maximum_facts: usize,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryConfigError> {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryConfigError::*;
        if maximum_evidence == 0 {
            return Err(ZeroMaximumEvidence);
        }
        if maximum_facts == 0 {
            return Err(ZeroMaximumFacts);
        }
        Ok(Self {
            maximum_evidence,
            maximum_evidence_u64: u64::try_from(maximum_evidence).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_evidence,
                }
            })?,
            maximum_facts: u64::try_from(maximum_facts).map_err(|_| BoundUnrepresentable {
                requested: maximum_facts,
            })?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryTotals {
    evidence_count: u64,
    fact_count: u64,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryTotals {
    pub(crate) const fn evidence_count(&self) -> u64 {
        self.evidence_count
    }
    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory {
    bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds,
    evidence: Vec<CallerRequestedFloat32ComparativeAdvisoryEvidence>,
    totals: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError {
    CollectionLengthOverflow {
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    EvidenceLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    EvidenceCountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    FactCountOverflow {
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    FactLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
    Allocation {
        requested_evidence: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    },
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        CallerRequestedFloat32ComparativeAdvisoryEvidence,
    ) {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, candidate }
            | EvidenceLimit {
                history, candidate, ..
            }
            | EvidenceCountUnrepresentable {
                history, candidate, ..
            }
            | FactCountOverflow { history, candidate }
            | FactLimit {
                history, candidate, ..
            }
            | Allocation {
                history, candidate, ..
            } => (history, candidate),
        }
    }
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory {
    pub(crate) fn new(
        bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds,
    ) -> Self {
        Self {
            bounds,
            evidence: Vec::new(),
            totals: Default::default(),
        }
    }
    pub(crate) fn evidence(&self) -> &[CallerRequestedFloat32ComparativeAdvisoryEvidence] {
        &self.evidence
    }
    pub(crate) const fn totals(
        &self,
    ) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryTotals {
        self.totals
    }
    pub(crate) fn into_evidence(self) -> Vec<CallerRequestedFloat32ComparativeAdvisoryEvidence> {
        self.evidence
    }

    pub(crate) fn append(
        self,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError> {
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
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidence,
        reserve: R,
        to_u64: C,
        add_u64: U,
        add_usize: Z,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32ComparativeAdvisoryEvidence>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnOnce(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history: self, candidate }) }; }
        let next_len = match add_usize(self.evidence.len(), 1) {
            Ok(value) => value,
            Err(()) => fail!(CollectionLengthOverflow {}),
        };
        if next_len > self.bounds.maximum_evidence {
            fail!(EvidenceLimit {
                limit: self.bounds.maximum_evidence,
                required: next_len
            });
        }
        let evidence_count = match to_u64(next_len) {
            Ok(value) => value,
            Err(()) => fail!(EvidenceCountUnrepresentable { actual: next_len }),
        };
        if evidence_count > self.bounds.maximum_evidence_u64 {
            fail!(EvidenceLimit {
                limit: self.bounds.maximum_evidence,
                required: next_len
            });
        }
        let fact_count = match add_u64(self.totals.fact_count, candidate.fact_count()) {
            Ok(value) => value,
            Err(()) => fail!(FactCountOverflow {}),
        };
        if fact_count > self.bounds.maximum_facts {
            fail!(FactLimit {
                limit: self.bounds.maximum_facts,
                required: fact_count
            });
        }
        if reserve(&mut self.evidence, 1).is_err() {
            fail!(Allocation {
                requested_evidence: 1
            });
        }
        self.evidence.push(candidate);
        self.totals = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryTotals {
            evidence_count,
            fact_count,
        };
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_advisory_report_package::{
        CallerRequestedFloat32AdvisoryReportPackageBounds,
        CallerRequestedFloat32AdvisoryReportPackageOwner,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_advisory_report_package_delta_proposal::{
        MorphospaceFloat32AdvisoryReportPackageDeltaBounds,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner,
    };
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_advisory_snapshot_history::MorphospaceFloat32ReportAdvisorySnapshotHistory;
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::morphospace_float32_retained_advisory_summary::{
        MorphospaceFloat32RetainedAdvisorySummaryBounds,
        MorphospaceFloat32RetainedAdvisorySummaryOwner,
    };
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessor,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn snapshot(
    ) -> crate::morphospace_float32_report_advisory_snapshot::MorphospaceFloat32ReportAdvisorySnapshot
    {
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 1).unwrap(),
        )
        .snapshot(
            MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap(),
            MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap(),
            stability,
        )
        .unwrap()
    }
    fn report_evidence(sequence: u64) -> crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidence{
        let mut processor = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(4),
        );
        let report = processor
            .process_record(
                sequence,
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![sequence as f32]).unwrap(),
                    RawSourceTimestamp::new(3.0).unwrap(),
                    None,
                ),
            )
            .unwrap();
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
        .compose(report, snapshot())
        .unwrap()
    }
    fn package(seed: u64) -> crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage{
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(report_evidence(seed))
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            report_evidence(seed + 1),
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                .unwrap()
                .append(snapshot())
                .unwrap(),
        )
        .unwrap();
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 2, 4).unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }
    fn candidate(seed: u64) -> CallerRequestedFloat32ComparativeAdvisoryEvidence {
        let earlier = package(seed);
        let later = package(seed + 10);
        let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(package(seed + 20), package(seed + 30))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
        )
        .compose(earlier, later, proposal)
        .unwrap()
    }
    fn identity(value: &CallerRequestedFloat32ComparativeAdvisoryEvidence) -> (
        *const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidenceFact,
        *const crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageDeltaFact,
        Vec<*const f32>,
    ){
        let sample_pointer = |package: &crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage| package.history().values()[0].report().sample().sample().values().as_ptr();
        (
            value.facts().as_ptr(),
            value.delta_proposal().facts().as_ptr(),
            vec![
                sample_pointer(value.earlier()),
                sample_pointer(value.later()),
                sample_pointer(value.delta_proposal().earlier()),
                sample_pointer(value.delta_proposal().later()),
            ],
        )
    }
    fn bounds(
        entries: usize,
        facts: usize,
    ) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds {
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(entries, facts).unwrap()
    }

    #[test]
    fn zero_exact_and_one_past_bounds_are_transactional() {
        assert_eq!(CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(0,1), Err(CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryConfigError::ZeroMaximumEvidence));
        assert_eq!(CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(1,0), Err(CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryConfigError::ZeroMaximumFacts));
        let first = candidate(1);
        let second = candidate(2);
        let second_id = identity(&second);
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(bounds(1, 16))
            .append(first)
            .unwrap();
        let error = history.append(second).unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError::EvidenceLimit {
                limit: 1,
                required: 2,
                ..
            }
        ));
        let (history, second) = error.into_parts();
        assert_eq!(history.totals().evidence_count(), 1);
        assert_eq!(identity(&second), second_id);
        let value = candidate(3);
        let id = identity(&value);
        let error = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(bounds(1, 7))
            .append(value)
            .unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryAppendError::FactLimit {
                limit: 7,
                required: 8,
                ..
            }
        ));
        let (history, value) = error.into_parts();
        assert!(history.evidence().is_empty());
        assert_eq!(identity(&value), id);
    }

    #[test]
    fn repeated_order_and_consuming_extraction_preserve_every_allocation() {
        let values = [candidate(10), candidate(20), candidate(30)];
        let ids: Vec<_> = values.iter().map(identity).collect();
        let history = values.into_iter().fold(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(bounds(3, 24)),
            |h, v| h.append(v).unwrap(),
        );
        assert_eq!(
            history.totals(),
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryTotals {
                evidence_count: 3,
                fact_count: 24
            }
        );
        assert_eq!(
            history.evidence().iter().map(identity).collect::<Vec<_>>(),
            ids
        );
        assert_eq!(
            history
                .into_evidence()
                .iter()
                .map(identity)
                .collect::<Vec<_>>(),
            ids
        );
    }

    #[test]
    fn allocation_conversion_and_arithmetic_failures_roll_back_both_owners() {
        for failure in 0..4 {
            let kept = candidate(40);
            let kept_id = identity(&kept);
            let value = candidate(50 + failure);
            let value_id = identity(&value);
            let mut history =
                CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(bounds(2, 32))
                    .append(kept)
                    .unwrap();
            let before = history.totals();
            if failure == 2 {
                history.totals.fact_count = u64::MAX;
            }
            let error = history
                .append_with(
                    value,
                    |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                    |v| if failure == 1 { Err(()) } else { Ok(v as u64) },
                    |a, b| a.checked_add(b).ok_or(()),
                    |a, b| {
                        if failure == 0 {
                            Err(())
                        } else {
                            a.checked_add(b).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            let (history, value) = error.into_parts();
            if failure == 2 {
                assert_eq!(history.totals().fact_count(), u64::MAX);
            } else {
                assert_eq!(history.totals(), before);
            }
            assert_eq!(identity(&history.evidence()[0]), kept_id);
            assert_eq!(identity(&value), value_id);
        }
    }

    #[test]
    fn boundary_is_private_default_inert_non_applying_and_non_authoritative() {
        let source =
            include_str!("caller_requested_float32_comparative_advisory_evidence_history.rs");
        for wording in [
            "crate-private",
            "default-inert",
            "infers\n//! neither loss nor continuity",
            "applies no advisory result",
            "runtime, liblsl, device, Manifold",
        ] {
            assert!(source.contains(wording));
        }
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(!include_str!("runtime.rs")
            .contains("CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_comparative_advisory_evidence_history"));
    }
}
