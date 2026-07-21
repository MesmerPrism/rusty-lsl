// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded transactional history of complete actual P43 advisory packages.
//!
//! This crate-private, default-inert owner retains packages in exact insertion
//! order without cloning or reconstructing any nested allocation or evidence.
//! It infers neither loss nor continuity, applies no policy, and grants no
//! application, Manifold, session, stream, transport, or control authority.
//! It makes no claim of liblsl equivalence.

use crate::caller_requested_float32_advisory_report_package::CallerRequestedFloat32AdvisoryReportPackage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32AdvisoryReportPackageHistoryConfigError {
    ZeroMaximumPackages,
    ZeroMaximumHistoryValues,
    ZeroMaximumHistoryEvidence,
    ZeroMaximumSummaryFacts,
    ZeroMaximumPackageFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackageHistoryBounds {
    maximum_packages: usize,
    maximum_packages_u64: u64,
    maximum_history_values: u64,
    maximum_history_evidence: u64,
    maximum_summary_facts: u64,
    maximum_package_facts: u64,
}

impl CallerRequestedFloat32AdvisoryReportPackageHistoryBounds {
    pub(crate) fn new(
        maximum_packages: usize,
        maximum_history_values: usize,
        maximum_history_evidence: usize,
        maximum_summary_facts: usize,
        maximum_package_facts: usize,
    ) -> Result<Self, CallerRequestedFloat32AdvisoryReportPackageHistoryConfigError> {
        use CallerRequestedFloat32AdvisoryReportPackageHistoryConfigError::*;
        for (value, zero) in [
            (maximum_packages, ZeroMaximumPackages),
            (maximum_history_values, ZeroMaximumHistoryValues),
            (maximum_history_evidence, ZeroMaximumHistoryEvidence),
            (maximum_summary_facts, ZeroMaximumSummaryFacts),
            (maximum_package_facts, ZeroMaximumPackageFacts),
        ] {
            if value == 0 {
                return Err(zero);
            }
        }
        Ok(Self {
            maximum_packages,
            maximum_packages_u64: u64::try_from(maximum_packages).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_packages,
                }
            })?,
            maximum_history_values: u64::try_from(maximum_history_values).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_history_values,
                }
            })?,
            maximum_history_evidence: u64::try_from(maximum_history_evidence).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_history_evidence,
                }
            })?,
            maximum_summary_facts: u64::try_from(maximum_summary_facts).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_summary_facts,
                }
            })?,
            maximum_package_facts: u64::try_from(maximum_package_facts).map_err(|_| {
                BoundUnrepresentable {
                    requested: maximum_package_facts,
                }
            })?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackageHistoryTotals {
    package_count: u64,
    history_value_count: u64,
    history_evidence_count: u64,
    summary_fact_count: u64,
    package_fact_count: u64,
}

