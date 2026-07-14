import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "fixtures/compatibility/lslc-002e-short-info-response-observation.json"
data = json.loads(path.read_text(encoding="utf-8"))
assert data["schema"] == "rusty.lsl.lslc_002e.short_info_response_observation.v1"
assert data["classification"] == "black-box-observed-public-safe-sanitized"
assert data["endpoint"]["protocol_version"] == 110
assert data["endpoint"]["implementation_source_used"] is False
assert data["method"]["case_count"] == len(data["cases"]) == 2
assert data["method"]["multicast_or_broadcast"] is False
for case in data["cases"]:
    digits = str(case["query_identifier"])
    assert digits == "0" or not digits.startswith("0")
    assert case["prefix_bytes"] == len(digits) + 2
    assert case["response_bytes"] == case["prefix_bytes"] + case["document_bytes"]
    assert len(case["document_sha256"]) == 64
assert data["common_facts"] == {
    "prefix": "canonical-unsigned-decimal-query-identifier",
    "delimiter_hex": "0d0a",
    "document_starts_immediately_after_delimiter": True,
    "document_internal_line_ending": "LF",
    "document_final_byte_hex": "0a",
    "lslc_002a_parse": "pass-unchanged",
    "lslc_002b_typed_admission": "pass-unchanged",
    "query_identifier_interpretation": "uninterpreted",
}
assert all(value is False for value in data["claims"].values())
text = path.read_text(encoding="utf-8").lower()
for forbidden in ["response_base64", "<?xml", "<info>", "peer_family"]:
    assert forbidden not in text
print("LSLC-002E sanitized observation validation passed")
