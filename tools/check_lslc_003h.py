#!/usr/bin/env python3
"""Validate LSLC-003H manifest/dispatcher ownership and routing."""

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
BASE = "40e164440c17fe96e916e49038662227bb474cd0"
EXPECTED = [*(f"lslc-002{letter}" for letter in "qrstuvwxyz"), *(f"lslc-003{letter}" for letter in "abcdefgh")]


def fail(message: str) -> None:
    raise SystemExit(message)


manifest = json.loads((ROOT / "tools/current-gates.json").read_text(encoding="utf-8"))
ids = manifest.get("expected_checker_ids")
if ids != EXPECTED or [gate.get("checker_id") for gate in manifest.get("gates", [])] != EXPECTED:
    fail("current-gates manifest does not name the exact accepted ordered inventory")

for index, gate in enumerate(manifest["gates"], start=1):
    expected_path = f"tools/check_{gate['checker_id'].replace('-', '_')}.ps1"
    if gate != {"order": index, "checker_id": gate["checker_id"], "path": expected_path}:
        fail(f"noncanonical gate entry at order {index}")
    if not (ROOT / expected_path).is_file():
        fail(f"required checker missing: {expected_path}")

historical = [f"tools/check_{checker.replace('-', '_')}.ps1" for checker in EXPECTED[:-1]]
changed = subprocess.run(
    ["git", "diff", "--name-only", BASE, "--", *historical],
    cwd=ROOT, check=True, text=True, capture_output=True,
).stdout.strip()
if changed:
    fail(f"accepted direct checker changed: {changed}")

aggregate = (ROOT / "tools/check_all.ps1").read_text(encoding="utf-8")
if "python tools/dispatch_current_gates.py" not in aggregate:
    fail("owner aggregate does not invoke the current-gates dispatcher")
if "check_lslc_" in aggregate:
    fail("owner aggregate still embeds a direct focused-checker list")

ci = (ROOT / ".github/workflows/ci.yml").read_text(encoding="utf-8")
if "tools\\check_all.ps1" not in ci or "tools\\dispatch_current_gates.py" in ci:
    fail("CI must route through the complete owner aggregate only")

for forbidden in ("morphospace/feature.lock.json", "crates/rusty-lsl/src"):
    if subprocess.run(["git", "diff", "--name-only", BASE, "--", forbidden], cwd=ROOT, check=True, text=True, capture_output=True).stdout.strip():
        fail(f"validation-only unit changed forbidden runtime/lock path: {forbidden}")

print("LSLC-003H current-gates dispatcher evidence passed")
