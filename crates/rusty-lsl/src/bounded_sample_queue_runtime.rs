// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-owned bounded FIFO behavior for accepted Float32 samples.

use crate::TimestampedSample;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

/// Selected feature identity for the bounded queue effect.
pub const BOUNDED_SAMPLE_QUEUE_FEATURE_ID: &str = "bounded-sample-queue";
/// Exact effective marker required beside explicit construction.
pub const BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER: &str = "rusty.lsl.bounded_sample_queue.effective";

/// Closed activation for queue construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedSampleQueueActivation;

impl BoundedSampleQueueActivation {
    /// Admits only the selected feature and exact runtime marker.
    pub fn new(feature: &str, marker: &str) -> Result<Self, BoundedSampleQueueActivationError> {
        if feature != BOUNDED_SAMPLE_QUEUE_FEATURE_ID {
            return Err(BoundedSampleQueueActivationError::FeatureMismatch);
        }
        if marker != BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER {
            return Err(BoundedSampleQueueActivationError::MarkerMismatch);
        }
        Ok(Self)
    }
}

/// Rejected queue activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedSampleQueueActivationError {
    /// Feature identity differed.
    FeatureMismatch,
    /// Effective marker differed.
    MarkerMismatch,
}

/// Explicit bounds for a blocking queue operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedSampleQueueWait {
    wait_slice: Duration,
    total_deadline: Duration,
}

impl BoundedSampleQueueWait {
    /// Requires nonzero polling and total wait bounds.
    pub fn new(
        wait_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, BoundedSampleQueueWaitError> {
        if wait_slice.is_zero() {
            return Err(BoundedSampleQueueWaitError::ZeroWaitSlice);
        }
        if total_deadline.is_zero() {
            return Err(BoundedSampleQueueWaitError::ZeroTotalDeadline);
        }
        Ok(Self {
            wait_slice,
            total_deadline,
        })
    }
}

/// Invalid blocking bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedSampleQueueWaitError {
    /// The polling slice was zero.
    ZeroWaitSlice,
    /// The total deadline was zero.
    ZeroTotalDeadline,
}

struct QueueState {
    samples: VecDeque<TimestampedSample<f32>>,
    closed: bool,
}

/// One caller-owned bounded FIFO with no owned worker.
pub struct BoundedSampleQueue {
    capacity: usize,
    state: Mutex<QueueState>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl BoundedSampleQueue {
    /// Allocates capacity only after validating a nonzero bound.
    pub fn new(
        _activation: BoundedSampleQueueActivation,
        capacity: usize,
    ) -> Result<Self, BoundedSampleQueueCreateError> {
        if capacity == 0 {
            return Err(BoundedSampleQueueCreateError::ZeroCapacity);
        }
        let mut samples = VecDeque::new();
        samples
            .try_reserve(capacity)
            .map_err(|_| BoundedSampleQueueCreateError::Allocation {
                requested: capacity,
            })?;
        Ok(Self {
            capacity,
            state: Mutex::new(QueueState {
                samples,
                closed: false,
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        })
    }

    /// Returns the fixed queue capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Attempts one immediate push, returning unchanged ownership on failure.
    pub fn try_push(
        &self,
        sample: TimestampedSample<f32>,
    ) -> Result<(), BoundedSampleQueuePushError> {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => return Err(BoundedSampleQueuePushError::Poisoned(sample)),
        };
        if state.closed {
            return Err(BoundedSampleQueuePushError::Closed(sample));
        }
        if state.samples.len() == self.capacity {
            return Err(BoundedSampleQueuePushError::Full(sample));
        }
        state.samples.push_back(sample);
        self.not_empty.notify_one();
        Ok(())
    }

    /// Waits within explicit bounds for capacity, cancellation, or closure.
    pub fn push(
        &self,
        sample: TimestampedSample<f32>,
        wait: BoundedSampleQueueWait,
        cancelled: &AtomicBool,
    ) -> Result<(), BoundedSampleQueuePushError> {
        let started = Instant::now();
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => return Err(BoundedSampleQueuePushError::Poisoned(sample)),
        };
        loop {
            if cancelled.load(Ordering::Acquire) {
                return Err(BoundedSampleQueuePushError::Cancelled(sample));
            }
            if state.closed {
                return Err(BoundedSampleQueuePushError::Closed(sample));
            }
            if state.samples.len() < self.capacity {
                state.samples.push_back(sample);
                self.not_empty.notify_one();
                return Ok(());
            }
            let remaining = match wait.total_deadline.checked_sub(started.elapsed()) {
                Some(remaining) => remaining,
                None => return Err(BoundedSampleQueuePushError::Deadline(sample)),
            };
            match self
                .not_full
                .wait_timeout(state, remaining.min(wait.wait_slice))
            {
                Ok((next, _)) => state = next,
                Err(_) => return Err(BoundedSampleQueuePushError::Poisoned(sample)),
            }
        }
    }

    /// Attempts one immediate FIFO pop.
    pub fn try_pop(&self) -> Result<TimestampedSample<f32>, BoundedSampleQueuePopError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| BoundedSampleQueuePopError::Poisoned)?;
        if let Some(sample) = state.samples.pop_front() {
            self.not_full.notify_one();
            return Ok(sample);
        }
        if state.closed {
            Err(BoundedSampleQueuePopError::Closed)
        } else {
            Err(BoundedSampleQueuePopError::Empty)
        }
    }

    /// Waits within explicit bounds for a sample, cancellation, or closure.
    pub fn pop(
        &self,
        wait: BoundedSampleQueueWait,
        cancelled: &AtomicBool,
    ) -> Result<TimestampedSample<f32>, BoundedSampleQueuePopError> {
        let started = Instant::now();
        let mut state = self
            .state
            .lock()
            .map_err(|_| BoundedSampleQueuePopError::Poisoned)?;
        loop {
            if cancelled.load(Ordering::Acquire) {
                return Err(BoundedSampleQueuePopError::Cancelled);
            }
            if let Some(sample) = state.samples.pop_front() {
                self.not_full.notify_one();
                return Ok(sample);
            }
            if state.closed {
                return Err(BoundedSampleQueuePopError::Closed);
            }
            let remaining = wait
                .total_deadline
                .checked_sub(started.elapsed())
                .ok_or(BoundedSampleQueuePopError::Deadline)?;
            match self
                .not_empty
                .wait_timeout(state, remaining.min(wait.wait_slice))
            {
                Ok((next, _)) => state = next,
                Err(_) => return Err(BoundedSampleQueuePopError::Poisoned),
            }
        }
    }

    /// Closes the queue and wakes all blocked callers; buffered samples remain drainable.
    pub fn close(&self) -> Result<(), BoundedSampleQueueCloseError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| BoundedSampleQueueCloseError::Poisoned)?;
        state.closed = true;
        self.not_empty.notify_all();
        self.not_full.notify_all();
        Ok(())
    }
}

