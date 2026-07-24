# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "fixtures/compatibility/lslc-002t-bounded-timestamped-float32-sample-runtime.json"
data = json.loads(path.read_text(encoding="utf-8"))
assert data["schema"] == "rusty.lsl.lslc_002t.bounded_timestamped_float32_sample_runtime.v1"
assert data["endpoint"]["protocol_version"] == 110
assert data["endpoint"]["implementation_source_used"] is False
assert data["method"]["transport_scope"] == "IPv4-loopback-only"
assert data["method"]["samples_sent"] == data["record"]["record_count"] == 1
assert data["method"]["raw_artifacts_committed"] is False
for key in ["driver_sha256", "raw_observation_sha256"]:
    value = data["method"][key]
    assert len(value) == 64 and all(c in "0123456789abcdef" for c in value)
assert data["record"] == {"record_count": 1, "prefix_role": "single-sample-record-marker", "timestamp_role": "explicit-raw-source-f64", "value_role": "single-channel-float32", "byte_order": "little-endian", "timestamp_input": "finite-exactly-recovered", "value_input": "exactly-recovered"}
assert data["runtime"]["loopback_exact_bits"] == "pass"
assert all(value is False for value in data["claims"].values())
text = path.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml", "<info>", "127.0.0.1", "private-sample", "protoys", "raw_line", "python_executable"]:
    assert forbidden not in text
print("LSLC-002T bounded timestamped float32 sample evidence validation passed")
