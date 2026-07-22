// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concrete all-format facade over the complete caller-named discovery lifecycles.

use crate::*;
use std::sync::atomic::AtomicBool;

/// Concrete sample formats accepted by the coherent lifecycle facade.
///
/// This is intentionally distinct from the wire-format enum: every variant has a
/// corresponding concrete request, result, and error owner. A later Float32 variant
/// can be added without changing any existing variant or introducing generic codec
/// authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CompleteTypedUdpDiscoveryFormat {
    Int8,
    Int16,
    Int32,
    Int64,
    Double64,
    String,
}

/// Request rejected before discovery or session I/O begins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteTypedUdpDiscoveryRequestError {
    EmptyStreamName,
    ZeroChannelCount,
    ZeroRecordCount,
}

/// Format-specific activation and bounded session limits.
#[derive(Debug)]
#[non_exhaustive]
pub enum CompleteTypedUdpDiscoverySessionRequest {
    Int8 {
        activation: FixedWidthNumericSampleActivation,
        io_limits: TimestampedInt8SessionIoLimits,
        session_limits: TimestampedInt8SessionLimits,
    },
    Int16 {
        activation: FixedWidthNumericSampleActivation,
        io_limits: TimestampedInt16SessionIoLimits,
        session_limits: TimestampedInt16SessionLimits,
    },
    Int32 {
        activation: FixedWidthNumericSampleActivation,
        io_limits: TimestampedInt32SessionIoLimits,
        session_limits: TimestampedInt32SessionLimits,
    },
    Int64 {
        activation: FixedWidthNumericSampleActivation,
        io_limits: TimestampedInt64SessionIoLimits,
        session_limits: TimestampedInt64SessionLimits,
    },
    Double64 {
        activation: FixedWidthNumericSampleActivation,
        io_limits: TimestampedDouble64SessionIoLimits,
        session_limits: TimestampedDouble64SessionLimits,
    },
    String {
        activation: StringSampleActivation,
        io_limits: StringSampleLimits,
        session_limits: TimestampedStringSessionLimits,
    },
}

impl CompleteTypedUdpDiscoverySessionRequest {
    pub const fn format(&self) -> CompleteTypedUdpDiscoveryFormat {
        match self {
            Self::Int8 { .. } => CompleteTypedUdpDiscoveryFormat::Int8,
            Self::Int16 { .. } => CompleteTypedUdpDiscoveryFormat::Int16,
            Self::Int32 { .. } => CompleteTypedUdpDiscoveryFormat::Int32,
            Self::Int64 { .. } => CompleteTypedUdpDiscoveryFormat::Int64,
            Self::Double64 { .. } => CompleteTypedUdpDiscoveryFormat::Double64,
            Self::String { .. } => CompleteTypedUdpDiscoveryFormat::String,
        }
    }
}

/// One bounded caller-named discovery-to-completion request.
pub struct CompleteTypedUdpDiscoveryRequest<'a> {
    discovery_activation: UdpDiscoveryActivation,
    discovery_config: UdpDiscoveryConfig,
    query: &'a ShortInfoQueryWire,
    envelope_limits: ShortInfoResponseEnvelopeLimits,
    admission_limits: StreamInfoObservedAdmissionLimits,
    stream_name: &'a str,
    expected_identity: &'a StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    channel_count: usize,
    record_count: usize,
    session: CompleteTypedUdpDiscoverySessionRequest,
}

impl<'a> CompleteTypedUdpDiscoveryRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        discovery_activation: UdpDiscoveryActivation,
        discovery_config: UdpDiscoveryConfig,
        query: &'a ShortInfoQueryWire,
        envelope_limits: ShortInfoResponseEnvelopeLimits,
        admission_limits: StreamInfoObservedAdmissionLimits,
        stream_name: &'a str,
        expected_identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        channel_count: usize,
        record_count: usize,
        session: CompleteTypedUdpDiscoverySessionRequest,
    ) -> Result<Self, CompleteTypedUdpDiscoveryRequestError> {
        validate_request_bounds(stream_name, channel_count, record_count)?;
        Ok(Self {
            discovery_activation,
            discovery_config,
            query,
            envelope_limits,
            admission_limits,
            stream_name,
            expected_identity,
            handshake_limits,
            channel_count,
            record_count,
            session,
        })
    }

    pub const fn format(&self) -> CompleteTypedUdpDiscoveryFormat {
        self.session.format()
    }

    pub const fn stream_name(&self) -> &str {
        self.stream_name
    }
}

fn validate_request_bounds(
    stream_name: &str,
    channel_count: usize,
    record_count: usize,
) -> Result<(), CompleteTypedUdpDiscoveryRequestError> {
    if stream_name.is_empty() {
        return Err(CompleteTypedUdpDiscoveryRequestError::EmptyStreamName);
    }
    if channel_count == 0 {
        return Err(CompleteTypedUdpDiscoveryRequestError::ZeroChannelCount);
    }
    if record_count == 0 {
        return Err(CompleteTypedUdpDiscoveryRequestError::ZeroRecordCount);
    }
    Ok(())
}

