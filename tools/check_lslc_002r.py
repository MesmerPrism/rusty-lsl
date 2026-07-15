# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "fixtures/compatibility/lslc-002r-official-loopback-stream-handshake-observation.json"
data = json.loads(path.read_text(encoding="utf-8"))

assert data["schema"] == "rusty.lsl.lslc_002r.official_loopback_stream_handshake_observation.v1"
assert data["classification"] == "black-box-observed-public-safe-sanitized"
assert data["direction"] == "official-outlet-to-official-inlet-connection-setup"
endpoint = data["endpoint"]
assert endpoint == {
    "package": "pylsl", "package_version": "1.18.2", "wheel_platform": "win_amd64",
    "wheel_sha256": "3ea2693417c7d79766cebf967250fde78aa1a3ad2b198e40246d36f549dbfde1",
    "library_version": 117, "protocol_version": 110,
    "native_library_sha256": "8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff",
    "implementation_source_used": False,
}
provenance = data["configuration_provenance"]
assert provenance["source_owner"] == "sccn/labstreaminglayer"
assert provenance["source_path"] == "docs/info/lslapicfg.rst"
assert provenance["source_revision"] == "f012f8cfe8894cab0529be77dd83c91d6d95537d"
assert provenance["source_utf8_sha256"] == "2778121741124e90d3005e5b9efb421a885b51a65d16912da0bada142a0b1ef9"
assert provenance["implementation_source_inspected"] is False

method = data["method"]
assert method["transport_scope"] == "IPv4-loopback-only"
assert method["case_count"] == len(data["cases"]) == 2
assert method["maximum_resolved_streams"] == 1
assert method["samples_sent"] == method["samples_received"] == 0
assert method["raw_artifacts_committed"] is False
assert method["failed_attempts_preserved_privately"] is True
for name in ["driver_sha256", "configuration_sha256", "raw_observation_sha256"]:
    digest = method[name]
    assert len(digest) == 64 and all(c in "0123456789abcdef" for c in digest)

assert [(case["channel_count"], case["channel_format"]) for case in data["cases"]] == [(1, "float32"), (3, "string")]
for case in data["cases"]:
    assert case["resolved_stream_count"] == 1
    assert case["open_stream"] == "pass"
    assert case["full_info_shape"] == "exact-match"
    assert case["established_tcp_connection_count"] == 2
    assert case["established_tcp_all_loopback"] is True
    assert case["close_stream"] == "pass"
    assert case["process_cleanup"] == "child-exited"
    assert case["sample_api_called"] is False

assert data["observed_roles"] == [
    "bounded-stream-resolution", "bounded-connection-open", "bounded-full-info-admission",
    "explicit-connection-close", "scope-owned-process-cleanup",
]
assert all(value is False for value in data["claims"].values())
text = path.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml", "<info>", "127.0.0.1", "raw_line", "source_id", "hostname", "v4address", "v6address", "diagnostic_output", "python_executable"]:
    assert forbidden not in text
print("LSLC-002R official loopback stream handshake observation validation passed")
