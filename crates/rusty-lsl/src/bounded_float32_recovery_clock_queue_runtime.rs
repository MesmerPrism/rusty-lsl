// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded Float32 recovery, clock-correction, and queue lifecycle composition.

use crate::{
    run_finite_sample_recovery, run_integrated_clock_correction, BoundedSampleQueue,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, ClockSource,
    FiniteSampleRecoveryActivation, FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryState, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, IntegratedClockCorrectionError, RecoveryAttemptFailure,
    TimestampedSample,
};
use std::sync::atomic::AtomicBool;

/// Caller-owned cancellation inputs observed by each existing lifecycle owner.
#[derive(Clone, Copy, Debug)]
pub struct BoundedFloat32PipelineCancellation<'a> {
    recovery: &'a AtomicBool,
    clock: &'a AtomicBool,
    queue: &'a AtomicBool,
}

impl<'a> BoundedFloat32PipelineCancellation<'a> {
    /// Groups existing cancellation signals without merging their ownership or meaning.
    #[must_use]
    pub const fn new(
        recovery: &'a AtomicBool,
        clock: &'a AtomicBool,
        queue: &'a AtomicBool,
    ) -> Self {
        Self {
            recovery,
            clock,
            queue,
        }
    }
}

/// Terminal result of one bounded production pipeline run.
#[derive(Debug)]
pub enum BoundedFloat32PipelineOutcome {
    /// One recovered and corrected sample entered the caller-owned queue.
    Queued {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The caller classified an acquisition failure as terminal.
    Terminal {
        /// Unchanged caller failure.
        failure: RecoveryAttemptFailure,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Every permitted acquisition attempt failed retryably.
    Exhausted {
        /// Last unchanged caller failure.
        failure: RecoveryAttemptFailure,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Recovery cancellation was observed before downstream work.
    Cancelled {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// The finite recovery deadline elapsed before downstream work.
    Deadline {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Owner-preserving setup or downstream failure.
#[derive(Debug)]
pub enum BoundedFloat32PipelineError {
    /// The recovery owner could not reserve its bounded state trace.
    Recovery(FiniteSampleRecoveryError),
    /// Clock correction failed and retains the unchanged recovered sample.
    Clock {
        /// Existing clock-correction failure.
        error: IntegratedClockCorrectionError,
        /// Unchanged recovered sample.
        sample: TimestampedSample<f32>,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Queue admission failed and its existing error retains the corrected sample.
    Queue {
        /// Existing queue failure.
        error: BoundedSampleQueuePushError,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Runs caller-supplied finite acquisition, then correction and queue admission once.
///
/// The caller owns acquisition, failure classification, every activation and policy,
/// the clock provider/domain, queue, and distinct cancellation signals. This function
/// performs no discovery, endpoint selection, policy inference, or background work.
#[allow(clippy::too_many_arguments)]
pub fn run_bounded_float32_recovery_clock_queue<C, A>(
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    mut acquire: A,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    cancellation: BoundedFloat32PipelineCancellation<'_>,
) -> Result<BoundedFloat32PipelineOutcome, BoundedFloat32PipelineError>
where
    C: ClockSource,
    A: FnMut(usize) -> Result<TimestampedSample<f32>, RecoveryAttemptFailure>,
{
    let recovery = run_finite_sample_recovery(
        recovery_activation,
        recovery_policy,
        cancellation.recovery,
        &mut acquire,
    )
    .map_err(BoundedFloat32PipelineError::Recovery)?;
    let (sample, states) = match recovery {
        FiniteSampleRecoveryOutcome::Recovered { sample, states } => (sample, states),
        FiniteSampleRecoveryOutcome::Terminal { failure, states } => {
            return Ok(BoundedFloat32PipelineOutcome::Terminal { failure, states });
        }
        FiniteSampleRecoveryOutcome::Exhausted { failure, states } => {
            return Ok(BoundedFloat32PipelineOutcome::Exhausted { failure, states });
        }
        FiniteSampleRecoveryOutcome::Cancelled { states } => {
            return Ok(BoundedFloat32PipelineOutcome::Cancelled { states });
        }
        FiniteSampleRecoveryOutcome::Deadline { states } => {
            return Ok(BoundedFloat32PipelineOutcome::Deadline { states });
        }
    };
    let correction = match run_integrated_clock_correction(
        clock_activation,
        clock_config,
        clock,
        sample.raw_source_timestamp(),
        cancellation.clock,
    ) {
        Ok(correction) => correction,
        Err(error) => {
            return Err(BoundedFloat32PipelineError::Clock {
                error,
                sample,
                states,
            });
        }
    };
    let (values, raw, _) = sample.into_parts();
    let corrected = TimestampedSample::new(values, raw, Some(correction.application().derived()));
    if let Err(error) = queue.push(corrected, queue_wait, cancellation.queue) {
        return Err(BoundedFloat32PipelineError::Queue { error, states });
    }
    Ok(BoundedFloat32PipelineOutcome::Queued { states })
}
