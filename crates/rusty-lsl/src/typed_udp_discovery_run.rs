// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit composition of one bounded UDP discovery call into typed observations.

use crate::{
    run_udp_discovery, ShortInfoQueryWire, ShortInfoResponseEnvelopeLimits,
    StreamInfoObservedAdmissionLimits, TypedUdpDiscoveryResponse, TypedUdpDiscoveryResponseError,
    UdpDiscoveryActivation, UdpDiscoveryConfig, UdpDiscoveryError, UdpDiscoveryTermination,
};
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;

/// Completed bounded discovery call with typed responses in receive order.
#[derive(Debug, PartialEq)]
pub struct TypedUdpDiscoveryRun {
    local_address: SocketAddr,
    termination: UdpDiscoveryTermination,
    responses: Vec<TypedUdpDiscoveryResponse>,
}

impl TypedUdpDiscoveryRun {
    /// Returns the actual local address observed by the existing UDP call.
    pub const fn local_address(&self) -> SocketAddr {
        self.local_address
    }

    /// Returns the unchanged bounded stop reason.
    pub const fn termination(&self) -> UdpDiscoveryTermination {
        self.termination
    }

    /// Returns typed responses in the existing receive order.
    pub fn responses(&self) -> &[TypedUdpDiscoveryResponse] {
        &self.responses
    }

    /// Recovers the typed response allocations.
    pub fn into_responses(self) -> Vec<TypedUdpDiscoveryResponse> {
        self.responses
    }
}

/// Stable failure from the existing UDP call or one indexed typed projection.
#[derive(Debug, PartialEq)]
pub enum TypedUdpDiscoveryRunError {
    /// The existing bounded UDP call failed unchanged.
    Udp(UdpDiscoveryError),
    /// The typed response collection allocation failed.
    ResponseAllocationFailed {
        /// Requested response capacity.
        requested: usize,
    },
    /// One admitted response failed typed projection.
    Response {
        /// Zero-based receive-order index.
        index: usize,
        /// Existing typed projection failure.
        error: TypedUdpDiscoveryResponseError,
    },
}

