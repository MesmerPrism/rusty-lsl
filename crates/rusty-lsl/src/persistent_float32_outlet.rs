// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Caller-owned persistent Float32 outlet with bounded allocation-free chunk fan-out.

use crate::bounded_fixed_record_transport::{
    write_exact_bounded_with_state_progress, BoundedFixedRecordError, BoundedWriteState,
};
use crate::stream_handshake::admit_accepted_handshake_stream_with_format;
use crate::timestamped_float32_session_runtime::codec::{Float32WriterState, RECORD_MARKER};
use crate::{
    RawSourceTimestamp, RuntimeModule, RuntimeModuleCapability, StreamHandshakeError,
    StreamHandshakeIdentity, StreamHandshakeLimits, TimestampedFloat32SampleActivation,
    TimestampedFloat32SampleError, TimestampedFloat32SampleLimits,
};
use std::io::ErrorKind;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};

const FRAME_PREFIX_BYTES: usize = 1 + core::mem::size_of::<f64>();
const MAX_REUSABLE_CHUNK_BYTES: usize = 16 * 1024 * 1024;
const MAX_CONSUMERS: usize = 64;

/// Stable marker naming this public persistent-outlet API surface.
pub const PERSISTENT_FLOAT32_OUTLET_API_MARKER: &str = "rusty.lsl.persistent_float32_outlet.api";
/// Feature identity admitted by the complete runtime lock.
pub const PERSISTENT_FLOAT32_OUTLET_FEATURE_ID: &str = "persistent-float32-outlet";
/// Consumer-observed effective marker admitted by the complete runtime lock.
pub const PERSISTENT_FLOAT32_OUTLET_EFFECTIVE_MARKER: &str =
    "rusty.lsl.persistent_float32_outlet.effective";

/// Proof that the caller explicitly activated the admitted Float32 transport capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletActivation {
    _capability: RuntimeModuleCapability,
    sample: TimestampedFloat32SampleActivation,
}

impl PersistentFloat32OutletActivation {
    /// Composes the admitted persistent-outlet capability with its Float32 dependency.
    ///
    /// # Errors
    ///
    /// Returns [`PersistentFloat32OutletActivationError::WrongModule`] when
    /// the capability belongs to a different selected runtime module.
    pub const fn new(
        capability: RuntimeModuleCapability,
        sample: TimestampedFloat32SampleActivation,
    ) -> Result<Self, PersistentFloat32OutletActivationError> {
        if !capability.matches(RuntimeModule::PersistentFloat32Outlet) {
            return Err(PersistentFloat32OutletActivationError::WrongModule);
        }
        Ok(Self {
            _capability: capability,
            sample,
        })
    }

    fn sample(self) -> TimestampedFloat32SampleActivation {
        self.sample
    }
}

/// Failure to compose the exact persistent Float32 outlet capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletActivationError {
    /// The capability belongs to a different selected runtime module.
    WrongModule,
}

/// Caller-selected retained-resource bounds for one outlet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletLimits {
    max_records_per_chunk: usize,
    max_consumers: usize,
}

impl PersistentFloat32OutletLimits {
    /// Admits nonzero limits within the crate's fixed retained-memory bounds.
    ///
    /// # Errors
    ///
    /// Returns a typed error for a zero bound or a consumer bound above the
    /// fixed implementation ceiling.
    pub fn new(
        max_records_per_chunk: usize,
        max_consumers: usize,
    ) -> Result<Self, PersistentFloat32OutletLimitError> {
        if max_records_per_chunk == 0 {
            return Err(PersistentFloat32OutletLimitError::ZeroRecordsPerChunk);
        }
        if max_consumers == 0 {
            return Err(PersistentFloat32OutletLimitError::ZeroConsumers);
        }
        if max_consumers > MAX_CONSUMERS {
            return Err(PersistentFloat32OutletLimitError::ConsumerLimitExceeded {
                actual: max_consumers,
                limit: MAX_CONSUMERS,
            });
        }
        Ok(Self {
            max_records_per_chunk,
            max_consumers,
        })
    }

    /// Maximum records accepted by one `push_chunk` call.
    #[must_use]
    pub const fn max_records_per_chunk(self) -> usize {
        self.max_records_per_chunk
    }

    /// Maximum simultaneously retained consumers.
    #[must_use]
    pub const fn max_consumers(self) -> usize {
        self.max_consumers
    }
}