impl CallerRequestedFloat32AdvisoryReportPackageHistoryTotals {
    pub(crate) const fn package_count(&self) -> u64 {
        self.package_count
    }
    pub(crate) const fn history_value_count(&self) -> u64 {
        self.history_value_count
    }
    pub(crate) const fn history_evidence_count(&self) -> u64 {
        self.history_evidence_count
    }
    pub(crate) const fn summary_fact_count(&self) -> u64 {
        self.summary_fact_count
    }
    pub(crate) const fn package_fact_count(&self) -> u64 {
        self.package_fact_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackageHistory {
    bounds: CallerRequestedFloat32AdvisoryReportPackageHistoryBounds,
    packages: Vec<CallerRequestedFloat32AdvisoryReportPackage>,
    totals: CallerRequestedFloat32AdvisoryReportPackageHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError {
    CollectionLengthOverflow {
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    PackageLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    PackageCountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    CounterOverflow {
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    HistoryValueLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    HistoryEvidenceLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    SummaryFactLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    PackageFactLimit {
        limit: u64,
        required: u64,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
    Allocation {
        requested_packages: usize,
        history: CallerRequestedFloat32AdvisoryReportPackageHistory,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    },
}

impl CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32AdvisoryReportPackageHistory,
        CallerRequestedFloat32AdvisoryReportPackage,
    ) {
        use CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, package }
            | PackageLimit {
                history, package, ..
            }
            | PackageCountUnrepresentable {
                history, package, ..
            }
            | CounterOverflow { history, package }
            | HistoryValueLimit {
                history, package, ..
            }
            | HistoryEvidenceLimit {
                history, package, ..
            }
            | SummaryFactLimit {
                history, package, ..
            }
            | PackageFactLimit {
                history, package, ..
            }
            | Allocation {
                history, package, ..
            } => (history, package),
        }
    }
}

impl CallerRequestedFloat32AdvisoryReportPackageHistory {
    pub(crate) fn new(bounds: CallerRequestedFloat32AdvisoryReportPackageHistoryBounds) -> Self {
        Self {
            bounds,
            packages: Vec::new(),
            totals: Default::default(),
        }
    }

    pub(crate) fn packages(&self) -> &[CallerRequestedFloat32AdvisoryReportPackage] {
        &self.packages
    }
    pub(crate) const fn totals(&self) -> CallerRequestedFloat32AdvisoryReportPackageHistoryTotals {
        self.totals
    }
    pub(crate) fn into_packages(self) -> Vec<CallerRequestedFloat32AdvisoryReportPackage> {
        self.packages
    }

    pub(crate) fn append(
        self,
        package: CallerRequestedFloat32AdvisoryReportPackage,
    ) -> Result<Self, CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError> {
        self.append_with(
            package,
            |packages, requested| packages.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        package: CallerRequestedFloat32AdvisoryReportPackage,
        reserve: R,
        to_u64: C,
        mut add_u64: U,
        add_usize: Z,
    ) -> Result<Self, CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError>
    where
        R: FnOnce(&mut Vec<CallerRequestedFloat32AdvisoryReportPackage>, usize) -> Result<(), ()>,
        C: FnOnce(usize) -> Result<u64, ()>,
        U: FnMut(u64, u64) -> Result<u64, ()>,
        Z: FnOnce(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history: self, package }) }; }
        let next_len = match add_usize(self.packages.len(), 1) {
            Ok(v) => v,
            Err(()) => fail!(CollectionLengthOverflow {}),
        };
        if next_len > self.bounds.maximum_packages {
            fail!(PackageLimit {
                limit: self.bounds.maximum_packages,
                required: next_len
            });
        }
        let package_count = match to_u64(next_len) {
            Ok(v) => v,
            Err(()) => fail!(PackageCountUnrepresentable { actual: next_len }),
        };
        if package_count > self.bounds.maximum_packages_u64 {
            fail!(PackageLimit {
                limit: self.bounds.maximum_packages,
                required: next_len
            });
        }
        let candidate = package.totals();
        let history_value_count = match add_u64(
            self.totals.history_value_count,
            candidate.history_value_count(),
        ) {
            Ok(v) => v,
            Err(()) => fail!(CounterOverflow {}),
        };
        let history_evidence_count = match add_u64(
            self.totals.history_evidence_count,
            candidate.history_evidence_count(),
        ) {
            Ok(v) => v,
            Err(()) => fail!(CounterOverflow {}),
        };
        let summary_fact_count = match add_u64(
            self.totals.summary_fact_count,
            candidate.summary_fact_count(),
        ) {
            Ok(v) => v,
            Err(()) => fail!(CounterOverflow {}),
        };
        let package_fact_count = match add_u64(
            self.totals.package_fact_count,
            candidate.package_fact_count(),
        ) {
            Ok(v) => v,
            Err(()) => fail!(CounterOverflow {}),
        };
        for (required, limit, kind) in [
            (history_value_count, self.bounds.maximum_history_values, 0u8),
            (
                history_evidence_count,
                self.bounds.maximum_history_evidence,
                1,
            ),
            (summary_fact_count, self.bounds.maximum_summary_facts, 2),
            (package_fact_count, self.bounds.maximum_package_facts, 3),
        ] {
            if required > limit {
                return Err(match kind {
                    0 => HistoryValueLimit {
                        limit,
                        required,
                        history: self,
                        package,
                    },
                    1 => HistoryEvidenceLimit {
                        limit,
                        required,
                        history: self,
                        package,
                    },
                    2 => SummaryFactLimit {
                        limit,
                        required,
                        history: self,
                        package,
                    },
                    _ => PackageFactLimit {
                        limit,
                        required,
                        history: self,
                        package,
                    },
                });
            }
        }
        if reserve(&mut self.packages, 1).is_err() {
            fail!(Allocation {
                requested_packages: 1
            });
        }
        self.packages.push(package);
        self.totals = CallerRequestedFloat32AdvisoryReportPackageHistoryTotals {
            package_count,
            history_value_count,
            history_evidence_count,
            summary_fact_count,
            package_fact_count,
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
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
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

    fn evidence(sequence: u64, value: f32) -> crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidence{
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
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
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

    fn package(seed: u64) -> CallerRequestedFloat32AdvisoryReportPackage {
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(2)
            .unwrap()
            .append(evidence(seed, seed as f32))
            .unwrap()
            .append(evidence(seed + 2, seed as f32 + 2.0))
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(
            evidence(seed + 1, seed as f32 + 1.0),
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
                .unwrap()
                .append(snapshot())
                .unwrap(),
        )
        .unwrap();
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(2, 2, 2, 6).unwrap(),
        )
        .package(history, summary)
        .unwrap()
    }

    fn bounds(maximum_packages: usize) -> CallerRequestedFloat32AdvisoryReportPackageHistoryBounds {
        CallerRequestedFloat32AdvisoryReportPackageHistoryBounds::new(
            maximum_packages,
            maximum_packages * 2,
            maximum_packages * 2,
            maximum_packages * 2,
            maximum_packages * 6,
        )
        .unwrap()
    }

    fn pointers(package: &CallerRequestedFloat32AdvisoryReportPackage) -> Vec<*const f32> {
        package
            .history()
            .values()
            .iter()
            .map(|value| value.report().sample().sample().values().as_ptr())
            .chain(std::iter::once(
                package
                    .summary()
                    .retained()
                    .report()
                    .sample()
                    .sample()
                    .values()
                    .as_ptr(),
            ))
            .collect()
    }

    #[test]
    fn zero_exact_and_one_past_every_bound_are_explicit_and_atomic() {
        use CallerRequestedFloat32AdvisoryReportPackageHistoryConfigError::*;
        for (values, error) in [
            ([0, 1, 1, 1, 1], ZeroMaximumPackages),
            ([1, 0, 1, 1, 1], ZeroMaximumHistoryValues),
            ([1, 1, 0, 1, 1], ZeroMaximumHistoryEvidence),
            ([1, 1, 1, 0, 1], ZeroMaximumSummaryFacts),
            ([1, 1, 1, 1, 0], ZeroMaximumPackageFacts),
        ] {
            assert_eq!(
                CallerRequestedFloat32AdvisoryReportPackageHistoryBounds::new(
                    values[0], values[1], values[2], values[3], values[4]
                ),
                Err(error)
            );
        }
        if usize::BITS <= u64::BITS {
            let edge = CallerRequestedFloat32AdvisoryReportPackageHistoryBounds::new(
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
            )
            .unwrap();
            assert_eq!(edge.maximum_packages, usize::MAX);
            assert_eq!(edge.maximum_packages_u64, usize::MAX as u64);
        }
        let kept = package(10);
        let candidate = package(20);
        let candidate_pointers = pointers(&candidate);
        let history = CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds(1))
            .append(kept)
            .unwrap();
        assert_eq!(history.totals().package_count(), 1);
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError::PackageLimit {
                limit: 1,
                required: 2,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals().package_count(), 1);
        assert_eq!(pointers(&candidate), candidate_pointers);
    }

