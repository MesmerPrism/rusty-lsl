// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit bounded UDP clock acquisition composed with accepted correction contracts.

use crate::{
    ClockFilterSelection, ClockFilterSelectionError, ClockFilterSelectionLimit, ClockOffset,
    ClockOffsetApplication, ClockOffsetApplicationError, ClockOffsetError, RawClockExchange,
    RawClockExchangeFormulaError, RawClockExchangeFormulaResult, RawClockExchangeInputError,
    RawSourceTimestamp, RuntimeModule, RuntimeModuleCapability, TimestampedFloat32SampleActivation,
};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Feature selected for integrated clock correction.
pub const INTEGRATED_CLOCK_CORRECTION_FEATURE_ID: &str = "integrated-clock-correction";
/// Exact effective marker required at runtime.
pub const INTEGRATED_CLOCK_CORRECTION_EFFECTIVE_MARKER: &str =
    "rusty.lsl.integrated_clock_correction.effective";

/// Proof of selected clock feature and exact runtime marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegratedClockCorrectionActivation {
    _sample: TimestampedFloat32SampleActivation,
}
impl IntegratedClockCorrectionActivation {
    /// Admits only the selected feature and marker.
    pub fn new(
        capability: RuntimeModuleCapability,
        sample: TimestampedFloat32SampleActivation,
    ) -> Result<Self, IntegratedClockCorrectionActivationError> {
        if !capability.matches(RuntimeModule::IntegratedClockCorrection) {
            return Err(IntegratedClockCorrectionActivationError::WrongModule);
        }
        Ok(Self { _sample: sample })
    }
}

/// Rejected integrated-clock activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegratedClockCorrectionActivationError {
    /// The admitted capability named a different module.
    WrongModule,
}

/// Explicit caller-owned finite clock provider.
pub trait ClockSource {
    /// Reads one timestamp in the caller's selected clock domain.
    fn now(&mut self) -> f64;
}

/// Finite configuration for one exchange batch and timestamp mapping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegratedClockCorrectionConfig {
    bind_address: SocketAddr,
    peer: SocketAddr,
    first_query_id: u64,
    exchange_count: usize,
    max_datagram_bytes: usize,
    receive_slice: Duration,
    total_deadline: Duration,
}
impl IntegratedClockCorrectionConfig {
    /// Creates explicit configuration, rejecting zero bounds in argument order.
    pub fn new(
        bind_address: SocketAddr,
        peer: SocketAddr,
        first_query_id: u64,
        exchange_count: usize,
        max_datagram_bytes: usize,
        receive_slice: Duration,
        total_deadline: Duration,
    ) -> Result<Self, IntegratedClockCorrectionConfigError> {
        if first_query_id == 0 {
            return Err(IntegratedClockCorrectionConfigError::ZeroFirstQueryId);
        }
        if exchange_count == 0 {
            return Err(IntegratedClockCorrectionConfigError::ZeroExchangeCount);
        }
        if max_datagram_bytes == 0 {
            return Err(IntegratedClockCorrectionConfigError::ZeroDatagramBytes);
        }
        if receive_slice.is_zero() {
            return Err(IntegratedClockCorrectionConfigError::ZeroReceiveSlice);
        }
        if total_deadline.is_zero() {
            return Err(IntegratedClockCorrectionConfigError::ZeroTotalDeadline);
        }
        first_query_id
            .checked_add((exchange_count - 1) as u64)
            .ok_or(IntegratedClockCorrectionConfigError::QueryIdOverflow)?;
        Ok(Self {
            bind_address,
            peer,
            first_query_id,
            exchange_count,
            max_datagram_bytes,
            receive_slice,
            total_deadline,
        })
    }
    /// Caller-selected exchange count.
    pub const fn exchange_count(self) -> usize {
        self.exchange_count
    }
}

/// Invalid integrated-clock configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegratedClockCorrectionConfigError {
    /// First query identifier was zero.
    ZeroFirstQueryId,
    /// Exchange count was zero.
    ZeroExchangeCount,
    /// Datagram maximum was zero.
    ZeroDatagramBytes,
    /// Receive slice was zero.
    ZeroReceiveSlice,
    /// Total deadline was zero.
    ZeroTotalDeadline,
    /// Final query identifier overflowed.
    QueryIdOverflow,
}