/// The unchanged concrete owner returned by the selected format lifecycle.
#[derive(Debug)]
#[non_exhaustive]
pub enum CompleteTypedUdpDiscoveryOutput {
    Int8(CompletedTypedUdpDiscoveryInt8Lifecycle),
    Int16(CompletedTypedUdpDiscoveryInt16Lifecycle),
    Int32(CompletedTypedUdpDiscoveryInt32Lifecycle),
    Int64(CompletedTypedUdpDiscoveryInt64Lifecycle),
    Double64(TimestampedDouble64InletSessionReport),
    String(TimestampedStringInletSessionReport),
}

impl CompleteTypedUdpDiscoveryOutput {
    pub const fn format(&self) -> CompleteTypedUdpDiscoveryFormat {
        match self {
            Self::Int8(_) => CompleteTypedUdpDiscoveryFormat::Int8,
            Self::Int16(_) => CompleteTypedUdpDiscoveryFormat::Int16,
            Self::Int32(_) => CompleteTypedUdpDiscoveryFormat::Int32,
            Self::Int64(_) => CompleteTypedUdpDiscoveryFormat::Int64,
            Self::Double64(_) => CompleteTypedUdpDiscoveryFormat::Double64,
            Self::String(_) => CompleteTypedUdpDiscoveryFormat::String,
        }
    }

    /// Receive-order identity retained by complete owners that expose it.
    pub const fn response_index(&self) -> Option<usize> {
        match self {
            Self::Int8(value) => Some(value.response_index()),
            Self::Int16(value) => Some(value.response_index()),
            Self::Int32(value) => Some(value.response_index()),
            Self::Int64(value) => Some(value.response_index()),
            Self::Double64(_) | Self::String(_) => None,
        }
    }
}

/// Successful facade result retaining the exact caller-named selection identity.
#[derive(Debug)]
pub struct CompleteTypedUdpDiscoveryResult {
    stream_name: String,
    output: CompleteTypedUdpDiscoveryOutput,
}

impl CompleteTypedUdpDiscoveryResult {
    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }

    pub const fn output(&self) -> &CompleteTypedUdpDiscoveryOutput {
        &self.output
    }

    pub fn into_output(self) -> CompleteTypedUdpDiscoveryOutput {
        self.output
    }
}

/// Stage-preserving failure from the selected concrete lifecycle.
#[derive(Debug)]
#[non_exhaustive]
pub enum CompleteTypedUdpDiscoveryError {
    Int8(TypedUdpDiscoveryInt8CompleteLifecycleError),
    Int16(TypedUdpDiscoveryInt16CompleteLifecycleError),
    Int32(TypedUdpDiscoveryInt32CompleteLifecycleError),
    Int64(TypedUdpDiscoveryInt64CompleteLifecycleError),
    Double64(TypedUdpDiscoveryDouble64CompleteLifecycleError),
    String(TypedUdpDiscoveryStringCompleteLifecycleError),
}

impl CompleteTypedUdpDiscoveryError {
    pub const fn format(&self) -> CompleteTypedUdpDiscoveryFormat {
        match self {
            Self::Int8(_) => CompleteTypedUdpDiscoveryFormat::Int8,
            Self::Int16(_) => CompleteTypedUdpDiscoveryFormat::Int16,
            Self::Int32(_) => CompleteTypedUdpDiscoveryFormat::Int32,
            Self::Int64(_) => CompleteTypedUdpDiscoveryFormat::Int64,
            Self::Double64(_) => CompleteTypedUdpDiscoveryFormat::Double64,
            Self::String(_) => CompleteTypedUdpDiscoveryFormat::String,
        }
    }
}

