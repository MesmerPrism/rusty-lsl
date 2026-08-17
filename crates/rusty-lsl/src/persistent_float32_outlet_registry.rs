// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded caller-polled ownership of multiple persistent Float32 outlets.

use crate::persistent_float32_outlet::PersistentFloat32ManagedRequest;
use crate::{
    ParsedShortInfoQuery, PersistentFloat32AcceptError, PersistentFloat32ConsumerAccepted,
    PersistentFloat32DiscoveryHandled, PersistentFloat32FullInfoServed, PersistentFloat32Outlet,
    PersistentFloat32OutletHealth, PersistentFloat32OutletServiceCreateError,
    PersistentFloat32OutletServiceLimits, PersistentFloat32OutletServicePollError,
    PersistentFloat32PushError, PersistentFloat32PushReport, PersistentFloat32StreamInfo,
    PersistentFloat32TimedataHandled, RawSourceTimestamp, ShortInfoResponderActivation,
};
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};

/// Fixed upper bound admitted for one registry.
pub const MAX_PERSISTENT_FLOAT32_REGISTRY_OUTLETS: usize = 64;

/// Stable index assigned to an outlet for the registry lifetime.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PersistentFloat32OutletId(usize);

impl PersistentFloat32OutletId {
    /// Zero-based stable index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Bounded retained-resource policy for a multi-outlet registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletRegistryLimits {
    max_outlets: usize,
    service: PersistentFloat32OutletServiceLimits,
}

impl PersistentFloat32OutletRegistryLimits {
    /// Admits a nonzero outlet bound no larger than the public fixed ceiling.
    ///
    /// # Errors
    ///
    /// Rejects zero or an extent above [`MAX_PERSISTENT_FLOAT32_REGISTRY_OUTLETS`].
    pub fn new(
        max_outlets: usize,
        service: PersistentFloat32OutletServiceLimits,
    ) -> Result<Self, PersistentFloat32OutletRegistryLimitError> {
        if max_outlets == 0 {
            return Err(PersistentFloat32OutletRegistryLimitError::ZeroOutlets);
        }
        if max_outlets > MAX_PERSISTENT_FLOAT32_REGISTRY_OUTLETS {
            return Err(
                PersistentFloat32OutletRegistryLimitError::OutletLimitExceeded {
                    actual: max_outlets,
                    limit: MAX_PERSISTENT_FLOAT32_REGISTRY_OUTLETS,
                },
            );
        }
        Ok(Self {
            max_outlets,
            service,
        })
    }

    /// Maximum retained outlets.
    #[must_use]
    pub const fn max_outlets(self) -> usize {
        self.max_outlets
    }
}

/// Invalid registry limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletRegistryLimitError {
    /// No outlet could ever be registered.
    ZeroOutlets,
    /// Requested outlet extent exceeded the fixed public ceiling.
    OutletLimitExceeded {
        /// Requested extent.
        actual: usize,
        /// Fixed ceiling.
        limit: usize,
    },
}

/// Failure before a registry becomes usable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletRegistryCreateError {
    /// The selected interface was unspecified, multicast, or broadcast.
    NonConcreteIpv4Interface,
    /// The discovery socket could not be inspected.
    DiscoveryLocalAddress(ErrorKind),
    /// The discovery socket could not be made nonblocking.
    DiscoveryNonblocking(ErrorKind),
    /// The bounded discovery allocation failed.
    DiscoveryBufferAllocationFailed {
        /// Exact requested capacity.
        requested: usize,
    },
    /// The discovery probe length overflowed.
    DiscoveryProbeLengthOverflow,
    /// The bounded outlet allocation failed.
    OutletAllocationFailed {
        /// Exact requested capacity.
        requested: usize,
    },
}

/// Rejected outlet registration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletRegistrationError {
    /// The selected registry capacity was already occupied.
    Capacity {
        /// Selected outlet capacity.
        limit: usize,
    },
    /// Another entry retained the same handshake UID.
    DuplicateUid,
    /// Another entry retained the same TCP/UDP service port.
    DuplicateServicePort,
    /// The body, listener, or timedata endpoint failed the single-service contract.
    Service(PersistentFloat32OutletServiceCreateError),
}

/// Discovery work completed for all registered outlets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32RegistryDiscoveryHandled {
    base: PersistentFloat32DiscoveryHandled,
    responses: usize,
}

impl PersistentFloat32RegistryDiscoveryHandled {
    /// Socket that supplied the query.
    #[must_use]
    pub const fn source(self) -> SocketAddr {
        self.base.source()
    }

