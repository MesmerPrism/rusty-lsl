# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "fixtures/compatibility/lslc-002s-bounded-stream-handshake-runtime.json"
data = json.loads(path.read_text(encoding="utf-8"))
assert data["schema"] == "rusty.lsl.lslc_002s.bounded_stream_handshake_runtime.v1"
assert data["classification"] == "black-box-derived-public-safe-sanitized-plus-independent-runtime"
assert data["endpoint"]["protocol_version"] == 110
assert data["endpoint"]["implementation_source_used"] is False
assert data["method"]["transport_scope"] == "IPv4-loopback-only"
assert data["method"]["raw_artifacts_committed"] is False
for key in ["request_driver_sha256", "request_raw_sha256", "response_driver_sha256", "response_raw_sha256"]:
    value = data["method"][key]
    assert len(value) == 64 and all(c in "0123456789abcdef" for c in value)
assert data["request"]["termination"] == data["response"]["termination"] == "empty-crlf-line"
assert data["runtime"]["loopback_pair"] == "pass"
assert data["runtime"]["port_rebind_after_return"] == "pass"
assert all(value is False for value in data["claims"].values())
text = path.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml", "<info>", "127.0.0.1", "raw_line", "private-wire", "protoys", "python_executable"]:
    assert forbidden not in text
print("LSLC-002S bounded stream handshake evidence validation passed")