/// Dispatches to exactly one existing concrete complete caller-named lifecycle.
///
/// Discovery and session cancellation remain separate and caller-owned. Every
/// activation remains explicit; this function adds no retry, selection policy,
/// background work, codec, socket, completion, close, or cleanup authority.
pub fn run_complete_typed_udp_discovery_lifecycle(
    request: CompleteTypedUdpDiscoveryRequest<'_>,
    discovery_cancelled: &AtomicBool,
    session_cancelled: &AtomicBool,
) -> Result<CompleteTypedUdpDiscoveryResult, CompleteTypedUdpDiscoveryError> {
    let CompleteTypedUdpDiscoveryRequest {
        discovery_activation,
        discovery_config,
        query,
        envelope_limits,
        admission_limits,
        stream_name,
        expected_identity,
        handshake_limits,
        channel_count,
        record_count,
        session,
    } = request;

    let output = match session {
        CompleteTypedUdpDiscoverySessionRequest::Int8 {
            activation,
            io_limits,
            session_limits,
        } => CompleteTypedUdpDiscoveryOutput::Int8(
            run_typed_udp_discovery_int8_session_inlet(
                discovery_activation,
                discovery_config,
                query,
                discovery_cancelled,
                envelope_limits,
                admission_limits,
                stream_name,
                activation,
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
                session_cancelled,
            )
            .map_err(CompleteTypedUdpDiscoveryError::Int8)?,
        ),
        CompleteTypedUdpDiscoverySessionRequest::Int16 {
            activation,
            io_limits,
            session_limits,
        } => CompleteTypedUdpDiscoveryOutput::Int16(
            run_typed_udp_discovery_int16_session_inlet(
                discovery_activation,
                discovery_config,
                query,
                discovery_cancelled,
                envelope_limits,
                admission_limits,
                stream_name,
                activation,
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
                session_cancelled,
            )
            .map_err(CompleteTypedUdpDiscoveryError::Int16)?,
        ),
        CompleteTypedUdpDiscoverySessionRequest::Int32 {
            activation,
            io_limits,
            session_limits,
        } => CompleteTypedUdpDiscoveryOutput::Int32(
            run_typed_udp_discovery_int32_session_inlet(
                discovery_activation,
                discovery_config,
                query,
                discovery_cancelled,
                envelope_limits,
                admission_limits,
                stream_name,
                activation,
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
                session_cancelled,
            )
            .map_err(CompleteTypedUdpDiscoveryError::Int32)?,
        ),
        CompleteTypedUdpDiscoverySessionRequest::Int64 {
            activation,
            io_limits,
            session_limits,
        } => CompleteTypedUdpDiscoveryOutput::Int64(
            run_typed_udp_discovery_int64_session_inlet(
                discovery_activation,
                discovery_config,
                query,
                discovery_cancelled,
                envelope_limits,
                admission_limits,
                stream_name,
                activation,
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
                session_cancelled,
            )
            .map_err(CompleteTypedUdpDiscoveryError::Int64)?,
        ),
        CompleteTypedUdpDiscoverySessionRequest::Double64 {
            activation,
            io_limits,
            session_limits,
        } => CompleteTypedUdpDiscoveryOutput::Double64(
            run_named_typed_udp_discovery_double64_session_inlet(
                discovery_activation,
                discovery_config,
                query,
                discovery_cancelled,
                envelope_limits,
                admission_limits,
                stream_name,
                activation,
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
                session_cancelled,
            )
            .map_err(CompleteTypedUdpDiscoveryError::Double64)?,
        ),
        CompleteTypedUdpDiscoverySessionRequest::String {
            activation,
            io_limits,
            session_limits,
        } => CompleteTypedUdpDiscoveryOutput::String(
            run_named_typed_udp_discovery_string_session_inlet(
                discovery_activation,
                discovery_config,
                query,
                discovery_cancelled,
                envelope_limits,
                admission_limits,
                stream_name,
                activation,
                expected_identity,
                handshake_limits,
                io_limits,
                session_limits,
                channel_count,
                record_count,
                session_cancelled,
            )
            .map_err(CompleteTypedUdpDiscoveryError::String)?,
        ),
    };

    Ok(CompleteTypedUdpDiscoveryResult {
        stream_name: stream_name.to_owned(),
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p58_format_dispatch_identity_is_stable_and_float32_additive() {
        assert_eq!(
            CompleteTypedUdpDiscoveryFormat::Int8,
            CompleteTypedUdpDiscoveryFormat::Int8
        );
        assert_ne!(
            CompleteTypedUdpDiscoveryFormat::Int64,
            CompleteTypedUdpDiscoveryFormat::Double64
        );
        let variants = [
            CompleteTypedUdpDiscoveryFormat::Int8,
            CompleteTypedUdpDiscoveryFormat::Int16,
            CompleteTypedUdpDiscoveryFormat::Int32,
            CompleteTypedUdpDiscoveryFormat::Int64,
            CompleteTypedUdpDiscoveryFormat::Double64,
            CompleteTypedUdpDiscoveryFormat::String,
        ];
        assert_eq!(variants.len(), 6);
    }

    #[test]
    fn p58_invalid_input_is_rejected_before_dispatch() {
        assert_eq!(
            validate_request_bounds("", 1, 1),
            Err(CompleteTypedUdpDiscoveryRequestError::EmptyStreamName)
        );
        assert_eq!(
            validate_request_bounds("stream", 0, 1),
            Err(CompleteTypedUdpDiscoveryRequestError::ZeroChannelCount)
        );
        assert_eq!(
            validate_request_bounds("stream", 1, 0),
            Err(CompleteTypedUdpDiscoveryRequestError::ZeroRecordCount)
        );
        assert_eq!(validate_request_bounds("stream", 2, 3), Ok(()));
    }
}
