#!/usr/bin/env python3
"""Validate the exact LSLC-003I Rust 1.80 const-bit correction."""

import struct
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = "7181672bfb1f6baceabd87c7c27c4f2f3922b06b"
SOURCE = Path("crates/rusty-lsl/src/timestamped_float32_sample_runtime.rs")

before = subprocess.run(
    ["git", "show", f"{BASE}:{SOURCE.as_posix()}"],
    cwd=ROOT, check=True, text=True, capture_output=True,
).stdout
after = (ROOT / SOURCE).read_text(encoding="utf-8")

replacements = {
    "123_456.789_f64.to_bits()": "0x40fe240c9fbe76c9",
    "4.0_f32.to_bits()": "0x40800000",
    "2.0_f32.to_bits()": "0x40000000",
}
expected = before
for old, new in replacements.items():
    if expected.count(old) != 1:
        raise SystemExit(f"baseline does not contain exactly one {old}")
    expected = expected.replace(old, new)
if after != expected:
    raise SystemExit("runtime source differs from the exact three-expression substitution")

if struct.unpack(">Q", struct.pack(">d", 123_456.789))[0] != 0x40FE240C9FBE76C9:
    raise SystemExit("timestamp literal is not bit-identical")
if struct.unpack(">I", struct.pack(">f", 4.0))[0] != 0x40800000:
    raise SystemExit("first initialization value literal is not bit-identical")
if struct.unpack(">I", struct.pack(">f", 2.0))[0] != 0x40000000:
    raise SystemExit("second initialization value literal is not bit-identical")

for protected in ("morphospace/feature.lock.json", "features", "Cargo.toml", "Cargo.lock"):
    changed = subprocess.run(
        ["git", "diff", "--name-only", BASE, "--", protected],
        cwd=ROOT, check=True, text=True, capture_output=True,
    ).stdout.strip()
    if changed:
        raise SystemExit(f"out-of-scope protected path changed: {changed}")

print("LSLC-003I exact const-bit compatibility evidence passed")