/// Runs one explicitly activated existing discovery call and types every admitted response.
pub fn run_typed_udp_discovery(
    activation: UdpDiscoveryActivation,
    config: UdpDiscoveryConfig,
    query: &ShortInfoQueryWire,
    cancelled: &AtomicBool,
    envelope_limits: ShortInfoResponseEnvelopeLimits,
    admission_limits: StreamInfoObservedAdmissionLimits,
) -> Result<TypedUdpDiscoveryRun, TypedUdpDiscoveryRunError> {
    let run = run_udp_discovery(activation, config, query, cancelled)
        .map_err(TypedUdpDiscoveryRunError::Udp)?;
    let local_address = run.local_address();
    let termination = run.termination();
    let responses = run.into_responses();
    let requested = responses.len();
    let mut typed = Vec::new();
    typed
        .try_reserve_exact(requested)
        .map_err(|_| TypedUdpDiscoveryRunError::ResponseAllocationFailed { requested })?;
    for (index, response) in responses.iter().enumerate() {
        typed.push(
            TypedUdpDiscoveryResponse::project(response, envelope_limits, admission_limits)
                .map_err(|error| TypedUdpDiscoveryRunError::Response { index, error })?,
        );
    }
    Ok(TypedUdpDiscoveryRun {
        local_address,
        termination,
        responses: typed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        MetadataTreeLimits, RuntimeModule, ShortInfoQuery, ShortInfoQueryWireLimits,
        StreamDescriptorLimits, StreamInfoObservedAdmissionError, StreamInfoVolatileFieldLimits,
        TypedShortInfoResponseObservationError, UdpDiscoveryLimits,
    };
    use std::net::UdpSocket;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::Duration;

    fn activation() -> UdpDiscoveryActivation {
        UdpDiscoveryActivation::new(test_capability(RuntimeModule::UdpDiscovery)).unwrap()
    }

    fn query() -> ShortInfoQueryWire {
        let limits = ShortInfoQueryWireLimits::new(8, 128).unwrap();
        let query = ShortInfoQuery::new("loopback".to_owned(), 1, 19, limits).unwrap();
        ShortInfoQueryWire::encode(&query, limits).unwrap()
    }

    fn document(channel_count: &str) -> String {
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
        let values = [
            "typed-run",
            "independent",
            channel_count,
            "float32",
            "fresh-source",
            "100.0000000000000",
            "110",
            "1",
            "fresh-uid",
            "fresh-session",
            "fresh-host",
            "203.0.113.10",
            "43001",
            "43002",
            "2001:db8::10",
            "43003",
            "43004",
        ];
        let mut body = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (name, value) in names.into_iter().zip(values) {
            body.push_str(&format!("\t<{name}>{value}</{name}>\n"));
        }
        body.push_str("\t<desc />\n</info>\n");
        body
    }

    fn admission_limits() -> StreamInfoObservedAdmissionLimits {
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(64, 64, 64, 4).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 8, 8).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
        )
    }

    fn run_with(channel_count: &str) -> Result<TypedUdpDiscoveryRun, TypedUdpDiscoveryRunError> {
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let destination = peer.local_addr().unwrap();
        let response = format!("19\r\n{}", document(channel_count)).into_bytes();
        let document_bytes = response.len() - 4;
        let worker = thread::spawn(move || {
            let mut query = [0_u8; 256];
            let (_, source) = peer.recv_from(&mut query).unwrap();
            peer.send_to(&response, source).unwrap();
        });
        let result = run_typed_udp_discovery(
            activation(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                destination,
                UdpDiscoveryLimits::new(
                    document_bytes + 32,
                    1,
                    Duration::from_millis(10),
                    Duration::from_secs(1),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            ),
            &query(),
            &AtomicBool::new(false),
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
        );
        worker.join().unwrap();
        result
    }

    #[test]
    fn lslc_004v_runs_and_preserves_typed_response_order_state() {
        let run = run_with("1").unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::ResponseLimit);
        assert!(run.local_address().ip().is_loopback());
        assert_eq!(run.responses().len(), 1);
        assert_eq!(run.responses()[0].observation().query_id(), 19);
        assert_eq!(
            run.responses()[0]
                .observation()
                .fields()
                .definition()
                .descriptor()
                .name(),
            "typed-run"
        );
        assert!(run.responses()[0].source().ip().is_loopback());
        assert_eq!(run.into_responses().len(), 1);
    }

    #[test]
    fn lslc_004v_reports_receive_order_index_for_typed_damage() {
        assert_eq!(
            run_with("01"),
            Err(TypedUdpDiscoveryRunError::Response {
                index: 0,
                error: TypedUdpDiscoveryResponseError::Typed(
                    TypedShortInfoResponseObservationError::Admission(
                        StreamInfoObservedAdmissionError::InvalidChannelCount,
                    ),
                ),
            })
        );
    }

    #[test]
    fn lslc_004v_preserves_pre_io_cancellation_without_resources() {
        let cancelled = AtomicBool::new(true);
        assert!(cancelled.load(Ordering::Acquire));
        let document_bytes = document("1").len();
        let run = run_typed_udp_discovery(
            activation(),
            UdpDiscoveryConfig::new(
                "127.0.0.1:0".parse().unwrap(),
                "127.0.0.1:9".parse().unwrap(),
                UdpDiscoveryLimits::new(
                    document_bytes + 32,
                    1,
                    Duration::from_millis(10),
                    Duration::from_secs(1),
                )
                .unwrap(),
                ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            ),
            &query(),
            &cancelled,
            ShortInfoResponseEnvelopeLimits::new(document_bytes, document_bytes + 32).unwrap(),
            admission_limits(),
        )
        .unwrap();
        assert_eq!(run.termination(), UdpDiscoveryTermination::Cancelled);
        assert!(run.responses().is_empty());
    }

    #[test]
    fn lslc_004w_suggests_only_the_first_exact_name_match() {
        let run = run_with("1").unwrap();
        assert_eq!(
            crate::suggest_typed_udp_discovery_response(&run, "typed-run"),
            Ok(Some(0))
        );
        assert_eq!(
            crate::suggest_typed_udp_discovery_response(&run, "other"),
            Ok(None)
        );
        assert_eq!(
            crate::suggest_typed_udp_discovery_response(&run, ""),
            Err(crate::TypedUdpDiscoverySelectionError::EmptyStreamName)
        );
    }
}
