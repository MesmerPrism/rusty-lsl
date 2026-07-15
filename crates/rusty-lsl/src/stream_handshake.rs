// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicitly activated, bounded protocol-110 TCP connection setup.

use crate::{RuntimeModule, RuntimeModuleCapability};
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Feature identity selected by the project lock.
pub const STREAM_HANDSHAKE_FEATURE_ID: &str = "stream-handshake";
/// Exact marker required beside the selected feature.
pub const STREAM_HANDSHAKE_EFFECTIVE_MARKER: &str = "rusty.lsl.stream_handshake.effective";

/// Proof of explicit feature and runtime-input selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamHandshakeActivation(());

impl StreamHandshakeActivation {
    /// Admits only the selected feature and exact effective marker.
    pub fn new(
        capability: RuntimeModuleCapability,
    ) -> Result<Self, StreamHandshakeActivationError> {
        if !capability.matches(RuntimeModule::StreamHandshake) {
            return Err(StreamHandshakeActivationError::WrongModule);
        }
        Ok(Self(()))
    }
}

/// Rejected activation input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamHandshakeActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}

/// Finite allocation and blocking limits for one connection setup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamHandshakeLimits {
    max_header_bytes: usize,
    max_field_bytes: usize,
    io_slice: Duration,
    total_deadline: Duration,
}

impl StreamHandshakeLimits {
    /// Creates nonzero limits in argument order.
    pub fn new(
        max_header_bytes: usize,
        max_field_bytes: usize,
        io_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, StreamHandshakeLimitError> {
        if max_header_bytes == 0 {
            return Err(StreamHandshakeLimitError::ZeroHeaderBytes);
        }
        if max_field_bytes == 0 {
            return Err(StreamHandshakeLimitError::ZeroFieldBytes);
        }
        if io_slice.is_zero() {
            return Err(StreamHandshakeLimitError::ZeroIoSlice);
        }
        if total_deadline.is_zero() {
            return Err(StreamHandshakeLimitError::ZeroTotalDeadline);
        }
        Ok(Self {
            max_header_bytes,
            max_field_bytes,
            io_slice,
            total_deadline,
        })
    }
}

/// Invalid finite limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamHandshakeLimitError {
    /// Header bound was zero.
    ZeroHeaderBytes,
    /// Field bound was zero.
    ZeroFieldBytes,
    /// I/O slice was zero.
    ZeroIoSlice,
    /// Deadline was zero.
    ZeroTotalDeadline,
}

/// Caller-owned opaque identity fields used by the observed handshake.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamHandshakeIdentity {
    uid: String,
    hostname: String,
    source_id: String,
    session_id: String,
}

impl StreamHandshakeIdentity {
    /// Admits nonempty printable ASCII fields below the selected bound.
    pub fn new(
        uid: String,
        hostname: String,
        source_id: String,
        session_id: String,
        limits: StreamHandshakeLimits,
    ) -> Result<Self, StreamHandshakeIdentityError> {
        for (role, value) in [
            (StreamHandshakeIdentityRole::Uid, &uid),
            (StreamHandshakeIdentityRole::Hostname, &hostname),
            (StreamHandshakeIdentityRole::SourceId, &source_id),
            (StreamHandshakeIdentityRole::SessionId, &session_id),
        ] {
            if value.is_empty() {
                return Err(StreamHandshakeIdentityError::Empty(role));
            }
            if value.len() > limits.max_field_bytes {
                return Err(StreamHandshakeIdentityError::Limit {
                    role,
                    actual: value.len(),
                    limit: limits.max_field_bytes,
                });
            }
            if !value.bytes().all(|b| (0x20..=0x7e).contains(&b)) {
                return Err(StreamHandshakeIdentityError::NonPrintableAscii(role));
            }
        }
        Ok(Self {
            uid,
            hostname,
            source_id,
            session_id,
        })
    }
    /// Opaque stream UID.
    pub fn uid(&self) -> &str {
        &self.uid
    }
}