    /// Query-selected response destination.
    #[must_use]
    pub const fn destination(self) -> SocketAddr {
        self.base.destination()
    }

    /// Opaque query correlation value.
    #[must_use]
    pub const fn query_id(self) -> u64 {
        self.base.query_id()
    }

    /// Number of canonical stream-info responses emitted.
    #[must_use]
    pub const fn responses(self) -> usize {
        self.responses
    }
}

/// Timedata work completed for one registered outlet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PersistentFloat32RegistryTimedataHandled {
    outlet: PersistentFloat32OutletId,
    exchange: PersistentFloat32TimedataHandled,
}

impl PersistentFloat32RegistryTimedataHandled {
    /// Outlet whose service port answered the request.
    #[must_use]
    pub const fn outlet(self) -> PersistentFloat32OutletId {
        self.outlet
    }

    /// Exact timedata observation.
    #[must_use]
    pub const fn exchange(self) -> PersistentFloat32TimedataHandled {
        self.exchange
    }
}

/// Consumer admission completed for one registered outlet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32RegistryConsumerAccepted {
    outlet: PersistentFloat32OutletId,
    accepted: PersistentFloat32ConsumerAccepted,
}

/// Full-info response completed for one registered outlet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32RegistryFullInfoServed {
    outlet: PersistentFloat32OutletId,
    served: PersistentFloat32FullInfoServed,
}

impl PersistentFloat32RegistryFullInfoServed {
    /// Outlet that answered the auxiliary request.
    #[must_use]
    pub const fn outlet(self) -> PersistentFloat32OutletId {
        self.outlet
    }

    /// Existing full-info response report.
    #[must_use]
    pub const fn served(self) -> PersistentFloat32FullInfoServed {
        self.served
    }
}

impl PersistentFloat32RegistryConsumerAccepted {
    /// Outlet that admitted the consumer.
    #[must_use]
    pub const fn outlet(self) -> PersistentFloat32OutletId {
        self.outlet
    }

    /// Existing persistent-consumer admission report.
    #[must_use]
    pub const fn accepted(self) -> PersistentFloat32ConsumerAccepted {
        self.accepted
    }
}

/// Bounded work completed by one registry poll.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PersistentFloat32OutletRegistryPoll {
    discovery: Option<PersistentFloat32RegistryDiscoveryHandled>,
    timedata: Option<PersistentFloat32RegistryTimedataHandled>,
    consumer: Option<PersistentFloat32RegistryConsumerAccepted>,
    full_info: Option<PersistentFloat32RegistryFullInfoServed>,
}

impl PersistentFloat32OutletRegistryPoll {
    /// Discovery query handled by this poll, if any.
    #[must_use]
    pub const fn discovery(self) -> Option<PersistentFloat32RegistryDiscoveryHandled> {
        self.discovery
    }

    /// Timedata query handled by this poll, if any.
    #[must_use]
    pub const fn timedata(self) -> Option<PersistentFloat32RegistryTimedataHandled> {
        self.timedata
    }

    /// Consumer admitted by this poll, if any.
    #[must_use]
    pub const fn consumer(self) -> Option<PersistentFloat32RegistryConsumerAccepted> {
        self.consumer
    }

    /// Full-info auxiliary request answered by this poll, if any.
    #[must_use]
    pub const fn full_info(self) -> Option<PersistentFloat32RegistryFullInfoServed> {
        self.full_info
    }

    /// Whether no socket had work at the selected round-robin positions.
    #[must_use]
    pub const fn is_idle(self) -> bool {
        self.discovery.is_none()
            && self.timedata.is_none()
            && self.consumer.is_none()
            && self.full_info.is_none()
    }
}

/// Failure while one bounded registry poll executes.
#[derive(Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletRegistryPollError {
    /// Caller cancellation was selected before socket work.
    Cancelled,
    /// Shared discovery handling failed.
    Discovery(PersistentFloat32OutletServicePollError),
    /// Per-outlet timedata handling failed.
    Timedata {
        /// Stable outlet index.
        outlet: PersistentFloat32OutletId,
        /// Existing timedata failure contract.
        error: PersistentFloat32OutletServicePollError,
    },
    /// Per-outlet consumer admission failed.
    Accept {
        /// Stable outlet index.
        outlet: PersistentFloat32OutletId,
        /// Existing consumer-admission failure contract.
        error: PersistentFloat32AcceptError,
    },
}

