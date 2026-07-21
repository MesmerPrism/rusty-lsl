// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded caller-requested page over one actual retained comparison report.
//!
//! This crate-private owner consumes and retains the complete P50 report
//! unchanged. It copies only the report's existing `Copy` evidence facts in
//! their existing order. It is default-inert, advisory, and non-applying; it
//! infers no missing evidence, loss, continuity, or causality, claims no
//! liblsl equivalence, and grants no Manifold, session, stream, transport,
//! control, routing, admission, activation, device, oracle, or policy authority.

use crate::caller_requested_float32_retained_comparative_snapshot_report::{
    CallerRequestedFloat32RetainedComparativeSnapshotReport,
    CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageConfigError {
    ZeroMaximumPageLength,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds {
    maximum_page_length: usize,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds {
    pub(crate) fn new(
        maximum_page_length: usize,
    ) -> Result<Self, CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageConfigError>
    {
        if maximum_page_length == 0 {
            return Err(
                CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageConfigError::ZeroMaximumPageLength,
            );
        }
        u64::try_from(maximum_page_length).map_err(|_| {
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageConfigError::BoundUnrepresentable {
                requested: maximum_page_length,
            }
        })?;
        Ok(Self {
            maximum_page_length,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage {
    report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    start: u64,
    end: u64,
    total: u64,
    evidence: Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence>,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage {
    pub(crate) const fn report(&self) -> &CallerRequestedFloat32RetainedComparativeSnapshotReport {
        &self.report
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

    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32RetainedComparativeSnapshotReport,
        Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence>,
    ) {
        (self.report, self.evidence)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError {
    TotalUnrepresentable {
        actual: usize,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    StartUnrepresentable {
        start: u64,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    StartOutOfRange {
        start: u64,
        total: u64,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    RangeOverflow {
        start: u64,
        length: usize,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    EndUnrepresentable {
        actual: usize,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
    Allocation {
        requested: usize,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
    },
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError {
    pub(crate) fn into_report(self) -> CallerRequestedFloat32RetainedComparativeSnapshotReport {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::*;
        match self {
            TotalUnrepresentable { report, .. }
            | StartUnrepresentable { report, .. }
            | StartOutOfRange { report, .. }
            | RangeOverflow { report, .. }
            | EndUnrepresentable { report, .. }
            | Allocation { report, .. } => report,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner {
    bounds: CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds,
}

impl CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner {
    pub(crate) const fn new(
        bounds: CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds,
    ) -> Self {
        Self { bounds }
    }

    pub(crate) fn page(
        &self,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
        start: u64,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage,
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError,
    > {
        self.page_with(
            report,
            start,
            |evidence, requested| evidence.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |value| usize::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn page_with<R, U, S, A>(
        &self,
        report: CallerRequestedFloat32RetainedComparativeSnapshotReport,
        start: u64,
        reserve: R,
        to_u64: U,
        to_usize: S,
        add_u64: A,
    ) -> Result<
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage,
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence>,
            usize,
        ) -> Result<(), ()>,
        U: Fn(usize) -> Result<u64, ()>,
        S: Fn(u64) -> Result<usize, ()>,
        A: Fn(u64, u64) -> Result<u64, ()>,
    {
        use CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::*;
        macro_rules! fail {
            ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => {
                return Err($variant { $($field: $value,)* report })
            };
        }

        let total_usize = report.evidence().len();
        let total = match to_u64(total_usize) {
            Ok(value) => value,
            Err(()) => fail!(TotalUnrepresentable {
                actual: total_usize
            }),
        };
        if start > total {
            fail!(StartOutOfRange {
                start: start,
                total: total
            });
        }
        let start_usize = match to_usize(start) {
            Ok(value) => value,
            Err(()) => fail!(StartUnrepresentable { start: start }),
        };
        let remaining = match total_usize.checked_sub(start_usize) {
            Some(value) => value,
            None => fail!(StartOutOfRange {
                start: start,
                total: total
            }),
        };
        let length = remaining.min(self.bounds.maximum_page_length);
        let length_u64 = match to_u64(length) {
            Ok(value) => value,
            Err(()) => fail!(EndUnrepresentable { actual: length }),
        };
        let end = match add_u64(start, length_u64) {
            Ok(value) => value,
            Err(()) => fail!(RangeOverflow {
                start: start,
                length: length
            }),
        };
        let end_usize = match start_usize.checked_add(length) {
            Some(value) => value,
            None => fail!(RangeOverflow {
                start: start,
                length: length
            }),
        };
        if to_u64(end_usize).map_err(|_| ()).ok() != Some(end) {
            fail!(EndUnrepresentable { actual: end_usize });
        }

        let mut evidence = Vec::new();
        if reserve(&mut evidence, length).is_err() {
            fail!(Allocation { requested: length });
        }
        evidence.extend_from_slice(&report.evidence()[start_usize..end_usize]);
        Ok(
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePage {
                report,
                start,
                end,
                total,
                evidence,
            },
        )
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

    fn report() -> CallerRequestedFloat32RetainedComparativeSnapshotReport {
        let (history, package) = crate::tests::p50_actual_inputs();
        CallerRequestedFloat32RetainedComparativeSnapshotReportOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportBounds::new(2, 8, 10).unwrap(),
        )
        .report(history, package)
        .unwrap()
    }

    fn identity(
        report: &CallerRequestedFloat32RetainedComparativeSnapshotReport,
    ) -> (
        *const CallerRequestedFloat32RetainedComparativeSnapshotReportEvidence,
        *const f32,
    ) {
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
    }

    fn owner(
        maximum: usize,
    ) -> CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner {
        CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageOwner::new(
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds::new(maximum)
                .unwrap(),
        )
    }

    #[test]
    fn config_rejects_zero_bound() {
        assert_eq!(
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageBounds::new(0),
            Err(CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageConfigError::ZeroMaximumPageLength)
        );
    }

    #[test]
    fn pages_are_exact_ordered_and_retain_the_complete_actual_report() {
        let expected = report().evidence().to_vec();
        for (start, maximum, end) in [(0, 3, 3), (3, 4, 7), (7, 8, 10), (10, 4, 10)] {
            let original = report();
            let original_identity = identity(&original);
            let page = owner(maximum).page(original, start).unwrap();
            assert_eq!((page.start(), page.end(), page.total()), (start, end, 10));
            assert_eq!(page.evidence(), &expected[start as usize..end as usize]);
            assert_eq!(identity(page.report()), original_identity);
            let (retained, copied) = page.into_parts();
            assert_eq!(identity(&retained), original_identity);
            assert_eq!(copied, expected[start as usize..end as usize]);
        }
    }

    #[test]
    fn exact_end_has_empty_evidence_but_beyond_end_is_typed() {
        let empty = owner(4).page(report(), 10).unwrap();
        assert_eq!((empty.start(), empty.end(), empty.total()), (10, 10, 10));
        assert!(empty.evidence().is_empty());

        let original = report();
        let original_identity = identity(&original);
        let error = owner(4).page(original, 11).unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::StartOutOfRange { start: 11, total: 10, .. }));
        assert_eq!(identity(&error.into_report()), original_identity);
    }

    #[test]
    fn conversion_range_and_allocation_failures_return_the_unchanged_report() {
        let original = report();
        let original_identity = identity(&original);
        let error = owner(2)
            .page_with(
                original,
                0,
                |_, _| Ok(()),
                |_| Err(()),
                |_| Ok(0),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::TotalUnrepresentable { actual: 10, .. }));
        assert_eq!(identity(&error.into_report()), original_identity);

        let original = report();
        let original_identity = identity(&original);
        let conversions = Cell::new(0);
        let error = owner(2)
            .page_with(
                original,
                1,
                |_, _| Ok(()),
                |value| {
                    let conversion = conversions.get();
                    conversions.set(conversion + 1);
                    if conversion == 2 {
                        Err(())
                    } else {
                        Ok(value as u64)
                    }
                },
                |value| Ok(value as usize),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err();
        assert_eq!(conversions.get(), 3);
        assert!(matches!(
            error,
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::EndUnrepresentable {
                actual: 3,
                ..
            }
        ));
        assert_eq!(identity(&error.into_report()), original_identity);

        let original = report();
        let original_identity = identity(&original);
        let error = owner(2)
            .page_with(
                original,
                1,
                |_, _| Ok(()),
                |v| Ok(v as u64),
                |_| Err(()),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::StartUnrepresentable { start: 1, .. }));
        assert_eq!(identity(&error.into_report()), original_identity);

        let original = report();
        let original_identity = identity(&original);
        let conversions = Cell::new(0);
        let error = owner(2)
            .page_with(
                original,
                1,
                |_, _| Ok(()),
                |value| {
                    let conversion = conversions.get();
                    conversions.set(conversion + 1);
                    if conversion == 1 {
                        Err(())
                    } else {
                        Ok(value as u64)
                    }
                },
                |value| Ok(value as usize),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::EndUnrepresentable {
                actual: 2,
                ..
            }
        ));
        assert_eq!(identity(&error.into_report()), original_identity);

        let original = report();
        let original_identity = identity(&original);
        let error = owner(2)
            .page_with(
                original,
                1,
                |_, _| Ok(()),
                |v| Ok(v as u64),
                |v| Ok(v as usize),
                |_, _| Err(()),
            )
            .unwrap_err();
        assert!(matches!(error, CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::RangeOverflow { start: 1, length: 2, .. }));
        assert_eq!(identity(&error.into_report()), original_identity);

        let original = report();
        let original_identity = identity(&original);
        let error = owner(2)
            .page_with(
                original,
                1,
                |_, requested| {
                    assert_eq!(requested, 2);
                    Err(())
                },
                |v| Ok(v as u64),
                |v| Ok(v as usize),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32RetainedComparativeSnapshotReportEvidencePageError::Allocation {
                requested: 2,
                ..
            }
        ));
        assert_eq!(identity(&error.into_report()), original_identity);
    }
}
