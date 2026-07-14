# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
from __future__ import annotations
import json, re, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_volatile_snapshot.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001s-volatile-provider-snapshot-admission-results.json"
TESTS = {"lslc_001s_complete_snapshot_preserves_values_and_allocations", "lslc_001s_lane_limits_reject_before_entry_inspection", "lslc_001s_cross_lane_rejects_with_exact_location", "lslc_001s_duplicate_rejects_before_missing_role", "lslc_001s_missing_role_uses_fixed_observed_order", "lslc_001s_complete_snapshot_delegates_unchanged_o_error"}
def require(condition: bool, message: str) -> None:
    if not condition: raise ValueError(message)
def main() -> int:
    subprocess.run(["python", "tools/check_lslc_001o.py"], cwd=ROOT, check=True)
    source = SOURCE.read_text(encoding="utf-8"); implementation = source.split("#[cfg(test)]", 1)[0]
    for marker in ("implementation_assigned", "runtime_assigned", "transport_owned", "LaneLimitExceeded", "CrossLaneRole", "DuplicateRole", "MissingRole", "StreamInfoVolatileFields::new", "values.map(Option::unwrap)", "no clock, revision, epoch, or witness"):
        require(marker in implementation, f"snapshot invariant missing: {marker}")
    for forbidden in ("unsafe", "std::time", "SystemTime", "std::env", "std::net", ".parse(", "Xml"):
        require(forbidden not in implementation, f"acquisition or representation surface opened: {forbidden}")
    require(set(re.findall(r"fn (lslc_001s_[a-z0-9_]+)\(", source)) == TESTS, "focused test inventory drifted")
    overlay = json.loads(OVERLAY.read_text(encoding="utf-8"))
    require(overlay["freshness_policy"] == "not-represented-or-inferred", "unsupported freshness policy entered")
    require(overlay["fixed_lane_counts"] == {"implementation-assigned": 1, "runtime-assigned": 4, "transport-owned": 6}, "lane counts drifted")
    require(set(overlay["focused_rust_tests"]) == TESTS, "overlay tests drifted")
    result = subprocess.run(["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked", "stream_info_volatile_snapshot::tests::lslc_001s_"], cwd=ROOT, check=True, capture_output=True, text=True)
    require("6 passed" in result.stdout, "focused Rust tests did not all pass")
    for path, marker in {"AGENTS.md":"check_lslc_001s.ps1", "README.md":"StreamInfoVolatileProviderSnapshot", "docs/ARCHITECTURE.md":"stream_info_volatile_snapshot", "docs/COMPATIBILITY.md":"LSLC-001S", "docs/PROVENANCE.md":"LSLC-001S", "docs/VALIDATION.md":"check_lslc_001s.ps1", "fixtures/compatibility/README.md":"LSLC-001S", "morphospace/README.md":"rlsl-lslc-001s-volatile-provider-snapshot-admission"}.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    print("LSLC-001S volatile provider snapshot admission checks passed."); return 0
if __name__ == "__main__": raise SystemExit(main())