/// Completed acquisition, selection, offset, and explicit timestamp mapping.
#[derive(Debug, PartialEq)]
pub struct IntegratedClockCorrection {
    local_address: SocketAddr,
    selection: ClockFilterSelection,
    offset: ClockOffset,
    application: ClockOffsetApplication,
}
impl IntegratedClockCorrection {
    /// Actual scope-owned UDP bind address used during the call.
    pub const fn local_address(&self) -> SocketAddr {
        self.local_address
    }
    /// Accepted batch and first minimum-RTT selection.
    pub const fn selection(&self) -> &ClockFilterSelection {
        &self.selection
    }
    /// Finite selected offset.
    pub const fn offset(&self) -> ClockOffset {
        self.offset
    }
    /// Unchanged raw timestamp beside its `ClockCorrected` mapping.
    pub const fn application(&self) -> ClockOffsetApplication {
        self.application
    }
}

/// Stable integrated-clock failure.
#[derive(Debug, PartialEq)]
pub enum IntegratedClockCorrectionError {
    /// Caller cancellation was observed.
    Cancelled,
    /// Total batch deadline elapsed.
    Deadline,
    /// Socket operation failed.
    Io(ErrorKind),
    /// Datagram allocation failed.
    AllocationFailed {
        /// Requested capacity.
        requested: usize,
    },
    /// Datagram exceeded the selected maximum.
    DatagramLimitExceeded {
        /// Selected maximum.
        limit: usize,
    },
    /// Response source differed from the selected peer.
    PeerMismatch,
    /// Request or response was not valid UTF-8/framing/numeric text.
    InvalidDatagram,
    /// Response identifier did not match the outstanding exchange.
    QueryIdMismatch,
    /// Caller clock returned a non-finite value.
    NonFiniteClock {
        /// Unchanged rejected bits.
        bits: u64,
    },
    /// Raw four-timestamp admission failed.
    Exchange(RawClockExchangeInputError),
    /// Exchange formula evaluation failed.
    Formula(RawClockExchangeFormulaError),
    /// Bounded minimum-RTT selection failed.
    Selection(ClockFilterSelectionError),
    /// Selected offset was rejected.
    Offset(ClockOffsetError),
    /// Raw timestamp plus selected offset was non-finite.
    Application(ClockOffsetApplicationError),
}

fn clock_now<C: ClockSource>(clock: &mut C) -> Result<f64, IntegratedClockCorrectionError> {
    let value = clock.now();
    if value.is_finite() {
        Ok(value)
    } else {
        Err(IntegratedClockCorrectionError::NonFiniteClock {
            bits: value.to_bits(),
        })
    }
}

fn parse_response(
    text: &str,
    expected_id: u64,
    expected_t0: f64,
    t3: f64,
) -> Result<RawClockExchangeFormulaResult, IntegratedClockCorrectionError> {
    let fields: Vec<&str> = text.split_ascii_whitespace().collect();
    if fields.len() != 4 {
        return Err(IntegratedClockCorrectionError::InvalidDatagram);
    }
    let id = fields[0]
        .parse::<u64>()
        .map_err(|_| IntegratedClockCorrectionError::InvalidDatagram)?;
    if id != expected_id {
        return Err(IntegratedClockCorrectionError::QueryIdMismatch);
    }
    let echoed_t0 = fields[1]
        .parse::<f64>()
        .map_err(|_| IntegratedClockCorrectionError::InvalidDatagram)?;
    if echoed_t0.to_bits() != expected_t0.to_bits() {
        return Err(IntegratedClockCorrectionError::InvalidDatagram);
    }
    let t1 = fields[2]
        .parse::<f64>()
        .map_err(|_| IntegratedClockCorrectionError::InvalidDatagram)?;
    let t2 = fields[3]
        .parse::<f64>()
        .map_err(|_| IntegratedClockCorrectionError::InvalidDatagram)?;
    RawClockExchange::new(expected_t0, t1, t2, t3)
        .map_err(IntegratedClockCorrectionError::Exchange)?
        .evaluate()
        .map_err(IntegratedClockCorrectionError::Formula)
}

