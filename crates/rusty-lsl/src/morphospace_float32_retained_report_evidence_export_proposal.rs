// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded Morphospace advisory export proposal over completed cursor output.
//!
//! This crate-private owner consumes one completed P51 cursor together with
//! the exact cursor pages retained by its caller. The cursor remains the sole
//! report-history owner. Construction validates completion and the complete,
//! ordered page composition before producing a deterministic manifest of
//! existing `Copy` evidence facts. Every report, page, outer evidence vector,
//! and nested sample allocation remains owned exactly once by the output.
//!
//! Construction is fallible, bounded, transactional, and fail-closed. Every
//! error returns the unchanged cursor-owned input. The result is default-inert,
//! advisory, and non-applying: it is not a public serialization protocol,
//! infers no loss, continuity, or causality, claims no liblsl equivalence, and
//! grants no Manifold, session, stream, transport, control, application,
//! activation, routing, admission, device, oracle, discovery, or policy
//! authority.

use crate::caller_requested_float32_retained_comparative_snapshot_report::CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence;
use crate::caller_requested_float32_retained_report_evidence_cursor::{
    CallerRequestedFloat32RetainedReportEvidenceCursor,
    CallerRequestedFloat32RetainedReportEvidenceCursorFacts,
    CallerRequestedFloat32RetainedReportEvidenceCursorPage,
};

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

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedReportEvidenceCursorOutput {
    cursor: CallerRequestedFloat32RetainedReportEvidenceCursor,
    pages: Vec<CallerRequestedFloat32RetainedReportEvidenceCursorPage>,
}

impl MorphospaceFloat32RetainedReportEvidenceCursorOutput {
    pub(crate) const fn new(
        cursor: CallerRequestedFloat32RetainedReportEvidenceCursor,
        pages: Vec<CallerRequestedFloat32RetainedReportEvidenceCursorPage>,
    ) -> Self {
        Self { cursor, pages }
    }

    pub(crate) const fn cursor(&self) -> &CallerRequestedFloat32RetainedReportEvidenceCursor {
        &self.cursor
    }

