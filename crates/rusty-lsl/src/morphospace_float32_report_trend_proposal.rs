// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic, effect-free trend advice over an ordered window of exact P36 observations.
//!
//! This disposable candidate is crate-private and unwired. It returns the owned window on every
//! result and failure and exposes no apply, accept, route, lease, revision, authorization,
//! application, or audit mechanism. It neither invents loss nor introduces terminal facts.

/// Borrowed record facts already exposed by one frozen P36 observation.
pub(crate) trait Float32ReportTrendObservedRecord {
    fn record_index(&self) -> u64;
    fn sequence(&self) -> u64;
    fn signed_adjustment_bits(&self) -> u64;
}

/// Borrowed exact facts already checked by one frozen P36 observation.
pub(crate) trait Float32ReportTrendObservedReport {
    type Record: Float32ReportTrendObservedRecord;

    fn report_index(&self) -> u64;
    fn records(&self) -> &[Self::Record];
    fn exact_record_count(&self) -> u64;
    fn explicit_missing_sequence_count(&self) -> u64;
    fn duplicate_count(&self) -> u64;
    fn out_of_order_count(&self) -> u64;
    fn retained_changed_count(&self) -> u64;
}

/// Owned ordered window. The proposal only borrows this view before returning the owner intact.
pub(crate) trait Float32ReportTrendObservationWindow {
    type Report: Float32ReportTrendObservedReport;