/// Identity field role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamHandshakeIdentityRole {
    /// UID.
    Uid,
    /// Host label.
    Hostname,
    /// Source label.
    SourceId,
    /// Session label.
    SessionId,
}

/// Rejected identity input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamHandshakeIdentityError {
    /// Field was empty.
    Empty(StreamHandshakeIdentityRole),
    /// Field exceeded its byte bound.
    Limit {
        /// Field role.
        role: StreamHandshakeIdentityRole,
        /// Observed bytes.
        actual: usize,
        /// Selected maximum.
        limit: usize,
    },
    /// Field contained bytes outside printable ASCII.
    NonPrintableAscii(StreamHandshakeIdentityRole),
}

/// Completed client-side setup.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInletHandshake {
    peer: SocketAddr,
    uid: String,
}
impl StreamInletHandshake {
    /// Connected peer selected by the caller.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
    /// Accepted response UID.
    pub fn uid(&self) -> &str {
        &self.uid
    }
}

/// Completed server-side setup.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamOutletHandshake {
    local: SocketAddr,
    peer: SocketAddr,
}
impl StreamOutletHandshake {
    /// Actual listener address.
    pub const fn local_address(&self) -> SocketAddr {
        self.local
    }
    /// Accepted peer address.
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }
}

/// Stable bounded runtime failure.
#[derive(Debug, Eq, PartialEq)]
pub enum StreamHandshakeError {
    /// Caller cancellation was observed.
    Cancelled,
    /// Total deadline elapsed.
    Deadline,
    /// Socket operation failed.
    Io(ErrorKind),
    /// Header exceeded its selected bound.
    HeaderLimitExceeded {
        /// Selected maximum.
        limit: usize,
    },
    /// Header allocation failed.
    AllocationFailed {
        /// Requested capacity.
        requested: usize,
    },
    /// Header was not UTF-8.
    InvalidUtf8,
    /// Header shape or fixed values differed.
    InvalidHeader,
    /// Stream identity differed.
    IdentityMismatch,
}

fn request(identity: &StreamHandshakeIdentity) -> String {
    request_with_value_size(identity, 4)
}

fn request_with_value_size(identity: &StreamHandshakeIdentity, value_size: usize) -> String {
    request_with_format(identity, value_size, true)
}
fn request_with_format(
    identity: &StreamHandshakeIdentity,
    value_size: usize,
    supports_subnormals: bool,
) -> String {
    let sub = usize::from(supports_subnormals);
    format!("LSL:streamfeed/110 {}\r\nNative-Byte-Order: 1234\r\nEndian-Performance: 1.0\r\nHas-IEEE754-Floats: 1\r\nSupports-Subnormals: {sub}\r\nValue-Size: {value_size}\r\nData-Protocol-Version: 110\r\nMax-Buffer-Length: 100\r\nMax-Chunk-Length: 1\r\nHostname: {}\r\nSource-Id: {}\r\nSession-Id: {}\r\n\r\n",identity.uid,identity.hostname,identity.source_id,identity.session_id)
}

fn request_matches(received: &str, identity: &StreamHandshakeIdentity) -> bool {
    request_matches_value_size(received, identity, 4)
}

fn request_matches_value_size(
    received: &str,
    identity: &StreamHandshakeIdentity,
    value_size: usize,
) -> bool {
    request_matches_format(received, identity, value_size, true)
}
fn request_matches_format(
    received: &str,
    identity: &StreamHandshakeIdentity,
    value_size: usize,
    supports_subnormals: bool,
) -> bool {
    let lines: Vec<&str> = received.split("\r\n").collect();
    if lines.len() != 14 {
        return false;
    }
    let performance = match lines[2].strip_prefix("Endian-Performance: ") {
        Some(value) => value,
        None => return false,
    };
    let performance = match performance.parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 => value,
        _ => return false,
    };
    let _ = performance;
    lines[0] == format!("LSL:streamfeed/110 {}", identity.uid)
        && lines[1] == "Native-Byte-Order: 1234"
        && lines[3] == "Has-IEEE754-Floats: 1"
        && lines[4] == format!("Supports-Subnormals: {}", usize::from(supports_subnormals))
        && lines[5] == format!("Value-Size: {value_size}")
        && lines[6] == "Data-Protocol-Version: 110"
        && lines[7] == "Max-Buffer-Length: 100"
        && lines[8] == "Max-Chunk-Length: 1"
        && lines[9] == format!("Hostname: {}", identity.hostname)
        && lines[10] == format!("Source-Id: {}", identity.source_id)
        && lines[11] == format!("Session-Id: {}", identity.session_id)
        && lines[12].is_empty()
        && lines[13].is_empty()
}
fn response(identity: &StreamHandshakeIdentity) -> String {
    format!("LSL/110 200 OK\r\nUID: {}\r\nByte-Order: 1234\r\nSuppress-Subnormals: 0\r\nData-Protocol-Version: 110\r\n\r\n", identity.uid)
}

