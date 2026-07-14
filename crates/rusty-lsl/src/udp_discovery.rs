// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit, caller-configured, bounded UDP discovery activation.

use crate::{
    ParsedShortInfoResponseEnvelope, ShortInfoQueryWire, ShortInfoResponseEnvelopeLimits,
    ShortInfoResponseEnvelopeParseError,
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
    pub fn new(
        feature_id: &str,
        effective_marker: &str,
    ) -> Result<Self, UdpDiscoveryActivationError> {
        if feature_id != UDP_DISCOVERY_FEATURE_ID {
            return Err(UdpDiscoveryActivationError::FeatureMismatch);
        }
        if effective_marker != UDP_DISCOVERY_EFFECTIVE_MARKER {
            return Err(UdpDiscoveryActivationError::EffectiveMarkerMismatch);
        }
        Ok(Self { private: () })
    }
}

/// Rejected explicit runtime activation input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UdpDiscoveryActivationError {
    /// The selected feature identity did not match.
    FeatureMismatch,
    /// The effective marker did not match.
    EffectiveMarkerMismatch,
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
    use crate::{ShortInfoQuery, ShortInfoQueryWireLimits, StreamInfoObservedDocumentParseLimit};
    use std::sync::Arc;
    use std::thread;

    fn activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(UDP_DISCOVERY_FEATURE_ID, UDP_DISCOVERY_EFFECTIVE_MARKER)
            .unwrap()
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
            UdpDiscoveryActivation::new("other-feature", UDP_DISCOVERY_EFFECTIVE_MARKER),
            Err(UdpDiscoveryActivationError::FeatureMismatch)
        );
        assert_eq!(
            UdpDiscoveryActivation::new(UDP_DISCOVERY_FEATURE_ID, "other.marker"),
            Err(UdpDiscoveryActivationError::EffectiveMarkerMismatch)
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
}
