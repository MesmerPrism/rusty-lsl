#!/usr/bin/env python3
"""Validate the additive LSLC-003G public role/plane facades."""

from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
BASE = "67818eb02b79209e5c0b7a472ff2b454b8ca2958"


def fail(message: str) -> None:
    raise SystemExit(message)


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, text=True, capture_output=True
    ).stdout


lib_diff = git("diff", "--unified=0", BASE, "--", "crates/rusty-lsl/src/lib.rs")
added = [line[1:] for line in lib_diff.splitlines() if line.startswith("+") and not line.startswith("+++")]
removed = [line[1:] for line in lib_diff.splitlines() if line.startswith("-") and not line.startswith("---")]
if removed or sorted(added) != ["pub mod contract;", "pub mod runtime;"]:
    fail("crate-root compatibility exports changed beyond the two additive facades")

for relative in ("crates/rusty-lsl/src/contract.rs", "crates/rusty-lsl/src/runtime.rs"):
    text = (ROOT / relative).read_text(encoding="utf-8")
    if "pub use crate::" not in text:
        fail(f"{relative} must project crate-root public names")
    if re.search(r"^pub\s+(?:struct|enum|trait|fn|const|static|type|mod)\b", text, re.MULTILINE):
        fail(f"{relative} must remain re-export-only")

if git("diff", "--name-only", BASE, "--", "morphospace/feature.lock.json").strip():
    fail("feature lock changed in a facade-only unit")

runtime_sources = [
    "bounded_sample_queue_runtime.rs", "finite_sample_recovery_runtime.rs",
    "fixed_width_numeric_sample_runtime.rs", "integrated_clock_correction_runtime.rs",
    "runtime_activation.rs", "short_info_discovery_responder_runtime.rs",
    "stream_handshake.rs", "timestamped_float32_sample_runtime.rs", "udp_discovery.rs",
]
for source in runtime_sources:
    relative = f"crates/rusty-lsl/src/{source}"
    if git("diff", "--name-only", BASE, "--", relative).strip():
        fail(f"runtime implementation changed: {relative}")

test = (ROOT / "crates/rusty-lsl/tests/public_api.rs").read_text(encoding="utf-8")
for required in ("use rusty_lsl::{contract, runtime};", "rusty_lsl::RawSourceTimestamp", "runtime::RuntimeModule"):
    if required not in test:
        fail(f"external consumer is missing public-only proof: {required}")

print("LSLC-003G public role/plane facade evidence passed")
