# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json
from pathlib import Path
root = Path(__file__).resolve().parents[1]
path = root / "fixtures/compatibility/lslc-002x-official-loopback-float32-sample-interoperability-observation.json"
data = json.loads(path.read_text(encoding="utf-8"))
assert data["schema"] == "rusty.lsl.lslc_002x.official_loopback_float32_sample_interoperability_observation.v1"
assert data["endpoint"]["package_version"] == "1.18.2" and data["endpoint"]["implementation_source_used"] is False
assert data["method"]["transport_scope"] == "IPv4-loopback-only" and data["method"]["case_count"] == 2
for key, value in data["method"].items():
    if key.endswith("sha256"): assert len(value) == 64 and all(c in "0123456789abcdef" for c in value)
directions = {case["direction"]: case for case in data["cases"]}
forward = directions["official-outlet-to-rust-inlet"]
assert forward["connection"] == "completed" and forward["rust_outcome"] == "record-returned"
assert forward["expected_timestamp_bits_preserved"] is False and forward["expected_value_bits_preserved"] is False
reverse = directions["rust-outlet-to-official-inlet"]
assert reverse["resolution_count"] == 1 and reverse["rust_outcome"] == "typed-handshake-identity-mismatch"
assert reverse["official_outcome"] == "stream-lost-before-sample"
assert all(value is False for value in data["claims"].values())
text = path.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml", "<info>", "127.0.0.1", "private-002x", "appdata", "\\users\\", "endpoint_port", "raw_bytes"]: assert forbidden not in text
print("LSLC-002X official Float32 interoperability failure observation validation passed")
