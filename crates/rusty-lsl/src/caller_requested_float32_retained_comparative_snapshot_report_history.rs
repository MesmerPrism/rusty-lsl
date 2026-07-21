// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested bounded history of actual retained comparative reports.
//!
//! This crate-private owner is default-inert and preserves complete report
//! ownership, caller order, repeated reports, and every nested allocation. It
//! records only exact report and evidence counts: it infers no loss,
//! continuity, or causality. It is not liblsl-equivalent and grants no public
//! API, runtime activation, session, transport, control, Manifold, or other
//! applying authority.

use crate::caller_requested_float32_retained_comparative_snapshot_report::CallerRequestedFloat32RetainedComparativeSnapshotReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryConfigError {
    ZeroMaximumReports,
    ZeroMaximumEvidenceEntries,
    ReportBoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds {
    maximum_reports: usize,
    maximum_reports_u64: u64,
    maximum_evidence_entries: u64,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds {
    pub(crate) fn new(
        maximum_reports: usize,
        maximum_evidence_entries: u64,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryConfigError>
    {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryConfigError::*;
        if maximum_reports == 0 {
            return Err(ZeroMaximumReports);
        }
        if maximum_evidence_entries == 0 {
            return Err(ZeroMaximumEvidenceEntries);
        }
        let maximum_reports_u64 = u64::try_from(maximum_reports)
            .map_err(|_| ReportBoundUnrepresentable { requested: maximum_reports })?;
        Ok(Self {
            maximum_reports,
            maximum_reports_u64,
            maximum_evidence_entries,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryTotals {
    report_count: u64,
    evidence_count: u64,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryTotals {
    pub(crate) const fn report_count(self) -> u64 {
        self.report_count
    }

    pub(crate) const fn evidence_count(self) -> u64 {
        self.evidence_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportHistory {
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds,
    reports: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReport>,
    totals: CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError {
    CollectionLengthOverflow {
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    ReportLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    ReportCountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    EvidenceCountOverflow {
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    EvidenceLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    Allocation {
        requested_reports: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotReport,
    ) {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, report }
            | ReportLimit { history, report, .. }
            | ReportCountUnrepresentable { history, report, .. }
            | EvidenceCountOverflow { history, report }
            | EvidenceLimit { history, report, .. }
            | Allocation { history, report, .. } => (history, report),
        }
    }
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportHistory {
    pub(crate) fn new(
        bounds: CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds,
    ) -> Self {
        Self {
            bounds,
            reports: Vec::new(),
            totals: Default::default(),
        }
    }

    pub(crate) fn reports(
        &self,
    ) -> &[CallerRequestedFloat32RetainedComparativeSnapshotReport] {
        &self.reports
    }

    pub(crate) const fn totals(
        &self,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryTotals {
        self.totals
    }

    pub(crate) fn into_reports(
        self,
    ) -> Vec<CallerRequestedFloat32RetainedComparativeSnapshotReport> {
        self.reports
    }

    pub(crate) fn append(
        self,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError>
    {
        self.append_with(
            report,
            |reports, requested| reports.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
        reserve: R,
        convert: C,
        add_u64: U,
        add_usize: Z,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError>
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32RetainedComparativeSnapshotReport>,
            usize,
        ) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnOnce(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError::*;
        macro_rules! fail {
            ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => {
                return Err($variant { $($field: $value,)* history: self, report })
            };
        }

        // Every fallible calculation and allocation completes before push or
        // total replacement, making append a single visible commit.
        let next_len = match add_usize(self.reports.len(), 1) {
            Ok(value) => value,
            Err(()) => fail!(CollectionLengthOverflow {}),
        };
        if next_len > self.bounds.maximum_reports {
            fail!(ReportLimit {
                limit: self.bounds.maximum_reports,
                required: next_len
            });
        }
        let report_count = match convert(next_len) {
            Ok(value) => value,
            Err(()) => fail!(ReportCountUnrepresentable { actual: next_len }),
        };
        if report_count > self.bounds.maximum_reports_u64 {
            fail!(ReportLimit {
                limit: self.bounds.maximum_reports,
                required: next_len
            });
        }
        let evidence_count = match add_u64(self.totals.evidence_count, report.evidence_count()) {
            Ok(value) => value,
            Err(()) => fail!(EvidenceCountOverflow {}),
        };
        if evidence_count > self.bounds.maximum_evidence_entries {
            fail!(EvidenceLimit {
                limit: self.bounds.maximum_evidence_entries,
                required: evidence_count
            });
        }
        if reserve(&mut self.reports, 1).is_err() {
            fail!(Allocation {
                requested_reports: 1
            });
        }

        self.reports.push(report);
        self.totals = CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryTotals {
            report_count,
            evidence_count,
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
    use crate::caller_requested_float32_comparative_advisory_evidence_snapshot_history::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds,
    };
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
    use crate::caller_requested_float32_retained_comparative_snapshot_package::{
        CallerRequestedFloat32RetainedComparativeSnapshotPackageBounds,
        CallerRequestedFloat32RetainedComparativeSnapshotPackageOwner,
    };
    use crate::caller_requested_float32_retained_comparative_snapshot_report::{
        CallerRequestedFloat32RetainedComparativeSnapshotReportBounds,
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner,
    };
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
    use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_history::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryBounds,
    };
    use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_proposal::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner,
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

    fn report(seed: u64) -> CallerRequestedFloat32RetainedComparativeSnapshotReport {
        let advisory_snapshot = || {
            let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
                MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0)
                    .unwrap(),
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
        };
        let evidence = |sequence| {
            let mut processor = Float32SessionReportRequestedPostProcessing::new(
                RequestedTimestampPostProcessor::new(
                    RequestedTimestampPostProcessing::Monotonic(
                        RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
                    ),
                )
                .unwrap(),
                ExactSequenceLossHealth::new(4),
            );
            let processed = processor
                .process_record(
                    sequence,
                    TimestampedSample::new(
                        Sample::new(
                            SampleLimits::new(1).unwrap(),
                            1,
                            vec![sequence as f32],
                        )
                        .unwrap(),
                        RawSourceTimestamp::new(3.0).unwrap(),
                        None,
                    ),
                )
                .unwrap();
            CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
            )
            .compose(processed, advisory_snapshot())
            .unwrap()
        };
        let package = |base| {
            CallerRequestedFloat32AdvisoryReportPackageOwner::new(
                CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 2, 4).unwrap(),
            )
            .package(
                CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(1)
                    .unwrap()
                    .append(evidence(base))
                    .unwrap(),
                MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
                    MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
                )
                .summarize(
                    evidence(base + 1),
                    MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                        .unwrap()
                        .append(advisory_snapshot())
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap()
        };
        let comparative = |base| {
            let delta = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
                MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
            )
            .propose(package(base), package(base + 10))
            .unwrap();
            CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
            )
            .compose(package(base + 20), package(base + 30), delta)
            .unwrap()
        };
        let snapshot = |base| {
            let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(1, 8)
                    .unwrap(),
            )
            .append(comparative(base))
            .unwrap();
            let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
                MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
            )
            .propose(comparative(base + 100), comparative(base + 200))
            .unwrap();
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(1, 4, 5)
                    .unwrap(),
            )
            .snapshot(history, delta)
            .unwrap()
        };
        let snapshot_delta = |base| {
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner::new(
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds::new(6).unwrap(),
            )
            .propose(snapshot(base), snapshot(base + 1_000))
            .unwrap()
        };
        let delta_history =
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistory::new(
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaHistoryBounds::new(1, 6)
                    .unwrap(),
            )
            .append(snapshot_delta(seed))
            .unwrap();
        let retained_package = CallerRequestedFloat32RetainedComparativeSnapshotPackageOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotPackageBounds::new(1, 6, 7).unwrap(),
        )
        .package(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds::new(1, 5)
                    .unwrap(),
            )
            .append(snapshot(seed + 2_000))
            .unwrap(),
            snapshot_delta(seed + 3_000),
        )
        .unwrap();
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportBounds::new(1, 7, 8).unwrap(),
        )
        .report(delta_history, retained_package)
        .unwrap()
    }

    fn identity(
        value: &CallerRequestedFloat32RetainedComparativeSnapshotReport,
    ) -> (*const f32, *const crate::caller_requested_float32_retained_comparative_snapshot_report::CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence) {
        (
            value.delta_history().proposals()[0].earlier().history().evidence()[0]
                .earlier().history().values()[0].report().sample().sample().values().as_ptr(),
            value.evidence().as_ptr(),
        )
    }

    fn bounds(reports: usize, evidence: u64) -> CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds {
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds::new(reports, evidence).unwrap()
    }

    #[test]
    fn zero_bounds_empty_construction_and_constructible_extremes_are_exact() {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryConfigError::*;
        assert_eq!(CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds::new(0, 1), Err(ZeroMaximumReports));
        assert_eq!(CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds::new(1, 0), Err(ZeroMaximumEvidenceEntries));
        let history = CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(bounds(usize::MAX, u64::MAX));
        assert!(history.reports().is_empty());
        assert_eq!(history.totals(), Default::default());
        assert_eq!(history.totals().report_count(), 0);
        assert_eq!(history.totals().evidence_count(), 0);
    }

    #[test]
    fn first_middle_final_repeated_order_and_consuming_identity_are_preserved() {
        let first = report(1);
        let middle = report(2);
        let final_report = report(1);
        let ids = [identity(&first), identity(&middle), identity(&final_report)];
        let history = CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(bounds(3, 24))
            .append(first).unwrap().append(middle).unwrap().append(final_report).unwrap();
        assert_eq!(history.totals().report_count(), 3);
        assert_eq!(history.totals().evidence_count(), 24);
        assert_eq!(history.reports().iter().map(identity).collect::<Vec<_>>(), ids);
        assert_eq!(history.reports()[0], history.reports()[2]);
        assert_eq!(history.into_reports().iter().map(identity).collect::<Vec<_>>(), ids);
    }

    #[test]
    fn full_rejection_returns_unchanged_live_history_and_incoming_report() {
        let kept = report(10);
        let incoming = report(20);
        let kept_id = identity(&kept);
        let incoming_id = identity(&incoming);
        let error = CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(bounds(1, 16))
            .append(kept).unwrap().append(incoming).unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError::ReportLimit { limit: 1, required: 2, .. }));
        let (history, incoming) = error.into_parts();
        assert_eq!(history.reports().len(), 1);
        assert_eq!(history.totals().evidence_count(), 8);
        assert_eq!(identity(&history.reports()[0]), kept_id);
        assert_eq!(identity(&incoming), incoming_id);

        let incoming = report(21);
        let incoming_id = identity(&incoming);
        let error =
            CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(bounds(2, 8))
                .append(report(11))
                .unwrap()
                .append(incoming)
                .unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryAppendError::EvidenceLimit {
                limit: 8,
                required: 16,
                ..
            }
        ));
        let (history, incoming) = error.into_parts();
        assert_eq!(history.reports().len(), 1);
        assert_eq!(history.totals().evidence_count(), 8);
        assert_eq!(identity(&incoming), incoming_id);
    }

    #[test]
    fn injected_fallible_seams_and_checked_arithmetic_are_transactional() {
        for failure in 0..4 {
            let kept = report(30);
            let incoming = report(40 + failure);
            let kept_id = identity(&kept);
            let incoming_id = identity(&incoming);
            let mut history = CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(bounds(2, u64::MAX)).append(kept).unwrap();
            if failure == 2 { history.totals.evidence_count = u64::MAX; }
            let error = history.append_with(
                incoming,
                |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                |value| if failure == 1 { Err(()) } else { u64::try_from(value).map_err(|_| ()) },
                |left, right| left.checked_add(right).ok_or(()),
                |left, right| if failure == 0 { Err(()) } else { left.checked_add(right).ok_or(()) },
            ).unwrap_err();
            let (history, incoming) = error.into_parts();
            assert_eq!(history.reports().len(), 1);
            assert_eq!(identity(&history.reports()[0]), kept_id);
            assert_eq!(identity(&incoming), incoming_id);
        }
    }

    #[test]
    fn boundary_is_private_inert_non_authoritative_and_non_equivalent() {
        let source = include_str!("caller_requested_float32_retained_comparative_snapshot_report_history.rs");
        for wording in ["crate-private", "default-inert", "infers no loss", "continuity", "causality", "not liblsl-equivalent", "Manifold"] { assert!(source.contains(wording)); }
        for operation in [concat!("fn ap", "ply("), concat!("fn act", "ivate("), concat!("fn auth", "orize(")] { assert!(!source.contains(operation)); }
        assert!(!include_str!("runtime.rs").contains("CallerRequestedFloat32RetainedComparativeSnapshotReportHistory"));
        assert!(!include_str!("lib.rs").contains("pub use caller_requested_float32_retained_comparative_snapshot_report_history"));
    }
}