fn write_all_bounded(
    stream: &mut TcpStream,
    bytes: &[u8],
    limits: StreamHandshakeLimits,
    started: Instant,
    cancelled: &AtomicBool,
) -> Result<(), StreamHandshakeError> {
    let mut written = 0;
    while written < bytes.len() {
        if cancelled.load(Ordering::Acquire) {
            return Err(StreamHandshakeError::Cancelled);
        }
        let remaining = limits
            .total_deadline
            .checked_sub(started.elapsed())
            .ok_or(StreamHandshakeError::Deadline)?;
        stream
            .set_write_timeout(Some(remaining.min(limits.io_slice)))
            .map_err(|e| StreamHandshakeError::Io(e.kind()))?;
        match stream.write(&bytes[written..]) {
            Ok(0) => return Err(StreamHandshakeError::Io(ErrorKind::WriteZero)),
            Ok(n) => written += n,
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => continue,
            Err(e) => return Err(StreamHandshakeError::Io(e.kind())),
        }
    }
    Ok(())
}

fn read_header(
    stream: &mut TcpStream,
    limits: StreamHandshakeLimits,
    started: Instant,
    cancelled: &AtomicBool,
) -> Result<String, StreamHandshakeError> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(limits.max_header_bytes)
        .map_err(|_| StreamHandshakeError::AllocationFailed {
            requested: limits.max_header_bytes,
        })?;
    while !bytes.ends_with(b"\r\n\r\n") {
        if bytes.len() == limits.max_header_bytes {
            return Err(StreamHandshakeError::HeaderLimitExceeded {
                limit: limits.max_header_bytes,
            });
        }
        if cancelled.load(Ordering::Acquire) {
            return Err(StreamHandshakeError::Cancelled);
        }
        let remaining = limits
            .total_deadline
            .checked_sub(started.elapsed())
            .ok_or(StreamHandshakeError::Deadline)?;
        stream
            .set_read_timeout(Some(remaining.min(limits.io_slice)))
            .map_err(|e| StreamHandshakeError::Io(e.kind()))?;
        let mut byte = [0u8; 1];
        match stream.read(&mut byte) {
            Ok(0) => return Err(StreamHandshakeError::InvalidHeader),
            Ok(_) => bytes.push(byte[0]),
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => continue,
            Err(e) => return Err(StreamHandshakeError::Io(e.kind())),
        }
    }
    String::from_utf8(bytes).map_err(|_| StreamHandshakeError::InvalidUtf8)
}

/// Connects to one caller-selected peer, exchanges one request/response header, and closes on return.
pub fn run_stream_inlet_handshake(
    _activation: StreamHandshakeActivation,
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
) -> Result<StreamInletHandshake, StreamHandshakeError> {
    let stream = connect_handshake_stream(peer, identity, limits, cancelled)?;
    drop(stream);
    Ok(StreamInletHandshake {
        peer,
        uid: identity.uid.clone(),
    })
}

pub(crate) fn connect_handshake_stream(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
) -> Result<TcpStream, StreamHandshakeError> {
    connect_handshake_stream_with_value_size(peer, identity, limits, cancelled, 4)
}

