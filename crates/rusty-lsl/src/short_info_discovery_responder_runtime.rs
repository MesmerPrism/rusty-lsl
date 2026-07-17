// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Finite caller-configured UDP short-info response activation.

use crate::{
    ParsedShortInfoQuery, ParsedStreamInfoObservedDocument, RuntimeModule, RuntimeModuleCapability,
    ShortInfoQueryParseError, ShortInfoQueryWireLimits, ShortInfoResponseEnvelope,
    ShortInfoResponseEnvelopeEncodeError, ShortInfoResponseEnvelopeLimits,
};
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Selected feature identity.
pub const SHORT_INFO_RESPONDER_FEATURE_ID: &str = "short-info-discovery-responder";
/// Explicit effective marker.
pub const SHORT_INFO_RESPONDER_EFFECTIVE_MARKER: &str =
    "rusty.lsl.short_info_discovery_responder.effective";
/// Exact LSLC-004C-observed IPv4 multicast group.
pub const DOCUMENTED_IPV4_MULTICAST_GROUP: Ipv4Addr = Ipv4Addr::new(239, 255, 172, 215);
/// Exact LSLC-004C-observed UDP discovery port.
pub const DOCUMENTED_IPV4_MULTICAST_PORT: u16 = 16_571;

/// Nominal proof of explicit activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortInfoResponderActivation(());
impl ShortInfoResponderActivation {
    /// Admits only the selected feature and marker.
    pub fn new(
        capability: RuntimeModuleCapability,
    ) -> Result<Self, ShortInfoResponderActivationError> {
        if !capability.matches(RuntimeModule::ShortInfoDiscoveryResponder) {
            return Err(ShortInfoResponderActivationError::WrongModule);
        }
        Ok(Self(()))
    }
}
/// Rejected activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShortInfoResponderActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}

/// Explicit finite call limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortInfoResponderLimits {
    max_datagram_bytes: usize,
    max_requests: usize,
    receive_slice: Duration,
    total_deadline: Duration,
}
impl ShortInfoResponderLimits {
    /// Creates nonzero limits in argument order.
    pub fn new(
        max_datagram_bytes: usize,
        max_requests: usize,
        receive_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, ShortInfoResponderLimitError> {
        if max_datagram_bytes == 0 {
            return Err(ShortInfoResponderLimitError::ZeroDatagramBytes);
        }
        if max_requests == 0 {
            return Err(ShortInfoResponderLimitError::ZeroRequests);
        }
        if receive_slice.is_zero() {
            return Err(ShortInfoResponderLimitError::ZeroReceiveSlice);
        }
        if total_deadline.is_zero() {
            return Err(ShortInfoResponderLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            max_datagram_bytes,
            max_requests,
            receive_slice,
            total_deadline,
        })
    }
}
/// Invalid limit declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShortInfoResponderLimitError {
    /// Datagram maximum was zero.
    ZeroDatagramBytes,
    /// Request maximum was zero.
    ZeroRequests,
    /// Receive slice was zero.
    ZeroReceiveSlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
}

/// Why the finite call stopped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShortInfoResponderTermination {
    /// Cancellation was observed.
    Cancelled,
    /// Deadline elapsed.
    Deadline,
    /// Request bound was reached.
    RequestLimit,
}
/// Completed call outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortInfoResponderRun {
    local_address: SocketAddr,
    termination: ShortInfoResponderTermination,
    requests: usize,
}
impl ShortInfoResponderRun {
    /// Bound local address.
    pub const fn local_address(self) -> SocketAddr {
        self.local_address
    }
    /// Accepted request count.
    pub const fn requests(self) -> usize {
        self.requests
    }
    /// Stop reason.
    pub const fn termination(self) -> ShortInfoResponderTermination {
        self.termination
    }
}