    fn reports(&self) -> &[Self::Report];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportTrendThresholds {
    maximum_reports: usize,
    maximum_records_per_report: u64,
    maximum_total_records: u64,
    maximum_explicit_missing_sequences: u64,
    maximum_duplicates: u64,
    maximum_out_of_order: u64,
    maximum_retained_changed: u64,
    maximum_absolute_adjustment_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportTrendThresholdError {
    ZeroMaximumReports,
    MaximumReportsUnrepresentable { requested: usize },
    NonFiniteMaximumAbsoluteAdjustment { bits: u64 },
    NegativeMaximumAbsoluteAdjustment { bits: u64 },
}

impl Float32ReportTrendThresholds {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        maximum_reports: usize,
        maximum_records_per_report: u64,
        maximum_total_records: u64,
        maximum_explicit_missing_sequences: u64,
        maximum_duplicates: u64,
        maximum_out_of_order: u64,
        maximum_retained_changed: u64,
        maximum_absolute_adjustment: f64,
    ) -> Result<Self, Float32ReportTrendThresholdError> {
        if maximum_reports == 0 {
            return Err(Float32ReportTrendThresholdError::ZeroMaximumReports);
        }
        u64::try_from(maximum_reports).map_err(|_| {
            Float32ReportTrendThresholdError::MaximumReportsUnrepresentable {
                requested: maximum_reports,
            }
        })?;
        if !maximum_absolute_adjustment.is_finite() {
            return Err(
                Float32ReportTrendThresholdError::NonFiniteMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                },
            );
        }
        if maximum_absolute_adjustment < 0.0 {
            return Err(
                Float32ReportTrendThresholdError::NegativeMaximumAbsoluteAdjustment {
                    bits: maximum_absolute_adjustment.to_bits(),
                },
            );
        }
        Ok(Self {
            maximum_reports,
            maximum_records_per_report,
            maximum_total_records,
            maximum_explicit_missing_sequences,
            maximum_duplicates,
            maximum_out_of_order,
            maximum_retained_changed,
            maximum_absolute_adjustment_bits: maximum_absolute_adjustment.to_bits(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportTrendLargestAdjustment {
    pub(crate) report_index: u64,
    pub(crate) record_index: u64,
    pub(crate) sequence: u64,
    pub(crate) signed_adjustment_bits: u64,
    pub(crate) absolute_adjustment_bits: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportTrendAggregates {
    pub(crate) report_count: u64,
    pub(crate) total_records: u64,
    pub(crate) explicit_missing_sequences: u64,
    pub(crate) duplicates: u64,
    pub(crate) out_of_order: u64,
    pub(crate) retained_changed: u64,
    pub(crate) largest_absolute_adjustment: Option<Float32ReportTrendLargestAdjustment>,
}

/// Review reasons are emitted in this declaration order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportTrendReviewReason {
    TotalRecords {
        observed: u64,
        maximum: u64,
    },
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
        report_index: u64,
        record_index: u64,
        sequence: u64,
        signed_adjustment_bits: u64,
        observed_absolute_bits: u64,
        maximum_absolute_bits: u64,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum Float32ReportTrendProposal<W> {
    Retain {
        window: W,
        aggregates: Float32ReportTrendAggregates,
    },
    Review {
        window: W,
        aggregates: Float32ReportTrendAggregates,
        reasons: Vec<Float32ReportTrendReviewReason>,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) enum Float32ReportTrendError<W> {
    EmptyWindow {
        window: W,
    },
    ReportBound {
        limit: usize,
        actual: usize,
        window: W,
    },
    ReportCountUnrepresentable {
        actual: usize,
        window: W,
    },
    RecordBound {
        report_index: u64,
        limit: u64,
        actual: u64,
        window: W,
    },
    RecordExtentMismatch {
        report_index: u64,
        declared: u64,
        actual: u64,
        window: W,
    },
    CounterOverflow {
        report_index: u64,
        counter: Float32ReportTrendCounter,
        window: W,
    },
    NonFiniteAdjustment {
        report_index: u64,
        record_index: u64,
        sequence: u64,
        bits: u64,
        window: W,
    },
    Allocation {
        requested_reasons: usize,
        window: W,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Float32ReportTrendCounter {
    TotalRecords,
    ExplicitMissingSequences,
    Duplicates,
    OutOfOrder,
    RetainedChanged,
}

impl<W> Float32ReportTrendError<W> {
    pub(crate) fn into_window(self) -> W {
        match self {
            Self::EmptyWindow { window }
            | Self::ReportBound { window, .. }
            | Self::ReportCountUnrepresentable { window, .. }
            | Self::RecordBound { window, .. }
            | Self::RecordExtentMismatch { window, .. }
            | Self::CounterOverflow { window, .. }
            | Self::NonFiniteAdjustment { window, .. }
            | Self::Allocation { window, .. } => window,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Float32ReportTrendProposalOwner {
    thresholds: Float32ReportTrendThresholds,
}

impl Float32ReportTrendProposalOwner {
    pub(crate) const fn new(thresholds: Float32ReportTrendThresholds) -> Self {
        Self { thresholds }
    }

    pub(crate) fn propose<W: Float32ReportTrendObservationWindow>(
        &self,
        window: W,
    ) -> Result<Float32ReportTrendProposal<W>, Float32ReportTrendError<W>> {
        self.propose_with(window, |reasons, count| {
            reasons.try_reserve_exact(count).map_err(|_| ())
        })
    }

    fn propose_with<W, R>(
        &self,
        window: W,
        reserve: R,
    ) -> Result<Float32ReportTrendProposal<W>, Float32ReportTrendError<W>>
    where
        W: Float32ReportTrendObservationWindow,
        R: FnOnce(&mut Vec<Float32ReportTrendReviewReason>, usize) -> Result<(), ()>,
    {
        let aggregates = match collect(window.reports(), self.thresholds.maximum_records_per_report)
        {
            Ok(value) => value,
            Err(failure) => return Err(failure.attach(window)),
        };
        if window.reports().len() > self.thresholds.maximum_reports {
            return Err(Float32ReportTrendError::ReportBound {
                limit: self.thresholds.maximum_reports,
                actual: window.reports().len(),
                window,
            });
        }
        const MAXIMUM_REASONS: usize = 6;
        let mut reasons = Vec::new();
        if reserve(&mut reasons, MAXIMUM_REASONS).is_err() {
            return Err(Float32ReportTrendError::Allocation {
                requested_reasons: MAXIMUM_REASONS,
                window,
            });
        }
        push(
            &mut reasons,
            aggregates.total_records,
            self.thresholds.maximum_total_records,
            |observed, maximum| Float32ReportTrendReviewReason::TotalRecords { observed, maximum },
        );
        push(
            &mut reasons,
            aggregates.explicit_missing_sequences,
            self.thresholds.maximum_explicit_missing_sequences,
            |observed, maximum| Float32ReportTrendReviewReason::ExplicitMissingSequences {
                observed,
                maximum,
            },
        );
        push(
            &mut reasons,
            aggregates.duplicates,
            self.thresholds.maximum_duplicates,
            |observed, maximum| Float32ReportTrendReviewReason::Duplicates { observed, maximum },
        );
        push(
            &mut reasons,
            aggregates.out_of_order,
            self.thresholds.maximum_out_of_order,
            |observed, maximum| Float32ReportTrendReviewReason::OutOfOrder { observed, maximum },
        );
        push(
            &mut reasons,
            aggregates.retained_changed,
            self.thresholds.maximum_retained_changed,
            |observed, maximum| Float32ReportTrendReviewReason::RetainedChanged {
                observed,
                maximum,
            },
        );
        if let Some(largest) = aggregates.largest_absolute_adjustment {
            if f64::from_bits(largest.absolute_adjustment_bits)
                > f64::from_bits(self.thresholds.maximum_absolute_adjustment_bits)
            {
                reasons.push(Float32ReportTrendReviewReason::AbsoluteAdjustment {
                    report_index: largest.report_index,
                    record_index: largest.record_index,
                    sequence: largest.sequence,
                    signed_adjustment_bits: largest.signed_adjustment_bits,
                    observed_absolute_bits: largest.absolute_adjustment_bits,
                    maximum_absolute_bits: self.thresholds.maximum_absolute_adjustment_bits,
                });
            }
        }
        if reasons.is_empty() {
            Ok(Float32ReportTrendProposal::Retain { window, aggregates })
        } else {
            Ok(Float32ReportTrendProposal::Review {
                window,
                aggregates,
                reasons,
            })
        }
    }
}

enum CollectFailure {
    EmptyWindow,
    ReportCountUnrepresentable {
        actual: usize,
    },
    RecordBound {
        report_index: u64,
        limit: u64,
        actual: u64,
    },
    RecordExtentMismatch {
        report_index: u64,
        declared: u64,
        actual: u64,
    },
    CounterOverflow {
        report_index: u64,
        counter: Float32ReportTrendCounter,
    },
    NonFiniteAdjustment {
        report_index: u64,
        record_index: u64,
        sequence: u64,
        bits: u64,
    },
}

impl CollectFailure {
    fn attach<W>(self, window: W) -> Float32ReportTrendError<W> {
        match self {
            Self::EmptyWindow => Float32ReportTrendError::EmptyWindow { window },
            Self::ReportCountUnrepresentable { actual } => {
                Float32ReportTrendError::ReportCountUnrepresentable { actual, window }
            }
            Self::RecordBound {
                report_index,
                limit,
                actual,
            } => Float32ReportTrendError::RecordBound {
                report_index,
                limit,
                actual,
                window,
            },
            Self::RecordExtentMismatch {
                report_index,
                declared,
                actual,
            } => Float32ReportTrendError::RecordExtentMismatch {
                report_index,
                declared,
                actual,
                window,
            },
            Self::CounterOverflow {
                report_index,
                counter,
            } => Float32ReportTrendError::CounterOverflow {
                report_index,
                counter,
                window,
            },
            Self::NonFiniteAdjustment {
                report_index,
                record_index,
                sequence,
                bits,
            } => Float32ReportTrendError::NonFiniteAdjustment {
                report_index,
                record_index,
                sequence,
                bits,
                window,
            },
        }
    }
}

fn collect<R: Float32ReportTrendObservedReport>(
    reports: &[R],
    per_report_limit: u64,
) -> Result<Float32ReportTrendAggregates, CollectFailure> {
    if reports.is_empty() {
        return Err(CollectFailure::EmptyWindow);
    }
    let report_count =
        u64::try_from(reports.len()).map_err(|_| CollectFailure::ReportCountUnrepresentable {
            actual: reports.len(),
        })?;
    let mut result = Float32ReportTrendAggregates {
        report_count,
        total_records: 0,
        explicit_missing_sequences: 0,
        duplicates: 0,
        out_of_order: 0,
        retained_changed: 0,
        largest_absolute_adjustment: None,
    };
    for report in reports {
        let report_index = report.report_index();
        let actual =
            u64::try_from(report.records().len()).map_err(|_| CollectFailure::CounterOverflow {
                report_index,
                counter: Float32ReportTrendCounter::TotalRecords,
            })?;
        if actual > per_report_limit {
            return Err(CollectFailure::RecordBound {
                report_index,
                limit: per_report_limit,
                actual,
            });
        }
        if actual != report.exact_record_count() {
            return Err(CollectFailure::RecordExtentMismatch {
                report_index,
                declared: report.exact_record_count(),
                actual,
            });
        }
        checked_add(
            &mut result.total_records,
            actual,
            report_index,
            Float32ReportTrendCounter::TotalRecords,
        )?;
        checked_add(
            &mut result.explicit_missing_sequences,
            report.explicit_missing_sequence_count(),
            report_index,
            Float32ReportTrendCounter::ExplicitMissingSequences,
        )?;
        checked_add(
            &mut result.duplicates,
            report.duplicate_count(),
            report_index,
            Float32ReportTrendCounter::Duplicates,
        )?;
        checked_add(
            &mut result.out_of_order,
            report.out_of_order_count(),
            report_index,
            Float32ReportTrendCounter::OutOfOrder,
        )?;
        checked_add(
            &mut result.retained_changed,
            report.retained_changed_count(),
            report_index,
            Float32ReportTrendCounter::RetainedChanged,
        )?;
        for record in report.records() {
            let adjustment = f64::from_bits(record.signed_adjustment_bits());
            if !adjustment.is_finite() {
                return Err(CollectFailure::NonFiniteAdjustment {
                    report_index,
                    record_index: record.record_index(),
                    sequence: record.sequence(),
                    bits: adjustment.to_bits(),
                });
            }
            let candidate = Float32ReportTrendLargestAdjustment {
                report_index,
                record_index: record.record_index(),
                sequence: record.sequence(),
                signed_adjustment_bits: adjustment.to_bits(),
                absolute_adjustment_bits: adjustment.abs().to_bits(),
            };
            let replace = result
                .largest_absolute_adjustment
                .map(|current| {
                    adjustment.abs() > f64::from_bits(current.absolute_adjustment_bits)
                        || (adjustment.abs() == f64::from_bits(current.absolute_adjustment_bits)
                            && (candidate.report_index, candidate.record_index)
                                < (current.report_index, current.record_index))
                })
                .unwrap_or(true);
            if replace {
                result.largest_absolute_adjustment = Some(candidate);
            }
        }
    }
    Ok(result)
}

fn checked_add(
    target: &mut u64,
    value: u64,
    report_index: u64,
    counter: Float32ReportTrendCounter,
) -> Result<(), CollectFailure> {
    *target = target
        .checked_add(value)
        .ok_or(CollectFailure::CounterOverflow {
            report_index,
            counter,
        })?;
    Ok(())
}

fn push(
    reasons: &mut Vec<Float32ReportTrendReviewReason>,
    observed: u64,
    maximum: u64,
    make: fn(u64, u64) -> Float32ReportTrendReviewReason,
) {
    if observed > maximum {
        reasons.push(make(observed, maximum));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Record {
        index: u64,
        sequence: u64,
        adjustment: f64,
        allocation: Box<u8>,
    }
    impl Float32ReportTrendObservedRecord for Record {
        fn record_index(&self) -> u64 {
            self.index
        }
        fn sequence(&self) -> u64 {
            self.sequence
        }
        fn signed_adjustment_bits(&self) -> u64 {
            self.adjustment.to_bits()
        }
    }
    #[derive(Debug, PartialEq)]
    struct Report {
        index: u64,
        declared: u64,
        missing: u64,
        duplicates: u64,
        out: u64,
        changed: u64,
        records: Vec<Record>,
    }
    impl Float32ReportTrendObservedReport for Report {
        type Record = Record;
        fn report_index(&self) -> u64 {
            self.index
        }
        fn records(&self) -> &[Record] {
            &self.records
        }
        fn exact_record_count(&self) -> u64 {
            self.declared
        }
        fn explicit_missing_sequence_count(&self) -> u64 {
            self.missing
        }
        fn duplicate_count(&self) -> u64 {
            self.duplicates
        }
        fn out_of_order_count(&self) -> u64 {
            self.out
        }
        fn retained_changed_count(&self) -> u64 {
            self.changed
        }
    }
    #[derive(Debug, PartialEq)]
    struct Window(Vec<Report>);
    impl Float32ReportTrendObservationWindow for Window {
        type Report = Report;
        fn reports(&self) -> &[Report] {
            &self.0
        }
    }
    fn record(index: u64, sequence: u64, adjustment: f64) -> Record {
        Record {
            index,
            sequence,
            adjustment,
            allocation: Box::new(index as u8),
        }
    }
    fn report(index: u64, adjustment: f64) -> Report {
        Report {
            index,
            declared: 1,
            missing: index,
            duplicates: index,
            out: index,
            changed: index,
            records: vec![record(0, 7, adjustment)],
        }
    }
    fn thresholds(max: u64) -> Float32ReportTrendThresholds {
        Float32ReportTrendThresholds::new(4, u64::MAX, max, max, max, max, max, max as f64).unwrap()
    }

    #[test]
    fn zero_and_extreme_thresholds_are_explicit() {
        assert_eq!(
            Float32ReportTrendThresholds::new(0, 0, 0, 0, 0, 0, 0, 0.0),
            Err(Float32ReportTrendThresholdError::ZeroMaximumReports)
        );
        assert!(Float32ReportTrendThresholds::new(
            usize::try_from(u64::MAX).unwrap_or(usize::MAX),
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            f64::MAX
        )
        .is_ok());
        assert!(matches!(
            Float32ReportTrendThresholds::new(1, 0, 0, 0, 0, 0, 0, f64::INFINITY),
            Err(Float32ReportTrendThresholdError::NonFiniteMaximumAbsoluteAdjustment { .. })
        ));
    }

    #[test]
    fn repeated_windows_have_exact_aggregates_reason_order_and_index_ties() {
        for _ in 0..2 {
            let window = Window(vec![report(9, -2.0), report(3, 2.0)]);
            let proposal = Float32ReportTrendProposalOwner::new(thresholds(0))
                .propose(window)
                .unwrap();
            let (window, aggregates, reasons) = match proposal {
                Float32ReportTrendProposal::Review {
                    window,
                    aggregates,
                    reasons,
                } => (window, aggregates, reasons),
                _ => panic!(),
            };
            assert_eq!((aggregates.report_count, aggregates.total_records), (2, 2));
            let largest = aggregates.largest_absolute_adjustment.unwrap();
            assert_eq!(
                (largest.report_index, largest.record_index, largest.sequence),
                (3, 0, 7)
            );
            assert!(matches!(
                reasons.as_slice(),
                [
                    Float32ReportTrendReviewReason::TotalRecords { .. },
                    Float32ReportTrendReviewReason::ExplicitMissingSequences { .. },
                    Float32ReportTrendReviewReason::Duplicates { .. },
                    Float32ReportTrendReviewReason::OutOfOrder { .. },
                    Float32ReportTrendReviewReason::RetainedChanged { .. },
                    Float32ReportTrendReviewReason::AbsoluteAdjustment { .. }
                ]
            ));
            assert_eq!(window.0.len(), 2);
        }
        assert!(matches!(
            Float32ReportTrendProposalOwner::new(thresholds(u64::MAX))
                .propose(Window(vec![report(0, 0.0)]))
                .unwrap(),
            Float32ReportTrendProposal::Retain { .. }
        ));
    }

    #[test]
    fn bound_counter_and_allocation_failures_preserve_owned_window() {
        let window = Window(vec![report(1, 1.0)]);
        let pointer = window.0[0].records[0].allocation.as_ref() as *const u8;
        let returned = Float32ReportTrendProposalOwner::new(
            Float32ReportTrendThresholds::new(1, 0, 9, 9, 9, 9, 9, 9.0).unwrap(),
        )
        .propose(window)
        .unwrap_err()
        .into_window();
        assert_eq!(
            returned.0[0].records[0].allocation.as_ref() as *const u8,
            pointer
        );

        let overflow = Window(vec![
            Report {
                index: 0,
                declared: 0,
                missing: u64::MAX,
                duplicates: 0,
                out: 0,
                changed: 0,
                records: vec![],
            },
            Report {
                index: 1,
                declared: 0,
                missing: 1,
                duplicates: 0,
                out: 0,
                changed: 0,
                records: vec![],
            },
        ]);
        assert!(matches!(
            Float32ReportTrendProposalOwner::new(thresholds(u64::MAX)).propose(overflow),
            Err(Float32ReportTrendError::CounterOverflow {
                counter: Float32ReportTrendCounter::ExplicitMissingSequences,
                ..
            })
        ));

        let window = Window(vec![report(0, 1.0)]);
        let returned = Float32ReportTrendProposalOwner::new(thresholds(9))
            .propose_with(window, |_, _| Err(()))
            .unwrap_err()
            .into_window();
        assert_eq!(returned.0.len(), 1);
    }

    #[test]
    fn extent_and_nonfinite_failures_are_typed_and_authority_is_denied() {
        let mismatch = Window(vec![Report {
            declared: 2,
            ..report(4, 0.0)
        }]);
        assert!(matches!(
            Float32ReportTrendProposalOwner::new(thresholds(9)).propose(mismatch),
            Err(Float32ReportTrendError::RecordExtentMismatch {
                report_index: 4,
                ..
            })
        ));
        let nonfinite = Window(vec![report(5, f64::NAN)]);
        assert!(matches!(
            Float32ReportTrendProposalOwner::new(thresholds(9)).propose(nonfinite),
            Err(Float32ReportTrendError::NonFiniteAdjustment {
                report_index: 5,
                record_index: 0,
                ..
            })
        ));
        let source = include_str!("morphospace_float32_report_trend_proposal.rs");
        for denied in [
            "apply",
            "accept",
            "route",
            "lease",
            "revision",
            "authorization",
            "application",
            "audit",
            "Manifold",
        ] {
            assert!(source.contains(denied));
        }
        assert!(!source.contains(concat!("enum Float32ReportObserved", "TerminalHealth")));
    }
}
