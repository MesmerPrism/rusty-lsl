// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-requested, bounded snapshot of actual P38/P39 Float32 advisory evidence.
//!
//! The crate-private owner only retains the three complete caller inputs and a
//! deterministic, allocation-fallible evidence index. It neither reconstructs
//! observations nor applies, accepts, routes, authorizes, or activates advice.

use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
use crate::morphospace_float32_report_window_delta_proposal::MorphospaceFloat32ReportWindowDeltaEvidence;
use crate::morphospace_float32_report_window_stability_proposal::{
    MorphospaceFloat32ReportWindowStabilityEvidence,
    MorphospaceFloat32ReportWindowStabilityProposal,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisorySnapshotConfigError {
    ZeroMaximumObservationWindows,
    ZeroMaximumDeltas,
    ZeroMaximumDeltaEvidence,
    ZeroMaximumStabilityEvidence,
    ZeroMaximumOrderedEvidence,
    BoundUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisorySnapshotBounds {
    maximum_observation_windows: usize,
    maximum_deltas: usize,
    maximum_delta_evidence: usize,
    maximum_stability_evidence: usize,
    maximum_ordered_evidence: usize,
}

impl MorphospaceFloat32ReportAdvisorySnapshotBounds {
    pub(crate) fn new(
        maximum_observation_windows: usize,
        maximum_deltas: usize,
        maximum_delta_evidence: usize,
        maximum_stability_evidence: usize,
        maximum_ordered_evidence: usize,
    ) -> Result<Self, MorphospaceFloat32ReportAdvisorySnapshotConfigError> {
        use MorphospaceFloat32ReportAdvisorySnapshotConfigError::*;
        for (value, error) in [
            (maximum_observation_windows, ZeroMaximumObservationWindows),
            (maximum_deltas, ZeroMaximumDeltas),
            (maximum_delta_evidence, ZeroMaximumDeltaEvidence),
            (maximum_stability_evidence, ZeroMaximumStabilityEvidence),
            (maximum_ordered_evidence, ZeroMaximumOrderedEvidence),
        ] {
            if value == 0 {
                return Err(error);
            }
            u64::try_from(value).map_err(|_| BoundUnrepresentable { requested: value })?;
        }
        Ok(Self {
            maximum_observation_windows,
            maximum_deltas,
            maximum_delta_evidence,
            maximum_stability_evidence,
            maximum_ordered_evidence,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisorySnapshotEvidence {
    ObservationWindow {
        window_index: u64,
    },
    Delta {
        delta_index: u64,
        evidence_index: u64,
        evidence: MorphospaceFloat32ReportWindowDeltaEvidence,
    },
    Stability {
        evidence_index: u64,
        evidence: MorphospaceFloat32ReportWindowStabilityEvidence,
    },
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisorySnapshot {
    observation_history: MorphospaceFloat32ReportObservationHistory,
    delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
    stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    evidence: Vec<MorphospaceFloat32ReportAdvisorySnapshotEvidence>,
}

impl MorphospaceFloat32ReportAdvisorySnapshot {
    pub(crate) fn observation_history(&self) -> &MorphospaceFloat32ReportObservationHistory {
        &self.observation_history
    }
    pub(crate) fn delta_history(&self) -> &MorphospaceFloat32ReportWindowDeltaHistory {
        &self.delta_history
    }
    pub(crate) fn stability_proposal(&self) -> &MorphospaceFloat32ReportWindowStabilityProposal {
        &self.stability_proposal
    }
    pub(crate) fn evidence(&self) -> &[MorphospaceFloat32ReportAdvisorySnapshotEvidence] {
        &self.evidence
    }
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportObservationHistory,
        MorphospaceFloat32ReportWindowDeltaHistory,
        MorphospaceFloat32ReportWindowStabilityProposal,
    ) {
        (
            self.observation_history,
            self.delta_history,
            self.stability_proposal,
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisorySnapshotError {
    ObservationWindowLimit {
        limit: usize,
        actual: usize,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    DeltaLimit {
        limit: usize,
        actual: usize,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    DeltaEvidenceLimit {
        delta_index: u64,
        limit: usize,
        actual: usize,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    StabilityEvidenceLimit {
        limit: usize,
        actual: usize,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    IndexUnrepresentable {
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    EvidenceCountOverflow {
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    OrderedEvidenceLimit {
        limit: usize,
        required: usize,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
    Allocation {
        requested: usize,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    },
}

impl MorphospaceFloat32ReportAdvisorySnapshotError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportObservationHistory,
        MorphospaceFloat32ReportWindowDeltaHistory,
        MorphospaceFloat32ReportWindowStabilityProposal,
    ) {
        use MorphospaceFloat32ReportAdvisorySnapshotError::*;
        match self {
            ObservationWindowLimit {
                observation_history,
                delta_history,
                stability_proposal,
                ..
            }
            | DeltaLimit {
                observation_history,
                delta_history,
                stability_proposal,
                ..
            }
            | DeltaEvidenceLimit {
                observation_history,
                delta_history,
                stability_proposal,
                ..
            }
            | StabilityEvidenceLimit {
                observation_history,
                delta_history,
                stability_proposal,
                ..
            }
            | IndexUnrepresentable {
                observation_history,
                delta_history,
                stability_proposal,
            }
            | EvidenceCountOverflow {
                observation_history,
                delta_history,
                stability_proposal,
            }
            | OrderedEvidenceLimit {
                observation_history,
                delta_history,
                stability_proposal,
                ..
            }
            | Allocation {
                observation_history,
                delta_history,
                stability_proposal,
                ..
            } => (observation_history, delta_history, stability_proposal),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisorySnapshotOwner {
    bounds: MorphospaceFloat32ReportAdvisorySnapshotBounds,
}

impl MorphospaceFloat32ReportAdvisorySnapshotOwner {
    pub(crate) const fn new(bounds: MorphospaceFloat32ReportAdvisorySnapshotBounds) -> Self {
        Self { bounds }
    }

    pub(crate) fn snapshot(
        &self,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
    ) -> Result<
        MorphospaceFloat32ReportAdvisorySnapshot,
        MorphospaceFloat32ReportAdvisorySnapshotError,
    > {
        self.snapshot_with(
            observation_history,
            delta_history,
            stability_proposal,
            |e, n| e.try_reserve_exact(n).map_err(|_| ()),
            |a, b| a.checked_add(b).ok_or(()),
        )
    }

    fn snapshot_with<R, A>(
        &self,
        observation_history: MorphospaceFloat32ReportObservationHistory,
        delta_history: MorphospaceFloat32ReportWindowDeltaHistory,
        stability_proposal: MorphospaceFloat32ReportWindowStabilityProposal,
        reserve: R,
        mut add: A,
    ) -> Result<
        MorphospaceFloat32ReportAdvisorySnapshot,
        MorphospaceFloat32ReportAdvisorySnapshotError,
    >
    where
        R: FnOnce(
            &mut Vec<MorphospaceFloat32ReportAdvisorySnapshotEvidence>,
            usize,
        ) -> Result<(), ()>,
        A: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32ReportAdvisorySnapshotError::*;
        let windows = observation_history.windows().len();
        let deltas = delta_history.deltas().len();
        let stability = stability_proposal.evidence().len();
        macro_rules! fail { ($variant:ident { $($field:ident : $value:expr),* $(,)? }) => { return Err($variant { $($field: $value,)* observation_history, delta_history, stability_proposal }) }; }
        if windows > self.bounds.maximum_observation_windows {
            fail!(ObservationWindowLimit {
                limit: self.bounds.maximum_observation_windows,
                actual: windows
            });
        }
        if deltas > self.bounds.maximum_deltas {
            fail!(DeltaLimit {
                limit: self.bounds.maximum_deltas,
                actual: deltas
            });
        }
        if stability > self.bounds.maximum_stability_evidence {
            fail!(StabilityEvidenceLimit {
                limit: self.bounds.maximum_stability_evidence,
                actual: stability
            });
        }
        if u64::try_from(windows).is_err()
            || u64::try_from(deltas).is_err()
            || u64::try_from(stability).is_err()
        {
            fail!(IndexUnrepresentable {});
        }
        let mut required = match add(windows, stability) {
            Ok(v) => v,
            Err(_) => fail!(EvidenceCountOverflow {}),
        };
        for (di, delta) in delta_history.deltas().iter().enumerate() {
            let actual = delta.evidence().len();
            let delta_index = match u64::try_from(di) {
                Ok(v) => v,
                Err(_) => fail!(IndexUnrepresentable {}),
            };
            if actual > self.bounds.maximum_delta_evidence {
                fail!(DeltaEvidenceLimit {
                    delta_index: delta_index,
                    limit: self.bounds.maximum_delta_evidence,
                    actual: actual
                });
            }
            required = match add(required, actual) {
                Ok(v) => v,
                Err(_) => fail!(EvidenceCountOverflow {}),
            };
        }
        if required > self.bounds.maximum_ordered_evidence {
            fail!(OrderedEvidenceLimit {
                limit: self.bounds.maximum_ordered_evidence,
                required: required
            });
        }
        let mut evidence = Vec::new();
        if reserve(&mut evidence, required).is_err() {
            fail!(Allocation {
                requested: required
            });
        }
        for wi in 0..windows {
            evidence.push(
                MorphospaceFloat32ReportAdvisorySnapshotEvidence::ObservationWindow {
                    window_index: u64::try_from(wi).expect("validated index"),
                },
            );
        }
        for (di, delta) in delta_history.deltas().iter().enumerate() {
            for (ei, item) in delta.evidence().iter().copied().enumerate() {
                evidence.push(MorphospaceFloat32ReportAdvisorySnapshotEvidence::Delta {
                    delta_index: u64::try_from(di).expect("validated index"),
                    evidence_index: u64::try_from(ei).expect("bounded evidence index"),
                    evidence: item,
                });
            }
        }
        for (ei, item) in stability_proposal.evidence().iter().copied().enumerate() {
            evidence.push(
                MorphospaceFloat32ReportAdvisorySnapshotEvidence::Stability {
                    evidence_index: u64::try_from(ei).expect("bounded evidence index"),
                    evidence: item,
                },
            );
        }
        Ok(MorphospaceFloat32ReportAdvisorySnapshot {
            observation_history,
            delta_history,
            stability_proposal,
            evidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_observation::{
        tests::outcome, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
    use crate::morphospace_float32_report_window_delta_proposal::{
        MorphospaceFloat32ReportWindowDeltaBounds, MorphospaceFloat32ReportWindowDeltaProposalOwner,
    };
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn window(value: f32) -> MorphospaceFloat32ReportObservationWindow {
        let record = TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(value as f64 + 10.0).unwrap(),
            None,
        );
        let observation = MorphospaceFloat32ReportObservationOwner::new(1)
            .unwrap()
            .observe(outcome(vec![0], vec![record]))
            .unwrap();
        MorphospaceFloat32ReportObservationWindow::new(1, 1)
            .unwrap()
            .append(observation)
            .unwrap()
    }

    fn history(values: &[f32]) -> MorphospaceFloat32ReportObservationHistory {
        values.iter().copied().fold(
            MorphospaceFloat32ReportObservationHistory::new(values.len().max(1), 1).unwrap(),
            |h, v| h.append(window(v)).unwrap(),
        )
    }

    fn inputs(
        values: &[f32],
    ) -> (
        MorphospaceFloat32ReportObservationHistory,
        MorphospaceFloat32ReportWindowDeltaHistory,
        MorphospaceFloat32ReportWindowStabilityProposal,
        [Vec<*const f32>; 3],
    ) {
        let observation_history = history(values);
        let observation_pointers = pointers(&observation_history);
        let delta_values = if values.len() >= 2 {
            (values[0], values[1])
        } else {
            (1.0, 1.0)
        };
        let delta_earlier = window(delta_values.0);
        let delta_later = window(delta_values.1);
        let delta_pointers = [
            delta_earlier.observations()[0].records()[0]
                .processed()
                .sample()
                .sample()
                .values()
                .as_ptr(),
            delta_later.observations()[0].records()[0]
                .processed()
                .sample()
                .sample()
                .values()
                .as_ptr(),
        ]
        .to_vec();
        let delta = MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
            MorphospaceFloat32ReportWindowDeltaBounds::new(1, 1, 12).unwrap(),
        )
        .propose(delta_earlier, delta_later)
        .unwrap();
        let delta_history = MorphospaceFloat32ReportWindowDeltaHistory::new(2, 12)
            .unwrap()
            .append(delta)
            .unwrap();
        let stability_history = history(values);
        let stability_pointers = pointers(&stability_history);
        let stability_proposal = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(
                values.len().max(1),
                1,
                1,
                values.len().saturating_sub(1).saturating_mul(12).max(1),
                0,
                0.0,
            )
            .unwrap(),
        )
        .propose(stability_history)
        .unwrap();
        (
            observation_history,
            delta_history,
            stability_proposal,
            [observation_pointers, delta_pointers, stability_pointers],
        )
    }

    fn owner(max_ordered: usize) -> MorphospaceFloat32ReportAdvisorySnapshotOwner {
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(4, 2, 12, 36, max_ordered).unwrap(),
        )
    }

    fn pointers(history: &MorphospaceFloat32ReportObservationHistory) -> Vec<*const f32> {
        history
            .windows()
            .iter()
            .flat_map(|w| w.observations())
            .flat_map(|o| o.records())
            .map(|r| r.processed().sample().sample().values().as_ptr())
            .collect()
    }

    #[test]
    fn zero_bounds_and_extreme_bounds_are_explicit() {
        use MorphospaceFloat32ReportAdvisorySnapshotConfigError::*;
        let cases = [
            (0, 1, 1, 1, 1, ZeroMaximumObservationWindows),
            (1, 0, 1, 1, 1, ZeroMaximumDeltas),
            (1, 1, 0, 1, 1, ZeroMaximumDeltaEvidence),
            (1, 1, 1, 0, 1, ZeroMaximumStabilityEvidence),
            (1, 1, 1, 1, 0, ZeroMaximumOrderedEvidence),
        ];
        for (a, b, c, d, e, expected) in cases {
            assert_eq!(
                MorphospaceFloat32ReportAdvisorySnapshotBounds::new(a, b, c, d, e),
                Err(expected)
            );
        }
        assert!(MorphospaceFloat32ReportAdvisorySnapshotBounds::new(
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX
        )
        .is_ok());
    }

    #[test]
    fn actual_repeated_equal_and_changing_evidence_has_fixed_order_and_identity() {
        for values in [&[3.0, 3.0][..], &[3.0, 7.0][..], &[7.0, 3.0][..]] {
            let (h, d, s, expected) = inputs(values);
            let snapshot = owner(26).snapshot(h, d, s).unwrap();
            assert_eq!(pointers(snapshot.observation_history()), expected[0]);
            assert_eq!(snapshot.evidence().len(), 26);
            assert!(matches!(
                snapshot.evidence()[0],
                MorphospaceFloat32ReportAdvisorySnapshotEvidence::ObservationWindow {
                    window_index: 0
                }
            ));
            assert!(matches!(
                snapshot.evidence()[1],
                MorphospaceFloat32ReportAdvisorySnapshotEvidence::ObservationWindow {
                    window_index: 1
                }
            ));
            assert!(matches!(
                snapshot.evidence()[2],
                MorphospaceFloat32ReportAdvisorySnapshotEvidence::Delta {
                    delta_index: 0,
                    evidence_index: 0,
                    ..
                }
            ));
            assert!(matches!(
                snapshot.evidence()[25],
                MorphospaceFloat32ReportAdvisorySnapshotEvidence::Stability {
                    evidence_index: 11,
                    ..
                }
            ));
            let (h, d, s) = snapshot.into_parts();
            assert_eq!(pointers(&h), expected[0]);
            let delta_windows = d.into_deltas().pop().unwrap().into_windows();
            let actual_delta = [delta_windows.0, delta_windows.1]
                .iter()
                .flat_map(|w| w.observations())
                .flat_map(|o| o.records())
                .map(|r| r.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>();
            assert_eq!(actual_delta, expected[1]);
            assert_eq!(pointers(&s.into_history()), expected[2]);
        }
    }

    #[test]
    fn empty_actual_histories_snapshot_without_invented_evidence() {
        let h = history(&[]);
        let d = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap();
        let s = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(history(&[]))
        .unwrap();
        let snapshot = owner(1).snapshot(h, d, s).unwrap();
        assert!(snapshot.evidence().is_empty());
        assert!(snapshot.observation_history().windows().is_empty());
        assert!(snapshot.delta_history().deltas().is_empty());
        assert!(snapshot.stability_proposal().evidence().is_empty());
    }

    #[test]
    fn every_limit_allocation_and_overflow_failure_rolls_back_all_inputs() {
        for kind in 0..7 {
            let (h, mut d, s, expected) = inputs(&[1.0, 2.0]);
            if kind == 1 {
                let extra = MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
                    MorphospaceFloat32ReportWindowDeltaBounds::new(1, 1, 12).unwrap(),
                )
                .propose(window(2.0), window(3.0))
                .unwrap();
                d = d.append(extra).unwrap();
            }
            let error = match kind {
                0 => MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
                    MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 2, 12, 36, 40).unwrap(),
                )
                .snapshot(h, d, s)
                .unwrap_err(),
                1 => MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
                    MorphospaceFloat32ReportAdvisorySnapshotBounds::new(4, 1, 12, 36, 52).unwrap(),
                )
                .snapshot(h, d, s)
                .unwrap_err(),
                2 => MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
                    MorphospaceFloat32ReportAdvisorySnapshotBounds::new(4, 2, 11, 36, 40).unwrap(),
                )
                .snapshot(h, d, s)
                .unwrap_err(),
                3 => MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
                    MorphospaceFloat32ReportAdvisorySnapshotBounds::new(4, 2, 12, 11, 40).unwrap(),
                )
                .snapshot(h, d, s)
                .unwrap_err(),
                4 => owner(25).snapshot(h, d, s).unwrap_err(),
                5 => owner(40)
                    .snapshot_with(h, d, s, |_, _| Err(()), |a, b| a.checked_add(b).ok_or(()))
                    .unwrap_err(),
                _ => owner(40)
                    .snapshot_with(h, d, s, |_, _| Ok(()), |_, _| Err(()))
                    .unwrap_err(),
            };
            let (h, d, s) = error.into_parts();
            assert_eq!(pointers(&h), expected[0]);
            let mut deltas = d.into_deltas();
            let first = deltas.remove(0).into_windows();
            let actual_delta = [first.0, first.1]
                .iter()
                .flat_map(|w| w.observations())
                .flat_map(|o| o.records())
                .map(|r| r.processed().sample().sample().values().as_ptr())
                .collect::<Vec<_>>();
            assert_eq!(actual_delta, expected[1]);
            assert_eq!(pointers(&s.into_history()), expected[2]);
        }
    }

    #[test]
    fn source_and_shared_surfaces_remain_non_applying_and_private() {
        let source = include_str!("morphospace_float32_report_advisory_snapshot.rs");
        for forbidden in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
            concat!("fn act", "ivate("),
        ] {
            assert!(!source.contains(forbidden));
        }
        assert!(!include_str!("runtime.rs").contains("MorphospaceFloat32ReportAdvisorySnapshot"));
    }
}
