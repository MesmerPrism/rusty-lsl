#!/usr/bin/env python3
"""Deterministically verify DOC-024 first-hop history coverage."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXPECTED_BASE = "8c6a01a80c5b41c59548b186c7063b937a94abf0"
EXPECTED_BASE_AGENTS_SHA256 = (
    "ec0b833c0f577e0f81e6a979ff590da0b2b819a7aa869676c4928ab9a033c735"
)
HISTORY_MARKER = "## DOC-024 First-Hop Pre-Compaction Snapshot"
UNIT_TOKEN = re.compile(r"\b(?:LSLC-|CORE-|STRM-|P\d{1,2}\b)", re.IGNORECASE)
FOCUSED_ROUTE_MARKERS = """
LSLC-001A LSLC-001B LSLC-001C LSLC-001D LSLC-001H
LSLC-003O LSLC-003P LSLC-003Q LSLC-003S LSLC-003T LSLC-003U LSLC-003V
LSLC-003W LSLC-003X LSLC-003Y LSLC-003Z LSLC-004A LSLC-004B LSLC-004C
LSLC-004D LSLC-004E LSLC-004F LSLC-004H LSLC-004J LSLC-004K LSLC-004M
LSLC-004N LSLC-004O LSLC-004P LSLC-004R LSLC-004S LSLC-004T LSLC-004U
LSLC-004V check_lslc_001e.ps1 check_lslc_001f.ps1 check_lslc_001g.ps1
check_lslc_001k.ps1 check_lslc_001l.ps1 check_lslc_001m.ps1
check_lslc_001n.ps1 check_lslc_001o.ps1 check_lslc_001p.ps1
check_lslc_001q.ps1 check_lslc_001r.ps1 check_lslc_001s.ps1
check_lslc_001t.ps1 check_lslc_001u.ps1 check_lslc_001v.ps1
check_lslc_001x.ps1 check_lslc_001z.ps1 check_lslc_002a.ps1
""".split()


def fail(message: str) -> None:
    raise SystemExit(f"DOC-024 coverage failed: {message}")


def git_blob(commit: str, path: str) -> bytes:
    result = subprocess.run(
        ["git", "show", f"{commit}:{path}"],
        cwd=ROOT,
        check=False,
        capture_output=True,
    )
    if result.returncode:
        fail(f"cannot read {path} from base commit {commit}")
    return result.stdout


def blocks(text: str) -> list[tuple[int, str]]:
    result: list[tuple[int, str]] = []
    start = 1
    current: list[str] = []
    for number, line in enumerate(text.splitlines(), start=1):
        if line.strip():
            if not current:
                start = number
            current.append(line)
            continue
        if current:
            result.append((start, " ".join(" ".join(current).split())))
            current = []
    if current:
        result.append((start, " ".join(" ".join(current).split())))
    return result


def require_fragments(text: str, fragments: list[str], context: str) -> None:
    missing = [fragment for fragment in fragments if fragment not in text]
    if missing:
        fail(f"{context} is missing: {', '.join(missing)}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-commit", default=EXPECTED_BASE)
    args = parser.parse_args()
    if args.base_commit != EXPECTED_BASE:
        fail(f"base commit must remain {EXPECTED_BASE}")

    base_bytes = git_blob(args.base_commit, "AGENTS.md")
    if hashlib.sha256(base_bytes).hexdigest() != EXPECTED_BASE_AGENTS_SHA256:
        fail("base AGENTS.md bytes do not match the admitted identity")
    try:
        base = base_bytes.decode("utf-8")
        agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        history = (ROOT / "docs/history/LSLC-WORK-UNIT-HISTORY.md").read_text(
            encoding="utf-8"
        )
        classification = (ROOT / "docs/WORKTREE-CLASSIFICATION.md").read_text(
            encoding="utf-8"
        )
    except UnicodeDecodeError as error:
        fail(f"non-UTF-8 text: {error}")

    base_blocks = blocks(base)
    agent_values = {value for _, value in blocks(agents)}
    history_values = {value for _, value in blocks(history)}
    missing = [line for line, value in base_blocks if value not in agent_values | history_values]
    if missing:
        fail(f"base paragraphs missing at starting lines {missing}")

    history_line = next(
        (number for number, line in enumerate(base.splitlines(), start=1) if line == "## Work-Unit History"),
        None,
    )
    if history_line is None:
        fail("base AGENTS.md has no Work-Unit History boundary")
    chronology = [
        (line, value)
        for line, value in base_blocks
        if (1 < line < history_line) or UNIT_TOKEN.search(value)
    ]
    missing_history = [line for line, value in chronology if value not in history_values]
    retained_router = [line for line, value in chronology if value in agent_values]
    if missing_history:
        fail(f"chronology paragraphs missing from history at lines {missing_history}")
    if retained_router:
        fail(f"base chronology paragraphs remain in AGENTS.md at lines {retained_router}")

    agent_lines = agents.splitlines()
    if len(agent_lines) > 180:
        fail(f"AGENTS.md has {len(agent_lines)} lines; maximum is 180")
    require_fragments(
        agents,
        [
            "## Current status",
            "## Purpose and authority",
            "## Read order",
            "## Implementation rules",
            "## Workflow and worktrees",
            "## Validation",
            "## Public and private boundary",
            "## Release boundary",
            "## Preserved history",
            "`0.0.0`",
            "`publish = false`",
            "default-disabled",
            "tools/validation-policy.json",
            "tools/dispatch_validation.py",
            "one writer per branch and worktree",
            "`codex/*`",
            "public embedded `morphospace/`",
            "`AGPL-3.0-or-later`",
        ],
        "AGENTS.md",
    )
    require_fragments(agents, FOCUSED_ROUTE_MARKERS, "AGENTS.md focused routes")
    if HISTORY_MARKER not in history:
        fail("canonical history is missing the DOC-024 snapshot marker")
    require_fragments(
        " ".join(classification.split()),
        [
            "d98d4cba1f28c5be7c2d4c7d2c361fa7eb95700d6e983d86d7a1128f6fd4602b",
            "99562173362bba103ea7a52faa733a85e4c65d8b",
            "6a5f71f8795a7e4ddeb04d82fd4da72e1353772c4a5c3de2da04eb9ef9a74308",
            "1116b1ccdbce642f96204ab7501d33b300466bc50dd44b019bf84df550960b5f",
            "56ed0c71776d7e74e74f420ea74fa9571514ab113b67a48c8d3c69feaa26bf68",
            "0c79d3ee86a692bc71196ac4b88c05023f759d60",
            "2bb28938a4d15e30a1a3adebec372aa82c52aacb",
            "bdf0464796d6bee8eca75b59ec83f117a362ce91",
            "8/8 passed",
            "No runtime bytes adopted",
            "Remote refs are a separate authority surface",
        ],
        "worktree classification",
    )

    report = {
        "base_commit": args.base_commit,
        "base_paragraphs": len(base_blocks),
        "chronology_paragraphs_moved": len(chronology),
        "current_agents_lines": len(agent_lines),
        "missing_paragraphs": 0,
        "result": "pass",
    }
    print(json.dumps(report, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
