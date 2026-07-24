// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Completed Float32 inlet-session report to bounded production pipeline adapter.

use crate::{
    run_bounded_float32_recovery_clock_queue, BoundedFloat32PipelineCancellation,
    BoundedFloat32PipelineError, BoundedFloat32PipelineOutcome, BoundedSampleQueue,
    BoundedSampleQueueWait, ClockSource, FiniteSampleRecoveryActivation, FiniteSampleRecoveryError,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryState, IntegratedClockCorrectionActivation,
    IntegratedClockCorrectionConfig, RecoveryAttemptFailure, RecoveryFailureClass,
    TimestampedFloat32InletSessionReport, TimestampedSample,
};
use std::collections::VecDeque;

/// Exact completion evidence for one report record.
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct Float32SessionReportRecordOutcome {
    /// Zero-based report record index.
    pub index: usize,
    /// Ordered recovery states produced for this record.
    pub states: Vec<FiniteSampleRecoveryState>,
}

/// Successful bounded batch result.
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct Float32SessionReportBatchOutcome {
    /// Exact ordered per-record completion evidence.
    pub completed: Vec<Float32SessionReportRecordOutcome>,
}

/// A per-record termination observed before clock or queue work.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Float32SessionReportBatchTermination {
    /// Recovery cancellation preceded acquisition.
    Cancelled,
    /// The recovery deadline preceded acquisition.
    Deadline,
    /// The acquisition owner classified a failure as terminal.
    Terminal {
        /// Exact caller-labelled terminal failure.
        failure: RecoveryAttemptFailure,
    },
    /// The acquisition owner exhausted its bounded attempts.
    Exhausted {
        /// Exact last caller-labelled retryable failure.
        failure: RecoveryAttemptFailure,
    },
}

/// Owner-preserving indexed rejection from the batch adapter.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Float32SessionReportBatchError {
    /// A completed report unexpectedly contained no records.
    EmptyReport {
        /// Unchanged completed report and all evidence it owns.
        report: TimestampedFloat32InletSessionReport,
    },
    /// Recovery stopped before the indexed record was acquired.
    NotAcquired {
        /// Zero-based report record index.
        index: usize,
        /// Exact pre-acquisition termination.
        termination: Float32SessionReportBatchTermination,
        /// Ordered recovery states for the current record.
        states: Vec<FiniteSampleRecoveryState>,
        /// Exact ordered completed prefix.
        completed: Vec<Float32SessionReportRecordOutcome>,
        /// Untouched current record followed by the untouched report suffix.
        remaining: Vec<TimestampedSample<f32>>,
    },
    /// Recovery setup failed before the indexed record was acquired.
    Recovery {
        /// Zero-based report record index.
        index: usize,
        /// Existing recovery setup error.
        error: FiniteSampleRecoveryError,
        /// Exact ordered completed prefix.
        completed: Vec<Float32SessionReportRecordOutcome>,
        /// Untouched current record followed by the untouched report suffix.
        remaining: Vec<TimestampedSample<f32>>,
    },
    /// Clock or queue work failed for the indexed record.
    Pipeline {
        /// Zero-based report record index.
        index: usize,
        /// Existing pipeline error retaining the current record when acquired.
        error: BoundedFloat32PipelineError,
        /// Exact ordered completed prefix.
        completed: Vec<Float32SessionReportRecordOutcome>,
        /// Untouched report suffix after the current record retained by `error`.
        remaining: Vec<TimestampedSample<f32>>,
    },
    /// The generic pipeline violated the adapter's infallible acquisition contract.
    Invariant {
        /// Zero-based report record index.
        index: usize,
        /// Unchanged impossible generic outcome.
        outcome: BoundedFloat32PipelineOutcome,
        /// Exact ordered completed prefix.
        completed: Vec<Float32SessionReportRecordOutcome>,
        /// Any record ownership still retained by the adapter.
        remaining: Vec<TimestampedSample<f32>>,
    },
}

#[allow(dead_code)]
fn restore_current(
    current: Option<TimestampedSample<f32>>,
    suffix: VecDeque<TimestampedSample<f32>>,
) -> Vec<TimestampedSample<f32>> {
    current.into_iter().chain(suffix).collect()
}

