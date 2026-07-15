#!/usr/bin/env python3
"""Validate durable live gate roles and current dependency relationships."""

import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys

from dispatch_current_gates import load_and_validate

ROOT = Path(__file__).resolve().parents[1]
SHA256 = re.compile(r"^[0-9a-f]{64}$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")


def fail(message):
    raise SystemExit(message)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def safe_relative(value):
    if not isinstance(value, str):
        fail("relationship path is not a string")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or str(path) != value:
        fail(f"relationship path escapes the repository: {value}")
    return path


def git(*arguments, text=False):
    result = subprocess.run(["git", *arguments], cwd=ROOT, check=False, capture_output=True, text=text)
    if result.returncode != 0:
        detail = result.stderr.strip() if text else result.stderr.decode(errors="replace").strip()
        fail(f"git {' '.join(arguments)} failed: {detail}")
    return result.stdout


roles = load_and_validate(ROOT / "tools/current-gates-v2.json", ROOT)
v1 = json.loads((ROOT / "tools/current-gates.json").read_text(encoding="utf-8"))
v1_ids = v1["expected_checker_ids"]
if [gate["checker_id"] for gate in roles["historical"][: len(v1_ids)]] != v1_ids:
    fail("historical roles do not preserve the immutable v1 prefix")
if not roles["current"]:
    fail("the live current role is empty")

descriptor_path = ROOT / "morphospace/features/timestamped-float32-sample.json"
descriptor = json.loads(descriptor_path.read_text(encoding="utf-8"))
source_binding = descriptor.get("source")
if not isinstance(source_binding, dict) or set(source_binding) != {"repo_id", "revision", "path", "sha256"}:
    fail("timestamped Float32 descriptor source binding is malformed")
revision, expected_source_sha = source_binding["revision"], source_binding["sha256"]
if source_binding["repo_id"] != "rusty-lsl" or not isinstance(revision, str) or not COMMIT.fullmatch(revision) or not isinstance(expected_source_sha, str) or not SHA256.fullmatch(expected_source_sha):
    fail("timestamped Float32 source identity is noncanonical")
source_relative = safe_relative(source_binding["path"])
source_path = ROOT / Path(*source_relative.parts)
if not source_path.is_file() or sha(source_path) != expected_source_sha:
    fail("current Float32 source bytes do not match the descriptor")
if subprocess.run(["git", "merge-base", "--is-ancestor", revision, "HEAD"], cwd=ROOT, capture_output=True).returncode != 0:
    fail("descriptor source revision does not exist as an ancestor of HEAD")
if hashlib.sha256(git("show", f"{revision}:{source_relative}")).hexdigest() != expected_source_sha:
    fail("descriptor source revision/path does not contain the declared source bytes")

lock = json.loads((ROOT / "morphospace/feature.lock.json").read_text(encoding="utf-8"))
revision_value, fingerprint = lock.get("revision"), lock.get("lock_fingerprint")
if not isinstance(revision_value, int) or revision_value < 1 or not isinstance(fingerprint, str) or not SHA256.fullmatch(fingerprint):
    fail("feature lock revision or fingerprint is noncanonical")
if lock.get("resolver_version") != "rusty-morphospace-feature-resolver/2":
    fail("feature lock is not resolver-owned v2 output")
fingerprint_input = dict(lock)
fingerprint_input["lock_fingerprint"] = "0" * 64
calculated = hashlib.sha256(json.dumps(fingerprint_input, separators=(",", ":")).encode()).hexdigest()
if calculated != fingerprint:
    fail("feature lock fingerprint does not match its canonical relationships")
feature = next((item for item in lock.get("features", []) if item.get("feature_id") == descriptor.get("feature_id")), None)
if feature is None:
    fail("lock omits the timestamped Float32 descriptor feature")
locked_descriptor = feature.get("descriptor")
if not isinstance(locked_descriptor, dict):
    fail("lock descriptor projection is malformed")
descriptor_relative = safe_relative(locked_descriptor.get("path"))
if ROOT / "morphospace" / Path(*descriptor_relative.parts) != descriptor_path:
    fail("lock descriptor path does not identify the current descriptor")
if locked_descriptor.get("sha256") != sha(descriptor_path):
    fail("lock does not bind the current descriptor bytes")
for lock_key, source_key in (("source_repo", "repo_id"), ("source_revision", "revision"), ("source_path", "path"), ("source_sha256", "sha256")):
    if locked_descriptor.get(lock_key) != source_binding[source_key]:
        fail(f"lock source relationship disagrees at {lock_key}")

activation = (ROOT / "crates/rusty-lsl/src/runtime_activation.rs").read_text(encoding="utf-8")
fingerprint_match = re.search(r'ACCEPTED_FEATURE_LOCK_FINGERPRINT: &str =\s*"([0-9a-f]{64})"', activation)
revision_match = re.search(r"ACCEPTED_FEATURE_LOCK_REVISION: u64 = ([0-9]+);", activation)
if not fingerprint_match or fingerprint_match.group(1) != fingerprint or not revision_match or int(revision_match.group(1)) != revision_value:
    fail("runtime activation does not project the current feature lock")

workspace_path = Path(sys.argv[1]).resolve() if len(sys.argv) == 2 else ROOT / "morphospace/workspace.state.json"
workspace = json.loads(workspace_path.read_text(encoding="utf-8"))
registry = workspace.get("module_registry", {})
if registry.get("lock_revision") != revision_value or registry.get("lock_fingerprint") != fingerprint:
    fail("workspace module registry does not project the current feature lock")
lock_modules = sorted(item["module_id"] for item in lock["features"])
registry_modules = sorted(item["module_id"] for item in registry.get("modules", []))
if lock_modules != registry_modules:
    fail("workspace module inventory disagrees with the resolved lock")
if not any(item["blocker_id"] == "rlsl-lslc-003i-pinned-rust-180-const-bits-validation-blocked" for item in workspace.get("blockers", [])):
    fail("LSLC-003I blocked history was not preserved")

print("Durable live gate-role and dependency relationships passed")