/// Acquires one bounded batch, selects minimum RTT, and maps one raw timestamp explicitly.
pub fn run_integrated_clock_correction<C: ClockSource>(
    _activation: IntegratedClockCorrectionActivation,
    config: IntegratedClockCorrectionConfig,
    clock: &mut C,
    raw_timestamp: RawSourceTimestamp,
    cancelled: &AtomicBool,
) -> Result<IntegratedClockCorrection, IntegratedClockCorrectionError> {
    if cancelled.load(Ordering::Acquire) {
        return Err(IntegratedClockCorrectionError::Cancelled);
    }
    let socket = UdpSocket::bind(config.bind_address)
        .map_err(|e| IntegratedClockCorrectionError::Io(e.kind()))?;
    let local_address = socket
        .local_addr()
        .map_err(|e| IntegratedClockCorrectionError::Io(e.kind()))?;
    let probe = config.max_datagram_bytes.checked_add(1).ok_or(
        IntegratedClockCorrectionError::AllocationFailed {
            requested: config.max_datagram_bytes,
        },
    )?;
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(probe)
        .map_err(|_| IntegratedClockCorrectionError::AllocationFailed { requested: probe })?;
    buffer.resize(probe, 0);
    let mut results = Vec::new();
    results
        .try_reserve_exact(config.exchange_count)
        .map_err(|_| IntegratedClockCorrectionError::AllocationFailed {
            requested: config.exchange_count,
        })?;
    let started = Instant::now();
    for index in 0..config.exchange_count {
        if cancelled.load(Ordering::Acquire) {
            return Err(IntegratedClockCorrectionError::Cancelled);
        }
        let id = config.first_query_id + index as u64;
        let t0 = clock_now(clock)?;
        let request = format!("LSL:timedata\r\n{id} {t0}\r\n");
        if request.len() > config.max_datagram_bytes {
            return Err(IntegratedClockCorrectionError::DatagramLimitExceeded {
                limit: config.max_datagram_bytes,
            });
        }
        socket
            .send_to(request.as_bytes(), config.peer)
            .map_err(|e| IntegratedClockCorrectionError::Io(e.kind()))?;
        loop {
            if cancelled.load(Ordering::Acquire) {
                return Err(IntegratedClockCorrectionError::Cancelled);
            }
            let remaining = config
                .total_deadline
                .checked_sub(started.elapsed())
                .ok_or(IntegratedClockCorrectionError::Deadline)?;
            socket
                .set_read_timeout(Some(remaining.min(config.receive_slice)))
                .map_err(|e| IntegratedClockCorrectionError::Io(e.kind()))?;
            match socket.recv_from(&mut buffer) {
                Ok((length, source)) => {
                    if source != config.peer {
                        return Err(IntegratedClockCorrectionError::PeerMismatch);
                    }
                    if length > config.max_datagram_bytes {
                        return Err(IntegratedClockCorrectionError::DatagramLimitExceeded {
                            limit: config.max_datagram_bytes,
                        });
                    }
                    let t3 = clock_now(clock)?;
                    let text = std::str::from_utf8(&buffer[..length])
                        .map_err(|_| IntegratedClockCorrectionError::InvalidDatagram)?;
                    results.push(parse_response(text, id, t0, t3)?);
                    break;
                }
                Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                    continue
                }
                Err(e) => return Err(IntegratedClockCorrectionError::Io(e.kind())),
            }
        }
    }
    let selection = ClockFilterSelection::select(
        results,
        ClockFilterSelectionLimit::new(config.exchange_count).expect("nonzero config count"),
    )
    .map_err(IntegratedClockCorrectionError::Selection)?;
    let offset = ClockOffset::new(selection.selected().clock_offset())
        .map_err(IntegratedClockCorrectionError::Offset)?;
    let application = ClockOffsetApplication::apply(raw_timestamp, offset)
        .map_err(IntegratedClockCorrectionError::Application)?;
    Ok(IntegratedClockCorrection {
        local_address,
        selection,
        offset,
        application,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{DerivedTimestampKind, StreamHandshakeActivation};
    use std::thread;

    struct SequenceClock {
        values: Vec<f64>,
        index: usize,
    }
    impl ClockSource for SequenceClock {
        fn now(&mut self) -> f64 {
            let value = self.values[self.index];
            self.index += 1;
            value
        }
    }
    fn activation() -> IntegratedClockCorrectionActivation {
        let handshake =
            StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                .unwrap();
        let sample = TimestampedFloat32SampleActivation::new(
            test_capability(RuntimeModule::TimestampedFloat32Sample),
            handshake,
        )
        .unwrap();
        IntegratedClockCorrectionActivation::new(
            test_capability(RuntimeModule::IntegratedClockCorrection),
            sample,
        )
        .unwrap()
    }
    fn config(peer: SocketAddr, count: usize) -> IntegratedClockCorrectionConfig {
        IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            peer,
            40,
            count,
            256,
            Duration::from_millis(5),
            Duration::from_secs(1),
        )
        .unwrap()
    }

    #[test]
    fn lslc_002u_acquires_selects_and_maps_raw_timestamp() {
        let responder = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer = responder.local_addr().unwrap();
        let worker = thread::spawn(move || {
            for (expected, remote) in [(40u64, (5.0, 5.1)), (41, (11.0, 11.1)), (42, (21.0, 21.1))]
            {
                let mut bytes = [0u8; 256];
                let (length, source) = responder.recv_from(&mut bytes).unwrap();
                let text = std::str::from_utf8(&bytes[..length]).unwrap();
                let line = text.split("\r\n").nth(1).unwrap();
                let mut fields = line.split(' ');
                let id: u64 = fields.next().unwrap().parse().unwrap();
                let t0 = fields.next().unwrap();
                assert_eq!(id, expected);
                responder
                    .send_to(
                        format!(" {id} {t0} {} {}", remote.0, remote.1).as_bytes(),
                        source,
                    )
                    .unwrap();
            }
        });
        let mut clock = SequenceClock {
            values: vec![0.0, 10.0, 10.0, 12.0, 20.0, 24.0],
            index: 0,
        };
        let result = run_integrated_clock_correction(
            activation(),
            config(peer, 3),
            &mut clock,
            RawSourceTimestamp::new(100.0).unwrap(),
            &AtomicBool::new(false),
        )
        .unwrap();
        worker.join().unwrap();
        assert_eq!(result.selection().selected_index(), 1);
        assert_eq!(result.offset().value(), 0.04999999999999982);
        assert_eq!(result.application().raw().value(), 100.0);
        assert_eq!(
            result.application().derived().kind(),
            DerivedTimestampKind::ClockCorrected
        );
        assert_eq!(result.application().derived().value(), 100.05);
        UdpSocket::bind(result.local_address()).unwrap();
    }

    #[test]
    fn lslc_002u_config_activation_cancellation_and_timeout_fail_closed() {
        assert_eq!(
            IntegratedClockCorrectionActivation::new(
                test_capability(RuntimeModule::UdpDiscovery),
                TimestampedFloat32SampleActivation::new(
                    test_capability(RuntimeModule::TimestampedFloat32Sample),
                    StreamHandshakeActivation::new(test_capability(RuntimeModule::StreamHandshake))
                        .unwrap(),
                )
                .unwrap(),
            ),
            Err(IntegratedClockCorrectionActivationError::WrongModule)
        );
        let one = Duration::from_millis(1);
        assert_eq!(
            IntegratedClockCorrectionConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                "127.0.0.1:9".parse().unwrap(),
                1,
                0,
                1,
                one,
                one
            ),
            Err(IntegratedClockCorrectionConfigError::ZeroExchangeCount)
        );
        let mut clock = SequenceClock {
            values: vec![0.0],
            index: 0,
        };
        let cancelled = AtomicBool::new(true);
        assert_eq!(
            run_integrated_clock_correction(
                activation(),
                config("127.0.0.1:9".parse().unwrap(), 1),
                &mut clock,
                RawSourceTimestamp::new(0.0).unwrap(),
                &cancelled
            ),
            Err(IntegratedClockCorrectionError::Cancelled)
        );
        let sink = UdpSocket::bind("127.0.0.1:0").unwrap();
        let peer = sink.local_addr().unwrap();
        let short = IntegratedClockCorrectionConfig::new(
            "127.0.0.1:0".parse().unwrap(),
            peer,
            1,
            1,
            256,
            Duration::from_millis(2),
            Duration::from_millis(10),
        )
        .unwrap();
        let mut clock = SequenceClock {
            values: vec![0.0],
            index: 0,
        };
        assert_eq!(
            run_integrated_clock_correction(
                activation(),
                short,
                &mut clock,
                RawSourceTimestamp::new(0.0).unwrap(),
                &AtomicBool::new(false)
            ),
            Err(IntegratedClockCorrectionError::Deadline)
        );
    }

    #[test]
    fn lslc_002u_malformed_mismatch_and_nonfinite_clock_are_typed() {
        for (payload, expected) in [
            (
                b"bad".as_slice(),
                IntegratedClockCorrectionError::InvalidDatagram,
            ),
            (
                b" 99 0 1 2".as_slice(),
                IntegratedClockCorrectionError::QueryIdMismatch,
            ),
        ] {
            let responder = UdpSocket::bind("127.0.0.1:0").unwrap();
            let peer = responder.local_addr().unwrap();
            let body = payload.to_vec();
            let worker = thread::spawn(move || {
                let mut bytes = [0u8; 256];
                let (_, source) = responder.recv_from(&mut bytes).unwrap();
                responder.send_to(&body, source).unwrap();
            });
            let mut clock = SequenceClock {
                values: vec![0.0, 3.0],
                index: 0,
            };
            assert_eq!(
                run_integrated_clock_correction(
                    activation(),
                    config(peer, 1),
                    &mut clock,
                    RawSourceTimestamp::new(0.0).unwrap(),
                    &AtomicBool::new(false)
                ),
                Err(expected)
            );
            worker.join().unwrap();
        }
        let mut clock = SequenceClock {
            values: vec![f64::NAN],
            index: 0,
        };
        let error = run_integrated_clock_correction(
            activation(),
            config("127.0.0.1:9".parse().unwrap(), 1),
            &mut clock,
            RawSourceTimestamp::new(0.0).unwrap(),
            &AtomicBool::new(false),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            IntegratedClockCorrectionError::NonFiniteClock { .. }
        ));
    }
}
