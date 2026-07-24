# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "fixtures/compatibility/lslc-002q-official-loopback-discovery-client-observation.json"
data = json.loads(path.read_text(encoding="utf-8"))

assert data["schema"] == "rusty.lsl.lslc_002q.official_loopback_discovery_client_observation.v1"
assert data["classification"] == "black-box-observed-public-safe-sanitized"
assert data["direction"] == "rusty-lsl-client-to-official-responder"
assert data["client"] == {
    "accepted_feature_head": "8e90370d267ae27775c703798e76b16e5b00d41c",
    "feature_id": "udp-discovery",
    "implementation_source": "independently-authored-rusty-lsl",
}
endpoint = data["endpoint"]
assert endpoint["package"] == "pylsl"
assert endpoint["package_version"] == "1.18.2"
assert endpoint["wheel_sha256"] == "3ea2693417c7d79766cebf967250fde78aa1a3ad2b198e40246d36f549dbfde1"
assert endpoint["library_version"] == 117
assert endpoint["protocol_version"] == 110
assert endpoint["native_library_sha256"] == "8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff"
assert endpoint["implementation_source_used"] is False

method = data["method"]
assert method["transport"] == "IPv4-loopback-unicast"
assert method["case_count"] == len(data["cases"]) == 2
assert method["maximum_datagram_bytes"] == 65535
assert method["maximum_responses"] == 1
assert method["runtime_deadline_seconds"] == 5
assert method["raw_artifacts_committed"] is False
assert method["failed_attempts_preserved_privately"] is True
for digest in [
    method["rust_driver_sha256"],
    method["orchestration_driver_sha256"],
    method["raw_observation_sha256"],
]:
    assert len(digest) == 64 and all(c in "0123456789abcdef" for c in digest)

assert [case["query_identifier"] for case in data["cases"]] == [17, 18446744073709551615]
for case in data["cases"]:
    assert case["response_bytes"] > case["document_bytes"] > 0
    assert case["termination"] == "response-limit"
    assert case["response_envelope_admission"] == "pass"
    assert case["document_shape_admission"] == "pass"
    assert case["typed_document_admission"] == "pass"

assert data["observed_facts"] == {
    "accepted_query_sent": True,
    "official_response_received": True,
    "unchanged_response_envelope_admitted": True,
    "unchanged_document_shape_admitted": True,
    "separate_typed_document_admission": "pass",
    "socket_cleanup": "scope-owned-client-process-exited",
    "query_identifier_interpretation": "uninterpreted",
}
assert all(value is False for value in data["claims"].values())

text = path.read_text(encoding="utf-8").lower()
for forbidden in [
    "response_base64", "<?xml", "<info>", "127.0.0.1", "return_port",
    "peer_family", "diagnostic_output",
]:
    assert forbidden not in text

print("LSLC-002Q official loopback discovery client observation validation passed")
