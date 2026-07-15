#!/usr/bin/env python3
"""Pinned Rust 1.80 Clippy baseline policy for LSLC-003K."""

from __future__ import annotations

import argparse
import copy
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BASELINE = ROOT / "tools" / "clippy-baseline-rust-1.80.json"
SCHEMA = "rusty.lsl.clippy_baseline.v1"
TOOLCHAIN = "1.80.0"
CLIPPY_VERSION = "clippy 0.1.80 (05147895 2024-07-21)"
RUSTC_RELEASE = "1.80.0"
RUSTC_COMMIT = "051478957371ee0084a7c0913941d2a8c4757bb9"
EXPECTED_REPORTED_COUNTS = [319, 350]


class PolicyError(RuntimeError):
    pass


def canonical(value: Any) -> str:
    return json.dumps(value, ensure_ascii=True, sort_keys=True, separators=(",", ":"))


def normalize_path(value: str) -> str:
    path = value.replace("\\", "/")
    root = str(ROOT).replace("\\", "/")
    if path.lower().startswith(root.lower() + "/"):
        path = path[len(root) + 1 :]
    rustc_prefix = f"/rustc/{RUSTC_COMMIT}/"
    if path.startswith(rustc_prefix):
        path = "<rustc>/" + path[len(rustc_prefix) :]
    if re.match(r"^[A-Za-z]:/", path) or path.startswith("/"):
        raise PolicyError(f"diagnostic path is not project-relative: {value}")
    return path


def normalize_span(span: dict[str, Any]) -> dict[str, Any]:
    return {
        "file_name": normalize_path(span["file_name"]),
        "line_start": span["line_start"],
        "line_end": span["line_end"],
        "column_start": span["column_start"],
        "column_end": span["column_end"],
        "is_primary": span["is_primary"],
        "label": span.get("label"),
        "suggested_replacement": span.get("suggested_replacement"),
        "suggestion_applicability": span.get("suggestion_applicability"),
    }


def normalize_message(message: dict[str, Any]) -> dict[str, Any]:
    code = message.get("code")
    return {
        "level": message["level"],
        "code": None if code is None else code.get("code"),
        "message": message["message"],
        "spans": [normalize_span(span) for span in message.get("spans", [])],
        "children": [normalize_message(child) for child in message.get("children", [])],
    }


def parse_clippy_stream(stdout: str) -> tuple[list[dict[str, Any]], list[int]]:
    diagnostics: list[dict[str, Any]] = []
    reported: list[int] = []
    for raw in stdout.splitlines():
        try:
            record = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if record.get("reason") != "compiler-message":
            continue
        message = record.get("message", {})
        if message.get("level") != "warning":
            continue
        if message.get("code") is None:
            match = re.fullmatch(r"(\d+) warnings emitted", message.get("message", ""))
            if match:
                reported.append(int(match.group(1)))
            continue
        diagnostics.append(normalize_message(message))
    diagnostics.sort(key=canonical)
    return diagnostics, reported


def inspect_toolchain() -> dict[str, str]:
    clippy = subprocess.run(
        ["rustup", "run", TOOLCHAIN, "cargo", "clippy", "-V"],
        cwd=ROOT, text=True, encoding="utf-8", errors="strict", capture_output=True, check=True,
    ).stdout.strip()
    rustc = subprocess.run(
        ["rustup", "run", TOOLCHAIN, "rustc", "-Vv"],
        cwd=ROOT, text=True, encoding="utf-8", errors="strict", capture_output=True, check=True,
    ).stdout
    release = re.search(r"^release: (.+)$", rustc, re.MULTILINE)
    commit = re.search(r"^commit-hash: (.+)$", rustc, re.MULTILINE)
    observed = {
        "rustup_toolchain": TOOLCHAIN,
        "clippy_version": clippy,
        "rustc_release": release.group(1) if release else "",
        "rustc_commit": commit.group(1) if commit else "",
    }
    expected = {
        "rustup_toolchain": TOOLCHAIN,
        "clippy_version": CLIPPY_VERSION,
        "rustc_release": RUSTC_RELEASE,
        "rustc_commit": RUSTC_COMMIT,
    }
    if observed != expected:
        raise PolicyError(f"pinned toolchain identity mismatch: {observed!r}")
    return observed