/// Cumulative bounded registry observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletRegistryHealth {
    outlets: usize,
    discovery_queries: u64,
    discovery_responses: u64,
    timedata_queries: u64,
    consumers_accepted: u64,
    full_info_responses: u64,
}

impl PersistentFloat32OutletRegistryHealth {
    /// Currently retained outlets.
    #[must_use]
    pub const fn outlets(self) -> usize {
        self.outlets
    }

    /// Syntactically admitted shared discovery queries.
    #[must_use]
    pub const fn discovery_queries(self) -> u64 {
        self.discovery_queries
    }

    /// Canonical stream-info responses sent across those queries.
    #[must_use]
    pub const fn discovery_responses(self) -> u64 {
        self.discovery_responses
    }

    /// Timedata requests answered across all service ports.
    #[must_use]
    pub const fn timedata_queries(self) -> u64 {
        self.timedata_queries
    }

    /// Persistent consumers admitted across all outlets.
    #[must_use]
    pub const fn consumers_accepted(self) -> u64 {
        self.consumers_accepted
    }

    /// Exact official full-info requests answered across registered outlets.
    #[must_use]
    pub const fn full_info_responses(self) -> u64 {
        self.full_info_responses
    }
}

/// Explicit registry close accounting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletRegistryCloseReport {
    outlets: usize,
    consumers: usize,
    auxiliaries: usize,
}

impl PersistentFloat32OutletRegistryCloseReport {
    /// Outlets closed by the registry.
    #[must_use]
    pub const fn outlets(self) -> usize {
        self.outlets
    }

    /// Retained data consumers closed across those outlets.
    #[must_use]
    pub const fn consumers(self) -> usize {
        self.consumers
    }

    /// Retained full-info auxiliary connections closed across all outlets.
    #[must_use]
    pub const fn auxiliaries(self) -> usize {
        self.auxiliaries
    }
}

struct RegistryEntry {
    outlet: PersistentFloat32Outlet,
    body: String,
    timedata: super::persistent_float32_outlet_service::PersistentFloat32TimedataSocket,
}

/// One shared discovery socket with bounded round-robin outlet service.
pub struct PersistentFloat32OutletRegistry {
    discovery: UdpSocket,
    discovery_local: SocketAddr,
    advertised_ipv4: Ipv4Addr,
    limits: PersistentFloat32OutletRegistryLimits,
    receive: Vec<u8>,
    entries: Vec<RegistryEntry>,
    timedata_cursor: usize,
    consumer_cursor: usize,
    discovery_queries: u64,
    discovery_responses: u64,
    timedata_queries: u64,
    consumers_accepted: u64,
    full_info_responses: u64,
}

impl PersistentFloat32OutletRegistry {
    /// Creates an empty registry around one caller-bound discovery socket.
    ///
    /// The caller owns multicast binding and membership. No worker is spawned.
    ///
    /// # Errors
    ///
    /// Rejects an invalid advertised interface or bounded allocation/socket setup failure.
    pub fn new_prebound(
        _activation: ShortInfoResponderActivation,
        advertised_ipv4: Ipv4Addr,
        discovery: UdpSocket,
        limits: PersistentFloat32OutletRegistryLimits,
    ) -> Result<Self, PersistentFloat32OutletRegistryCreateError> {
        if advertised_ipv4.is_unspecified()
            || advertised_ipv4.is_multicast()
            || advertised_ipv4 == Ipv4Addr::BROADCAST
        {
            return Err(PersistentFloat32OutletRegistryCreateError::NonConcreteIpv4Interface);
        }
        let discovery_local = discovery.local_addr().map_err(|error| {
            PersistentFloat32OutletRegistryCreateError::DiscoveryLocalAddress(error.kind())
        })?;
        discovery.set_nonblocking(true).map_err(|error| {
            PersistentFloat32OutletRegistryCreateError::DiscoveryNonblocking(error.kind())
        })?;
        let probe = limits
            .service
            .max_datagram_bytes()
            .checked_add(1)
            .ok_or(PersistentFloat32OutletRegistryCreateError::DiscoveryProbeLengthOverflow)?;
        let mut receive = Vec::new();
        receive.try_reserve_exact(probe).map_err(|_| {
            PersistentFloat32OutletRegistryCreateError::DiscoveryBufferAllocationFailed {
                requested: probe,
            }
        })?;
        receive.resize(probe, 0);
        let mut entries = Vec::new();
        entries.try_reserve_exact(limits.max_outlets).map_err(|_| {
            PersistentFloat32OutletRegistryCreateError::OutletAllocationFailed {
                requested: limits.max_outlets,
            }
        })?;
        Ok(Self {
            discovery,
            discovery_local,
            advertised_ipv4,
            limits,
            receive,
            entries,
            timedata_cursor: 0,
            consumer_cursor: 0,
            discovery_queries: 0,
            discovery_responses: 0,
            timedata_queries: 0,
            consumers_accepted: 0,
            full_info_responses: 0,
        })
    }

