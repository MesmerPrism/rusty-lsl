# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
from __future__ import annotations
import json, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_implementation_version_provider.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001t-implementation-version-acquisition-evidence-results.json"
TESTS = {"evidence_and_version_bounds_fail_closed", "exact_owner_witness_acquires_once_and_preserves_allocation", "provider_failure_and_each_witness_mismatch_are_typed"}
def require(condition: bool, message: str) -> None:
    if not condition: raise ValueError(message)
def main() -> int:
    subprocess.run(["python", "tools/check_lslc_001s.py"], cwd=ROOT, check=True)
    source = SOURCE.read_text(encoding="utf-8"); implementation = source.split("#[cfg(test)]", 1)[0]
    for marker in ("trait StreamInfoImplementationVersionProvider", "fn acquire(&mut self)", ".acquire()", "ProviderIdentityMismatch", "EpochMismatch", "RevisionMismatch", "max_implementation_code_points", "StreamInfoVolatileProviderValue::new"):
        require(marker in implementation, f"acquisition invariant missing: {marker}")
    require(implementation.count(".acquire()") == 1, "provider must be called exactly once")
    for forbidden in ("unsafe", "std::time", "SystemTime", "Instant", "std::env", "std::net", "std::thread", "spawn(", "StreamInfoVolatileProviderSnapshot::new"):
        require(forbidden not in implementation, f"ambient or complete-admission surface opened: {forbidden}")
    overlay = json.loads(OVERLAY.read_text(encoding="utf-8"))
    require(overlay["currentness_policy"] == "exact-owner-issued-witness-match-only", "currentness policy drifted")
    require(overlay["projected_roles"] == ["version"], "non-implementation role entered")
    require(set(overlay["focused_rust_tests"]) == TESTS, "test inventory drifted")
    result = subprocess.run(["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked", "stream_info_implementation_version_provider::tests::"], cwd=ROOT, check=True, capture_output=True, text=True)
    require("3 passed" in result.stdout, "focused Rust tests did not all pass")
    for path, marker in {"AGENTS.md":"check_lslc_001t.ps1", "README.md":"StreamInfoImplementationVersionProvider", "docs/ARCHITECTURE.md":"stream_info_implementation_version_provider", "docs/COMPATIBILITY.md":"LSLC-001T", "docs/PROVENANCE.md":"LSLC-001T", "docs/VALIDATION.md":"check_lslc_001t.ps1", "fixtures/compatibility/README.md":"LSLC-001T", "morphospace/README.md":"rlsl-lslc-001t-implementation-version-acquisition-evidence"}.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    print("LSLC-001T implementation-version acquisition evidence checks passed."); return 0
if __name__ == "__main__": raise SystemExit(main())