def run_clippy() -> tuple[list[dict[str, Any]], list[int]]:
    completed = subprocess.run(
        ["rustup", "run", TOOLCHAIN, "cargo", "clippy", "--workspace", "--all-targets",
         "--offline", "--locked", "--message-format=json"],
        cwd=ROOT, text=True, encoding="utf-8", errors="strict", capture_output=True, check=False,
    )
    if completed.returncode != 0:
        raise PolicyError(f"pinned Clippy execution failed ({completed.returncode}): {completed.stderr[-2000:]}")
    diagnostics, reported = parse_clippy_stream(completed.stdout)
    if reported != EXPECTED_REPORTED_COUNTS:
        raise PolicyError(f"reported warning counts drifted: expected {EXPECTED_REPORTED_COUNTS}, got {reported}")
    if len(diagnostics) != sum(reported):
        raise PolicyError(f"coded diagnostic count {len(diagnostics)} does not equal reported total {sum(reported)}")
    return diagnostics, reported


def make_document(diagnostics: list[dict[str, Any]], reported: list[int]) -> dict[str, Any]:
    return {
        "schema": SCHEMA,
        "revision": 1,
        "toolchain": {
            "rustup_toolchain": TOOLCHAIN,
            "clippy_version": CLIPPY_VERSION,
            "rustc_release": RUSTC_RELEASE,
            "rustc_commit": RUSTC_COMMIT,
        },
        "command": ["cargo", "clippy", "--workspace", "--all-targets", "--offline", "--locked", "--message-format=json"],
        "normalization": "coded warnings; project-relative slash paths; rendered text omitted; canonical diagnostic sort; duplicates retained",
        "reported_warning_counts": {"library": reported[0], "all_target_test": reported[1]},
        "coded_diagnostic_count": len(diagnostics),
        "diagnostics": diagnostics,
    }


def validate_document(document: dict[str, Any]) -> None:
    expected_keys = {"schema", "revision", "toolchain", "command", "normalization", "reported_warning_counts", "coded_diagnostic_count", "diagnostics"}
    if set(document) != expected_keys:
        raise PolicyError("baseline keys are not exact")
    if document["schema"] != SCHEMA or document["revision"] != 1:
        raise PolicyError("baseline schema or revision mismatch")
    if document["toolchain"] != make_document([], EXPECTED_REPORTED_COUNTS)["toolchain"]:
        raise PolicyError("baseline toolchain identity mismatch")
    counts = document["reported_warning_counts"]
    if counts != {"library": 319, "all_target_test": 350}:
        raise PolicyError("baseline reported warning counts mismatch")
    diagnostics = document["diagnostics"]
    if document["coded_diagnostic_count"] != 669 or len(diagnostics) != 669:
        raise PolicyError("baseline coded diagnostic count mismatch")
    if diagnostics != sorted(diagnostics, key=canonical):
        raise PolicyError("baseline diagnostics are not in canonical order")
    for diagnostic in diagnostics:
        if diagnostic.get("level") != "warning" or not diagnostic.get("code"):
            raise PolicyError("baseline contains a non-coded warning")
        for span in diagnostic.get("spans", []):
            normalize_path(span["file_name"])


def compare(document: dict[str, Any], diagnostics: list[dict[str, Any]], reported: list[int]) -> None:
    validate_document(document)
    if reported != [document["reported_warning_counts"]["library"], document["reported_warning_counts"]["all_target_test"]]:
        raise PolicyError("reported warning counts differ from baseline")
    if diagnostics != document["diagnostics"]:
        expected = [canonical(item) for item in document["diagnostics"]]
        actual = [canonical(item) for item in diagnostics]
        missing = sorted(set(expected) - set(actual))
        new = sorted(set(actual) - set(expected))
        raise PolicyError(f"diagnostic baseline drift: missing={len(missing)}, new={len(new)}, expected={len(expected)}, actual={len(actual)}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write-baseline", action="store_true")
    args = parser.parse_args()
    inspect_toolchain()
    diagnostics, reported = run_clippy()
    if args.write_baseline:
        document = make_document(diagnostics, reported)
        validate_document(document)
        BASELINE.write_text(json.dumps(document, indent=2, ensure_ascii=True) + "\n", encoding="utf-8", newline="\n")
        print(f"wrote {BASELINE.relative_to(ROOT)} with {len(diagnostics)} coded warnings")
        return 0
    document = json.loads(BASELINE.read_text(encoding="utf-8"))
    compare(document, diagnostics, reported)
    print("LSLC-003K pinned Rust 1.80 Clippy baseline policy passed (319 library / 350 all-target warnings).")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (PolicyError, OSError, subprocess.SubprocessError, json.JSONDecodeError) as error:
        print(f"LSLC-003K failed: {error}", file=sys.stderr)
        raise SystemExit(1)
