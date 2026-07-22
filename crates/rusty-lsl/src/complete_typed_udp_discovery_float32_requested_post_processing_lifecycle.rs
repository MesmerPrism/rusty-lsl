// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete caller-named Float32 discovery/session lifecycle followed by one
//! caller-requested transactional report post-processing operation.

use crate::caller_requested_float32_report_post_processing::{
    CallerRequestedFloat32ReportPostProcessing, CallerRequestedFloat32ReportPostProcessingError,
};
use crate::exact_sequence_loss_health::ExactSequenceLossHealthSnapshot;
use crate::float32_session_report_post_processing_batch::{
    Float32PostProcessingBatchConfigError, Float32PostProcessingBatchOutcome,
};
use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;
use crate::{
    run_typed_udp_discovery_float32_session_inlet, ShortInfoQueryWire,
    ShortInfoResponseEnvelopeLimits, StreamHandshakeIdentity, StreamHandshakeLimits,
    StreamInfoObservedAdmissionLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleLimits, TimestampedFloat32SessionLimits,
    TypedUdpDiscoveryFloat32CompleteLifecycleError, TypedUdpDiscoveryRun, UdpDiscoveryActivation,
    UdpDiscoveryConfig,
};
use std::sync::atomic::AtomicBool;

/// Public projection of one exact caller-sequence relationship.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteTypedUdpDiscoveryFloat32SequenceClassification {
    First,
    Contiguous,
    Gap { missing_sequence_count: u64 },
    Duplicate,
    OutOfOrder { behind_high_water_by: u64 },
}

impl From<crate::exact_sequence_loss_health::ExactSequenceClassification>
    for CompleteTypedUdpDiscoveryFloat32SequenceClassification
{
    fn from(value: crate::exact_sequence_loss_health::ExactSequenceClassification) -> Self {
        use crate::exact_sequence_loss_health::ExactSequenceClassification as Private;
        match value {
            Private::First => Self::First,
            Private::Contiguous => Self::Contiguous,
            Private::Gap {
                missing_sequence_count,
            } => Self::Gap {
                missing_sequence_count,
            },
            Private::Duplicate => Self::Duplicate,
            Private::OutOfOrder {
                behind_high_water_by,
            } => Self::OutOfOrder {
                behind_high_water_by,
            },
        }
    }
}

/// Allocation-free public projection of the transaction's exact committed health.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
    observation_count: u64,
    first_count: u64,
    contiguous_count: u64,
    gap_count: u64,
    explicit_missing_sequence_count: u64,
    duplicate_count: u64,
    out_of_order_count: u64,
    retained_unchanged_count: u64,
    retained_changed_count: u64,
    high_water_sequence: Option<u64>,
    last_classification: Option<CompleteTypedUdpDiscoveryFloat32SequenceClassification>,
}

impl CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
    pub const fn observation_count(self) -> u64 {
        self.observation_count
    }
    pub const fn first_count(self) -> u64 {
        self.first_count
    }
    pub const fn contiguous_count(self) -> u64 {
        self.contiguous_count
    }
    pub const fn gap_count(self) -> u64 {
        self.gap_count
    }
    pub const fn explicit_missing_sequence_count(self) -> u64 {
        self.explicit_missing_sequence_count
    }
    pub const fn duplicate_count(self) -> u64 {
        self.duplicate_count
    }
    pub const fn out_of_order_count(self) -> u64 {
        self.out_of_order_count
    }
    pub const fn retained_unchanged_count(self) -> u64 {
        self.retained_unchanged_count
    }
    pub const fn retained_changed_count(self) -> u64 {
        self.retained_changed_count
    }
    pub const fn high_water_sequence(self) -> Option<u64> {
        self.high_water_sequence
    }
    pub const fn last_classification(
        self,
    ) -> Option<CompleteTypedUdpDiscoveryFloat32SequenceClassification> {
        self.last_classification
    }
}

fn public_health(
    snapshot: ExactSequenceLossHealthSnapshot,
) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
    CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
        observation_count: snapshot.observation_count(),
        first_count: snapshot.first_count(),
        contiguous_count: snapshot.contiguous_count(),
        gap_count: snapshot.gap_count(),
        explicit_missing_sequence_count: snapshot.explicit_missing_sequence_count(),
        duplicate_count: snapshot.duplicate_count(),
        out_of_order_count: snapshot.out_of_order_count(),
        retained_unchanged_count: snapshot.retained_unchanged_count(),
        retained_changed_count: snapshot.retained_changed_count(),
        high_water_sequence: snapshot.high_water_sequence(),
        last_classification: snapshot.last_classification().map(Into::into),
    }
}