/// Stable bounded responder failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShortInfoResponderError {
    /// The exact multicast composition requires an explicit loopback interface.
    NonLoopbackMulticastInterface,
    /// The exact multicast composition requires a caller-explicit concrete
    /// unicast or loopback IPv4 interface address.
    NonConcreteMulticastInterface,
    /// Bind failed.
    Bind(ErrorKind),
    /// Joining the exact IPv4 multicast group failed.
    JoinMulticast(ErrorKind),
    /// Local-address read failed.
    LocalAddress(ErrorKind),
    /// Timeout setup failed.
    ReceiveTimeout(ErrorKind),
    /// Receive failed.
    Receive(ErrorKind),
    /// Send failed.
    Send(ErrorKind),
    /// Datagram was oversized.
    DatagramLimitExceeded {
        /// Selected maximum.
        limit: usize,
        /// Observed bounded size.
        actual: usize,
    },
    /// Query admission failed.
    InvalidQuery(ShortInfoQueryParseError),
    /// Response encoding failed.
    Response(ShortInfoResponseEnvelopeEncodeError),
    /// Allocation failed.
    AllocationFailed {
        /// Requested bytes.
        requested: usize,
    },
    /// Probe length overflowed.
    ProbeLengthOverflow,
    /// Datagram send was partial.
    PartialSend {
        /// Expected bytes.
        expected: usize,
        /// Sent bytes.
        actual: usize,
    },
}

/// Runs one synchronous caller-configured response loop.
pub fn run_short_info_responder(
    _activation: ShortInfoResponderActivation,
    bind: SocketAddr,
    limits: ShortInfoResponderLimits,
    query_limits: ShortInfoQueryWireLimits,
    response_limits: ShortInfoResponseEnvelopeLimits,
    body: &ParsedStreamInfoObservedDocument<'_>,
    cancelled: &AtomicBool,
) -> Result<ShortInfoResponderRun, ShortInfoResponderError> {
    if cancelled.load(Ordering::Acquire) {
        return Ok(ShortInfoResponderRun {
            local_address: bind,
            termination: ShortInfoResponderTermination::Cancelled,
            requests: 0,
        });
    }
    let socket = UdpSocket::bind(bind).map_err(|e| ShortInfoResponderError::Bind(e.kind()))?;
    let local_address = socket
        .local_addr()
        .map_err(|e| ShortInfoResponderError::LocalAddress(e.kind()))?;
    run_short_info_responder_on_socket(
        socket,
        local_address,
        limits,
        query_limits,
        response_limits,
        body,
        cancelled,
    )
}

/// Runs the existing bounded responder on the exact documented IPv4 multicast
/// destination and one caller-explicit loopback interface.
pub fn run_explicit_loopback_multicast_short_info_responder(
    activation: ShortInfoResponderActivation,
    interface: Ipv4Addr,
    limits: ShortInfoResponderLimits,
    query_limits: ShortInfoQueryWireLimits,
    response_limits: ShortInfoResponseEnvelopeLimits,
    body: &ParsedStreamInfoObservedDocument<'_>,
    cancelled: &AtomicBool,
) -> Result<ShortInfoResponderRun, ShortInfoResponderError> {
    if !interface.is_loopback() {
        return Err(ShortInfoResponderError::NonLoopbackMulticastInterface);
    }
    run_explicit_ipv4_multicast_short_info_responder(
        activation,
        interface,
        limits,
        query_limits,
        response_limits,
        body,
        cancelled,
    )
}

