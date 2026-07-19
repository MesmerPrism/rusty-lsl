// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit, caller-configured, bounded UDP discovery activation.

use crate::{
    ParsedShortInfoResponseEnvelope, RuntimeModule, RuntimeModuleCapability, ShortInfoQueryWire,
    ShortInfoResponseEnvelopeLimits, ShortInfoResponseEnvelopeParseError,
};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Feature identity selected by the LSLC-002P lock.
pub const UDP_DISCOVERY_FEATURE_ID: &str = "udp-discovery";
/// Effective marker required as explicit runtime input.
pub const UDP_DISCOVERY_EFFECTIVE_MARKER: &str = "rusty.lsl.udp_discovery.effective";

/// Nominal proof that the caller supplied the selected feature and runtime marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UdpDiscoveryActivation {
    private: (),
}

impl UdpDiscoveryActivation {
    /// Admits only the exact selected feature identity and effective marker.
    pub fn new(capability: RuntimeModuleCapability) -> Result<Self, UdpDiscoveryActivationError> {
        if !capability.matches(RuntimeModule::UdpDiscovery) {
            return Err(UdpDiscoveryActivationError::WrongModule);
        }
        Ok(Self { private: () })
    }
}

/// Rejected explicit runtime activation input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UdpDiscoveryActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}

/// Nonzero finite resource and time limits for one discovery call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UdpDiscoveryLimits {
    max_datagram_bytes: usize,
    max_responses: usize,
    receive_slice: Duration,
    total_deadline: Duration,
}

impl UdpDiscoveryLimits {
    /// Creates explicit finite limits in deterministic argument order.
    pub fn new(
        max_datagram_bytes: usize,
        max_responses: usize,
        receive_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, UdpDiscoveryLimitError> {
        if max_datagram_bytes == 0 {
            return Err(UdpDiscoveryLimitError::ZeroDatagramBytes);
        }
        if max_responses == 0 {
            return Err(UdpDiscoveryLimitError::ZeroResponses);
        }
        if receive_slice.is_zero() {
            return Err(UdpDiscoveryLimitError::ZeroReceiveSlice);
        }
        if total_deadline.is_zero() {
            return Err(UdpDiscoveryLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            max_datagram_bytes,
            max_responses,
            receive_slice,
            total_deadline,
        })
    }

    /// Maximum bytes admitted from one datagram.
    pub const fn max_datagram_bytes(self) -> usize {
        self.max_datagram_bytes
    }
    /// Maximum admitted response count.
    pub const fn max_responses(self) -> usize {
        self.max_responses
    }
    /// Maximum duration of one blocking receive slice.
    pub const fn receive_slice(self) -> Duration {
        self.receive_slice
    }
    /// Total duration allowed after the query send begins.
    pub const fn total_deadline(self) -> Duration {
        self.total_deadline
    }
}

/// Invalid discovery limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UdpDiscoveryLimitError {
    /// Datagram maximum was zero.
    ZeroDatagramBytes,
    /// Response maximum was zero.
    ZeroResponses,
    /// Receive slice was zero.
    ZeroReceiveSlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
}

/// Explicit caller-owned socket configuration for one call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UdpDiscoveryConfig {
    bind_address: SocketAddr,
    destination: SocketAddr,
    limits: UdpDiscoveryLimits,
    envelope_limits: ShortInfoResponseEnvelopeLimits,
}

impl UdpDiscoveryConfig {
    /// Retains caller selections without interface or endpoint discovery.
    pub const fn new(
        bind_address: SocketAddr,
        destination: SocketAddr,
        limits: UdpDiscoveryLimits,
        envelope_limits: ShortInfoResponseEnvelopeLimits,
    ) -> Self {
        Self {
            bind_address,
            destination,
            limits,
            envelope_limits,
        }
    }
    /// Caller-selected local bind address.
    pub const fn bind_address(self) -> SocketAddr {
        self.bind_address
    }
    /// Caller-selected destination.
    pub const fn destination(self) -> SocketAddr {
        self.destination
    }
    /// Caller-selected runtime limits.
    pub const fn limits(self) -> UdpDiscoveryLimits {
        self.limits
    }
    /// Caller-selected response-envelope limits.
    pub const fn envelope_limits(self) -> ShortInfoResponseEnvelopeLimits {
        self.envelope_limits
    }
}

/// Why one bounded call stopped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UdpDiscoveryTermination {
    /// Caller cancellation was observed.
    Cancelled,
    /// The total deadline elapsed.
    Deadline,
    /// The response-count bound was reached.
    ResponseLimit,
}

