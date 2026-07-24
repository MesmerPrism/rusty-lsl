// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded ownership of complete actual P38 Float32 window-delta proposals.
//!
//! This crate-private, default-inert owner retains caller-supplied proposals in
//! insertion order. It does not reconstruct evidence or infer loss, is not a
//! claim of liblsl equivalence, and grants no Manifold or application admission,
//! authorization, routing, application, or audit authority.

use crate::morphospace_float32_report_window_delta_proposal::MorphospaceFloat32ReportWindowDeltaProposal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaHistoryConfigError {
    ZeroMaximumDeltas,
    ZeroMaximumEvidencePerDelta,
    MaximumDeltasUnrepresentable { requested: usize },
    MaximumEvidencePerDeltaUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowDeltaHistoryTotals {
    delta_count: u64,
    window_count: u64,
    evidence_count: u64,
}

impl MorphospaceFloat32ReportWindowDeltaHistoryTotals {
    pub(crate) const fn delta_count(&self) -> u64 {
        self.delta_count
    }
    pub(crate) const fn window_count(&self) -> u64 {
        self.window_count
    }
    pub(crate) const fn evidence_count(&self) -> u64 {
        self.evidence_count
    }

    fn checked_with(self, evidence_count: u64) -> Option<Self> {
        Some(Self {
            delta_count: self.delta_count.checked_add(1)?,
            window_count: self.window_count.checked_add(2)?,
            evidence_count: self.evidence_count.checked_add(evidence_count)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportWindowDeltaHistory {
    maximum_deltas: usize,
    maximum_evidence_per_delta: usize,
    maximum_evidence_per_delta_u64: u64,
    deltas: Vec<MorphospaceFloat32ReportWindowDeltaProposal>,
    totals: MorphospaceFloat32ReportWindowDeltaHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportWindowDeltaHistoryAppendError {
    HistoryLimit {
        limit: usize,
        history: MorphospaceFloat32ReportWindowDeltaHistory,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    },
    EvidenceCountUnrepresentable {
        actual: usize,
        history: MorphospaceFloat32ReportWindowDeltaHistory,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    },
    EvidenceLimit {
        limit: usize,
        actual: u64,
        history: MorphospaceFloat32ReportWindowDeltaHistory,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    },
    CollectionLengthOverflow {
        history: MorphospaceFloat32ReportWindowDeltaHistory,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    },
    CounterOverflow {
        history: MorphospaceFloat32ReportWindowDeltaHistory,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    },
    Allocation {
        requested_deltas: usize,
        history: MorphospaceFloat32ReportWindowDeltaHistory,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    },
}

impl MorphospaceFloat32ReportWindowDeltaHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportWindowDeltaHistory,
        MorphospaceFloat32ReportWindowDeltaProposal,
    ) {
        match self {
            Self::HistoryLimit {
                history, proposal, ..
            }
            | Self::EvidenceCountUnrepresentable {
                history, proposal, ..
            }
            | Self::EvidenceLimit {
                history, proposal, ..
            }
            | Self::CollectionLengthOverflow { history, proposal }
            | Self::CounterOverflow { history, proposal }
            | Self::Allocation {
                history, proposal, ..
            } => (history, proposal),
        }
    }
}

impl MorphospaceFloat32ReportWindowDeltaHistory {
    pub(crate) fn new(
        maximum_deltas: usize,
        maximum_evidence_per_delta: usize,
    ) -> Result<Self, MorphospaceFloat32ReportWindowDeltaHistoryConfigError> {
        if maximum_deltas == 0 {
            return Err(MorphospaceFloat32ReportWindowDeltaHistoryConfigError::ZeroMaximumDeltas);
        }
        if maximum_evidence_per_delta == 0 {
            return Err(
                MorphospaceFloat32ReportWindowDeltaHistoryConfigError::ZeroMaximumEvidencePerDelta,
            );
        }
        u64::try_from(maximum_deltas).map_err(|_| {
            MorphospaceFloat32ReportWindowDeltaHistoryConfigError::MaximumDeltasUnrepresentable {
                requested: maximum_deltas,
            }
        })?;
        let maximum_evidence_per_delta_u64 = u64::try_from(maximum_evidence_per_delta).map_err(|_| MorphospaceFloat32ReportWindowDeltaHistoryConfigError::MaximumEvidencePerDeltaUnrepresentable { requested: maximum_evidence_per_delta })?;
        Ok(Self {
            maximum_deltas,
            maximum_evidence_per_delta,
            maximum_evidence_per_delta_u64,
            deltas: Vec::new(),
            totals: Default::default(),
        })
    }

    pub(crate) const fn maximum_deltas(&self) -> usize {
        self.maximum_deltas
    }
    pub(crate) const fn maximum_evidence_per_delta(&self) -> usize {
        self.maximum_evidence_per_delta
    }
    pub(crate) fn deltas(&self) -> &[MorphospaceFloat32ReportWindowDeltaProposal] {
        &self.deltas
    }
    pub(crate) const fn totals(&self) -> MorphospaceFloat32ReportWindowDeltaHistoryTotals {
        self.totals
    }
    pub(crate) fn into_deltas(self) -> Vec<MorphospaceFloat32ReportWindowDeltaProposal> {
        self.deltas
    }

    pub(crate) fn append(
        self,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
    ) -> Result<Self, MorphospaceFloat32ReportWindowDeltaHistoryAppendError> {
        self.append_with(proposal, |deltas, requested| {
            deltas.try_reserve_exact(requested).map_err(|_| ())
        })
    }

    fn append_with<R>(
        mut self,
        proposal: MorphospaceFloat32ReportWindowDeltaProposal,
        reserve: R,
    ) -> Result<Self, MorphospaceFloat32ReportWindowDeltaHistoryAppendError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportWindowDeltaProposal>, usize) -> Result<(), ()>,
    {
        let Some(next_len) = self.deltas.len().checked_add(1) else {
            return Err(
                MorphospaceFloat32ReportWindowDeltaHistoryAppendError::CollectionLengthOverflow {
                    history: self,
                    proposal,
                },
            );
        };
        if next_len > self.maximum_deltas {
            return Err(
                MorphospaceFloat32ReportWindowDeltaHistoryAppendError::HistoryLimit {
                    limit: self.maximum_deltas,
                    history: self,
                    proposal,
                },
            );
        }
        let evidence_usize = proposal.evidence().len();
        let evidence_count = match u64::try_from(evidence_usize) {
            Ok(value) => value,
            Err(_) => return Err(MorphospaceFloat32ReportWindowDeltaHistoryAppendError::EvidenceCountUnrepresentable { actual: evidence_usize, history: self, proposal }),
        };
        if evidence_count > self.maximum_evidence_per_delta_u64 {
            return Err(
                MorphospaceFloat32ReportWindowDeltaHistoryAppendError::EvidenceLimit {
                    limit: self.maximum_evidence_per_delta,
                    actual: evidence_count,
                    history: self,
                    proposal,
                },
            );
        }
        let Some(totals) = self.totals.checked_with(evidence_count) else {
            return Err(
                MorphospaceFloat32ReportWindowDeltaHistoryAppendError::CounterOverflow {
                    history: self,
                    proposal,
                },
            );
        };
        if reserve(&mut self.deltas, 1).is_err() {
            return Err(
                MorphospaceFloat32ReportWindowDeltaHistoryAppendError::Allocation {
                    requested_deltas: 1,
                    history: self,
                    proposal,
                },
            );
        }
        self.deltas.push(proposal);
        self.totals = totals;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_observation::{
        tests::outcome, MorphospaceFloat32ReportObservationOwner,
    };
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;
    use crate::morphospace_float32_report_window_delta_proposal::{
        MorphospaceFloat32ReportWindowDeltaBounds, MorphospaceFloat32ReportWindowDeltaProposalOwner,
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

    fn proposal(a: f32, b: f32) -> (MorphospaceFloat32ReportWindowDeltaProposal, Vec<*const f32>) {
        let windows = MorphospaceFloat32ReportObservationHistory::new(2, 1)
            .unwrap()
            .append(window(a))
            .unwrap()
            .append(window(b))
            .unwrap()
            .into_windows();
        let pointers = windows.iter().flat_map(window_pointers).collect();
        let mut windows = windows.into_iter();
        let proposal = MorphospaceFloat32ReportWindowDeltaProposalOwner::new(
            MorphospaceFloat32ReportWindowDeltaBounds::new(1, 1, 12).unwrap(),
        )
        .propose(windows.next().unwrap(), windows.next().unwrap())
        .unwrap();
        (proposal, pointers)
    }

    fn window_pointers(window: &MorphospaceFloat32ReportObservationWindow) -> Vec<*const f32> {
        window
            .observations()
            .iter()
            .flat_map(|o| o.records())
            .map(|r| r.processed().sample().sample().values().as_ptr())
            .collect()
    }

    fn extracted_pointers(value: MorphospaceFloat32ReportWindowDeltaProposal) -> Vec<*const f32> {
        let (earlier, later) = value.into_windows();
        [window_pointers(&earlier), window_pointers(&later)].concat()
    }

    #[test]
    fn nonzero_bounds_and_platform_extremes_are_explicit() {
        use MorphospaceFloat32ReportWindowDeltaHistoryConfigError::*;
        assert_eq!(
            MorphospaceFloat32ReportWindowDeltaHistory::new(0, 1),
            Err(ZeroMaximumDeltas)
        );
        assert_eq!(
            MorphospaceFloat32ReportWindowDeltaHistory::new(1, 0),
            Err(ZeroMaximumEvidencePerDelta)
        );
        let history =
            MorphospaceFloat32ReportWindowDeltaHistory::new(usize::MAX, usize::MAX).unwrap();
        assert_eq!(history.maximum_deltas(), usize::MAX);
        assert_eq!(history.maximum_evidence_per_delta(), usize::MAX);
        assert_eq!(history.totals(), Default::default());
    }

    #[test]
    fn p38_p39_composition_keeps_delta_order_evidence_and_exact_sample_allocations() {
        let values = [
            proposal(1.0, 2.0),
            proposal(2.0, 3.0),
            proposal(3.0, 1.0),
            proposal(1.0, 2.0),
        ];
        let expected: Vec<_> = values
            .iter()
            .map(|(p, pointers)| (p.evidence().to_vec(), pointers.clone()))
            .collect();
        let values = values.map(|(proposal, _)| proposal);
        let history = values.into_iter().fold(
            MorphospaceFloat32ReportWindowDeltaHistory::new(4, 12).unwrap(),
            |h, p| h.append(p).unwrap(),
        );
        assert_eq!(history.totals().delta_count(), 4);
        assert_eq!(history.totals().window_count(), 8);
        assert_eq!(history.totals().evidence_count(), 48);
        for (actual, (evidence, _)) in history.deltas().iter().zip(&expected) {
            assert_eq!(actual.evidence(), evidence);
        }
        for (actual, (_, allocations)) in history.into_deltas().into_iter().zip(expected) {
            assert_eq!(extracted_pointers(actual), allocations);
        }
    }

    #[test]
    fn consuming_extraction_preserves_proposal_order_and_sample_identity() {
        let values = [proposal(7.0, 8.0), proposal(8.0, 7.0)];
        let expected: Vec<_> = values
            .iter()
            .map(|(_, pointers)| pointers.clone())
            .collect();
        let values = values.map(|(proposal, _)| proposal);
        let deltas = values
            .into_iter()
            .fold(
                MorphospaceFloat32ReportWindowDeltaHistory::new(2, 12).unwrap(),
                |h, p| h.append(p).unwrap(),
            )
            .into_deltas();
        assert_eq!(deltas.len(), 2);
        for (actual, expected) in deltas.into_iter().zip(expected) {
            assert_eq!(extracted_pointers(actual), expected);
        }
    }

    #[test]
    fn p38_p39_typed_failures_return_complete_inputs_without_partial_mutation() {
        let (kept, kept_ptrs) = proposal(1.0, 2.0);
        let (candidate, candidate_ptrs) = proposal(2.0, 4.0);
        let history = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 12)
            .unwrap()
            .append(kept)
            .unwrap();
        let before = history.totals();
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportWindowDeltaHistoryAppendError::HistoryLimit { limit: 1, .. }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(
            extracted_pointers(history.into_deltas().pop().unwrap()),
            kept_ptrs
        );
        let retry = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 12)
            .unwrap()
            .append(candidate)
            .unwrap();
        assert_eq!(
            extracted_pointers(retry.into_deltas().pop().unwrap()),
            candidate_ptrs
        );

        let (candidate, candidate_ptrs) = proposal(5.0, 6.0);
        let error = MorphospaceFloat32ReportWindowDeltaHistory::new(2, 11)
            .unwrap()
            .append(candidate)
            .unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportWindowDeltaHistoryAppendError::EvidenceLimit {
                limit: 11,
                actual: 12,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert!(history.deltas().is_empty());
        let history = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 12)
            .unwrap()
            .append(candidate)
            .unwrap();
        assert_eq!(
            extracted_pointers(history.into_deltas().pop().unwrap()),
            candidate_ptrs
        );

        let (candidate, candidate_ptrs) = proposal(9.0, 10.0);
        let error = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 12)
            .unwrap()
            .append_with(candidate, |_, requested| {
                assert_eq!(requested, 1);
                Err(())
            })
            .unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportWindowDeltaHistoryAppendError::Allocation {
                requested_deltas: 1,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert!(history.deltas().is_empty());
        assert_eq!(
            extracted_pointers(
                history
                    .append(candidate)
                    .unwrap()
                    .into_deltas()
                    .pop()
                    .unwrap()
            ),
            candidate_ptrs
        );
    }

    #[test]
    fn every_checked_total_overflow_returns_unchanged_live_owners() {
        let setters: [fn(&mut MorphospaceFloat32ReportWindowDeltaHistoryTotals); 3] = [
            |t| t.delta_count = u64::MAX,
            |t| t.window_count = u64::MAX - 1,
            |t| t.evidence_count = u64::MAX - 11,
        ];
        for set in setters {
            let (candidate, candidate_ptrs) = proposal(11.0, 12.0);
            let mut history = MorphospaceFloat32ReportWindowDeltaHistory::new(1, 12).unwrap();
            set(&mut history.totals);
            let before = history.totals();
            let error = history.append(candidate).unwrap_err();
            assert!(matches!(
                error,
                MorphospaceFloat32ReportWindowDeltaHistoryAppendError::CounterOverflow { .. }
            ));
            let (mut history, candidate) = error.into_parts();
            assert_eq!(history.totals(), before);
            assert!(history.deltas().is_empty());
            history.totals = Default::default();
            let retry = history.append(candidate).unwrap();
            assert_eq!(
                extracted_pointers(retry.into_deltas().pop().unwrap()),
                candidate_ptrs
            );
        }
    }

    #[test]
    fn owner_is_private_default_inert_and_denies_external_authority() {
        let source = include_str!("morphospace_float32_report_window_delta_history.rs");
        for operation in [
            concat!("fn ap", "ply("),
            concat!("fn ac", "cept("),
            concat!("fn act", "ivate("),
            concat!("fn ro", "ute("),
            concat!("fn auth", "orize("),
        ] {
            assert!(!source.contains(operation));
        }
        assert!(!include_str!("runtime.rs").contains("MorphospaceFloat32ReportWindowDeltaHistory"));
    }
}
