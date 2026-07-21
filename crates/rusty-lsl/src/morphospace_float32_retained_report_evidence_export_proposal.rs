// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded Morphospace advisory export proposal over actual P50/P51 evidence.
//!
//! This crate-private, caller-requested owner retains the complete report
//! history and every complete evidence page, including their original nested
//! allocations. It creates only a deterministic manifest of existing `Copy`
//! evidence in caller, report, and evidence order. Construction is fallible,
//! bounded, transactional, and fail-closed. The result is default-inert,
//! advisory, and non-applying: it is not a public serialization protocol,
//! infers no loss, continuity, or causality, claims no liblsl equivalence, and
//! grants no Manifold, session, stream, transport, control, application,
//! activation, routing, admission, device, oracle, discovery, or policy
//! authority.

use crate::caller_requested_float32_retained_comparative_snapshot_report::CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence;
use crate::caller_requested_float32_retained_comparative_snapshot_report_evidence_page::CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage;
use crate::caller_requested_float32_retained_comparative_snapshot_report_history::CallerRequestedFloat32RetainedComparativeSnapshotReportHistory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedReportEvidenceExportProposalConfigError {
    ZeroMaximumReports,
    ZeroMaximumPages,
    ZeroMaximumEvidence,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedReportEvidenceExportProposalBounds {
    maximum_reports: usize,
    maximum_pages: usize,
    maximum_evidence: usize,
}

impl MorphospaceFloat32RetainedReportEvidenceExportProposalBounds {
    pub(crate) fn new(
        maximum_reports: usize,
        maximum_pages: usize,
        maximum_evidence: usize,
    ) -> Result<Self, MorphospaceFloat32RetainedReportEvidenceExportProposalConfigError> {
        use MorphospaceFloat32RetainedReportEvidenceExportProposalConfigError::*;
        for (value, zero) in [
            (maximum_reports, ZeroMaximumReports),
            (maximum_pages, ZeroMaximumPages),
            (maximum_evidence, ZeroMaximumEvidence),
        ] {
            if value == 0 {
                return Err(zero);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_reports,
            maximum_pages,
            maximum_evidence,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedReportEvidenceExportEntry {
    History {
        export_index: u64,
        report_index: u64,
        evidence_index: u64,
        evidence: CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
    },
    Page {
        export_index: u64,
        page_index: u64,
        page_evidence_index: u64,
        source_start: u64,
        source_end: u64,
        source_total: u64,
        evidence: CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedReportEvidenceExportProposal {
    history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    entries: Vec<MorphospaceFloat32RetainedReportEvidenceExportEntry>,
}

impl MorphospaceFloat32RetainedReportEvidenceExportProposal {
    pub(crate) const fn history(
        &self,
    ) -> &CallerRequestedFloat32RetainedComparativeSnapshotReportHistory {
        &self.history
    }

    pub(crate) fn pages(
        &self,
    ) -> &[CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage] {
        &self.pages
    }

    pub(crate) fn entries(&self) -> &[MorphospaceFloat32RetainedReportEvidenceExportEntry] {
        &self.entries
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    ) {
        (self.history, self.pages)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedReportEvidenceExportProposalError {
    ReportLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
    PageLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
    EvidenceCountOverflow {
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
    EvidenceLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
    CountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
    IndexUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
    Allocation {
        requested: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    },
}

impl MorphospaceFloat32RetainedReportEvidenceExportProposalError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    ) {
        use MorphospaceFloat32RetainedReportEvidenceExportProposalError::*;
        match self {
            ReportLimit { history, pages, .. }
            | PageLimit { history, pages, .. }
            | EvidenceCountOverflow { history, pages }
            | EvidenceLimit { history, pages, .. }
            | CountUnrepresentable { history, pages, .. }
            | IndexUnrepresentable { history, pages, .. }
            | Allocation { history, pages, .. } => (history, pages),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedReportEvidenceExportProposalOwner {
    bounds: MorphospaceFloat32RetainedReportEvidenceExportProposalBounds,
}

impl MorphospaceFloat32RetainedReportEvidenceExportProposalOwner {
    pub(crate) const fn new(
        bounds: MorphospaceFloat32RetainedReportEvidenceExportProposalBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn propose(
        &self,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
    ) -> Result<
        MorphospaceFloat32RetainedReportEvidenceExportProposal,
        MorphospaceFloat32RetainedReportEvidenceExportProposalError,
    > {
        self.propose_with(
            history,
            pages,
            |entries, count| entries.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn propose_with<R, C, A>(
        &self,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        pages: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage>,
        reserve: R,
        mut convert: C,
        mut add: A,
    ) -> Result<
        MorphospaceFloat32RetainedReportEvidenceExportProposal,
        MorphospaceFloat32RetainedReportEvidenceExportProposalError,
    >
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32RetainedReportEvidenceExportEntry>,
            usize,
        ) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32RetainedReportEvidenceExportProposalError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history, pages }) }; }

        let report_count = history.reports().len();
        let page_count = pages.len();
        if report_count > self.bounds.maximum_reports {
            fail!(ReportLimit {
                limit: self.bounds.maximum_reports,
                actual: report_count
            });
        }
        if page_count > self.bounds.maximum_pages {
            fail!(PageLimit {
                limit: self.bounds.maximum_pages,
                actual: page_count
            });
        }
        let mut required = 0usize;
        for report in history.reports() {
            required = match add(required, report.evidence().len()) {
                Ok(value) => value,
                Err(()) => fail!(EvidenceCountOverflow {}),
            };
        }
        for page in &pages {
            required = match add(required, page.evidence().len()) {
                Ok(value) => value,
                Err(()) => fail!(EvidenceCountOverflow {}),
            };
        }
        if required > self.bounds.maximum_evidence {
            fail!(EvidenceLimit {
                limit: self.bounds.maximum_evidence,
                required: required
            });
        }
        if convert(required).is_err() {
            fail!(CountUnrepresentable { actual: required });
        }

        // Preflight every index before allocation or manifest mutation.
        let mut export_index = 0usize;
        for (report_index, report) in history.reports().iter().enumerate() {
            if convert(report_index).is_err() {
                fail!(IndexUnrepresentable {
                    actual: report_index
                });
            }
            for evidence_index in 0..report.evidence().len() {
                if convert(evidence_index).is_err() {
                    fail!(IndexUnrepresentable {
                        actual: evidence_index
                    });
                }
                if convert(export_index).is_err() {
                    fail!(IndexUnrepresentable {
                        actual: export_index
                    });
                }
                export_index = match add(export_index, 1) {
                    Ok(value) => value,
                    Err(()) => fail!(EvidenceCountOverflow {}),
                };
            }
        }
        for (page_index, page) in pages.iter().enumerate() {
            if convert(page_index).is_err() {
                fail!(IndexUnrepresentable { actual: page_index });
            }
            for page_evidence_index in 0..page.evidence().len() {
                if convert(page_evidence_index).is_err() {
                    fail!(IndexUnrepresentable {
                        actual: page_evidence_index
                    });
                }
                if convert(export_index).is_err() {
                    fail!(IndexUnrepresentable {
                        actual: export_index
                    });
                }
                export_index = match add(export_index, 1) {
                    Ok(value) => value,
                    Err(()) => fail!(EvidenceCountOverflow {}),
                };
            }
        }

        let mut entries = Vec::new();
        if reserve(&mut entries, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        let mut export_index = 0u64;
        for (report_index, report) in history.reports().iter().enumerate() {
            for (evidence_index, evidence) in report.evidence().iter().copied().enumerate() {
                entries.push(
                    MorphospaceFloat32RetainedReportEvidenceExportEntry::History {
                        export_index,
                        report_index: report_index as u64,
                        evidence_index: evidence_index as u64,
                        evidence,
                    },
                );
                export_index += 1;
            }
        }
        for (page_index, page) in pages.iter().enumerate() {
            for (page_evidence_index, evidence) in page.evidence().iter().copied().enumerate() {
                entries.push(MorphospaceFloat32RetainedReportEvidenceExportEntry::Page {
                    export_index,
                    page_index: page_index as u64,
                    page_evidence_index: page_evidence_index as u64,
                    source_start: page.start(),
                    source_end: page.end(),
                    source_total: page.total(),
                    evidence,
                });
                export_index += 1;
            }
        }
        Ok(MorphospaceFloat32RetainedReportEvidenceExportProposal {
            history,
            pages,
            entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_retained_comparative_snapshot_report::{
        CallerRequestedFloat32RetainedComparativeSnapshotReport,
        CallerRequestedFloat32RetainedComparativeSnapshotReportBounds,
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner,
    };
    use crate::caller_requested_float32_retained_comparative_snapshot_report_evidence_page::{
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds,
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner,
    };
    use crate::caller_requested_float32_retained_comparative_snapshot_report_history::{
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds,
    };

    fn report() -> CallerRequestedFloat32RetainedComparativeSnapshotReport {
        let (history, package) = crate::tests::p50_actual_inputs();
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportBounds::new(2, 8, 10).unwrap(),
        )
        .report(history, package)
        .unwrap()
    }
    fn history(count: usize) -> CallerRequestedFloat32RetainedComparativeSnapshotReportHistory {
        let mut history = CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds::new(
                count.max(1),
                (count.max(1) * 10) as u64,
            )
            .unwrap(),
        );
        for _ in 0..count {
            history = history.append(report()).unwrap();
        }
        history
    }
    fn page(
        start: u64,
        maximum: usize,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage {
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds::new(maximum)
                .unwrap(),
        )
        .page(report(), start)
        .unwrap()
    }
    fn sample_pointer(
        report: &CallerRequestedFloat32RetainedComparativeSnapshotReport,
    ) -> *const f32 {
        report.delta_history().proposals()[0]
            .earlier()
            .history()
            .evidence()[0]
            .earlier()
            .history()
            .values()[0]
            .report()
            .sample()
            .sample()
            .values()
            .as_ptr()
    }
    fn owner(
        r: usize,
        p: usize,
        e: usize,
    ) -> MorphospaceFloat32RetainedReportEvidenceExportProposalOwner {
        MorphospaceFloat32RetainedReportEvidenceExportProposalOwner::new(
            MorphospaceFloat32RetainedReportEvidenceExportProposalBounds::new(r, p, e).unwrap(),
        )
    }

    #[test]
    fn zero_exact_and_one_past_bounds_are_typed() {
        use MorphospaceFloat32RetainedReportEvidenceExportProposalConfigError::*;
        assert_eq!(
            MorphospaceFloat32RetainedReportEvidenceExportProposalBounds::new(0, 1, 1),
            Err(ZeroMaximumReports)
        );
        assert_eq!(
            MorphospaceFloat32RetainedReportEvidenceExportProposalBounds::new(1, 0, 1),
            Err(ZeroMaximumPages)
        );
        assert_eq!(
            MorphospaceFloat32RetainedReportEvidenceExportProposalBounds::new(1, 1, 0),
            Err(ZeroMaximumEvidence)
        );
        let empty = owner(1, 1, 1).propose(history(0), Vec::new()).unwrap();
        assert!(empty.history().reports().is_empty());
        assert!(empty.pages().is_empty());
        assert!(empty.entries().is_empty());
        assert!(owner(2, 2, 25)
            .propose(history(2), vec![page(0, 3), page(7, 2)])
            .is_ok());
        assert!(matches!(
            owner(1, 2, 25).propose(history(2), vec![]),
            Err(
                MorphospaceFloat32RetainedReportEvidenceExportProposalError::ReportLimit {
                    actual: 2,
                    ..
                }
            )
        ));
        assert!(matches!(
            owner(2, 1, 25).propose(history(1), vec![page(0, 1), page(1, 1)]),
            Err(
                MorphospaceFloat32RetainedReportEvidenceExportProposalError::PageLimit {
                    actual: 2,
                    ..
                }
            )
        ));
        assert!(matches!(
            owner(2, 2, 24).propose(history(2), vec![page(0, 3), page(7, 2)]),
            Err(
                MorphospaceFloat32RetainedReportEvidenceExportProposalError::EvidenceLimit {
                    required: 25,
                    ..
                }
            )
        ));
    }

    #[test]
    fn multiple_reports_and_pages_preserve_order_and_allocation_identity() {
        let history = history(2);
        let pages = vec![page(4, 2), page(8, 2)];
        let history_ids: Vec<_> = history.reports().iter().map(sample_pointer).collect();
        let page_ids: Vec<_> = pages.iter().map(|p| sample_pointer(p.report())).collect();
        let proposal = owner(2, 2, 24).propose(history, pages).unwrap();
        assert_eq!(proposal.entries().len(), 24);
        for (index, entry) in proposal.entries().iter().enumerate() {
            match entry {
                MorphospaceFloat32RetainedReportEvidenceExportEntry::History {
                    export_index,
                    report_index,
                    evidence_index,
                    ..
                } => {
                    assert_eq!(*export_index, index as u64);
                    assert_eq!(*report_index, (index / 10) as u64);
                    assert_eq!(*evidence_index, (index % 10) as u64);
                }
                MorphospaceFloat32RetainedReportEvidenceExportEntry::Page {
                    export_index,
                    page_index,
                    page_evidence_index,
                    source_start,
                    ..
                } => {
                    assert_eq!(*export_index, index as u64);
                    let local = index - 20;
                    assert_eq!(*page_index, (local / 2) as u64);
                    assert_eq!(*page_evidence_index, (local % 2) as u64);
                    assert_eq!(*source_start, if local < 2 { 4 } else { 8 });
                }
            }
        }
        assert_eq!(
            proposal
                .history()
                .reports()
                .iter()
                .map(sample_pointer)
                .collect::<Vec<_>>(),
            history_ids
        );
        assert_eq!(
            proposal
                .pages()
                .iter()
                .map(|p| sample_pointer(p.report()))
                .collect::<Vec<_>>(),
            page_ids
        );
        let (history, pages) = proposal.into_parts();
        assert_eq!(
            history
                .reports()
                .iter()
                .map(sample_pointer)
                .collect::<Vec<_>>(),
            history_ids
        );
        assert_eq!(
            pages
                .iter()
                .map(|p| sample_pointer(p.report()))
                .collect::<Vec<_>>(),
            page_ids
        );
    }

    #[test]
    fn overflow_conversion_allocation_and_rollback_return_complete_sources() {
        for failure in 0..4 {
            let history = history(1);
            let pages = vec![page(0, 2)];
            let history_id = sample_pointer(&history.reports()[0]);
            let page_id = sample_pointer(pages[0].report());
            let error = owner(1, 1, 12)
                .propose_with(
                    history,
                    pages,
                    |_, requested| {
                        if failure == 3 {
                            assert_eq!(requested, 12);
                            Err(())
                        } else {
                            Ok(())
                        }
                    },
                    |value| {
                        if failure == 1 {
                            Err(())
                        } else {
                            u64::try_from(value).map_err(|_| ())
                        }
                    },
                    |left, right| {
                        if failure == 0 || (failure == 2 && right == 1) {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            assert!(matches!((&error, failure), (MorphospaceFloat32RetainedReportEvidenceExportProposalError::EvidenceCountOverflow { .. }, 0 | 2) | (MorphospaceFloat32RetainedReportEvidenceExportProposalError::CountUnrepresentable { actual: 12, .. }, 1) | (MorphospaceFloat32RetainedReportEvidenceExportProposalError::Allocation { requested: 12, .. }, 3)));
            let (history, pages) = error.into_parts();
            assert_eq!(sample_pointer(&history.reports()[0]), history_id);
            assert_eq!(sample_pointer(pages[0].report()), page_id);
        }
    }

    #[test]
    fn boundary_is_private_default_inert_advisory_and_non_applying() {
        let source =
            include_str!("morphospace_float32_retained_report_evidence_export_proposal.rs");
        for wording in [
            "crate-private",
            "default-inert",
            "advisory",
            "non-applying",
            "not a public serialization protocol",
            "infers no loss",
            "continuity",
            "causality",
            "no liblsl equivalence",
            "Manifold",
        ] {
            assert!(source.contains(wording));
        }
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn auth", "orize("),
            concat!("serde", "::"),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(!include_str!("runtime.rs")
            .contains("MorphospaceFloat32RetainedReportEvidenceExportProposal"));
        assert!(!include_str!("lib.rs")
            .contains("pub use morphospace_float32_retained_report_evidence_export_proposal"));
    }
}
