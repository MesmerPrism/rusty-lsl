#!/usr/bin/env python3
"""Reject public-tree content that belongs only in private or local state."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BUILD_SUFFIXES = {
    ".dll",
    ".dylib",
    ".exe",
    ".o",
    ".obj",
    ".pdb",
    ".rlib",
    ".rmeta",
    ".so",
}

PRIVATE_WORKSPACE_NAME = "Rusty" + "-XR-Refactor"
AGENT_COORDINATION_NAME = "Agent" + " Bureau"
LOCAL_UNIX_ROOT = "/" + "root" + "/"

CONTENT_PATTERNS = {
    "local Windows path": re.compile(r"(?i)(?<![A-Za-z0-9_])[A-Z]:[\\/]"),
    "local UNC path": re.compile(
        r"(?:^|[\s\"'(])\\\\[A-Za-z0-9._-]+\\[A-Za-z0-9$._-]+"
    ),
    "local Unix user path": re.compile(
        r"/(?:Users|home)/[^/\s]+/|" + re.escape(LOCAL_UNIX_ROOT)
    ),
    "private planning workspace": re.compile(
        re.escape(PRIVATE_WORKSPACE_NAME) + "|" + re.escape(AGENT_COORDINATION_NAME),
        re.IGNORECASE,
    ),
    "private QCL identifier": re.compile(r"\bQCL[-_ ]?\d{3}\b", re.IGNORECASE),
    "device serial assignment": re.compile(
        r"(?i)\b(?:device[_ -]?id|serial)\s*[:=]\s*[\"']?[A-Za-z0-9._:-]{8,}"
    ),
    "ADB device selector": re.compile(r"(?i)\badb\s+-s\s+[A-Za-z0-9._:-]{6,}"),
    "private key block": re.compile(r"BEGIN (?:RSA |OPENSSH |EC )?PRIVATE KEY"),
    "credential assignment": re.compile(
        r"(?i)\b(?:api[_-]?key|password|passwd|secret|token)\s*[:=]\s*[\"']?[^\s\"']{8,}"
    ),
    "bearer credential": re.compile(r"(?i)\bBearer\s+[A-Za-z0-9._~-]{12,}"),
    "provider credential": re.compile(r"\b(?:sk|gh[pousr])_[A-Za-z0-9_-]{12,}\b"),
    "trailing whitespace": re.compile(r"(?m)[ \t]+$"),
}


def repository_files() -> list[Path]:
    """Return tracked and visible untracked files using Git ignore rules."""
    result = subprocess.run(
        [
            "git",
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
        cwd=ROOT,
        check=True,
        capture_output=True,
    )
    return [ROOT / item.decode("utf-8") for item in result.stdout.split(b"\0") if item]


def content_violations(text: str) -> list[str]:
    """Return the names of boundary rules violated by one text payload."""
    return [name for name, pattern in CONTENT_PATTERNS.items() if pattern.search(text)]


def is_build_artifact(relative: Path) -> bool:
    """Identify likely build outputs without rejecting fixture binaries."""
    if relative.parts and relative.parts[0].lower() == "target":
        return True
    build_context = len(relative.parts) == 1 or any(
        part.lower() in {"build", "dist", "out", "target"}
        for part in relative.parts[:-1]
    )
    return relative.suffix.lower() in BUILD_SUFFIXES and build_context


def self_test() -> None:
    """Exercise representative accepts and rejects without embedding live leaks."""
    allowed = (
        "Tokens, credentials, device identifiers, and build artifacts must stay private. "
        "Rusty Morphospace observations are proposals, not authority."
    )
    if content_violations(allowed):
        raise AssertionError("ordinary public policy prose was rejected")

    rejected = [
        "C" + ":\\" + "Users\\example\\capture.json",
        "\\" + "\\host\\share\\capture.json",
        "/" + "root/private/capture.json",
        PRIVATE_WORKSPACE_NAME,
        "QCL" + "-100",
        "serial" + "=" + "ABCDEFGH12345678",
        "adb" + " -s " + "ABCDEFGH12345678",
        "token" + "=" + "example-secret-value",
        "trailing space" + " ",
    ]
    if any(not content_violations(value) for value in rejected):
        raise AssertionError("a representative private value was accepted")
    if not is_build_artifact(Path("target/debug/rusty-lsl.exe")):
        raise AssertionError("a Cargo build artifact was accepted")
    if is_build_artifact(Path("fixtures/oracle/example.dll")):
        raise AssertionError("a fixture binary was rejected only for its suffix")


def main() -> int:
    """Scan the public tree and report every rejected path or content match."""
    self_test()
    violations: list[str] = []
    for path in repository_files():
        relative = path.relative_to(ROOT)
        relative_text = relative.as_posix()
        violations.extend(
            f"{relative}: path contains {name}"
            for name in content_violations(relative_text)
            if name != "trailing whitespace"
        )
        if is_build_artifact(relative):
            violations.append(f"{relative}: tracked build artifact")
            continue
        data = path.read_bytes()
        if b"\0" in data:
            continue
        text = data.decode("utf-8", errors="strict")
        violations.extend(f"{relative}: {name}" for name in content_violations(text))
        if text and not text.endswith(("\n", "\r")):
            violations.append(f"{relative}: missing terminal newline")

    if violations:
        print("Public-boundary check failed:", file=sys.stderr)
        for violation in violations:
            print(f"- {violation}", file=sys.stderr)
        return 1

    print("Public-boundary check passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