fn map_record_result(
    index: usize,
    result: Result<BoundedFloat32PipelineOutcome, BoundedFloat32PipelineError>,
    current: Option<TimestampedSample<f32>>,
    remaining: VecDeque<TimestampedSample<f32>>,
    mut completed: Vec<Float32SessionReportRecordOutcome>,
) -> Result<
    (
        Vec<Float32SessionReportRecordOutcome>,
        VecDeque<TimestampedSample<f32>>,
    ),
    Float32SessionReportBatchError,
> {
    match result {
        Ok(BoundedFloat32PipelineOutcome::Queued { states }) => {
            completed.push(Float32SessionReportRecordOutcome { index, states });
            Ok((completed, remaining))
        }
        Ok(BoundedFloat32PipelineOutcome::Cancelled { states }) => {
            Err(Float32SessionReportBatchError::NotAcquired {
                index,
                termination: Float32SessionReportBatchTermination::Cancelled,
                states,
                completed,
                remaining: restore_current(current, remaining),
            })
        }
        Ok(BoundedFloat32PipelineOutcome::Deadline { states }) => {
            Err(Float32SessionReportBatchError::NotAcquired {
                index,
                termination: Float32SessionReportBatchTermination::Deadline,
                states,
                completed,
                remaining: restore_current(current, remaining),
            })
        }
        Ok(BoundedFloat32PipelineOutcome::Terminal { failure, states }) => {
            Err(Float32SessionReportBatchError::NotAcquired {
                index,
                termination: Float32SessionReportBatchTermination::Terminal { failure },
                states,
                completed,
                remaining: restore_current(current, remaining),
            })
        }
        Ok(BoundedFloat32PipelineOutcome::Exhausted { failure, states }) => {
            Err(Float32SessionReportBatchError::NotAcquired {
                index,
                termination: Float32SessionReportBatchTermination::Exhausted { failure },
                states,
                completed,
                remaining: restore_current(current, remaining),
            })
        }
        Err(BoundedFloat32PipelineError::Recovery(error)) if current.is_some() => {
            Err(Float32SessionReportBatchError::Recovery {
                index,
                error,
                completed,
                remaining: restore_current(current, remaining),
            })
        }
        Err(error) => Err(Float32SessionReportBatchError::Pipeline {
            index,
            error,
            completed,
            remaining: restore_current(current, remaining),
        }),
    }
}

/// Sequentially feeds the report's exact nonzero record extent through the sole pipeline owner.
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
pub fn run_float32_inlet_session_report_batch_recovery_clock_queue<C>(
    report: TimestampedFloat32InletSessionReport,
    recovery_activation: FiniteSampleRecoveryActivation,
    recovery_policy: FiniteSampleRecoveryPolicy,
    clock_activation: IntegratedClockCorrectionActivation,
    clock_config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    queue: &BoundedSampleQueue,
    queue_wait: BoundedSampleQueueWait,
    cancellation: BoundedFloat32PipelineCancellation<'_>,
) -> Result<Float32SessionReportBatchOutcome, Float32SessionReportBatchError>
where
    C: ClockSource,
{
    if report.record_count() == 0 {
        return Err(Float32SessionReportBatchError::EmptyReport { report });
    }
    let mut remaining: VecDeque<_> = report.into_records().into();
    let mut completed = Vec::with_capacity(remaining.len());
    let record_count = remaining.len();
    for index in 0..record_count {
        let mut current = remaining.pop_front();
        let result = run_bounded_float32_recovery_clock_queue(
            recovery_activation,
            recovery_policy,
            |_| {
                current.take().ok_or_else(|| {
                    RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, u32::MAX)
                })
            },
            clock_activation,
            clock_config,
            clock,
            queue,
            queue_wait,
            cancellation,
        );
        (completed, remaining) = map_record_result(index, result, current, remaining, completed)?;
    }
    Ok(Float32SessionReportBatchOutcome { completed })
}

