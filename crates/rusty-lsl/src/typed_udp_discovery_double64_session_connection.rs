// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-selected typed-discovery response to bounded Double64 inlet-session composition.

use crate::typed_udp_discovery_session_contract::{
    validate_selected_typed_udp_discovery_session_contract,
    TypedUdpDiscoverySessionContractMismatch,
};
use crate::{
    propose_typed_udp_discovery_ipv4_service_endpoint, run_typed_udp_discovery,
    suggest_typed_udp_discovery_response, ChannelFormat, FixedWidthNumericSampleActivation,
    ShortInfoQueryWire, ShortInfoResponseEnvelopeLimits, StreamHandshakeIdentity,
    StreamHandshakeIdentityRole, StreamHandshakeLimits, StreamInfoObservedAdmissionLimits,
    TimestampedDouble64ConnectedInletSession, TimestampedDouble64InletSession,
    TimestampedDouble64InletSessionReport, TimestampedDouble64SessionError,
    TimestampedDouble64SessionIncomplete, TimestampedDouble64SessionIoLimits,
    TimestampedDouble64SessionLimits, TimestampedDouble64SessionPreflightError,
    TimestampedDouble64SessionTransferError, TypedUdpDiscoveryEndpointError, TypedUdpDiscoveryRun,
    TypedUdpDiscoveryRunError, TypedUdpDiscoverySelectionError, UdpDiscoveryActivation,
    UdpDiscoveryConfig,
};
use std::sync::atomic::AtomicBool;

/// Failure from the caller-selected discovery-to-Double64 session composition.
#[derive(Debug, Eq, PartialEq)]
pub enum TypedUdpDiscoveryDouble64SessionConnectionError {
    /// Strict projection of the caller-selected response failed.
    Endpoint(TypedUdpDiscoveryEndpointError),
    /// The selected response advertises a different sample format.
    FormatMismatch {
        /// Format required by the concrete adapter.
        expected: ChannelFormat,
        /// Format advertised by the selected response.
        actual: ChannelFormat,
    },
    /// The selected response advertises a different channel count.
    ChannelCountMismatch {
        /// Channel count requested by the caller.
        expected: usize,
        /// Channel count advertised by the selected response.
        actual: usize,
    },
    /// The selected response advertises a different handshake identity field.
    IdentityMismatch {
        /// Identity role whose value differs.
        role: StreamHandshakeIdentityRole,
        /// Caller-owned expected identity value.
        expected: String,
        /// Selected-response identity value.
        actual: String,
    },
    /// The selected endpoint or requested shape failed bounded session preflight.
    Preflight(TimestampedDouble64SessionPreflightError),
    /// Connect, transfer, terminal close, or cleanup failed.
    Session(TimestampedDouble64SessionError),
}

/// Failure from one caller-explicit discovery, exact-name selection, and Double64 session run.
#[derive(Debug, PartialEq)]
pub enum TypedUdpDiscoveryDouble64CompleteLifecycleError {
    Discovery(TypedUdpDiscoveryRunError),
    Selection(TypedUdpDiscoverySelectionError),
    NoMatchingStreamName {
        stream_name: String,
        discovery: TypedUdpDiscoveryRun,
    },
    Connection(TypedUdpDiscoveryDouble64SessionConnectionError),
    Transfer(TimestampedDouble64SessionTransferError),
    Incomplete(TimestampedDouble64SessionIncomplete),
    Session(TimestampedDouble64SessionError),
}

/// Connected Double64 inlet retaining the caller-selected discovery identity.
pub struct ConnectedSelectedTypedUdpDiscoveryDouble64Session<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    session: TimestampedDouble64ConnectedInletSession,
}

