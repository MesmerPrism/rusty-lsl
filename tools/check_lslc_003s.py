#!/usr/bin/env python3
import hashlib,json,re,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
load=lambda p:json.loads((ROOT/p).read_text(encoding="utf-8-sig"))
sha=lambda p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()
fixture=load("fixtures/compatibility/lslc-003s-string-sample-activation-descriptor.json")
lock=load("morphospace/feature.lock.json"); descriptor=load("morphospace/features/string-sample.json"); project=load("morphospace/project.spec.json"); state=load("morphospace/workspace.state.json")
assert fixture["schema"]=="rusty.lsl.lslc_003s.string_sample_activation.v1"
assert fixture["lock"]["revision"]==lock["revision"]==14 and fixture["lock"]["fingerprint"]==lock["lock_fingerprint"]
fingerprint=dict(lock); fingerprint["lock_fingerprint"]="0"*64
assert hashlib.sha256(json.dumps(fingerprint,separators=(",",":")).encode()).hexdigest()==lock["lock_fingerprint"]
feature=next(x for x in lock["features"] if x["feature_id"]=="string-sample")
assert feature["descriptor"]["path"]=="features/string-sample.json" and feature["descriptor"]["sha256"]==sha("morphospace/features/string-sample.json")
assert feature["dependencies"]==["stream-handshake"] and feature["selected"] and feature["run_activation_default"]=="disabled"
assert feature["effects"]["permissions"]==[] and lock["default_activation"]=="disabled" and lock["activation_rule"]=="selected-lock-and-runtime-input"
assert descriptor["source"]["revision"]=="6b6216830d591b39c99e045e040ee157686aaed0" and descriptor["source"]["sha256"]=="d2451c5fab59abef6a2b5a33ca6f5efae9a0672dfd913486c8731de8b3a7c23b"
shown=subprocess.run(["git","show",f'{descriptor["source"]["revision"]}:{descriptor["source"]["path"]}'],cwd=ROOT,capture_output=True,check=True).stdout
assert hashlib.sha256(shown).hexdigest()==descriptor["source"]["sha256"] and b"StringSample" in shown
assert "string-sample" in project["composition"]["selected_features"] and "string-sample" in project["composition"]["selected_modules"]
assert state["module_registry"]["lock_revision"]==14 and state["module_registry"]["lock_fingerprint"]==lock["lock_fingerprint"]
assert any(x["module_id"]=="string-sample" for x in state["module_registry"]["modules"])
source=(ROOT/"crates/rusty-lsl/src/runtime_activation.rs").read_text(encoding="utf-8")
for token in ["StringSample","rusty.lsl.string_sample.effective","lslc_003s_string_sample_is_distinct_dependency_closed_and_inert_when_absent"]: assert token in source
assert f'"{lock["lock_fingerprint"]}"' in source and "ACCEPTED_FEATURE_LOCK_REVISION: u64 = 14;" in source
assert fixture["damaged"]==["absent-string-selection","unknown-module","marker-mismatch","duplicate-string-module","stale-lock-fingerprint","missing-stream-handshake-dependency","unselected-module"]
assert not any(x in source for x in ["std::net","std::process","unsafe"])
for path,marker in {"AGENTS.md":"LSLC-003S","README.md":"LSLC-003S","docs/ARCHITECTURE.md":"LSLC-003S","docs/VALIDATION.md":"check_lslc_003s.ps1"}.items(): assert marker in (ROOT/path).read_text(encoding="utf-8")
print("LSLC-003S StringSample activation descriptor passed")
