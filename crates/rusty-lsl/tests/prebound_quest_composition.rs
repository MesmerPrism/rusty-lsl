use rusty_lsl::{
    admit_runtime_activation, run_prebound_short_info_responder, MetadataTreeLimits,
    ParsedShortInfoResponseEnvelope, ParsedStreamInfoObservedDocument, RuntimeActivationSelection,
    RuntimeModule, ShortInfoQuery, ShortInfoQueryWire, ShortInfoQueryWireLimits,
    ShortInfoResponderActivation, ShortInfoResponderLimits, ShortInfoResponderTermination,
    ShortInfoResponseEnvelopeLimits, StreamDescriptorLimits, StreamInfoObservedAdmissionLimits,
    StreamInfoObservedDocumentParseLimit, StreamInfoObservedFields, StreamInfoVolatileFieldLimits,
    ACCEPTED_FEATURE_LOCK_FINGERPRINT,
};
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::{Duration, Instant};

const SELF_PROBE_QUERY_ID: u64 = 70_000_010;

fn quest_xml(service_port: u16) -> String {
    format!(
        "<?xml version=\"1.0\"?>\n<info>\n\t<name>p70-quest-outlet</name>\n\t<type>qualification</type>\n\t<channel_count>1</channel_count>\n\t<channel_format>float32</channel_format>\n\t<source_id>p70-quest-source</source_id>\n\t<nominal_srate>0.000000000000000</nominal_srate>\n\t<version>1.100000000000000</version>\n\t<created_at>1.0</created_at>\n\t<uid>70000000-2222-4333-8444-555555555570</uid>\n\t<session_id>p70</session_id>\n\t<hostname>quest</hostname>\n\t<v4address>192.0.2.70</v4address>\n\t<v4data_port>{service_port}</v4data_port>\n\t<v4service_port>{service_port}</v4service_port>\n\t<v6address></v6address>\n\t<v6data_port>0</v6data_port>\n\t<v6service_port>0</v6service_port>\n\t<desc />\n</info>\n"
    )
}

#[test]
fn exact_quest_prebound_self_probe_completes_one_response_and_bounded_join() {
    let legacy = quest_xml(41_170).replace("\n\t", "\n");
    assert!(ParsedStreamInfoObservedDocument::parse(
        StreamInfoObservedDocumentParseLimit::new(legacy.len()).unwrap(),
        &legacy,
    )
    .is_err());

    let admission = admit_runtime_activation(
        ACCEPTED_FEATURE_LOCK_FINGERPRINT,
        "p70-rust-on-quest-lan-outlet",
        &[RuntimeActivationSelection::new(
            RuntimeModule::ShortInfoDiscoveryResponder.id(),
            RuntimeModule::ShortInfoDiscoveryResponder.effective_marker(),
        )],
    )
    .unwrap();
    let activation = ShortInfoResponderActivation::new(
        admission
            .capability(RuntimeModule::ShortInfoDiscoveryResponder)
            .unwrap(),
    )
    .unwrap();
    let responder_socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let responder_address = responder_socket.local_addr().unwrap();
    let xml = quest_xml(41_170);
    let query_limits = ShortInfoQueryWireLimits::new(256, 1024).unwrap();
    let response_limits = ShortInfoResponseEnvelopeLimits::new(xml.len(), xml.len() + 32).unwrap();
    let responder_xml = xml.clone();
    let responder = thread::spawn(move || {
        let document = ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(responder_xml.len()).unwrap(),
            &responder_xml,
        )
        .unwrap();
        run_prebound_short_info_responder(
            activation,
            responder_socket,
            ShortInfoResponderLimits::new(
                2048,
                1,
                Duration::from_millis(10),
                Duration::from_secs(2),
            )
            .unwrap(),
            query_limits,
            response_limits,
            &document,
            &AtomicBool::new(false),
        )
    });

    let probe = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let return_port = probe.local_addr().unwrap().port();
    let query = ShortInfoQuery::new(
        "name='p70-quest-outlet'".into(),
        return_port,
        SELF_PROBE_QUERY_ID,
        query_limits,
    )
    .unwrap();
    let wire = ShortInfoQueryWire::encode(&query, query_limits).unwrap();
    assert_eq!(
        probe.send_to(wire.as_bytes(), responder_address).unwrap(),
        wire.as_bytes().len()
    );

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut bytes = vec![0; response_limits.max_envelope_bytes() + 1];
    let (length, source) = loop {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .expect("bounded responder response");
        probe
            .set_read_timeout(Some(remaining.min(Duration::from_millis(20))))
            .unwrap();
        match probe.recv_from(&mut bytes) {
            Ok(value) => break value,
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(error) => panic!("unexpected receive failure: {error}"),
        }
    };
    assert_eq!(source, responder_address);
    let text = std::str::from_utf8(&bytes[..length]).unwrap();
    let parsed = ParsedShortInfoResponseEnvelope::parse(text, response_limits).unwrap();
    assert_eq!(parsed.query_id(), SELF_PROBE_QUERY_ID);
    assert_eq!(parsed.body().source(), xml);
    StreamInfoObservedFields::admit(
        StreamInfoObservedAdmissionLimits::new(
            StreamDescriptorLimits::new(64, 64, 64, 64).unwrap(),
            MetadataTreeLimits::new(1, 1, 1, 4, 1).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
        ),
        ParsedStreamInfoObservedDocument::parse(
            StreamInfoObservedDocumentParseLimit::new(parsed.body().source().len()).unwrap(),
            parsed.body().source(),
        )
        .unwrap(),
    )
    .unwrap();

    let run = responder.join().expect("responder thread").unwrap();
    assert_eq!(run.local_address(), responder_address);
    assert_eq!(run.requests(), 1);
    assert_eq!(
        run.termination(),
        ShortInfoResponderTermination::RequestLimit
    );
    assert!(UdpSocket::bind(responder_address).is_ok());
}
