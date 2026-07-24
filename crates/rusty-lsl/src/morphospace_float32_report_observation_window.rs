// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded longitudinal retention of exact P36 Morphospace observations.
//!
//! The window only retains caller-supplied observations and sums their supplied
//! terminal counters. It does not infer loss, continuity, policy, or authority.

use crate::morphospace_float32_report_observation::MorphospaceFloat32ReportObservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportObservationWindowConfigError {
    ZeroMaximumReports,
    ZeroMaximumRecords,
    MaximumReportsUnrepresentable { requested: usize },
    MaximumRecordsUnrepresentable { requested: usize },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportObservationWindowTotals {
    report_count: u64,
    record_count: u64,
    observation_count: u64,
    first_count: u64,
    contiguous_count: u64,
    gap_count: u64,
    explicit_missing_sequence_count: u64,
    duplicate_count: u64,
    out_of_order_count: u64,
    retained_unchanged_count: u64,
    retained_changed_count: u64,
}

impl MorphospaceFloat32ReportObservationWindowTotals {
    pub(crate) const fn report_count(&self) -> u64 {
        self.report_count
    }
    pub(crate) const fn record_count(&self) -> u64 {
        self.record_count
    }
    pub(crate) const fn observation_count(&self) -> u64 {
        self.observation_count
    }
    pub(crate) const fn first_count(&self) -> u64 {
        self.first_count
    }
    pub(crate) const fn contiguous_count(&self) -> u64 {
        self.contiguous_count
    }
    pub(crate) const fn gap_count(&self) -> u64 {
        self.gap_count
    }
    pub(crate) const fn explicit_missing_sequence_count(&self) -> u64 {
        self.explicit_missing_sequence_count
    }
    pub(crate) const fn duplicate_count(&self) -> u64 {
        self.duplicate_count
    }
    pub(crate) const fn out_of_order_count(&self) -> u64 {
        self.out_of_order_count
    }
    pub(crate) const fn retained_unchanged_count(&self) -> u64 {
        self.retained_unchanged_count
    }
    pub(crate) const fn retained_changed_count(&self) -> u64 {
        self.retained_changed_count
    }

    fn checked_with(self, observation: &MorphospaceFloat32ReportObservation) -> Option<Self> {
        let health = observation.terminal_health();
        Some(Self {
            report_count: self.report_count.checked_add(1)?,
            record_count: self
                .record_count
                .checked_add(u64::try_from(observation.records().len()).ok()?)?,
            observation_count: self
                .observation_count
                .checked_add(health.observation_count())?,
            first_count: self.first_count.checked_add(health.first_count())?,
            contiguous_count: self
                .contiguous_count
                .checked_add(health.contiguous_count())?,
            gap_count: self.gap_count.checked_add(health.gap_count())?,
            explicit_missing_sequence_count: self
                .explicit_missing_sequence_count
                .checked_add(health.explicit_missing_sequence_count())?,
            duplicate_count: self.duplicate_count.checked_add(health.duplicate_count())?,
            out_of_order_count: self
                .out_of_order_count
                .checked_add(health.out_of_order_count())?,
            retained_unchanged_count: self
                .retained_unchanged_count
                .checked_add(health.retained_unchanged_count())?,
            retained_changed_count: self
                .retained_changed_count
                .checked_add(health.retained_changed_count())?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct MorphospaceFloat32ReportObservationWindow {
    maximum_reports: usize,
    maximum_records: usize,
    observations: Vec<MorphospaceFloat32ReportObservation>,
    retained_records: usize,
    totals: MorphospaceFloat32ReportObservationWindowTotals,
}

#[derive(Debug, PartialEq)]
pub(crate) enum MorphospaceFloat32ReportObservationWindowAppendError {
    ReportLimit {
        limit: usize,
        window: MorphospaceFloat32ReportObservationWindow,
        observation: MorphospaceFloat32ReportObservation,
    },
    RecordLimit {
        limit: usize,
        retained: usize,
        appended: usize,
        window: MorphospaceFloat32ReportObservationWindow,
        observation: MorphospaceFloat32ReportObservation,
    },
    CounterOverflow {
        window: MorphospaceFloat32ReportObservationWindow,
        observation: MorphospaceFloat32ReportObservation,
    },
    Allocation {
        requested_reports: usize,
        window: MorphospaceFloat32ReportObservationWindow,
        observation: MorphospaceFloat32ReportObservation,
    },
}

impl MorphospaceFloat32ReportObservationWindowAppendError {
    pub(crate) fn into_parts(
        self,
    ) -> (
        MorphospaceFloat32ReportObservationWindow,
        MorphospaceFloat32ReportObservation,
    ) {
        match self {
            Self::ReportLimit {
                window,
                observation,
                ..
            }
            | Self::RecordLimit {
                window,
                observation,
                ..
            }
            | Self::CounterOverflow {
                window,
                observation,
            }
            | Self::Allocation {
                window,
                observation,
                ..
            } => (window, observation),
        }
    }
}

impl MorphospaceFloat32ReportObservationWindow {
    pub(crate) fn new(
        maximum_reports: usize,
        maximum_records: usize,
    ) -> Result<Self, MorphospaceFloat32ReportObservationWindowConfigError> {
        if maximum_reports == 0 {
            return Err(MorphospaceFloat32ReportObservationWindowConfigError::ZeroMaximumReports);
        }
        if maximum_records == 0 {
            return Err(MorphospaceFloat32ReportObservationWindowConfigError::ZeroMaximumRecords);
        }
        u64::try_from(maximum_reports).map_err(|_| {
            MorphospaceFloat32ReportObservationWindowConfigError::MaximumReportsUnrepresentable {
                requested: maximum_reports,
            }
        })?;
        u64::try_from(maximum_records).map_err(|_| {
            MorphospaceFloat32ReportObservationWindowConfigError::MaximumRecordsUnrepresentable {
                requested: maximum_records,
            }
        })?;
        Ok(Self {
            maximum_reports,
            maximum_records,
            observations: Vec::new(),
            retained_records: 0,
            totals: MorphospaceFloat32ReportObservationWindowTotals::default(),
        })
    }

    pub(crate) const fn maximum_reports(&self) -> usize {
        self.maximum_reports
    }
    pub(crate) const fn maximum_records(&self) -> usize {
        self.maximum_records
    }
    pub(crate) fn observations(&self) -> &[MorphospaceFloat32ReportObservation] {
        &self.observations
    }
    pub(crate) const fn retained_records(&self) -> usize {
        self.retained_records
    }
    pub(crate) const fn totals(&self) -> MorphospaceFloat32ReportObservationWindowTotals {
        self.totals
    }

    pub(crate) fn append(
        self,
        observation: MorphospaceFloat32ReportObservation,
    ) -> Result<Self, MorphospaceFloat32ReportObservationWindowAppendError> {
        self.append_with(observation, |observations, requested| {
            observations.try_reserve_exact(requested).map_err(|_| ())
        })
    }

    fn append_with<R>(
        mut self,
        observation: MorphospaceFloat32ReportObservation,
        reserve: R,
    ) -> Result<Self, MorphospaceFloat32ReportObservationWindowAppendError>
    where
        R: FnOnce(&mut Vec<MorphospaceFloat32ReportObservation>, usize) -> Result<(), ()>,
    {
        if self.observations.len() >= self.maximum_reports {
            return Err(
                MorphospaceFloat32ReportObservationWindowAppendError::ReportLimit {
                    limit: self.maximum_reports,
                    window: self,
                    observation,
                },
            );
        }
        let appended = observation.records().len();
        let Some(retained_records) = self.retained_records.checked_add(appended) else {
            return Err(
                MorphospaceFloat32ReportObservationWindowAppendError::CounterOverflow {
                    window: self,
                    observation,
                },
            );
        };
        if retained_records > self.maximum_records {
            return Err(
                MorphospaceFloat32ReportObservationWindowAppendError::RecordLimit {
                    limit: self.maximum_records,
                    retained: self.retained_records,
                    appended,
                    window: self,
                    observation,
                },
            );
        }
        let Some(totals) = self.totals.checked_with(&observation) else {
            return Err(
                MorphospaceFloat32ReportObservationWindowAppendError::CounterOverflow {
                    window: self,
                    observation,
                },
            );
        };
        if reserve(&mut self.observations, 1).is_err() {
            return Err(
                MorphospaceFloat32ReportObservationWindowAppendError::Allocation {
                    requested_reports: 1,
                    window: self,
                    observation,
                },
            );
        }
        self.observations.push(observation);
        self.retained_records = retained_records;
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

    fn observation(sequences: Vec<u64>, values: Vec<f32>) -> MorphospaceFloat32ReportObservation {
        let timestamps = (0..values.len()).map(|index| index as f64 + 1.0).collect();
        observation_with_timestamps(sequences, values, timestamps)
    }

    fn observation_with_timestamps(
        sequences: Vec<u64>,
        values: Vec<f32>,
        timestamps: Vec<f64>,
    ) -> MorphospaceFloat32ReportObservation {
        let records = values
            .into_iter()
            .zip(timestamps)
            .map(|(value, timestamp)| {
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
                    RawSourceTimestamp::new(timestamp).unwrap(),
                    None,
                )
            })
            .collect::<Vec<_>>();
        MorphospaceFloat32ReportObservationOwner::new(records.len())
            .unwrap()
            .observe(outcome(sequences, records))
            .unwrap()
    }

    fn pointers(observation: &MorphospaceFloat32ReportObservation) -> Vec<*const f32> {
        observation
            .records()
            .iter()
            .map(|record| record.processed().sample().sample().values().as_ptr())
            .collect()
    }

    #[test]
    fn zero_bounds_are_rejected_and_usize_extremes_are_inert() {
        assert_eq!(
            MorphospaceFloat32ReportObservationWindow::new(0, 1),
            Err(MorphospaceFloat32ReportObservationWindowConfigError::ZeroMaximumReports)
        );
        assert_eq!(
            MorphospaceFloat32ReportObservationWindow::new(1, 0),
            Err(MorphospaceFloat32ReportObservationWindowConfigError::ZeroMaximumRecords)
        );
        let window =
            MorphospaceFloat32ReportObservationWindow::new(usize::MAX, usize::MAX).unwrap();
        assert_eq!(window.maximum_reports(), usize::MAX);
        assert_eq!(window.maximum_records(), usize::MAX);
        assert!(window.observations().is_empty());
        assert_eq!(window.totals(), Default::default());
    }

    #[test]
    fn bound_and_repeated_reports_preserve_order_allocations_and_exact_supplied_totals() {
        let first = observation(vec![0, u64::MAX], vec![1.0, 2.0]);
        let second = observation(vec![7, 7], vec![3.0, 4.0]);
        let first_pointers = pointers(&first);
        let second_pointers = pointers(&second);
        let window = MorphospaceFloat32ReportObservationWindow::new(2, 4)
            .unwrap()
            .append(first)
            .unwrap()
            .append(second)
            .unwrap();
        assert_eq!(pointers(&window.observations()[0]), first_pointers);
        assert_eq!(pointers(&window.observations()[1]), second_pointers);
        assert_eq!(window.retained_records(), 4);
        let totals = window.totals();
        assert_eq!(totals.report_count(), 2);
        assert_eq!(totals.record_count(), 4);
        assert_eq!(totals.observation_count(), 4);
        assert_eq!(totals.first_count(), 2);
        assert_eq!(totals.gap_count(), 1);
        assert_eq!(totals.explicit_missing_sequence_count(), u64::MAX - 1);
        assert_eq!(totals.duplicate_count(), 1);
        assert_eq!(totals.contiguous_count(), 0);
        assert_eq!(totals.out_of_order_count(), 0);
        assert_eq!(
            totals.retained_unchanged_count() + totals.retained_changed_count(),
            4
        );

        let extra = observation(vec![9], vec![5.0]);
        let extra_pointers = pointers(&extra);
        let (returned, extra) = window.append(extra).unwrap_err().into_parts();
        assert_eq!(returned.observations().len(), 2);
        assert_eq!(pointers(&extra), extra_pointers);
    }

    #[test]
    fn record_bound_refusal_returns_both_owners_unchanged() {
        let kept = observation(vec![1], vec![1.0]);
        let incoming = observation(vec![2, 3], vec![2.0, 3.0]);
        let kept_pointers = pointers(&kept);
        let incoming_pointers = pointers(&incoming);
        let before_totals;
        let window = MorphospaceFloat32ReportObservationWindow::new(3, 2)
            .unwrap()
            .append(kept)
            .unwrap();
        before_totals = window.totals();
        let (window, incoming) = window.append(incoming).unwrap_err().into_parts();
        assert_eq!(window.totals(), before_totals);
        assert_eq!(pointers(&window.observations()[0]), kept_pointers);
        assert_eq!(pointers(&incoming), incoming_pointers);
    }

    #[test]
    fn allocation_rollback_then_success_preserves_identity() {
        let incoming = observation(vec![u64::MAX], vec![f32::from_bits(0x7f7f_ffff)]);
        let incoming_pointers = pointers(&incoming);
        let window = MorphospaceFloat32ReportObservationWindow::new(1, 1).unwrap();
        let (window, incoming) = window
            .append_with(incoming, |_, requested| {
                assert_eq!(requested, 1);
                Err(())
            })
            .unwrap_err()
            .into_parts();
        assert!(window.observations().is_empty());
        assert_eq!(window.totals(), Default::default());
        assert_eq!(pointers(&incoming), incoming_pointers);
        let window = window.append(incoming).unwrap();
        assert_eq!(pointers(&window.observations()[0]), incoming_pointers);
    }

    #[test]
    fn every_aggregate_counter_overflow_is_atomic() {
        let cases: [(
            fn(&mut MorphospaceFloat32ReportObservationWindowTotals),
            MorphospaceFloat32ReportObservation,
        ); 11] = [
            (
                |t| t.report_count = u64::MAX,
                observation(vec![0], vec![1.0]),
            ),
            (
                |t| t.record_count = u64::MAX,
                observation(vec![0], vec![1.0]),
            ),
            (
                |t| t.observation_count = u64::MAX,
                observation(vec![0], vec![1.0]),
            ),
            (
                |t| t.first_count = u64::MAX,
                observation(vec![0], vec![1.0]),
            ),
            (
                |t| t.contiguous_count = u64::MAX,
                observation(vec![0, 1], vec![1.0, 2.0]),
            ),
            (
                |t| t.gap_count = u64::MAX,
                observation(vec![0, 2], vec![1.0, 2.0]),
            ),
            (
                |t| t.explicit_missing_sequence_count = u64::MAX,
                observation(vec![0, 2], vec![1.0, 2.0]),
            ),
            (
                |t| t.duplicate_count = u64::MAX,
                observation(vec![0, 0], vec![1.0, 2.0]),
            ),
            (
                |t| t.out_of_order_count = u64::MAX,
                observation(vec![1, 0], vec![1.0, 2.0]),
            ),
            (
                |t| t.retained_unchanged_count = u64::MAX,
                observation(vec![0], vec![1.0]),
            ),
            (
                |t| t.retained_changed_count = u64::MAX,
                observation_with_timestamps(vec![0, 1], vec![1.0, 2.0], vec![2.0, 1.0]),
            ),
        ];
        for (set_extreme, incoming) in cases {
            let incoming_pointers = pointers(&incoming);
            let mut window = MorphospaceFloat32ReportObservationWindow::new(2, 8).unwrap();
            set_extreme(&mut window.totals);
            let before = window.totals;
            let (window, incoming) = window.append(incoming).unwrap_err().into_parts();
            assert_eq!(window.totals, before);
            assert!(window.observations.is_empty());
            assert_eq!(pointers(&incoming), incoming_pointers);
        }

        let incoming = observation(vec![0], vec![9.0]);
        let incoming_pointers = pointers(&incoming);
        let mut window = MorphospaceFloat32ReportObservationWindow::new(2, usize::MAX).unwrap();
        window.retained_records = usize::MAX;
        let before = window.totals;
        let (window, incoming) = window.append(incoming).unwrap_err().into_parts();
        assert_eq!(window.retained_records, usize::MAX);
        assert_eq!(window.totals, before);
        assert_eq!(pointers(&incoming), incoming_pointers);
    }
}