    /// Registers one fully validated outlet and binds timedata to its service port.
    ///
    /// # Errors
    ///
    /// Rejects capacity, duplicate identity/port, body mismatch, or UDP bind failure.
    pub fn register(
        &mut self,
        outlet: PersistentFloat32Outlet,
        body: String,
    ) -> Result<PersistentFloat32OutletId, PersistentFloat32OutletRegistrationError> {
        self.validate_registration(&outlet)?;
        super::persistent_float32_outlet_service::validate_body(
            &body,
            self.limits.service,
            self.advertised_ipv4,
            &outlet,
        )
        .map_err(PersistentFloat32OutletRegistrationError::Service)?;
        self.install(outlet, body)
    }

    /// Registers stream-info produced by the structured composer.
    ///
    /// This proof-carrying path admits nested channel metadata that the historical
    /// observed-body parser deliberately does not reinterpret.
    ///
    /// # Errors
    ///
    /// Rejects capacity, duplicate identity/port, proof/outlet mismatch, or UDP bind failure.
    pub fn register_stream_info(
        &mut self,
        outlet: PersistentFloat32Outlet,
        stream_info: PersistentFloat32StreamInfo,
    ) -> Result<PersistentFloat32OutletId, PersistentFloat32OutletRegistrationError> {
        self.validate_registration(&outlet)?;
        stream_info
            .validate_outlet(self.advertised_ipv4, &outlet)
            .map_err(PersistentFloat32OutletRegistrationError::Service)?;
        self.install(outlet, stream_info.into_body())
    }

    fn validate_registration(
        &self,
        outlet: &PersistentFloat32Outlet,
    ) -> Result<(), PersistentFloat32OutletRegistrationError> {
        if self.entries.len() == self.limits.max_outlets {
            return Err(PersistentFloat32OutletRegistrationError::Capacity {
                limit: self.limits.max_outlets,
            });
        }
        if self
            .entries
            .iter()
            .any(|entry| entry.outlet.stream_identity().uid() == outlet.stream_identity().uid())
        {
            return Err(PersistentFloat32OutletRegistrationError::DuplicateUid);
        }
        if self
            .entries
            .iter()
            .any(|entry| entry.outlet.local_address().port() == outlet.local_address().port())
        {
            return Err(PersistentFloat32OutletRegistrationError::DuplicateServicePort);
        }
        Ok(())
    }

    fn install(
        &mut self,
        outlet: PersistentFloat32Outlet,
        body: String,
    ) -> Result<PersistentFloat32OutletId, PersistentFloat32OutletRegistrationError> {
        let timedata =
            super::persistent_float32_outlet_service::PersistentFloat32TimedataSocket::bind(
                SocketAddr::new(
                    IpAddr::V4(self.advertised_ipv4),
                    outlet.local_address().port(),
                ),
                self.limits.service.max_datagram_bytes(),
            )
            .map_err(PersistentFloat32OutletRegistrationError::Service)?;
        let id = PersistentFloat32OutletId(self.entries.len());
        self.entries.push(RegistryEntry {
            outlet,
            body,
            timedata,
        });
        Ok(id)
    }

    /// Actual shared discovery socket address.
    #[must_use]
    pub const fn discovery_local_address(&self) -> SocketAddr {
        self.discovery_local
    }

    /// Number of registered outlets.
    #[must_use]
    pub fn outlet_count(&self) -> usize {
        self.entries.len()
    }

    /// Actual TCP listener address for one stable outlet index.
    #[must_use]
    pub fn outlet_local_address(&self, id: PersistentFloat32OutletId) -> Option<SocketAddr> {
        self.entries
            .get(id.0)
            .map(|entry| entry.outlet.local_address())
    }

    /// UDP timedata address for one stable outlet index.
    #[must_use]
    pub fn timedata_local_address(&self, id: PersistentFloat32OutletId) -> Option<SocketAddr> {
        self.entries
            .get(id.0)
            .map(|entry| entry.timedata.local_address())
    }

