#!/usr/bin/env python3
"""Validate and dispatch Rusty LSL historical pins and live current gates."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import tempfile
from typing import Callable

SCHEMA_V1 = "rusty.lsl.current_gates.v1"
SCHEMA_V2 = "rusty.lsl.current_gates.v2"
CHECKER_ID = re.compile(r"^lslc-[0-9]{3}[a-z]$")
CHECKER_PATH = re.compile(r"^tools/check_lslc_[0-9]{3}[a-z]\.ps1$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")


class ManifestError(ValueError):
    """The selected gate manifest is malformed or incomplete."""


def _read_json(path: Path) -> dict[str, object]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ManifestError(f"cannot read gate manifest: {error}") from error
    if not isinstance(value, dict):
        raise ManifestError("gate manifest must be an object")
    return value


def _safe_path(value: object, pattern: re.Pattern[str] | None = None) -> str:
    if not isinstance(value, str) or (pattern is not None and not pattern.fullmatch(value)):
        raise ManifestError(f"noncanonical path: {value}")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or str(path) != value:
        raise ManifestError(f"path escapes repository boundary: {value}")
    return value


def _git(root: Path, *arguments: str, text: bool = False) -> bytes | str:
    result = subprocess.run(["git", *arguments], cwd=root, check=False, capture_output=True, text=text)
    if result.returncode != 0:
        detail = result.stderr.strip() if text else result.stderr.decode(errors="replace").strip()
        raise ManifestError(f"git {' '.join(arguments)} failed: {detail}")
    return result.stdout


def _sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_v1(manifest_path: Path, root: Path) -> list[dict[str, object]]:
    document = _read_json(manifest_path)
    if document.get("schema") != SCHEMA_V1 or document.get("revision") != 1:
        raise ManifestError("unsupported v1 schema or revision")
    expected, gates = document.get("expected_checker_ids"), document.get("gates")
    if not isinstance(expected, list) or not expected or not isinstance(gates, list):
        raise ManifestError("expected_checker_ids and gates must be nonempty arrays")
    actual = []
    for index, gate in enumerate(gates, 1):
        if not isinstance(gate, dict) or set(gate) != {"order", "checker_id", "path"}:
            raise ManifestError(f"v1 gate {index} has an unexpected shape")
        checker_id = gate["checker_id"]
        relative = _safe_path(gate["path"], CHECKER_PATH)
        if gate["order"] != index or not isinstance(checker_id, str) or not CHECKER_ID.fullmatch(checker_id):
            raise ManifestError("v1 gate order or ID is noncanonical")
        if checker_id.replace("-", "_") not in Path(relative).name or not (root / relative).is_file():
            raise ManifestError(f"v1 checker missing or mismatched: {checker_id}")
        actual.append(checker_id)
    if actual != expected or len(actual) != len(set(actual)):
        raise ManifestError("v1 inventory is incomplete, duplicated, or reordered")
    return gates


def load_and_validate(manifest_path: Path, root: Path) -> dict[str, list[dict[str, object]]]:
    document = _read_json(manifest_path)
    if document.get("schema") == SCHEMA_V1:
        return {"historical": [], "current": load_v1(manifest_path, root)}
    if document.get("schema") != SCHEMA_V2 or document.get("revision") != 2:
        raise ManifestError("unsupported gate schema or revision")
    if set(document) != {"schema", "revision", "v1_manifest", "historical_gates", "current_gates"}:
        raise ManifestError("v2 manifest has an unexpected shape")
    v1 = document["v1_manifest"]
    if not isinstance(v1, dict) or set(v1) != {"path", "sha256"}:
        raise ManifestError("v1 manifest binding is malformed")
    v1_path = _safe_path(v1["path"])
    if v1_path != "tools/current-gates.json" or not isinstance(v1["sha256"], str) or not SHA256.fullmatch(v1["sha256"]):
        raise ManifestError("v1 manifest binding is noncanonical")
    try:
        v1_bytes = (root / v1_path).read_bytes()
    except OSError as error:
        raise ManifestError(f"cannot read v1 manifest: {error}") from error
    if _sha(v1_bytes) != v1["sha256"]:
        raise ManifestError("v1 manifest bytes changed")
    v1_ids = [str(g["checker_id"]) for g in load_v1(root / v1_path, root)]

    historical, current = document["historical_gates"], document["current_gates"]
    if not isinstance(historical, list) or not isinstance(current, list) or not historical or not current:
        raise ManifestError("both v2 role inventories must be nonempty arrays")
    historical_ids = []
    historical_keys = {"order", "checker_id", "role", "pin", "receipt_path", "receipt_sha256", "launcher_path", "launcher_sha256", "companion_path", "companion_sha256"}
    for index, gate in enumerate(historical, 1):
        if not isinstance(gate, dict) or set(gate) != historical_keys or gate.get("order") != index or gate.get("role") != "historical":
            raise ManifestError(f"historical gate {index} has an unexpected shape, order, or role")
        checker_id, pin = gate["checker_id"], gate["pin"]
        if not isinstance(checker_id, str) or not CHECKER_ID.fullmatch(checker_id) or not isinstance(pin, str) or not COMMIT.fullmatch(pin):
            raise ManifestError(f"historical gate {index} has a noncanonical ID or pin")
        launcher = _safe_path(gate["launcher_path"], CHECKER_PATH)
        companion = _safe_path(gate["companion_path"])
        receipt = _safe_path(gate["receipt_path"])
        if launcher != f"tools/check_{checker_id.replace('-', '_')}.ps1" or companion != launcher[:-4] + ".py" or not receipt.startswith("morphospace/receipts/"):
            raise ManifestError(f"historical gate {index} paths do not match its role")
        for field in ("receipt_sha256", "launcher_sha256", "companion_sha256"):
            if not isinstance(gate[field], str) or not SHA256.fullmatch(gate[field]):
                raise ManifestError(f"historical gate {index} has an invalid {field}")
        if subprocess.run(["git", "merge-base", "--is-ancestor", pin, "HEAD"], cwd=root, capture_output=True).returncode != 0:
            raise ManifestError(f"historical pin is not ancestral: {checker_id}")
        for path_field, hash_field in (("receipt_path", "receipt_sha256"), ("launcher_path", "launcher_sha256"), ("companion_path", "companion_sha256")):
            if _sha(_git(root, "show", f"{pin}:{gate[path_field]}")) != gate[hash_field]:
                raise ManifestError(f"historical {path_field} hash mismatch: {checker_id}")
        historical_ids.append(checker_id)
    if historical_ids != v1_ids:
        raise ManifestError("historical roles do not exactly preserve the v1 inventory")

    current_ids = []
    for index, gate in enumerate(current, 1):
        if not isinstance(gate, dict) or set(gate) != {"order", "checker_id", "role", "path"} or gate.get("order") != index or gate.get("role") != "current":
            raise ManifestError(f"current gate {index} has an unexpected shape, order, or role")
        checker_id = gate["checker_id"]
        path = _safe_path(gate["path"], CHECKER_PATH)
        if not isinstance(checker_id, str) or not CHECKER_ID.fullmatch(checker_id) or path != f"tools/check_{checker_id.replace('-', '_')}.ps1" or not (root / path).is_file():
            raise ManifestError(f"current gate {index} is missing or noncanonical")
        current_ids.append(checker_id)
    if len(set(historical_ids + current_ids)) != len(historical_ids) + len(current_ids):
        raise ManifestError("historical and current roles overlap")
    return {"historical": historical, "current": current}


def dispatch(roles: dict[str, list[dict[str, object]]], root: Path, runner: Callable[..., object] = subprocess.run, materialize: bool = True) -> None:
    for gate in roles["historical"]:
        checker_id, pin = str(gate["checker_id"]), str(gate["pin"])
        print(f"HISTORICAL-GATE {gate['order']:02d} {checker_id} {pin}", flush=True)
        temporary = Path(tempfile.mkdtemp(prefix=f"rusty-lsl-{checker_id}-"))
        added = False
        try:
            if materialize:
                temporary.rmdir()
                result = subprocess.run(["git", "worktree", "add", "--detach", str(temporary), pin], cwd=root, check=False, capture_output=True, text=True)
                if result.returncode != 0:
                    raise RuntimeError(f"historical materialization failed: {checker_id}: {result.stderr.strip()}")
                added = True
            result = runner(["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(gate["launcher_path"])], cwd=temporary, check=False)
            if int(getattr(result, "returncode", 1)) != 0:
                raise RuntimeError(f"historical gate failed ({result.returncode}): {checker_id}")
        finally:
            if added:
                subprocess.run(["git", "worktree", "remove", "--force", str(temporary)], cwd=root, check=False, capture_output=True)
            shutil.rmtree(temporary, ignore_errors=True)
            if temporary.exists():
                raise RuntimeError(f"historical cleanup failed: {checker_id}")
    for gate in roles["current"]:
        checker_id, relative = str(gate["checker_id"]), str(gate["path"])
        print(f"CURRENT-GATE {gate['order']:02d} {checker_id} {relative}", flush=True)
        result = runner(["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", relative], cwd=root, check=False)
        if int(getattr(result, "returncode", 1)) != 0:
            raise RuntimeError(f"current gate failed ({result.returncode}): {checker_id}")


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    manifest = Path(sys.argv[1]).resolve() if len(sys.argv) == 2 else root / "tools/current-gates-v2.json"
    try:
        dispatch(load_and_validate(manifest, root), root)
    except (ManifestError, RuntimeError) as error:
        print(f"Current-gates dispatch failed: {error}", file=sys.stderr)
        return 1
    print("All declared historical and current gates passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