pub(crate) fn connect_handshake_stream_with_value_size(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
    value_size: usize,
) -> Result<TcpStream, StreamHandshakeError> {
    connect_handshake_stream_with_format(peer, identity, limits, cancelled, value_size, true)
}
pub(crate) fn connect_handshake_stream_with_format(
    peer: SocketAddr,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
    value_size: usize,
    supports_subnormals: bool,
) -> Result<TcpStream, StreamHandshakeError> {
    if cancelled.load(Ordering::Acquire) {
        return Err(StreamHandshakeError::Cancelled);
    }
    let started = Instant::now();
    let mut stream = TcpStream::connect_timeout(&peer, limits.total_deadline)
        .map_err(|e| StreamHandshakeError::Io(e.kind()))?;
    let request = request_with_format(identity, value_size, supports_subnormals);
    if request.len() > limits.max_header_bytes {
        return Err(StreamHandshakeError::HeaderLimitExceeded {
            limit: limits.max_header_bytes,
        });
    }
    write_all_bounded(&mut stream, request.as_bytes(), limits, started, cancelled)?;
    let received = read_header(&mut stream, limits, started, cancelled)?;
    if received != response(identity) {
        return Err(if received.starts_with("LSL/110 200 OK\r\nUID: ") {
            StreamHandshakeError::IdentityMismatch
        } else {
            StreamHandshakeError::InvalidHeader
        });
    }
    Ok(stream)
}

/// Accepts one connection on a caller-selected listener, admits one request, responds, and closes on return.
pub fn run_stream_outlet_handshake(
    _activation: StreamHandshakeActivation,
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
) -> Result<StreamOutletHandshake, StreamHandshakeError> {
    let (stream, local, peer) = accept_handshake_stream(listener, identity, limits, cancelled)?;
    drop(stream);
    Ok(StreamOutletHandshake { local, peer })
}

pub(crate) fn accept_handshake_stream(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
    accept_handshake_stream_with_value_size(listener, identity, limits, cancelled, 4)
}

