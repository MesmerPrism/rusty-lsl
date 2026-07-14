// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{DerivedTimestamp, DerivedTimestampKind, RawSourceTimestamp};

/// One finite caller-supplied clock offset retained without normalization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClockOffset(f64);

impl ClockOffset {
    /// Admits one finite offset.
    pub fn new(value: f64) -> Result<Self, ClockOffsetError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(ClockOffsetError {
                bits: value.to_bits(),
            })
        }
    }
    /// Returns the unchanged value.
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// A rejected non-finite offset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockOffsetError {
    bits: u64,
}
impl ClockOffsetError {
    /// Returns the unchanged rejected bits.
    pub const fn bits(self) -> u64 {
        self.bits
    }
}

/// A rejected non-finite timestamp-plus-offset sum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockOffsetApplicationError {
    bits: u64,
}
impl ClockOffsetApplicationError {
    /// Returns the unchanged rejected result bits.
    pub const fn bits(self) -> u64 {
        self.bits
    }
}

/// The raw timestamp and offset beside their explicit clock-corrected result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClockOffsetApplication {
    raw: RawSourceTimestamp,
    offset: ClockOffset,
    derived: DerivedTimestamp,
}

impl ClockOffsetApplication {
    /// Explicitly adds one accepted offset to one accepted raw timestamp.
    pub fn apply(
        raw: RawSourceTimestamp,
        offset: ClockOffset,
    ) -> Result<Self, ClockOffsetApplicationError> {
        let value = raw.value() + offset.value();
        if !value.is_finite() {
            return Err(ClockOffsetApplicationError {
                bits: value.to_bits(),
            });
        }
        let derived = DerivedTimestamp::new(DerivedTimestampKind::ClockCorrected, value)
            .expect("a finite value is accepted by DerivedTimestamp");
        Ok(Self {
            raw,
            offset,
            derived,
        })
    }
    /// Returns the unchanged raw timestamp.
    pub const fn raw(self) -> RawSourceTimestamp {
        self.raw
    }
    /// Returns the unchanged offset.
    pub const fn offset(self) -> ClockOffset {
        self.offset
    }
    /// Returns the derived clock-corrected timestamp.
    pub const fn derived(self) -> DerivedTimestamp {
        self.derived
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lslc_002o_finite_application_preserves_inputs() {
        let raw = RawSourceTimestamp::new(-0.0).unwrap();
        let offset = ClockOffset::new(2.5).unwrap();
        let applied = ClockOffsetApplication::apply(raw, offset).unwrap();
        assert_eq!(applied.raw().value().to_bits(), (-0.0f64).to_bits());
        assert_eq!(applied.offset(), offset);
        assert_eq!(
            applied.derived().kind(),
            DerivedTimestampKind::ClockCorrected
        );
        assert_eq!(applied.derived().value(), 2.5);
    }
    #[test]
    fn lslc_002o_nonfinite_offset_and_sum_reject() {
        let error = ClockOffset::new(f64::NAN).unwrap_err();
        assert!(f64::from_bits(error.bits()).is_nan());
        let error = ClockOffsetApplication::apply(
            RawSourceTimestamp::new(f64::MAX).unwrap(),
            ClockOffset::new(f64::MAX).unwrap(),
        )
        .unwrap_err();
        assert_eq!(f64::from_bits(error.bits()), f64::INFINITY);
    }
}