    /// Current and cumulative health for one stable outlet index.
    #[must_use]
    pub fn outlet_health(
        &self,
        id: PersistentFloat32OutletId,
    ) -> Option<PersistentFloat32OutletHealth> {
        self.entries.get(id.0).map(|entry| entry.outlet.health())
    }

    /// Cumulative registry observations.
    #[must_use]
    pub fn health(&self) -> PersistentFloat32OutletRegistryHealth {
        PersistentFloat32OutletRegistryHealth {
            outlets: self.entries.len(),
            discovery_queries: self.discovery_queries,
            discovery_responses: self.discovery_responses,
            timedata_queries: self.timedata_queries,
            consumers_accepted: self.consumers_accepted,
            full_info_responses: self.full_info_responses,
        }
    }

    /// Executes at most one discovery receive, one timedata socket, and one listener.
    ///
    /// Timedata and consumer work advance round-robin so one hot outlet cannot starve
    /// another. Discovery emits at most the configured bounded outlet count.
    ///
    /// # Errors
    ///
    /// Returns typed cancellation, discovery, timedata, or consumer-admission evidence.
    pub fn poll(
        &mut self,
        cancelled: &AtomicBool,
    ) -> Result<PersistentFloat32OutletRegistryPoll, PersistentFloat32OutletRegistryPollError> {
        if cancelled.load(Ordering::Acquire) {
            return Err(PersistentFloat32OutletRegistryPollError::Cancelled);
        }
        let discovery = self
            .poll_discovery()
            .map_err(PersistentFloat32OutletRegistryPollError::Discovery)?;
        let timedata = self.poll_timedata()?;
        let (consumer, full_info) = self.poll_request(cancelled)?;
        Ok(PersistentFloat32OutletRegistryPoll {
            discovery,
            timedata,
            consumer,
            full_info,
        })
    }

    /// Pushes one fail-fast chunk to one registered outlet.
    ///
    /// # Errors
    ///
    /// Returns `None` for an unknown ID and otherwise preserves the outlet contract.
    pub fn try_push_chunk(
        &mut self,
        id: PersistentFloat32OutletId,
        values: &[f32],
        timestamps: &[RawSourceTimestamp],
        cancelled: &AtomicBool,
    ) -> Option<Result<PersistentFloat32PushReport, PersistentFloat32PushError>> {
        self.entries
            .get_mut(id.0)
            .map(|entry| entry.outlet.try_push_chunk(values, timestamps, cancelled))
    }

    /// Closes every retained data consumer and drops all service sockets.
    #[must_use]
    pub fn close(self) -> PersistentFloat32OutletRegistryCloseReport {
        let outlets = self.entries.len();
        let (consumers, auxiliaries) =
            self.entries
                .into_iter()
                .fold((0usize, 0usize), |(consumers, auxiliaries), entry| {
                    let closed = entry.outlet.close();
                    (
                        consumers + closed.closed_consumers(),
                        auxiliaries + closed.closed_auxiliaries(),
                    )
                });
        PersistentFloat32OutletRegistryCloseReport {
            outlets,
            consumers,
            auxiliaries,
        }
    }