pub(crate) fn accept_handshake_stream_with_value_size(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
    value_size: usize,
) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
    accept_handshake_stream_with_format(listener, identity, limits, cancelled, value_size, true)
}
pub(crate) fn accept_handshake_stream_with_format(
    listener: TcpListener,
    identity: &StreamHandshakeIdentity,
    limits: StreamHandshakeLimits,
    cancelled: &AtomicBool,
    value_size: usize,
    supports_subnormals: bool,
) -> Result<(TcpStream, SocketAddr, SocketAddr), StreamHandshakeError> {
    let local = listener
        .local_addr()
        .map_err(|e| StreamHandshakeError::Io(e.kind()))?;
    listener
        .set_nonblocking(true)
        .map_err(|e| StreamHandshakeError::Io(e.kind()))?;
    let started = Instant::now();
    let (mut stream, peer) = loop {
        if cancelled.load(Ordering::Acquire) {
            return Err(StreamHandshakeError::Cancelled);
        }
        if started.elapsed() >= limits.total_deadline {
            return Err(StreamHandshakeError::Deadline);
        }
        match listener.accept() {
            Ok(value) => break value,
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(limits.io_slice.min(Duration::from_millis(2)))
            }
            Err(e) => return Err(StreamHandshakeError::Io(e.kind())),
        }
    };
    let received = read_header(&mut stream, limits, started, cancelled)?;
    if !request_matches_format(&received, identity, value_size, supports_subnormals) {
        return Err(if received.starts_with("LSL:streamfeed/110 ") {
            StreamHandshakeError::IdentityMismatch
        } else {
            StreamHandshakeError::InvalidHeader
        });
    }
    let response = response(identity);
    write_all_bounded(&mut stream, response.as_bytes(), limits, started, cancelled)?;
    Ok((stream, local, peer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use std::thread;
    fn activation() -> StreamHandshakeActivation {
        StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake)).unwrap()
    }
    fn limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 64, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }
    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "11111111-2222-4333-8444-555555555555".into(),
            "synthetic-host".into(),
            "synthetic-source".into(),
            "synthetic-session".into(),
            limits(),
        )
        .unwrap()
    }
    #[test]
    fn lslc_002s_loopback_exchanges_and_releases_listener() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server_identity = identity();
        let server = thread::spawn(move || {
            run_stream_outlet_handshake(
                activation(),
                listener,
                &server_identity,
                limits(),
                &AtomicBool::new(false),
            )
            .unwrap()
        });
        let client = run_stream_inlet_handshake(
            activation(),
            address,
            &identity(),
            limits(),
            &AtomicBool::new(false),
        )
        .unwrap();
        let outlet = server.join().unwrap();
        assert_eq!(client.uid(), identity().uid());
        assert_eq!(outlet.local_address(), address);
        assert!(outlet.peer().ip().is_loopback());
        TcpListener::bind(address).unwrap();
    }
    #[test]
    fn lslc_002s_cancellation_timeout_malformed_and_identity_mismatch_are_typed() {
        let cancelled = AtomicBool::new(true);
        assert_eq!(
            run_stream_inlet_handshake(
                activation(),
                "127.0.0.1:9".parse().unwrap(),
                &identity(),
                limits(),
                &cancelled
            ),
            Err(StreamHandshakeError::Cancelled)
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let tiny =
            StreamHandshakeLimits::new(4, 64, Duration::from_millis(2), Duration::from_millis(20))
                .unwrap();
        assert_eq!(
            run_stream_outlet_handshake(
                activation(),
                listener,
                &identity(),
                tiny,
                &AtomicBool::new(false)
            ),
            Err(StreamHandshakeError::Deadline)
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            run_stream_outlet_handshake(
                activation(),
                listener,
                &identity(),
                limits(),
                &AtomicBool::new(false),
            )
        });
        let mut peer = TcpStream::connect(address).unwrap();
        peer.write_all(b"damaged\r\n\r\n").unwrap();
        assert_eq!(
            worker.join().unwrap(),
            Err(StreamHandshakeError::InvalidHeader)
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            run_stream_outlet_handshake(
                activation(),
                listener,
                &identity(),
                limits(),
                &AtomicBool::new(false),
            )
        });
        let other = StreamHandshakeIdentity::new(
            "different".into(),
            "synthetic-host".into(),
            "synthetic-source".into(),
            "synthetic-session".into(),
            limits(),
        )
        .unwrap();
        assert_eq!(
            run_stream_inlet_handshake(
                activation(),
                address,
                &other,
                limits(),
                &AtomicBool::new(false)
            ),
            Err(StreamHandshakeError::InvalidHeader)
        );
        assert_eq!(
            worker.join().unwrap(),
            Err(StreamHandshakeError::IdentityMismatch)
        );
    }

    #[test]
    fn lslc_002y_official_performance_value_is_bounded_and_finite() {
        let identity = identity();
        let canonical = request(&identity);
        let official = canonical.replace(
            "Endian-Performance: 1.0\r\n",
            "Endian-Performance: 5.24262e+06\r\n",
        );
        assert!(request_matches(&official, &identity));
        for damaged in ["nan", "inf", "0", "-1", "not-a-number"] {
            assert!(!request_matches(
                &canonical.replace(
                    "Endian-Performance: 1.0\r\n",
                    &format!("Endian-Performance: {damaged}\r\n"),
                ),
                &identity
            ));
        }
        assert!(!request_matches(
            &official.replace("Source-Id: synthetic-source", "Source-Id: other"),
            &identity
        ));
    }
    #[test]
    fn lslc_002s_activation_and_limits_fail_closed() {
        assert_eq!(
            StreamHandshakeActivation::new(test_capability(RuntimeModule::UdpDiscovery)),
            Err(StreamHandshakeActivationError::WrongModule)
        );
        assert_eq!(
            StreamHandshakeLimits::new(0, 0, Duration::ZERO, Duration::ZERO),
            Err(StreamHandshakeLimitError::ZeroHeaderBytes)
        );
    }
}