/// Borrowed public evidence for one successfully processed retained record.
#[derive(Clone, Copy, Debug)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord<'a> {
    record: &'a crate::float32_session_report_post_processing_batch::Float32PostProcessingBatchRecordOutcome,
}

impl<'a> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord<'a> {
    pub const fn index(self) -> usize {
        self.record.index()
    }
    pub const fn sequence(self) -> u64 {
        self.record.sequence()
    }
    pub const fn sample(self) -> &'a crate::TimestampedSample<f32> {
        self.record.processed().sample()
    }
    pub fn classification(self) -> CompleteTypedUdpDiscoveryFloat32SequenceClassification {
        self.record.classification().into()
    }
    pub fn health(self) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
        public_health(self.record.health())
    }
}

/// Successful complete lifecycle evidence and its committed processing result.
#[derive(Debug)]
pub struct CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle {
    discovery: TypedUdpDiscoveryRun,
    response_index: usize,
    processing: Float32PostProcessingBatchOutcome,
}

impl CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle {
    /// Borrows the exact completed bounded discovery run.
    pub const fn discovery(&self) -> &TypedUdpDiscoveryRun {
        &self.discovery
    }

    /// Returns the exact receive-order index selected by the caller's name.
    pub const fn response_index(&self) -> usize {
        self.response_index
    }

    /// Borrows the unchanged canonical transactional processing outcome.
    pub fn record_count(&self) -> usize {
        self.processing.records().len()
    }

    /// Borrows one ordered processed record through a public evidence projection.
    pub fn record(
        &self,
        index: usize,
    ) -> Option<CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord<'_>> {
        self.processing
            .records()
            .get(index)
            .map(|record| CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecord { record })
    }

    /// Projects the exact health committed by the subordinate transaction owner.
    pub fn health(&self) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
        public_health(self.processing.health())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorStage {
    Lifecycle,
    Processing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification {
    Lifecycle,
    EmptyReport,
    SequenceExtentMismatch,
    RecordLimitExceeded,
    AdmissionMismatch,
    Empty,
    SequenceCount,
    RecordLimit,
    Allocation,
    Record,
}

/// Public construction refusal without exposing the private transaction owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingConfigError {
    ZeroMaximumRecords,
    MaximumRecordsUnrepresentable { requested: usize },
    InvalidRequest,
}

impl From<Float32PostProcessingBatchConfigError>
    for CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingConfigError
{
    fn from(value: Float32PostProcessingBatchConfigError) -> Self {
        match value {
            Float32PostProcessingBatchConfigError::ZeroMaximumRecords => Self::ZeroMaximumRecords,
            Float32PostProcessingBatchConfigError::MaximumRecordsUnrepresentable { requested } => {
                Self::MaximumRecordsUnrepresentable { requested }
            }
            Float32PostProcessingBatchConfigError::PostProcessing(_) => Self::InvalidRequest,
        }
    }
}

/// Public stage-specific failure retaining private transactional owners internally.
#[derive(Debug)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError {
    kind: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind,
}

#[derive(Debug)]
enum CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind {
    /// Discovery, selection, connection, transfer, completion, or cleanup failed.
    Lifecycle {
        /// Exact caller-requested processing mode, not consumed by processing.
        request: RequestedTimestampPostProcessing,
        /// Exact caller-supplied sequences, in unchanged allocation and order.
        sequences: Vec<u64>,
        /// Existing stage-specific complete lifecycle error.
        error: TypedUdpDiscoveryFloat32CompleteLifecycleError,
    },
    /// The completed report was transactionally refused.
    Processing {
        /// Exact completed discovery evidence.
        discovery: TypedUdpDiscoveryRun,
        /// Exact receive-order selected response index.
        response_index: usize,
        /// Existing owner-preserving admission or processing refusal.
        error: CallerRequestedFloat32ReportPostProcessingError,
    },
}