/// Runs the existing bounded responder on the exact documented IPv4 multicast
/// destination and one caller-explicit concrete IPv4 interface.
///
/// This entry point does not enumerate interfaces, select a default, or fall
/// back from the supplied address. Unspecified, multicast, and broadcast
/// interface values reject before socket I/O.
pub fn run_explicit_ipv4_multicast_short_info_responder(
    _activation: ShortInfoResponderActivation,
    interface: Ipv4Addr,
    limits: ShortInfoResponderLimits,
    query_limits: ShortInfoQueryWireLimits,
    response_limits: ShortInfoResponseEnvelopeLimits,
    body: &ParsedStreamInfoObservedDocument<'_>,
    cancelled: &AtomicBool,
) -> Result<ShortInfoResponderRun, ShortInfoResponderError> {
    if interface.is_unspecified() || interface.is_multicast() || interface == Ipv4Addr::BROADCAST {
        return Err(ShortInfoResponderError::NonConcreteMulticastInterface);
    }
    let destination = SocketAddr::new(
        IpAddr::V4(DOCUMENTED_IPV4_MULTICAST_GROUP),
        DOCUMENTED_IPV4_MULTICAST_PORT,
    );
    if cancelled.load(Ordering::Acquire) {
        return Ok(ShortInfoResponderRun {
            local_address: destination,
            termination: ShortInfoResponderTermination::Cancelled,
            requests: 0,
        });
    }
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DOCUMENTED_IPV4_MULTICAST_PORT))
        .map_err(|e| ShortInfoResponderError::Bind(e.kind()))?;
    socket
        .join_multicast_v4(&DOCUMENTED_IPV4_MULTICAST_GROUP, &interface)
        .map_err(|e| ShortInfoResponderError::JoinMulticast(e.kind()))?;
    let local_address = socket
        .local_addr()
        .map_err(|e| ShortInfoResponderError::LocalAddress(e.kind()))?;
    run_short_info_responder_on_socket(
        socket,
        local_address,
        limits,
        query_limits,
        response_limits,
        body,
        cancelled,
    )
}