impl<'a> ConnectedSelectedTypedUdpDiscoveryDouble64Session<'a> {
    pub const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }
    pub const fn response_index(&self) -> usize {
        self.response_index
    }
    pub fn peer(&self) -> std::net::SocketAddr {
        self.session.peer()
    }
    pub fn channel_count(&self) -> usize {
        self.session.channel_count()
    }
    pub fn record_count(&self) -> usize {
        self.session.record_count()
    }
    pub fn completed_record_count(&self) -> usize {
        self.session.completed_record_count()
    }
    pub fn received_records(&self) -> &[crate::TimestampedSample<f64>] {
        self.session.received_records()
    }
    pub fn transfer_next(
        &mut self,
        cancelled: &AtomicBool,
    ) -> Result<(), TimestampedDouble64SessionTransferError> {
        self.session.transfer_next(cancelled)
    }
    pub fn complete(
        self,
        cancelled: &AtomicBool,
    ) -> Result<
        Result<
            CompletedSelectedTypedUdpDiscoveryDouble64Session<'a>,
            TimestampedDouble64SessionError,
        >,
        TimestampedDouble64SessionIncomplete,
    > {
        let Self {
            discovery,
            response_index,
            session,
        } = self;
        session.complete(cancelled).map(|result| {
            result.map(|report| CompletedSelectedTypedUdpDiscoveryDouble64Session {
                discovery,
                response_index,
                report,
            })
        })
    }
    pub fn finish(
        self,
        cancelled: &AtomicBool,
    ) -> Result<
        CompletedSelectedTypedUdpDiscoveryDouble64Session<'a>,
        TimestampedDouble64SessionError,
    > {
        let Self {
            discovery,
            response_index,
            session,
        } = self;
        session
            .finish(cancelled)
            .map(|report| CompletedSelectedTypedUdpDiscoveryDouble64Session {
                discovery,
                response_index,
                report,
            })
    }
    pub fn close(self) {
        self.session.close();
    }
}

/// Canonically completed Double64 report retaining the caller-selected discovery identity.
pub struct CompletedSelectedTypedUdpDiscoveryDouble64Session<'a> {
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    report: TimestampedDouble64InletSessionReport,
}

impl<'a> CompletedSelectedTypedUdpDiscoveryDouble64Session<'a> {
    pub const fn discovery(&self) -> &'a TypedUdpDiscoveryRun {
        self.discovery
    }
    pub const fn response_index(&self) -> usize {
        self.response_index
    }
    pub const fn report(&self) -> &TimestampedDouble64InletSessionReport {
        &self.report
    }
    pub fn into_report(self) -> TimestampedDouble64InletSessionReport {
        self.report
    }
}

fn contract_error(
    mismatch: TypedUdpDiscoverySessionContractMismatch<'_>,
) -> TypedUdpDiscoveryDouble64SessionConnectionError {
    match mismatch {
        TypedUdpDiscoverySessionContractMismatch::Format { expected, actual } => {
            TypedUdpDiscoveryDouble64SessionConnectionError::FormatMismatch { expected, actual }
        }
        TypedUdpDiscoverySessionContractMismatch::ChannelCount { expected, actual } => {
            TypedUdpDiscoveryDouble64SessionConnectionError::ChannelCountMismatch {
                expected,
                actual,
            }
        }
        TypedUdpDiscoverySessionContractMismatch::Identity {
            role,
            expected,
            actual,
        } => TypedUdpDiscoveryDouble64SessionConnectionError::IdentityMismatch {
            role,
            expected: expected.to_owned(),
            actual: actual.to_owned(),
        },
    }
}

impl<'a> TimestampedDouble64InletSession<'a> {
    /// Resolves one caller-selected discovery response into socket-free Double64 preflight.
    #[allow(clippy::too_many_arguments)]
    pub fn preflight_selected_typed_udp_discovery(
        discovery: &TypedUdpDiscoveryRun,
        response_index: usize,
        session_activation: FixedWidthNumericSampleActivation,
        expected_identity: &'a StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        io_limits: TimestampedDouble64SessionIoLimits,
        session_limits: TimestampedDouble64SessionLimits,
        channel_count: usize,
        record_count: usize,
    ) -> Result<Self, TypedUdpDiscoveryDouble64SessionConnectionError> {
        let endpoint = propose_typed_udp_discovery_ipv4_service_endpoint(discovery, response_index)
            .map_err(TypedUdpDiscoveryDouble64SessionConnectionError::Endpoint)?;
        validate_selected_typed_udp_discovery_session_contract(
            &discovery.responses()[response_index],
            ChannelFormat::Double64,
            channel_count,
            expected_identity,
        )
        .map_err(contract_error)?;
        Self::preflight_bounded(
            session_activation,
            endpoint.into(),
            expected_identity,
            handshake_limits,
            io_limits,
            session_limits,
            channel_count,
            record_count,
        )
        .map_err(TypedUdpDiscoveryDouble64SessionConnectionError::Preflight)
    }
}

