// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-owned polling composition for persistent Float32 discovery and transfer.

use crate::{
    ChannelFormat, ParsedShortInfoQuery, ParsedStreamInfoObservedDocument,
    PersistentFloat32AcceptError, PersistentFloat32ConsumerAccepted, PersistentFloat32Outlet,
    PersistentFloat32OutletCloseReport, PersistentFloat32PushError, PersistentFloat32PushReport,
    RawSourceTimestamp, ShortInfoQueryParseError, ShortInfoQueryWireLimits,
    ShortInfoResponderActivation, ShortInfoResponseEnvelope, ShortInfoResponseEnvelopeEncodeError,
    ShortInfoResponseEnvelopeLimits, StreamInfoObservedAdmissionError,
    StreamInfoObservedAdmissionLimits, StreamInfoObservedDocumentParseError,
    StreamInfoObservedDocumentParseLimit, StreamInfoObservedFields, StreamInfoVolatileFieldRole,
    DOCUMENTED_IPV4_MULTICAST_GROUP, DOCUMENTED_IPV4_MULTICAST_PORT,
};
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};

const MAX_DISCOVERY_DATAGRAM_BYTES: usize = 65_507;

/// Bounded parsing and retained-buffer limits for one managed service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletServiceLimits {
    max_datagram_bytes: usize,
    document: StreamInfoObservedDocumentParseLimit,
    admission: StreamInfoObservedAdmissionLimits,
    query: ShortInfoQueryWireLimits,
    response: ShortInfoResponseEnvelopeLimits,
}

impl PersistentFloat32OutletServiceLimits {
    /// Groups the existing stream-info and short-info contracts with one datagram bound.
    ///
    /// # Errors
    ///
    /// Rejects a zero datagram bound or a value above the largest IPv4 UDP payload.
    pub fn new(
        max_datagram_bytes: usize,
        document: StreamInfoObservedDocumentParseLimit,
        admission: StreamInfoObservedAdmissionLimits,
        query: ShortInfoQueryWireLimits,
        response: ShortInfoResponseEnvelopeLimits,
    ) -> Result<Self, PersistentFloat32OutletServiceLimitError> {
        if max_datagram_bytes == 0 {
            return Err(PersistentFloat32OutletServiceLimitError::ZeroDatagramBytes);
        }
        if max_datagram_bytes > MAX_DISCOVERY_DATAGRAM_BYTES {
            return Err(
                PersistentFloat32OutletServiceLimitError::DatagramLimitExceeded {
                    actual: max_datagram_bytes,
                    limit: MAX_DISCOVERY_DATAGRAM_BYTES,
                },
            );
        }
        Ok(Self {
            max_datagram_bytes,
            document,
            admission,
            query,
            response,
        })
    }

    /// Maximum accepted discovery datagram bytes.
    #[must_use]
    pub const fn max_datagram_bytes(self) -> usize {
        self.max_datagram_bytes
    }
}

/// Invalid managed-service retained-resource limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletServiceLimitError {
    /// The discovery datagram limit was zero.
    ZeroDatagramBytes,
    /// The selected bound exceeded the maximum IPv4 UDP payload.
    DatagramLimitExceeded {
        /// Requested bytes.
        actual: usize,
        /// Fixed ceiling.
        limit: usize,
    },
}

/// One handled discovery query and response destination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32DiscoveryHandled {
    source: SocketAddr,
    destination: SocketAddr,
    query_id: u64,
}

impl PersistentFloat32DiscoveryHandled {
    /// Socket that supplied the query datagram.
    #[must_use]
    pub const fn source(self) -> SocketAddr {
        self.source
    }

    /// Query-selected response destination.
    #[must_use]
    pub const fn destination(self) -> SocketAddr {
        self.destination
    }

    /// Uninterpreted query correlation value.
    #[must_use]
    pub const fn query_id(self) -> u64 {
        self.query_id
    }
}

/// Bounded work completed by one caller-owned poll.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletServicePoll {
    discovery: Option<PersistentFloat32DiscoveryHandled>,
    consumer: Option<PersistentFloat32ConsumerAccepted>,
}