/// Queue construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedSampleQueueCreateError {
    /// Capacity was zero.
    ZeroCapacity,
    /// Capacity reservation failed.
    Allocation {
        /// Requested element capacity.
        requested: usize,
    },
}

/// Push outcome retaining the unchanged rejected sample.
#[derive(Debug)]
pub enum BoundedSampleQueuePushError {
    /// Immediate backpressure because capacity was full.
    Full(TimestampedSample<f32>),
    /// Queue was closed.
    Closed(TimestampedSample<f32>),
    /// Caller cancellation was observed.
    Cancelled(TimestampedSample<f32>),
    /// Total blocking deadline elapsed.
    Deadline(TimestampedSample<f32>),
    /// A prior panic poisoned the caller-owned queue state.
    Poisoned(TimestampedSample<f32>),
}

/// Pop outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedSampleQueuePopError {
    /// Immediate backpressure because no sample was present.
    Empty,
    /// Queue was closed and drained.
    Closed,
    /// Caller cancellation was observed.
    Cancelled,
    /// Total blocking deadline elapsed.
    Deadline,
    /// A prior panic poisoned the caller-owned queue state.
    Poisoned,
}

/// Queue close failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedSampleQueueCloseError {
    /// A prior panic poisoned the caller-owned queue state.
    Poisoned,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        run_timestamped_float32_inlet, run_timestamped_float32_outlet, RawSourceTimestamp, Sample,
        SampleLimits, StreamHandshakeActivation, StreamHandshakeIdentity, StreamHandshakeLimits,
        TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
        STREAM_HANDSHAKE_EFFECTIVE_MARKER, STREAM_HANDSHAKE_FEATURE_ID,
        TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER, TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID,
    };
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::thread;

    fn activation() -> BoundedSampleQueueActivation {
        BoundedSampleQueueActivation::new(
            BOUNDED_SAMPLE_QUEUE_FEATURE_ID,
            BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER,
        )
        .unwrap()
    }

    fn sample(timestamp: f64, value: f32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
            RawSourceTimestamp::new(timestamp).unwrap(),
            None,
        )
    }

    fn wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(100)).unwrap()
    }

    #[test]
    fn lslc_002v_fifo_full_and_empty_preserve_exact_bits() {
        let queue = BoundedSampleQueue::new(activation(), 1).unwrap();
        queue
            .try_push(sample(-0.0, f32::from_bits(0x7fc0_1234)))
            .unwrap();
        let rejected = match queue.try_push(sample(2.0, -3.0)) {
            Err(BoundedSampleQueuePushError::Full(sample)) => sample,
            _ => panic!("expected full backpressure"),
        };
        assert_eq!(
            rejected.raw_source_timestamp().value().to_bits(),
            2.0f64.to_bits()
        );
        let accepted = queue.try_pop().unwrap();
        assert_eq!(
            accepted.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(accepted.sample().values()[0].to_bits(), 0x7fc0_1234);
        assert_eq!(
            queue.try_pop().unwrap_err(),
            BoundedSampleQueuePopError::Empty
        );
    }

    #[test]
    fn lslc_002v_blocked_operations_cancel_or_close() {
        let queue = Arc::new(BoundedSampleQueue::new(activation(), 1).unwrap());
        queue.try_push(sample(1.0, 1.0)).unwrap();
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_queue = Arc::clone(&queue);
        let worker_cancelled = Arc::clone(&cancelled);
        let worker =
            thread::spawn(move || worker_queue.push(sample(2.0, 2.0), wait(), &worker_cancelled));
        thread::sleep(Duration::from_millis(10));
        cancelled.store(true, Ordering::Release);
        assert!(matches!(
            worker.join().unwrap(),
            Err(BoundedSampleQueuePushError::Cancelled(_))
        ));

        queue.try_pop().unwrap();
        let worker_queue = Arc::clone(&queue);
        let worker = thread::spawn(move || worker_queue.pop(wait(), &AtomicBool::new(false)));
        thread::sleep(Duration::from_millis(10));
        queue.close().unwrap();
        assert_eq!(
            worker.join().unwrap().unwrap_err(),
            BoundedSampleQueuePopError::Closed
        );
    }

    #[test]
    fn lslc_002v_limits_activation_deadline_and_drain_are_typed() {
        assert_eq!(
            BoundedSampleQueue::new(activation(), 0).err(),
            Some(BoundedSampleQueueCreateError::ZeroCapacity)
        );
        assert_eq!(
            BoundedSampleQueueWait::new(Duration::ZERO, Duration::from_millis(1)),
            Err(BoundedSampleQueueWaitError::ZeroWaitSlice)
        );
        let queue = BoundedSampleQueue::new(activation(), 1).unwrap();
        assert_eq!(
            queue
                .pop(
                    BoundedSampleQueueWait::new(Duration::from_millis(1), Duration::from_millis(2))
                        .unwrap(),
                    &AtomicBool::new(false)
                )
                .unwrap_err(),
            BoundedSampleQueuePopError::Deadline
        );
        queue.try_push(sample(1.0, 1.0)).unwrap();
        queue.close().unwrap();
        assert_eq!(
            queue.try_pop().unwrap().sample().values()[0].to_bits(),
            1.0f32.to_bits()
        );
        assert_eq!(
            queue.try_pop().unwrap_err(),
            BoundedSampleQueuePopError::Closed
        );
    }

    #[test]
    fn lslc_002v_received_loopback_sample_composes_with_queue() {
        let handshake = StreamHandshakeActivation::new(
            STREAM_HANDSHAKE_FEATURE_ID,
            STREAM_HANDSHAKE_EFFECTIVE_MARKER,
        )
        .unwrap();
        let sample_activation = TimestampedFloat32SampleActivation::new(
            TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID,
            TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER,
            handshake,
        )
        .unwrap();
        let handshake_limits =
            StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
                .unwrap();
        let sample_limits =
            TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap();
        let identity = StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "synthetic-host".into(),
            "synthetic-source".into(),
            "synthetic-session".into(),
            handshake_limits,
        )
        .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker_identity = identity.clone();
        let worker = thread::spawn(move || {
            let sent = sample(-0.0, f32::from_bits(0x7fc0_4321));
            run_timestamped_float32_outlet(
                sample_activation,
                listener,
                &worker_identity,
                handshake_limits,
                sample_limits,
                &sent,
                &AtomicBool::new(false),
            )
        });
        let received = run_timestamped_float32_inlet(
            sample_activation,
            address,
            &identity,
            handshake_limits,
            sample_limits,
            &AtomicBool::new(false),
        )
        .unwrap();
        let queue = BoundedSampleQueue::new(activation(), 1).unwrap();
        queue.try_push(received).unwrap();
        let drained = queue.try_pop().unwrap();
        assert_eq!(
            drained.raw_source_timestamp().value().to_bits(),
            (-0.0f64).to_bits()
        );
        assert_eq!(drained.sample().values()[0].to_bits(), 0x7fc0_4321);
        assert_eq!(worker.join().unwrap().unwrap(), address);
        TcpListener::bind(address).unwrap();
    }
}
