// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Completed Float32 inlet-session report to bounded production pipeline adapter.

use crate::{
    run_bounded_float32_recovery_clock_queue, BoundedFloat32PipelineCancellation,
    BoundedFloat32PipelineError, BoundedFloat32PipelineOutcome, BoundedSampleQueue,
    BoundedSampleQueueWait, ClockSource, FiniteSampleRecoveryActivation,
    FiniteSampleRecoveryPolicy, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, TimestampedFloat32InletSessionReport,
};

/// Owner-preserving rejection from the completed-report adapter.
#[derive(Debug)]
pub enum Float32SessionReportPipelineError {
    /// The completed report did not contain exactly one record.
    RecordCount {
        /// Exact received record count.
        actual: usize,
        /// Unchanged completed report and all records it owns.
        report: TimestampedFloat32InletSessionReport,
    },
    /// Existing bounded pipeline failure.
    Pipeline(BoundedFloat32PipelineError),
}

/// Feeds the sole owned record from a completed inlet report into the bounded pipeline.
///
/// Count validation precedes recovery, clock, and queue work. Session execution and errors
/// remain outside this adapter; it performs no connection, retry, retention, or discovery.
#[allow(clippy::too_many_arguments)]
pub fn run_float32_inlet_session_report_recovery_clock_queue<C>(
    report: TimestampedFloat32InletSessionReport,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    cancellation: BoundedFloat32PipelineCancellation<'_>,
) -> Result<BoundedFloat32PipelineOutcome, Float32SessionReportPipelineError>
where
    C: ClockSource,
{
    if report.record_count() != 1 {
        return Err(Float32SessionReportPipelineError::RecordCount {
            actual: report.record_count(),
            report,
        });
    }

    let mut records = report.into_records().into_iter();
    run_bounded_float32_recovery_clock_queue(
        recovery_activation,
        recovery_policy,
        |_| {
            Ok(records
                .next()
                .expect("the exactly-one report is acquired exactly once"))
        },
        clock_activation,
        clock_config,
        clock,
        queue,
        queue_wait,
        cancellation,
    )
    .map_err(Float32SessionReportPipelineError::Pipeline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        BoundedSampleQueueActivation, BoundedSampleQueuePopError, FiniteSampleRecoveryState,
        IntegratedClockCorrectionError, RawSourceTimestamp, RuntimeModule, Sample, SampleLimits,
        StreamHandshakeActivation, StreamHandshakeIdentity, StreamHandshakeLimits,
        TimestampedFloat32InletSession, TimestampedFloat32OutletSession,
        TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits, TimestampedSample,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    struct CountingClock(AtomicUsize);

    impl ClockSource for CountingClock {
        fn now(&mut self) -> f64 {
            self.0.fetch_add(1, Ordering::Relaxed);
            0.0
        }
    }

    fn sample_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn queue_activation() -> BoundedSampleQueueActivation {
        BoundedSampleQueueActivation::new(
            test_capability(RuntimeModule::BoundedSampleQueue),
            sample_activation(),
        )
        .unwrap()
    }

    fn recovery_activation() -> FiniteSampleRecoveryActivation {
        FiniteSampleRecoveryActivation::new(
            test_capability(RuntimeModule::FiniteSampleRecovery),
            queue_activation(),
        )
        .unwrap()
    }

    fn clock_activation() -> IntegratedClockCorrectionActivation {
        IntegratedClockCorrectionActivation::new(
            test_capability(RuntimeModule::IntegratedClockCorrection),
            sample_activation(),
        )
        .unwrap()
    }

    fn policy() -> FiniteSampleRecoveryPolicy {
        FiniteSampleRecoveryPolicy::new(
            1,
            3,
            Duration::ZERO,
            Duration::from_millis(1),
            Duration::from_secs(1),
        )
        .unwrap()
    }

    fn wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(1), Duration::from_secs(1)).unwrap()
    }

    fn config() -> IntegratedClockCorrectionConfig {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let address = peer.local_addr().unwrap();
        drop(peer);
        IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            address,
            71,
            1,
            256,
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
        .unwrap()
    }

    fn record(timestamp_bits: u64, value_bits: u32) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(value_bits)],
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(timestamp_bits)).unwrap(),
            None,
        )
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn completed_report(
        records: Vec<TimestampedSample<f32>>,
    ) -> TimestampedFloat32InletSessionReport {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let count = records.len();
        let worker = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight(
                sample_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let report = TimestampedFloat32InletSession::preflight(
            sample_activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            count,
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        worker.join().unwrap();
        report
    }

    #[test]
    fn p9_non_single_report_is_rejected_with_ownership_before_clock_or_queue() {
        let report = completed_report(vec![
            record(0x3ff0_0000_0000_0001, 0x3f80_0001),
            record(0x4000_0000_0000_0001, 0x4000_0001),
        ]);
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let mut clock = CountingClock(AtomicUsize::new(0));
        let error = run_float32_inlet_session_report_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config(),
            &mut clock,
            &queue,
            wait(),
            BoundedFloat32PipelineCancellation::new(
                &AtomicBool::new(false),
                &AtomicBool::new(false),
                &AtomicBool::new(false),
            ),
        )
        .unwrap_err();
        match error {
            Float32SessionReportPipelineError::RecordCount { actual, report } => {
                assert_eq!(actual, 2);
                let records = report.into_records();
                assert_eq!(records[0].sample().values()[0].to_bits(), 0x3f80_0001);
                assert_eq!(records[1].sample().values()[0].to_bits(), 0x4000_0001);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(clock.0.load(Ordering::Relaxed), 0);
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }

    #[test]
    fn p9_single_owned_record_reaches_pipeline_and_clock_cancellation_bypasses_queue() {
        let report = completed_report(vec![record(0x4008_0000_0000_0001, 0x7fc0_9009)]);
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let mut clock = CountingClock(AtomicUsize::new(0));
        let error = run_float32_inlet_session_report_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config(),
            &mut clock,
            &queue,
            wait(),
            BoundedFloat32PipelineCancellation::new(
                &AtomicBool::new(false),
                &AtomicBool::new(true),
                &AtomicBool::new(false),
            ),
        )
        .unwrap_err();
        match error {
            Float32SessionReportPipelineError::Pipeline(BoundedFloat32PipelineError::Clock {
                error: IntegratedClockCorrectionError::Cancelled,
                sample,
                states,
            }) => {
                assert_eq!(
                    sample.raw_source_timestamp().value().to_bits(),
                    0x4008_0000_0000_0001
                );
                assert_eq!(sample.sample().values()[0].to_bits(), 0x7fc0_9009);
                assert_eq!(
                    states,
                    vec![
                        FiniteSampleRecoveryState::Attempting { attempt: 1 },
                        FiniteSampleRecoveryState::Recovered { attempt: 1 },
                    ]
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert_eq!(clock.0.load(Ordering::Relaxed), 0);
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }
}
