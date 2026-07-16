#!/usr/bin/env python3
import copy, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-003v-string-utf8-boundary-runtime-conformance.json"
SOURCE=ROOT/"crates/rusty-lsl/src/string_sample_runtime.rs"
def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_003v.v1"
    assert data["production_baseline_revision"]=="f1e68657253b572c6f1bfae58747b11637ec07ee"
    assert [(c["case_id"],c["utf8_bytes"],c["synthetic_loopback"]) for c in data["cases"]]==[("multibyte-utf8",9,"pass"),("exact-127-utf8-bytes",127,"pass")]
    assert data["invariants"]=={"channels":1,"caller_records":1,"length_form":1,"required_module":"string-sample","required_dependency":"stream-handshake","finite_deadlines":True,"cleanup":True,"production_prefix_unchanged":True}
    assert all(not value for value in data["claims"].values())
data=json.loads(FIXTURE.read_text(encoding="utf-8"));validate(data)
for route,value in [(('cases',0,'utf8_bytes'),8),(('cases',1,'synthetic_loopback'),'fail'),(('invariants','production_prefix_unchanged'),False),(('claims','production_change'),True)]:
    damaged=copy.deepcopy(data);target=damaged
    for part in route[:-1]:target=target[part]
    target[route[-1]]=value
    try:validate(damaged)
    except (AssertionError,KeyError,TypeError):continue
    raise SystemExit("damaged fixture accepted")
current=SOURCE.read_bytes().split(b"#[cfg(test)]",1)[0]
baseline=subprocess.run(["git","show",f"{data['production_baseline_revision']}:crates/rusty-lsl/src/string_sample_runtime.rs"],cwd=ROOT,check=True,capture_output=True).stdout.split(b"#[cfg(test)]",1)[0]
assert current==baseline,"production prefix changed"
source=SOURCE.read_text(encoding="utf-8")
assert "lslc_003v_observed_utf8_cases_conform_without_production_change" in source
routes={"AGENTS.md":"LSLC-003V","README.md":"LSLC-003V","docs/ARCHITECTURE.md":"LSLC-003V","docs/COMPATIBILITY.md":"LSLC-003V","docs/VALIDATION.md":"check_lslc_003v.ps1","fixtures/compatibility/README.md":FIXTURE.name}
for path,marker in routes.items():assert marker in (ROOT/path).read_text(encoding="utf-8"),path
print("LSLC-003V String boundary runtime conformance passed")
