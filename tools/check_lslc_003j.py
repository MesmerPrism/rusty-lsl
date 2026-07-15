#!/usr/bin/env python3
"""Validate live LSLC-003J gate roles and dependency closure."""

import hashlib
import json
from pathlib import Path
import re
import subprocess

from dispatch_current_gates import load_and_validate

ROOT = Path(__file__).resolve().parents[1]
FINGERPRINT = "b9a2d4bba914d679c8cef1af350721b394130d49935b46f9824ba22b71470b3c"
SOURCE_SHA = "d493411d4751a2758022deeb495318611b96bddf371d38f11bd5b3a94bd1d5eb"
V1_BASE = "7181672bfb1f6baceabd87c7c27c4f2f3922b06b"


def fail(message):
    raise SystemExit(message)


roles = load_and_validate(ROOT / "tools/current-gates-v2.json", ROOT)
if len(roles["historical"]) != 18 or [gate["checker_id"] for gate in roles["current"]] != ["lslc-003j"]:
    fail("v2 roles are not the exact 18 historical plus one live-current inventory")

protected = ["tools/current-gates.json"]
for gate in roles["historical"]:
    protected.extend((gate["launcher_path"], gate["companion_path"]))
changed = subprocess.run(["git", "diff", "--name-only", V1_BASE, "--", *protected], cwd=ROOT, check=True, text=True, capture_output=True).stdout.strip()
if changed:
    fail(f"v1 manifest or historical checker bytes changed: {changed}")

source = ROOT / "crates/rusty-lsl/src/timestamped_float32_sample_runtime.rs"
if hashlib.sha256(source.read_bytes()).hexdigest() != SOURCE_SHA:
    fail("Float32 runtime source does not match the preserved three-substitution hash")

descriptor_path = ROOT / "morphospace/features/timestamped-float32-sample.json"
descriptor = json.loads(descriptor_path.read_text(encoding="utf-8"))
if descriptor["source"] != {"repo_id":"rusty-lsl","revision":"24a1f7636353ea3ff96906adc287dd4c1fc1f2c7","path":"crates/rusty-lsl/src/timestamped_float32_sample_runtime.rs","sha256":SOURCE_SHA}:
    fail("timestamped Float32 descriptor source binding is not exact")

lock = json.loads((ROOT / "morphospace/feature.lock.json").read_text(encoding="utf-8"))
if lock.get("revision") != 13 or lock.get("lock_fingerprint") != FINGERPRINT or lock.get("resolver_version") != "rusty-morphospace-feature-resolver/2":
    fail("resolver-owned revision-13 lock binding is not exact")
feature = next((item for item in lock["features"] if item["feature_id"] == "timestamped-float32-sample"), None)
if feature is None or feature["descriptor"]["source_sha256"] != SOURCE_SHA or feature["descriptor"]["sha256"] != hashlib.sha256(descriptor_path.read_bytes()).hexdigest():
    fail("lock does not bind the exact descriptor and source hashes")

activation = (ROOT / "crates/rusty-lsl/src/runtime_activation.rs").read_text(encoding="utf-8")
if FINGERPRINT not in activation or "ACCEPTED_FEATURE_LOCK_REVISION: u64 = 13" not in activation:
    fail("runtime activation does not bind revision 13")
for relative in ("fixtures/compatibility/lslc-003c-lock-bound-runtime-activation-capability.json", "fixtures/compatibility/lslc-003f-dependency-closed-bounded-record-transport-correction.json"):
    fixture = json.loads((ROOT / relative).read_text(encoding="utf-8"))
    binding = fixture.get("lock", fixture.get("accepted_lock"))
    if binding != {"revision": 13, "fingerprint": FINGERPRINT}:
        fail(f"activation fixture is stale: {relative}")

workspace = json.loads((ROOT / "morphospace/workspace.state.json").read_text(encoding="utf-8"))
registry = workspace["module_registry"]
if registry["lock_revision"] != 13 or registry["lock_fingerprint"] != FINGERPRINT:
    fail("workspace module projection is stale")
if workspace["current_unit"] != "rlsl-lslc-003j-current-gate-role-closure-recovery":
    fail("LSLC-003J is not the active claimed unit")
if not any(item["blocker_id"] == "rlsl-lslc-003i-pinned-rust-180-const-bits-validation-blocked" for item in workspace["blockers"]):
    fail("LSLC-003I blocked history was not preserved")

audit = json.loads((ROOT / "fixtures/compatibility/lslc-003j-current-gate-role-audit.json").read_text(encoding="utf-8"))
if audit.get("historical_gate_count") != 18 or audit.get("current_gate") != "lslc-003j" or audit.get("workflow_owner_commit") != "708a3401b0433f1cd587d83ad4f12e13a707202d":
    fail("LSLC-003J audit fixture is incomplete")

print("LSLC-003J live gate-role and dependency-closure evidence passed")
