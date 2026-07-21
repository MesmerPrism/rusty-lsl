// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested package of actual P42 history and retained advisory summary.
//!
//! The package retains both canonical inputs unchanged and builds only a
//! bounded deterministic index of `Copy` facts they already expose. It is
//! crate-private, default-inert, non-applying, and infers neither loss nor
//! continuity.

use crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidenceItem;
use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
use crate::morphospace_float32_retained_advisory_summary::{
    MorphospaceFloat32RetainedAdvisorySummary, MorphospaceFloat32RetainedAdvisorySummaryFact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32AdvisoryReportPackageConfigError {
    ZeroMaximumHistoryValues,
    ZeroMaximumHistoryEvidence,
    ZeroMaximumSummaryFacts,
    ZeroMaximumPackageFacts,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackageBounds {
    maximum_history_values: usize,
    maximum_history_evidence: usize,
    maximum_summary_facts: usize,
    maximum_package_facts: usize,
}

impl CallerRequestedFloat32AdvisoryReportPackageBounds {
    pub(crate) fn new(
        maximum_history_values: usize,
        maximum_history_evidence: usize,
        maximum_summary_facts: usize,
        maximum_package_facts: usize,
    ) -> Result<Self, CallerRequestedFloat32AdvisoryReportPackageConfigError> {
        use CallerRequestedFloat32AdvisoryReportPackageConfigError::*;
        for (value, zero) in [
            (maximum_history_values, ZeroMaximumHistoryValues),
            (maximum_history_evidence, ZeroMaximumHistoryEvidence),
            (maximum_summary_facts, ZeroMaximumSummaryFacts),
            (maximum_package_facts, ZeroMaximumPackageFacts),
        ] {
            if value == 0 {
                return Err(zero);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_history_values,
            maximum_history_evidence,
            maximum_summary_facts,
            maximum_package_facts,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32AdvisoryReportPackageFact {
    HistoryValue {
        history_index: u64,
        ordered_evidence_count: u64,
    },
    HistoryEvidence {
        history_index: u64,
        evidence_index: u64,
        evidence: CallerRequestedFloat32ReportAdvisoryEvidenceItem,
    },
    RetainedSummaryFact {
        summary_index: u64,
        fact: MorphospaceFloat32RetainedAdvisorySummaryFact,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackageTotals {
    history_value_count: u64,
    history_evidence_count: u64,
    summary_fact_count: u64,
    package_fact_count: u64,
}

impl CallerRequestedFloat32AdvisoryReportPackageTotals {
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
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackage {
    history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
    summary: MorphospaceFloat32RetainedAdvisorySummary,
    totals: CallerRequestedFloat32AdvisoryReportPackageTotals,
    facts: Vec<CallerRequestedFloat32AdvisoryReportPackageFact>,
}

impl CallerRequestedFloat32AdvisoryReportPackage {
    pub(crate) const fn history(&self) -> &CallerRequestedFloat32ReportAdvisoryEvidenceHistory {
        &self.history
    }
    pub(crate) const fn summary(&self) -> &MorphospaceFloat32RetainedAdvisorySummary {
        &self.summary
    }
    pub(crate) const fn totals(&self) -> CallerRequestedFloat32AdvisoryReportPackageTotals {
        self.totals
    }
    pub(crate) fn facts(&self) -> &[CallerRequestedFloat32AdvisoryReportPackageFact] {
        &self.facts
    }
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        MorphospaceFloat32RetainedAdvisorySummary,
    ) {
        (self.history, self.summary)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallerRequestedFloat32AdvisoryReportPackageError {
    HistoryValueLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
    HistoryEvidenceLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
    SummaryFactLimit {
        limit: usize,
        actual: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
    PackageFactLimit {
        limit: usize,
        required: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
    CountUnrepresentable {
        actual: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
    CountOverflow {
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
    Allocation {
        requested: usize,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    },
}

impl CallerRequestedFloat32AdvisoryReportPackageError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        MorphospaceFloat32RetainedAdvisorySummary,
    ) {
        use CallerRequestedFloat32AdvisoryReportPackageError::*;
        match self {
            HistoryValueLimit {
                history, summary, ..
            }
            | HistoryEvidenceLimit {
                history, summary, ..
            }
            | SummaryFactLimit {
                history, summary, ..
            }
            | PackageFactLimit {
                history, summary, ..
            }
            | CountUnrepresentable {
                history, summary, ..
            }
            | CountOverflow { history, summary }
            | Allocation {
                history, summary, ..
            } => (history, summary),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32AdvisoryReportPackageOwner {
    bounds: CallerRequestedFloat32AdvisoryReportPackageBounds,
}

impl CallerRequestedFloat32AdvisoryReportPackageOwner {
    pub(crate) const fn new(bounds: CallerRequestedFloat32AdvisoryReportPackageBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn package(
        &self,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
    ) -> Result<
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackageError,
    > {
        self.package_with(
            history,
            summary,
            |facts, requested| facts.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn package_with<R, C, A>(
        &self,
        history: CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: MorphospaceFloat32RetainedAdvisorySummary,
        reserve: R,
        mut convert: C,
        mut add: A,
    ) -> Result<
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackageError,
    >
    where
        R: FnOnce(
            &mut Vec<CallerRequestedFloat32AdvisoryReportPackageFact>,
            usize,
        ) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use CallerRequestedFloat32AdvisoryReportPackageError::*;
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* history, summary }) }; }

        let history_value_count = history.values().len();
        if history_value_count > self.bounds.maximum_history_values {
            fail!(HistoryValueLimit {
                limit: self.bounds.maximum_history_values,
                actual: history_value_count
            });
        }
        let history_evidence_count = history
            .values()
            .iter()
            .try_fold(0usize, |total, value| add(total, value.ordered().len()));
        let history_evidence_count = match history_evidence_count {
            Ok(value) => value,
            Err(()) => fail!(CountOverflow {}),
        };
        if history_evidence_count > self.bounds.maximum_history_evidence {
            fail!(HistoryEvidenceLimit {
                limit: self.bounds.maximum_history_evidence,
                actual: history_evidence_count
            });
        }
        let summary_fact_count = summary.facts().len();
        if summary_fact_count > self.bounds.maximum_summary_facts {
            fail!(SummaryFactLimit {
                limit: self.bounds.maximum_summary_facts,
                actual: summary_fact_count
            });
        }
        let required = match add(history_value_count, history_evidence_count) {
            Ok(value) => value,
            Err(()) => fail!(CountOverflow {}),
        };
        let required = match add(required, summary_fact_count) {
            Ok(value) => value,
            Err(()) => fail!(CountOverflow {}),
        };
        if required > self.bounds.maximum_package_facts {
            fail!(PackageFactLimit {
                limit: self.bounds.maximum_package_facts,
                required: required
            });
        }

        let history_value_count_u64 = match convert(history_value_count) {
            Ok(value) => value,
            Err(()) => fail!(CountUnrepresentable {
                actual: history_value_count
            }),
        };
        let history_evidence_count_u64 = match convert(history_evidence_count) {
            Ok(value) => value,
            Err(()) => fail!(CountUnrepresentable {
                actual: history_evidence_count
            }),
        };
        let summary_fact_count_u64 = match convert(summary_fact_count) {
            Ok(value) => value,
            Err(()) => fail!(CountUnrepresentable {
                actual: summary_fact_count
            }),
        };
        let required_u64 = match convert(required) {
            Ok(value) => value,
            Err(()) => fail!(CountUnrepresentable { actual: required }),
        };
        for (history_index, value) in history.values().iter().enumerate() {
            if convert(history_index).is_err() {
                fail!(CountUnrepresentable {
                    actual: history_index
                });
            }
            if convert(value.ordered().len()).is_err() {
                fail!(CountUnrepresentable {
                    actual: value.ordered().len()
                });
            }
            for evidence_index in 0..value.ordered().len() {
                if convert(evidence_index).is_err() {
                    fail!(CountUnrepresentable {
                        actual: evidence_index
                    });
                }
            }
        }
        for summary_index in 0..summary_fact_count {
            if convert(summary_index).is_err() {
                fail!(CountUnrepresentable {
                    actual: summary_index
                });
            }
        }

        let mut facts = Vec::new();
        if reserve(&mut facts, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for (history_index, value) in history.values().iter().enumerate() {
            let history_index = u64::try_from(history_index).expect("validated history index");
            facts.push(
                CallerRequestedFloat32AdvisoryReportPackageFact::HistoryValue {
                    history_index,
                    ordered_evidence_count: u64::try_from(value.ordered().len())
                        .expect("validated evidence count"),
                },
            );
            facts.extend(value.ordered().iter().copied().enumerate().map(
                |(evidence_index, evidence)| {
                    CallerRequestedFloat32AdvisoryReportPackageFact::HistoryEvidence {
                        history_index,
                        evidence_index: u64::try_from(evidence_index)
                            .expect("validated evidence index"),
                        evidence,
                    }
                },
            ));
        }
        facts.extend(
            summary
                .facts()
                .iter()
                .copied()
                .enumerate()
                .map(|(summary_index, fact)| {
                    CallerRequestedFloat32AdvisoryReportPackageFact::RetainedSummaryFact {
                        summary_index: u64::try_from(summary_index)
                            .expect("validated summary index"),
                        fact,
                    }
                }),
        );
        Ok(CallerRequestedFloat32AdvisoryReportPackage {
            history,
            summary,
            totals: CallerRequestedFloat32AdvisoryReportPackageTotals {
                history_value_count: history_value_count_u64,
                history_evidence_count: history_evidence_count_u64,
                summary_fact_count: summary_fact_count_u64,
                package_fact_count: required_u64,
            },
            facts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
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

    fn evidence(sequence: u64, sample_value: f32) -> crate::caller_requested_float32_report_advisory_evidence::CallerRequestedFloat32ReportAdvisoryEvidence{
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
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![sample_value]).unwrap(),
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

    fn inputs() -> (
        CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        MorphospaceFloat32RetainedAdvisorySummary,
    ) {
        let history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(2)
            .unwrap()
            .append(evidence(42, 4.0))
            .unwrap()
            .append(evidence(44, 6.0))
            .unwrap();
        let retained = evidence(43, 5.0);
        let history_snapshots = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(snapshot())
            .unwrap();
        let summary = MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(retained, history_snapshots)
        .unwrap();
        (history, summary)
    }

    fn owner() -> CallerRequestedFloat32AdvisoryReportPackageOwner {
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(2, 2, 2, 6).unwrap(),
        )
    }

    fn pointers(
        history: &CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: &MorphospaceFloat32RetainedAdvisorySummary,
    ) -> Vec<*const f32> {
        history
            .values()
            .iter()
            .map(|value| value.report().sample().sample().values().as_ptr())
            .chain(std::iter::once(
                summary
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
    fn actual_p42_history_and_retained_summary_are_packaged_unchanged() {
        let (history, summary) = inputs();
        let before = pointers(&history, &summary);
        let package = owner().package(history, summary).unwrap();
        assert_eq!(pointers(package.history(), package.summary()), before);
        assert_eq!(package.totals().history_value_count(), 2);
        assert_eq!(package.totals().history_evidence_count(), 2);
        assert_eq!(package.totals().summary_fact_count(), 2);
        assert_eq!(package.totals().package_fact_count(), 6);
        assert_eq!(package.facts().len(), 6);
        assert!(matches!(
            package.facts()[0],
            CallerRequestedFloat32AdvisoryReportPackageFact::HistoryValue {
                history_index: 0,
                ordered_evidence_count: 1
            }
        ));
        assert!(matches!(
            package.facts()[1],
            CallerRequestedFloat32AdvisoryReportPackageFact::HistoryEvidence {
                history_index: 0,
                evidence_index: 0,
                ..
            }
        ));
        assert!(matches!(
            package.facts()[4],
            CallerRequestedFloat32AdvisoryReportPackageFact::RetainedSummaryFact {
                summary_index: 0,
                ..
            }
        ));
        let (history, summary) = package.into_parts();
        assert_eq!(pointers(&history, &summary), before);
    }

    #[test]
    fn limits_and_every_fallible_stage_return_both_inputs_unchanged() {
        for failure in 0..4 {
            let (history, summary) = inputs();
            let before = pointers(&history, &summary);
            let mut conversions = 0;
            let mut additions = 0;
            let error = owner()
                .package_with(
                    history,
                    summary,
                    |_, _| if failure == 3 { Err(()) } else { Ok(()) },
                    |value| {
                        conversions += 1;
                        if failure == 2 && conversions == 1 {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |left, right| {
                        additions += 1;
                        if failure < 2 && additions == failure + 1 {
                            Err(())
                        } else {
                            left.checked_add(right).ok_or(())
                        }
                    },
                )
                .unwrap_err();
            let (history, summary) = error.into_parts();
            assert_eq!(pointers(&history, &summary), before);
            assert_eq!(history.totals().value_count(), 2);
            assert_eq!(summary.totals().summary_fact_count(), 2);
        }
        for bounds in [(1, 2, 2, 6), (2, 1, 2, 6), (2, 2, 1, 6), (2, 2, 2, 5)] {
            let (history, summary) = inputs();
            let before = pointers(&history, &summary);
            let error = CallerRequestedFloat32AdvisoryReportPackageOwner::new(
                CallerRequestedFloat32AdvisoryReportPackageBounds::new(
                    bounds.0, bounds.1, bounds.2, bounds.3,
                )
                .unwrap(),
            )
            .package(history, summary)
            .unwrap_err();
            let (history, summary) = error.into_parts();
            assert_eq!(pointers(&history, &summary), before);
        }
    }

    #[test]
    fn configuration_and_private_default_inert_boundary_are_exact() {
        use CallerRequestedFloat32AdvisoryReportPackageConfigError::*;
        assert_eq!(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(0, 1, 1, 1),
            Err(ZeroMaximumHistoryValues)
        );
        assert_eq!(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 0, 1, 1),
            Err(ZeroMaximumHistoryEvidence)
        );
        assert_eq!(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 0, 1),
            Err(ZeroMaximumSummaryFacts)
        );
        assert_eq!(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(1, 1, 1, 0),
            Err(ZeroMaximumPackageFacts)
        );
        let source = include_str!("caller_requested_float32_advisory_report_package.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs").contains("CallerRequestedFloat32AdvisoryReportPackage"));
        assert!(!include_str!("lib.rs")
            .contains("pub mod caller_requested_float32_advisory_report_package"));
        assert!(!include_str!("lib.rs")
            .contains("pub use caller_requested_float32_advisory_report_package"));
    }
}