/// One owned validated response datagram and its observed source.
#[derive(Debug, Eq, PartialEq)]
pub struct UdpDiscoveryResponse {
    source: SocketAddr,
    query_id: u64,
    bytes: Vec<u8>,
}

impl UdpDiscoveryResponse {
    /// Observed datagram source; this grants no endpoint ownership.
    pub const fn source(&self) -> SocketAddr {
        self.source
    }
    /// Uninterpreted identifier parsed from the admitted envelope.
    pub const fn query_id(&self) -> u64 {
        self.query_id
    }
    /// Unchanged datagram bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Recovers the unchanged allocation.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Completed bounded call state.
#[derive(Debug, Eq, PartialEq)]
pub struct UdpDiscoveryRun {
    local_address: SocketAddr,
    termination: UdpDiscoveryTermination,
    responses: Vec<UdpDiscoveryResponse>,
}

impl UdpDiscoveryRun {
    /// Actual local address assigned to the owned socket during the call.
    pub const fn local_address(&self) -> SocketAddr {
        self.local_address
    }
    /// Bounded stop reason.
    pub const fn termination(&self) -> UdpDiscoveryTermination {
        self.termination
    }
    /// Admitted responses in receive order.
    pub fn responses(&self) -> &[UdpDiscoveryResponse] {
        &self.responses
    }
    /// Recovers response allocations.
    pub fn into_responses(self) -> Vec<UdpDiscoveryResponse> {
        self.responses
    }
}

/// Stable failure from socket setup, I/O, sizing, or response admission.
#[derive(Debug, Eq, PartialEq)]
pub enum UdpDiscoveryError {
    /// Socket bind failed.
    Bind(ErrorKind),
    /// Reading the assigned local address failed.
    LocalAddress(ErrorKind),
    /// Configuring the receive timeout failed.
    ReceiveTimeout(ErrorKind),
    /// Sending the query failed.
    Send(ErrorKind),
    /// A successful send reported a partial datagram.
    PartialSend {
        /// Query datagram byte count.
        expected: usize,
        /// Reported sent byte count.
        actual: usize,
    },
    /// Receiving a datagram failed.
    Receive(ErrorKind),
    /// The receive buffer allocation failed.
    DatagramAllocationFailed {
        /// Requested byte count.
        requested: usize,
    },
    /// The one-byte oversize probe could not be represented.
    DatagramProbeLengthOverflow,
    /// The response collection allocation failed.
    ResponseAllocationFailed {
        /// Requested response capacity.
        requested: usize,
    },
    /// The datagram exceeded the selected maximum.
    DatagramLimitExceeded {
        /// Selected buffer limit.
        limit: usize,
        /// Observed bytes, bounded to exactly one past the limit.
        actual: usize,
    },
    /// A datagram was not UTF-8.
    InvalidUtf8 {
        /// First byte not covered by the valid UTF-8 prefix.
        valid_up_to: usize,
    },
    /// A datagram failed the accepted response-envelope contract.
    InvalidEnvelope(ShortInfoResponseEnvelopeParseError),
}

/// Executes one explicitly configured UDP discovery exchange.
pub fn run_udp_discovery(
    _activation: UdpDiscoveryActivation,
    config: UdpDiscoveryConfig,
    query: &ShortInfoQueryWire,
    cancelled: &AtomicBool,
) -> Result<UdpDiscoveryRun, UdpDiscoveryError> {
    if cancelled.load(Ordering::Acquire) {
        return Ok(UdpDiscoveryRun {
            local_address: config.bind_address,
            termination: UdpDiscoveryTermination::Cancelled,
            responses: Vec::new(),
        });
    }

    let socket = UdpSocket::bind(config.bind_address)
        .map_err(|error| UdpDiscoveryError::Bind(error.kind()))?;
    let local_address = socket
        .local_addr()
        .map_err(|error| UdpDiscoveryError::LocalAddress(error.kind()))?;
    let sent = socket
        .send_to(query.as_bytes(), config.destination)
        .map_err(|error| UdpDiscoveryError::Send(error.kind()))?;
    if sent != query.as_bytes().len() {
        return Err(UdpDiscoveryError::PartialSend {
            expected: query.as_bytes().len(),
            actual: sent,
        });
    }

    let probe_bytes = config
        .limits
        .max_datagram_bytes
        .checked_add(1)
        .ok_or(UdpDiscoveryError::DatagramProbeLengthOverflow)?;
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(probe_bytes).map_err(|_| {
        UdpDiscoveryError::DatagramAllocationFailed {
            requested: probe_bytes,
        }
    })?;
    buffer.resize(probe_bytes, 0);
    let mut responses = Vec::new();
    responses
        .try_reserve_exact(config.limits.max_responses)
        .map_err(|_| UdpDiscoveryError::ResponseAllocationFailed {
            requested: config.limits.max_responses,
        })?;

    let started = Instant::now();
    loop {
        if cancelled.load(Ordering::Acquire) {
            return Ok(UdpDiscoveryRun {
                local_address,
                termination: UdpDiscoveryTermination::Cancelled,
                responses,
            });
        }
        if responses.len() == config.limits.max_responses {
            return Ok(UdpDiscoveryRun {
                local_address,
                termination: UdpDiscoveryTermination::ResponseLimit,
                responses,
            });
        }
        let Some(remaining) = config.limits.total_deadline.checked_sub(started.elapsed()) else {
            return Ok(UdpDiscoveryRun {
                local_address,
                termination: UdpDiscoveryTermination::Deadline,
                responses,
            });
        };
        let receive_timeout = remaining.min(config.limits.receive_slice);
        if receive_timeout.is_zero() {
            return Ok(UdpDiscoveryRun {
                local_address,
                termination: UdpDiscoveryTermination::Deadline,
                responses,
            });
        }
        socket
            .set_read_timeout(Some(receive_timeout))
            .map_err(|error| UdpDiscoveryError::ReceiveTimeout(error.kind()))?;
        let (length, source) = match socket.recv_from(&mut buffer) {
            Ok(value) => value,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                continue;
            }
            Err(error) => return Err(UdpDiscoveryError::Receive(error.kind())),
        };
        if length > config.limits.max_datagram_bytes {
            return Err(UdpDiscoveryError::DatagramLimitExceeded {
                limit: config.limits.max_datagram_bytes,
                actual: length,
            });
        }
        let text = std::str::from_utf8(&buffer[..length]).map_err(|error| {
            UdpDiscoveryError::InvalidUtf8 {
                valid_up_to: error.valid_up_to(),
            }
        })?;
        let parsed = ParsedShortInfoResponseEnvelope::parse(text, config.envelope_limits)
            .map_err(UdpDiscoveryError::InvalidEnvelope)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(length)
            .map_err(|_| UdpDiscoveryError::DatagramAllocationFailed { requested: length })?;
        bytes.extend_from_slice(&buffer[..length]);
        responses.push(UdpDiscoveryResponse {
            source,
            query_id: parsed.query_id(),
            bytes,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{ShortInfoQuery, ShortInfoQueryWireLimits, StreamInfoObservedDocumentParseLimit};
    use std::net::Ipv4Addr;
    use std::sync::Arc;
    use std::thread;

    fn activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap()
    }

    fn body() -> String {
        let names = [
            "name",
            "type",
            "channel_count",
            "channel_format",
            "source_id",
            "nominal_srate",
            "version",
            "created_at",
            "uid",
            "session_id",
            "hostname",
            "v4address",
            "v4data_port",
            "v4service_port",
            "v6address",
            "v6data_port",
            "v6service_port",
        ];
        let mut text = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for name in names {
            text.push_str(&format!("\t<{name}>x</{name}>\n"));
        }
        text.push_str("\t<desc />\n</info>\n");
        text
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQuery::new("loopback".to_owned(), 1, 7, limits).unwrap();
        ShortInfoQueryWire::encode(&query, limits).unwrap()
    }

    fn config(
        bind_address: SocketAddr,
        destination: SocketAddr,
        max_datagram_bytes: usize,
        max_responses: usize,
        receive_slice: Duration,
        total_deadline: Duration,
    ) -> UdpDiscoveryConfig {
        let document_bytes = body().len();
        UdpDiscoveryConfig::new(
            bind_address,
            destination,
            UdpDiscoveryLimits::new(
                max_datagram_bytes,
                max_responses,
                receive_slice,
                total_deadline,
            )
            .unwrap(),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
        )
    }

    fn response() -> Vec<u8> {
        let text = format!("7\r\n{}", body());
        ParsedShortInfoResponseEnvelope::parse(
            &text,
            ShortInfoResponseEnvelopeLimits::new(body().len(), text.len()).unwrap(),
        )
        .unwrap();
        text.into_bytes()
    }

    #[test]
    fn lslc_002p_limits_reject_in_argument_order() {
        let one = Duration::from_millis(1);
        assert_eq!(
            UdpDiscoveryLimits::new(0, 0, Duration::ZERO, Duration::ZERO),
            Err(UdpDiscoveryLimitError::ZeroDatagramBytes)
        );
        assert_eq!(
            UdpDiscoveryLimits::new(1, 0, Duration::ZERO, Duration::ZERO),
            Err(UdpDiscoveryLimitError::ZeroResponses)
        );
        assert_eq!(
            UdpDiscoveryLimits::new(1, 1, Duration::ZERO, Duration::ZERO),
            Err(UdpDiscoveryLimitError::ZeroReceiveSlice)
        );
        assert_eq!(
            UdpDiscoveryLimits::new(1, 1, one, Duration::ZERO),
            Err(UdpDiscoveryLimitError::ZeroTotalDeadline)
        );
    }

    #[test]
    fn lslc_002p_loopback_admits_response_and_reaches_count_limit() {
        let server = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = server.local_addr().unwrap();
        let expected_query = query().as_bytes().to_vec();
        let response = response();
        let expected_response = response.clone();
        let response_len = response.len();
        let worker = thread::spawn(move || {
            let mut bytes = [0u8; 256];
            let (length, source) = server.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..length], expected_query);
            server.send_to(&response, source).unwrap();
        });
        let cancelled = AtomicBool::new(false);
        let run = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                response_len,
                1,
                Duration::from_millis(20),
                Duration::from_secs(1),
            ),
            &query(),
            &cancelled,
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert_eq!(run.responses().len(), 1);
        assert_eq!(run.responses()[0].query_id(), 7);
        assert_eq!(run.responses()[0].as_bytes(), expected_response.as_slice());
        assert!(run.responses()[0].source().ip().is_loopback());
    }

    #[test]
    fn lslc_002p_pre_send_and_blocked_receive_cancellation_are_bounded() {
        let destination: SocketAddr = "127.0.0.1:9".parse().unwrap();
        let cancelled = AtomicBool::new(true);
        let run = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                512,
                1,
                Duration::from_millis(5),
                Duration::from_secs(1),
            ),
            &query(),
            &cancelled,
        )
        .unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::Cancelled);
        assert!(run.responses().is_empty());

        let sink = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = sink.local_addr().unwrap();
        let cancellation = Arc::new(AtomicBool::new(false));
        let signal = Arc::clone(&cancellation);
        let worker = thread::spawn(move || {
            let mut bytes = [0u8; 256];
            sink.recv_from(&mut bytes).unwrap();
            thread::sleep(Duration::from_millis(10));
            signal.store(true, Ordering::Release);
        });
        let run = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                512,
                1,
                Duration::from_millis(5),
                Duration::from_secs(1),
            ),
            &query(),
            &cancellation,
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::Cancelled);
    }

    #[test]
    fn lslc_002p_deadline_releases_the_caller_selected_port() {
        let probe = UdpSocket::bind("127.0.0.1:0").unwrap();
        let bind_address = probe.local_addr().unwrap();
        drop(probe);
        let sink = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = sink.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let mut bytes = [0u8; 256];
            sink.recv_from(&mut bytes).unwrap();
        });
        let run = run_udp_discovery(
            activation(),
            config(
                bind_address,
                destination,
                512,
                1,
                Duration::from_millis(5),
                Duration::from_millis(20),
            ),
            &query(),
            &AtomicBool::new(false),
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::Deadline);
        let rebound = UdpSocket::bind(bind_address).unwrap();
        assert_eq!(rebound.local_addr().unwrap(), bind_address);
    }

    #[test]
    fn lslc_002p_invalid_utf8_and_one_past_datagram_reject() {
        for (payload, maximum, expected) in [
            (
                vec![0xff],
                8,
                UdpDiscoveryError::InvalidUtf8 { valid_up_to: 0 },
            ),
            (
                vec![b'x'; 9],
                8,
                UdpDiscoveryError::DatagramLimitExceeded {
                    limit: 8,
                    actual: 9,
                },
            ),
        ] {
            let server = UdpSocket::bind("127.0.0.1:0").unwrap();
            let destination = server.local_addr().unwrap();
            let worker = thread::spawn(move || {
                let mut bytes = [0u8; 256];
                let (_, source) = server.recv_from(&mut bytes).unwrap();
                server.send_to(&payload, source).unwrap();
            });
            let error = run_udp_discovery(
                activation(),
                config(
                    "127.0.0.1:0".parse().unwrap(),
                    destination,
                    maximum,
                    1,
                    Duration::from_millis(20),
                    Duration::from_secs(1),
                ),
                &query(),
                &AtomicBool::new(false),
            )
            .unwrap_err();
            worker.join().unwrap();
            assert_eq!(error, expected);
        }
    }

    #[test]
    fn lslc_002p_invalid_envelope_is_delegated_unchanged() {
        let server = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = server.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let mut bytes = [0u8; 256];
            let (_, source) = server.recv_from(&mut bytes).unwrap();
            server.send_to(b"7\ninvalid", source).unwrap();
        });
        let error = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                64,
                1,
                Duration::from_millis(20),
                Duration::from_secs(1),
            ),
            &query(),
            &AtomicBool::new(false),
        )
        .unwrap_err();
        worker.join().unwrap();
        assert_eq!(
            error,
            UdpDiscoveryError::InvalidEnvelope(
                ShortInfoResponseEnvelopeParseError::InvalidDelimiter { offset: 1 }
            )
        );
    }

    #[test]
    fn lslc_002p_mismatched_activation_rejects_before_runtime() {
        assert_eq!(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::StreamHandshake)),
            Err(UdpDiscoveryActivationError::WrongModule)
        );
    }

    #[test]
    fn lslc_002p_response_bytes_remain_parseable_after_socket_cleanup() {
        let response = response();
        let text = std::str::from_utf8(&response).unwrap();
        let limits = ShortInfoResponseEnvelopeLimits::new(
            StreamInfoObservedDocumentParseLimit::new(body().len())
                .unwrap()
                .max_input_bytes(),
            response.len(),
        )
        .unwrap();
        assert_eq!(
            ParsedShortInfoResponseEnvelope::parse(text, limits)
                .unwrap()
                .query_id(),
            7
        );
    }

    #[test]
    fn lslc_004d_explicit_loopback_requester_composes_with_one_joined_peer() {
        let _multicast_test_lock = crate::MULTICAST_LOOPBACK_TEST_LOCK.lock().unwrap();
        const GROUP: Ipv4Addr = Ipv4Addr::new(239, 255, 172, 215);
        const INTERFACE: Ipv4Addr = Ipv4Addr::LOCALHOST;
        const PORT: u16 = 16_571;

        let peer = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT)).unwrap();
        peer.join_multicast_v4(&GROUP, &INTERFACE).unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
        let expected_query = query().as_bytes().to_vec();
        let response = response();
        let expected_response = response.clone();
        let response_len = response.len();
        let worker = thread::spawn(move || {
            let mut bytes = [0u8; 256];
            let (length, source) = peer.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..length], expected_query);
            assert_eq!(source.ip(), std::net::IpAddr::V4(INTERFACE));
            peer.send_to(&response, source).unwrap();
            peer.leave_multicast_v4(&GROUP, &INTERFACE).unwrap();
        });

        let run = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                SocketAddr::from((GROUP, PORT)),
                response_len,
                1,
                Duration::from_millis(20),
                Duration::from_secs(1),
            ),
            &query(),
            &AtomicBool::new(false),
        )
        .unwrap();
        worker.join().unwrap();

        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert_eq!(run.responses().len(), 1);
        assert_eq!(run.responses()[0].query_id(), 7);
        assert_eq!(run.responses()[0].as_bytes(), expected_response.as_slice());
        assert!(run.responses()[0].source().ip().is_loopback());

        let rebound = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT)).unwrap();
        assert_eq!(rebound.local_addr().unwrap().port(), PORT);
    }

    #[test]
    #[ignore = "requires the private pinned official LSLC-004P outlet harness"]
    fn lslc_004p_private_official_outlet_observation_driver() {
        const GROUP: Ipv4Addr = Ipv4Addr::new(239, 255, 172, 215);
        const PORT: u16 = 16_571;
        let interface: Ipv4Addr = std::env::var("RLSL_LSLC_004P_INTERFACE")
            .expect("private driver supplies an explicit active IPv4 interface")
            .parse()
            .expect("private interface is IPv4");
        let reply_port: u16 = std::env::var("RLSL_LSLC_004P_REPLY_PORT")
            .expect("private driver supplies the reserved reply port")
            .parse()
            .expect("private reply port is u16");
        let expected_source = std::env::var("RLSL_LSLC_004P_SOURCE_TOKEN")
            .expect("private driver supplies the independently selected source token");
        let query_id = 9_223_372_036_854_775_123_u64;
        let query_limits = ShortInfoQueryWireLimits::new(20, 128).unwrap();
        let query = ShortInfoQuery::new(
            "session_id='default'".to_owned(),
            reply_port,
            query_id,
            query_limits,
        )
        .unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        let run = run_udp_discovery(
            activation(),
            UdpDiscoveryConfig::new(
                SocketAddr::from((interface, reply_port)),
                SocketAddr::from((GROUP, PORT)),
                UdpDiscoveryLimits::new(
                    65_535,
                    1,
                    Duration::from_millis(25),
                    Duration::from_secs(3),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(65_500, 65_535).unwrap(),
            ),
            &wire,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(
            run.local_address(),
            SocketAddr::from((interface, reply_port))
        );
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert_eq!(run.responses().len(), 1);
        assert_eq!(run.responses()[0].query_id(), query_id);
        assert_eq!(run.responses()[0].source().ip(), interface);
        let text = std::str::from_utf8(run.responses()[0].as_bytes()).unwrap();
        assert!(text.contains(&expected_source));
        drop(run);
        assert!(UdpSocket::bind((interface, reply_port)).is_ok());
    }

    fn lslc_004s_independent_document() -> String {
        let names = [
            "name",
            "type",
            "channel_count",
            "channel_format",
            "source_id",
            "nominal_srate",
            "version",
            "created_at",
            "uid",
            "session_id",
            "hostname",
            "v4address",
            "v4data_port",
            "v4service_port",
            "v6address",
            "v6data_port",
            "v6service_port",
        ];
        let mut values = [
            "independent-alpha".to_owned(),
            "independent-beta".to_owned(),
            "1".to_owned(),
            "float32".to_owned(),
            "fresh-source-token".to_owned(),
            "0".to_owned(),
            "110".to_owned(),
            "1234.5".to_owned(),
            "fresh-uid-token".to_owned(),
            "independent-session".to_owned(),
            "independent-host".to_owned(),
            "203.0.113.7".to_owned(),
            "41001".to_owned(),
            "41002".to_owned(),
            "2001:db8::7".to_owned(),
            "41003".to_owned(),
            "41004".to_owned(),
        ];
        let render = |values: &[String; 17]| {
            let mut text = String::from("<?xml version=\"1.0\"?>\n<info>\n");
            for (name, value) in names.into_iter().zip(values) {
                text.push_str(&format!("\t<{name}>{value}</{name}>\n"));
            }
            text.push_str("\t<desc />\n</info>\n");
            text
        };
        let initial = render(&values);
        values[10].push_str(&"q".repeat(711 - initial.len()));
        let document = render(&values);
        assert_eq!(document.len(), 711);
        document
    }

    #[test]
    fn lslc_004s_independent_public_structure_composes_with_unchanged_requester() {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = peer.local_addr().unwrap();
        let query_id = 4_321_098_765_432_109_876_u64;
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQuery::new("fresh".to_owned(), 41_111, query_id, limits).unwrap();
        let wire = ShortInfoQueryWire::encode(&query, limits).unwrap();
        let expected_query = wire.as_bytes().to_vec();
        let response = format!("{query_id}\r\n{}", lslc_004s_independent_document()).into_bytes();
        assert_eq!(response.len(), 732);
        let expected_response = response.clone();
        let worker = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (length, source) = peer.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..length], expected_query);
            peer.send_to(&response, source).unwrap();
        });
        let bind_probe = UdpSocket::bind("127.0.0.1:0").unwrap();
        let bind_address = bind_probe.local_addr().unwrap();
        drop(bind_probe);
        let run = run_udp_discovery(
            activation(),
            UdpDiscoveryConfig::new(
                bind_address,
                destination,
                UdpDiscoveryLimits::new(732, 1, Duration::from_millis(10), Duration::from_secs(1))
                    .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(711, 732).unwrap(),
            ),
            &wire,
            &AtomicBool::new(false),
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert_eq!(run.responses()[0].query_id(), query_id);
        assert_eq!(run.responses()[0].as_bytes(), expected_response);
        assert_eq!(run.responses()[0].source(), destination);
        drop(run);
        assert!(UdpSocket::bind(bind_address).is_ok());

        let sink = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = sink.local_addr().unwrap();
        let deadline_worker = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            sink.recv_from(&mut bytes).unwrap();
        });
        let deadline = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                732,
                1,
                Duration::from_millis(5),
                Duration::from_millis(20),
            ),
            &wire,
            &AtomicBool::new(false),
        )
        .unwrap();
        deadline_worker.join().unwrap();
        assert_eq!(deadline.termination(), UdpDiscoveryTermination::Deadline);
        let cancelled = AtomicBool::new(true);
        let cancelled_run = run_udp_discovery(
            activation(),
            config(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                732,
                1,
                Duration::from_millis(5),
                Duration::from_millis(20),
            ),
            &wire,
            &cancelled,
        )
        .unwrap();
        assert_eq!(
            cancelled_run.termination(),
            UdpDiscoveryTermination::Cancelled
        );
    }

    #[test]
    fn lslc_004s_public_structure_damage_rejects() {
        let query_id = 4_321_098_765_432_109_876_u64;
        let document = lslc_004s_independent_document();
        let accepted = format!("{query_id}\r\n{document}");
        let limits = ShortInfoResponseEnvelopeLimits::new(711, 732).unwrap();
        assert!(ParsedShortInfoResponseEnvelope::parse(&accepted, limits).is_ok());
        for damaged in [
            accepted.replacen("\r\n", "\n", 1),
            accepted.replacen("<name>", "<type>", 1),
            accepted.replacen("<type>", "<name>", 1),
            accepted.replacen("<v6service_port>", "<v7service_port>", 1),
            accepted.trim_end_matches('\n').to_owned(),
        ] {
            assert!(ParsedShortInfoResponseEnvelope::parse(
                &damaged,
                ShortInfoResponseEnvelopeLimits::new(711, 732).unwrap(),
            )
            .is_err());
        }
        let one_past = format!("{accepted}x");
        assert!(ParsedShortInfoResponseEnvelope::parse(&one_past, limits).is_err());
    }

    fn lslc_004t_admission_limits() -> crate::StreamInfoObservedAdmissionLimits {
        crate::StreamInfoObservedAdmissionLimits::new(
            crate::StreamDescriptorLimits::new(64, 64, 64, 4).unwrap(),
            crate::MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
            crate::StreamInfoVolatileFieldLimits::new(1024, 64, 64).unwrap(),
        )
    }

    #[test]
    fn lslc_004t_requester_response_composes_with_existing_typed_observation() {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = peer.local_addr().unwrap();
        let query_id = 4_321_098_765_432_109_876_u64;
        let query_limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query =
            ShortInfoQuery::new("typed".to_owned(), 41_112, query_id, query_limits).unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        let expected_query = wire.as_bytes().to_vec();
        let typed_document = lslc_004s_independent_document()
            .replacen(
                "<nominal_srate>0</nominal_srate>",
                "<nominal_srate>100.0000000000000</nominal_srate>",
                1,
            )
            .replacen("qqqqqqqqqqqqqqqqq", "q", 1);
        assert_eq!(typed_document.len(), 711);
        let response = format!("{query_id}\r\n{typed_document}").into_bytes();
        let worker = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (length, source) = peer.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..length], expected_query);
            peer.send_to(&response, source).unwrap();
        });
        let bind_probe = UdpSocket::bind("127.0.0.1:0").unwrap();
        let bind_address = bind_probe.local_addr().unwrap();
        drop(bind_probe);
        let run = run_udp_discovery(
            activation(),
            UdpDiscoveryConfig::new(
                bind_address,
                destination,
                UdpDiscoveryLimits::new(732, 1, Duration::from_millis(10), Duration::from_secs(1))
                    .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(711, 732).unwrap(),
            ),
            &wire,
            &AtomicBool::new(false),
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        let retained = std::str::from_utf8(run.responses()[0].as_bytes()).unwrap();
        let parsed = ParsedShortInfoResponseEnvelope::parse(
            retained,
            ShortInfoResponseEnvelopeLimits::new(711, 732).unwrap(),
        )
        .unwrap();
        let typed =
            crate::TypedShortInfoResponseObservation::admit(parsed, lslc_004t_admission_limits())
                .unwrap();
        assert_eq!(typed.query_id(), query_id);
        assert_eq!(
            typed.fields().definition().descriptor().name(),
            "independent-alpha"
        );
        assert_eq!(typed.fields().definition().descriptor().channel_count(), 1);
        assert_eq!(run.responses()[0].source(), destination);
        drop(run);
        assert!(UdpSocket::bind(bind_address).is_ok());

        let damaged_document = lslc_004s_independent_document().replacen(
            "<channel_count>1</channel_count>",
            "<channel_count>01</channel_count>",
            1,
        );
        let damaged = format!("{query_id}\r\n{damaged_document}");
        let envelope = ParsedShortInfoResponseEnvelope::parse(
            &damaged,
            ShortInfoResponseEnvelopeLimits::new(damaged_document.len(), damaged.len()).unwrap(),
        )
        .unwrap();
        assert_eq!(
            crate::TypedShortInfoResponseObservation::admit(envelope, lslc_004t_admission_limits(),),
            Err(crate::TypedShortInfoResponseObservationError::Admission(
                crate::StreamInfoObservedAdmissionError::InvalidChannelCount
            ))
        );
    }

    fn lslc_006c_response_for(query_id: u64) -> Vec<u8> {
        let text = format!("{query_id}\r\n{}", body());
        ParsedShortInfoResponseEnvelope::parse(
            &text,
            ShortInfoResponseEnvelopeLimits::new(body().len(), text.len()).unwrap(),
        )
        .unwrap();
        text.into_bytes()
    }

    #[test]
    fn lslc_006c_receive_order_identity_ownership_and_caller_port_cleanup() {
        let first_peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let first_source = first_peer.local_addr().unwrap();
        let second_peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let second_source = second_peer.local_addr().unwrap();
        assert_ne!(first_source, second_source);
        let first_bytes = lslc_006c_response_for(17);
        let second_bytes = lslc_006c_response_for(29);
        let expected_first = first_bytes.clone();
        let expected_second = second_bytes.clone();
        let expected_query = query().as_bytes().to_vec();
        let worker = thread::spawn(move || {
            let mut bytes = [0_u8; 256];
            let (length, requester) = first_peer.recv_from(&mut bytes).unwrap();
            assert_eq!(&bytes[..length], expected_query);
            first_peer.send_to(&first_bytes, requester).unwrap();
            thread::sleep(Duration::from_millis(20));
            second_peer.send_to(&second_bytes, requester).unwrap();
        });
        let bind_probe = UdpSocket::bind("127.0.0.1:0").unwrap();
        let bind_address = bind_probe.local_addr().unwrap();
        drop(bind_probe);
        let run = run_udp_discovery(
            activation(),
            config(
                bind_address,
                first_source,
                expected_second.len(),
                2,
                Duration::from_millis(50),
                Duration::from_secs(1),
            ),
            &query(),
            &AtomicBool::new(false),
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(run.local_address(), bind_address);
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert_eq!(
            run.responses()
                .iter()
                .map(|response| (response.source(), response.query_id(), response.as_bytes()))
                .collect::<Vec<_>>(),
            vec![
                (first_source, 17, expected_first.as_slice()),
                (second_source, 29, expected_second.as_slice()),
            ]
        );
        let response_allocation = run.responses().as_ptr();
        let byte_allocations = [
            run.responses()[0].as_bytes().as_ptr(),
            run.responses()[1].as_bytes().as_ptr(),
        ];
        let responses = run.into_responses();
        assert_eq!(responses.as_ptr(), response_allocation);
        let mut responses = responses.into_iter();
        let first = responses.next().unwrap().into_bytes();
        let second = responses.next().unwrap().into_bytes();
        assert!(responses.next().is_none());
        assert_eq!(first.as_ptr(), byte_allocations[0]);
        assert_eq!(second.as_ptr(), byte_allocations[1]);
        assert_eq!(first, expected_first);
        assert_eq!(second, expected_second);
        assert_eq!(
            UdpSocket::bind(bind_address).unwrap().local_addr().unwrap(),
            bind_address
        );
    }

    #[test]
    fn lslc_006c_pre_cancel_precedes_socket_and_response_limit_work() {
        let occupied = UdpSocket::bind("127.0.0.1:0").unwrap();
        let bind_address = occupied.local_addr().unwrap();
        let cancelled = AtomicBool::new(true);
        let run = run_udp_discovery(
            activation(),
            config(
                bind_address,
                "127.0.0.1:9".parse().unwrap(),
                response().len(),
                1,
                Duration::from_millis(1),
                Duration::from_millis(1),
            ),
            &query(),
            &cancelled,
        )
        .unwrap();
        assert_eq!(run.local_address(), bind_address);
        assert_eq!(run.termination(), UdpDiscoveryTermination::Cancelled);
        assert!(run.responses().is_empty());
    }
}
