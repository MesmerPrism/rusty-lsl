#!/usr/bin/env python3
import copy,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
PATH=ROOT/"fixtures/compatibility/lslc-003y-exact-128-string-observation.json"
VALUE_SHA="6836cf13bac400e9105071cd6af47084dfacad4e5e302c94bfed24e013afb73e"
def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_003y.v1"
    assert data["official"]=={"package":"pylsl","version":"1.18.2","library_version":117,"protocol_version":110,"implementation_source_used":False}
    assert data["bounds"]=={"channels":1,"caller_records":1,"initialization_records":2,"directions":2,"repeats":2,"payload_bytes":128,"network":"ipv4-loopback","finite_deadlines":True}
    record=data["caller_record"]
    assert (record["classification"],record["utf8_bytes"],record["unicode_scalar_values"])==("independently-authored-exact-128-byte-ascii-utf8",128,128)
    assert record["value_sha256"]==VALUE_SHA
    assert record["official_outlet_to_private_inlet"]=={"outcome":"pass","marker":2,"timestamp":1234.5,"length_form":1,"length":128,"record_sha256":"265c2cab161bfbbb8c224df69245cce0c5e4795018ca3c4fdee74c32ad2d5453"}
    assert record["private_outlet_to_official_inlet"]=={"outcome":"pass","timestamp":1234.5,"channels":1,"value_sha256":VALUE_SHA}
    assert len(data["provenance"]["raw_attempt_sha256"])==2
    for value in [data["provenance"]["driver_sha256"],data["provenance"]["configuration_sha256"],data["provenance"]["official_binary_sha256"],*data["provenance"]["raw_attempt_sha256"]]:assert len(value)==64
    assert data["excluded_private_evidence"]["pylsl_1_18_1_drift_runs"]==2 and data["excluded_private_evidence"]["acceptance_use"] is False
    assert all(value is False for key,value in data["excluded_private_evidence"].items() if key not in {"pylsl_1_18_1_drift_runs","acceptance_use"})
    assert all(value is False for value in data["claims"].values())
data=json.loads(PATH.read_text(encoding="utf-8"));validate(data)
for route,value in [(('bounds','payload_bytes'),127),(('bounds','repeats'),1),(('caller_record','official_outlet_to_private_inlet','length_form'),2),(('caller_record','private_outlet_to_official_inlet','channels'),2),(('excluded_private_evidence','acceptance_use'),True),(('claims','runtime_change'),True)]:
    damaged=copy.deepcopy(data);target=damaged
    for part in route[:-1]:target=target[part]
    target[route[-1]]=value
    try:validate(damaged)
    except (AssertionError,KeyError,TypeError):continue
    raise SystemExit("damaged fixture accepted")
routes={"AGENTS.md":"LSLC-003Y","README.md":"LSLC-003Y","docs/COMPATIBILITY.md":"LSLC-003Y","docs/PROVENANCE.md":"LSLC-003Y","docs/VALIDATION.md":"check_lslc_003y.ps1","fixtures/compatibility/README.md":PATH.name}
for path,marker in routes.items():
    if marker not in (ROOT/path).read_text(encoding="utf-8"):raise SystemExit(f"missing route: {path}")
for path in [PATH,Path(__file__)]:
    if not path.read_bytes().endswith(b"\n"):raise SystemExit(f"missing terminal newline: {path}")
print("LSLC-003Y exact 128-byte String observation passed")
