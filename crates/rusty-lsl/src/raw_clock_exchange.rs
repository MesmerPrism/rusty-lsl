// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

/// One role in the documented four-timestamp exchange.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RawClockExchangeTimestampRole {
    /// Local query-submission timestamp.
    T0,
    /// Remote receipt timestamp.
    T1,
    /// Remote return-submission timestamp.
    T2,
    /// Local return-receipt timestamp.
    T3,
}

/// A rejected non-finite raw timestamp.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawClockExchangeInputError {
    role: RawClockExchangeTimestampRole,
    bits: u64,
}

impl RawClockExchangeInputError {
    /// Returns the first rejected timestamp role.
    pub const fn role(self) -> RawClockExchangeTimestampRole {
        self.role
    }

    /// Returns the unchanged rejected `f64` bits.
    pub const fn bits(self) -> u64 {
        self.bits
    }
}

/// Four finite opaque timestamps in documented exchange-role order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawClockExchange {
    timestamps: [f64; 4],
}

impl RawClockExchange {
    /// Admits four finite values, rejecting in `t0` through `t3` order.
    pub fn new(t0: f64, t1: f64, t2: f64, t3: f64) -> Result<Self, RawClockExchangeInputError> {
        let timestamps = [t0, t1, t2, t3];
        let roles = [
            RawClockExchangeTimestampRole::T0,
            RawClockExchangeTimestampRole::T1,
            RawClockExchangeTimestampRole::T2,
            RawClockExchangeTimestampRole::T3,
        ];
        for (value, role) in timestamps.iter().zip(roles) {
            if !value.is_finite() {
                return Err(RawClockExchangeInputError {
                    role,
                    bits: value.to_bits(),
                });
            }
        }
        Ok(Self { timestamps })
    }

    /// Returns `t0` unchanged.
    pub const fn t0(self) -> f64 {
        self.timestamps[0]
    }
    /// Returns `t1` unchanged.
    pub const fn t1(self) -> f64 {
        self.timestamps[1]
    }
    /// Returns `t2` unchanged.
    pub const fn t2(self) -> f64 {
        self.timestamps[2]
    }
    /// Returns `t3` unchanged.
    pub const fn t3(self) -> f64 {
        self.timestamps[3]
    }

    /// Evaluates the documented RTT and OFS formulas without reading a clock.
    pub fn evaluate(self) -> Result<RawClockExchangeFormulaResult, RawClockExchangeFormulaError> {
        let elapsed = finite(self.t3() - self.t0(), RawClockExchangeFormulaStage::Elapsed)?;
        let remote_processing = finite(
            self.t2() - self.t1(),
            RawClockExchangeFormulaStage::RemoteProcessing,
        )?;
        let round_trip_time = finite(
            elapsed - remote_processing,
            RawClockExchangeFormulaStage::RoundTripTime,
        )?;
        let forward = finite(
            self.t1() - self.t0(),
            RawClockExchangeFormulaStage::ForwardOffsetTerm,
        )?;
        let return_term = finite(
            self.t2() - self.t3(),
            RawClockExchangeFormulaStage::ReturnOffsetTerm,
        )?;
        let offset_sum = finite(
            forward + return_term,
            RawClockExchangeFormulaStage::OffsetSum,
        )?;
        let clock_offset = finite(offset_sum / 2.0, RawClockExchangeFormulaStage::ClockOffset)?;
        Ok(RawClockExchangeFormulaResult {
            exchange: self,
            round_trip_time,
            clock_offset,
        })
    }
}

/// The first arithmetic stage that produced a non-finite value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RawClockExchangeFormulaStage {
    /// `t3 - t0`.
    Elapsed,
    /// `t2 - t1`.
    RemoteProcessing,
    /// `(t3 - t0) - (t2 - t1)`.
    RoundTripTime,
    /// `t1 - t0`.
    ForwardOffsetTerm,
    /// `t2 - t3`.
    ReturnOffsetTerm,
    /// `(t1 - t0) + (t2 - t3)`.
    OffsetSum,
    /// The offset sum divided by two.
    ClockOffset,
}

