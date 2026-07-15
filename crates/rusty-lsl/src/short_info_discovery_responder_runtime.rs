// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Finite caller-configured UDP short-info response activation.

use crate::{
    ParsedShortInfoQuery, ParsedStreamInfoObservedDocument, RuntimeModule, RuntimeModuleCapability,
    ShortInfoQueryParseError, ShortInfoQueryWireLimits, ShortInfoResponseEnvelope,
    ShortInfoResponseEnvelopeEncodeError, ShortInfoResponseEnvelopeLimits,
};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Selected feature identity.
pub const SHORT_INFO_RESPONDER_FEATURE_ID: &str = "short-info-discovery-responder";
/// Explicit effective marker.
pub const SHORT_INFO_RESPONDER_EFFECTIVE_MARKER: &str =
    "rusty.lsl.short_info_discovery_responder.effective";

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
    /// Bind failed.
    Bind(ErrorKind),
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
    use crate::{ShortInfoQuery, ShortInfoQueryWire, StreamInfoObservedDocumentParseLimit};
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

    #[test]
    fn lslc_002z_valid_query_returns_matching_envelope_and_releases_port() {
        let address = free_address();
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
            run_short_info_responder(
                activation(),
                address,
                limits(1024, 1),
                ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                &parsed,
                &AtomicBool::new(false),
            )
        });
        thread::sleep(Duration::from_millis(20));
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
            let address = free_address();
            let text = body();
            let worker = thread::spawn(move || {
                let parsed = ParsedStreamInfoObservedDocument::parse(
                    StreamInfoObservedDocumentParseLimit::new(text.len()).unwrap(),
                    &text,
                )
                .unwrap();
                run_short_info_responder(
                    activation(),
                    address,
                    limits(16, 1),
                    ShortInfoQueryWireLimits::new(128, 256).unwrap(),
                    ShortInfoResponseEnvelopeLimits::new(text.len(), text.len() + 32).unwrap(),
                    &parsed,
                    &AtomicBool::new(false),
                )
            });
            thread::sleep(Duration::from_millis(20));
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
}
