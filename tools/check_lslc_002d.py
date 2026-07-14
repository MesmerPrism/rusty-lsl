import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
fixture = json.loads((root / "fixtures/compatibility/lslc-002d-short-info-query-crlf-correction-fixtures.json").read_text(encoding="utf-8"))
assert fixture["schema"] == "rusty.lsl.lslc_002d.short_info_query_crlf_correction_fixtures.v1"
assert fixture["primary_rejection"]["rejected_feature_head"] == "f013520852e4c60cc887a214f1d8c4a666f54ce2"
assert fixture["primary_rejection"]["history_preserved"] is True
p = fixture["provenance"]
assert p["source_utf8_sha256"] == "b96b5976d569018713187b73b3b83b0c7136f8d128b46e5184fa41e9c8536294"
assert p["logged_packet_byte_count"] == 65 == len(p["canonical_candidate"].encode("ascii"))
assert "presentation-only" in p["rst_code_block_indentation"]
assert "CRLF" in p["line_ending_inference"] and "62" in p["line_ending_inference"]
assert len(fixture["line_ending_damaged"]) >= 6
assert {x["id"] for x in fixture["line_ending_damaged"]} >= {"lf-only", "mixed-query-lf", "mixed-final-lf", "missing-cr", "extra-cr", "extra-lf"}
assert len(fixture["truncated"]) >= 4 and len(fixture["oversized"]) >= 2 and len(fixture["noncanonical"]) >= 5
assert not any(fixture["claims"].values())
source = (root / "crates/rusty-lsl/src/short_info_query_wire.rs").read_text(encoding="utf-8")
assert 'b"LSL:shortinfo\\r\\n"' in source
assert "InvalidLineEnding" in source and "find_crlf" in source
for forbidden in ["TcpStream", "UdpSocket", "std::net", "unsafe {"]:
    assert forbidden not in source
print("LSLC-002D CRLF correction fixture and source checks passed.")