/// A non-finite formula intermediate or output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawClockExchangeFormulaError {
    stage: RawClockExchangeFormulaStage,
    bits: u64,
}

impl RawClockExchangeFormulaError {
    /// Returns the first failing arithmetic stage.
    pub const fn stage(self) -> RawClockExchangeFormulaStage {
        self.stage
    }
    /// Returns the unchanged non-finite result bits.
    pub const fn bits(self) -> u64 {
        self.bits
    }
}

fn finite(
    value: f64,
    stage: RawClockExchangeFormulaStage,
) -> Result<f64, RawClockExchangeFormulaError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(RawClockExchangeFormulaError {
            stage,
            bits: value.to_bits(),
        })
    }
}

/// Finite results beside the unchanged admitted raw exchange.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawClockExchangeFormulaResult {
    exchange: RawClockExchange,
    round_trip_time: f64,
    clock_offset: f64,
}

impl RawClockExchangeFormulaResult {
    /// Returns the unchanged admitted exchange.
    pub const fn exchange(self) -> RawClockExchange {
        self.exchange
    }
    /// Returns the finite documented RTT result.
    pub const fn round_trip_time(self) -> f64 {
        self.round_trip_time
    }
    /// Returns the finite documented OFS result.
    pub const fn clock_offset(self) -> f64 {
        self.clock_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lslc_002m_valid_formula_and_bits_are_preserved() {
        let exchange = RawClockExchange::new(-0.0, 4.0, 5.0, 9.0).unwrap();
        assert_eq!(exchange.t0().to_bits(), (-0.0f64).to_bits());
        let result = exchange.evaluate().unwrap();
        assert_eq!(result.exchange(), exchange);
        assert_eq!(result.round_trip_time(), 8.0);
        assert_eq!(result.clock_offset(), 0.0);
        let result = RawClockExchange::new(1.0, 4.0, 5.0, 9.0)
            .unwrap()
            .evaluate()
            .unwrap();
        assert_eq!(result.round_trip_time(), 7.0);
        assert_eq!(result.clock_offset(), -0.5);
    }

    #[test]
    fn lslc_002m_nonfinite_inputs_reject_in_role_order() {
        for (values, role) in [
            (
                [f64::NAN, f64::INFINITY, f64::INFINITY, f64::INFINITY],
                RawClockExchangeTimestampRole::T0,
            ),
            (
                [0.0, f64::NEG_INFINITY, f64::NAN, f64::INFINITY],
                RawClockExchangeTimestampRole::T1,
            ),
            (
                [0.0, 0.0, f64::NAN, f64::INFINITY],
                RawClockExchangeTimestampRole::T2,
            ),
            (
                [0.0, 0.0, 0.0, f64::INFINITY],
                RawClockExchangeTimestampRole::T3,
            ),
        ] {
            assert_eq!(
                RawClockExchange::new(values[0], values[1], values[2], values[3])
                    .unwrap_err()
                    .role(),
                role
            );
        }
    }

    #[test]
    fn lslc_002m_reachable_nonfinite_stages_fail_first() {
        let max = f64::MAX;
        let half = max / 2.0;
        for (values, stage) in [
            ([-max, 0.0, 0.0, max], RawClockExchangeFormulaStage::Elapsed),
            (
                [0.0, -max, max, 0.0],
                RawClockExchangeFormulaStage::RemoteProcessing,
            ),
            (
                [0.0, 0.0, -max, max],
                RawClockExchangeFormulaStage::RoundTripTime,
            ),
            (
                [-max, max, max, -max],
                RawClockExchangeFormulaStage::ForwardOffsetTerm,
            ),
            (
                [-half, half, half, -half],
                RawClockExchangeFormulaStage::OffsetSum,
            ),
        ] {
            let error = RawClockExchange::new(values[0], values[1], values[2], values[3])
                .unwrap()
                .evaluate()
                .unwrap_err();
            assert_eq!(error.stage(), stage);
            assert!(!f64::from_bits(error.bits()).is_finite());
        }
    }
}
