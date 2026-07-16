#!/usr/bin/env python3
import copy, json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
PATH=ROOT/"fixtures/compatibility/lslc-003u-string-utf8-boundary-observation.json"

def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_003u.v1"
    assert data["official"]=={"package":"pylsl","version":"1.18.2","library_version":117,"protocol_version":110,"implementation_source_used":False}
    assert data["bounds"]=={"channels":1,"caller_records":1,"initialization_records":2,"directions":2,"repeats_per_case":2,"min_payload_bytes":1,"max_payload_bytes":127,"network":"ipv4-loopback","finite_deadlines":True}
    expected={"marker":2,"timestamp":123456.789,"length_form":1,"length":2,"value":"10"}
    assert data["initialization"]==[expected,expected]
    cases={case["case_id"]:case for case in data["cases"]}
    assert set(cases)=={"multibyte-utf8","exact-127-utf8-bytes"}
    assert (cases["multibyte-utf8"]["utf8_bytes"],cases["multibyte-utf8"]["unicode_scalar_values"])==(9,3)
    assert (cases["exact-127-utf8-bytes"]["utf8_bytes"],cases["exact-127-utf8-bytes"]["unicode_scalar_values"])==(127,126)
    for case in cases.values():
        assert case["official_outlet_to_private_inlet"]["outcome"]=="pass"
        assert case["private_outlet_to_official_inlet"]["outcome"]=="pass"
        assert case["official_outlet_to_private_inlet"]["length"]==case["utf8_bytes"]
        assert case["official_outlet_to_private_inlet"]["length_form"]==1
        assert case["value_sha256"]==case["private_outlet_to_official_inlet"]["value_sha256"]
    assert data["excluded_private_evidence"]["pylsl_1_18_1_drift_runs"]==4
    assert data["excluded_private_evidence"]["acceptance_use"] is False
    assert all(not value for value in data["claims"].values())
    assert len(data["provenance"]["raw_attempt_sha256"])==4
    for value in [data["provenance"]["driver_sha256"],data["provenance"]["configuration_sha256"],data["provenance"]["official_binary_sha256"],*data["provenance"]["raw_attempt_sha256"]]: assert len(value)==64

data=json.loads(PATH.read_text(encoding="utf-8")); validate(data)
mutations=[]
for route,value in [(('bounds','max_payload_bytes'),128),(('bounds','repeats_per_case'),1),(('cases',0,'utf8_bytes'),8),(('cases',1,'official_outlet_to_private_inlet','length_form'),2),(('excluded_private_evidence','acceptance_use'),True),(('claims','implementation_change'),True)]:
    damaged=copy.deepcopy(data); target=damaged
    for part in route[:-1]: target=target[part]
    target[route[-1]]=value; mutations.append(damaged)
for damaged in mutations:
    try: validate(damaged)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit("damaged fixture accepted")
routes={"AGENTS.md":"LSLC-003U","README.md":"LSLC-003U","docs/COMPATIBILITY.md":"LSLC-003U","docs/PROVENANCE.md":"LSLC-003U","docs/VALIDATION.md":"check_lslc_003u.ps1","fixtures/compatibility/README.md":PATH.name}
for path,marker in routes.items():
    if marker not in (ROOT/path).read_text(encoding="utf-8"): raise SystemExit(f"missing route: {path}")
for path in [PATH,Path(__file__)]:
    if not path.read_bytes().endswith(b"\n"): raise SystemExit(f"missing terminal newline: {path}")
print("LSLC-003U String UTF-8 boundary observation passed")