fn run_short_info_responder_on_socket(
    socket: UdpSocket,
    local_address: SocketAddr,
    limits: ShortInfoResponderLimits,
    query_limits: ShortInfoQueryWireLimits,
    response_limits: ShortInfoResponseEnvelopeLimits,
    body: &ParsedStreamInfoObservedDocument<'_>,
    cancelled: &AtomicBool,
) -> Result<ShortInfoResponderRun, ShortInfoResponderError> {
    let probe = limits
        .max_datagram_bytes
        .checked_add(1)
        .ok_or(ShortInfoResponderError::ProbeLengthOverflow)?;
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(probe)
        .map_err(|_| ShortInfoResponderError::AllocationFailed { requested: probe })?;
    buffer.resize(probe, 0);
    let started = Instant::now();
    let mut requests = 0;
    loop {
        if cancelled.load(Ordering::Acquire) {
            return Ok(ShortInfoResponderRun {
                local_address,
                termination: ShortInfoResponderTermination::Cancelled,
                requests,
            });
        }
        if requests == limits.max_requests {
            return Ok(ShortInfoResponderRun {
                local_address,
                termination: ShortInfoResponderTermination::RequestLimit,
                requests,
            });
        }
        let Some(remaining) = limits.total_deadline.checked_sub(started.elapsed()) else {
            return Ok(ShortInfoResponderRun {
                local_address,
                termination: ShortInfoResponderTermination::Deadline,
                requests,
            });
        };
        let timeout = remaining.min(limits.receive_slice);
        if timeout.is_zero() {
            return Ok(ShortInfoResponderRun {
                local_address,
                termination: ShortInfoResponderTermination::Deadline,
                requests,
            });
        }
        socket
            .set_read_timeout(Some(timeout))
            .map_err(|e| ShortInfoResponderError::ReceiveTimeout(e.kind()))?;
        let (length, source) = match socket.recv_from(&mut buffer) {
            Ok(v) => v,
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => continue,
            Err(e) => return Err(ShortInfoResponderError::Receive(e.kind())),
        };
        if length > limits.max_datagram_bytes {
            return Err(ShortInfoResponderError::DatagramLimitExceeded {
                limit: limits.max_datagram_bytes,
                actual: length,
            });
        }
        let query = ParsedShortInfoQuery::parse(&buffer[..length], query_limits)
            .map_err(ShortInfoResponderError::InvalidQuery)?;
        let response = ShortInfoResponseEnvelope::encode(query.query_id(), body, response_limits)
            .map_err(ShortInfoResponderError::Response)?;
        let destination = SocketAddr::new(source.ip(), query.return_port());
        let sent = socket
            .send_to(response.as_bytes(), destination)
            .map_err(|e| ShortInfoResponderError::Send(e.kind()))?;
        if sent != response.as_bytes().len() {
            return Err(ShortInfoResponderError::PartialSend {
                expected: response.as_bytes().len(),
                actual: sent,
            });
        }
        requests += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        run_udp_discovery, ShortInfoQuery, ShortInfoQueryWire,
        StreamInfoObservedDocumentParseLimit, UdpDiscoveryActivation, UdpDiscoveryConfig,
        UdpDiscoveryLimits, UdpDiscoveryTermination,
    };
    use std::thread;

    fn activation() -> ShortInfoResponderActivation {
        ShortInfoResponderActivation::new(test_capability(
            RuntimeModule::ShortInfoDiscoveryResponder,
        ))
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

    fn limits(max: usize, requests: usize) -> ShortInfoResponderLimits {
        ShortInfoResponderLimits::new(
            max,
            requests,
            Duration::from_millis(10),
            Duration::from_secs(1),
        )
        .unwrap()
    }

    fn free_address() -> SocketAddr {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        socket.local_addr().unwrap()
    }

    fn run_prebound_short_info_responder(
        socket: UdpSocket,
        limits: ShortInfoResponderLimits,
        query_limits: ShortInfoQueryWireLimits,
        response_limits: ShortInfoResponseEnvelopeLimits,
        body: &ParsedStreamInfoObservedDocument<'_>,
    ) -> Result<ShortInfoResponderRun, ShortInfoResponderError> {
        let local_address = socket.local_addr().unwrap();
        run_short_info_responder_on_socket(
            socket,
            local_address,
            limits,
            query_limits,
            response_limits,
            body,
            &AtomicBool::new(false),
        )
    }

    #[test]
    fn lslc_002z_valid_query_returns_matching_envelope_and_releases_port() {
        let responder_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let address = responder_socket.local_addr().unwrap();
        let response_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        response_socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let response_port = response_socket.local_addr().unwrap().port();
        let text = body();
        let worker = thread::spawn(move || {
            let parsed = ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                &text,
            )
            .unwrap();
            run_prebound_short_info_responder(
                responder_socket,
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                &parsed,
            )
        });
        let query = ShortInfoQuery::new(
            "name='bounded'".into(),
            response_port,
            77,
            ShortInfoQueryWireLimits::new(128, 256).unwrap(),
        )
        .unwrap();
        let wire =
            ShortInfoQueryWire::encode(&query, ShortInfoQueryWireLimits::new(128, 256).unwrap())
                .unwrap();
        response_socket.send_to(wire.as_bytes(), address).unwrap();
        let mut bytes = [0_u8; 1024];
        let (count, _) = response_socket.recv_from(&mut bytes).unwrap();
        assert!(std::str::from_utf8(&bytes[..count])
            .unwrap()
            .starts_with("77\r\n"));
        assert_eq!(
            worker.join().unwrap().unwrap().termination(),
            ShortInfoResponderTermination::RequestLimit
        );
        assert!(UdpSocket::bind(address).is_ok());
    }

    #[test]
    fn lslc_002z_malformed_and_oversized_queries_fail_typed() {
        for (payload, expected_oversize) in
            [(b"damaged".as_slice(), false), (&[b'x'; 17][..], true)]
        {
            let responder_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
            let address = responder_socket.local_addr().unwrap();
            let text = body();
            let worker = thread::spawn(move || {
                let parsed = ParsedStreamInfoObservedDocument::parse(
                    StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                    &text,
                )
                .unwrap();
                run_prebound_short_info_responder(
                    responder_socket,
                    limits(16, 1),
                    ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                    ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                    &parsed,
                )
            });
            UdpSocket::bind("127.0.0.1:0")
                .unwrap()
                .send_to(payload, address)
                .unwrap();
            let error = worker.join().unwrap().unwrap_err();
            assert_eq!(
                matches!(error, ShortInfoResponderError::DatagramLimitExceeded { .. }),
                expected_oversize
            );
            if !expected_oversize {
                assert!(matches!(error, ShortInfoResponderError::InvalidQuery(_)));
            }
        }
    }

    #[test]
    fn lslc_002z_cancellation_deadline_limits_and_cleanup_are_finite() {
        assert_eq!(
            ShortInfoResponderLimits::new(0, 1, Duration::from_millis(1), Duration::from_millis(1)),
            Err(ShortInfoResponderLimitError::ZeroDatagramBytes)
        );
        let text = body();
        let parsed = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
            &text,
        )
        .unwrap();
        let cancelled = AtomicBool::new(true);
        let address = free_address();
        let run = run_short_info_responder(
            activation(),
            address,
            limits(256, 1),
            ShortInfoQueryWireLimits::new(128, 256).unwrap(),
            ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
            &parsed,
            &cancelled,
        )
        .unwrap();
        assert_eq!(run.termination(), ShortInfoResponderTermination::Cancelled);
        let short = ShortInfoResponderLimits::new(
            256,
            1,
            Duration::from_millis(2),
            Duration::from_millis(5),
        )
        .unwrap();
        let run = run_short_info_responder(
            activation(),
            address,
            short,
            ShortInfoQueryWireLimits::new(128, 256).unwrap(),
            ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
            &parsed,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(run.termination(), ShortInfoResponderTermination::Deadline);
        assert!(UdpSocket::bind(address).is_ok());
    }

    #[test]
    fn lslc_004e_exact_group_explicit_loopback_serves_one_query_and_cleans_up() {
        let _multicast_test_lock = crate::lock_multicast_loopback_tests();
        let response_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        response_socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let response_port = response_socket.local_addr().unwrap().port();
        let text = body();
        let worker = thread::spawn(move || {
            let parsed = ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                &text,
            )
            .unwrap();
            run_explicit_loopback_multicast_short_info_responder(
                activation(),
                Ipv4Addr::LOCALHOST,
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                &parsed,
                &AtomicBool::new(false),
            )
        });
        thread::sleep(Duration::from_millis(20));
        let query = ShortInfoQuery::new(
            "name='multicast'".into(),
            response_port,
            79,
            ShortInfoQueryWireLimits::new(128, 256).unwrap(),
        )
        .unwrap();
        let wire =
            ShortInfoQueryWire::encode(&query, ShortInfoQueryWireLimits::new(128, 256).unwrap())
                .unwrap();
        response_socket
            .send_to(
                wire.as_bytes(),
                (
                    DOCUMENTED_IPV4_MULTICAST_GROUP,
                    DOCUMENTED_IPV4_MULTICAST_PORT,
                ),
            )
            .unwrap();
        let mut bytes = [0_u8; 1024];
        let (count, source) = response_socket.recv_from(&mut bytes).unwrap();
        assert!(source.ip().is_loopback());
        assert!(std::str::from_utf8(&bytes[..count])
            .unwrap()
            .starts_with("79\r\n"));
        let run = worker.join().unwrap().unwrap();
        assert_eq!(run.requests(), 1);
        assert_eq!(
            run.termination(),
            ShortInfoResponderTermination::RequestLimit
        );
        let mut rebound = None;
        for _ in 0..20 {
            match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DOCUMENTED_IPV4_MULTICAST_PORT)) {
                Ok(socket) => {
                    rebound = Some(socket);
                    break;
                }
                Err(error) if error.kind() == ErrorKind::AddrInUse => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("unexpected cleanup probe failure: {error}"),
            }
        }
        assert!(rebound.is_some());

        let damaged_text = body();
        let parsed = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(damaged_text.len()).unwrap(),
            &damaged_text,
        )
        .unwrap();
        assert_eq!(
            run_explicit_loopback_multicast_short_info_responder(
                activation(),
                Ipv4Addr::new(192, 0, 2, 1),
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(damaged_text.len(), damaged_text.len() + 32,)
                    .unwrap(),
                &parsed,
                &AtomicBool::new(false),
            ),
            Err(ShortInfoResponderError::NonLoopbackMulticastInterface)
        );
    }

    #[test]
    fn lslc_004j_concrete_interface_entry_point_preserves_loopback_composition() {
        let _multicast_test_lock = crate::lock_multicast_loopback_tests();
        let response_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        response_socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let response_port = response_socket.local_addr().unwrap().port();
        let text = body();
        let worker = thread::spawn(move || {
            let parsed = ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                &text,
            )
            .unwrap();
            run_explicit_ipv4_multicast_short_info_responder(
                activation(),
                Ipv4Addr::LOCALHOST,
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                &parsed,
                &AtomicBool::new(false),
            )
        });
        thread::sleep(Duration::from_millis(20));
        let query_limits = ShortInfoQueryWireLimits::new(128, 256).unwrap();
        let query = ShortInfoQuery::new(
            "name='explicit-interface'".into(),
            response_port,
            89,
            query_limits,
        )
        .unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        response_socket
            .send_to(
                wire.as_bytes(),
                (
                    DOCUMENTED_IPV4_MULTICAST_GROUP,
                    DOCUMENTED_IPV4_MULTICAST_PORT,
                ),
            )
            .unwrap();
        let mut bytes = [0_u8; 1024];
        let (count, _) = response_socket.recv_from(&mut bytes).unwrap();
        assert!(std::str::from_utf8(&bytes[..count])
            .unwrap()
            .starts_with("89\r\n"));
        let run = worker.join().unwrap().unwrap();
        assert_eq!(run.requests(), 1);
        assert_eq!(
            run.termination(),
            ShortInfoResponderTermination::RequestLimit
        );
        assert!(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DOCUMENTED_IPV4_MULTICAST_PORT)).is_ok());
    }

    #[test]
    fn lslc_004j_nonconcrete_interfaces_reject_before_io() {
        let text = body();
        let parsed = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
            &text,
        )
        .unwrap();
        for interface in [
            Ipv4Addr::UNSPECIFIED,
            DOCUMENTED_IPV4_MULTICAST_GROUP,
            Ipv4Addr::BROADCAST,
        ] {
            assert_eq!(
                run_explicit_ipv4_multicast_short_info_responder(
                    activation(),
                    interface,
                    limits(1024, 1),
                    ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                    ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                    &parsed,
                    &AtomicBool::new(false),
                ),
                Err(ShortInfoResponderError::NonConcreteMulticastInterface)
            );
        }
    }

    #[test]
    fn lslc_004m_observed_official_query_structure_reaches_unchanged_responder() {
        let _multicast_test_lock = crate::lock_multicast_loopback_tests();
        let response_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        response_socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let response_port = response_socket.local_addr().unwrap().port();
        assert_eq!(response_port.to_string().len(), 5);
        let text = body();
        let worker = thread::spawn(move || {
            let parsed = ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                &text,
            )
            .unwrap();
            run_explicit_ipv4_multicast_short_info_responder(
                activation(),
                Ipv4Addr::LOCALHOST,
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                &parsed,
                &AtomicBool::new(false),
            )
        });
        thread::sleep(Duration::from_millis(20));
        let query_limits = ShortInfoQueryWireLimits::new(128, 256).unwrap();
        let query_id = 10_000_000_000_000_000_001_u64;
        let query = ShortInfoQuery::new(
            "session_id='default'".into(),
            response_port,
            query_id,
            query_limits,
        )
        .unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        assert_eq!(wire.as_bytes().len(), 65);
        assert!(wire.as_bytes().starts_with(b"LSL:shortinfo\r\n"));
        assert!(wire.as_bytes().ends_with(b"\r\n"));
        response_socket
            .send_to(
                wire.as_bytes(),
                (
                    DOCUMENTED_IPV4_MULTICAST_GROUP,
                    DOCUMENTED_IPV4_MULTICAST_PORT,
                ),
            )
            .unwrap();
        let mut bytes = [0_u8; 1024];
        let (count, _) = response_socket.recv_from(&mut bytes).unwrap();
        assert!(std::str::from_utf8(&bytes[..count])
            .unwrap()
            .starts_with("10000000000000000001\r\n"));
        let run = worker.join().unwrap().unwrap();
        assert_eq!(run.requests(), 1);
        assert_eq!(
            run.termination(),
            ShortInfoResponderTermination::RequestLimit
        );
        assert!(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DOCUMENTED_IPV4_MULTICAST_PORT)).is_ok());
    }

    #[test]
    #[ignore = "private active-interface pinned-official observation harness"]
    fn lslc_004o_private_active_interface_production_responder() {
        let interface = std::env::var("RLSL_LSLC_004O_INTERFACE")
            .expect("private harness requires an explicit interface")
            .parse::<Ipv4Addr>()
            .expect("private harness interface must be IPv4");
        let text = format!(
            "<?xml version=\"1.0\"?>\n<info>\n\
\t<name>rlsl-lslc-004o</name>\n\
\t<type>test</type>\n\
\t<channel_count>1</channel_count>\n\
\t<channel_format>float32</channel_format>\n\
\t<source_id>rlsl-lslc-004o</source_id>\n\
\t<nominal_srate>0</nominal_srate>\n\
\t<version>1.10</version>\n\
\t<created_at>1</created_at>\n\
\t<uid>rlsl-lslc-004o</uid>\n\
\t<session_id>default</session_id>\n\
\t<hostname>rlsl-lslc-004o</hostname>\n\
\t<v4address>{interface}</v4address>\n\
\t<v4data_port>1</v4data_port>\n\
\t<v4service_port>1</v4service_port>\n\
\t<v6address>::</v6address>\n\
\t<v6data_port>0</v6data_port>\n\
\t<v6service_port>0</v6service_port>\n\
\t<desc />\n</info>\n"
        );
        let parsed = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
            &text,
        )
        .unwrap();
        let run = run_explicit_ipv4_multicast_short_info_responder(
            activation(),
            interface,
            limits(1024, 1),
            ShortInfoQueryWireLimits::new(128, 256).unwrap(),
            ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
            &parsed,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(run.requests(), 1);
        assert_eq!(
            run.termination(),
            ShortInfoResponderTermination::RequestLimit
        );
    }

    #[test]
    fn lslc_004f_unchanged_requester_and_responder_compose_exactly_once() {
        let _multicast_test_lock = crate::lock_multicast_loopback_tests();
        let probe = UdpSocket::bind("127.0.0.1:0").unwrap();
        let requester_bind = probe.local_addr().unwrap();
        drop(probe);
        let text = body();
        let response_maximum = text.len() + 32;
        let responder = thread::spawn(move || {
            let parsed = ParsedStreamInfoObservedDocument::parse(
                StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                &text,
            )
            .unwrap();
            run_explicit_loopback_multicast_short_info_responder(
                activation(),
                Ipv4Addr::LOCALHOST,
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(text.len(), response_maximum).unwrap(),
                &parsed,
                &AtomicBool::new(false),
            )
        });
        thread::sleep(Duration::from_millis(20));
        let query_limits = ShortInfoQueryWireLimits::new(128, 256).unwrap();
        let query = ShortInfoQuery::new(
            "name='production-composition'".into(),
            requester_bind.port(),
            83,
            query_limits,
        )
        .unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        let run = run_udp_discovery(
            UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap(),
            UdpDiscoveryConfig::new(
                requester_bind,
                SocketAddr::from((
                    DOCUMENTED_IPV4_MULTICAST_GROUP,
                    DOCUMENTED_IPV4_MULTICAST_PORT,
                )),
                UdpDiscoveryLimits::new(
                    response_maximum,
                    1,
                    Duration::from_millis(10),
                    Duration::from_secs(1),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(body().len(), response_maximum).unwrap(),
            ),
            &wire,
            &AtomicBool::new(false),
        )
        .unwrap();
        let responder_run = responder.join().unwrap().unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert_eq!(run.responses().len(), 1);
        assert_eq!(run.responses()[0].query_id(), 83);
        assert_eq!(responder_run.requests(), 1);
        assert_eq!(
            responder_run.termination(),
            ShortInfoResponderTermination::RequestLimit
        );
        assert!(UdpSocket::bind(requester_bind).is_ok());
        assert!(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DOCUMENTED_IPV4_MULTICAST_PORT)).is_ok());
    }
}
