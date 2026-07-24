#!/usr/bin/env python3
import copy, json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
PATH=ROOT/"fixtures/compatibility/lslc-003w-empty-string-record-observation.json"
EMPTY_SHA="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_003w.v1"
    assert data["official"]=={"package":"pylsl","version":"1.18.2","library_version":117,"protocol_version":110,"implementation_source_used":False}
    assert data["bounds"]=={"channels":1,"caller_records":1,"initialization_records":2,"directions":2,"repeats":2,"payload_bytes":0,"network":"ipv4-loopback","finite_deadlines":True}
    expected={"marker":2,"timestamp":123456.789,"length_form":1,"length":2,"value":"10"}
    assert data["initialization"]==[expected,expected]
    record=data["caller_record"]
    assert (record["classification"],record["utf8_bytes"],record["unicode_scalar_values"])==("empty-string",0,0)
    assert record["value_sha256"]==EMPTY_SHA
    incoming=record["official_outlet_to_private_inlet"]
    assert incoming=={"outcome":"pass","marker":2,"timestamp":1234.5,"length_form":1,"length":0,"record_sha256":"0bc24c3517fbee79d0740f4dff7bb292b5e5a555978c28925a3752d490da50fe"}
    outgoing=record["private_outlet_to_official_inlet"]
    assert outgoing=={"outcome":"pass","timestamp":1234.5,"channels":1,"value_sha256":EMPTY_SHA}
    assert len(data["provenance"]["raw_attempt_sha256"])==2
    for value in [data["provenance"]["driver_sha256"],data["provenance"]["configuration_sha256"],data["provenance"]["official_binary_sha256"],*data["provenance"]["raw_attempt_sha256"]]: assert len(value)==64
    assert all(value is False for value in data["excluded_private_evidence"].values())
    assert all(value is False for value in data["claims"].values())

data=json.loads(PATH.read_text(encoding="utf-8")); validate(data)
mutations=[]
for route,value in [(('bounds','payload_bytes'),1),(('bounds','repeats'),1),(('caller_record','official_outlet_to_private_inlet','length_form'),2),(('caller_record','private_outlet_to_official_inlet','channels'),2),(('excluded_private_evidence','raw_records'),True),(('claims','runtime_change'),True)]:
    damaged=copy.deepcopy(data); target=damaged
    for part in route[:-1]: target=target[part]
    target[route[-1]]=value; mutations.append(damaged)
for damaged in mutations:
    try: validate(damaged)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit("damaged fixture accepted")
routes={"AGENTS.md":"LSLC-003W","README.md":"LSLC-003W","docs/COMPATIBILITY.md":"LSLC-003W","docs/PROVENANCE.md":"LSLC-003W","docs/VALIDATION.md":"check_lslc_003w.ps1","fixtures/compatibility/README.md":PATH.name}
for path,marker in routes.items():
    if marker not in (ROOT/path).read_text(encoding="utf-8"): raise SystemExit(f"missing route: {path}")
for path in [PATH,Path(__file__)]:
    if not path.read_bytes().endswith(b"\n"): raise SystemExit(f"missing terminal newline: {path}")
print("LSLC-003W empty String observation passed")
