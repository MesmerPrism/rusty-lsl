// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded history of complete actual P40 Float32 advisory snapshots.
//!
//! This crate-private, default-inert owner retains caller-supplied snapshots in
//! insertion order without reconstructing their evidence. It is observation and
//! proposal evidence only: it does not infer loss or continuity, apply advice,
//! activate work, or grant Manifold, session, stream, transport, control, or
//! application authority, and it makes no claim of liblsl equivalence.

use crate::morphospace_float32_report_advisory_snapshot::MorphospaceFloat32ReportAdvisorySnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisorySnapshotHistoryConfigError {
    ZeroMaximumSnapshots,
    MaximumSnapshotsUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisorySnapshotHistoryTotals {
    snapshot_count: u64,
    evidence_count: u64,
}

impl MorphospaceFloat32ReportAdvisorySnapshotHistoryTotals {
    pub(crate) const fn snapshot_count(&self) -> u64 {
        self.snapshot_count
    }

    pub(crate) const fn evidence_count(&self) -> u64 {
        self.evidence_count
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportAdvisorySnapshotHistory {
    maximum_snapshots: usize,
    maximum_snapshots_u64: u64,
    snapshots: Vec<MorphospaceFloat32ReportAdvisorySnapshot>,
    totals: MorphospaceFloat32ReportAdvisorySnapshotHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError {
    CollectionLengthOverflow {
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    HistoryLimit {
        limit: usize,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    SnapshotCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    EvidenceCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    CounterOverflow {
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    },
    Allocation {
        requested_snapshots: usize,
        history: MorphospaceFloat32ReportAdvisorySnapshotHistory,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    },
}

impl MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportAdvisorySnapshotHistory,
        MorphospaceFloat32ReportAdvisorySnapshot,
    ) {
        use MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::*;
        match self {
            CollectionLengthOverflow { history, snapshot }
            | HistoryLimit {
                history, snapshot, ..
            }
            | SnapshotCountUnrepresentable {
                history, snapshot, ..
            }
            | EvidenceCountUnrepresentable {
                history, snapshot, ..
            }
            | CounterOverflow { history, snapshot }
            | Allocation {
                history, snapshot, ..
            } => (history, snapshot),
        }
    }
}

impl MorphospaceFloat32ReportAdvisorySnapshotHistory {
    pub(crate) fn new(
        maximum_snapshots: usize,
    ) -> Result<Self, MorphospaceFloat32ReportAdvisorySnapshotHistoryConfigError> {
        if maximum_snapshots == 0 {
            return Err(
                MorphospaceFloat32ReportAdvisorySnapshotHistoryConfigError::ZeroMaximumSnapshots,
            );
        }
        let maximum_snapshots_u64 = u64::try_from(maximum_snapshots).map_err(|_| {
            MorphospaceFloat32ReportAdvisorySnapshotHistoryConfigError::MaximumSnapshotsUnrepresentable {
                requested: maximum_snapshots,
            }
        })?;
        Ok(Self {
            maximum_snapshots,
            maximum_snapshots_u64,
            snapshots: Vec::new(),
            totals: Default::default(),
        })
    }

    pub(crate) const fn maximum_snapshots(&self) -> usize {
        self.maximum_snapshots
    }

    pub(crate) fn snapshots(&self) -> &[MorphospaceFloat32ReportAdvisorySnapshot] {
        &self.snapshots
    }

    pub(crate) const fn totals(&self) -> MorphospaceFloat32ReportAdvisorySnapshotHistoryTotals {
        self.totals
    }

    pub(crate) fn into_snapshots(self) -> Vec<MorphospaceFloat32ReportAdvisorySnapshot> {
        self.snapshots
    }

    pub(crate) fn append(
        self,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
    ) -> Result<Self, MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError> {
        self.append_with(
            snapshot,
            |snapshots, requested| snapshots.try_reserve_exact(requested).map_err(|_| ()),
            |value| u64::try_from(value).map_err(|_| ()),
            |left, right| left.checked_add(right).ok_or(()),
            |left, right| left.checked_add(right).ok_or(()),
        )
    }

    fn append_with<R, C, U, Z>(
        mut self,
        snapshot: MorphospaceFloat32ReportAdvisorySnapshot,
        reserve: R,
        mut to_u64: C,
        mut add_u64: U,
        mut add_usize: Z,
    ) -> Result<Self, MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportAdvisorySnapshot>, usize) -> Result<(), ()>,
        C: FnMut(usize) -> Result<u64, ()>,
        U: FnMut(u64, u64) -> Result<u64, ()>,
        Z: FnMut(usize, usize) -> Result<usize, ()>,
    {
        use MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::*;
        let next_len = match add_usize(self.snapshots.len(), 1) {
            Ok(value) => value,
            Err(()) => {
                return Err(CollectionLengthOverflow {
                    history: self,
                    snapshot,
                })
            }
        };
        if next_len > self.maximum_snapshots {
            return Err(HistoryLimit {
                limit: self.maximum_snapshots,
                history: self,
                snapshot,
            });
        }
        let next_count = match to_u64(next_len) {
            Ok(value) => value,
            Err(()) => {
                return Err(SnapshotCountUnrepresentable {
                    actual: next_len,
                    history: self,
                    snapshot,
                })
            }
        };
        if next_count > self.maximum_snapshots_u64 {
            return Err(HistoryLimit {
                limit: self.maximum_snapshots,
                history: self,
                snapshot,
            });
        }
        let evidence_usize = snapshot.evidence().len();
        let evidence_count = match to_u64(evidence_usize) {
            Ok(value) => value,
            Err(()) => {
                return Err(EvidenceCountUnrepresentable {
                    actual: evidence_usize,
                    history: self,
                    snapshot,
                })
            }
        };
        let snapshot_count = match add_u64(self.totals.snapshot_count, 1) {
            Ok(value) => value,
            Err(()) => {
                return Err(CounterOverflow {
                    history: self,
                    snapshot,
                })
            }
        };
        let evidence_count = match add_u64(self.totals.evidence_count, evidence_count) {
            Ok(value) => value,
            Err(()) => {
                return Err(CounterOverflow {
                    history: self,
                    snapshot,
                })
            }
        };
        if reserve(&mut self.snapshots, 1).is_err() {
            return Err(Allocation {
                requested_snapshots: 1,
                history: self,
                snapshot,
            });
        }
        self.snapshots.push(snapshot);
        self.totals = MorphospaceFloat32ReportAdvisorySnapshotHistoryTotals {
            snapshot_count,
            evidence_count,
        };
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_observation::{
        tests::outcome, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
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
            .observe(outcome(vec![value.to_bits() as u64], vec![record]))
            .unwrap();
        MorphospaceFloat32ReportObservationWindow::new(1, 1)
            .unwrap()
            .append(observation)
            .unwrap()
    }

    fn snapshot(value: f32) -> MorphospaceFloat32ReportAdvisorySnapshot {
        let observation_history = MorphospaceFloat32ReportObservationHistory::new(1, 1)
            .unwrap()
            .append(window(value))
            .unwrap();
        let delta = MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
            MorphospaceFloat32ReportWindowDeltaBounds::new(1, 1, 12).unwrap(),
        )
        .propose(window(value), window(value + 1.0))
        .unwrap();
        let delta_history = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 12)
            .unwrap()
            .append(delta)
            .unwrap();
        let stability_history = MorphospaceFloat32ReportObservationHistory::new(1, 1)
            .unwrap()
            .append(window(value))
            .unwrap();
        let stability_proposal = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(stability_history)
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 12, 1, 14).unwrap(),
        )
        .snapshot(observation_history, delta_history, stability_proposal)
        .unwrap()
    }

    fn sample_pointer(snapshot: &MorphospaceFloat32ReportAdvisorySnapshot) -> *const f32 {
        snapshot.observation_history().windows()[0].observations()[0].records()[0]
            .processed()
            .sample()
            .sample()
            .values()
            .as_ptr()
    }

    #[test]
    fn zero_bound_and_platform_edge_are_explicit() {
        assert_eq!(
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(0),
            Err(MorphospaceFloat32ReportAdvisorySnapshotHistoryConfigError::ZeroMaximumSnapshots)
        );
        let history = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(usize::MAX).unwrap();
        assert_eq!(history.maximum_snapshots(), usize::MAX);
        assert!(history.snapshots().is_empty());
        assert_eq!(history.totals(), Default::default());
    }

    #[test]
    fn actual_p40_snapshots_keep_caller_order_and_complete_allocation_identity() {
        let values = [snapshot(1.0), snapshot(3.0), snapshot(1.0)];
        let expected: Vec<_> = values.iter().map(sample_pointer).collect();
        let history = values.into_iter().fold(
            MorphospaceFloat32ReportAdvisorySnapshotHistory::new(3).unwrap(),
            |history, snapshot| history.append(snapshot).unwrap(),
        );
        assert_eq!(history.totals().snapshot_count(), 3);
        assert_eq!(history.totals().evidence_count(), 39);
        assert_eq!(
            history
                .snapshots()
                .iter()
                .map(sample_pointer)
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn capacity_failure_is_atomic_and_candidate_can_be_retried() {
        let kept = snapshot(2.0);
        let candidate = snapshot(4.0);
        let kept_pointer = sample_pointer(&kept);
        let candidate_pointer = sample_pointer(&candidate);
        let history = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(kept)
            .unwrap();
        let before = history.totals();
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::HistoryLimit {
                limit: 1,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(sample_pointer(&history.snapshots()[0]), kept_pointer);
        assert_eq!(sample_pointer(&candidate), candidate_pointer);
        let retry = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(candidate)
            .unwrap();
        assert_eq!(sample_pointer(&retry.snapshots()[0]), candidate_pointer);
    }

    #[test]
    fn allocation_failure_rolls_back_without_moving_evidence() {
        let candidate = snapshot(f32::from_bits(0x7f7f_ffff));
        let pointer = sample_pointer(&candidate);
        let error = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
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
            MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::Allocation {
                requested_snapshots: 1,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert!(history.snapshots().is_empty());
        assert_eq!(history.totals(), Default::default());
        assert_eq!(sample_pointer(&candidate), pointer);
    }

    #[test]
    fn usize_and_u64_edge_failures_return_every_owner_unchanged() {
        for failure in 0..4 {
            let candidate = snapshot(8.0 + failure as f32);
            let pointer = sample_pointer(&candidate);
            let mut history = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(2).unwrap();
            if failure == 2 {
                history.totals.snapshot_count = u64::MAX;
            }
            if failure == 3 {
                history.totals.evidence_count = u64::MAX;
            }
            let before = history.totals();
            let mut calls = 0;
            let error = history
                .append_with(
                    candidate,
                    |_, _| Ok(()),
                    |value| {
                        calls += 1;
                        if (failure == 1 && calls == 1) || (failure == 0 && calls == 2) {
                            Err(())
                        } else {
                            Ok(value as u64)
                        }
                    },
                    |a, b| a.checked_add(b).ok_or(()),
                    |a, b| a.checked_add(b).ok_or(()),
                )
                .unwrap_err();
            assert!(matches!(
                (&error, failure),
                (MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::EvidenceCountUnrepresentable { .. }, 0)
                    | (MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::SnapshotCountUnrepresentable { .. }, 1)
                    | (MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::CounterOverflow { .. }, 2 | 3)
            ));
            let (history, candidate) = error.into_parts();
            assert_eq!(history.totals(), before);
            assert!(history.snapshots().is_empty());
            assert_eq!(sample_pointer(&candidate), pointer);
        }
        let candidate = snapshot(13.0);
        let pointer = sample_pointer(&candidate);
        let error = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append_with(
                candidate,
                |_, _| Ok(()),
                |v| Ok(v as u64),
                |a, b| a.checked_add(b).ok_or(()),
                |_, _| Err(()),
            )
            .unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportAdvisorySnapshotHistoryAppendError::CollectionLengthOverflow { .. }
        ));
        let (history, candidate) = error.into_parts();
        assert!(history.snapshots().is_empty());
        assert_eq!(sample_pointer(&candidate), pointer);
    }

    #[test]
    fn consuming_extraction_preserves_snapshot_order_and_allocations() {
        let values = [snapshot(5.0), snapshot(6.0)];
        let expected: Vec<_> = values.iter().map(sample_pointer).collect();
        let snapshots = values
            .into_iter()
            .fold(
                MorphospaceFloat32ReportAdvisorySnapshotHistory::new(2).unwrap(),
                |history, snapshot| history.append(snapshot).unwrap(),
            )
            .into_snapshots();
        assert_eq!(
            snapshots.iter().map(sample_pointer).collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn owner_is_private_default_inert_and_denies_external_authority() {
        let source = include_str!("morphospace_float32_report_advisory_snapshot_history.rs");
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(
            !include_str!("runtime.rs").contains("MorphospaceFloat32ReportAdvisorySnapshotHistory")
        );
    }
}