impl PersistentFloat32OutletServicePoll {
    /// Discovery query handled by this poll, if any.
    #[must_use]
    pub const fn discovery(self) -> Option<PersistentFloat32DiscoveryHandled> {
        self.discovery
    }

    /// Consumer admitted by this poll, if any.
    #[must_use]
    pub const fn consumer(self) -> Option<PersistentFloat32ConsumerAccepted> {
        self.consumer
    }

    /// Whether neither socket had pending work.
    #[must_use]
    pub const fn is_idle(self) -> bool {
        self.discovery.is_none() && self.consumer.is_none()
    }
}

/// Explicit close accounting for one managed service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletServiceCloseReport {
    outlet: PersistentFloat32OutletCloseReport,
}

impl PersistentFloat32OutletServiceCloseReport {
    /// Close report from the retained persistent outlet.
    #[must_use]
    pub const fn outlet(self) -> PersistentFloat32OutletCloseReport {
        self.outlet
    }
}

/// Failure before the managed service becomes usable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletServiceCreateError {
    /// The selected interface was unspecified, multicast, or broadcast.
    NonConcreteIpv4Interface,
    /// The documented multicast socket could not be bound.
    BindDiscovery(ErrorKind),
    /// The exact documented multicast group could not be joined on the selected interface.
    JoinDiscovery(ErrorKind),
    /// The caller-owned discovery socket could not be inspected.
    DiscoveryLocalAddress(ErrorKind),
    /// The caller-owned discovery socket could not be made nonblocking.
    DiscoveryNonblocking(ErrorKind),
    /// The bounded discovery receive allocation failed.
    DiscoveryBufferAllocationFailed {
        /// Exact requested capacity.
        requested: usize,
    },
    /// The receive probe length overflowed.
    DiscoveryProbeLengthOverflow,
    /// The retained body was not a canonical observed document.
    Document(StreamInfoObservedDocumentParseError),
    /// The retained body did not admit into the existing typed contracts.
    Admission(StreamInfoObservedAdmissionError),
    /// The advertised channel format was not Float32.
    WrongChannelFormat,
    /// The advertised channel count differed from the outlet shape.
    ChannelCountMismatch {
        /// Body channel count.
        advertised: usize,
        /// Outlet channel count.
        outlet: usize,
    },
    /// One advertised identity field differed from the outlet handshake identity.
    IdentityMismatch(PersistentFloat32OutletServiceIdentityRole),
    /// The advertised IPv4 address was not the caller-selected interface spelling.
    AdvertisedIpv4AddressMismatch,
    /// The advertised IPv4 data port differed from the persistent listener.
    AdvertisedDataPortMismatch,
    /// The advertised IPv4 service port differed from the persistent listener.
    AdvertisedServicePortMismatch,
    /// A concrete listener address differed from the selected advertised interface.
    ListenerAddressMismatch,
    /// The persistent outlet was not backed by an IPv4 listener.
    NonIpv4Outlet,
}

/// Identity role checked between discovery and handshake owners.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletServiceIdentityRole {
    /// Stream UID.
    Uid,
    /// Host name.
    Hostname,
    /// Source ID.
    SourceId,
    /// Session ID.
    SessionId,
}

/// Failure while one caller-owned poll executes bounded work.
#[derive(Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletServicePollError {
    /// Caller cancellation was selected before socket work.
    Cancelled,
    /// Discovery receive failed.
    Receive(ErrorKind),
    /// The datagram exceeded the selected bound.
    DatagramLimitExceeded {
        /// Selected bound.
        limit: usize,
        /// Bounded observed extent.
        actual: usize,
    },
    /// Query admission failed.
    Query(ShortInfoQueryParseError),
    /// Response encoding failed.
    Response(ShortInfoResponseEnvelopeEncodeError),
    /// Discovery response send failed.
    Send(ErrorKind),
    /// UDP reported a partial datagram send.
    PartialSend {
        /// Response bytes.
        expected: usize,
        /// Reported sent bytes.
        actual: usize,
    },
    /// Persistent-consumer admission failed.
    Accept(PersistentFloat32AcceptError),
}