impl CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError {
    pub const fn stage(&self) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorStage {
        match self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                ..
            } => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorStage::Lifecycle,
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                ..
            } => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorStage::Processing,
        }
    }
    pub const fn lifecycle_error(&self) -> Option<&TypedUdpDiscoveryFloat32CompleteLifecycleError> {
        match &self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                error,
                ..
            } => Some(error),
            _ => None,
        }
    }
    pub const fn discovery(&self) -> Option<&TypedUdpDiscoveryRun> {
        match &self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                discovery,
                ..
            } => Some(discovery),
            _ => None,
        }
    }
    pub const fn response_index(&self) -> Option<usize> {
        match self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                response_index,
                ..
            } => Some(response_index),
            _ => None,
        }
    }
    pub const fn request(&self) -> Option<RequestedTimestampPostProcessing> {
        match self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                request,
                ..
            } => Some(request),
            _ => None,
        }
    }
    pub fn sequences(&self) -> Option<&[u64]> {
        match &self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                sequences,
                ..
            } => Some(sequences),
            _ => None,
        }
    }

    /// Returns the exact retained caller-sequence extent when processing owned it.
    pub fn sequence_count(&self) -> Option<usize> {
        use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmissionError as Admission;
        use crate::float32_session_report_post_processing_batch::Float32PostProcessingBatchError as Batch;
        match &self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                sequences,
                ..
            } => Some(sequences.len()),
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                error,
                ..
            } => match error {
                CallerRequestedFloat32ReportPostProcessingError::Admission(
                    Admission::EmptyReport { sequences, .. }
                    | Admission::SequenceExtentMismatch { sequences, .. }
                    | Admission::RecordLimitExceeded { sequences, .. },
                ) => Some(sequences.len()),
                CallerRequestedFloat32ReportPostProcessingError::AdmissionMismatch {
                    plan, ..
                } => Some(plan.sequences().len()),
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(
                    Batch::Empty { sequences, .. }
                    | Batch::SequenceCount { sequences, .. }
                    | Batch::RecordLimit { sequences, .. }
                    | Batch::Allocation { sequences, .. },
                ) => Some(sequences.len()),
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(
                    Batch::Record {
                        completed,
                        remaining_sequences,
                        ..
                    },
                ) => Some(completed.len() + 1 + remaining_sequences.len()),
            },
        }
    }

    /// Returns the exact retained report-record extent when a report existed.
    pub fn record_count(&self) -> Option<usize> {
        use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmissionError as Admission;
        use crate::float32_session_report_post_processing_batch::Float32PostProcessingBatchError as Batch;
        match &self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                ..
            } => None,
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                error,
                ..
            } => match error {
                CallerRequestedFloat32ReportPostProcessingError::Admission(
                    Admission::EmptyReport { report, .. }
                    | Admission::SequenceExtentMismatch { report, .. }
                    | Admission::RecordLimitExceeded { report, .. },
                ) => Some(report.record_count()),
                CallerRequestedFloat32ReportPostProcessingError::AdmissionMismatch {
                    plan, ..
                } => Some(plan.record_count()),
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(
                    Batch::Empty { records, .. }
                    | Batch::SequenceCount { records, .. }
                    | Batch::RecordLimit { records, .. }
                    | Batch::Allocation { records, .. },
                ) => Some(records.len()),
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(
                    Batch::Record {
                        completed,
                        remaining_records,
                        ..
                    },
                ) => Some(completed.len() + 1 + remaining_records.len()),
            },
        }
    }
    pub const fn classification(
        &self,
    ) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification {
        use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmissionError as Admission;
        use crate::float32_session_report_post_processing_batch::Float32PostProcessingBatchError as Batch;
        match &self.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle { .. } => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::Lifecycle,
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing { error, .. } => match error {
                CallerRequestedFloat32ReportPostProcessingError::Admission(Admission::EmptyReport { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::EmptyReport,
                CallerRequestedFloat32ReportPostProcessingError::Admission(Admission::SequenceExtentMismatch { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::SequenceExtentMismatch,
                CallerRequestedFloat32ReportPostProcessingError::Admission(Admission::RecordLimitExceeded { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::RecordLimitExceeded,
                CallerRequestedFloat32ReportPostProcessingError::AdmissionMismatch { .. } => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::AdmissionMismatch,
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(Batch::Empty { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::Empty,
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(Batch::SequenceCount { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::SequenceCount,
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(Batch::RecordLimit { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::RecordLimit,
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(Batch::Allocation { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::Allocation,
                CallerRequestedFloat32ReportPostProcessingError::PostProcessing(Batch::Record { .. }) => CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::Record,
            }
        }
    }
}

/// Sole composition owner retaining the existing transactional processing owner.
#[derive(Debug, PartialEq)]
pub struct CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing {
    processing: CallerRequestedFloat32ReportPostProcessing,
}

impl CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing {
    /// Constructs an explicitly bounded owner for exactly one requested mode.
    pub fn new(
        maximum_records: usize,
        request: RequestedTimestampPostProcessing,
    ) -> Result<Self, CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingConfigError> {
        Ok(Self {
            processing: CallerRequestedFloat32ReportPostProcessing::new(maximum_records, request)
                .map_err(
                CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingConfigError::from,
            )?,
        })
    }

    /// Returns the caller-requested mode bound to this transaction owner.
    pub const fn request(&self) -> RequestedTimestampPostProcessing {
        self.processing.request()
    }

    /// Returns the exact per-transaction report bound.
    pub const fn maximum_records(&self) -> usize {
        self.processing.maximum_records()
    }

    /// Borrows the currently committed exact sequence/processing health.
    pub fn health(&self) -> CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingHealth {
        public_health(self.processing.health())
    }

    /// Runs bounded exact-name discovery and the canonical Float32 session, then
    /// delegates the completed report to the existing all-or-nothing owner.
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &mut self,
        discovery_activation: UdpDiscoveryActivation,
        discovery_config: UdpDiscoveryConfig,
        query: &ShortInfoQueryWire,
        discovery_cancelled: &AtomicBool,
        envelope_limits: ShortInfoResponseEnvelopeLimits,
        admission_limits: StreamInfoObservedAdmissionLimits,
        stream_name: &str,
        session_activation: TimestampedFloat32SampleActivation,
        expected_identity: &StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        sample_limits: TimestampedFloat32SampleLimits,
        session_limits: TimestampedFloat32SessionLimits,
        channel_count: usize,
        record_count: usize,
        session_cancelled: &AtomicBool,
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
    ) -> Result<
        CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError,
    > {
        let completed = match run_typed_udp_discovery_float32_session_inlet(
            discovery_activation,
            discovery_config,
            query,
            discovery_cancelled,
            envelope_limits,
            admission_limits,
            stream_name,
            session_activation,
            expected_identity,
            handshake_limits,
            sample_limits,
            session_limits,
            channel_count,
            record_count,
            session_cancelled,
        ) {
            Ok(completed) => completed,
            Err(error) => {
                return Err(
                    CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError { kind: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Lifecycle {
                        request,
                        sequences,
                        error,
                    }},
                );
            }
        };
        let (discovery, response_index, report) = completed.into_parts();
        let processing = match self
            .processing
            .process_requested_report(request, sequences, report)
        {
            Ok(processing) => processing,
            Err(error) => {
                return Err(
                    CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError { kind: CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                        discovery,
                        response_index,
                        error,
                    }},
                );
            }
        };
        Ok(
            CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle {
                discovery,
                response_index,
                processing,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller_requested_float32_report_post_processing_admission::CallerRequestedFloat32ReportPostProcessingAdmissionError;
    use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessingConfig;
    use crate::runtime_activation::test_capability;
    use crate::{
        MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample, SampleLimits,
        ShortInfoQuery, ShortInfoQueryWireLimits, StreamDescriptorLimits,
        StreamHandshakeActivation, StreamInfoVolatileFieldLimits, TimestampedFloat32OutletSession,
        TimestampedSample, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn request() -> RequestedTimestampPostProcessing {
        RequestedTimestampPostProcessing::DeJitter(
            RequestedTimestampPostProcessingConfig::new(4, 1.0, 10.0).unwrap(),
        )
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(5))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "59595959-2222-4333-8444-555555555555".into(),
            "p59-host".into(),
            "p59-source".into(),
            "p59-session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn discovery_activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap()
    }

    fn session_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 19, limits).unwrap(),
            limits,
        )
        .unwrap()
    }

    fn admission_limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
        )
    }

    fn document(port: u16) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", "1".to_owned()),
            ("channel_format", "float32".to_owned()),
            ("source_id", "p59-source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "59595959-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "p59-session".to_owned()),
            ("hostname", "p59-host".to_owned()),
            ("v4address", "127.0.0.1".to_owned()),
            ("v4data_port", "43001".to_owned()),
            ("v4service_port", port.to_string()),
            ("v6address", "2001:db8::10".to_owned()),
            ("v6data_port", "43003".to_owned()),
            ("v6service_port", "43004".to_owned()),
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in fields {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn run(
        owner: &mut CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing,
        sequences: Vec<u64>,
    ) -> Result<
        CompletedTypedUdpDiscoveryFloat32RequestedPostProcessingLifecycle,
        CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingError,
    > {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let records = vec![
            TimestampedSample::new(
                Sample::new(
                    SampleLimits::new(1).unwrap(),
                    1,
                    vec![f32::from_bits(0x3fc0_0001)],
                )
                .unwrap(),
                RawSourceTimestamp::new(10.0).unwrap(),
                None,
            ),
            TimestampedSample::new(
                Sample::new(
                    SampleLimits::new(1).unwrap(),
                    1,
                    vec![f32::from_bits(0xc020_0001)],
                )
                .unwrap(),
                RawSourceTimestamp::new(11.0).unwrap(),
                None,
            ),
        ];
        let outlet = thread::spawn(move || {
            TimestampedFloat32OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                TimestampedFloat32SampleLimits::new(
                    Duration::from_millis(5),
                    Duration::from_secs(1),
                )
                .unwrap(),
                TimestampedFloat32SessionLimits::new(1, 2).unwrap(),
                &records,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let document = document(endpoint.port());
        let bytes = document.len();
        let responder = thread::spawn(move || {
            let mut buffer = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut buffer).unwrap();
            socket
                .send_to(format!("19\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let result = owner.run(
            discovery_activation(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    bytes + 32,
                    1,
                    Duration::from_millis(5),
                    Duration::from_millis(250),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            ),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(bytes, bytes + 32).unwrap(),
            admission_limits(),
            "selected",
            session_activation(),
            &identity(),
            handshake_limits(),
            TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap(),
            TimestampedFloat32SessionLimits::new(1, 2).unwrap(),
            1,
            2,
            &AtomicBool::new(false),
            request(),
            sequences,
        );
        responder.join().unwrap();
        outlet.join().unwrap();
        TcpListener::bind(endpoint).unwrap();
        result
    }

    #[test]
    fn p60_public_success_projects_discovery_selection_records_and_exact_health() {
        let mut owner =
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing::new(2, request()).unwrap();
        let outcome = run(&mut owner, vec![u64::MIN, u64::MAX]).unwrap();
        assert_eq!(outcome.response_index(), 0);
        assert_eq!(outcome.discovery().responses().len(), 1);
        assert_eq!(outcome.health(), owner.health());
        assert_eq!(
            (0..outcome.record_count())
                .map(|index| outcome.record(index).unwrap().sequence())
                .collect::<Vec<_>>(),
            vec![u64::MIN, u64::MAX]
        );
        assert_eq!(
            outcome.record(0).unwrap().sample().sample().values()[0].to_bits(),
            0x3fc0_0001
        );
        assert_eq!(outcome.health().observation_count(), 2);
        assert_eq!(outcome.health().first_count(), 1);
        assert_eq!(outcome.health().gap_count(), 1);
        assert_eq!(
            outcome.health().explicit_missing_sequence_count(),
            u64::MAX - 1
        );
        assert_eq!(outcome.health().high_water_sequence(), Some(u64::MAX));
        assert_eq!(
            outcome.record(0).unwrap().classification(),
            CompleteTypedUdpDiscoveryFloat32SequenceClassification::First
        );
    }

    #[test]
    fn p60_public_processing_error_projects_stage_classification_and_selection_evidence() {
        let mut owner =
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessing::new(2, request()).unwrap();
        let before = owner.health();
        let error = run(&mut owner, vec![77]).unwrap_err();
        assert_eq!(
            error.stage(),
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorStage::Processing
        );
        assert_eq!(error.classification(), CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorClassification::SequenceExtentMismatch);
        assert_eq!(error.response_index(), Some(0));
        assert_eq!(error.discovery().unwrap().responses().len(), 1);
        assert_eq!(error.sequence_count(), Some(1));
        assert_eq!(error.record_count(), Some(2));
        match error.kind {
            CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingErrorKind::Processing {
                discovery, response_index: 0,
                error: CallerRequestedFloat32ReportPostProcessingError::Admission(
                    CallerRequestedFloat32ReportPostProcessingAdmissionError::SequenceExtentMismatch {
                        sequence_count: 1, report_record_count: 2, sequences, report, ..
                    }),
            } => {
                assert_eq!(discovery.responses().len(), 1);
                assert_eq!(sequences, [77]);
                assert_eq!(report.record_count(), 2);
                assert_eq!(report.records()[0].sample().values()[0].to_bits(), 0x3fc0_0001);
            }
            other => panic!("unexpected refusal: {other:?}"),
        }
        assert_eq!(owner.health(), before);
    }
}