/// Rejected retained-resource limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletLimitError {
    /// The chunk record limit was zero.
    ZeroRecordsPerChunk,
    /// The consumer limit was zero.
    ZeroConsumers,
    /// The requested consumer bound exceeded the fixed implementation ceiling.
    ConsumerLimitExceeded {
        /// Requested consumers.
        actual: usize,
        /// Fixed ceiling.
        limit: usize,
    },
}

/// Failure before a persistent outlet becomes usable.
#[derive(Debug, Eq, PartialEq)]
pub enum PersistentFloat32OutletCreateError {
    /// Channel count was zero.
    ZeroChannels,
    /// The reusable chunk byte length overflowed `usize`.
    ChunkSizeOverflow,
    /// The selected shape exceeded the fixed retained-buffer ceiling.
    ChunkBufferLimitExceeded {
        /// Requested retained bytes.
        requested: usize,
        /// Fixed ceiling.
        limit: usize,
    },
    /// Reusable chunk-buffer allocation failed.
    ChunkBufferAllocationFailed {
        /// Requested retained bytes.
        requested: usize,
    },
    /// Consumer registry allocation failed.
    ConsumerRegistryAllocationFailed {
        /// Requested retained consumer slots.
        requested: usize,
    },
    /// Listener inspection or configuration failed.
    Io(ErrorKind),
}

/// Stable transport failure for one retained consumer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32TransportError {
    /// Caller cancellation was observed.
    Cancelled,
    /// The selected total write deadline elapsed.
    Deadline,
    /// The socket operation failed.
    Io(ErrorKind),
    /// The peer closed before the selected bytes completed.
    Truncated {
        /// Byte prefix observed before closure.
        actual: usize,
    },
}

/// Failure while admitting one pending consumer.
#[derive(Debug, Eq, PartialEq)]
pub enum PersistentFloat32AcceptError {
    /// Caller cancellation was observed before accepting a socket.
    Cancelled,
    /// The retained consumer registry is full.
    ConsumerCapacityReached {
        /// Selected maximum consumers.
        limit: usize,
    },
    /// Listener accept or socket configuration failed.
    Io(ErrorKind),
    /// Protocol-110 connection setup failed.
    Handshake(StreamHandshakeError),
    /// The required two-record Float32 initialization failed.
    Initialization(TimestampedFloat32SampleError),
}

/// Evidence that one pending consumer completed setup and is now retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32ConsumerAccepted {
    peer: SocketAddr,
    connected_consumers: usize,
}

impl PersistentFloat32ConsumerAccepted {
    /// Remote socket address of the accepted consumer.
    #[must_use]
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }

    /// Retained consumer count after admission.
    #[must_use]
    pub const fn connected_consumers(&self) -> usize {
        self.connected_consumers
    }
}

/// Invalid caller input rejected before chunk bytes are written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentFloat32PushError {
    /// Caller cancellation was already selected.
    Cancelled,
    /// No records were supplied.
    EmptyChunk,
    /// Record count exceeded the selected limit.
    RecordLimitExceeded {
        /// Supplied records.
        actual: usize,
        /// Selected maximum.
        limit: usize,
    },
    /// Flat values did not equal `records * channels`.
    ValueCountMismatch {
        /// Supplied values.
        actual: usize,
        /// Required values.
        expected: usize,
    },
}

/// First failed consumer retained in a no-allocation fan-out report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32ConsumerFailure {
    peer: SocketAddr,
    written_bytes: usize,
    completed_records: usize,
    error: PersistentFloat32TransportError,
}

impl PersistentFloat32ConsumerFailure {
    /// Remote socket address of the failed consumer.
    #[must_use]
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }

    /// Chunk byte prefix accepted by the socket before failure.
    #[must_use]
    pub const fn written_bytes(&self) -> usize {
        self.written_bytes
    }

    /// Whole records contained in that accepted prefix.
    #[must_use]
    pub const fn completed_records(&self) -> usize {
        self.completed_records
    }

    /// Stable failure classification.
    #[must_use]
    pub const fn error(&self) -> PersistentFloat32TransportError {
        self.error
    }
}

/// Completed one-call chunk fan-out accounting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32PushReport {
    record_count: usize,
    consumers_before: usize,
    consumers_after: usize,
    complete_deliveries: usize,
    failed_consumers: usize,
    first_failure: Option<PersistentFloat32ConsumerFailure>,
}

