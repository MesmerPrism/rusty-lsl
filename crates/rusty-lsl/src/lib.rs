// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Dependency-free local contracts for Rusty LSL.
//!
//! This crate currently implements only bounded metadata, sample-shape,
//! timestamped-chunk, core stream-descriptor, and flat metadata-tree
//! construction, descriptor/sample shape and format binding, and timestamped
//! descriptor/sample, non-empty descriptor/chunk, and stream-definition
//! composition, a borrowed static stream-info semantic projection, plus
//! a bounded static numeric lexical projection for its channel count and
//! nominal sample rate, plus
//! bounded XML legal-text, element-name, and character-data
//! representation contracts, leaf-only XML value composition, and a bounded
//! parent-before-child XML container/leaf hierarchy, plus bounded borrowed
//! element-tree string serialization, bounded opaque volatile stream-info
//! accepted data, and its bounded local XML element projection.
//! A separate one-shot provider-input snapshot contract admits complete,
//! disjoint caller-supplied volatile lanes before producing that accepted data.
//! Accepted static-plus-description and volatile trees can also be consumed
//! into one bounded local static, volatile, then `desc` element hierarchy.
//! A separate bounded borrowed projection can apply the accepted observed
//! stream-info document envelope without changing compact serialization.
//! One separate bounded local contract encodes and parses only the canonical
//! three-line protocol-110 short-info query payload candidate.
//! Another bounded local contract encodes and parses only the
//! observed short-info response envelope around an accepted document body.
//! A closed allocation-free data contract also exposes only the documented
//! default discovery port and exact displayed destination spellings.
//! One separately locked and explicitly invoked synchronous runtime call owns
//! only bounded caller-configured UDP discovery with loopback evidence.
//! It does not implement endpoint selection, official interoperability,
//! clocks, inlet, outlet, FFI, or Morphospace authority behavior.

#[cfg(test)]
/// Test-only mutex whose ownership remains recoverable after a failed assertion.
pub(crate) struct RecoveringTestMutex(std::sync::Mutex<()>);

#[cfg(test)]
impl RecoveringTestMutex {
    /// Creates an unlocked test-only mutex.
    pub(crate) const fn new() -> Self {
        Self(std::sync::Mutex::new(()))
    }

    /// Acquires the mutex while retaining ownership after earlier test panic poison.
    pub(crate) fn lock(&self) -> Result<std::sync::MutexGuard<'_, ()>, std::convert::Infallible> {
        Ok(self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner))
    }
}

#[cfg(test)]
pub(crate) static MULTICAST_LOOPBACK_TEST_LOCK: RecoveringTestMutex = RecoveringTestMutex::new();

#[cfg(test)]
pub(crate) fn lock_multicast_loopback_tests() -> std::sync::MutexGuard<'static, ()> {
    MULTICAST_LOOPBACK_TEST_LOCK.lock().unwrap()
}

mod all_format_bounded_chunk_session;
mod bounded_fixed_record_transport;
mod bounded_float32_recovery_clock_queue_runtime;
mod bounded_sample_queue_runtime;
mod caller_requested_float32_advisory_report_package;
mod caller_requested_float32_advisory_report_package_history;
mod caller_requested_float32_comparative_advisory_evidence;
mod caller_requested_float32_comparative_advisory_evidence_history;
mod caller_requested_float32_comparative_advisory_evidence_snapshot;
mod caller_requested_float32_comparative_advisory_evidence_snapshot_history;
mod caller_requested_float32_report_advisory_evidence;
mod caller_requested_float32_report_advisory_evidence_history;
mod caller_requested_float32_report_post_processing;
mod caller_requested_float32_report_post_processing_admission;
mod clock_filter_selection;
mod clock_offset_application;
pub mod contract;
mod descriptor_sample;
mod documented_discovery_destination;
mod documented_discovery_query_proposal;
mod exact_sequence_loss_health;
mod finite_sample_recovery_runtime;
mod fixed_width_numeric_sample_runtime;
mod float32_session_batch_health;
mod float32_session_report_post_processing_batch;
mod float32_session_report_recovery_clock_queue;
mod float32_session_report_requested_post_processing;
mod format_neutral_session_runtime;
mod integrated_clock_correction_runtime;
mod metadata;
mod metadata_tree;
mod metadata_xml_projection;
mod morphospace_float32_advisory_report_package_delta_history;
mod morphospace_float32_advisory_report_package_delta_proposal;
mod morphospace_float32_comparative_advisory_evidence_delta_history;
mod morphospace_float32_comparative_advisory_evidence_delta_proposal;
mod morphospace_float32_comparative_advisory_evidence_snapshot_delta_proposal;
mod morphospace_float32_report_advisory_proposal;
mod morphospace_float32_report_advisory_snapshot;
mod morphospace_float32_report_advisory_snapshot_history;
mod morphospace_float32_report_observation;
mod morphospace_float32_report_observation_history;
mod morphospace_float32_report_observation_window;
mod morphospace_float32_report_trend_proposal;
mod morphospace_float32_report_window_delta_history;
mod morphospace_float32_report_window_delta_proposal;
mod morphospace_float32_report_window_stability_history;
mod morphospace_float32_report_window_stability_proposal;
mod morphospace_float32_retained_advisory_summary;
mod morphospace_float32_retained_advisory_summary_history;
mod raw_clock_exchange;
mod requested_timestamp_post_processing;
mod requested_timestamp_post_processing_loss_health;
pub mod runtime;
mod runtime_activation;
mod sample;
mod short_info_discovery_responder_runtime;
mod short_info_query_wire;
mod short_info_response_envelope;
mod stream_definition;
mod stream_descriptor;
mod stream_handshake;
mod stream_info_description_xml;
mod stream_info_implementation_version_provider;
mod stream_info_observed_document;
mod stream_info_observed_document_admission;
mod stream_info_observed_document_parser;
mod stream_info_ordered_xml;
mod stream_info_runtime_provider;
mod stream_info_static_fields;
mod stream_info_static_numeric_spellings;
mod stream_info_static_xml;
mod stream_info_three_owner_observed_document;
mod stream_info_three_owner_snapshot;
mod stream_info_transport_provider;
mod stream_info_volatile_fields;
mod stream_info_volatile_snapshot;
mod stream_info_volatile_xml;
mod string_sample_runtime;
mod timestamped;
mod timestamped_descriptor_chunk;
mod timestamped_descriptor_sample;
mod timestamped_float32_sample_runtime;
mod timestamped_float32_session_runtime;
mod timestamped_float32_two_record_chunk_runtime;
mod typed_short_info_response_observation;
mod typed_udp_discovery_double64_session_connection;
mod typed_udp_discovery_endpoint;
mod typed_udp_discovery_float32;
mod typed_udp_discovery_float32_clock_correction_queue;
mod typed_udp_discovery_float32_queue;
mod typed_udp_discovery_float32_recovery_clock_correction_queue;
mod typed_udp_discovery_float32_recovery_queue;
mod typed_udp_discovery_float32_session_batch_pipeline;
mod typed_udp_discovery_float32_session_connection;
mod typed_udp_discovery_handshake;
mod typed_udp_discovery_integer_session_connection;
mod typed_udp_discovery_response;
mod typed_udp_discovery_run;
mod typed_udp_discovery_selection;
mod typed_udp_discovery_session_contract;
mod typed_udp_discovery_string_session_connection;
mod udp_discovery;
mod xml_character_data;
mod xml_element_serialization;
mod xml_element_tree;
mod xml_leaf_element;
mod xml_value;

