// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Project-owned bounded history of exact P37 Float32 report observation windows.
//!
//! This crate-private, default-inert owner retains only complete caller-supplied
//! windows and their original allocations and evidence. It is not a claim of
//! liblsl equivalence and grants no Manifold or application admission,
//! application, routing, policy, or other authority.

use crate::morphospace_float32_report_observation_window::MorphospaceFloat32ReportObservationWindow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportObservationHistoryConfigError {
    ZeroMaximumWindows,
    ZeroMaximumReportsPerWindow,
    MaximumWindowsUnrepresentable { requested: usize },
    MaximumReportsPerWindowUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportObservationHistoryTotals {
    window_count: u64,
    report_count: u64,
    record_count: u64,
}

impl MorphospaceFloat32ReportObservationHistoryTotals {
    pub(crate) const fn window_count(&self) -> u64 {
        self.window_count
    }

    pub(crate) const fn report_count(&self) -> u64 {
        self.report_count
    }

    pub(crate) const fn record_count(&self) -> u64 {
        self.record_count
    }

    fn checked_with(self, window: &MorphospaceFloat32ReportObservationWindow) -> Option<Self> {
        let window_totals = window.totals();
        Some(Self {
            window_count: self.window_count.checked_add(1)?,
            report_count: self
                .report_count
                .checked_add(window_totals.report_count())?,
            record_count: self
                .record_count
                .checked_add(window_totals.record_count())?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportObservationHistory {
    maximum_windows: usize,
    maximum_reports_per_window: usize,
    maximum_reports_per_window_u64: u64,
    windows: Vec<MorphospaceFloat32ReportObservationWindow>,
    totals: MorphospaceFloat32ReportObservationHistoryTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportObservationHistoryAppendError {
    EmptyWindow {
        history: MorphospaceFloat32ReportObservationHistory,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    HistoryLimit {
        limit: usize,
        history: MorphospaceFloat32ReportObservationHistory,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    WindowReportLimit {
        limit: usize,
        actual: u64,
        history: MorphospaceFloat32ReportObservationHistory,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    CounterOverflow {
        history: MorphospaceFloat32ReportObservationHistory,
        window: MorphospaceFloat32ReportObservationWindow,
    },
    Allocation {
        requested_windows: usize,
        history: MorphospaceFloat32ReportObservationHistory,
        window: MorphospaceFloat32ReportObservationWindow,
    },
}

impl MorphospaceFloat32ReportObservationHistoryAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportObservationHistory,
        MorphospaceFloat32ReportObservationWindow,
    ) {
        match self {
            Self::EmptyWindow { history, window }
            | Self::HistoryLimit {
                history, window, ..
            }
            | Self::WindowReportLimit {
                history, window, ..
            }
            | Self::CounterOverflow { history, window }
            | Self::Allocation {
                history, window, ..
            } => (history, window),
        }
    }
}

impl MorphospaceFloat32ReportObservationHistory {
    pub(crate) fn new(
        maximum_windows: usize,
        maximum_reports_per_window: usize,
    ) -> Result<Self, MorphospaceFloat32ReportObservationHistoryConfigError> {
        if maximum_windows == 0 {
            return Err(MorphospaceFloat32ReportObservationHistoryConfigError::ZeroMaximumWindows);
        }
        if maximum_reports_per_window == 0 {
            return Err(
                MorphospaceFloat32ReportObservationHistoryConfigError::ZeroMaximumReportsPerWindow,
            );
        }
        u64::try_from(maximum_windows).map_err(|_| {
            MorphospaceFloat32ReportObservationHistoryConfigError::MaximumWindowsUnrepresentable {
                requested: maximum_windows,
            }
        })?;
        let maximum_reports_per_window_u64 =
            u64::try_from(maximum_reports_per_window).map_err(|_| {
                MorphospaceFloat32ReportObservationHistoryConfigError::MaximumReportsPerWindowUnrepresentable {
                    requested: maximum_reports_per_window,
                }
            })?;
        Ok(Self {
            maximum_windows,
            maximum_reports_per_window,
            maximum_reports_per_window_u64,
            windows: Vec::new(),
            totals: MorphospaceFloat32ReportObservationHistoryTotals::default(),
        })
    }

    pub(crate) const fn maximum_windows(&self) -> usize {
        self.maximum_windows
    }

    pub(crate) const fn maximum_reports_per_window(&self) -> usize {
        self.maximum_reports_per_window
    }

    pub(crate) fn windows(&self) -> &[MorphospaceFloat32ReportObservationWindow] {
        &self.windows
    }

    pub(crate) const fn totals(&self) -> MorphospaceFloat32ReportObservationHistoryTotals {
        self.totals
    }

    pub(crate) fn append(
        self,
        window: MorphospaceFloat32ReportObservationWindow,
    ) -> Result<Self, MorphospaceFloat32ReportObservationHistoryAppendError> {
        self.append_with(window, |windows, requested| {
            windows.try_reserve_exact(requested).map_err(|_| ())
        })
    }

    fn append_with<R>(
        mut self,
        window: MorphospaceFloat32ReportObservationWindow,
        reserve: R,
    ) -> Result<Self, MorphospaceFloat32ReportObservationHistoryAppendError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportObservationWindow>, usize) -> Result<(), ()>,
    {
        let report_count = window.totals().report_count();
        if report_count == 0 {
            return Err(
                MorphospaceFloat32ReportObservationHistoryAppendError::EmptyWindow {
                    history: self,
                    window,
                },
            );
        }
        if self.windows.len() >= self.maximum_windows {
            return Err(
                MorphospaceFloat32ReportObservationHistoryAppendError::HistoryLimit {
                    limit: self.maximum_windows,
                    history: self,
                    window,
                },
            );
        }
        if report_count > self.maximum_reports_per_window_u64 {
            return Err(
                MorphospaceFloat32ReportObservationHistoryAppendError::WindowReportLimit {
                    limit: self.maximum_reports_per_window,
                    actual: report_count,
                    history: self,
                    window,
                },
            );
        }
        let Some(totals) = self.totals.checked_with(&window) else {
            return Err(
                MorphospaceFloat32ReportObservationHistoryAppendError::CounterOverflow {
                    history: self,
                    window,
                },
            );
        };
        if reserve(&mut self.windows, 1).is_err() {
            return Err(
                MorphospaceFloat32ReportObservationHistoryAppendError::Allocation {
                    requested_windows: 1,
                    history: self,
                    window,
                },
            );
        }
        self.windows.push(window);
        self.totals = totals;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphospace_float32_report_observation::tests::outcome;
    use crate::morphospace_float32_report_observation::MorphospaceFloat32ReportObservationOwner;
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn observation(
        values: &[f32],
    ) -> crate::morphospace_float32_report_observation::MorphospaceFloat32ReportObservation {
        let records = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![*value]).unwrap(),
                    RawSourceTimestamp::new(index as f64 + 1.0).unwrap(),
                    None,
                )
            })
            .collect::<Vec<_>>();
        let observation = MorphospaceFloat32ReportObservationOwner::new(records.len())
            .unwrap()
            .observe(outcome((0..records.len() as u64).collect(), records))
            .unwrap();
        observation
    }

    fn window(values: &[f32]) -> MorphospaceFloat32ReportObservationWindow {
        MorphospaceFloat32ReportObservationWindow::new(1, values.len().max(1))
            .unwrap()
            .append(observation(values))
            .unwrap()
    }

    fn two_report_window() -> MorphospaceFloat32ReportObservationWindow {
        MorphospaceFloat32ReportObservationWindow::new(2, 2)
            .unwrap()
            .append(observation(&[2.0]))
            .unwrap()
            .append(observation(&[3.0]))
            .unwrap()
    }

    fn pointers(window: &MorphospaceFloat32ReportObservationWindow) -> Vec<*const f32> {
        window
            .observations()
            .iter()
            .flat_map(|observation| observation.records())
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect()
    }

    #[test]
    fn empty_history_and_zero_bounds_are_explicit() {
        assert_eq!(
            MorphospaceFloat32ReportObservationHistory::new(0, 1),
            Err(MorphospaceFloat32ReportObservationHistoryConfigError::ZeroMaximumWindows)
        );
        assert_eq!(
            MorphospaceFloat32ReportObservationHistory::new(1, 0),
            Err(MorphospaceFloat32ReportObservationHistoryConfigError::ZeroMaximumReportsPerWindow)
        );
        let history = MorphospaceFloat32ReportObservationHistory::new(2, 3).unwrap();
        assert_eq!(history.maximum_windows(), 2);
        assert_eq!(history.maximum_reports_per_window(), 3);
        assert!(history.windows().is_empty());
        assert_eq!(history.totals(), Default::default());
    }

    #[test]
    fn first_middle_final_and_repeated_windows_preserve_order_and_allocations() {
        let first = window(&[1.0]);
        let middle = window(&[2.0, 3.0]);
        let final_window = window(&[1.0]);
        let expected = [pointers(&first), pointers(&middle), pointers(&final_window)];
        let history = MorphospaceFloat32ReportObservationHistory::new(3, 1)
            .unwrap()
            .append(first)
            .unwrap()
            .append(middle)
            .unwrap()
            .append(final_window)
            .unwrap();
        assert_eq!(history.windows().len(), 3);
        for (actual, expected) in history.windows().iter().zip(expected) {
            assert_eq!(pointers(actual), expected);
        }
        assert_eq!(history.totals().window_count(), 3);
        assert_eq!(history.totals().report_count(), 3);
        assert_eq!(history.totals().record_count(), 4);
    }

    #[test]
    fn empty_candidate_and_capacity_boundaries_are_atomic() {
        let empty = MorphospaceFloat32ReportObservationWindow::new(1, 1).unwrap();
        let history = MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap();
        let (history, empty) = history.append(empty).unwrap_err().into_parts();
        assert!(history.windows().is_empty());
        assert!(empty.observations().is_empty());

        let kept = window(&[1.0]);
        let kept_pointers = pointers(&kept);
        let candidate = window(&[2.0]);
        let candidate_pointers = pointers(&candidate);
        let history = history.append(kept).unwrap();
        let before = history.totals();
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportObservationHistoryAppendError::HistoryLimit { limit: 1, .. }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(pointers(&history.windows()[0]), kept_pointers);
        assert_eq!(pointers(&candidate), candidate_pointers);
    }

    #[test]
    fn per_window_bound_failure_returns_owners_and_can_retry() {
        let first = window(&[1.0]);
        let candidate = two_report_window();
        let first_pointers = pointers(&first);
        let candidate_pointers = pointers(&candidate);
        let history = MorphospaceFloat32ReportObservationHistory::new(2, 1)
            .unwrap()
            .append(first)
            .unwrap();
        let before = history.totals();
        let error = history.append(candidate).unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportObservationHistoryAppendError::WindowReportLimit {
                limit: 1,
                actual: 2,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert_eq!(history.totals(), before);
        assert_eq!(pointers(&history.windows()[0]), first_pointers);
        assert_eq!(pointers(&candidate), candidate_pointers);
        let retry = window(&[2.0]);
        let retry_pointers = pointers(&retry);
        let history = history.append(retry).unwrap();
        assert_eq!(history.windows().len(), 2);
        assert_eq!(pointers(&history.windows()[1]), retry_pointers);
    }

    #[test]
    fn allocation_failure_is_atomic_and_retry_preserves_identity() {
        let candidate = window(&[f32::from_bits(0x7f7f_ffff)]);
        let candidate_pointers = pointers(&candidate);
        let history = MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap();
        let error = history
            .append_with(candidate, |_, requested| {
                assert_eq!(requested, 1);
                Err(())
            })
            .unwrap_err();
        assert!(matches!(
            error,
            MorphospaceFloat32ReportObservationHistoryAppendError::Allocation {
                requested_windows: 1,
                ..
            }
        ));
        let (history, candidate) = error.into_parts();
        assert!(history.windows().is_empty());
        assert_eq!(history.totals(), Default::default());
        assert_eq!(pointers(&candidate), candidate_pointers);
        let history = history.append(candidate).unwrap();
        assert_eq!(pointers(&history.windows()[0]), candidate_pointers);
    }

    #[test]
    fn checked_counter_extremes_return_both_owners_without_partial_mutation() {
        let setters: [fn(&mut MorphospaceFloat32ReportObservationHistoryTotals); 3] = [
            |totals| totals.window_count = u64::MAX,
            |totals| totals.report_count = u64::MAX,
            |totals| totals.record_count = u64::MAX,
        ];
        for set_extreme in setters {
            let candidate = window(&[9.0]);
            let candidate_pointers = pointers(&candidate);
            let mut history =
                MorphospaceFloat32ReportObservationHistory::new(usize::MAX, usize::MAX).unwrap();
            set_extreme(&mut history.totals);
            let before = history.totals();
            let error = history.append(candidate).unwrap_err();
            assert!(matches!(
                error,
                MorphospaceFloat32ReportObservationHistoryAppendError::CounterOverflow { .. }
            ));
            let (history, candidate) = error.into_parts();
            assert_eq!(history.totals(), before);
            assert!(history.windows().is_empty());
            assert_eq!(pointers(&candidate), candidate_pointers);
        }
    }
}