impl PersistentFloat32PushReport {
    /// Records encoded once for this call.
    #[must_use]
    pub const fn record_count(&self) -> usize {
        self.record_count
    }

    /// Consumers present before fan-out.
    #[must_use]
    pub const fn consumers_before(&self) -> usize {
        self.consumers_before
    }

    /// Consumers retained after failed sockets were removed.
    #[must_use]
    pub const fn consumers_after(&self) -> usize {
        self.consumers_after
    }

    /// Consumers that accepted the complete contiguous chunk.
    #[must_use]
    pub const fn complete_deliveries(&self) -> usize {
        self.complete_deliveries
    }

    /// Consumers removed after an incomplete write.
    #[must_use]
    pub const fn failed_consumers(&self) -> usize {
        self.failed_consumers
    }

    /// First failed consumer, if any; no failure collection is allocated.
    #[must_use]
    pub const fn first_failure(&self) -> Option<PersistentFloat32ConsumerFailure> {
        self.first_failure
    }
}

/// Explicit close accounting for caller-owned resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32OutletCloseReport {
    closed_consumers: usize,
}

impl PersistentFloat32OutletCloseReport {
    /// Consumers shut down during explicit close.
    #[must_use]
    pub const fn closed_consumers(&self) -> usize {
        self.closed_consumers
    }
}

struct Consumer {
    stream: TcpStream,
    peer: SocketAddr,
    transport: BoundedWriteState,
}

/// Caller-owned persistent listener, reusable chunk buffer, and retained consumers.
pub struct PersistentFloat32Outlet {
    listener: TcpListener,
    local: SocketAddr,
    identity: StreamHandshakeIdentity,
    handshake_limits: StreamHandshakeLimits,
    sample_limits: TimestampedFloat32SampleLimits,
    channel_count: usize,
    record_bytes: usize,
    limits: PersistentFloat32OutletLimits,
    chunk: Vec<u8>,
    consumers: Vec<Consumer>,
}

