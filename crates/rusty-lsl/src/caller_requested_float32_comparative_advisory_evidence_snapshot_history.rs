// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested retained history over actual published P47 snapshots.
//!
//! This crate-private bounded collection is fallibly allocated, default-inert,
//! advisory, and non-applying. It preserves the exact owned snapshots in caller
//! order, including equal repeated values and their existing allocations. It
//! infers neither loss nor continuity and grants no liblsl-equivalence, runtime,
//! session, transport, control, Manifold, or other applying authority.

use crate::caller_requested_float32_comparative_advisory_evidence_snapshot::CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryConfigError {
    ZeroMaximumSnapshots,
    ZeroMaximumFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds {
    maximum_snapshots: usize,
    maximum_snapshots_u64: u64,
    maximum_facts: u64,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds {
    pub(crate) fn new(
        maximum_snapshots: usize,
        maximum_facts: u64,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryConfigError>
    {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryConfigError::*;
        if maximum_snapshots == 0 {
            return Err(ZeroMaximumSnapshots);
        }
        if maximum_facts == 0 {
            return Err(ZeroMaximumFacts);
        }
        Ok(Self {
            maximum_snapshots,
            maximum_snapshots_u64: u64::try_from(maximum_snapshots).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_snapshots,
                }
            })?,
            maximum_facts,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryTotals {
    snapshot_count: u64,
    fact_count: u64,
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryTotals {
    pub(crate) const fn snapshot_count(&self) -> u64 {
        self.snapshot_count
    }
    pub(crate) const fn fact_count(&self) -> u64 {
        self.fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory {
    bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds,
    snapshots: Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot>,
    totals: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError {
    CollectionLengthOverflow {
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    SnapshotLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    SnapshotCountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    FactCountOverflow {
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    FactLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
    Allocation {
        requested_snapshots: usize,
        history: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    },
}

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    ) {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, candidate }
            | SnapshotLimit {
                history, candidate, ..
            }
            | SnapshotCountUnrepresentable {
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

impl CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory {
    pub(crate) fn new(
        bounds: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds,
    ) -> Self {
        Self {
            bounds,
            snapshots: Vec::new(),
            totals: Default::default(),
        }
    }
    pub(crate) fn snapshots(&self) -> &[CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot] {
        &self.snapshots
    }
    pub(crate) const fn totals(
        &self,
    ) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryTotals {
        self.totals
    }
    pub(crate) fn into_snapshots(
        self,
    ) -> Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot> {
        self.snapshots
    }

    pub(crate) fn append(
        self,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError>
    {
        self.append_with(
            candidate,
            |values, count| values.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
            |a, b| a.checked_add(b).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        candidate: CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        reserve: R,
        to_u64: C,
        add_u64: U,
        add_usize: Z,
    ) -> Result<Self, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnOnce(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history: self, candidate }) }; }
        let next_len = match add_usize(self.snapshots.len(), 1) {
            Ok(value) => value,
            Err(()) => fail!(CollectionLengthOverflow {}),
        };
        if next_len > self.bounds.maximum_snapshots {
            fail!(SnapshotLimit {
                limit: self.bounds.maximum_snapshots,
                required: next_len
            });
        }
        let snapshot_count = match to_u64(next_len) {
            Ok(value) => value,
            Err(()) => fail!(SnapshotCountUnrepresentable { actual: next_len }),
        };
        if snapshot_count > self.bounds.maximum_snapshots_u64 {
            fail!(SnapshotLimit {
                limit: self.bounds.maximum_snapshots,
                required: next_len
            });
        }
        let fact_count = match add_u64(self.totals.fact_count, candidate.observation_count()) {
            Ok(value) => value,
            Err(()) => fail!(FactCountOverflow {}),
        };
        if fact_count > self.bounds.maximum_facts {
            fail!(FactLimit {
                limit: self.bounds.maximum_facts,
                required: fact_count
            });
        }
        if reserve(&mut self.snapshots, 1).is_err() {
            fail!(Allocation {
                requested_snapshots: 1
            });
        }
        self.snapshots.push(candidate);
        self.totals = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryTotals {
            snapshot_count,
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
    use crate::caller_requested_float32_comparative_advisory_evidence_history::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence_snapshot::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner,
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
    use crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner,
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

    fn advisory_snapshot(
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
    fn package(sequence: u64) -> crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage{
        let evidence = |sequence| {
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
                        Sample::new(SampleLimits::new(1).unwrap(), 1, vec![sequence as f32])
                            .unwrap(),
                        RawSourceTimestamp::new(3.0).unwrap(),
                        None,
                    ),
                )
                .unwrap();
            CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
            )
            .compose(report, advisory_snapshot())
            .unwrap()
        };
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
            .unwrap()
            .append(evidence(sequence))
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            evidence(sequence + 1),
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                .unwrap()
                .append(advisory_snapshot())
                .unwrap(),
        )
        .unwrap();
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 2, 4).unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }
    fn comparative(seed: u64) -> crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence{
        let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(package(seed), package(seed + 10))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
        )
        .compose(package(seed + 20), package(seed + 30), proposal)
        .unwrap()
    }
    fn snapshot(seed: u64) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(2, 16).unwrap(),
        )
        .append(comparative(seed))
        .unwrap()
        .append(comparative(seed + 50))
        .unwrap();
        let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(comparative(seed + 100), comparative(seed + 150))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(2, 4, 6).unwrap(),
        )
        .snapshot(history, delta)
        .unwrap()
    }
    fn identity(value: &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot) -> (*const crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence, *const crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaFact, *const f32){
        (
            value.history().evidence().as_ptr(),
            value.delta_proposal().facts().as_ptr(),
            value.history().evidence()[0].earlier().history().values()[0]
                .report()
                .sample()
                .sample()
                .values()
                .as_ptr(),
        )
    }
    fn bounds(
        snapshots: usize,
        facts: u64,
    ) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds {
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds::new(
            snapshots, facts,
        )
        .unwrap()
    }

    #[test]
    fn zero_and_extreme_bounds_are_checked() {
        use CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryConfigError::*;
        assert_eq!(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds::new(0, 1),
            Err(ZeroMaximumSnapshots)
        );
        assert_eq!(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds::new(1, 0),
            Err(ZeroMaximumFacts)
        );
        let extreme = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(
            bounds(usize::MAX, u64::MAX),
        );
        assert_eq!(extreme.totals(), Default::default());
    }

    #[test]
    fn capacity_fact_limits_and_repeated_snapshots_are_transactional() {
        let first = snapshot(1);
        let first_id = identity(&first);
        let repeated = snapshot(1);
        let repeated_id = identity(&repeated);
        let history =
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(bounds(2, 12))
                .append(first)
                .unwrap()
                .append(repeated)
                .unwrap();
        assert_eq!(
            history.totals(),
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryTotals {
                snapshot_count: 2,
                fact_count: 12
            }
        );
        assert_eq!(history.totals().snapshot_count(), 2);
        assert_eq!(
            history.snapshots().iter().map(identity).collect::<Vec<_>>(),
            vec![first_id, repeated_id]
        );
        assert_eq!(history.snapshots()[0], history.snapshots()[1]);
        assert_eq!(
            history
                .into_snapshots()
                .iter()
                .map(identity)
                .collect::<Vec<_>>(),
            vec![first_id, repeated_id]
        );

        let candidate = snapshot(20);
        let id = identity(&candidate);
        let error =
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(bounds(1, 12))
                .append(snapshot(10))
                .unwrap()
                .append(candidate)
                .unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError::SnapshotLimit { limit: 1, required: 2, .. }));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals().fact_count(), 6);
        assert_eq!(identity(&candidate), id);

        let candidate = snapshot(30);
        let id = identity(&candidate);
        let error =
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(bounds(2, 5))
                .append(candidate)
                .unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryAppendError::FactLimit { limit: 5, required: 6, .. }));
        let (history, candidate) = error.into_parts();
        assert!(history.snapshots().is_empty());
        assert_eq!(identity(&candidate), id);
    }

    #[test]
    fn injected_allocation_conversion_and_overflows_roll_back_unchanged() {
        for failure in 0..4 {
            let kept = snapshot(40);
            let kept_id = identity(&kept);
            let candidate = snapshot(50 + failure);
            let candidate_id = identity(&candidate);
            let mut history =
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(bounds(
                    2,
                    u64::MAX,
                ))
                .append(kept)
                .unwrap();
            if failure == 2 {
                history.totals.fact_count = u64::MAX;
            }
            let error = history
                .append_with(
                    candidate,
                    |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                    |value| {
                        if failure == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
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
            let (history, candidate) = error.into_parts();
            assert_eq!(identity(&history.snapshots()[0]), kept_id);
            assert_eq!(identity(&candidate), candidate_id);
            assert_eq!(history.snapshots().len(), 1);
        }
    }

    #[test]
    fn boundary_is_private_caller_requested_inert_and_non_authoritative() {
        let source = include_str!(
            "caller_requested_float32_comparative_advisory_evidence_snapshot_history.rs"
        );
        for wording in [
            "crate-private",
            "caller-requested",
            "default-inert",
            "advisory",
            "non-applying",
            "infers neither loss nor continuity",
            "liblsl-equivalence",
            "Manifold",
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
            .contains("CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory"));
        assert!(!include_str!("lib.rs").contains(
            "pub use caller_requested_float32_comparative_advisory_evidence_snapshot_history"
        ));
    }
}