    fn poll_discovery(
        &mut self,
    ) -> Result<
        Option<PersistentFloat32RegistryDiscoveryHandled>,
        PersistentFloat32OutletServicePollError,
    > {
        let (length, source) = match self.discovery.recv_from(&mut self.receive) {
            Ok(received) => received,
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
        let limit = self.limits.service.max_datagram_bytes();
        if length > limit {
            return Err(
                PersistentFloat32OutletServicePollError::DatagramLimitExceeded {
                    limit,
                    actual: length,
                },
            );
        }
        let query_limits = self.limits.service.query_limits();
        let query = ParsedShortInfoQuery::parse(&self.receive[..length], query_limits)
            .map_err(PersistentFloat32OutletServicePollError::Query)?;
        let destination = SocketAddr::new(source.ip(), query.return_port());
        for entry in &self.entries {
            let response_limits = self.limits.service.response_limits();
            let response = super::persistent_float32_outlet_service::encode_retained_body_response(
                query.query_id(),
                &entry.body,
                response_limits,
            )
            .map_err(PersistentFloat32OutletServicePollError::Response)?;
            let sent = self
                .discovery
                .send_to(&response, destination)
                .map_err(|error| PersistentFloat32OutletServicePollError::Send(error.kind()))?;
            if sent != response.len() {
                return Err(PersistentFloat32OutletServicePollError::PartialSend {
                    expected: response.len(),
                    actual: sent,
                });
            }
        }
        let responses = self.entries.len();
        self.discovery_queries = self.discovery_queries.saturating_add(1);
        self.discovery_responses = self.discovery_responses.saturating_add(responses as u64);
        Ok(Some(PersistentFloat32RegistryDiscoveryHandled {
            base: PersistentFloat32DiscoveryHandled::new(source, destination, query.query_id()),
            responses,
        }))
    }

    fn poll_timedata(
        &mut self,
    ) -> Result<
        Option<PersistentFloat32RegistryTimedataHandled>,
        PersistentFloat32OutletRegistryPollError,
    > {
        if self.entries.is_empty() {
            return Ok(None);
        }
        let index = self.timedata_cursor % self.entries.len();
        self.timedata_cursor = (index + 1) % self.entries.len();
        let outlet = PersistentFloat32OutletId(index);
        let exchange = self.entries[index].timedata.poll().map_err(|error| {
            PersistentFloat32OutletRegistryPollError::Timedata { outlet, error }
        })?;
        if exchange.is_some() {
            self.timedata_queries = self.timedata_queries.saturating_add(1);
        }
        Ok(exchange.map(|exchange| PersistentFloat32RegistryTimedataHandled { outlet, exchange }))
    }

    fn poll_request(
        &mut self,
        cancelled: &AtomicBool,
    ) -> Result<
        (
            Option<PersistentFloat32RegistryConsumerAccepted>,
            Option<PersistentFloat32RegistryFullInfoServed>,
        ),
        PersistentFloat32OutletRegistryPollError,
    > {
        if self.entries.is_empty() {
            return Ok((None, None));
        }
        let index = self.consumer_cursor % self.entries.len();
        self.consumer_cursor = (index + 1) % self.entries.len();
        let outlet = PersistentFloat32OutletId(index);
        let entry = &mut self.entries[index];
        let handled = entry
            .outlet
            .poll_managed_request(&entry.body, cancelled)
            .map_err(|error| PersistentFloat32OutletRegistryPollError::Accept { outlet, error })?;
        match handled {
            Some(PersistentFloat32ManagedRequest::Consumer(accepted)) => {
                self.consumers_accepted = self.consumers_accepted.saturating_add(1);
                Ok((
                    Some(PersistentFloat32RegistryConsumerAccepted { outlet, accepted }),
                    None,
                ))
            }
            Some(PersistentFloat32ManagedRequest::FullInfo(served)) => {
                self.full_info_responses = self.full_info_responses.saturating_add(1);
                Ok((
                    None,
                    Some(PersistentFloat32RegistryFullInfoServed { outlet, served }),
                ))
            }
            None => Ok((None, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::stream_handshake::connect_handshake_stream;
    use crate::timestamped_float32_session_runtime::codec::read_initialization_for_channels;
    use crate::{
        MetadataTreeLimits, PersistentFloat32OutletActivation, PersistentFloat32OutletLimits,
        ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
        ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamHandshakeActivation,
        StreamHandshakeIdentity, StreamHandshakeLimits, StreamInfoObservedAdmissionLimits,
        StreamInfoObservedDocumentParseLimit, StreamInfoVolatileFieldLimits,
        TimestampedFloat32SampleActivation, TimestampedFloat32SampleLimits,
    };
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
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

    fn identity(index: usize) -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            format!("73000000-0000-4000-8000-{index:012}"),
            "polar-host".into(),
            format!("polar-source-{index}"),
            "polar-session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn outlet(index: usize, channels: usize) -> PersistentFloat32Outlet {
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
            TcpListener::bind("127.0.0.1:0").unwrap(),
            identity(index),
            handshake_limits(),
            sample_limits(),
            channels,
            PersistentFloat32OutletLimits::new(64, 1).unwrap(),
        )
        .unwrap()
    }

    fn body(index: usize, name: &str, port: u16, channels: usize, rate: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?>\n<info>\n\
\t<name>{name}</name>\n\
\t<type>Polar</type>\n\
\t<channel_count>{channels}</channel_count>\n\
\t<channel_format>float32</channel_format>\n\
\t<source_id>polar-source-{index}</source_id>\n\
\t<nominal_srate>{rate}</nominal_srate>\n\
\t<version>1.100000000000000</version>\n\
\t<created_at>1.0</created_at>\n\
\t<uid>73000000-0000-4000-8000-{index:012}</uid>\n\
\t<session_id>polar-session</session_id>\n\
\t<hostname>polar-host</hostname>\n\
\t<v4address>127.0.0.1</v4address>\n\
\t<v4data_port>{port}</v4data_port>\n\
\t<v4service_port>{port}</v4service_port>\n\
\t<v6address></v6address>\n\
\t<v6data_port>0</v6data_port>\n\
\t<v6service_port>0</v6service_port>\n\
\t<desc />\n</info>\n"
        )
    }

    fn service_limits(max_body: usize) -> PersistentFloat32OutletServiceLimits {
        PersistentFloat32OutletServiceLimits::new(
            4096,
            StreamInfoObservedDocumentParseLimit::new(max_body).unwrap(),
            StreamInfoObservedAdmissionLimits::new(
                StreamDescriptorLimits::new(128, 128, 128, 8).unwrap(),
                MetadataTreeLimits::new(1, 1, 1, 32, 64).unwrap(),
                StreamInfoVolatileFieldLimits::new(128, 128, 128).unwrap(),
            ),
            ShortInfoQueryWireLimits::new(256, 512).unwrap(),
            ShortInfoResponseEnvelopeLimits::new(max_body, max_body + 64).unwrap(),
        )
        .unwrap()
    }

    fn responder_activation() -> ShortInfoResponderActivation {
        ShortInfoResponderActivation::new(test_capability(
            crate::RuntimeModule::ShortInfoDiscoveryResponder,
        ))
        .unwrap()
    }

    #[test]
    fn polar_001_multi_outlet_interop_002_ordering_and_fan_out_are_independent() {
        let ecg = outlet(1, 1);
        let acc = outlet(2, 3);
        let ecg_body = body(
            1,
            "Polar H10 ECG",
            ecg.local_address().port(),
            1,
            "130.0000000000000",
        );
        let acc_body = body(
            2,
            "Polar H10 ACC",
            acc.local_address().port(),
            3,
            "200.0000000000000",
        );
        let max_body = ecg_body.len().max(acc_body.len());
        let mut registry = PersistentFloat32OutletRegistry::new_prebound(
            responder_activation(),
            Ipv4Addr::LOCALHOST,
            UdpSocket::bind("127.0.0.1:0").unwrap(),
            PersistentFloat32OutletRegistryLimits::new(2, service_limits(max_body)).unwrap(),
        )
        .unwrap();
        let ecg_id = registry.register(ecg, ecg_body.clone()).unwrap();
        let acc_id = registry.register(acc, acc_body.clone()).unwrap();
        assert_eq!(ecg_id.index(), 0);
        assert_eq!(acc_id.index(), 1);

        let requester = UdpSocket::bind("127.0.0.1:0").unwrap();
        requester
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let query_limits = ShortInfoQueryWireLimits::new(256, 512).unwrap();
        let query = ShortInfoQuery::new(
            "type='Polar'".into(),
            requester.local_addr().unwrap().port(),
            91,
            query_limits,
        )
        .unwrap();
        let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
        requester
            .send_to(wire.as_bytes(), registry.discovery_local_address())
            .unwrap();
        let discovery = (0..100)
            .find_map(|_| {
                let result = registry.poll(&AtomicBool::new(false)).unwrap();
                if result.discovery().is_none() {
                    thread::sleep(Duration::from_millis(1));
                }
                result.discovery()
            })
            .unwrap();
        assert_eq!(discovery.responses(), 2);
        let mut discovered = Vec::new();
        for _ in 0..2 {
            let mut response = [0_u8; 4096];
            let length = requester.recv(&mut response).unwrap();
            discovered.push(std::str::from_utf8(&response[..length]).unwrap().to_owned());
        }
        assert!(discovered.iter().any(|body| body.contains("Polar H10 ECG")));
        assert!(discovered.iter().any(|body| body.contains("Polar H10 ACC")));

        requester
            .send_to(
                b"LSL:timedata\r\n101 1.0\r\n",
                registry.timedata_local_address(ecg_id).unwrap(),
            )
            .unwrap();
        requester
            .send_to(
                b"LSL:timedata\r\n102 2.0\r\n",
                registry.timedata_local_address(acc_id).unwrap(),
            )
            .unwrap();
        let mut timedata_ids = Vec::new();
        for _ in 0..200 {
            let result = registry.poll(&AtomicBool::new(false)).unwrap();
            if let Some(handled) = result.timedata() {
                timedata_ids.push(handled.exchange().query_id());
                if timedata_ids.len() == 2 {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
        timedata_ids.sort_unstable();
        assert_eq!(timedata_ids, vec![101, 102]);
        let mut timedata_response = [0_u8; 256];
        requester.recv(&mut timedata_response).unwrap();
        requester.recv(&mut timedata_response).unwrap();

        let ecg_address = registry.outlet_local_address(ecg_id).unwrap();
        let ecg_reader = thread::spawn(move || {
            let cancelled = AtomicBool::new(false);
            let mut stream =
                connect_handshake_stream(ecg_address, &identity(1), handshake_limits(), &cancelled)
                    .unwrap();
            read_initialization_for_channels(&mut stream, 1, sample_limits(), &cancelled).unwrap();
            let mut bytes = [0_u8; 13];
            stream.read_exact(&mut bytes).unwrap();
            bytes
        });
        let acc_address = registry.outlet_local_address(acc_id).unwrap();
        let acc_reader = thread::spawn(move || {
            let cancelled = AtomicBool::new(false);
            let mut stream =
                connect_handshake_stream(acc_address, &identity(2), handshake_limits(), &cancelled)
                    .unwrap();
            read_initialization_for_channels(&mut stream, 3, sample_limits(), &cancelled).unwrap();
            let mut bytes = [0_u8; 21];
            stream.read_exact(&mut bytes).unwrap();
            bytes
        });
        let mut accepted = Vec::new();
        for _ in 0..2000 {
            let result = registry.poll(&AtomicBool::new(false)).unwrap();
            if let Some(consumer) = result.consumer() {
                accepted.push(consumer.outlet());
                if accepted.len() == 2 {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
        accepted.sort_by_key(|id| id.index());
        assert_eq!(accepted, vec![ecg_id, acc_id]);

        let ecg_info = thread::spawn(move || {
            let mut stream = TcpStream::connect(ecg_address).unwrap();
            stream.write_all(b"LSL:fullinfo\r\n").unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            response
        });
        let acc_info = thread::spawn(move || {
            let mut stream = TcpStream::connect(acc_address).unwrap();
            stream.write_all(b"LSL:fullinfo\r\n").unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            response
        });
        let mut full_info = Vec::new();
        for _ in 0..2000 {
            let result = registry.poll(&AtomicBool::new(false)).unwrap();
            if let Some(served) = result.full_info() {
                full_info.push(served.outlet());
                if full_info.len() == 2 {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
        full_info.sort_by_key(|id| id.index());
        assert_eq!(full_info, vec![ecg_id, acc_id]);
        assert_eq!(ecg_info.join().unwrap(), ecg_body);
        assert_eq!(acc_info.join().unwrap(), acc_body);
        let timestamp = [RawSourceTimestamp::new(10.0).unwrap()];
        assert_eq!(
            registry
                .try_push_chunk(ecg_id, &[1.25], &timestamp, &AtomicBool::new(false))
                .unwrap()
                .unwrap()
                .complete_deliveries(),
            1
        );
        assert_eq!(
            registry
                .try_push_chunk(
                    acc_id,
                    &[2.0, 3.0, 4.0],
                    &timestamp,
                    &AtomicBool::new(false)
                )
                .unwrap()
                .unwrap()
                .complete_deliveries(),
            1
        );
        let ecg_bytes = ecg_reader.join().unwrap();
        let acc_bytes = acc_reader.join().unwrap();
        assert_eq!(
            f32::from_le_bytes(ecg_bytes[9..13].try_into().unwrap()),
            1.25
        );
        assert_eq!(
            f32::from_le_bytes(acc_bytes[9..13].try_into().unwrap()),
            2.0
        );
        assert_eq!(
            f32::from_le_bytes(acc_bytes[13..17].try_into().unwrap()),
            3.0
        );
        assert_eq!(
            f32::from_le_bytes(acc_bytes[17..21].try_into().unwrap()),
            4.0
        );
        let health = registry.health();
        assert_eq!(health.outlets(), 2);
        assert_eq!(health.discovery_queries(), 1);
        assert_eq!(health.discovery_responses(), 2);
        assert_eq!(health.timedata_queries(), 2);
        assert_eq!(health.consumers_accepted(), 2);
        assert_eq!(health.full_info_responses(), 2);
        let close = registry.close();
        assert_eq!(close.outlets(), 2);
        assert_eq!(close.consumers(), 2);
        assert_eq!(close.auxiliaries(), 2);
    }
}