    pub(crate) fn pages(&self) -> &[CallerRequestedFloat32RetainedReportEvidenceCursorPage] {
        &self.pages
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32RetainedReportEvidenceCursor,
        Vec<CallerRequestedFloat32RetainedReportEvidenceCursorPage>,
    ) {
        (self.cursor, self.pages)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedReportEvidenceExportEntry {
    export_index: u64,
    report_index: u64,
    page_index: u64,
    page_evidence_index: u64,
    source_start: u64,
    source_end: u64,
    source_total: u64,
    evidence: CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
}

impl MorphospaceFloat32RetainedReportEvidenceExportEntry {
    pub(crate) const fn export_index(self) -> u64 {
        self.export_index
    }
    pub(crate) const fn report_index(self) -> u64 {
        self.report_index
    }
    pub(crate) const fn page_index(self) -> u64 {
        self.page_index
    }
    pub(crate) const fn page_evidence_index(self) -> u64 {
        self.page_evidence_index
    }
    pub(crate) const fn source_start(self) -> u64 {
        self.source_start
    }
    pub(crate) const fn source_end(self) -> u64 {
        self.source_end
    }
    pub(crate) const fn source_total(self) -> u64 {
        self.source_total
    }
    pub(crate) const fn evidence(
        self,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence {
        self.evidence
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32RetainedReportEvidenceExportProposal {
    source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    completed_facts: CallerRequestedFloat32RetainedReportEvidenceCursorFacts,
    entries: Vec<MorphospaceFloat32RetainedReportEvidenceExportEntry>,
}

impl MorphospaceFloat32RetainedReportEvidenceExportProposal {
    pub(crate) const fn source(&self) -> &MorphospaceFloat32RetainedReportEvidenceCursorOutput {
        &self.source
    }
    pub(crate) const fn completed_facts(
        &self,
    ) -> CallerRequestedFloat32RetainedReportEvidenceCursorFacts {
        self.completed_facts
    }
    pub(crate) fn entries(&self) -> &[MorphospaceFloat32RetainedReportEvidenceExportEntry] {
        &self.entries
    }
    pub(crate) fn into_source(self) -> MorphospaceFloat32RetainedReportEvidenceCursorOutput {
        self.source
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedReportEvidenceCompositionFailure {
    IncompleteCursor,
    PageCountMismatch {
        recorded: u64,
        actual: usize,
    },
    PageReportMissing {
        page_position: usize,
        report_index: u64,
    },
    PageSequenceMismatch {
        page_position: usize,
    },
    PageRangeInvalid {
        page_position: usize,
    },
    ForeignEvidence {
        page_position: usize,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32RetainedReportEvidenceExportProposalError {
    Composition {
        failure: MorphospaceFloat32RetainedReportEvidenceCompositionFailure,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    ReportLimit {
        limit: usize,
        actual: usize,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    PageLimit {
        limit: usize,
        actual: usize,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    EvidenceCountOverflow {
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    EvidenceLimit {
        limit: usize,
        required: usize,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    CountUnrepresentable {
        actual: usize,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    IndexUnrepresentable {
        actual: usize,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
    Allocation {
        requested: usize,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    },
}

impl MorphospaceFloat32RetainedReportEvidenceExportProposalError {
    pub(crate) fn into_source(self) -> MorphospaceFloat32RetainedReportEvidenceCursorOutput {
        use MorphospaceFloat32RetainedReportEvidenceExportProposalError::*;
        match self {
            Composition { source, .. }
            | ReportLimit { source, .. }
            | PageLimit { source, .. }
            | EvidenceCountOverflow { source }
            | EvidenceLimit { source, .. }
            | CountUnrepresentable { source, .. }
            | IndexUnrepresentable { source, .. }
            | Allocation { source, .. } => source,
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
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    ) -> Result<
        MorphospaceFloat32RetainedReportEvidenceExportProposal,
        MorphospaceFloat32RetainedReportEvidenceExportProposalError,
    > {
        self.propose_with(
            source,
            |entries, count| entries.try_reserve_exact(count).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn propose_with<R, C, A>(
        &self,
        source: MorphospaceFloat32RetainedReportEvidenceCursorOutput,
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
        use MorphospaceFloat32RetainedReportEvidenceCompositionFailure as CF;
        use MorphospaceFloat32RetainedReportEvidenceExportProposalError as E;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err(E::$variant { $($field: $value,)* source }) }; }

        let facts = source.cursor.facts();
        if facts.current().is_some()
            || facts.completed_evidence() != facts.total_evidence()
            || facts.remaining_evidence() != 0
        {
            fail!(Composition {
                failure: CF::IncompleteCursor
            });
        }
        let report_count = source.cursor.reports().len();
        let page_count = source.pages.len();
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
        if usize::try_from(source.cursor.total_pages()).ok() != Some(page_count) {
            fail!(Composition {
                failure: CF::PageCountMismatch {
                    recorded: source.cursor.total_pages(),
                    actual: page_count
                }
            });
        }

        let mut expected_report = 0usize;
        let mut expected_page = 0u64;
        let mut expected_start = 0u64;
        let mut required = 0usize;
        for (page_position, page) in source.pages.iter().enumerate() {
            while expected_report < source.cursor.reports().len()
                && source.cursor.reports()[expected_report]
                    .evidence()
                    .is_empty()
            {
                expected_report += 1;
            }
            let Some(report) = source.cursor.reports().get(expected_report) else {
                fail!(Composition {
                    failure: CF::PageReportMissing {
                        page_position,
                        report_index: page.report_index()
                    }
                });
            };
            if u64::try_from(expected_report).ok() != Some(page.report_index())
                || page.page_index() != expected_page
                || page.start() != expected_start
            {
                fail!(Composition {
                    failure: CF::PageSequenceMismatch { page_position }
                });
            }
            let Ok(start) = usize::try_from(page.start()) else {
                fail!(Composition {
                    failure: CF::PageRangeInvalid { page_position }
                });
            };
            let Ok(end) = usize::try_from(page.end()) else {
                fail!(Composition {
                    failure: CF::PageRangeInvalid { page_position }
                });
            };
            let Ok(total) = usize::try_from(page.total()) else {
                fail!(Composition {
                    failure: CF::PageRangeInvalid { page_position }
                });
            };
            if start > end
                || end > total
                || total != report.evidence().len()
                || end - start != page.evidence().len()
            {
                fail!(Composition {
                    failure: CF::PageRangeInvalid { page_position }
                });
            }
            if page.evidence() != &report.evidence()[start..end] {
                fail!(Composition {
                    failure: CF::ForeignEvidence { page_position }
                });
            }
            required = match add(required, page.evidence().len()) {
                Ok(value) => value,
                Err(()) => fail!(EvidenceCountOverflow {}),
            };
            if end == total {
                expected_report += 1;
                expected_page = 0;
                expected_start = 0;
            } else {
                expected_page = match expected_page.checked_add(1) {
                    Some(value) => value,
                    None => fail!(EvidenceCountOverflow {}),
                };
                expected_start = page.end();
            }
        }
        while expected_report < source.cursor.reports().len()
            && source.cursor.reports()[expected_report]
                .evidence()
                .is_empty()
        {
            expected_report += 1;
        }
        if expected_report != source.cursor.reports().len()
            || required != usize::try_from(facts.total_evidence()).unwrap_or(usize::MAX)
        {
            fail!(Composition {
                failure: CF::PageCountMismatch {
                    recorded: source.cursor.total_pages(),
                    actual: page_count
                }
            });
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
        for (page_position, page) in source.pages.iter().enumerate() {
            if convert(page_position).is_err() {
                fail!(IndexUnrepresentable {
                    actual: page_position
                });
            }
            for evidence_position in 0..page.evidence().len() {
                if convert(evidence_position).is_err() {
                    fail!(IndexUnrepresentable {
                        actual: evidence_position
                    });
                }
            }
        }
        let mut entries = Vec::new();
        if reserve(&mut entries, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        let mut export_index = 0u64;
        for page in &source.pages {
            for (page_evidence_index, evidence) in page.evidence().iter().copied().enumerate() {
                entries.push(MorphospaceFloat32RetainedReportEvidenceExportEntry {
                    export_index,
                    report_index: page.report_index(),
                    page_index: page.page_index(),
                    page_evidence_index: page_evidence_index as u64,
                    source_start: page.start(),
                    source_end: page.end(),
                    source_total: page.total(),
                    evidence,
                });
                export_index = export_index
                    .checked_add(1)
                    .expect("preflight admitted the complete manifest extent");
            }
        }
        Ok(MorphospaceFloat32RetainedReportEvidenceExportProposal {
            source,
            completed_facts: facts,
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
    use crate::caller_requested_float32_retained_comparative_snapshot_report_history::{
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds,
    };
    use crate::caller_requested_float32_retained_report_evidence_cursor::{
        CallerRequestedFloat32RetainedReportEvidenceCursorAdvance,
        CallerRequestedFloat32RetainedReportEvidenceCursorBounds,
    };
    use std::cell::Cell;

    fn report() -> CallerRequestedFloat32RetainedComparativeSnapshotReport {
        let (history, package) = crate::tests::p50_actual_inputs();
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportBounds::new(2, 8, 10).unwrap(),
        )
        .report(history, package)
        .unwrap()
    }
    fn actual_cursor(
        count: usize,
        page_length: usize,
    ) -> CallerRequestedFloat32RetainedReportEvidenceCursor {
        let mut history = CallerRequestedFloat32RetainedComparativeSnapshotReportHistory::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds::new(
                count.max(1),
                u64::try_from(count.max(1) * 10).unwrap(),
            )
            .unwrap(),
        );
        for _ in 0..count {
            history = history.append(report()).unwrap();
        }
        CallerRequestedFloat32RetainedReportEvidenceCursor::new(
            CallerRequestedFloat32RetainedReportEvidenceCursorBounds::new(page_length).unwrap(),
            history,
        )
        .unwrap()
    }
    fn actual_source(
        count: usize,
        page_length: usize,
    ) -> MorphospaceFloat32RetainedReportEvidenceCursorOutput {
        let mut cursor = actual_cursor(count, page_length);
        let mut pages = Vec::new();
        loop {
            match cursor.advance().unwrap() {
                CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Page {
                    cursor: next,
                    page,
                } => {
                    pages.push(page);
                    cursor = next;
                }
                CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Complete(done) => {
                    return MorphospaceFloat32RetainedReportEvidenceCursorOutput::new(done, pages)
                }
            }
        }
    }
    fn identities(
        source: &MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    ) -> Vec<(
        *const CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
        *const f32,
    )> {
        source
            .cursor()
            .reports()
            .iter()
            .map(|report| {
                (
                    report.evidence().as_ptr(),
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
                        .as_ptr(),
                )
            })
            .collect()
    }
    fn page_ids(
        source: &MorphospaceFloat32RetainedReportEvidenceCursorOutput,
    ) -> Vec<*const CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence> {
        source
            .pages()
            .iter()
            .map(|page| page.evidence().as_ptr())
            .collect()
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
    fn actual_p50_p51_cursor_output_exports_exact_order_facts_and_identity() {
        let source = actual_source(2, 4);
        let report_ids = identities(&source);
        let retained_page_ids = page_ids(&source);
        let expected: Vec<_> = source
            .cursor()
            .reports()
            .iter()
            .flat_map(|r| r.evidence().iter().copied())
            .collect();
        let proposal = owner(2, 6, 20).propose(source).unwrap();
        assert_eq!(
            (
                proposal.completed_facts().report_count(),
                proposal.completed_facts().total_evidence(),
                proposal.completed_facts().completed_evidence(),
                proposal.completed_facts().current(),
                proposal.completed_facts().remaining_evidence()
            ),
            (2, 20, 20, None, 0)
        );
        assert_eq!(
            proposal
                .entries()
                .iter()
                .map(|e| e.evidence())
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            proposal
                .entries()
                .iter()
                .map(|e| (
                    e.export_index(),
                    e.report_index(),
                    e.page_index(),
                    e.page_evidence_index(),
                    e.source_start(),
                    e.source_end(),
                    e.source_total()
                ))
                .collect::<Vec<_>>(),
            vec![
                (0, 0, 0, 0, 0, 4, 10),
                (1, 0, 0, 1, 0, 4, 10),
                (2, 0, 0, 2, 0, 4, 10),
                (3, 0, 0, 3, 0, 4, 10),
                (4, 0, 1, 0, 4, 8, 10),
                (5, 0, 1, 1, 4, 8, 10),
                (6, 0, 1, 2, 4, 8, 10),
                (7, 0, 1, 3, 4, 8, 10),
                (8, 0, 2, 0, 8, 10, 10),
                (9, 0, 2, 1, 8, 10, 10),
                (10, 1, 0, 0, 0, 4, 10),
                (11, 1, 0, 1, 0, 4, 10),
                (12, 1, 0, 2, 0, 4, 10),
                (13, 1, 0, 3, 0, 4, 10),
                (14, 1, 1, 0, 4, 8, 10),
                (15, 1, 1, 1, 4, 8, 10),
                (16, 1, 1, 2, 4, 8, 10),
                (17, 1, 1, 3, 4, 8, 10),
                (18, 1, 2, 0, 8, 10, 10),
                (19, 1, 2, 1, 8, 10, 10)
            ]
        );
        assert_eq!(identities(proposal.source()), report_ids);
        assert_eq!(page_ids(proposal.source()), retained_page_ids);
        let source = proposal.into_source();
        assert_eq!(identities(&source), report_ids);
        assert_eq!(page_ids(&source), retained_page_ids);
    }

    #[test]
    fn incomplete_and_damaged_compositions_return_unchanged_owner() {
        let incomplete = {
            let cursor = actual_cursor(1, 4);
            let (cursor, page) = match cursor.advance().unwrap() {
                CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Page {
                    cursor,
                    page,
                } => (cursor, page),
                _ => unreachable!(),
            };
            MorphospaceFloat32RetainedReportEvidenceCursorOutput::new(cursor, vec![page])
        };
        let ids = identities(&incomplete);
        let pids = page_ids(&incomplete);
        let error = owner(1, 3, 10).propose(incomplete).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32RetainedReportEvidenceExportProposalError::Composition { .. }
        ));
        let source = error.into_source();
        assert_eq!(identities(&source), ids);
        assert_eq!(page_ids(&source), pids);
        for damage in 0..4 {
            let mut source = actual_source(2, 4);
            if damage == 0 {
                source.pages.swap(0, 1);
            } else if damage == 1 {
                source.pages.remove(1);
            } else if damage == 2 {
                let duplicate = actual_source(1, 4).pages.into_iter().next().unwrap();
                source.pages[1] = duplicate;
            } else {
                let foreign = actual_source(1, 3).pages.into_iter().next().unwrap();
                source.pages[0] = foreign;
            }
            let ids = identities(&source);
            let pids = page_ids(&source);
            let error = owner(2, 6, 20).propose(source).unwrap_err();
            let source = error.into_source();
            assert_eq!(identities(&source), ids);
            assert_eq!(page_ids(&source), pids);
        }
    }

    #[test]
    fn bounds_conversion_arithmetic_and_allocation_rollback_are_exact() {
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
        for failure in 0..6 {
            let source = actual_source(1, 4);
            let ids = identities(&source);
            let pids = page_ids(&source);
            let calls = Cell::new(0);
            let result = if failure == 0 {
                owner(1, 2, 10).propose(source)
            } else if failure == 1 {
                owner(1, 3, 9).propose(source)
            } else {
                owner(1, 3, 10).propose_with(
                    source,
                    |_, requested| {
                        if failure == 5 {
                            assert_eq!(requested, 10);
                            Err(())
                        } else {
                            Ok(())
                        }
                    },
                    |value| {
                        let call = calls.get();
                        calls.set(call + 1);
                        if (failure == 3 && call == 0) || (failure == 4 && call == 1) {
                            Err(())
                        } else {
                            u64::try_from(value).map_err(|_| ())
                        }
                    },
                    |left, right| {
                        if failure == 2 {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                )
            };
            let source = result.unwrap_err().into_source();
            assert_eq!(identities(&source), ids);
            assert_eq!(page_ids(&source), pids);
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
