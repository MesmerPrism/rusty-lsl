#!/usr/bin/env python3
"""Byte-preserving AGENTS first-hop history extraction gate."""

from __future__ import annotations

import hashlib
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = "6dabc9419df9239571a55c2b3d89b35c44413f97"
ORIGINAL_SHA = "f7098ccf7fa9d3e68f227440f9da60a074bd000f5cc900259b9af244648f574f"
HISTORY_SHA = "cec765a71b9804e5d7a3db52194151c09d97001b832f8c68a5e09f0e4fbfb1f3"
HISTORY_START = 2
HISTORY_END = 866
POINTER = (
    b"## Work-Unit History\n\n"
    b"Chronological LSLC unit notes are preserved byte-for-byte in\n"
    b"[LSLC Work-Unit History](docs/history/LSLC-WORK-UNIT-HISTORY.md). They are\n"
    b"historical evidence and focused-check routing; the durable instructions below\n"
    b"govern current work.\n\n"
)


class ExtractionError(RuntimeError):
    pass


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def split_original(original: bytes) -> tuple[bytes, bytes, bytes]:
    if sha(original) != ORIGINAL_SHA:
        raise ExtractionError("accepted AGENTS source hash mismatch")
    lines = original.splitlines(keepends=True)
    if len(lines) != 1253:
        raise ExtractionError("accepted AGENTS line count mismatch")
    prefix = b"".join(lines[:HISTORY_START])
    history = b"".join(lines[HISTORY_START:HISTORY_END])
    suffix = b"".join(lines[HISTORY_END:])
    if not history.startswith(b"## LSLC-003K ") or not history.endswith(b"```\n\n"):
        raise ExtractionError("history boundary markers mismatch")
    if not suffix.startswith(b"Rusty LSL is a public Rusty Morphospace repository"):
        raise ExtractionError("durable suffix boundary mismatch")
    if history.count(b"\n## LSLC-") + int(history.startswith(b"## LSLC-")) != 54:
        raise ExtractionError("historical heading count mismatch")
    if sha(history) != HISTORY_SHA:
        raise ExtractionError("historical payload hash mismatch")
    return prefix, history, suffix


def validate(original: bytes, archive: bytes, current: bytes, readme: bytes, validation: bytes) -> None:
    prefix, history, suffix = split_original(original)
    if archive != history:
        raise ExtractionError("archive bytes differ from accepted history payload")
    if current != prefix + POINTER + suffix:
        raise ExtractionError("current AGENTS is not the exact routed reconstruction")
    if len(current.splitlines()) > 400:
        raise ExtractionError("current AGENTS exceeds the first-hop line budget")
    route = b"docs/history/LSLC-WORK-UNIT-HISTORY.md"
    if current.count(route) != 1 or readme.count(route) != 1:
        raise ExtractionError("AGENTS and README must each contain one archive route")
    if b"check_lslc_003l.ps1" not in validation or b"test_lslc_003l.py" not in validation:
        raise ExtractionError("validation router omits LSLC-003L gates")


def main() -> int:
    original = subprocess.run(
        ["git", "show", f"{BASE}:AGENTS.md"], cwd=ROOT, capture_output=True, check=True
    ).stdout
    validate(
        original,
        (ROOT / "docs/history/LSLC-WORK-UNIT-HISTORY.md").read_bytes(),
        (ROOT / "AGENTS.md").read_bytes(),
        (ROOT / "README.md").read_bytes(),
        (ROOT / "docs/VALIDATION.md").read_bytes(),
    )
    print("LSLC-003L exact AGENTS history extraction passed (54 headings; 396-line router).")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ExtractionError, OSError, subprocess.SubprocessError) as error:
        print(f"LSLC-003L failed: {error}", file=sys.stderr)
        raise SystemExit(1)
