// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::RawClockExchangeFormulaResult;

/// An explicit nonzero maximum for one already evaluated result batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockFilterSelectionLimit {
    max_results: usize,
}

impl ClockFilterSelectionLimit {
    /// Creates a nonzero result-count maximum.
    pub const fn new(max_results: usize) -> Result<Self, ClockFilterSelectionLimitError> {
        if max_results == 0 {
            Err(ClockFilterSelectionLimitError::ZeroMaximum)
        } else {
            Ok(Self { max_results })
        }
    }

    /// Returns the selected maximum.
    pub const fn max_results(self) -> usize {
        self.max_results
    }
}

/// An invalid selection limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockFilterSelectionLimitError {
    /// The maximum result count was zero.
    ZeroMaximum,
}

/// A rejected result batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockFilterSelectionError {
    /// The batch was empty.
    Empty,
    /// The batch exceeded the caller-selected maximum.
    LimitExceeded {
        /// Selected maximum.
        expected: usize,
        /// Supplied result count.
        actual: usize,
    },
}

/// A bounded owned batch with the first minimum-RTT result selected.
#[derive(Debug, PartialEq)]
pub struct ClockFilterSelection {
    limit: ClockFilterSelectionLimit,
    results: Vec<RawClockExchangeFormulaResult>,
    selected_index: usize,
}

impl ClockFilterSelection {
    /// Selects the first numerically minimum finite RTT in one nonempty batch.
    pub fn select(
        results: Vec<RawClockExchangeFormulaResult>,
        limit: ClockFilterSelectionLimit,
    ) -> Result<Self, ClockFilterSelectionError> {
        if results.is_empty() {
            return Err(ClockFilterSelectionError::Empty);
        }
        if results.len() > limit.max_results {
            return Err(ClockFilterSelectionError::LimitExceeded {
                expected: limit.max_results,
                actual: results.len(),
            });
        }
        let mut selected_index = 0;
        for index in 1..results.len() {
            if results[index].round_trip_time() < results[selected_index].round_trip_time() {
                selected_index = index;
            }
        }
        Ok(Self {
            limit,
            results,
            selected_index,
        })
    }

    /// Returns the unchanged caller-selected limit.
    pub const fn limit(&self) -> ClockFilterSelectionLimit {
        self.limit
    }
    /// Returns every original result in input order.
    pub fn results(&self) -> &[RawClockExchangeFormulaResult] {
        &self.results
    }
    /// Returns the selected input index.
    pub const fn selected_index(&self) -> usize {
        self.selected_index
    }
    /// Returns the selected result without recomputation.
    pub fn selected(&self) -> &RawClockExchangeFormulaResult {
        &self.results[self.selected_index]
    }
    /// Recovers the original result vector allocation.
    pub fn into_results(self) -> Vec<RawClockExchangeFormulaResult> {
        self.results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RawClockExchange;

    fn result(rtt: f64) -> RawClockExchangeFormulaResult {
        RawClockExchange::new(0.0, 0.0, 0.0, rtt)
            .unwrap()
            .evaluate()
            .unwrap()
    }

    #[test]
    fn lslc_002n_limits_reject_before_selection() {
        assert_eq!(
            ClockFilterSelectionLimit::new(0),
            Err(ClockFilterSelectionLimitError::ZeroMaximum)
        );
        let limit = ClockFilterSelectionLimit::new(1).unwrap();
        assert_eq!(
            ClockFilterSelection::select(Vec::new(), limit),
            Err(ClockFilterSelectionError::Empty)
        );
        assert_eq!(
            ClockFilterSelection::select(vec![result(1.0), result(2.0)], limit),
            Err(ClockFilterSelectionError::LimitExceeded {
                expected: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn lslc_002n_selects_minimum_and_preserves_vector_allocation() {
        let results = vec![result(3.0), result(-1.0), result(2.0)];
        let pointer = results.as_ptr();
        let selection =
            ClockFilterSelection::select(results, ClockFilterSelectionLimit::new(3).unwrap())
                .unwrap();
        assert_eq!(selection.limit().max_results(), 3);
        assert_eq!(selection.selected_index(), 1);
        assert_eq!(selection.selected().round_trip_time(), -1.0);
        assert_eq!(selection.results().as_ptr(), pointer);
        let recovered = selection.into_results();
        assert_eq!(recovered.as_ptr(), pointer);
    }

    #[test]
    fn lslc_002n_equal_minima_select_first_input() {
        let selection = ClockFilterSelection::select(
            vec![result(1.0), result(-2.0), result(-2.0)],
            ClockFilterSelectionLimit::new(8).unwrap(),
        )
        .unwrap();
        assert_eq!(selection.selected_index(), 1);
    }
}
