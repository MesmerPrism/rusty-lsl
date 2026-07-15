# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json
from pathlib import Path
root=Path(__file__).resolve().parents[1]
path=root/"fixtures/compatibility/lslc-002y-official-float32-stream-initialization-compatibility-correction.json"
data=json.loads(path.read_text(encoding="utf-8"))
assert data["schema"]=="rusty.lsl.lslc_002y.official_float32_stream_initialization_compatibility_correction.v1"
assert data["basis"]=={"failure_unit":"LSLC-002X","official_implementation_source_used":False}
assert data["request_correction"]["variable_role"]=="finite-positive-endian-performance"
assert data["request_correction"]["all_other_fixed_roles"]==data["request_correction"]["identity_roles"]=="exact"
assert data["initialization"]["record_count"]==2 and data["initialization"]["position"]=="after-handshake-before-caller-sample"
for direction in ["official_outlet_to_rust_inlet","rust_outlet_to_official_inlet"]:
    case=data["rerun"][direction]; assert case["connection"]=="pass" and case["explicit_timestamp_bits"]==case["explicit_float32_bits"]=="pass"
for key,value in data["rerun"].items():
    if key.endswith("sha256"): assert len(value)==64 and all(c in "0123456789abcdef" for c in value)
assert all(value is False for value in data["claims"].values())
text=path.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml","<info>","127.0.0.1","private-002x","appdata","\\users\\"]: assert forbidden not in text
print("LSLC-002Y official Float32 compatibility correction validation passed")
