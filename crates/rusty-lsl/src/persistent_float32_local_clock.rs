// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process-local monotonic source clock shared by persistent Float32 outlets.

use crate::ClockSource;
use std::sync::OnceLock;
use std::time::Instant;

static PROCESS_CLOCK_ORIGIN: OnceLock<Instant> = OnceLock::new();

/// Reads the process-wide monotonic source clock in finite seconds.
///
/// Every Rusty LSL persistent outlet in one process observes the same clock
/// domain. The value has no wall-clock or cross-process meaning; official
/// inlet timedata exchanges map this source domain into each consumer domain.
#[must_use]
pub fn persistent_float32_local_clock() -> f64 {
    PROCESS_CLOCK_ORIGIN
        .get_or_init(Instant::now)
        .elapsed()
        .as_secs_f64()
}

/// Zero-sized caller-owned adapter for APIs that accept [`ClockSource`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PersistentFloat32LocalClock;

impl PersistentFloat32LocalClock {
    /// Reads the shared process-local source clock.
    #[must_use]
    pub fn now(self) -> f64 {
        persistent_float32_local_clock()
    }
}

impl ClockSource for PersistentFloat32LocalClock {
    fn now(&mut self) -> f64 {
        persistent_float32_local_clock()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polar_001_local_clock_is_process_shared_finite_and_monotonic() {
        let first = persistent_float32_local_clock();
        let second = PersistentFloat32LocalClock.now();
        let mut provider = PersistentFloat32LocalClock;
        let third = ClockSource::now(&mut provider);
        assert!(first.is_finite() && first >= 0.0);
        assert!(second >= first);
        assert!(third >= second);
    }
}