    #[test]
    fn repeated_append_retains_exact_order_package_and_nested_allocations() {
        let candidates = [package(1), package(11), package(21)];
        let package_pointers: Vec<_> = candidates.iter().map(|p| p.facts().as_ptr()).collect();
        let nested_pointers: Vec<_> = candidates.iter().map(pointers).collect();
        let history = candidates.into_iter().fold(
            CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds(3)),
            |history, package| history.append(package).unwrap(),
        );
        assert_eq!(history.totals().package_count(), 3);
        assert_eq!(history.totals().history_value_count(), 6);
        assert_eq!(history.totals().history_evidence_count(), 6);
        assert_eq!(history.totals().summary_fact_count(), 6);
        assert_eq!(history.totals().package_fact_count(), 18);
        assert_eq!(
            history
                .packages()
                .iter()
                .map(|p| p.facts().as_ptr())
                .collect::<Vec<_>>(),
            package_pointers
        );
        assert_eq!(
            history.packages().iter().map(pointers).collect::<Vec<_>>(),
            nested_pointers
        );
    }

    #[test]
    fn honest_allocation_refusal_returns_both_owners_bit_for_bit_unchanged() {
        let kept = package(30);
        let candidate = package(40);
        let kept_pointers = pointers(&kept);
        let candidate_pointers = pointers(&candidate);
        let history = CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds(2))
            .append(kept)
            .unwrap();
        let before = history.totals();
        let error = history
            .append_with(
                candidate,
                |_, requested| {
                    assert_eq!(requested, 1);
                    Err(())
                },
                |v| Ok(v as u64),
                |a, b| a.checked_add(b).ok_or(()),
                |a, b| a.checked_add(b).ok_or(()),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            CallerRequestedFloat32AdvisoryReportPackageHistoryAppendError::Allocation {
                requested_packages: 1,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(pointers(&history.packages()[0]), kept_pointers);
        assert_eq!(pointers(&candidate), candidate_pointers);
    }

    #[test]
    fn usize_conversion_and_every_u64_total_overflow_have_no_partial_commit() {
        for failure in 0usize..6 {
            let candidate = package(50 + failure as u64);
            let candidate_pointers = pointers(&candidate);
            let mut history = CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds(2));
            if failure >= 2 {
                match failure {
                    2 => history.totals.package_count = u64::MAX,
                    3 => history.totals.history_value_count = u64::MAX,
                    4 => history.totals.history_evidence_count = u64::MAX,
                    _ => history.totals.summary_fact_count = u64::MAX,
                }
            }
            let before = history.totals();
            let mut additions = 0usize;
            let error = history
                .append_with(
                    candidate,
                    |_, _| Ok(()),
                    |value| {
                        if failure == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |a, b| {
                        additions += 1;
                        if failure == 2 || additions == failure - 2 {
                            Err(())
                        } else {
                            a.checked_add(b).ok_or(())
                        }
                    },
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
            assert_eq!(history.totals(), before);
            assert!(history.packages().is_empty());
            assert_eq!(pointers(&candidate), candidate_pointers);
        }
        let candidate = package(70);
        let mut history = CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds(2));
        history.totals.package_fact_count = u64::MAX;
        let error = history.append(candidate).unwrap_err();
        let (history, _) = error.into_parts();
        assert_eq!(history.totals().package_fact_count(), u64::MAX);
    }

    #[test]
    fn aggregate_one_past_limits_return_unchanged_history_and_candidate() {
        for kind in 0..4 {
            let candidate = package(80 + kind);
            let candidate_pointers = pointers(&candidate);
            let limits = match kind {
                0 => [1, 2, 2, 6],
                1 => [2, 1, 2, 6],
                2 => [2, 2, 1, 6],
                _ => [2, 2, 2, 5],
            };
            let bounds = CallerRequestedFloat32AdvisoryReportPackageHistoryBounds::new(
                1, limits[0], limits[1], limits[2], limits[3],
            )
            .unwrap();
            let error = CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds)
                .append(candidate)
                .unwrap_err();
            let (history, candidate) = error.into_parts();
            assert_eq!(history.totals(), Default::default());
            assert!(history.packages().is_empty());
            assert_eq!(pointers(&candidate), candidate_pointers);
        }
    }

    #[test]
    fn consuming_extraction_preserves_order_and_allocation_identity() {
        let values = [package(90), package(100)];
        let expected: Vec<_> = values
            .iter()
            .map(|p| (p.facts().as_ptr(), pointers(p)))
            .collect();
        let packages = values
            .into_iter()
            .fold(
                CallerRequestedFloat32AdvisoryReportPackageHistory::new(bounds(2)),
                |history, package| history.append(package).unwrap(),
            )
            .into_packages();
        assert_eq!(
            packages
                .iter()
                .map(|p| (p.facts().as_ptr(), pointers(p)))
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn boundary_wording_is_private_default_inert_non_applying_and_non_equivalent() {
        let source = include_str!("caller_requested_float32_advisory_report_package_history.rs");
        for wording in [
            "crate-private",
            "default-inert",
            "infers neither loss nor continuity",
            "application, Manifold, session, stream, transport, or control authority",
            "no claim of liblsl equivalence",
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
            .contains("CallerRequestedFloat32AdvisoryReportPackageHistory"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_advisory_report_package_history"));
    }
}