impl PersistentFloat32Outlet {
    /// Creates a nonblocking listener owner and allocates all reusable push storage.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the channel shape is invalid, retained byte
    /// arithmetic overflows or exceeds the fixed ceiling, allocation fails, or
    /// the listener cannot be inspected or configured.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        activation: PersistentFloat32OutletActivation,
        listener: TcpListener,
        identity: StreamHandshakeIdentity,
        handshake_limits: StreamHandshakeLimits,
        sample_limits: TimestampedFloat32SampleLimits,
        channel_count: usize,
        limits: PersistentFloat32OutletLimits,
    ) -> Result<Self, PersistentFloat32OutletCreateError> {
        let _ = activation.sample();
        if channel_count == 0 {
            return Err(PersistentFloat32OutletCreateError::ZeroChannels);
        }
        let record_bytes = channel_count
            .checked_mul(core::mem::size_of::<f32>())
            .and_then(|bytes| bytes.checked_add(FRAME_PREFIX_BYTES))
            .ok_or(PersistentFloat32OutletCreateError::ChunkSizeOverflow)?;
        let chunk_bytes = record_bytes
            .checked_mul(limits.max_records_per_chunk)
            .ok_or(PersistentFloat32OutletCreateError::ChunkSizeOverflow)?;
        if chunk_bytes > MAX_REUSABLE_CHUNK_BYTES {
            return Err(
                PersistentFloat32OutletCreateError::ChunkBufferLimitExceeded {
                    requested: chunk_bytes,
                    limit: MAX_REUSABLE_CHUNK_BYTES,
                },
            );
        }
        let mut chunk = Vec::new();
        chunk.try_reserve_exact(chunk_bytes).map_err(|_| {
            PersistentFloat32OutletCreateError::ChunkBufferAllocationFailed {
                requested: chunk_bytes,
            }
        })?;
        chunk.resize(chunk_bytes, 0);
        let mut consumers = Vec::new();
        consumers
            .try_reserve_exact(limits.max_consumers)
            .map_err(
                |_| PersistentFloat32OutletCreateError::ConsumerRegistryAllocationFailed {
                    requested: limits.max_consumers,
                },
            )?;
        let local = listener
            .local_addr()
            .map_err(|error| PersistentFloat32OutletCreateError::Io(error.kind()))?;
        listener
            .set_nonblocking(true)
            .map_err(|error| PersistentFloat32OutletCreateError::Io(error.kind()))?;
        Ok(Self {
            listener,
            local,
            identity,
            handshake_limits,
            sample_limits,
            channel_count,
            record_bytes,
            limits,
            chunk,
            consumers,
        })
    }

    /// Actual local listener address.
    #[must_use]
    pub const fn local_address(&self) -> SocketAddr {
        self.local
    }

    /// Fixed channel count of every pushed record.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.channel_count
    }

    /// Handshake identity bound to discovery metadata by caller-owned compositions.
    #[must_use]
    pub const fn stream_identity(&self) -> &StreamHandshakeIdentity {
        &self.identity
    }

    /// Number of consumers currently retained for fan-out.
    #[must_use]
    pub fn connected_consumers(&self) -> usize {
        self.consumers.len()
    }

    /// Fixed retained-consumer bound selected at construction.
    #[must_use]
    pub const fn max_consumers(&self) -> usize {
        self.limits.max_consumers
    }

    /// Admits at most one pending consumer; idle polling returns immediately.
    ///
    /// # Errors
    ///
    /// Returns a typed error for cancellation, full capacity, socket setup,
    /// handshake admission, or required Float32 initialization failure.
    pub fn poll_accept_consumer(
        &mut self,
        cancelled: &AtomicBool,
    ) -> Result<Option<PersistentFloat32ConsumerAccepted>, PersistentFloat32AcceptError> {
        if cancelled.load(Ordering::Acquire) {
            return Err(PersistentFloat32AcceptError::Cancelled);
        }
        if self.consumers.len() == self.limits.max_consumers {
            return Err(PersistentFloat32AcceptError::ConsumerCapacityReached {
                limit: self.limits.max_consumers,
            });
        }
        let (mut stream, peer) = match self.listener.accept() {
            Ok(accepted) => accepted,
            Err(error) if error.kind() == ErrorKind::WouldBlock => return Ok(None),
            Err(error) => return Err(PersistentFloat32AcceptError::Io(error.kind())),
        };
        stream
            .set_nodelay(true)
            .map_err(|error| PersistentFloat32AcceptError::Io(error.kind()))?;
        admit_accepted_handshake_stream_with_format(
            &mut stream,
            &self.identity,
            self.handshake_limits,
            cancelled,
            core::mem::size_of::<f32>(),
            true,
        )
        .map_err(PersistentFloat32AcceptError::Handshake)?;
        Float32WriterState::new(self.channel_count)
            .and_then(|mut writer| {
                writer.write_initialization(&mut stream, self.sample_limits, cancelled)
            })
            .map_err(PersistentFloat32AcceptError::Initialization)?;
        self.consumers.push(Consumer {
            stream,
            peer,
            transport: BoundedWriteState::new(),
        });
        Ok(Some(PersistentFloat32ConsumerAccepted {
            peer,
            connected_consumers: self.consumers.len(),
        }))
    }

    /// Encodes once and performs one contiguous bounded write per retained consumer.
    ///
    /// # Errors
    ///
    /// Rejects preselected cancellation, an empty or oversized record extent,
    /// or a flat value extent that differs from `records * channels` before I/O.
    /// Per-consumer transport failures are retained in the successful report.
    pub fn push_chunk(
        &mut self,
        values: &[f32],
        timestamps: &[RawSourceTimestamp],
        cancelled: &AtomicBool,
    ) -> Result<PersistentFloat32PushReport, PersistentFloat32PushError> {
        if cancelled.load(Ordering::Acquire) {
            return Err(PersistentFloat32PushError::Cancelled);
        }
        let record_count = timestamps.len();
        if record_count == 0 {
            return Err(PersistentFloat32PushError::EmptyChunk);
        }
        if record_count > self.limits.max_records_per_chunk {
            return Err(PersistentFloat32PushError::RecordLimitExceeded {
                actual: record_count,
                limit: self.limits.max_records_per_chunk,
            });
        }
        let expected_values = record_count * self.channel_count;
        if values.len() != expected_values {
            return Err(PersistentFloat32PushError::ValueCountMismatch {
                actual: values.len(),
                expected: expected_values,
            });
        }
        let encoded_bytes = record_count * self.record_bytes;
        for (record_index, timestamp) in timestamps.iter().enumerate() {
            let start = record_index * self.record_bytes;
            let record = &mut self.chunk[start..start + self.record_bytes];
            record[0] = RECORD_MARKER;
            record[1..FRAME_PREFIX_BYTES].copy_from_slice(&timestamp.value().to_le_bytes());
            let value_start = record_index * self.channel_count;
            for (encoded, value) in record[FRAME_PREFIX_BYTES..]
                .chunks_exact_mut(core::mem::size_of::<f32>())
                .zip(&values[value_start..value_start + self.channel_count])
            {
                encoded.copy_from_slice(&value.to_le_bytes());
            }
        }
        let consumers_before = self.consumers.len();
        let mut complete_deliveries = 0;
        let mut failed_consumers = 0;
        let mut first_failure = None;
        let mut index = 0;
        while index < self.consumers.len() {
            let result = {
                let consumer = &mut self.consumers[index];
                write_exact_bounded_with_state_progress(
                    &mut consumer.stream,
                    &self.chunk[..encoded_bytes],
                    self.sample_limits.io_slice(),
                    self.sample_limits.total_deadline(),
                    cancelled,
                    &mut consumer.transport,
                )
            };
            match result {
                Ok(()) => {
                    complete_deliveries += 1;
                    index += 1;
                }
                Err(failure) => {
                    let consumer = self.consumers.remove(index);
                    let _ = consumer.stream.shutdown(Shutdown::Both);
                    failed_consumers += 1;
                    if first_failure.is_none() {
                        first_failure = Some(PersistentFloat32ConsumerFailure {
                            peer: consumer.peer,
                            written_bytes: failure.written,
                            completed_records: failure.written / self.record_bytes,
                            error: map_transport_error(failure.error),
                        });
                    }
                }
            }
        }
        Ok(PersistentFloat32PushReport {
            record_count,
            consumers_before,
            consumers_after: self.consumers.len(),
            complete_deliveries,
            failed_consumers,
            first_failure,
        })
    }

    /// Shuts down all retained consumers and releases the listener on return.
    #[must_use]
    pub fn close(mut self) -> PersistentFloat32OutletCloseReport {
        let closed_consumers = self.consumers.len();
        self.shutdown_consumers();
        PersistentFloat32OutletCloseReport { closed_consumers }
    }

    fn shutdown_consumers(&mut self) {
        for consumer in self.consumers.drain(..) {
            let _ = consumer.stream.shutdown(Shutdown::Both);
        }
    }

    #[cfg(test)]
    pub(crate) fn buffer_identity(&self) -> (*const u8, usize, usize) {
        (self.chunk.as_ptr(), self.chunk.len(), self.chunk.capacity())
    }
}

