#!/usr/bin/env python3
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-004r-official-outlet-response-datagram-observation.json"


def validate(data):
    assert data["schema"] == "rusty.lsl.official_outlet_response_datagram_observation.v1"
    assert data["source"] == {"rusty_lsl_commit":"4150cccac26e7400d226a701ce3461eb82b39f60","rusty_lsl_tree":"59adab9f13c6fcc6f2124ee07367bbe584b692a8"}
    assert data["official"] == {"pylsl":"1.18.2","liblsl":117,"protocol":110}
    assert data["scope"] == {"family":"ipv4","group":"239.255.172.215","port":16571,"interface_selection":"caller-explicit-active-private-ipv4","platform_class":"single-windows-desktop-host","serialized_repeats":2,"captures_per_repeat":1,"maximum_deadline_milliseconds":3000,"capture_point":"udp-datagram-before-production-parsing"}
    assert data["datagrams"] == [
        {"sha256":"152d0a894773bb87ef4b90052eac0ad286051fbdb9efb6eeb3bf2c6fb4b7305d","byte_length":732,"document_sha256":"a837f6119d45369e4aefafa68b4554f2767a937f80cb1d65e46a2d25715b6de7","document_byte_length":711},
        {"sha256":"218f2d486f6aafa08d642586ec3506755c25ef7021bfe7e5594f894c35bbbf95","byte_length":732,"document_sha256":"4153d94e143929083d4c778d152951be53604f83f6f1d3879ae07d0cc943a15e","document_byte_length":711},
    ]
    assert data["structure"]["envelope"] == {"query_id_syntax":"unsigned-decimal","query_id_digit_count":19,"value_published":False,"delimiter":"crlf","delimiter_offset":19}
    assert data["structure"]["document"] == {"encoding":"utf-8","xml_declaration":True,"root":"info","top_level_roles":["name","type","channel_count","channel_format","source_id","nominal_srate","version","created_at","uid","session_id","hostname","v4address","v4data_port","v4service_port","v6address","v6data_port","v6service_port","desc"],"private_source_id_matched":True,"values_published":False}
    assert data["repeat_relationship"] == {"complete_datagrams_equal":False,"hashes_equal":False,"lengths_equal":True,"document_hashes_equal":False,"structures_equal":True,"stable_claim":"length-and-minimum-structure-only","dynamic_fields_present":True,"claim_scope":"two-serialized-repeats-only"}
    assert data["results"] == {"capture_before_production_parsing":["pass","pass"],"finite_completion":["pass","pass"],"source_correlation_match":["pass","pass"],"socket_cleanup_rebind":["pass","pass"],"elapsed_milliseconds":[32.0,0.0]}
    assert data["private_artifacts"] == {"driver_source_sha256":"acf8bafaa0a2bf3dddb0720af4086c57eed24f623a727840fa9559b1a4bcaf0b","attempt_record_sha256":["045d827939531bf067f8c7babea7baae3b3450e937762912c37805495ddddec1","1c09e55a389e924e4f8ea2cf66e75f1f7dfde3ec580e241a8849d8055725692e"],"capsule_sha256":"f008500a7b794a84571f49ba647c6db4ec9138629f8df05bf1eae6b7cd550d6b","published":False}
    assert data["limitations"] == {"single_host":True,"single_platform":True,"two_repeat_structure_only":True,"dynamic_values_private":True,"production_parser_executed":False,"production_runtime_changed":False,"cross_host":False,"quest_or_device":False,"portable_interface_or_retry_policy":False}
    assert data["claims"] == {"observation_only":True,"addressable_exact_datagram_hashes":True,"raw_datagrams_published":False,"external_source_inspected":False,"runtime_or_activation_widened":False,"broad_compatibility":False,"manifold_authority":False}
    encoded = json.dumps(data, sort_keys=True)
    forbidden = ("192.168.", "Wi-Fi", "InterfaceIndex", "private-host", "device_serial", "serial_number", "rlsl-004r-private")
    assert not any(value in encoded for value in forbidden)


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
mutations = [
    (("datagrams", 0, "sha256"), "0" * 64),
    (("datagrams", 1, "byte_length"), 731),
    (("structure", "envelope", "delimiter"), "lf"),
    (("structure", "document", "root"), "stream"),
    (("official", "pylsl"), "1.18.1"),
    (("scope", "serialized_repeats"), 1),
    (("repeat_relationship", "stable_claim"), "complete-byte-stability"),
    (("private_artifacts", "capsule_sha256"), "f" * 64),
    (("limitations", "dynamic_values_private"), False),
    (("claims", "raw_datagrams_published"), True),
]
for route, value in mutations:
    damaged = copy.deepcopy(data)
    target = damaged
    for part in route[:-1]:
        target = target[part]
    target[route[-1]] = value
    try:
        validate(damaged)
    except (AssertionError, KeyError, TypeError):
        continue
    raise SystemExit(f"damaged fixture accepted: {route}")
for path, marker in {
    "AGENTS.md":"LSLC-004R", "docs/COMPATIBILITY.md":"LSLC-004R",
    "docs/PROVENANCE.md":"LSLC-004R", "docs/VALIDATION.md":"check_lslc_004r.ps1",
    "fixtures/compatibility/README.md":FIXTURE.name,
}.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8"), path
print("LSLC-004R official outlet response datagram observation passed (10 damaged fixtures rejected)")