pub use all_format_bounded_chunk_session::{
    run_timestamped_double64_bounded_chunk_inlet, run_timestamped_double64_bounded_chunk_outlet,
    run_timestamped_float32_bounded_chunk_inlet, run_timestamped_float32_bounded_chunk_outlet,
    run_timestamped_int16_bounded_chunk_inlet, run_timestamped_int16_bounded_chunk_outlet,
    run_timestamped_int32_bounded_chunk_inlet, run_timestamped_int32_bounded_chunk_outlet,
    run_timestamped_int64_bounded_chunk_inlet, run_timestamped_int64_bounded_chunk_outlet,
    run_timestamped_int8_bounded_chunk_inlet, run_timestamped_int8_bounded_chunk_outlet,
    run_timestamped_string_bounded_chunk_inlet, run_timestamped_string_bounded_chunk_outlet,
    TimestampedDouble64BoundedChunkError, TimestampedDouble64BoundedChunkInletSessionReport,
    TimestampedDouble64BoundedChunkOutletSessionReport, TimestampedFloat32BoundedChunkError,
    TimestampedFloat32BoundedChunkInletSessionReport,
    TimestampedFloat32BoundedChunkOutletSessionReport, TimestampedInt16BoundedChunkError,
    TimestampedInt16BoundedChunkInletSessionReport,
    TimestampedInt16BoundedChunkOutletSessionReport, TimestampedInt32BoundedChunkError,
    TimestampedInt32BoundedChunkInletSessionReport,
    TimestampedInt32BoundedChunkOutletSessionReport, TimestampedInt64BoundedChunkError,
    TimestampedInt64BoundedChunkInletSessionReport,
    TimestampedInt64BoundedChunkOutletSessionReport, TimestampedInt8BoundedChunkError,
    TimestampedInt8BoundedChunkInletSessionReport, TimestampedInt8BoundedChunkOutletSessionReport,
    TimestampedStringBoundedChunkError, TimestampedStringBoundedChunkInletSessionReport,
    TimestampedStringBoundedChunkOutletSessionReport,
};
pub use bounded_float32_recovery_clock_queue_runtime::{
    run_bounded_float32_recovery_clock_queue, BoundedFloat32PipelineCancellation,
    BoundedFloat32PipelineError, BoundedFloat32PipelineOutcome,
};
pub use bounded_sample_queue_runtime::{
    BoundedSampleQueue, BoundedSampleQueueActivation, BoundedSampleQueueActivationError,
    BoundedSampleQueueCloseError, BoundedSampleQueueCreateError, BoundedSampleQueuePopError,
    BoundedSampleQueuePushError, BoundedSampleQueueWait, BoundedSampleQueueWaitError,
    BOUNDED_SAMPLE_QUEUE_EFFECTIVE_MARKER, BOUNDED_SAMPLE_QUEUE_FEATURE_ID,
};
pub use clock_filter_selection::{
    ClockFilterSelection, ClockFilterSelectionError, ClockFilterSelectionLimit,
    ClockFilterSelectionLimitError,
};
pub use clock_offset_application::{
    ClockOffset, ClockOffsetApplication, ClockOffsetApplicationError, ClockOffsetError,
};
pub use descriptor_sample::{
    BoundDescriptorSample, DescriptorSampleBound, DescriptorSampleError, DescriptorSampleInput,
    DescriptorSampleLimits,
};
pub use documented_discovery_destination::{
    DocumentedDiscoveryDestination, DOCUMENTED_DEFAULT_DISCOVERY_PORT,
    DOCUMENTED_DISCOVERY_DESTINATIONS,
};
pub use documented_discovery_query_proposal::DocumentedDiscoveryQueryProposal;
pub use finite_sample_recovery_runtime::{
    run_finite_sample_recovery, FiniteSampleRecoveryActivation,
    FiniteSampleRecoveryActivationError, FiniteSampleRecoveryError, FiniteSampleRecoveryOutcome,
    FiniteSampleRecoveryPolicy, FiniteSampleRecoveryPolicyError, FiniteSampleRecoveryState,
    RecoveryAttemptFailure, RecoveryFailureClass, FINITE_SAMPLE_RECOVERY_EFFECTIVE_MARKER,
    FINITE_SAMPLE_RECOVERY_FEATURE_ID,
};
pub use fixed_width_numeric_sample_runtime::{
    run_fixed_width_numeric_inlet, run_fixed_width_numeric_outlet,
    run_fixed_width_numeric_sequence_inlet, run_fixed_width_numeric_sequence_outlet,
    FixedWidthNumericPairRecord, FixedWidthNumericRecord, FixedWidthNumericRecordSequence,
    FixedWidthNumericSampleActivation, FixedWidthNumericSampleActivationError,
    FixedWidthNumericSampleError, FixedWidthNumericSampleLimitError, FixedWidthNumericSampleLimits,
    FixedWidthNumericValue, FIXED_WIDTH_NUMERIC_SAMPLE_EFFECTIVE_MARKER,
    FIXED_WIDTH_NUMERIC_SAMPLE_FEATURE_ID,
};
pub use float32_session_batch_health::{
    Float32SessionBatchHealth, Float32SessionBatchHealthClassification,
};
pub use float32_session_report_recovery_clock_queue::{
    run_float32_inlet_session_report_batch_recovery_clock_queue,
    run_float32_inlet_session_report_recovery_clock_queue,
    Float32SessionReportAcquisitionTermination, Float32SessionReportBatchError,
    Float32SessionReportBatchOutcome, Float32SessionReportBatchTermination,
    Float32SessionReportPipelineError, Float32SessionReportPipelineOutcome,
    Float32SessionReportRecordOutcome,
};
pub use integrated_clock_correction_runtime::{
    run_integrated_clock_correction, ClockSource, IntegratedClockCorrection,
    IntegratedClockCorrectionActivation, IntegratedClockCorrectionActivationError,
    IntegratedClockCorrectionConfig, IntegratedClockCorrectionConfigError,
    IntegratedClockCorrectionError, INTEGRATED_CLOCK_CORRECTION_EFFECTIVE_MARKER,
    INTEGRATED_CLOCK_CORRECTION_FEATURE_ID,
};
pub use metadata::{
    BoundedMetadata, MetadataBound, MetadataDescription, MetadataError, MetadataField,
    MetadataLimits, MetadataTextRole,
};
pub use metadata_tree::{
    MetadataNode, MetadataNodeInput, MetadataTree, MetadataTreeBound, MetadataTreeError,
    MetadataTreeLimits, MetadataTreeTextRole,
};
pub use metadata_xml_projection::{
    project_metadata_tree_to_xml_element_tree, MetadataXmlProjectionError,
    MetadataXmlProjectionLimits,
};
pub use raw_clock_exchange::{
    RawClockExchange, RawClockExchangeFormulaError, RawClockExchangeFormulaResult,
    RawClockExchangeFormulaStage, RawClockExchangeInputError, RawClockExchangeTimestampRole,
};
pub use runtime_activation::{
    admit_runtime_activation, RuntimeActivationAdmission, RuntimeActivationError,
    RuntimeActivationOutcome, RuntimeActivationReceipt, RuntimeActivationSelection, RuntimeModule,
    RuntimeModuleCapability, ACCEPTED_FEATURE_LOCK_FINGERPRINT, ACCEPTED_FEATURE_LOCK_REVISION,
};
pub use sample::{Sample, SampleBound, SampleError, SampleLimits};
pub use short_info_discovery_responder_runtime::{
    run_explicit_ipv4_multicast_short_info_responder,
    run_explicit_loopback_multicast_short_info_responder, run_short_info_responder,
    ShortInfoResponderActivation, ShortInfoResponderActivationError, ShortInfoResponderError,
    ShortInfoResponderLimitError, ShortInfoResponderLimits, ShortInfoResponderRun,
    ShortInfoResponderTermination, DOCUMENTED_IPV4_MULTICAST_GROUP, DOCUMENTED_IPV4_MULTICAST_PORT,
    SHORT_INFO_RESPONDER_EFFECTIVE_MARKER, SHORT_INFO_RESPONDER_FEATURE_ID,
};
pub use short_info_query_wire::{
    ParsedShortInfoQuery, ShortInfoQuery, ShortInfoQueryEncodeError, ShortInfoQueryParseError,
    ShortInfoQueryValueError, ShortInfoQueryWire, ShortInfoQueryWireLimitError,
    ShortInfoQueryWireLimits,
};
pub use short_info_response_envelope::{
    ParsedShortInfoResponseEnvelope, ShortInfoResponseEnvelope,
    ShortInfoResponseEnvelopeEncodeError, ShortInfoResponseEnvelopeLimitError,
    ShortInfoResponseEnvelopeLimits, ShortInfoResponseEnvelopeParseError,
};
pub use stream_definition::StreamDefinition;
pub use stream_descriptor::{
    ChannelFormat, InvalidRegularSampleRate, NominalSampleRate, NominalSampleRateError,
    RegularSampleRate, StreamDescriptor, StreamDescriptorBound, StreamDescriptorError,
    StreamDescriptorLimits, StreamDescriptorTextRole,
};
pub use stream_handshake::{
    run_stream_inlet_handshake, run_stream_outlet_handshake, StreamHandshakeActivation,
    StreamHandshakeActivationError, StreamHandshakeError, StreamHandshakeIdentity,
    StreamHandshakeIdentityError, StreamHandshakeIdentityRole, StreamHandshakeLimitError,
    StreamHandshakeLimits, StreamInletHandshake, StreamOutletHandshake,
    STREAM_HANDSHAKE_EFFECTIVE_MARKER, STREAM_HANDSHAKE_FEATURE_ID,
};
pub use stream_info_description_xml::{StreamInfoDescriptionXml, StreamInfoDescriptionXmlError};
pub use stream_info_implementation_version_provider::{
    StreamInfoImplementationVersionAcquisition, StreamInfoImplementationVersionAcquisitionError,
    StreamInfoImplementationVersionEvidenceError, StreamInfoImplementationVersionEvidenceLimit,
    StreamInfoImplementationVersionProvider, StreamInfoImplementationVersionProviderOutput,
    StreamInfoImplementationVersionWitness,
};
pub use stream_info_observed_document::{
    StreamInfoObservedDocument, StreamInfoObservedDocumentError, StreamInfoObservedDocumentLimit,
};
pub use stream_info_observed_document_admission::{
    StreamInfoObservedAdmissionError, StreamInfoObservedAdmissionLimits, StreamInfoObservedFields,
};
pub use stream_info_observed_document_parser::{
    ParsedStreamInfoObservedDocument, StreamInfoObservedDocumentParseError,
    StreamInfoObservedDocumentParseLimit,
};
pub use stream_info_ordered_xml::{StreamInfoOrderedXml, StreamInfoOrderedXmlError};
pub use stream_info_runtime_provider::{
    StreamInfoRuntimeAcquisition, StreamInfoRuntimeAcquisitionError,
    StreamInfoRuntimeEvidenceError, StreamInfoRuntimeEvidenceLimit, StreamInfoRuntimeProvider,
    StreamInfoRuntimeProviderOutput, StreamInfoRuntimeValues, StreamInfoRuntimeWitness,
};
pub use stream_info_static_fields::{StreamInfoStaticFieldRole, StreamInfoStaticFields};
pub use stream_info_static_numeric_spellings::{
    StreamInfoStaticNumericSpellingError, StreamInfoStaticNumericSpellings,
};
pub use stream_info_static_xml::{
    StreamInfoStaticXml, StreamInfoStaticXmlError, StreamInfoStaticXmlLimits,
};
pub use stream_info_three_owner_observed_document::{
    StreamInfoThreeOwnerObservedDocument, StreamInfoThreeOwnerObservedDocumentError,
    StreamInfoThreeOwnerObservedDocumentLimits,
};
pub use stream_info_three_owner_snapshot::{
    StreamInfoThreeOwnerEvidence, StreamInfoThreeOwnerSnapshot,
};
pub use stream_info_transport_provider::{
    StreamInfoTransportAcquisition, StreamInfoTransportAcquisitionError,
    StreamInfoTransportEvidenceError, StreamInfoTransportEvidenceLimit,
    StreamInfoTransportProvider, StreamInfoTransportProviderOutput, StreamInfoTransportValues,
    StreamInfoTransportWitness,
};
pub use stream_info_volatile_fields::{
    StreamInfoVolatileFieldClass, StreamInfoVolatileFieldError, StreamInfoVolatileFieldInput,
    StreamInfoVolatileFieldLimits, StreamInfoVolatileFieldRole, StreamInfoVolatileFields,
};
pub use stream_info_volatile_snapshot::{
    StreamInfoVolatileProviderSnapshot, StreamInfoVolatileProviderSnapshotError,
    StreamInfoVolatileProviderSnapshotInput, StreamInfoVolatileProviderValue,
};
pub use stream_info_volatile_xml::{
    StreamInfoVolatileXml, StreamInfoVolatileXmlError, StreamInfoVolatileXmlLimits,
};
pub use string_sample_runtime::{
    run_string_sample_inlet, run_string_sample_outlet, StringSampleActivation,
    StringSampleActivationError, StringSampleError, StringSampleLimitError, StringSampleLimits,
    StringSampleRecord, STRING_SAMPLE_EFFECTIVE_MARKER, STRING_SAMPLE_FEATURE_ID,
};
pub use timestamped::{
    ChunkBound, ChunkError, ChunkLimits, DerivedTimestamp, DerivedTimestampKind,
    NonFiniteTimestamp, RawSourceTimestamp, TimestampError, TimestampRole, TimestampedChunk,
    TimestampedSample,
};
pub use timestamped_descriptor_chunk::{
    BoundTimestampedDescriptorChunk, TimestampedDescriptorChunkError,
    TimestampedDescriptorChunkInput,
};
pub use timestamped_descriptor_sample::{
    BoundTimestampedDescriptorSample, TimestampedDescriptorSampleInput,
};
pub use timestamped_float32_sample_runtime::{
    run_timestamped_float32_inlet, run_timestamped_float32_outlet,
    TimestampedFloat32SampleActivation, TimestampedFloat32SampleActivationError,
    TimestampedFloat32SampleError, TimestampedFloat32SampleLimitError,
    TimestampedFloat32SampleLimits, TIMESTAMPED_FLOAT32_SAMPLE_EFFECTIVE_MARKER,
    TIMESTAMPED_FLOAT32_SAMPLE_FEATURE_ID,
};
pub use timestamped_float32_session_runtime::{
    TimestampedDouble64AcceptedOutletSession, TimestampedDouble64ConnectedInletSession,
    TimestampedDouble64InletSession, TimestampedDouble64InletSessionReport,
    TimestampedDouble64OutletSession, TimestampedDouble64OutletSessionReport,
    TimestampedDouble64SessionError, TimestampedDouble64SessionIncomplete,
    TimestampedDouble64SessionIoLimitError, TimestampedDouble64SessionIoLimits,
    TimestampedDouble64SessionLimitError, TimestampedDouble64SessionLimits,
    TimestampedDouble64SessionPreflightError, TimestampedDouble64SessionTransferError,
    TimestampedFloat32AcceptedOutletSession, TimestampedFloat32ConnectedInletSession,
    TimestampedFloat32InletSession, TimestampedFloat32InletSessionReport,
    TimestampedFloat32OutletSession, TimestampedFloat32OutletSessionReport,
    TimestampedFloat32SessionCompletion, TimestampedFloat32SessionError,
    TimestampedFloat32SessionIncomplete, TimestampedFloat32SessionLimitError,
    TimestampedFloat32SessionLimits, TimestampedFloat32SessionPreflightError,
    TimestampedFloat32SessionRole, TimestampedFloat32SessionTransferError,
    TimestampedInt16AcceptedOutletSession, TimestampedInt16ConnectedInletSession,
    TimestampedInt16InletSession, TimestampedInt16InletSessionReport,
    TimestampedInt16OutletSession, TimestampedInt16OutletSessionReport,
    TimestampedInt16SessionError, TimestampedInt16SessionIncomplete,
    TimestampedInt16SessionIoLimitError, TimestampedInt16SessionIoLimits,
    TimestampedInt16SessionLimitError, TimestampedInt16SessionLimits,
    TimestampedInt16SessionPreflightError, TimestampedInt16SessionTransferError,
    TimestampedInt32AcceptedOutletSession, TimestampedInt32ConnectedInletSession,
    TimestampedInt32InletSession, TimestampedInt32InletSessionReport,
    TimestampedInt32OutletSession, TimestampedInt32OutletSessionReport,
    TimestampedInt32SessionError, TimestampedInt32SessionIncomplete,
    TimestampedInt32SessionIoLimitError, TimestampedInt32SessionIoLimits,
    TimestampedInt32SessionLimitError, TimestampedInt32SessionLimits,
    TimestampedInt32SessionPreflightError, TimestampedInt32SessionTransferError,
    TimestampedInt64AcceptedOutletSession, TimestampedInt64ConnectedInletSession,
    TimestampedInt64InletSession, TimestampedInt64InletSessionReport,
    TimestampedInt64OutletSession, TimestampedInt64OutletSessionReport,
    TimestampedInt64SessionError, TimestampedInt64SessionIncomplete,
    TimestampedInt64SessionIoLimitError, TimestampedInt64SessionIoLimits,
    TimestampedInt64SessionLimitError, TimestampedInt64SessionLimits,
    TimestampedInt64SessionPreflightError, TimestampedInt64SessionTransferError,
    TimestampedInt8AcceptedOutletSession, TimestampedInt8ConnectedInletSession,
    TimestampedInt8InletSession, TimestampedInt8InletSessionReport, TimestampedInt8OutletSession,
    TimestampedInt8OutletSessionReport, TimestampedInt8SessionError,
    TimestampedInt8SessionIncomplete, TimestampedInt8SessionIoLimitError,
    TimestampedInt8SessionIoLimits, TimestampedInt8SessionLimitError, TimestampedInt8SessionLimits,
    TimestampedInt8SessionPreflightError, TimestampedInt8SessionTransferError,
    TimestampedStringAcceptedOutletSession, TimestampedStringConnectedInletSession,
    TimestampedStringInletSession, TimestampedStringInletSessionReport,
    TimestampedStringOutletSession, TimestampedStringOutletSessionReport,
    TimestampedStringSessionCompletion, TimestampedStringSessionError,
    TimestampedStringSessionIncomplete, TimestampedStringSessionLimitError,
    TimestampedStringSessionLimits, TimestampedStringSessionPreflightError,
    TimestampedStringSessionRole, TimestampedStringSessionTransferError,
};
pub use timestamped_float32_two_record_chunk_runtime::{
    run_timestamped_float32_two_record_chunk_inlet,
    run_timestamped_float32_two_record_chunk_outlet,
    TimestampedFloat32TwoRecordChunkAcceptedOutletSession,
    TimestampedFloat32TwoRecordChunkConnectedInletSession, TimestampedFloat32TwoRecordChunkError,
    TimestampedFloat32TwoRecordChunkInletSession,
    TimestampedFloat32TwoRecordChunkInletSessionReport, TimestampedFloat32TwoRecordChunkLimitError,
    TimestampedFloat32TwoRecordChunkLimits, TimestampedFloat32TwoRecordChunkOutletSession,
    TimestampedFloat32TwoRecordChunkOutletSessionReport,
};
pub use typed_short_info_response_observation::{
    TypedShortInfoResponseObservation, TypedShortInfoResponseObservationError,
};
pub use typed_udp_discovery_double64_session_connection::{
    connect_selected_typed_udp_discovery_double64_session_inlet,
    run_selected_typed_udp_discovery_double64_session_inlet,
    TypedUdpDiscoveryDouble64SessionConnectionError,
};
pub use typed_udp_discovery_endpoint::{
    propose_typed_udp_discovery_ipv4_service_endpoint, TypedUdpDiscoveryEndpointError,
};
pub use typed_udp_discovery_float32::{
    run_selected_typed_udp_discovery_float32_inlet, TypedUdpDiscoveryFloat32Error,
};
pub use typed_udp_discovery_float32_clock_correction_queue::{
    run_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue,
    TypedUdpDiscoveryFloat32ClockCorrectionQueueError,
};
pub use typed_udp_discovery_float32_queue::{
    run_selected_typed_udp_discovery_float32_inlet_into_queue, TypedUdpDiscoveryFloat32QueueError,
};
pub use typed_udp_discovery_float32_recovery_clock_correction_queue::{
    run_recovering_selected_typed_udp_discovery_float32_inlet_with_clock_correction_into_queue,
    TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueError,
    TypedUdpDiscoveryFloat32RecoveryClockCorrectionQueueOutcome,
};
pub use typed_udp_discovery_float32_recovery_queue::{
    run_recovering_selected_typed_udp_discovery_float32_inlet_into_queue,
    TypedUdpDiscoveryFloat32RecoveryQueueError, TypedUdpDiscoveryFloat32RecoveryQueueOutcome,
};
pub use typed_udp_discovery_float32_session_batch_pipeline::{
    run_selected_typed_udp_discovery_float32_inlet_session_batch_recovery_clock_queue,
    SelectedTypedUdpDiscoveryFloat32SessionBatchError,
    SelectedTypedUdpDiscoveryFloat32SessionBatchErrorKind,
    SelectedTypedUdpDiscoveryFloat32SessionBatchOutcome,
};
pub use typed_udp_discovery_float32_session_connection::{
    connect_selected_typed_udp_discovery_float32_session_inlet,
    resolve_selected_typed_udp_discovery_float32_session_inlet,
    run_selected_typed_udp_discovery_float32_session_inlet,
    ResolvedTypedUdpDiscoveryFloat32Session, TypedUdpDiscoveryFloat32SessionConnectionError,
};
pub use typed_udp_discovery_handshake::{
    run_selected_typed_udp_discovery_inlet_handshake, TypedUdpDiscoveryHandshakeError,
};
pub use typed_udp_discovery_integer_session_connection::{
    connect_selected_typed_udp_discovery_int16_session_inlet,
    connect_selected_typed_udp_discovery_int32_session_inlet,
    connect_selected_typed_udp_discovery_int64_session_inlet,
    connect_selected_typed_udp_discovery_int8_session_inlet,
    run_selected_typed_udp_discovery_int16_session_inlet,
    run_selected_typed_udp_discovery_int32_session_inlet,
    run_selected_typed_udp_discovery_int64_session_inlet,
    run_selected_typed_udp_discovery_int8_session_inlet,
    TypedUdpDiscoveryInt16SessionConnectionError, TypedUdpDiscoveryInt32SessionConnectionError,
    TypedUdpDiscoveryInt64SessionConnectionError, TypedUdpDiscoveryInt8SessionConnectionError,
};
pub use typed_udp_discovery_response::{TypedUdpDiscoveryResponse, TypedUdpDiscoveryResponseError};
pub use typed_udp_discovery_run::{
    run_typed_udp_discovery, TypedUdpDiscoveryRun, TypedUdpDiscoveryRunError,
};
pub use typed_udp_discovery_selection::{
    suggest_typed_udp_discovery_response, TypedUdpDiscoverySelectionError,
};
pub use typed_udp_discovery_string_session_connection::{
    connect_selected_typed_udp_discovery_string_session_inlet,
    run_selected_typed_udp_discovery_string_session_inlet,
    TypedUdpDiscoveryStringSessionConnectionError,
};
pub use udp_discovery::{
    run_udp_discovery, UdpDiscoveryActivation, UdpDiscoveryActivationError, UdpDiscoveryConfig,
    UdpDiscoveryError, UdpDiscoveryLimitError, UdpDiscoveryLimits, UdpDiscoveryResponse,
    UdpDiscoveryRun, UdpDiscoveryTermination, UDP_DISCOVERY_EFFECTIVE_MARKER,
    UDP_DISCOVERY_FEATURE_ID,
};
pub use xml_character_data::{XmlCharacterData, XmlCharacterDataError, XmlCharacterDataLimit};
pub use xml_element_serialization::{
    XmlElementSerialization, XmlElementSerializationError, XmlElementSerializationLimit,
};
pub use xml_element_tree::{
    XmlElementNodeInput, XmlElementNodeValue, XmlElementTree, XmlElementTreeBound,
    XmlElementTreeError, XmlElementTreeLimits,
};
pub use xml_leaf_element::XmlLeafElement;
pub use xml_value::{
    XmlElementName, XmlNameError, XmlNameLimit, XmlText, XmlTextError, XmlTextLimit,
};

