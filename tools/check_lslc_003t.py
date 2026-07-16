#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
fixture = json.loads((ROOT / "fixtures/compatibility/lslc-003t-bounded-string-record-runtime.json").read_text(encoding="utf-8"))
source = (ROOT / "crates/rusty-lsl/src/string_sample_runtime.rs").read_text(encoding="utf-8")
lib = (ROOT / "crates/rusty-lsl/src/lib.rs").read_text(encoding="utf-8")
unit = json.loads((ROOT / "morphospace/iteration-units/rlsl-lslc-003t-bounded-string-record-runtime.json").read_text(encoding="utf-8"))
policy = json.loads((ROOT / "tools/validation-policy.json").read_text(encoding="utf-8"))

assert fixture["authority"]["required_module"] == "string-sample"
assert fixture["bounds"] == {"channels":1,"caller_records":1,"initialization_records":2,"min_payload_bytes":1,"max_payload_bytes":127,"network":"ipv4-loopback","finite_deadlines":True}
assert set(fixture["damaged_cases"]) == {"wrong-module-capability","empty","oversized","nonfinite-timestamp","wrong-marker","wrong-initialization","wrong-length-form","truncated","invalid-utf8","cancelled","deadline","io","handshake"}
for token in ["RuntimeModule::StringSample", "MAX_STRING_BYTES: usize = 127", "INITIALIZATION_TIMESTAMP: f64 = 123_456.789", "header[9] != 1", "b\"10\"", "lslc_003t_damage_cancellation_and_deadline_are_typed"]:
    assert token in source, token
for forbidden in ["unsafe {", "extern \"C\"", "std::process", "Command::new"]:
    assert forbidden not in source, forbidden
assert "mod string_sample_runtime;" in lib and "run_string_sample_inlet" in lib
assert unit["prerequisites"][:2] == ["rlsl-lslc-003q-bounded-string-record-observation", "rlsl-lslc-003s-string-sample-activation-descriptor"]
gates = {gate["id"] for gate in policy["gates"]}
assert "lslc-003t-runtime" in gates
for path in ["AGENTS.md", "README.md", "docs/ARCHITECTURE.md", "docs/COMPATIBILITY.md", "docs/PROVENANCE.md", "docs/VALIDATION.md"]:
    assert "LSLC-003T" in (ROOT / path).read_text(encoding="utf-8"), path
print("LSLC-003T bounded String runtime passed")
