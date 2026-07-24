#!/usr/bin/env python3
"""Current declared-Rust validation without rewriting historical LSLC-003K."""

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLCHAIN = "1.80.0"
RUSTC_COMMIT = "051478957371ee0084a7c0913941d2a8c4757bb9"


def run(command: list[str], *, capture: bool = False) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=ROOT, text=True, capture_output=capture)
    if result.returncode:
        if capture:
            sys.stderr.write(result.stdout)
            sys.stderr.write(result.stderr)
        raise SystemExit(f"current Rust 1.80 gate failed: {' '.join(command)}")
    return result


identity = run(["rustup", "run", TOOLCHAIN, "rustc", "-Vv"], capture=True).stdout
if not re.search(r"^release: 1\.80\.0$", identity, re.MULTILINE):
    raise SystemExit("current Rust 1.80 gate failed: release identity")
if not re.search(rf"^commit-hash: {RUSTC_COMMIT}$", identity, re.MULTILINE):
    raise SystemExit("current Rust 1.80 gate failed: commit identity")

run(["rustup", "run", TOOLCHAIN, "cargo", "test", "-p", "rusty-lsl", "--test", "public_api", "--offline", "--locked"])
run(["rustup", "run", TOOLCHAIN, "cargo", "clippy", "--workspace", "--all-targets", "--offline", "--locked"])
print(f"Current Rust {TOOLCHAIN} public API and Clippy passed at {RUSTC_COMMIT}.")