/// The implementation state exposed by the crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImplementationStatus {
    /// Bounded local contracts plus one explicitly activated UDP discovery call exist.
    FiniteSampleRecoveryRuntime,
}

/// A stable declaration of one side of the repository ownership boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OwnershipDeclaration {
    /// Capabilities and evidence owned by Rusty LSL.
    pub owns: &'static [&'static str],
    /// Authorities and adapters explicitly owned elsewhere.
    pub does_not_own: &'static [&'static str],
}

/// Returns the current implementation status.
#[must_use]
pub const fn implementation_status() -> ImplementationStatus {
    ImplementationStatus::FiniteSampleRecoveryRuntime
}

/// Returns the repository's current ownership declaration.
#[must_use]
pub const fn ownership_declaration() -> OwnershipDeclaration {
    OwnershipDeclaration {
        owns: &[
            "bounded local metadata construction",
            "bounded local sample-shape construction",
            "bounded local timestamped-sample and chunk construction",
            "bounded local core stream-descriptor construction",
            "bounded local flat metadata-tree construction",
            "bounded local descriptor/sample binding",
            "bounded local timestamped descriptor/sample composition",
            "bounded local non-empty timestamped descriptor/chunk composition",
            "bounded local stream-definition composition",
            "borrowed static stream-info semantic projection",
            "bounded static stream-info numeric lexical projection",
            "bounded local XML legal-text and element-name values",
            "bounded local XML character-data representation",
            "bounded local XML leaf-element composition",
            "bounded local XML container/leaf hierarchy",
            "bounded local metadata-to-XML-element-tree projection",
            "bounded local XML element-tree serialization",
            "bounded volatile stream-info accepted data",
            "bounded volatile stream-info XML element composition",
            "bounded local short-info query byte-shape contract",
            "bounded local short-info response-envelope contract",
            "documented discovery-destination data contract",
            "inert documented discovery-query proposal composition",
            "finite raw clock-exchange formula contract",
            "bounded minimum-RTT selection contract",
            "explicit finite clock-offset application contract",
            "bounded caller-configured UDP discovery runtime",
            "bounded caller-configured TCP stream-handshake runtime",
            "bounded one-record timestamped float32 sample runtime",
            "bounded integrated clock-correction runtime",
            "bounded caller-owned sample queue runtime",
            "finite caller-invoked sample recovery runtime",
            "future backend-neutral Rust LSL API",
            "compatibility evidence",
            "typed observations and proposals for downstream adapters",
        ],
        does_not_own: &[
            "Manifold authority",
            "Morphospace-native sample transport",
            "topology, identity, permission, or platform lifecycle",
            "Quest or Hostess adapters",
            "commands derived from inbound samples",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{implementation_status, ownership_declaration, ImplementationStatus};
    use crate::caller_requested_float32_advisory_report_package::{
        CallerRequestedFloat32AdvisoryReportPackage,
        CallerRequestedFloat32AdvisoryReportPackageBounds,
        CallerRequestedFloat32AdvisoryReportPackageFact,
        CallerRequestedFloat32AdvisoryReportPackageOwner,
    };
    use crate::caller_requested_float32_advisory_report_package_history::{
        CallerRequestedFloat32AdvisoryReportPackageHistory,
        CallerRequestedFloat32AdvisoryReportPackageHistoryBounds,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence_history::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence_snapshot::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner,
    };
    use crate::caller_requested_float32_comparative_advisory_evidence_snapshot_history::{
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory,
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds,
    };
    use crate::caller_requested_float32_report_advisory_evidence::{
        CallerRequestedFloat32ReportAdvisoryEvidence,
        CallerRequestedFloat32ReportAdvisoryEvidenceBounds,
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner,
    };
    use crate::caller_requested_float32_report_advisory_evidence_history::CallerRequestedFloat32ReportAdvisoryEvidenceHistory;
    use crate::exact_sequence_loss_health::ExactSequenceLossHealth;
    use crate::float32_session_report_requested_post_processing::Float32SessionReportRequestedPostProcessing;
    use crate::morphospace_float32_advisory_report_package_delta_history::{
        MorphospaceFloat32AdvisoryReportPackageDeltaHistory,
        MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds,
    };
    use crate::morphospace_float32_advisory_report_package_delta_proposal::{
        MorphospaceFloat32AdvisoryReportPackageCount,
        MorphospaceFloat32AdvisoryReportPackageDeltaBounds,
        MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner,
        MorphospaceFloat32AdvisoryReportPackageRelation,
    };
    use crate::morphospace_float32_comparative_advisory_evidence_delta_history::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds,
    };
    use crate::morphospace_float32_comparative_advisory_evidence_delta_proposal::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceCount,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds,
        MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner,
    };
    use crate::morphospace_float32_comparative_advisory_evidence_snapshot_delta_proposal::{
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds,
        MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner,
    };
    use crate::morphospace_float32_report_advisory_snapshot::{
        MorphospaceFloat32ReportAdvisorySnapshot, MorphospaceFloat32ReportAdvisorySnapshotBounds,
        MorphospaceFloat32ReportAdvisorySnapshotOwner,
    };
    use crate::morphospace_float32_report_advisory_snapshot_history::MorphospaceFloat32ReportAdvisorySnapshotHistory;
    use crate::morphospace_float32_report_observation_history::MorphospaceFloat32ReportObservationHistory;
    use crate::morphospace_float32_report_window_delta_history::MorphospaceFloat32ReportWindowDeltaHistory;
    use crate::morphospace_float32_report_window_stability_proposal::{
        MorphospaceFloat32ReportWindowStabilityBounds,
        MorphospaceFloat32ReportWindowStabilityProposalOwner,
    };
    use crate::morphospace_float32_retained_advisory_summary::{
        MorphospaceFloat32RetainedAdvisorySummary, MorphospaceFloat32RetainedAdvisorySummaryBounds,
        MorphospaceFloat32RetainedAdvisorySummaryOwner,
    };
    use crate::morphospace_float32_retained_advisory_summary_history::MorphospaceFloat32RetainedAdvisorySummaryHistory;
    use crate::requested_timestamp_post_processing::{
        RequestedTimestampPostProcessing, RequestedTimestampPostProcessingConfig,
        RequestedTimestampPostProcessor,
    };
    use crate::{RawSourceTimestamp, Sample, SampleLimits, TimestampedSample};

    fn p41_snapshot() -> MorphospaceFloat32ReportAdvisorySnapshot {
        let stability = MorphospaceFloat32ReportWindowStabilityProposalOwner::new(
            MorphospaceFloat32ReportWindowStabilityBounds::new(1, 1, 1, 1, 0, 0.0).unwrap(),
        )
        .propose(MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap())
        .unwrap();
        MorphospaceFloat32ReportAdvisorySnapshotOwner::new(
            MorphospaceFloat32ReportAdvisorySnapshotBounds::new(1, 1, 1, 1, 1).unwrap(),
        )
        .snapshot(
            MorphospaceFloat32ReportObservationHistory::new(1, 1).unwrap(),
            MorphospaceFloat32ReportWindowDeltaHistory::new(1, 1).unwrap(),
            stability,
        )
        .unwrap()
    }

    fn p34_evidence(sequence: u64, value: f32) -> CallerRequestedFloat32ReportAdvisoryEvidence {
        let mut processor = Float32SessionReportRequestedPostProcessing::new(
            RequestedTimestampPostProcessor::new(RequestedTimestampPostProcessing::Monotonic(
                RequestedTimestampPostProcessingConfig::new(2, 1.0, 10.0).unwrap(),
            ))
            .unwrap(),
            ExactSequenceLossHealth::new(4),
        );
        let report = processor
            .process_record(
                sequence,
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(1).unwrap(), 1, vec![value]).unwrap(),
                    RawSourceTimestamp::new(3.0).unwrap(),
                    None,
                ),
            )
            .unwrap();
        CallerRequestedFloat32ReportAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ReportAdvisoryEvidenceBounds::new(1, 1).unwrap(),
        )
        .compose(report, p41_snapshot())
        .unwrap()
    }

    fn p42_summary(sequence: u64, value: f32) -> MorphospaceFloat32RetainedAdvisorySummary {
        let retained = p34_evidence(sequence, value);
        let snapshots = MorphospaceFloat32ReportAdvisorySnapshotHistory::new(1)
            .unwrap()
            .append(p41_snapshot())
            .unwrap();
        MorphospaceFloat32RetainedAdvisorySummaryOwner::new(
            MorphospaceFloat32RetainedAdvisorySummaryBounds::new(1, 1, 2).unwrap(),
        )
        .summarize(retained, snapshots)
        .unwrap()
    }

    fn summary_pointer(summary: &MorphospaceFloat32RetainedAdvisorySummary) -> *const f32 {
        summary
            .retained()
            .report()
            .sample()
            .sample()
            .values()
            .as_ptr()
    }

    fn package_pointers(
        history: &CallerRequestedFloat32ReportAdvisoryEvidenceHistory,
        summary: &MorphospaceFloat32RetainedAdvisorySummary,
    ) -> Vec<*const f32> {
        history
            .values()
            .iter()
            .map(|value| value.report().sample().sample().values().as_ptr())
            .chain(std::iter::once(summary_pointer(summary)))
            .collect()
    }

    fn p43_package(
        history_value_count: usize,
        sequence: u64,
    ) -> CallerRequestedFloat32AdvisoryReportPackage {
        let mut history =
            CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(history_value_count.max(1))
                .unwrap();
        for offset in 0..history_value_count {
            history = history
                .append(p34_evidence(
                    sequence + offset as u64,
                    sequence as f32 + offset as f32,
                ))
                .unwrap();
        }
        CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(
                history_value_count.max(1),
                history_value_count.max(1),
                2,
                history_value_count * 2 + 2,
            )
            .unwrap(),
        )
        .package(
            history,
            p42_summary(sequence + history_value_count as u64, 99.0),
        )
        .unwrap()
    }

    fn nested_package_pointers(
        package: &CallerRequestedFloat32AdvisoryReportPackage,
    ) -> Vec<*const f32> {
        package_pointers(package.history(), package.summary())
    }

    fn delta_proposal_pointers(
        proposal: &crate::morphospace_float32_advisory_report_package_delta_proposal::MorphospaceFloat32AdvisoryReportPackageDeltaProposal,
    ) -> (Vec<*const f32>, Vec<*const f32>) {
        (
            nested_package_pointers(proposal.earlier()),
            nested_package_pointers(proposal.later()),
        )
    }

    fn p47_snapshot(
        history_evidence_count: usize,
        base: u64,
    ) -> CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot {
        let comparative = |offset| {
            let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
                MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
            )
            .propose(
                p43_package(1, base + offset),
                p43_package(2, base + offset + 10),
            )
            .unwrap();
            CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
            )
            .compose(
                p43_package(1, base + offset + 20),
                p43_package(2, base + offset + 30),
                proposal,
            )
            .unwrap()
        };
        let mut history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(
                history_evidence_count,
                history_evidence_count * 8,
            )
            .unwrap(),
        );
        for index in 0..history_evidence_count {
            history = history.append(comparative(index as u64 * 100)).unwrap();
        }
        let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(comparative(500), comparative(600))
        .unwrap();
        CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(
                history_evidence_count,
                4,
                history_evidence_count + 4,
            )
            .unwrap(),
        )
        .snapshot(history, delta)
        .unwrap()
    }

    #[test]
    fn status_names_only_the_implemented_local_contracts() {
        assert_eq!(
            implementation_status(),
            ImplementationStatus::FiniteSampleRecoveryRuntime
        );
    }

    #[test]
    fn authority_remains_outside_the_repository() {
        let declaration = ownership_declaration();
        assert!(declaration.does_not_own.contains(&"Manifold authority"));
        assert!(declaration
            .does_not_own
            .contains(&"commands derived from inbound samples"));
        assert!(declaration
            .owns
            .contains(&"bounded local short-info response-envelope contract"));
        assert!(declaration
            .owns
            .contains(&"documented discovery-destination data contract"));
        assert!(declaration
            .owns
            .contains(&"inert documented discovery-query proposal composition"));
        assert!(declaration
            .owns
            .contains(&"finite raw clock-exchange formula contract"));
        assert!(declaration
            .owns
            .contains(&"bounded minimum-RTT selection contract"));
        assert!(declaration
            .owns
            .contains(&"bounded caller-configured UDP discovery runtime"));
        assert!(declaration
            .owns
            .contains(&"explicit finite clock-offset application contract"));
        assert!(declaration
            .owns
            .contains(&"bounded caller-configured TCP stream-handshake runtime"));
        assert!(declaration
            .owns
            .contains(&"bounded one-record timestamped float32 sample runtime"));
        assert!(declaration
            .owns
            .contains(&"bounded integrated clock-correction runtime"));
        assert!(declaration
            .owns
            .contains(&"bounded caller-owned sample queue runtime"));
        assert!(declaration
            .owns
            .contains(&"finite caller-invoked sample recovery runtime"));
    }

    #[test]
    fn actual_p34_p40_p41_p42_p43_owners_compose_without_identity_or_order_loss() {
        let first = p42_summary(40, 1.0);
        let second = p42_summary(41, 2.0);
        let retained_pointers = [summary_pointer(&first), summary_pointer(&second)];
        assert_ne!(retained_pointers[0], retained_pointers[1]);
        let retained_history = MorphospaceFloat32RetainedAdvisorySummaryHistory::new(2)
            .unwrap()
            .append(first)
            .unwrap()
            .append(second)
            .unwrap();
        assert_eq!(
            retained_history
                .summaries()
                .iter()
                .map(summary_pointer)
                .collect::<Vec<_>>(),
            retained_pointers
        );

        let evidence_history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(2)
            .unwrap()
            .append(p34_evidence(42, 3.0))
            .unwrap()
            .append(p34_evidence(43, 4.0))
            .unwrap();
        let package_summary = p42_summary(44, 5.0);
        let package_pointers_before = package_pointers(&evidence_history, &package_summary);
        assert!(package_pointers_before
            .iter()
            .all(|pointer| !retained_pointers.contains(pointer)));
        let package = CallerRequestedFloat32AdvisoryReportPackageOwner::new(
            CallerRequestedFloat32AdvisoryReportPackageBounds::new(2, 2, 2, 6).unwrap(),
        )
        .package(evidence_history, package_summary)
        .unwrap();
        assert_eq!(
            package_pointers(package.history(), package.summary()),
            package_pointers_before
        );
        assert_eq!(package.totals().history_value_count(), 2);
        assert_eq!(package.totals().history_evidence_count(), 2);
        assert_eq!(package.totals().summary_fact_count(), 2);
        assert_eq!(package.totals().package_fact_count(), 6);
        assert!(matches!(
            package.facts()[0],
            CallerRequestedFloat32AdvisoryReportPackageFact::HistoryValue {
                history_index: 0,
                ..
            }
        ));
        assert!(matches!(
            package.facts()[1],
            CallerRequestedFloat32AdvisoryReportPackageFact::HistoryEvidence {
                history_index: 0,
                evidence_index: 0,
                ..
            }
        ));
        assert!(matches!(
            package.facts()[2],
            CallerRequestedFloat32AdvisoryReportPackageFact::HistoryValue {
                history_index: 1,
                ..
            }
        ));
        assert!(matches!(
            package.facts()[3],
            CallerRequestedFloat32AdvisoryReportPackageFact::HistoryEvidence {
                history_index: 1,
                evidence_index: 0,
                ..
            }
        ));
        assert!(package.facts()[4..]
            .iter()
            .enumerate()
            .all(|(index, fact)| {
                matches!(
                    fact,
                    CallerRequestedFloat32AdvisoryReportPackageFact::RetainedSummaryFact {
                        summary_index,
                        ..
                    } if *summary_index == index as u64
                )
            }));
        let (evidence_history, package_summary) = package.into_parts();
        assert_eq!(
            package_pointers(&evidence_history, &package_summary),
            package_pointers_before
        );
        assert_eq!(
            retained_history
                .into_summaries()
                .iter()
                .map(summary_pointer)
                .collect::<Vec<_>>(),
            retained_pointers
        );
    }

    #[test]
    fn p43_history_and_package_failures_are_transactional_across_real_upstream_owners() {
        let kept = p42_summary(50, 6.0);
        let candidate = p42_summary(51, 7.0);
        let kept_pointer = summary_pointer(&kept);
        let candidate_pointer = summary_pointer(&candidate);
        let history = MorphospaceFloat32RetainedAdvisorySummaryHistory::new(1)
            .unwrap()
            .append(kept)
            .unwrap();
        let totals_before = history.totals();
        let (history, candidate) = history.append(candidate).unwrap_err().into_parts();
        assert_eq!(history.totals(), totals_before);
        assert_eq!(history.summaries().len(), 1);
        assert_eq!(summary_pointer(&history.summaries()[0]), kept_pointer);
        assert_eq!(summary_pointer(&candidate), candidate_pointer);

        for bounds in [(1, 2, 2, 6), (2, 1, 2, 6), (2, 2, 1, 6), (2, 2, 2, 5)] {
            let evidence_history = CallerRequestedFloat32ReportAdvisoryEvidenceHistory::new(2)
                .unwrap()
                .append(p34_evidence(52, 8.0))
                .unwrap()
                .append(p34_evidence(53, 9.0))
                .unwrap();
            let summary = p42_summary(54, 10.0);
            let pointers_before = package_pointers(&evidence_history, &summary);
            let history_totals_before = evidence_history.totals();
            let summary_totals_before = summary.totals();
            let error = CallerRequestedFloat32AdvisoryReportPackageOwner::new(
                CallerRequestedFloat32AdvisoryReportPackageBounds::new(
                    bounds.0, bounds.1, bounds.2, bounds.3,
                )
                .unwrap(),
            )
            .package(evidence_history, summary)
            .unwrap_err();
            let (evidence_history, summary) = error.into_parts();
            assert_eq!(
                package_pointers(&evidence_history, &summary),
                pointers_before
            );
            assert_eq!(evidence_history.totals(), history_totals_before);
            assert_eq!(summary.totals(), summary_totals_before);
            assert_eq!(evidence_history.values().len(), 2);
        }
    }

    #[test]
    fn p44_package_history_and_two_package_delta_compose_in_exact_order_and_identity() {
        let earlier = p43_package(1, 60);
        let later = p43_package(2, 70);
        let earlier_pointers = nested_package_pointers(&earlier);
        let later_pointers = nested_package_pointers(&later);
        let history = CallerRequestedFloat32AdvisoryReportPackageHistory::new(
            CallerRequestedFloat32AdvisoryReportPackageHistoryBounds::new(2, 3, 3, 4, 10).unwrap(),
        )
        .append(earlier)
        .unwrap()
        .append(later)
        .unwrap();
        assert_eq!(history.totals().package_count(), 2);
        assert_eq!(
            nested_package_pointers(&history.packages()[0]),
            earlier_pointers
        );
        assert_eq!(
            nested_package_pointers(&history.packages()[1]),
            later_pointers
        );

        let mut packages = history.into_packages();
        let later = packages.pop().unwrap();
        let earlier = packages.pop().unwrap();
        assert!(packages.is_empty());
        let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(earlier, later)
        .unwrap();
        assert_eq!(proposal.relation_count(), 4);
        let expected = [
            (
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryValues,
                1,
                2,
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 },
            ),
            (
                MorphospaceFloat32AdvisoryReportPackageCount::HistoryEvidence,
                1,
                2,
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 },
            ),
            (
                MorphospaceFloat32AdvisoryReportPackageCount::SummaryFacts,
                2,
                2,
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal,
            ),
            (
                MorphospaceFloat32AdvisoryReportPackageCount::PackageFacts,
                4,
                6,
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 2 },
            ),
        ];
        assert!(proposal
            .facts()
            .iter()
            .zip(expected)
            .all(|(fact, expected)| {
                (fact.count(), fact.earlier(), fact.later(), fact.relation()) == expected
            }));
        let (earlier, later) = proposal.into_packages();
        assert_eq!(nested_package_pointers(&earlier), earlier_pointers);
        assert_eq!(nested_package_pointers(&later), later_pointers);
    }

    #[test]
    fn p44_error_rollbacks_return_unchanged_history_and_packages() {
        let earlier = p43_package(1, 80);
        let later = p43_package(2, 90);
        let earlier_pointers = nested_package_pointers(&earlier);
        let later_pointers = nested_package_pointers(&later);
        let history = CallerRequestedFloat32AdvisoryReportPackageHistory::new(
            CallerRequestedFloat32AdvisoryReportPackageHistoryBounds::new(1, 3, 3, 4, 10).unwrap(),
        )
        .append(earlier)
        .unwrap();
        let totals = history.totals();
        let (history, later) = history.append(later).unwrap_err().into_parts();
        assert_eq!(history.totals(), totals);
        assert_eq!(
            nested_package_pointers(&history.packages()[0]),
            earlier_pointers
        );
        assert_eq!(nested_package_pointers(&later), later_pointers);

        let earlier = history.into_packages().pop().unwrap();
        let error = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(3).unwrap(),
        )
        .propose(earlier, later)
        .unwrap_err();
        let (earlier, later) = error.into_packages();
        assert_eq!(nested_package_pointers(&earlier), earlier_pointers);
        assert_eq!(nested_package_pointers(&later), later_pointers);
    }

    #[test]
    fn p45_delta_history_to_comparative_evidence_preserves_nested_identity_and_order() {
        let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(p43_package(1, 100), p43_package(2, 110))
        .unwrap();
        let proposal_pointers = delta_proposal_pointers(&proposal);
        let proposal_facts = proposal.facts().as_ptr();
        let history = MorphospaceFloat32AdvisoryReportPackageDeltaHistory::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds::new(1, 2, 4).unwrap(),
        )
        .append(proposal)
        .unwrap();
        assert_eq!(history.totals().proposal_count(), 1);
        assert_eq!(history.totals().package_count(), 2);
        assert_eq!(history.totals().fact_count(), 4);
        assert_eq!(
            delta_proposal_pointers(&history.proposals()[0]),
            proposal_pointers
        );
        assert_eq!(history.proposals()[0].facts().as_ptr(), proposal_facts);

        let proposal = history.into_proposals().pop().unwrap();
        let earlier = p43_package(1, 120);
        let later = p43_package(2, 130);
        let earlier_pointers = nested_package_pointers(&earlier);
        let later_pointers = nested_package_pointers(&later);
        let evidence = CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
        )
        .compose(earlier, later, proposal)
        .unwrap();
        assert_eq!(evidence.fact_count(), 8);
        assert_eq!(
            nested_package_pointers(evidence.earlier()),
            earlier_pointers
        );
        assert_eq!(nested_package_pointers(evidence.later()), later_pointers);
        assert_eq!(
            delta_proposal_pointers(evidence.delta_proposal()),
            proposal_pointers
        );
        assert_eq!(evidence.delta_proposal().facts().as_ptr(), proposal_facts);
        for (proposal_fact, pair) in evidence
            .delta_proposal()
            .facts()
            .iter()
            .zip(evidence.facts().chunks_exact(2))
        {
            assert_eq!(pair[0].count(), proposal_fact.count());
            assert_eq!(pair[1].count(), proposal_fact.count());
            assert_eq!(pair[0].proposal_value(), proposal_fact.earlier());
            assert_eq!(pair[1].proposal_value(), proposal_fact.later());
        }

        let (earlier, later, proposal) = evidence.into_parts();
        assert_eq!(nested_package_pointers(&earlier), earlier_pointers);
        assert_eq!(nested_package_pointers(&later), later_pointers);
        assert_eq!(delta_proposal_pointers(&proposal), proposal_pointers);
        assert_eq!(proposal.facts().as_ptr(), proposal_facts);
    }

    #[test]
    fn p45_exact_and_one_past_bounds_return_complete_unchanged_inputs() {
        let first = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(p43_package(1, 140), p43_package(2, 150))
        .unwrap();
        let second = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
        )
        .propose(p43_package(2, 160), p43_package(1, 170))
        .unwrap();
        let first_pointers = delta_proposal_pointers(&first);
        let second_pointers = delta_proposal_pointers(&second);
        let history = MorphospaceFloat32AdvisoryReportPackageDeltaHistory::new(
            MorphospaceFloat32AdvisoryReportPackageDeltaHistoryBounds::new(1, 2, 4).unwrap(),
        )
        .append(first)
        .unwrap();
        let totals = history.totals();
        let (history, second) = history.append(second).unwrap_err().into_parts();
        assert_eq!(history.totals(), totals);
        assert_eq!(
            delta_proposal_pointers(&history.proposals()[0]),
            first_pointers
        );
        assert_eq!(delta_proposal_pointers(&second), second_pointers);

        let earlier = p43_package(1, 180);
        let later = p43_package(2, 190);
        let earlier_pointers = nested_package_pointers(&earlier);
        let later_pointers = nested_package_pointers(&later);
        let (earlier, later, second) = CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(7).unwrap(),
        )
        .compose(earlier, later, second)
        .unwrap_err()
        .into_parts();
        assert_eq!(nested_package_pointers(&earlier), earlier_pointers);
        assert_eq!(nested_package_pointers(&later), later_pointers);
        assert_eq!(delta_proposal_pointers(&second), second_pointers);
    }

    #[test]
    fn p46_actual_p44_p45_history_to_delta_is_transactional_ordered_and_identity_exact() {
        let comparative = |base| {
            let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
                MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
            )
            .propose(p43_package(1, base), p43_package(2, base + 10))
            .unwrap();
            CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
            )
            .compose(
                p43_package(1, base + 20),
                p43_package(2, base + 30),
                proposal,
            )
            .unwrap()
        };
        let identity = |value: &crate::caller_requested_float32_comparative_advisory_evidence::CallerRequestedFloat32ComparativeAdvisoryEvidence| {
            (
                nested_package_pointers(value.earlier()),
                nested_package_pointers(value.later()),
                value.delta_proposal().facts().as_ptr(),
                value.facts().as_ptr(),
            )
        };

        let first = comparative(200);
        let second = comparative(300);
        let rejected = comparative(400);
        let identities = [identity(&first), identity(&second)];
        let rejected_identity = identity(&rejected);
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(2, 16).unwrap(),
        )
        .append(first)
        .unwrap()
        .append(second)
        .unwrap();
        assert_eq!(history.totals().evidence_count(), 2);
        assert_eq!(history.totals().fact_count(), 16);
        assert_eq!(identity(&history.evidence()[0]), identities[0]);
        assert_eq!(identity(&history.evidence()[1]), identities[1]);

        let totals = history.totals();
        let (history, rejected) = history.append(rejected).unwrap_err().into_parts();
        assert_eq!(history.totals(), totals);
        assert_eq!(identity(&history.evidence()[0]), identities[0]);
        assert_eq!(identity(&history.evidence()[1]), identities[1]);
        assert_eq!(identity(&rejected), rejected_identity);

        let mut retained = history.into_evidence().into_iter();
        let first = retained.next().unwrap();
        let second = retained.next().unwrap();
        assert!(retained.next().is_none());
        let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(first, second)
        .unwrap();
        assert_eq!(identity(delta.earlier()), identities[0]);
        assert_eq!(identity(delta.later()), identities[1]);
        assert_eq!(
            delta
                .facts()
                .iter()
                .map(|fact| fact.count())
                .collect::<Vec<_>>(),
            vec![
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::Facts,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::EqualRelations,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::IncreaseRelations,
                MorphospaceFloat32ComparativeAdvisoryEvidenceCount::DecreaseRelations,
            ]
        );
    }

    #[test]
    fn p47_bounded_delta_history_composes_actual_p43_through_p46_evidence_in_exact_order() {
        let comparative = |base| {
            let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
                MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
            )
            .propose(p43_package(1, base), p43_package(2, base + 10))
            .unwrap();
            CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
            )
            .compose(
                p43_package(1, base + 20),
                p43_package(2, base + 30),
                proposal,
            )
            .unwrap()
        };
        let delta = |base| {
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
                MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
            )
            .propose(comparative(base), comparative(base + 100))
            .unwrap()
        };

        let first = delta(500);
        let second = delta(700);
        let identities = [
            (
                first.earlier().facts().as_ptr(),
                first.later().facts().as_ptr(),
                first.facts().as_ptr(),
            ),
            (
                second.earlier().facts().as_ptr(),
                second.later().facts().as_ptr(),
                second.facts().as_ptr(),
            ),
        ];
        let history = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistory::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaHistoryBounds::new(2, 8).unwrap(),
        )
        .append(first)
        .unwrap()
        .append(second)
        .unwrap();

        assert_eq!(history.totals().proposal_count(), 2);
        assert_eq!(history.totals().fact_count(), 8);
        for (proposal, identity) in history.proposals().iter().zip(identities) {
            assert_eq!(
                (
                    proposal.earlier().facts().as_ptr(),
                    proposal.later().facts().as_ptr(),
                    proposal.facts().as_ptr(),
                ),
                identity
            );
        }
        let proposals = history.into_proposals();
        assert_eq!(proposals.len(), 2);
        assert_eq!(proposals[0].facts().as_ptr(), identities[0].2);
        assert_eq!(proposals[1].facts().as_ptr(), identities[1].2);
    }

    #[test]
    fn p47_snapshot_retains_actual_p43_through_p46_owners_and_stays_private_inert() {
        let comparative = |base| {
            let proposal = MorphospaceFloat32AdvisoryReportPackageDeltaProposalOwner::new(
                MorphospaceFloat32AdvisoryReportPackageDeltaBounds::new(4).unwrap(),
            )
            .propose(p43_package(1, base), p43_package(2, base + 10))
            .unwrap();
            CallerRequestedFloat32ComparativeAdvisoryEvidenceOwner::new(
                CallerRequestedFloat32ComparativeAdvisoryEvidenceBounds::new(8).unwrap(),
            )
            .compose(
                p43_package(1, base + 20),
                p43_package(2, base + 30),
                proposal,
            )
            .unwrap()
        };

        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceHistoryBounds::new(2, 16).unwrap(),
        )
        .append(comparative(900))
        .unwrap()
        .append(comparative(1_100))
        .unwrap();
        let delta = MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaProposalOwner::new(
            MorphospaceFloat32ComparativeAdvisoryEvidenceDeltaBounds::new(4).unwrap(),
        )
        .propose(comparative(1_300), comparative(1_500))
        .unwrap();
        let history_identity = history.evidence().as_ptr();
        let retained_value_identity = history.evidence()[0].earlier().history().values()[0]
            .report()
            .sample()
            .sample()
            .values()
            .as_ptr();
        let delta_identity = delta.facts().as_ptr();

        let snapshot = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotOwner::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotBounds::new(2, 4, 6).unwrap(),
        )
        .snapshot(history, delta)
        .unwrap();

        assert_eq!(snapshot.observation_count(), 6);
        assert!(matches!(
            snapshot.observations(),
            [
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::HistoryEvidence {
                    evidence_index: 0,
                    fact_count: 8
                },
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::HistoryEvidence {
                    evidence_index: 1,
                    fact_count: 8
                },
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                    fact_index: 0,
                    ..
                },
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                    fact_index: 1,
                    ..
                },
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                    fact_index: 2,
                    ..
                },
                CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotObservation::DeltaFact {
                    fact_index: 3,
                    ..
                }
            ]
        ));
        assert_eq!(snapshot.history().evidence().as_ptr(), history_identity);
        assert_eq!(snapshot.delta_proposal().facts().as_ptr(), delta_identity);
        assert_eq!(
            snapshot.history().evidence()[0]
                .earlier()
                .history()
                .values()[0]
                .report()
                .sample()
                .sample()
                .values()
                .as_ptr(),
            retained_value_identity
        );

        let source =
            include_str!("caller_requested_float32_comparative_advisory_evidence_snapshot.rs");
        assert!(source.contains("crate-private"));
        assert!(source.contains("default-inert"));
        assert!(source.contains("non-applying"));
        assert!(source.contains("infers neither loss nor continuity"));
        assert!(!include_str!("runtime.rs")
            .contains("CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot"));
        assert!(!include_str!("lib.rs").contains(concat!(
            "pub use caller_requested_float32_comparative_advisory_",
            "evidence_snapshot"
        )));
    }

    #[test]
    fn p48_snapshot_history_to_delta_preserves_order_facts_and_nested_identity() {
        let earlier = p47_snapshot(1, 2_000);
        let later = p47_snapshot(2, 3_000);
        let identity = |snapshot: &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot| {
            (
                snapshot.observations().as_ptr(),
                snapshot.history().evidence().as_ptr(),
                snapshot.history().evidence()[0]
                    .earlier()
                    .history()
                    .values()[0]
                    .report()
                    .sample()
                    .sample()
                    .values()
                    .as_ptr(),
                snapshot.delta_proposal().facts().as_ptr(),
            )
        };
        let identities = [identity(&earlier), identity(&later)];
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds::new(2, 11)
                .unwrap(),
        )
        .append(earlier)
        .unwrap()
        .append(later)
        .unwrap();
        assert_eq!(history.totals().snapshot_count(), 2);
        assert_eq!(history.totals().fact_count(), 11);
        assert_eq!(identity(&history.snapshots()[0]), identities[0]);
        assert_eq!(identity(&history.snapshots()[1]), identities[1]);

        let mut snapshots = history.into_snapshots().into_iter();
        let earlier = snapshots.next().unwrap();
        let later = snapshots.next().unwrap();
        assert!(snapshots.next().is_none());
        assert_eq!(identity(&earlier), identities[0]);
        assert_eq!(identity(&later), identities[1]);
        let proposal =
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner::new(
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds::new(6).unwrap(),
            )
            .propose(earlier, later)
            .unwrap();
        assert_eq!(identity(proposal.earlier()), identities[0]);
        assert_eq!(identity(proposal.later()), identities[1]);
        assert_eq!(proposal.fact_count(), 6);
        let expected = [
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::Observations,
                5,
                6,
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 },
            ),
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::HistoryEvidence,
                1,
                2,
                MorphospaceFloat32AdvisoryReportPackageRelation::Increase { amount: 1 },
            ),
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::DeltaFacts,
                4,
                4,
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal,
            ),
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::EqualDeltaRelations,
                4,
                4,
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal,
            ),
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::IncreaseDeltaRelations,
                0,
                0,
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal,
            ),
            (
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotCount::DecreaseDeltaRelations,
                0,
                0,
                MorphospaceFloat32AdvisoryReportPackageRelation::Equal,
            ),
        ];
        assert!(proposal
            .facts()
            .iter()
            .zip(expected)
            .all(|(fact, expected)| {
                (fact.count(), fact.earlier(), fact.later(), fact.relation()) == expected
            }));
        let (earlier, later) = proposal.into_snapshots();
        assert_eq!(identity(&earlier), identities[0]);
        assert_eq!(identity(&later), identities[1]);
    }

    #[test]
    fn p48_composed_delta_failure_returns_both_snapshots_without_partial_mutation() {
        let earlier = p47_snapshot(1, 4_000);
        let later = p47_snapshot(2, 5_000);
        let identity = |snapshot: &CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot| {
            (
                snapshot.observations().as_ptr(),
                snapshot.history().evidence().as_ptr(),
                snapshot.delta_proposal().facts().as_ptr(),
                snapshot.observations().to_vec(),
            )
        };
        let identities = [identity(&earlier), identity(&later)];
        let history = CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistory::new(
            CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshotHistoryBounds::new(2, 11)
                .unwrap(),
        )
        .append(earlier)
        .unwrap()
        .append(later)
        .unwrap();
        let mut snapshots = history.into_snapshots().into_iter();
        let earlier = snapshots.next().unwrap();
        let later = snapshots.next().unwrap();
        let (earlier, later) =
            MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaProposalOwner::new(
                MorphospaceFloat32ComparativeAdvisoryEvidenceSnapshotDeltaBounds::new(5).unwrap(),
            )
            .propose(earlier, later)
            .unwrap_err()
            .into_snapshots();
        assert_eq!(identity(&earlier), identities[0]);
        assert_eq!(identity(&later), identities[1]);
    }
}
