#!/usr/bin/env python3
"""Validate and execute the declared current Rusty LSL focused gates."""

from __future__ import annotations

import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
from typing import Callable

SCHEMA = "rusty.lsl.current_gates.v1"
CHECKER_ID = re.compile(r"^lslc-[0-9]{3}[a-z]$")
CHECKER_PATH = re.compile(r"^tools/check_lslc_[0-9]{3}[a-z]\.ps1$")


class ManifestError(ValueError):
    """The current-gates manifest is malformed or incomplete."""


def load_and_validate(manifest_path: Path, root: Path) -> list[dict[str, object]]:
    """Return validated gates after checking the complete inventory and files."""
    try:
        document = json.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ManifestError(f"cannot read current-gates manifest: {error}") from error
    if document.get("schema") != SCHEMA or document.get("revision") != 1:
        raise ManifestError("unsupported current-gates schema or revision")
    expected = document.get("expected_checker_ids")
    gates = document.get("gates")
    if not isinstance(expected, list) or not expected or not isinstance(gates, list):
        raise ManifestError("expected_checker_ids and gates must be nonempty arrays")
    if len(expected) != len(set(expected)) or any(not isinstance(item, str) or not CHECKER_ID.fullmatch(item) for item in expected):
        raise ManifestError("expected checker IDs must be unique canonical IDs")
    actual: list[str] = []
    for index, gate in enumerate(gates, start=1):
        if not isinstance(gate, dict) or set(gate) != {"order", "checker_id", "path"}:
            raise ManifestError(f"gate {index} has an unexpected shape")
        checker_id, relative = gate["checker_id"], gate["path"]
        if gate["order"] != index:
            raise ManifestError("gate order must be contiguous and deterministic")
        if not isinstance(checker_id, str) or not isinstance(relative, str):
            raise ManifestError(f"gate {index} fields must be typed")
        if not CHECKER_ID.fullmatch(checker_id) or not CHECKER_PATH.fullmatch(relative):
            raise ManifestError(f"gate {index} is not a canonical tools checker")
        path = PurePosixPath(relative)
        if path.is_absolute() or ".." in path.parts:
            raise ManifestError(f"gate {index} escapes the tools boundary")
        if checker_id.replace("-", "_") not in path.name:
            raise ManifestError(f"gate {index} ID/path mismatch")
        if not (root / Path(*path.parts)).is_file():
            raise ManifestError(f"required checker is missing: {relative}")
        actual.append(checker_id)
    if actual != expected:
        raise ManifestError("declared gates do not exactly match the expected ordered inventory")
    return gates


def dispatch(gates: list[dict[str, object]], root: Path, runner: Callable[..., object] = subprocess.run) -> None:
    """Execute validated gates in order and stop at the first nonzero result."""
    for gate in gates:
        checker_id, relative = str(gate["checker_id"]), str(gate["path"])
        print(f"CURRENT-GATE {gate['order']:02d} {checker_id} {relative}", flush=True)
        result = runner(
            ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", relative],
            cwd=root,
            check=False,
        )
        code = int(getattr(result, "returncode", 1))
        if code != 0:
            raise RuntimeError(f"current gate failed ({code}): {checker_id}")


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    manifest = Path(sys.argv[1]).resolve() if len(sys.argv) == 2 else root / "tools/current-gates.json"
    try:
        dispatch(load_and_validate(manifest, root), root)
    except (ManifestError, RuntimeError) as error:
        print(f"Current-gates dispatch failed: {error}", file=sys.stderr)
        return 1
    print("All declared current gates passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