/// Explicitly activated, caller-polled discovery and persistent Float32 owner.
pub struct PersistentFloat32OutletService {
    outlet: PersistentFloat32Outlet,
    discovery: UdpSocket,
    discovery_local: SocketAddr,
    advertised_ipv4: Ipv4Addr,
    body: String,
    limits: PersistentFloat32OutletServiceLimits,
    receive: Vec<u8>,
}

impl PersistentFloat32OutletService {
    /// Binds the exact documented IPv4 discovery endpoint on one explicit interface.
    ///
    /// This constructor does not enumerate interfaces, select a default, fall back,
    /// spawn a worker, or retry. The caller remains responsible for polling.
    ///
    /// # Errors
    ///
    /// Rejects an invalid interface, socket setup failure, allocation failure, or
    /// any mismatch between the canonical stream-info body and the outlet.
    #[allow(clippy::too_many_arguments)]
    pub fn new_explicit_ipv4_multicast(
        discovery_activation: ShortInfoResponderActivation,
        interface: Ipv4Addr,
        outlet: PersistentFloat32Outlet,
        body: String,
        limits: PersistentFloat32OutletServiceLimits,
    ) -> Result<Self, PersistentFloat32OutletServiceCreateError> {
        validate_interface(interface)?;
        let discovery = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DOCUMENTED_IPV4_MULTICAST_PORT))
            .map_err(|error| {
                PersistentFloat32OutletServiceCreateError::BindDiscovery(error.kind())
            })?;
        discovery
            .join_multicast_v4(&DOCUMENTED_IPV4_MULTICAST_GROUP, &interface)
            .map_err(|error| {
                PersistentFloat32OutletServiceCreateError::JoinDiscovery(error.kind())
            })?;
        Self::new_prebound(
            discovery_activation,
            interface,
            outlet,
            discovery,
            body,
            limits,
        )
    }

    /// Composes one caller-bound discovery socket with an accepted persistent outlet.
    ///
    /// This entry point is useful when an application owns socket binding and
    /// multicast membership. It never joins a group or changes interface policy.
    ///
    /// # Errors
    ///
    /// Rejects malformed retained input, configuration mismatches, allocation
    /// failure, or socket inspection/configuration failure before returning state.
    #[allow(clippy::too_many_arguments)]
    pub fn new_prebound(
        _discovery_activation: ShortInfoResponderActivation,
        advertised_ipv4: Ipv4Addr,
        outlet: PersistentFloat32Outlet,
        discovery: UdpSocket,
        body: String,
        limits: PersistentFloat32OutletServiceLimits,
    ) -> Result<Self, PersistentFloat32OutletServiceCreateError> {
        validate_interface(advertised_ipv4)?;
        validate_body(&body, limits, advertised_ipv4, &outlet)?;
        let probe = limits
            .max_datagram_bytes
            .checked_add(1)
            .ok_or(PersistentFloat32OutletServiceCreateError::DiscoveryProbeLengthOverflow)?;
        let mut receive = Vec::new();
        receive.try_reserve_exact(probe).map_err(|_| {
            PersistentFloat32OutletServiceCreateError::DiscoveryBufferAllocationFailed {
                requested: probe,
            }
        })?;
        receive.resize(probe, 0);
        let discovery_local = discovery.local_addr().map_err(|error| {
            PersistentFloat32OutletServiceCreateError::DiscoveryLocalAddress(error.kind())
        })?;
        discovery.set_nonblocking(true).map_err(|error| {
            PersistentFloat32OutletServiceCreateError::DiscoveryNonblocking(error.kind())
        })?;
        Ok(Self {
            outlet,
            discovery,
            discovery_local,
            advertised_ipv4,
            body,
            limits,
            receive,
        })
    }

    /// Actual persistent TCP listener address.
    #[must_use]
    pub const fn outlet_local_address(&self) -> SocketAddr {
        self.outlet.local_address()
    }

    /// Actual caller-owned UDP discovery socket address.
    #[must_use]
    pub const fn discovery_local_address(&self) -> SocketAddr {
        self.discovery_local
    }

    /// Caller-selected advertised IPv4 interface.
    #[must_use]
    pub const fn advertised_ipv4(&self) -> Ipv4Addr {
        self.advertised_ipv4
    }

    /// Fixed channel count of the retained outlet.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.outlet.channel_count()
    }

    /// Number of consumers currently retained for fan-out.
    #[must_use]
    pub fn connected_consumers(&self) -> usize {
        self.outlet.connected_consumers()
    }

    /// Handles at most one pending discovery query and one pending consumer.
    ///
    /// Idle polling returns immediately. The accepted protocol handshake may use
    /// its caller-selected finite deadline after a TCP connection is accepted.
    ///
    /// # Errors
    ///
    /// Returns typed cancellation, discovery, response, or consumer-admission
    /// evidence. It does not retry, fall back, or start background work.
    pub fn poll(
        &mut self,
        cancelled: &AtomicBool,
    ) -> Result<PersistentFloat32OutletServicePoll, PersistentFloat32OutletServicePollError> {
        if cancelled.load(Ordering::Acquire) {
            return Err(PersistentFloat32OutletServicePollError::Cancelled);
        }
        let discovery = self.poll_discovery()?;
        // A full service retains its admitted data consumers and leaves any
        // auxiliary official-inlet connection in the bounded listener backlog.
        // The lower-level outlet keeps its explicit capacity error contract.
        let consumer = if self.outlet.connected_consumers() == self.outlet.max_consumers() {
            None
        } else {
            self.outlet
                .poll_accept_consumer(cancelled)
                .map_err(PersistentFloat32OutletServicePollError::Accept)?
        };
        Ok(PersistentFloat32OutletServicePoll {
            discovery,
            consumer,
        })
    }

    /// Delegates one allocation-free-after-setup chunk fan-out to the retained outlet.
    ///
    /// # Errors
    ///
    /// Preserves the existing persistent-outlet input rejection contract.
    pub fn push_chunk(
        &mut self,
        values: &[f32],
        timestamps: &[RawSourceTimestamp],
        cancelled: &AtomicBool,
    ) -> Result<PersistentFloat32PushReport, PersistentFloat32PushError> {
        self.outlet.push_chunk(values, timestamps, cancelled)
    }

    /// Closes retained consumers; the discovery membership and socket drop on return.
    #[must_use]
    pub fn close(self) -> PersistentFloat32OutletServiceCloseReport {
        PersistentFloat32OutletServiceCloseReport {
            outlet: self.outlet.close(),
        }
    }

    fn poll_discovery(
        &mut self,
    ) -> Result<Option<PersistentFloat32DiscoveryHandled>, PersistentFloat32OutletServicePollError>
    {
        let (length, source) = match self.discovery.recv_from(&mut self.receive) {
            Ok(received) => received,
            // Windows reports an ICMP port-unreachable from a completed resolver
            // reply as `ConnectionReset` on the next unconnected UDP receive. It
            // carries no discovery datagram and must not tear down the outlet.
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::WouldBlock | ErrorKind::ConnectionReset
                ) =>
            {
                return Ok(None)
            }
            Err(error) => {
                return Err(PersistentFloat32OutletServicePollError::Receive(
                    error.kind(),
                ))
            }
        };
        if length > self.limits.max_datagram_bytes {
            return Err(
                PersistentFloat32OutletServicePollError::DatagramLimitExceeded {
                    limit: self.limits.max_datagram_bytes,
                    actual: length,
                },
            );
        }
        let query = ParsedShortInfoQuery::parse(&self.receive[..length], self.limits.query)
            .map_err(PersistentFloat32OutletServicePollError::Query)?;
        let parsed = ParsedStreamInfoObservedDocument::parse(self.limits.document, &self.body)
            .expect("constructor retained one unchanged canonical body");
        let response =
            ShortInfoResponseEnvelope::encode(query.query_id(), &parsed, self.limits.response)
                .map_err(PersistentFloat32OutletServicePollError::Response)?;
        let destination = SocketAddr::new(source.ip(), query.return_port());
        let sent = self
            .discovery
            .send_to(response.as_bytes(), destination)
            .map_err(|error| PersistentFloat32OutletServicePollError::Send(error.kind()))?;
        if sent != response.as_bytes().len() {
            return Err(PersistentFloat32OutletServicePollError::PartialSend {
                expected: response.as_bytes().len(),
                actual: sent,
            });
        }
        Ok(Some(PersistentFloat32DiscoveryHandled {
            source,
            destination,
            query_id: query.query_id(),
        }))
    }
}