impl Drop for PersistentFloat32Outlet {
    fn drop(&mut self) {
        self.shutdown_consumers();
    }
}

fn map_transport_error(error: BoundedFixedRecordError) -> PersistentFloat32TransportError {
    match error {
        BoundedFixedRecordError::Cancelled => PersistentFloat32TransportError::Cancelled,
        BoundedFixedRecordError::Deadline => PersistentFloat32TransportError::Deadline,
        BoundedFixedRecordError::Io(kind) => PersistentFloat32TransportError::Io(kind),
        BoundedFixedRecordError::Truncated { actual } => {
            PersistentFloat32TransportError::Truncated { actual }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::stream_handshake::connect_handshake_stream;
    use crate::timestamped_float32_session_runtime::codec::{
        read_initialization_for_channels, read_record_for_channels,
    };
    use crate::{RuntimeModule, StreamHandshakeActivation};
    use std::thread;
    use std::time::Duration;

    fn sample_activation() -> TimestampedFloat32SampleActivation {
        TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap(),
        )
        .unwrap()
    }

    fn activation() -> PersistentFloat32OutletActivation {
        PersistentFloat32OutletActivation::new(
            test_capability(RuntimeModule::PersistentFloat32Outlet),
            sample_activation(),
        )
        .unwrap()
    }

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 128, Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }

    fn sample_limits() -> TimestampedFloat32SampleLimits {
        TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(2))
            .unwrap()
    }

    fn identity(uid: &str, source: &str) -> StreamHandshakeIdentity {
        StreamHandshakeIdentity::new(
            uid.into(),
            "perf-002-host".into(),
            source.into(),
            "perf-002-session".into(),
            handshake_limits(),
        )
        .unwrap()
    }

    fn outlet(channels: usize, identity: StreamHandshakeIdentity) -> PersistentFloat32Outlet {
        PersistentFloat32Outlet::new(
            activation(),
            TcpListener::bind("127.0.0.1:0").unwrap(),
            identity,
            handshake_limits(),
            sample_limits(),
            channels,
            PersistentFloat32OutletLimits::new(8, 4).unwrap(),
        )
        .unwrap()
    }

    fn accept_one(outlet: &mut PersistentFloat32Outlet) -> PersistentFloat32ConsumerAccepted {
        for _ in 0..500 {
            if let Some(accepted) = outlet
                .poll_accept_consumer(&AtomicBool::new(false))
                .unwrap()
            {
                return accepted;
            }
            thread::sleep(Duration::from_millis(1));
        }
        panic!("pending consumer was not accepted");
    }

    fn spawn_reader(
        address: SocketAddr,
        identity: StreamHandshakeIdentity,
        channels: usize,
        records: usize,
    ) -> thread::JoinHandle<Vec<(f64, Vec<f32>)>> {
        thread::spawn(move || {
            let cancelled = AtomicBool::new(false);
            let mut stream =
                connect_handshake_stream(address, &identity, handshake_limits(), &cancelled)
                    .unwrap();
            read_initialization_for_channels(&mut stream, channels, sample_limits(), &cancelled)
                .unwrap();
            (0..records)
                .map(|_| {
                    let record = read_record_for_channels(
                        &mut stream,
                        channels,
                        sample_limits(),
                        &cancelled,
                    )
                    .unwrap();
                    (
                        record.raw_source_timestamp().value(),
                        record.sample().values().to_vec(),
                    )
                })
                .collect()
        })
    }

    #[test]
    fn perf_002_idle_poll_shape_rejection_and_cancellation_are_inert() {
        assert_eq!(
            PersistentFloat32OutletActivation::new(
                test_capability(RuntimeModule::TimestampedFloat32Sample),
                sample_activation(),
            ),
            Err(PersistentFloat32OutletActivationError::WrongModule)
        );
        assert_eq!(
            PersistentFloat32OutletLimits::new(0, 1),
            Err(PersistentFloat32OutletLimitError::ZeroRecordsPerChunk)
        );
        assert_eq!(
            PersistentFloat32OutletLimits::new(1, 0),
            Err(PersistentFloat32OutletLimitError::ZeroConsumers)
        );
        assert_eq!(
            PersistentFloat32OutletLimits::new(1, MAX_CONSUMERS + 1),
            Err(PersistentFloat32OutletLimitError::ConsumerLimitExceeded {
                actual: MAX_CONSUMERS + 1,
                limit: MAX_CONSUMERS,
            })
        );
        let zero_channel_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        assert_eq!(
            PersistentFloat32Outlet::new(
                activation(),
                zero_channel_listener,
                identity("70000000-0000-4000-8000-000000000006", "zero-channels"),
                handshake_limits(),
                sample_limits(),
                0,
                PersistentFloat32OutletLimits::new(1, 1).unwrap(),
            )
            .err(),
            Some(PersistentFloat32OutletCreateError::ZeroChannels)
        );
        let mut outlet = outlet(2, identity("70000000-0000-4000-8000-000000000001", "inert"));
        let before = outlet.buffer_identity();
        assert_eq!(
            outlet.poll_accept_consumer(&AtomicBool::new(false)),
            Ok(None)
        );
        assert_eq!(
            outlet.poll_accept_consumer(&AtomicBool::new(true)),
            Err(PersistentFloat32AcceptError::Cancelled)
        );
        assert_eq!(
            outlet.push_chunk(&[], &[], &AtomicBool::new(false)),
            Err(PersistentFloat32PushError::EmptyChunk)
        );
        assert_eq!(
            outlet.push_chunk(
                &[1.0],
                &[RawSourceTimestamp::new(1.0).unwrap()],
                &AtomicBool::new(false)
            ),
            Err(PersistentFloat32PushError::ValueCountMismatch {
                actual: 1,
                expected: 2,
            })
        );
        assert_eq!(
            outlet.push_chunk(
                &[1.0, 2.0],
                &[RawSourceTimestamp::new(1.0).unwrap()],
                &AtomicBool::new(true)
            ),
            Err(PersistentFloat32PushError::Cancelled)
        );
        let oversized_timestamps = [RawSourceTimestamp::new(1.0).unwrap(); 9];
        assert_eq!(
            outlet.push_chunk(&[0.0; 18], &oversized_timestamps, &AtomicBool::new(false)),
            Err(PersistentFloat32PushError::RecordLimitExceeded {
                actual: 9,
                limit: 8,
            })
        );
        assert_eq!(outlet.buffer_identity(), before);
        assert_eq!(outlet.connected_consumers(), 0);
    }

    #[test]
    fn perf_002_one_chunk_is_encoded_once_and_fanned_out_to_two_consumers() {
        let identity = identity("70000000-0000-4000-8000-000000000002", "two-consumers");
        let mut outlet = outlet(2, identity.clone());
        let address = outlet.local_address();
        let first = spawn_reader(address, identity.clone(), 2, 3);
        let second = spawn_reader(address, identity, 2, 3);
        assert_eq!(accept_one(&mut outlet).connected_consumers(), 1);
        assert_eq!(accept_one(&mut outlet).connected_consumers(), 2);
        let before = outlet.buffer_identity();
        let timestamps = [
            RawSourceTimestamp::new(10.0).unwrap(),
            RawSourceTimestamp::new(11.0).unwrap(),
            RawSourceTimestamp::new(12.0).unwrap(),
        ];
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let report = outlet
            .push_chunk(&values, &timestamps, &AtomicBool::new(false))
            .unwrap();
        assert_eq!(report.record_count(), 3);
        assert_eq!(report.consumers_before(), 2);
        assert_eq!(report.consumers_after(), 2);
        assert_eq!(report.complete_deliveries(), 2);
        assert_eq!(report.failed_consumers(), 0);
        assert_eq!(report.first_failure(), None);
        assert_eq!(outlet.buffer_identity(), before);
        let expected = vec![
            (10.0, vec![1.0, 2.0]),
            (11.0, vec![3.0, 4.0]),
            (12.0, vec![5.0, 6.0]),
        ];
        assert_eq!(first.join().unwrap(), expected);
        assert_eq!(second.join().unwrap(), expected);
        assert_eq!(outlet.close().closed_consumers(), 2);
        TcpListener::bind(address).unwrap();
    }

    pub(crate) fn exercise_repeated_push_buffer_reuse() {
        let identity = identity("70000000-0000-4000-8000-000000000005", "repeated-pushes");
        let mut outlet = outlet(1, identity.clone());
        let reader = spawn_reader(outlet.local_address(), identity, 1, 2);
        accept_one(&mut outlet);
        let before = outlet.buffer_identity();
        for (timestamp, value) in [(30.0, 10.0), (31.0, 11.0)] {
            let report = outlet
                .push_chunk(
                    &[value],
                    &[RawSourceTimestamp::new(timestamp).unwrap()],
                    &AtomicBool::new(false),
                )
                .unwrap();
            assert_eq!(report.complete_deliveries(), 1);
            assert_eq!(outlet.buffer_identity(), before);
        }
        assert_eq!(
            reader.join().unwrap(),
            vec![(30.0, vec![10.0]), (31.0, vec![11.0])]
        );
    }

    #[test]
    fn perf_002_two_outlets_retain_independent_shapes_and_consumers() {
        let first_identity = identity("70000000-0000-4000-8000-000000000003", "first-outlet");
        let second_identity = identity("70000000-0000-4000-8000-000000000004", "second-outlet");
        let mut first_outlet = outlet(1, first_identity.clone());
        let mut second_outlet = outlet(2, second_identity.clone());
        let first_reader = spawn_reader(first_outlet.local_address(), first_identity, 1, 1);
        let second_reader = spawn_reader(second_outlet.local_address(), second_identity, 2, 1);
        accept_one(&mut first_outlet);
        accept_one(&mut second_outlet);
        first_outlet
            .push_chunk(
                &[7.0],
                &[RawSourceTimestamp::new(20.0).unwrap()],
                &AtomicBool::new(false),
            )
            .unwrap();
        second_outlet
            .push_chunk(
                &[8.0, 9.0],
                &[RawSourceTimestamp::new(21.0).unwrap()],
                &AtomicBool::new(false),
            )
            .unwrap();
        assert_eq!(first_reader.join().unwrap(), vec![(20.0, vec![7.0])]);
        assert_eq!(second_reader.join().unwrap(), vec![(21.0, vec![8.0, 9.0])]);
    }
}
