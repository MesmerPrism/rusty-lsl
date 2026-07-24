#!/usr/bin/env python3
import copy, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-003x-empty-string-record-runtime.json"
SOURCE=ROOT/"crates/rusty-lsl/src/string_sample_runtime.rs"
def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_003x.v1"
    assert data["authority"]=={"observation":"rlsl-lslc-003w-empty-string-record-observation","runtime":"rlsl-lslc-003t-bounded-string-record-runtime","activation":"rlsl-lslc-003s-string-sample-activation-descriptor","required_module":"string-sample","required_dependency":"stream-handshake"}
    assert data["bounds"]=={"channels":1,"caller_records":1,"initialization_records":2,"min_payload_bytes":0,"max_payload_bytes":127,"network":"ipv4-loopback","finite_deadlines":True}
    assert data["framing"]=={"marker":2,"caller_timestamp":1234.5,"length_form":1,"empty_length":0,"length_unit":"utf8-bytes"}
    assert data["valid_cases"]==["empty-observed-value","prior-nonempty-values","distinct-string-capability","loopback-cleanup"]
    assert all(not value for value in data["effects"].values())
    assert data["claims"]["one_channel_one_record"] is True
    assert all(not value for key,value in data["claims"].items() if key!="one_channel_one_record")
data=json.loads(FIXTURE.read_text(encoding="utf-8"));validate(data)
for route,value in [(('bounds','channels'),2),(('bounds','max_payload_bytes'),128),(('framing','length_form'),2),(('effects','ambient_activation'),True),(('claims','multiple_channels_or_records'),True),(('authority','required_module'),'fixed-width-numeric-sample')]:
    damaged=copy.deepcopy(data);target=damaged
    for part in route[:-1]:target=target[part]
    target[route[-1]]=value
    try:validate(damaged)
    except (AssertionError,KeyError,TypeError):continue
    raise SystemExit("damaged fixture accepted")
source=SOURCE.read_text(encoding="utf-8")
assert "lslc_003x_empty_string_preserves_timestamp_and_cleanup" in source
assert "if value.is_empty()" not in source and "if length == 0" not in source
for marker in ["MAX_STRING_BYTES: usize = 127","RuntimeModule::StringSample","StreamHandshakeActivation"]: assert marker in source
routes={"AGENTS.md":"LSLC-003X","README.md":"LSLC-003X","docs/ARCHITECTURE.md":"LSLC-003X","docs/COMPATIBILITY.md":"LSLC-003X","docs/VALIDATION.md":"check_lslc_003x.ps1","fixtures/compatibility/README.md":FIXTURE.name}
for path,marker in routes.items():assert marker in (ROOT/path).read_text(encoding="utf-8"),path
for path in [FIXTURE,Path(__file__)]:assert path.read_bytes().endswith(b"\n")
print("LSLC-003X empty String runtime passed")