/// Projects one caller-selected completed discovery response and connects one bounded inlet.
///
/// The caller retains the completed discovery run, receive-order selection, expected identity,
/// limits, cancellation, and activation. The strict endpoint projector runs before the existing
/// selected-response contract validation, Double64 preflight, and connect owners in that order.
/// The returned concrete connected owner retains phased
/// transfer, canonical completion, allocation ownership, and report-free close. This adapter owns
/// no discovery, ranking, retry, identity derivation, codec, cursor, lifecycle, socket, or report.
#[allow(clippy::too_many_arguments)]
pub fn connect_selected_typed_udp_discovery_double64_session_inlet<'a>(
    discovery: &'a TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: FixedWidthNumericSampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: TimestampedDouble64SessionIoLimits,
    session_limits: TimestampedDouble64SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<
    ConnectedSelectedTypedUdpDiscoveryDouble64Session<'a>,
    TypedUdpDiscoveryDouble64SessionConnectionError,
> {
    let session = TimestampedDouble64InletSession::preflight_selected_typed_udp_discovery(
        discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        io_limits,
        session_limits,
        channel_count,
        record_count,
    )?;
    let session = session
        .connect(session_cancelled)
        .map_err(TypedUdpDiscoveryDouble64SessionConnectionError::Session)?;
    Ok(ConnectedSelectedTypedUdpDiscoveryDouble64Session {
        discovery,
        response_index,
        session,
    })
}

