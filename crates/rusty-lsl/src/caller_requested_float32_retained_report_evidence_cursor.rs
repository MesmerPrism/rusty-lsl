// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Transactional caller-requested cursor over retained Float32 report evidence.
//!
//! This crate-private, default-inert session consumes one actual P51 report
//! history and uses the actual P51 evidence-page owner for each advance. The
//! complete reports remain owned in caller order, including every outer and
//! nested sample allocation. Only existing `Copy` evidence facts enter a
//! returned page. Exact report and evidence positions are descriptive facts;
//! this owner infers no continuity, missing evidence, loss, or causality.
//!
//! The cursor is advisory and non-applying. It adds no public export, runtime
//! activation, application, liblsl equivalence, or Manifold session, stream,
//! transport, control, routing, admission, policy, device, or oracle authority.

use crate::caller_requested_float32_retained_comparative_snapshot_report::{
    CallerRequestedFloat32RetainedComparativeSnapshotReport,
    CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
};
use crate::caller_requested_float32_retained_comparative_snapshot_report_evidence_page::{
    CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage,
    CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds,
    CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError,
    CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner,
};
use crate::caller_requested_float32_retained_comparative_snapshot_report_history::CallerRequestedFloat32RetainedComparativeSnapshotReportHistory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedReportEvidenceCursorConfigError {
    ZeroMaximumPageLength,
    PageBoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedReportEvidenceCursorBounds {
    maximum_page_length: usize,
    maximum_page_length_u64: u64,
}

impl CallerRequestedFloat32RetainedReportEvidenceCursorBounds {
    pub(crate) fn new(
        maximum_page_length: usize,
    ) -> Result<Self, CallerRequestedFloat32RetainedReportEvidenceCursorConfigError> {
        if maximum_page_length == 0 {
            return Err(
                CallerRequestedFloat32RetainedReportEvidenceCursorConfigError::ZeroMaximumPageLength,
            );
        }
        let maximum_page_length_u64 = u64::try_from(maximum_page_length).map_err(|_| {
            CallerRequestedFloat32RetainedReportEvidenceCursorConfigError::PageBoundUnrepresentable {
                requested: maximum_page_length,
            }
        })?;
        Ok(Self {
            maximum_page_length,
            maximum_page_length_u64,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedReportEvidencePosition {
    report_index: u64,
    page_index: u64,
    evidence_start: u64,
}

impl CallerRequestedFloat32RetainedReportEvidencePosition {
    pub(crate) const fn report_index(self) -> u64 {
        self.report_index
    }

    pub(crate) const fn page_index(self) -> u64 {
        self.page_index
    }

    pub(crate) const fn evidence_start(self) -> u64 {
        self.evidence_start
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedReportEvidenceCursorFacts {
    report_count: u64,
    total_evidence: u64,
    completed_evidence: u64,
    current: Option<CallerRequestedFloat32RetainedReportEvidencePosition>,
    remaining_evidence: u64,
}

impl CallerRequestedFloat32RetainedReportEvidenceCursorFacts {
    pub(crate) const fn report_count(self) -> u64 {
        self.report_count
    }

    pub(crate) const fn total_evidence(self) -> u64 {
        self.total_evidence
    }

    pub(crate) const fn completed_evidence(self) -> u64 {
        self.completed_evidence
    }

    pub(crate) const fn current(
        self,
    ) -> Option<CallerRequestedFloat32RetainedReportEvidencePosition> {
        self.current
    }

    pub(crate) const fn remaining_evidence(self) -> u64 {
        self.remaining_evidence
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError {
    ReportCountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    },
    EvidenceCountUnrepresentable {
        report_index: usize,
        actual: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    },
    EvidenceCountOverflow {
        report_index: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    },
    PageCountOverflow {
        report_index: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    },
    HistoryTotalsMismatch {
        recorded_reports: u64,
        calculated_reports: u64,
        recorded_evidence: u64,
        calculated_evidence: u64,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    },
    Allocation {
        requested_reports: usize,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    },
}

impl CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError {
    pub(crate) fn into_history(
        self,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotReportHistory {
        use CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError::*;
        match self {
            ReportCountUnrepresentable { history, .. }
            | EvidenceCountUnrepresentable { history, .. }
            | EvidenceCountOverflow { history, .. }
            | PageCountOverflow { history, .. }
            | HistoryTotalsMismatch { history, .. }
            | Allocation { history, .. } => history,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedReportEvidenceCursor {
    bounds: CallerRequestedFloat32RetainedReportEvidenceCursorBounds,
    reports: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReport>,
    report_evidence_starts: Vec<u64>,
    report_index: usize,
    page_index: u64,
    evidence_start: u64,
    total_evidence: u64,
    completed_evidence: u64,
    total_pages: u64,
}

impl CallerRequestedFloat32RetainedReportEvidenceCursor {
    pub(crate) fn new(
        bounds: CallerRequestedFloat32RetainedReportEvidenceCursorBounds,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
    ) -> Result<Self, CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError> {
        Self::new_with(
            bounds,
            history,
            |starts, requested| starts.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn new_with<R, C, A>(
        bounds: CallerRequestedFloat32RetainedReportEvidenceCursorBounds,
        history: CallerRequestedFloat32RetainedComparativeSnapshotReportHistory,
        reserve: R,
        convert: C,
        add: A,
    ) -> Result<Self, CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError>
    where
        R: FnOnce(&mut Vec<u64>, usize) -> Result<(), ()>,
        C: Fn(usize) -> Result<u64, ()>,
        A: Fn(u64, u64) -> Result<u64, ()>,
    {
        use CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError::*;
        let report_count_usize = history.reports().len();
        let report_count = match convert(report_count_usize) {
            Ok(value) => value,
            Err(()) => {
                return Err(ReportCountUnrepresentable {
                    actual: report_count_usize,
                    history,
                })
            }
        };
        let mut starts = Vec::new();
        if reserve(&mut starts, report_count_usize).is_err() {
            return Err(Allocation {
                requested_reports: report_count_usize,
                history,
            });
        }

        let mut total_evidence = 0_u64;
        let mut total_pages = 0_u64;
        for (report_index, report) in history.reports().iter().enumerate() {
            starts.push(total_evidence);
            let evidence_count = match convert(report.evidence().len()) {
                Ok(value) => value,
                Err(()) => {
                    return Err(EvidenceCountUnrepresentable {
                        report_index,
                        actual: report.evidence().len(),
                        history,
                    })
                }
            };
            total_evidence = match add(total_evidence, evidence_count) {
                Ok(value) => value,
                Err(()) => {
                    return Err(EvidenceCountOverflow {
                        report_index,
                        history,
                    })
                }
            };
            let pages = evidence_count
                .checked_add(bounds.maximum_page_length_u64 - 1)
                .and_then(|rounded| rounded.checked_div(bounds.maximum_page_length_u64))
                .ok_or(());
            total_pages = match pages.and_then(|pages| add(total_pages, pages)) {
                Ok(value) => value,
                Err(()) => {
                    return Err(PageCountOverflow {
                        report_index,
                        history,
                    })
                }
            };
        }
        let recorded = history.totals();
        if recorded.report_count() != report_count || recorded.evidence_count() != total_evidence {
            return Err(HistoryTotalsMismatch {
                recorded_reports: recorded.report_count(),
                calculated_reports: report_count,
                recorded_evidence: recorded.evidence_count(),
                calculated_evidence: total_evidence,
                history,
            });
        }

        let reports = history.into_reports();
        let mut cursor = Self {
            bounds,
            reports,
            report_evidence_starts: starts,
            report_index: 0,
            page_index: 0,
            evidence_start: 0,
            total_evidence,
            completed_evidence: 0,
            total_pages,
        };
        cursor.skip_empty_reports();
        Ok(cursor)
    }

    pub(crate) fn reports(&self) -> &[CallerRequestedFloat32RetainedComparativeSnapshotReport] {
        &self.reports
    }

    pub(crate) const fn total_pages(&self) -> u64 {
        self.total_pages
    }

    pub(crate) fn facts(&self) -> CallerRequestedFloat32RetainedReportEvidenceCursorFacts {
        let report_count = u64::try_from(self.reports.len())
            .expect("construction admitted a u64-representable report count");
        let current = (self.report_index < self.reports.len()).then_some(
            CallerRequestedFloat32RetainedReportEvidencePosition {
                report_index: u64::try_from(self.report_index)
                    .expect("construction admitted a u64-representable report index"),
                page_index: self.page_index,
                evidence_start: self.evidence_start,
            },
        );
        CallerRequestedFloat32RetainedReportEvidenceCursorFacts {
            report_count,
            total_evidence: self.total_evidence,
            completed_evidence: self.completed_evidence,
            current,
            remaining_evidence: self
                .total_evidence
                .checked_sub(self.completed_evidence)
                .expect("successful advancement cannot exceed admitted evidence"),
        }
    }

    pub(crate) fn advance(
        self,
    ) -> Result<
        CallerRequestedFloat32RetainedReportEvidenceCursorAdvance,
        CallerRequestedFloat32RetainedReportEvidenceCursorAdvanceError,
    > {
        self.advance_with(|owner, report, start| owner.page(report, start))
    }

    fn advance_with<E>(
        mut self,
        extract: E,
    ) -> Result<
        CallerRequestedFloat32RetainedReportEvidenceCursorAdvance,
        CallerRequestedFloat32RetainedReportEvidenceCursorAdvanceError,
    >
    where
        E: FnOnce(
            &CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner,
            CallerRequestedFloat32RetainedComparativeSnapshotReport,
            u64,
        ) -> Result<
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage,
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError,
        >,
    {
        if self.report_index >= self.reports.len() {
            return Ok(CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Complete(self));
        }

        let report_index = self.report_index;
        let page_index = self.page_index;
        let start = self.evidence_start;
        let report = self.reports.remove(report_index);
        let page_bounds =
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds::new(
                self.bounds.maximum_page_length,
            )
            .expect("cursor construction validated the identical P51 page bound");
        let owner = CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner::new(
            page_bounds,
        );
        let page = match extract(&owner, report, start) {
            Ok(page) => page,
            Err(source) => {
                let failure =
                    CallerRequestedFloat32RetainedReportEvidenceCursorPageFailure::from(&source);
                let report = source.into_report();
                self.reports.insert(report_index, report);
                return Err(
                    CallerRequestedFloat32RetainedReportEvidenceCursorAdvanceError {
                        report_index: u64::try_from(report_index)
                            .expect("construction admitted the current report index"),
                        page_index,
                        evidence_start: start,
                        failure,
                        cursor: self,
                    },
                );
            }
        };
        let end = page.end();
        let total = page.total();
        let (report, evidence) = page.into_parts();
        self.reports.insert(report_index, report);

        let length = u64::try_from(evidence.len())
            .expect("P51 page length was admitted by a u64-representable bound");
        self.completed_evidence = self
            .completed_evidence
            .checked_add(length)
            .expect("construction admitted the complete checked evidence extent");
        if end == total {
            self.report_index = self
                .report_index
                .checked_add(1)
                .expect("a live report index has a representable successor");
            self.page_index = 0;
            self.evidence_start = 0;
            self.skip_empty_reports();
        } else {
            self.page_index = self
                .page_index
                .checked_add(1)
                .expect("construction admitted the complete checked page extent");
            self.evidence_start = end;
        }

        Ok(
            CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Page {
                cursor: self,
                page: CallerRequestedFloat32RetainedReportEvidenceCursorPage {
                    report_index: u64::try_from(report_index)
                        .expect("construction admitted the page report index"),
                    page_index,
                    start,
                    end,
                    total,
                    evidence,
                },
            },
        )
    }

    fn skip_empty_reports(&mut self) {
        while self.report_index < self.reports.len()
            && self.reports[self.report_index].evidence().is_empty()
        {
            self.report_index += 1;
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedReportEvidenceCursorPage {
    report_index: u64,
    page_index: u64,
    start: u64,
    end: u64,
    total: u64,
    evidence: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence>,
}

impl CallerRequestedFloat32RetainedReportEvidenceCursorPage {
    pub(crate) const fn report_index(&self) -> u64 {
        self.report_index
    }

    pub(crate) const fn page_index(&self) -> u64 {
        self.page_index
    }

    pub(crate) const fn start(&self) -> u64 {
        self.start
    }

    pub(crate) const fn end(&self) -> u64 {
        self.end
    }

    pub(crate) const fn total(&self) -> u64 {
        self.total
    }

    pub(crate) fn evidence(
        &self,
    ) -> &[CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence] {
        &self.evidence
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedReportEvidenceCursorAdvance {
    Page {
        cursor: CallerRequestedFloat32RetainedReportEvidenceCursor,
        page: CallerRequestedFloat32RetainedReportEvidenceCursorPage,
    },
    Complete(CallerRequestedFloat32RetainedReportEvidenceCursor),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedReportEvidenceCursorPageFailure {
    TotalUnrepresentable { actual: usize },
    StartUnrepresentable { start: u64 },
    StartOutOfRange { start: u64, total: u64 },
    RangeOverflow { start: u64, length: usize },
    EndUnrepresentable { actual: usize },
    Allocation { requested: usize },
}

impl From<&CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError>
    for CallerRequestedFloat32RetainedReportEvidenceCursorPageFailure
{
    fn from(
        source: &CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError,
    ) -> Self {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::*;
        match source {
            TotalUnrepresentable { actual, .. } => Self::TotalUnrepresentable { actual: *actual },
            StartUnrepresentable { start, .. } => Self::StartUnrepresentable { start: *start },
            StartOutOfRange { start, total, .. } => Self::StartOutOfRange {
                start: *start,
                total: *total,
            },
            RangeOverflow { start, length, .. } => Self::RangeOverflow {
                start: *start,
                length: *length,
            },
            EndUnrepresentable { actual, .. } => Self::EndUnrepresentable { actual: *actual },
            Allocation { requested, .. } => Self::Allocation {
                requested: *requested,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedReportEvidenceCursorAdvanceError {
    report_index: u64,
    page_index: u64,
    evidence_start: u64,
    failure: CallerRequestedFloat32RetainedReportEvidenceCursorPageFailure,
    cursor: CallerRequestedFloat32RetainedReportEvidenceCursor,
}

impl CallerRequestedFloat32RetainedReportEvidenceCursorAdvanceError {
    pub(crate) const fn report_index(&self) -> u64 {
        self.report_index
    }

    pub(crate) const fn page_index(&self) -> u64 {
        self.page_index
    }

    pub(crate) const fn evidence_start(&self) -> u64 {
        self.evidence_start
    }

    pub(crate) const fn failure(
        &self,
    ) -> CallerRequestedFloat32RetainedReportEvidenceCursorPageFailure {
        self.failure
    }

    pub(crate) fn into_cursor(self) -> CallerRequestedFloat32RetainedReportEvidenceCursor {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use crate::caller_requested_float32_retained_comparative_snapshot_report::{
        CallerRequestedFloat32RetainedComparativeSnapshotReportBounds,
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner,
    };
    use crate::caller_requested_float32_retained_comparative_snapshot_report_evidence_page::CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError;
    use crate::caller_requested_float32_retained_comparative_snapshot_report_history::CallerRequestedFloat32RetainedComparativeSnapshotReportHistoryBounds;

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
                u64::try_from(count.max(1)).unwrap() * 10,
            )
            .unwrap(),
        );
        for _ in 0..count {
            history = history.append(report()).unwrap();
        }
        history
    }

    fn identities(
        reports: &[CallerRequestedFloat32RetainedComparativeSnapshotReport],
    ) -> Vec<(
        *const CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
        *const f32,
    )> {
        reports
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

    fn bounds(length: usize) -> CallerRequestedFloat32RetainedReportEvidenceCursorBounds {
        CallerRequestedFloat32RetainedReportEvidenceCursorBounds::new(length).unwrap()
    }

    #[test]
    fn zero_bound_is_rejected_before_any_history_is_consumed() {
        assert_eq!(
            CallerRequestedFloat32RetainedReportEvidenceCursorBounds::new(0),
            Err(CallerRequestedFloat32RetainedReportEvidenceCursorConfigError::ZeroMaximumPageLength)
        );
    }

    #[test]
    fn first_middle_final_and_repeated_completion_are_exact() {
        let source = history(2);
        let original = identities(source.reports());
        let expected: Vec<_> = source
            .reports()
            .iter()
            .map(|report| report.evidence().to_vec())
            .collect();
        let mut cursor =
            CallerRequestedFloat32RetainedReportEvidenceCursor::new(bounds(4), source).unwrap();
        assert_eq!(cursor.total_pages(), 6);
        assert_eq!(cursor.facts().report_count(), 2);
        assert_eq!(cursor.facts().total_evidence(), 20);
        assert_eq!(cursor.facts().completed_evidence(), 0);
        assert_eq!(cursor.facts().remaining_evidence(), 20);
        let first = cursor.facts().current().unwrap();
        assert_eq!(
            (
                first.report_index(),
                first.page_index(),
                first.evidence_start()
            ),
            (0, 0, 0)
        );
        for (report_index, page_index, start, end) in [
            (0, 0, 0, 4),
            (0, 1, 4, 8),
            (0, 2, 8, 10),
            (1, 0, 0, 4),
            (1, 1, 4, 8),
            (1, 2, 8, 10),
        ]
        .into_iter()
        {
            let (next, page) = match cursor.advance().unwrap() {
                CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Page {
                    cursor,
                    page,
                } => (cursor, page),
                CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Complete(_) => {
                    panic!("page expected")
                }
            };
            assert_eq!(
                (
                    page.report_index(),
                    page.page_index(),
                    page.start(),
                    page.end(),
                    page.total()
                ),
                (report_index, page_index, start, end, 10)
            );
            assert_eq!(
                page.evidence(),
                &expected[report_index as usize][start as usize..end as usize]
            );
            assert_eq!(next.facts().completed_evidence(), end + report_index * 10);
            assert_eq!(
                next.facts().remaining_evidence(),
                20 - (end + report_index * 10)
            );
            assert_eq!(identities(next.reports()), original);
            cursor = next;
        }
        assert_eq!(cursor.facts().current(), None);
        assert_eq!(cursor.facts().remaining_evidence(), 0);
        cursor = match cursor.advance().unwrap() {
            CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Complete(cursor) => cursor,
            _ => panic!("complete expected"),
        };
        cursor = match cursor.advance().unwrap() {
            CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Complete(cursor) => cursor,
            _ => panic!("repeat complete expected"),
        };
        assert_eq!(identities(cursor.reports()), original);
    }

    #[test]
    fn empty_history_is_already_at_end_and_retains_exact_facts() {
        let cursor =
            CallerRequestedFloat32RetainedReportEvidenceCursor::new(bounds(3), history(0)).unwrap();
        assert_eq!(cursor.total_pages(), 0);
        assert_eq!(
            cursor.facts(),
            CallerRequestedFloat32RetainedReportEvidenceCursorFacts {
                report_count: 0,
                total_evidence: 0,
                completed_evidence: 0,
                current: None,
                remaining_evidence: 0,
            }
        );
        assert!(matches!(
            cursor.advance().unwrap(),
            CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Complete(_)
        ));
    }

    #[test]
    fn construction_conversion_overflow_and_allocation_failures_return_actual_history() {
        for failure in 0..4 {
            let source = history(2);
            let original = identities(source.reports());
            let calls = Cell::new(0);
            let result = CallerRequestedFloat32RetainedReportEvidenceCursor::new_with(
                bounds(3),
                source,
                |_, requested| {
                    assert_eq!(requested, 2);
                    if failure == 0 {
                        Err(())
                    } else {
                        Ok(())
                    }
                },
                |value| {
                    let call = calls.get();
                    calls.set(call + 1);
                    if call + 1 == failure {
                        Err(())
                    } else {
                        u64::try_from(value).map_err(|_| ())
                    }
                },
                |left, right| {
                    if failure == 3 {
                        Err(())
                    } else {
                        left.checked_add(right).ok_or(())
                    }
                },
            );
            let error = result.unwrap_err();
            assert_eq!(identities(error.into_history().reports()), original);
        }

        let source = history(1);
        let original = identities(source.reports());
        let additions = Cell::new(0);
        let error = CallerRequestedFloat32RetainedReportEvidenceCursor::new_with(
            bounds(usize::MAX),
            source,
            |_, _| Ok(()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| {
                let addition = additions.get();
                additions.set(addition + 1);
                if addition == 1 {
                    Err(())
                } else {
                    left.checked_add(right).ok_or(())
                }
            },
        )
        .unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32RetainedReportEvidenceCursorConstructionError::PageCountOverflow {
                report_index: 0,
                ..
            }
        ));
        assert_eq!(identities(error.into_history().reports()), original);
    }

    #[test]
    fn page_allocation_and_conversion_failures_roll_back_the_whole_cursor() {
        let cursor =
            CallerRequestedFloat32RetainedReportEvidenceCursor::new(bounds(4), history(2)).unwrap();
        let before_facts = cursor.facts();
        let before_ids = identities(cursor.reports());
        let error = cursor
            .advance_with(|_, report, _start| {
                Err(CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::Allocation {
                    requested: 4,
                    report,
                })
            })
            .unwrap_err();
        assert_eq!(
            (
                error.report_index(),
                error.page_index(),
                error.evidence_start()
            ),
            (0, 0, 0)
        );
        assert_eq!(
            error.failure(),
            CallerRequestedFloat32RetainedReportEvidenceCursorPageFailure::Allocation {
                requested: 4
            }
        );
        let cursor = error.into_cursor();
        assert_eq!(cursor.facts(), before_facts);
        assert_eq!(identities(cursor.reports()), before_ids);

        let error = cursor
            .advance_with(|_, report, start| {
                Err(CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::StartUnrepresentable {
                    start,
                    report,
                })
            })
            .unwrap_err();
        let cursor = error.into_cursor();
        assert_eq!(cursor.facts(), before_facts);
        assert_eq!(identities(cursor.reports()), before_ids);

        let (cursor, _) = match cursor.advance().unwrap() {
            CallerRequestedFloat32RetainedReportEvidenceCursorAdvance::Page { cursor, page } => {
                (cursor, page)
            }
            _ => panic!("page expected"),
        };
        let middle_facts = cursor.facts();
        let middle_ids = identities(cursor.reports());
        let error = cursor
            .advance_with(|_, report, start| {
                Err(CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::RangeOverflow {
                    start,
                    length: usize::MAX,
                    report,
                })
            })
            .unwrap_err();
        let cursor = error.into_cursor();
        assert_eq!(cursor.facts(), middle_facts);
        assert_eq!(identities(cursor.reports()), middle_ids);
    }

    #[test]
    fn boundary_remains_private_advisory_and_non_authoritative() {
        let source = include_str!("caller_requested_float32_retained_report_evidence_cursor.rs");
        for wording in [
            "crate-private",
            "default-inert",
            "infers no continuity",
            "loss",
            "non-applying",
            "liblsl equivalence",
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
            .contains("CallerRequestedFloat32RetainedReportEvidenceCursor"));
        assert!(!include_str!("lib.rs").contains(concat!(
            "pub use caller_requested_float32_retained_report_evidence_",
            "cursor"
        )));
    }
}
