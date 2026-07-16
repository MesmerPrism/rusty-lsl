#!/usr/bin/env python3
import copy, json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
PATH=ROOT/"fixtures/compatibility/lslc-003q-bounded-string-record-observation.json"

def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_003q.v1"
    assert data["official"]=={"package":"pylsl","version":"1.18.2","library_version":117,"protocol_version":110,"implementation_source_used":False}
    assert data["bounds"]=={"channels":1,"caller_records":1,"initialization_records":2,"directions":2,"max_payload_bytes":127,"network":"ipv4-loopback","finite_deadlines":True}
    assert data["repeats"]==2 and data["value"]=={"classification":"independently-authored-ascii-utf8","utf8_bytes":13,"text":"Rusty-mu-snow"}
    expected={"marker":2,"timestamp":123456.789,"length_form":1,"length":2,"value":"10"}
    assert data["official_outlet_to_private_inlet"]["initialization"]==[expected,expected]
    assert data["official_outlet_to_private_inlet"]["caller_record"]=={"marker":2,"timestamp":1234.5,"length_form":1,"length":13,"value":"Rusty-mu-snow"}
    assert data["private_outlet_to_official_inlet"]=={"outcome":"pass","pulled":{"timestamp":1234.5,"values":["Rusty-mu-snow"]}}
    assert all(not value for value in data["claims"].values())
    assert len(data["provenance"]["raw_attempt_sha256"])==2
    assert all(len(value)==64 for key,value in data["provenance"].items() if key!="raw_attempt_sha256")
    assert all(len(value)==64 for value in data["provenance"]["raw_attempt_sha256"])

data=json.loads(PATH.read_text(encoding="utf-8")); validate(data)
mutations=[]
for route,value in [(("bounds","channels"),2),(("repeats",),1),(("value","utf8_bytes"),12),(("official_outlet_to_private_inlet","caller_record","length_form"),2),(("official_outlet_to_private_inlet","caller_record","value"),"changed"),(("claims","production_implementation"),True)]:
    damaged=copy.deepcopy(data); target=damaged
    for part in route[:-1]: target=target[part]
    target[route[-1]]=value; mutations.append(damaged)
for damaged in mutations:
    try: validate(damaged)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit("damaged fixture accepted")
routes={"AGENTS.md":"LSLC-003Q","README.md":"LSLC-003Q","docs/COMPATIBILITY.md":"LSLC-003Q","docs/PROVENANCE.md":"LSLC-003Q","docs/VALIDATION.md":"check_lslc_003q.ps1","fixtures/compatibility/README.md":PATH.name}
for path,marker in routes.items():
    if marker not in (ROOT/path).read_text(encoding="utf-8"): raise SystemExit(f"missing route: {path}")
for path in [PATH,Path(__file__)]:
    if not path.read_bytes().endswith(b"\n"): raise SystemExit(f"missing terminal newline: {path}")
print("LSLC-003Q bounded String observation passed")