/// Terminal result of the legacy exactly-one-record adapter.
#[derive(Debug)]
pub enum Float32SessionReportPipelineOutcome {
    /// The sole report record was corrected and queued.
    Queued {
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
    /// Recovery stopped before acquiring the report record.
    NotAcquired {
        /// Unchanged completed report and all evidence it owns.
        report: TimestampedFloat32InletSessionReport,
        /// Exact pre-acquisition termination.
        termination: Float32SessionReportAcquisitionTermination,
        /// Ordered recovery states.
        states: Vec<FiniteSampleRecoveryState>,
    },
}

/// Recovery termination observed before the report record was acquired.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Float32SessionReportAcquisitionTermination {
    /// Recovery cancellation preceded acquisition.
    Cancelled,
    /// Recovery deadline preceded acquisition.
    Deadline,
}

/// Owner-preserving rejection from the legacy completed-report adapter.
#[derive(Debug)]
pub enum Float32SessionReportPipelineError {
    /// The completed report did not contain exactly one record.
    RecordCount {
        /// Exact received record count.
        actual: usize,
        /// Unchanged completed report and all records it owns.
        report: TimestampedFloat32InletSessionReport,
    },
    /// Recovery setup failed before acquiring the report record.
    Recovery {
        /// Existing recovery setup error.
        error: FiniteSampleRecoveryError,
        /// Unchanged completed report and all evidence it owns.
        report: TimestampedFloat32InletSessionReport,
    },
    /// Existing bounded pipeline failure.
    Pipeline(BoundedFloat32PipelineError),
    /// The generic pipeline produced a structurally impossible acquisition outcome.
    Invariant {
        /// Unchanged generic outcome.
        outcome: BoundedFloat32PipelineOutcome,
        /// Report retained when acquisition never occurred.
        report: Option<TimestampedFloat32InletSessionReport>,
    },
}