fn validate_interface(
    interface: Ipv4Addr,
) -> Result<(), PersistentFloat32OutletServiceCreateError> {
    if interface.is_unspecified() || interface.is_multicast() || interface == Ipv4Addr::BROADCAST {
        return Err(PersistentFloat32OutletServiceCreateError::NonConcreteIpv4Interface);
    }
    Ok(())
}

fn validate_body(
    body: &str,
    limits: PersistentFloat32OutletServiceLimits,
    interface: Ipv4Addr,
    outlet: &PersistentFloat32Outlet,
) -> Result<(), PersistentFloat32OutletServiceCreateError> {
    let parsed = ParsedStreamInfoObservedDocument::parse(limits.document, body)
        .map_err(PersistentFloat32OutletServiceCreateError::Document)?;
    let observed = StreamInfoObservedFields::admit(limits.admission, parsed)
        .map_err(PersistentFloat32OutletServiceCreateError::Admission)?;
    let descriptor = observed.definition().descriptor();
    if descriptor.channel_format() != ChannelFormat::Float32 {
        return Err(PersistentFloat32OutletServiceCreateError::WrongChannelFormat);
    }
    if descriptor.channel_count() != outlet.channel_count() {
        return Err(
            PersistentFloat32OutletServiceCreateError::ChannelCountMismatch {
                advertised: descriptor.channel_count(),
                outlet: outlet.channel_count(),
            },
        );
    }
    let identity = outlet.stream_identity();
    let volatile = observed.volatile_fields();
    for (role, advertised, actual) in [
        (
            PersistentFloat32OutletServiceIdentityRole::Uid,
            volatile.field(StreamInfoVolatileFieldRole::Uid),
            identity.uid(),
        ),
        (
            PersistentFloat32OutletServiceIdentityRole::Hostname,
            volatile.field(StreamInfoVolatileFieldRole::Hostname),
            identity.hostname(),
        ),
        (
            PersistentFloat32OutletServiceIdentityRole::SourceId,
            descriptor.source_id().unwrap_or_default(),
            identity.source_id(),
        ),
        (
            PersistentFloat32OutletServiceIdentityRole::SessionId,
            volatile.field(StreamInfoVolatileFieldRole::SessionId),
            identity.session_id(),
        ),
    ] {
        if advertised != actual {
            return Err(PersistentFloat32OutletServiceCreateError::IdentityMismatch(
                role,
            ));
        }
    }
    if volatile.field(StreamInfoVolatileFieldRole::V4Address) != interface.to_string() {
        return Err(PersistentFloat32OutletServiceCreateError::AdvertisedIpv4AddressMismatch);
    }
    let local = outlet.local_address();
    let IpAddr::V4(listener_ipv4) = local.ip() else {
        return Err(PersistentFloat32OutletServiceCreateError::NonIpv4Outlet);
    };
    if !listener_ipv4.is_unspecified() && listener_ipv4 != interface {
        return Err(PersistentFloat32OutletServiceCreateError::ListenerAddressMismatch);
    }
    let port = local.port().to_string();
    if volatile.field(StreamInfoVolatileFieldRole::V4DataPort) != port {
        return Err(PersistentFloat32OutletServiceCreateError::AdvertisedDataPortMismatch);
    }
    if volatile.field(StreamInfoVolatileFieldRole::V4ServicePort) != port {
        return Err(PersistentFloat32OutletServiceCreateError::AdvertisedServicePortMismatch);
    }
    let _ = volatile.field(StreamInfoVolatileFieldRole::V4ServicePort);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::stream_handshake::connect_handshake_stream;
    use crate::timestamped_float32_session_runtime::codec::read_initialization_for_channels;
    use crate::{
        MetadataTreeLimits, PersistentFloat32OutletActivation, PersistentFloat32OutletLimits,
        ShortInfoQuery, ShortInfoQueryWire, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamHandshakeIdentity, StreamHandshakeLimits, StreamInfoVolatileFieldLimits,
        TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    };
    use std::io::Read;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 128, Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }

    fn identity() -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            "71000000-0000-4000-8000-000000000001".into(),
            "interop-host".into(),
            "interop-source".into(),
            "interop-session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn outlet(listener: TcpListener, channels: usize) -> PersistentFloat32Outlet {
        let activation = PersistentFloat32OutletActivation::new(
            test_capability(crate::RuntimeModule::PersistentFloat32Outlet),
            TimestampedFloat32SampleActivation::new(
                test_capability(crate::RuntimeModule::TimestampedFloat32Sample),
                StreamHandshakeActivation::new(test_capability(
                    crate::RuntimeModule::StreamHandshake,
                ))
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        PersistentFloat32Outlet::new(
            activation,
            listener,
            identity(),
            handshake_limits(),
            sample_limits(),
            channels,
            PersistentFloat32OutletLimits::new(10, 1).unwrap(),
        )
        .unwrap()
    }

    fn responder_activation() -> ShortInfoResponderActivation {
        ShortInfoResponderActivation::new(test_capability(
            crate::RuntimeModule::ShortInfoDiscoveryResponder,
        ))
        .unwrap()
    }

    fn body(port: u16, channels: usize) -> String {
        format!(
            "<?xml version=\"1.0\"?>\n<info>\n\
\t<name>interop-outlet</name>\n\
\t<type>qualification</type>\n\
\t<channel_count>{channels}</channel_count>\n\
\t<channel_format>float32</channel_format>\n\
\t<source_id>interop-source</source_id>\n\
\t<nominal_srate>100.0000000000000</nominal_srate>\n\
\t<version>1.100000000000000</version>\n\
\t<created_at>1.0</created_at>\n\
\t<uid>71000000-0000-4000-8000-000000000001</uid>\n\
\t<session_id>interop-session</session_id>\n\
\t<hostname>interop-host</hostname>\n\
\t<v4address>127.0.0.1</v4address>\n\
\t<v4data_port>{port}</v4data_port>\n\
\t<v4service_port>{port}</v4service_port>\n\
\t<v6address></v6address>\n\
\t<v6data_port>0</v6data_port>\n\
\t<v6service_port>0</v6service_port>\n\
\t<desc />\n</info>\n"
        )
    }

    fn limits(body_len: usize) -> PersistentFloat32OutletServiceLimits {
        PersistentFloat32OutletServiceLimits::new(
            2048,
            StreamInfoObservedDocumentParseLimit::new(body_len).unwrap(),
            StreamInfoObservedAdmissionLimits::new(
                StreamDescriptorLimits::new(128, 128, 128, 256).unwrap(),
                MetadataTreeLimits::new(1, 1, 1, 4, 1).unwrap(),
                StreamInfoVolatileFieldLimits::new(128, 128, 128).unwrap(),
            ),
            ShortInfoQueryWireLimits::new(256, 512).unwrap(),
            ShortInfoResponseEnvelopeLimits::new(body_len, body_len + 32).unwrap(),
        )
        .unwrap()
    }

    fn service() -> PersistentFloat32OutletService {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let outlet = outlet(listener, 1);
        let text = body(outlet.local_address().port(), 1);
        let discovery = UdpSocket::bind("127.0.0.1:0").unwrap();
        PersistentFloat32OutletService::new_prebound(
            responder_activation(),
            Ipv4Addr::LOCALHOST,
            outlet,
            discovery,
            text.clone(),
            limits(text.len()),
        )
        .unwrap()
    }

    #[test]
    fn interop_001_polls_discovery_accepts_consumer_and_pushes_one_chunk() {
        let mut service = service();
        assert!(service.poll(&AtomicBool::new(false)).unwrap().is_idle());

        let requester = UdpSocket::bind("127.0.0.1:0").unwrap();
        requester
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let query_limits = ShortInfoQueryWireLimits::new(256, 512).unwrap();
        let query = ShortInfoQuery::new(
            "source_id='interop-source'".into(),
            requester.local_addr().unwrap().port(),
            71,
            query_limits,
        )
        .unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        requester
            .send_to(wire.as_bytes(), service.discovery_local_address())
            .unwrap();
        let handled = (0..100)
            .find_map(|_| {
                let poll = service.poll(&AtomicBool::new(false)).unwrap();
                if poll.discovery().is_none() {
                    thread::sleep(Duration::from_millis(1));
                }
                poll.discovery()
            })
            .unwrap();
        assert_eq!(handled.query_id(), 71);
        let mut response = [0u8; 2048];
        let length = requester.recv(&mut response).unwrap();
        assert!(std::str::from_utf8(&response[..length])
            .unwrap()
            .starts_with("71\r\n<?xml version=\"1.0\"?>"));

        let address = service.outlet_local_address();
        let reader = thread::spawn(move || {
            let cancelled = AtomicBool::new(false);
            let mut stream =
                connect_handshake_stream(address, &identity(), handshake_limits(), &cancelled)
                    .unwrap();
            read_initialization_for_channels(&mut stream, 1, sample_limits(), &cancelled).unwrap();
            let mut bytes = [0u8; 26];
            stream.read_exact(&mut bytes).unwrap();
            bytes
        });
        let accepted = (0..2000)
            .find_map(|_| {
                let poll = service.poll(&AtomicBool::new(false)).unwrap();
                if poll.consumer().is_none() {
                    thread::sleep(Duration::from_millis(1));
                }
                poll.consumer()
            })
            .unwrap();
        assert_eq!(accepted.connected_consumers(), 1);
        assert!(service.poll(&AtomicBool::new(false)).unwrap().is_idle());
        let timestamps = [
            RawSourceTimestamp::new(10.0).unwrap(),
            RawSourceTimestamp::new(10.01).unwrap(),
        ];
        let report = service
            .push_chunk(&[1.25, 2.5], &timestamps, &AtomicBool::new(false))
            .unwrap();
        assert_eq!(report.complete_deliveries(), 1);
        let bytes = reader.join().unwrap();
        assert_eq!(
            bytes[0],
            crate::timestamped_float32_session_runtime::codec::RECORD_MARKER
        );
        assert_eq!(
            bytes[13],
            crate::timestamped_float32_session_runtime::codec::RECORD_MARKER
        );
        assert_eq!(f32::from_le_bytes(bytes[9..13].try_into().unwrap()), 1.25);
        assert_eq!(f32::from_le_bytes(bytes[22..26].try_into().unwrap()), 2.5);
        assert_eq!(service.close().outlet().closed_consumers(), 1);
    }

    #[test]
    fn interop_001_rejects_mismatched_body_and_cancelled_poll() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let outlet = outlet(listener, 1);
        let text = body(outlet.local_address().port(), 2);
        let error = PersistentFloat32OutletService::new_prebound(
            responder_activation(),
            Ipv4Addr::LOCALHOST,
            outlet,
            UdpSocket::bind("127.0.0.1:0").unwrap(),
            text.clone(),
            limits(text.len()),
        )
        .err()
        .unwrap();
        assert_eq!(
            error,
            PersistentFloat32OutletServiceCreateError::ChannelCountMismatch {
                advertised: 2,
                outlet: 1
            }
        );

        let mut service = service();
        let cancelled = AtomicBool::new(true);
        assert_eq!(
            service.poll(&cancelled),
            Err(PersistentFloat32OutletServicePollError::Cancelled)
        );
    }

    #[test]
    fn interop_001_limits_reject_zero_and_oversized_datagrams() {
        let document = StreamInfoObservedDocumentParseLimit::new(1).unwrap();
        let admission = StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(1, 1, 1, 1).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 1, 1).unwrap(),
            StreamInfoVolatileFieldLimits::new(1, 1, 1).unwrap(),
        );
        let query = ShortInfoQueryWireLimits::new(1, 1).unwrap();
        let response = ShortInfoResponseEnvelopeLimits::new(1, 1).unwrap();
        assert_eq!(
            PersistentFloat32OutletServiceLimits::new(0, document, admission, query, response),
            Err(PersistentFloat32OutletServiceLimitError::ZeroDatagramBytes)
        );
        assert!(matches!(
            PersistentFloat32OutletServiceLimits::new(
                MAX_DISCOVERY_DATAGRAM_BYTES + 1,
                document,
                admission,
                query,
                response
            ),
            Err(PersistentFloat32OutletServiceLimitError::DatagramLimitExceeded { .. })
        ));
    }
}
