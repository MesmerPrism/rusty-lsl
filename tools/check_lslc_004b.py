#!/usr/bin/env python3
import copy, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; FIXTURE=ROOT/"fixtures/compatibility/lslc-004b-exact-129-string-record-runtime.json"; SOURCE=ROOT/"crates/rusty-lsl/src/string_sample_runtime.rs"
def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_004b.v1"
    assert data["authority"]=={"observation":"rlsl-lslc-004a-exact-129-string-observation","runtime":"rlsl-lslc-003z-exact-128-string-record-runtime","activation":"rlsl-lslc-003s-string-sample-activation-descriptor","required_module":"string-sample","required_dependency":"stream-handshake"}
    assert data["bounds"]=={"channels":1,"caller_records":1,"initialization_records":2,"min_payload_bytes":0,"max_payload_bytes":129,"first_rejected_payload_bytes":130,"network":"ipv4-loopback","finite_deadlines":True}
    assert data["framing"]=={"marker":2,"caller_timestamp":1237.5,"length_form":1,"exact_boundary_length":129,"length_unit":"utf8-bytes"}
    assert data["valid_cases"]==["empty-value","prior-nonempty-values-through-128","exact-129-observed-value","distinct-string-capability","loopback-cleanup"]
    assert all(not value for value in data["effects"].values())
    assert data["claims"]["one_channel_one_record"] and data["claims"]["exact_129_boundary"]
    assert all(not value for key,value in data["claims"].items() if key not in {"one_channel_one_record","exact_129_boundary"})
data=json.loads(FIXTURE.read_text(encoding="utf-8")); validate(data)
for route,value in [(('bounds','channels'),2),(('bounds','max_payload_bytes'),130),(('bounds','first_rejected_payload_bytes'),131),(('framing','length_form'),2),(('effects','ambient_activation'),True),(('claims','multiple_channels_or_records'),True)]:
    damaged=copy.deepcopy(data); target=damaged
    for part in route[:-1]: target=target[part]
    target[route[-1]]=value
    try: validate(damaged)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit("damaged fixture accepted")
source=SOURCE.read_text(encoding="utf-8")
for marker in ["MAX_STRING_BYTES: usize = 129","lslc_004b_exact_129_bytes_preserve_timestamp_and_cleanup","StringSampleRecord::new(1237.5, \"q\".repeat(130))","RuntimeModule::StringSample","StreamHandshakeActivation","lslc_003v_observed_utf8_cases_conform_without_production_change","lslc_003x_empty_string_preserves_timestamp_and_cleanup"]: assert marker in source
routes={"AGENTS.md":"LSLC-004B","README.md":"LSLC-004B","docs/ARCHITECTURE.md":"LSLC-004B","docs/COMPATIBILITY.md":"LSLC-004B","docs/VALIDATION.md":"check_lslc_004b.ps1","fixtures/compatibility/README.md":FIXTURE.name}
for path,marker in routes.items(): assert marker in (ROOT/path).read_text(encoding="utf-8"),path
for path in [FIXTURE,Path(__file__)]: assert path.read_bytes().endswith(b"\n")
print("LSLC-004B exact 129-byte String runtime passed")