/// Runs the selected bounded Double64 inlet to its canonical completion report.
#[allow(clippy::too_many_arguments)]
pub fn run_selected_typed_udp_discovery_double64_session_inlet(
    discovery: &TypedUdpDiscoveryRun,
    response_index: usize,
    session_activation: FixedWidthNumericSampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: TimestampedDouble64SessionIoLimits,
    session_limits: TimestampedDouble64SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<TimestampedDouble64InletSessionReport, TypedUdpDiscoveryDouble64SessionConnectionError>
{
    connect_selected_typed_udp_discovery_double64_session_inlet(
        discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        io_limits,
        session_limits,
        channel_count,
        record_count,
        session_cancelled,
    )?
    .finish(session_cancelled)
    .map(CompletedSelectedTypedUdpDiscoveryDouble64Session::into_report)
    .map_err(TypedUdpDiscoveryDouble64SessionConnectionError::Session)
}

/// Runs bounded typed discovery, exact-name suggestion, and the selected Double64 session.
#[allow(clippy::too_many_arguments)]
pub fn run_named_typed_udp_discovery_double64_session_inlet(
    discovery_activation: UdpDiscoveryActivation,
    discovery_config: UdpDiscoveryConfig,
    query: &ShortInfoQueryWire,
    discovery_cancelled: &AtomicBool,
    envelope_limits: ShortInfoResponseEnvelopeLimits,
    admission_limits: StreamInfoObservedAdmissionLimits,
    stream_name: &str,
    session_activation: FixedWidthNumericSampleActivation,
    expected_identity: &StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    io_limits: TimestampedDouble64SessionIoLimits,
    session_limits: TimestampedDouble64SessionLimits,
    channel_count: usize,
    record_count: usize,
    session_cancelled: &AtomicBool,
) -> Result<TimestampedDouble64InletSessionReport, TypedUdpDiscoveryDouble64CompleteLifecycleError>
{
    let discovery = run_typed_udp_discovery(
        discovery_activation,
        discovery_config,
        query,
        discovery_cancelled,
        envelope_limits,
        admission_limits,
    )
    .map_err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Discovery)?;
    let response_index = match suggest_typed_udp_discovery_response(&discovery, stream_name)
        .map_err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Selection)?
    {
        Some(index) => index,
        None => {
            return Err(
                TypedUdpDiscoveryDouble64CompleteLifecycleError::NoMatchingStreamName {
                    stream_name: stream_name.to_owned(),
                    discovery,
                },
            );
        }
    };
    let mut connected = connect_selected_typed_udp_discovery_double64_session_inlet(
        &discovery,
        response_index,
        session_activation,
        expected_identity,
        handshake_limits,
        io_limits,
        session_limits,
        channel_count,
        record_count,
        session_cancelled,
    )
    .map_err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Connection)?;
    while connected.completed_record_count() < connected.record_count() {
        connected
            .transfer_next(session_cancelled)
            .map_err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Transfer)?;
    }
    connected
        .complete(session_cancelled)
        .map_err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Incomplete)?
        .map(CompletedSelectedTypedUdpDiscoveryDouble64Session::into_report)
        .map_err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_typed_udp_discovery, MetadataTreeLimits, RawSourceTimestamp, RuntimeModule, Sample,
        SampleLimits, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamInfoObservedAdmissionLimits, StreamInfoVolatileFieldLimits,
        TimestampedDouble64OutletSession, TimestampedSample, UdpDiscoveryActivation,
        UdpDiscoveryConfig, UdpDiscoveryLimits,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn io_limits() -> TimestampedDouble64SessionIoLimits {
        TimestampedDouble64SessionIoLimits::new(Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "77777777-2222-4333-8444-555555555555".into(),
            "host".into(),
            "source".into(),
            "session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn discovery_activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap()
    }

    fn session_activation() -> FixedWidthNumericSampleActivation {
        FixedWidthNumericSampleActivation::new(
            test_capability(RuntimeModule::FixedWidthNumericSample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        ShortInfoQueryWire::encode(
            &ShortInfoQuery::new("selected".into(), 1, 20, limits).unwrap(),
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

    fn document(address: &str, port: u16, channels: usize) -> String {
        let fields = [
            ("name", "selected".to_owned()),
            ("type", "independent".to_owned()),
            ("channel_count", channels.to_string()),
            ("channel_format", "double64".to_owned()),
            ("source_id", "source".to_owned()),
            ("nominal_srate", "100.0000000000000".to_owned()),
            ("version", "110".to_owned()),
            ("created_at", "1".to_owned()),
            ("uid", "77777777-2222-4333-8444-555555555555".to_owned()),
            ("session_id", "session".to_owned()),
            ("hostname", "host".to_owned()),
            ("v4address", address.to_owned()),
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

    fn completed_discovery(document: String) -> TypedUdpDiscoveryRun {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let bytes = document.len();
        let responder = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut query).unwrap();
            socket
                .send_to(format!("20\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let run = run_typed_udp_discovery(
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
        )
        .unwrap();
        responder.join().unwrap();
        run
    }

    fn run_named(
        document: String,
        stream_name: &str,
        channels: usize,
        count: usize,
        session_cancelled: &AtomicBool,
    ) -> Result<
        TimestampedDouble64InletSessionReport,
        TypedUdpDiscoveryDouble64CompleteLifecycleError,
    > {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = socket.local_addr().unwrap();
        let bytes = document.len();
        let responder = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = socket.recv_from(&mut query).unwrap();
            socket
                .send_to(format!("20\r\n{document}").as_bytes(), source)
                .unwrap();
        });
        let result = run_named_typed_udp_discovery_double64_session_inlet(
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
            stream_name,
            session_activation(),
            &identity(),
            handshake_limits(),
            io_limits(),
            TimestampedDouble64SessionLimits::new(channels, count).unwrap(),
            channels,
            count,
            session_cancelled,
        );
        responder.join().unwrap();
        result
    }

    fn records(channels: usize, count: usize) -> Vec<TimestampedSample<f64>> {
        (0..count)
            .map(|record| {
                let values = (0..channels)
                    .map(|channel| {
                        f64::from_bits(0x3ff0_0000_0000_0000 + (record * 4 + channel) as u64)
                    })
                    .collect();
                TimestampedSample::new(
                    Sample::new(SampleLimits::new(channels).unwrap(), channels, values).unwrap(),
                    RawSourceTimestamp::new(10.0 + record as f64).unwrap(),
                    None,
                )
            })
            .collect()
    }

    fn contract_failure(
        document: String,
        expected_identity: &StreamHandshakeIdentity,
        channels: usize,
    ) -> TypedUdpDiscoveryDouble64SessionConnectionError {
        let discovery = completed_discovery(document);
        match connect_selected_typed_udp_discovery_double64_session_inlet(
            &discovery,
            0,
            session_activation(),
            expected_identity,
            handshake_limits(),
            io_limits(),
            TimestampedDouble64SessionLimits::new(channels, if channels == 1 { 1 } else { 3 })
                .unwrap(),
            channels,
            if channels == 1 { 1 } else { 3 },
            &AtomicBool::new(false),
        ) {
            Err(error) => error,
            Ok(_) => panic!("selected contract unexpectedly connected"),
        }
    }

    fn assert_phased_shape(channels: usize, count: usize) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let sent = records(channels, count);
        let expected_bits: Vec<Vec<u64>> = sent
            .iter()
            .map(|record| {
                record
                    .sample()
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect()
            })
            .collect();
        let outlet = thread::spawn(move || {
            TimestampedDouble64OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(channels, count).unwrap(),
                &sent,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port(), channels));
        let expected_identity = identity();
        let mut connected = connect_selected_typed_udp_discovery_double64_session_inlet(
            &discovery,
            0,
            session_activation(),
            &expected_identity,
            handshake_limits(),
            io_limits(),
            TimestampedDouble64SessionLimits::new(channels, count).unwrap(),
            channels,
            count,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(discovery.responses().len(), 1);
        assert!(std::ptr::eq(connected.discovery(), &discovery));
        assert_eq!(connected.response_index(), 0);
        assert_eq!(connected.peer(), endpoint);
        for completed in 1..=count {
            connected.transfer_next(&AtomicBool::new(false)).unwrap();
            assert_eq!(connected.completed_record_count(), completed);
        }
        let completed = connected
            .complete(&AtomicBool::new(false))
            .unwrap()
            .unwrap();
        assert!(std::ptr::eq(completed.discovery(), &discovery));
        assert_eq!(completed.response_index(), 0);
        let actual_bits: Vec<Vec<u64>> = completed
            .report()
            .records()
            .iter()
            .map(|record| {
                record
                    .sample()
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect()
            })
            .collect();
        assert_eq!(actual_bits, expected_bits);
        assert_eq!(outlet.join().unwrap().record_count(), count);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p56_double64_transfer_failure_retains_selection_and_close_reuses_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let outlet = thread::spawn(move || {
            let sent = records(2, 3);
            TimestampedDouble64OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
                &sent,
            )
            .unwrap()
            .accept(&AtomicBool::new(false))
            .unwrap()
            .close()
        });
        let discovery = completed_discovery(document("127.0.0.1", endpoint.port(), 2));
        let mut connected = connect_selected_typed_udp_discovery_double64_session_inlet(
            &discovery,
            0,
            session_activation(),
            &identity(),
            handshake_limits(),
            io_limits(),
            TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
            2,
            3,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(matches!(
            connected.transfer_next(&AtomicBool::new(true)),
            Err(TimestampedDouble64SessionTransferError::Session(
                TimestampedDouble64SessionError::Record {
                    index: None,
                    error: crate::FixedWidthNumericSampleError::Cancelled
                }
            ))
        ));
        assert!(std::ptr::eq(connected.discovery(), &discovery));
        assert_eq!(connected.response_index(), 0);
        assert_eq!(connected.completed_record_count(), 0);
        assert!(connected.received_records().is_empty());
        connected.close();
        outlet.join().unwrap();
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p20_selected_response_enters_phased_double64_for_only_accepted_shapes() {
        assert_phased_shape(1, 1);
        assert_phased_shape(2, 3);
    }

    #[test]
    fn p20_selection_and_shape_rejections_precede_tcp_io() {
        let discovery = completed_discovery(document("127.0.0.1", 9, 2));
        let invalid_shape_discovery = completed_discovery(document("127.0.0.1", 9, 3));
        assert!(matches!(
            connect_selected_typed_udp_discovery_double64_session_inlet(
                &discovery,
                1,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
                2,
                3,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryDouble64SessionConnectionError::Endpoint(
                TypedUdpDiscoveryEndpointError::ResponseUnavailable {
                    index: 1,
                    response_count: 1
                }
            ))
        ));
        assert!(matches!(
            connect_selected_typed_udp_discovery_double64_session_inlet(
                &invalid_shape_discovery,
                0,
                session_activation(),
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(3, 3).unwrap(),
                3,
                3,
                &AtomicBool::new(false),
            ),
            Err(TypedUdpDiscoveryDouble64SessionConnectionError::Preflight(
                TimestampedDouble64SessionPreflightError::ChannelCount {
                    index: 0,
                    actual: 3
                }
            ))
        ));
        assert_eq!(discovery.responses().len(), 1);
    }

    #[test]
    fn selected_resolution_p20_contract_mismatches_are_owned_in_frozen_precedence() {
        let expected_identity = identity();
        assert_eq!(
            contract_failure(
                document("127.0.0.1", 9, 2).replace("double64", "int32"),
                &expected_identity,
                1,
            ),
            TypedUdpDiscoveryDouble64SessionConnectionError::FormatMismatch {
                expected: ChannelFormat::Double64,
                actual: ChannelFormat::Int32,
            }
        );
        assert_eq!(
            contract_failure(document("127.0.0.1", 9, 2), &expected_identity, 1),
            TypedUdpDiscoveryDouble64SessionConnectionError::ChannelCountMismatch {
                expected: 1,
                actual: 2,
            }
        );

        for (from, to, role, expected, actual) in [
            (
                "77777777-2222-4333-8444-555555555555",
                "uid-x",
                StreamHandshakeIdentityRole::Uid,
                "77777777-2222-4333-8444-555555555555",
                "uid-x",
            ),
            (
                "<hostname>host</hostname>",
                "<hostname>host-x</hostname>",
                StreamHandshakeIdentityRole::Hostname,
                "host",
                "host-x",
            ),
            (
                "<source_id>source</source_id>",
                "<source_id>source-x</source_id>",
                StreamHandshakeIdentityRole::SourceId,
                "source",
                "source-x",
            ),
            (
                "<session_id>session</session_id>",
                "<session_id>session-x</session_id>",
                StreamHandshakeIdentityRole::SessionId,
                "session",
                "session-x",
            ),
        ] {
            assert_eq!(
                contract_failure(
                    document("127.0.0.1", 9, 1).replace(from, to),
                    &expected_identity,
                    1
                ),
                TypedUdpDiscoveryDouble64SessionConnectionError::IdentityMismatch {
                    role,
                    expected: expected.to_owned(),
                    actual: actual.to_owned(),
                }
            );
        }
    }

    #[test]
    fn p57_named_discovery_completes_double64_bits_and_reuses_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = listener.local_addr().unwrap();
        let sent = records(2, 3);
        let expected: Vec<Vec<u64>> = sent
            .iter()
            .map(|record| {
                record
                    .sample()
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect()
            })
            .collect();
        let outlet = thread::spawn(move || {
            TimestampedDouble64OutletSession::preflight_bounded(
                session_activation(),
                listener,
                &identity(),
                handshake_limits(),
                io_limits(),
                TimestampedDouble64SessionLimits::new(2, 3).unwrap(),
                &sent,
            )
            .unwrap()
            .finish(&AtomicBool::new(false))
            .unwrap()
        });
        let report = run_named(
            document("127.0.0.1", endpoint.port(), 2),
            "selected",
            2,
            3,
            &AtomicBool::new(false),
        )
        .unwrap();
        let actual: Vec<Vec<u64>> = report
            .records()
            .iter()
            .map(|record| {
                record
                    .sample()
                    .values()
                    .iter()
                    .map(|value| value.to_bits())
                    .collect()
            })
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(outlet.join().unwrap().record_count(), 3);
        TcpListener::bind(endpoint).unwrap();
    }

    #[test]
    fn p57_double64_empty_name_no_match_and_session_cancellation_are_typed() {
        assert!(matches!(
            run_named(
                document("127.0.0.1", 9, 1),
                "",
                1,
                1,
                &AtomicBool::new(false)
            ),
            Err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Selection(
                TypedUdpDiscoverySelectionError::EmptyStreamName
            ))
        ));
        assert!(matches!(
            run_named(document("127.0.0.1", 9, 1), "absent", 1, 1, &AtomicBool::new(false)),
            Err(TypedUdpDiscoveryDouble64CompleteLifecycleError::NoMatchingStreamName { stream_name, .. })
                if stream_name == "absent"
        ));
        assert!(matches!(
            run_named(
                document("127.0.0.1", 9, 1),
                "selected",
                1,
                1,
                &AtomicBool::new(true)
            ),
            Err(TypedUdpDiscoveryDouble64CompleteLifecycleError::Connection(
                TypedUdpDiscoveryDouble64SessionConnectionError::Session(_)
            ))
        ));
    }
}