/// Preserves the historical exactly-one-record behavior.
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
) -> Result<Float32SessionReportPipelineOutcome, Float32SessionReportPipelineError>
where
    C: ClockSource,
{
    if report.record_count() != 1 {
        return Err(Float32SessionReportPipelineError::RecordCount {
            actual: report.record_count(),
            report,
        });
    }
    let mut retained_report = Some(report);
    let result = run_bounded_float32_recovery_clock_queue(
        recovery_activation,
        recovery_policy,
        |_| {
            let Some(report) = retained_report.take() else {
                return Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Terminal,
                    u32::MAX,
                ));
            };
            let mut records = report.into_records();
            match records.pop() {
                Some(sample) if records.is_empty() => Ok(sample),
                _ => Err(RecoveryAttemptFailure::new(
                    RecoveryFailureClass::Terminal,
                    u32::MAX,
                )),
            }
        },
        clock_activation,
        clock_config,
        clock,
        queue,
        queue_wait,
        cancellation,
    );
    match result {
        Ok(BoundedFloat32PipelineOutcome::Queued { states }) => {
            Ok(Float32SessionReportPipelineOutcome::Queued { states })
        }
        Ok(BoundedFloat32PipelineOutcome::Cancelled { states }) => match retained_report {
            Some(report) => Ok(Float32SessionReportPipelineOutcome::NotAcquired {
                report,
                termination: Float32SessionReportAcquisitionTermination::Cancelled,
                states,
            }),
            None => Err(Float32SessionReportPipelineError::Invariant {
                outcome: BoundedFloat32PipelineOutcome::Cancelled { states },
                report: None,
            }),
        },
        Ok(BoundedFloat32PipelineOutcome::Deadline { states }) => match retained_report {
            Some(report) => Ok(Float32SessionReportPipelineOutcome::NotAcquired {
                report,
                termination: Float32SessionReportAcquisitionTermination::Deadline,
                states,
            }),
            None => Err(Float32SessionReportPipelineError::Invariant {
                outcome: BoundedFloat32PipelineOutcome::Deadline { states },
                report: None,
            }),
        },
        Ok(outcome) => Err(Float32SessionReportPipelineError::Invariant {
            outcome,
            report: retained_report,
        }),
        Err(BoundedFloat32PipelineError::Recovery(error)) => match retained_report {
            Some(report) => Err(Float32SessionReportPipelineError::Recovery { error, report }),
            None => Err(Float32SessionReportPipelineError::Pipeline(
                BoundedFloat32PipelineError::Recovery(error),
            )),
        },
        Err(error) => Err(Float32SessionReportPipelineError::Pipeline(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        BoundedSampleQueueActivation, BoundedSampleQueuePopError, BoundedSampleQueuePushError,
        IntegratedClockCorrectionError, RawSourceTimestamp, RuntimeModule, Sample, SampleLimits,
        StreamHandshakeActivation, StreamHandshakeIdentity, StreamHandshakeLimits,
        TimestampedFloat32InletSession, TimestampedFloat32OutletSession,
        TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
        TimestampedFloat32SessionLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;

    struct SequenceClock {
        values: Vec<f64>,
        index: usize,
    }
    impl ClockSource for SequenceClock {
        fn now(&mut self) -> f64 {
            let value = self.values[self.index];
            self.index += 1;
            value
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
            Duration::from_millis(2),
            Duration::from_secs(2),
        )
        .unwrap()
    }
    fn wait() -> BoundedSampleQueueWait {
        BoundedSampleQueueWait::new(Duration::from_millis(2), Duration::from_millis(30)).unwrap()
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
    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }
    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }
    fn record(index: usize) -> TimestampedSample<f32> {
        TimestampedSample::new(
            Sample::new(
                SampleLimits::new(1).unwrap(),
                1,
                vec![f32::from_bits(0x3f80_0100 + index as u32)],
            )
            .unwrap(),
            RawSourceTimestamp::new(f64::from_bits(0x4008_0000_0000_0100 + index as u64)).unwrap(),
            None,
        )
    }
    fn completed_report(
        records: Vec<TimestampedSample<f32>>,
    ) -> TimestampedFloat32InletSessionReport {
        let count = records.len();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                sample_activation(),
                listener,
                &identity(),
                handshake_limits(),
                sample_limits(),
                TimestampedFloat32SessionLimits::new(1, count).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let report = TimestampedFloat32InletSession::preflight_bounded(
            sample_activation(),
            address,
            &identity(),
            handshake_limits(),
            sample_limits(),
            TimestampedFloat32SessionLimits::new(1, count).unwrap(),
            1,
            count,
        )
        .unwrap()
        .finish(&AtomicBool::new(false))
        .unwrap();
        worker.join().unwrap();
        report
    }
    fn correction(count: usize) -> (IntegratedClockCorrectionConfig, thread::JoinHandle<()>) {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer = socket.local_addr().unwrap();
        let worker = thread::spawn(move || {
            for _ in 0..count {
                let mut bytes = [0u8; 256];
                let (length, source) = socket.recv_from(&mut bytes).unwrap();
                let text = std::str::from_utf8(&bytes[..length]).unwrap();
                let mut fields = text.split("\r\n").nth(1).unwrap().split(' ');
                let id = fields.next().unwrap();
                let t0 = fields.next().unwrap();
                socket
                    .send_to(format!(" {id} {t0} 4.0 4.0").as_bytes(), source)
                    .unwrap();
            }
        });
        (
            IntegratedClockCorrectionConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                peer,
                91,
                1,
                256,
                Duration::from_millis(5),
                Duration::from_secs(2),
            )
            .unwrap(),
            worker,
        )
    }
    fn cancellation<'a>(
        r: &'a AtomicBool,
        c: &'a AtomicBool,
        q: &'a AtomicBool,
    ) -> BoundedFloat32PipelineCancellation<'a> {
        BoundedFloat32PipelineCancellation::new(r, c, q)
    }

    #[test]
    fn p4_batch_uses_each_reports_actual_nonzero_extent_and_preserves_order_and_allocations() {
        // One, two, and three are representative milestone fixtures; four proves that three is
        // not a universal maximum. Each completed report's own record_count is the exact extent.
        for count in [1, 2, 3, 4] {
            let report = completed_report((0..count).map(record).collect());
            let pointers: Vec<_> = report
                .records()
                .iter()
                .map(|r| r.sample().values().as_ptr())
                .collect();
            let queue = BoundedSampleQueue::new(queue_activation(), count).unwrap();
            let (config, responder) = correction(count);
            let mut clock = SequenceClock {
                values: (0..count)
                    .flat_map(|i| [i as f64 * 2.0, i as f64 * 2.0 + 1.0])
                    .collect(),
                index: 0,
            };
            let off = AtomicBool::new(false);
            let outcome = run_float32_inlet_session_report_batch_recovery_clock_queue(
                report,
                recovery_activation(),
                policy(),
                clock_activation(),
                config,
                &mut clock,
                &queue,
                wait(),
                cancellation(&off, &off, &off),
            )
            .unwrap();
            assert_eq!(
                outcome
                    .completed
                    .iter()
                    .map(|o| o.index)
                    .collect::<Vec<_>>(),
                (0..count).collect::<Vec<_>>()
            );
            for index in 0..count {
                let queued = queue.try_pop().unwrap();
                assert_eq!(queued.sample().values().as_ptr(), pointers[index]);
                assert_eq!(
                    queued.raw_source_timestamp().value().to_bits(),
                    0x4008_0000_0000_0100 + index as u64
                );
                assert_eq!(
                    queued.sample().values()[0].to_bits(),
                    0x3f80_0100 + index as u32
                );
            }
            responder.join().unwrap();
        }
    }

    #[test]
    fn p4_recovery_cancellation_retains_untouched_full_suffix_before_clock_queue() {
        let report = completed_report((0..3).map(record).collect());
        let pointers: Vec<_> = report
            .records()
            .iter()
            .map(|r| r.sample().values().as_ptr())
            .collect();
        let queue = BoundedSampleQueue::new(queue_activation(), 3).unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let config = IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            socket.local_addr().unwrap(),
            91,
            1,
            256,
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
        .unwrap();
        let mut clock = SequenceClock {
            values: vec![],
            index: 0,
        };
        let yes = AtomicBool::new(true);
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&yes, &yes, &yes),
        )
        .unwrap_err();
        match error {
            Float32SessionReportBatchError::NotAcquired {
                index: 0,
                termination: Float32SessionReportBatchTermination::Cancelled,
                completed,
                remaining,
                ..
            } => {
                assert!(completed.is_empty());
                assert_eq!(remaining.len(), 3);
                for i in 0..3 {
                    assert_eq!(remaining[i].sample().values().as_ptr(), pointers[i]);
                }
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }

    #[test]
    fn p4_queue_deadline_is_indexed_after_completed_prefix_and_retains_current_plus_suffix() {
        let report = completed_report((0..3).map(record).collect());
        let suffix_pointer = report.records()[2].sample().values().as_ptr();
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let (config, responder) = correction(2);
        let mut clock = SequenceClock {
            values: vec![0.0, 1.0, 2.0, 3.0],
            index: 0,
        };
        let no = AtomicBool::new(false);
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&no, &no, &no),
        )
        .unwrap_err();
        match error {
            Float32SessionReportBatchError::Pipeline {
                index: 1,
                error:
                    BoundedFloat32PipelineError::Queue {
                        error: BoundedSampleQueuePushError::Deadline(sample),
                        ..
                    },
                completed,
                remaining,
            } => {
                assert_eq!(completed.len(), 1);
                assert_eq!(completed[0].index, 0);
                assert_eq!(sample.sample().values()[0].to_bits(), 0x3f80_0101);
                assert_eq!(remaining.len(), 1);
                assert_eq!(remaining[0].sample().values().as_ptr(), suffix_pointer);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        responder.join().unwrap();
        drop(queue.try_pop().unwrap());
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }

    #[test]
    fn p4_clock_cancellation_precedes_queue_cancellation() {
        let report = completed_report(vec![record(0)]);
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let config = IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            socket.local_addr().unwrap(),
            91,
            1,
            256,
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
        .unwrap();
        let mut clock = SequenceClock {
            values: vec![],
            index: 0,
        };
        let no = AtomicBool::new(false);
        let yes = AtomicBool::new(true);
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&no, &yes, &yes),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            Float32SessionReportBatchError::Pipeline {
                index: 0,
                error: BoundedFloat32PipelineError::Clock { .. },
                ..
            }
        ));
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }

    #[test]
    fn p4_legacy_one_record_recovery_cancellation_preserves_the_completed_report() {
        let report = completed_report(vec![record(0)]);
        let pointer = report.records()[0].sample().values().as_ptr();
        let queue = BoundedSampleQueue::new(queue_activation(), 1).unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let config = IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            socket.local_addr().unwrap(),
            91,
            1,
            256,
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
        .unwrap();
        let mut clock = SequenceClock {
            values: vec![],
            index: 0,
        };
        let yes = AtomicBool::new(true);
        let no = AtomicBool::new(false);
        let outcome = run_float32_inlet_session_report_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&yes, &no, &no),
        )
        .unwrap();
        assert!(matches!(
            outcome,
            Float32SessionReportPipelineOutcome::NotAcquired {
                ref report,
                termination: Float32SessionReportAcquisitionTermination::Cancelled,
                ..
            } if report.records()[0].sample().values().as_ptr() == pointer
        ));
        assert!(yes.load(Ordering::Relaxed));
    }

    #[test]
    fn p4_recovery_deadline_and_closed_queue_retain_indexed_ownership() {
        let report = completed_report(vec![record(0), record(1)]);
        let pointers: Vec<_> = report
            .records()
            .iter()
            .map(|r| r.sample().values().as_ptr())
            .collect();
        let queue = BoundedSampleQueue::new(queue_activation(), 2).unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let config = IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            socket.local_addr().unwrap(),
            91,
            1,
            256,
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
        .unwrap();
        let deadline_policy = FiniteSampleRecoveryPolicy::new(
            1,
            3,
            Duration::ZERO,
            Duration::from_nanos(1),
            Duration::from_nanos(1),
        )
        .unwrap();
        let mut clock = SequenceClock {
            values: vec![],
            index: 0,
        };
        let no = AtomicBool::new(false);
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            deadline_policy,
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&no, &no, &no),
        )
        .unwrap_err();
        match error {
            Float32SessionReportBatchError::NotAcquired {
                index: 0,
                termination: Float32SessionReportBatchTermination::Deadline,
                remaining,
                ..
            } => {
                assert_eq!(remaining.len(), 2);
                assert_eq!(remaining[0].sample().values().as_ptr(), pointers[0]);
                assert_eq!(remaining[1].sample().values().as_ptr(), pointers[1]);
            }
            other => panic!("unexpected error: {other:?}"),
        }

        let report = completed_report(vec![record(0), record(1)]);
        let suffix_pointer = report.records()[1].sample().values().as_ptr();
        let queue = BoundedSampleQueue::new(queue_activation(), 2).unwrap();
        queue.close().unwrap();
        let (config, responder) = correction(1);
        let mut clock = SequenceClock {
            values: vec![0.0, 1.0],
            index: 0,
        };
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&no, &no, &no),
        )
        .unwrap_err();
        match error {
            Float32SessionReportBatchError::Pipeline {
                index: 0,
                error:
                    BoundedFloat32PipelineError::Queue {
                        error: BoundedSampleQueuePushError::Closed(sample),
                        ..
                    },
                completed,
                remaining,
            } => {
                assert!(completed.is_empty());
                assert_eq!(sample.sample().values()[0].to_bits(), 0x3f80_0100);
                assert_eq!(remaining.len(), 1);
                assert_eq!(remaining[0].sample().values().as_ptr(), suffix_pointer);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        responder.join().unwrap();
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Closed)
        ));
    }

    #[test]
    fn p4_terminal_and_exhausted_retain_exact_failure_payload_current_and_suffix() {
        for (outcome, expected_class, expected_code) in [
            (
                BoundedFloat32PipelineOutcome::Terminal {
                    failure: RecoveryAttemptFailure::new(RecoveryFailureClass::Terminal, 71),
                    states: vec![FiniteSampleRecoveryState::TerminalFailure {
                        attempt: 1,
                        code: 71,
                    }],
                },
                RecoveryFailureClass::Terminal,
                71,
            ),
            (
                BoundedFloat32PipelineOutcome::Exhausted {
                    failure: RecoveryAttemptFailure::new(RecoveryFailureClass::Retryable, 72),
                    states: vec![FiniteSampleRecoveryState::Exhausted { attempts: 3 }],
                },
                RecoveryFailureClass::Retryable,
                72,
            ),
        ] {
            let current = record(1);
            let current_pointer = current.sample().values().as_ptr();
            let suffix = record(2);
            let suffix_pointer = suffix.sample().values().as_ptr();
            let error = map_record_result(
                1,
                Ok(outcome),
                Some(current),
                VecDeque::from([suffix]),
                vec![Float32SessionReportRecordOutcome {
                    index: 0,
                    states: vec![FiniteSampleRecoveryState::Recovered { attempt: 1 }],
                }],
            )
            .unwrap_err();
            match error {
                Float32SessionReportBatchError::NotAcquired {
                    index: 1,
                    termination,
                    completed,
                    remaining,
                    ..
                } => {
                    let failure = match termination {
                        Float32SessionReportBatchTermination::Terminal { failure }
                        | Float32SessionReportBatchTermination::Exhausted { failure } => failure,
                        other => panic!("unexpected termination: {other:?}"),
                    };
                    assert_eq!(failure.class(), expected_class);
                    assert_eq!(failure.code(), expected_code);
                    assert_eq!(completed[0].index, 0);
                    assert_eq!(remaining[0].sample().values().as_ptr(), current_pointer);
                    assert_eq!(remaining[1].sample().values().as_ptr(), suffix_pointer);
                }
                other => panic!("unexpected error: {other:?}"),
            }
        }
    }

    #[test]
    fn p4_non_cancellation_clock_failure_retains_current_suffix_and_releases_endpoints() {
        let report = completed_report(vec![record(0), record(1)]);
        let tcp_address = report.peer();
        let current_pointer = report.records()[0].sample().values().as_ptr();
        let suffix_pointer = report.records()[1].sample().values().as_ptr();
        let reserved = UdpSocket::bind("127.0.0.1:0").unwrap();
        let udp_address = reserved.local_addr().unwrap();
        drop(reserved);
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let config = IntegratedClockCorrectionConfig::new(
            udp_address,
            peer.local_addr().unwrap(),
            91,
            1,
            256,
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
        .unwrap();
        let queue = BoundedSampleQueue::new(queue_activation(), 2).unwrap();
        let mut clock = SequenceClock {
            values: vec![f64::NAN],
            index: 0,
        };
        let no = AtomicBool::new(false);
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&no, &no, &no),
        )
        .unwrap_err();
        match error {
            Float32SessionReportBatchError::Pipeline {
                index: 0,
                error:
                    BoundedFloat32PipelineError::Clock {
                        error: IntegratedClockCorrectionError::NonFiniteClock { .. },
                        sample,
                        ..
                    },
                remaining,
                ..
            } => {
                assert_eq!(sample.sample().values().as_ptr(), current_pointer);
                assert_eq!(remaining[0].sample().values().as_ptr(), suffix_pointer);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        drop(peer);
        drop(TcpListener::bind(tcp_address).unwrap());
        drop(UdpSocket::bind(udp_address).unwrap());
    }

    #[test]
    fn p4_queue_cancellation_retains_current_and_untouched_suffix() {
        let report = completed_report(vec![record(0), record(1)]);
        let current_pointer = report.records()[0].sample().values().as_ptr();
        let suffix_pointer = report.records()[1].sample().values().as_ptr();
        let queue = BoundedSampleQueue::new(queue_activation(), 2).unwrap();
        let (config, responder) = correction(1);
        let mut clock = SequenceClock {
            values: vec![0.0, 1.0],
            index: 0,
        };
        let no = AtomicBool::new(false);
        let yes = AtomicBool::new(true);
        let error = run_float32_inlet_session_report_batch_recovery_clock_queue(
            report,
            recovery_activation(),
            policy(),
            clock_activation(),
            config,
            &mut clock,
            &queue,
            wait(),
            cancellation(&no, &no, &yes),
        )
        .unwrap_err();
        match error {
            Float32SessionReportBatchError::Pipeline {
                index: 0,
                error:
                    BoundedFloat32PipelineError::Queue {
                        error: BoundedSampleQueuePushError::Cancelled(sample),
                        ..
                    },
                completed,
                remaining,
            } => {
                assert!(completed.is_empty());
                assert_eq!(sample.sample().values().as_ptr(), current_pointer);
                assert_eq!(remaining[0].sample().values().as_ptr(), suffix_pointer);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        responder.join().unwrap();
        assert!(matches!(
            queue.try_pop(),
            Err(BoundedSampleQueuePopError::Empty)
        ));
    }
}
