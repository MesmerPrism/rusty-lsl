import json
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
fixture_path = root / "fixtures/compatibility/lslc-002c-protocol-110-short-info-query-fixtures.json"
fixture_text = fixture_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_text)
assert fixture["schema"] == "rusty.lsl.lslc_002c.short_info_query_fixtures.v1"
provenance = fixture["provenance"]
assert provenance["classification"] == "independently-authored-public-safe"
assert provenance["source_role"] == "public-specification-evidence-only"
assert provenance["source_revision"] == "f012f8cfe8894cab0529be77dd83c91d6d95537d"
assert provenance["implementation_source_used"] is False
assert provenance["external_oracle_run"] is False
assert provenance["observed_example"].encode("ascii") == b"LSL:shortinfo\nsession_id='default'\n16577 11973266323178842010\n"
for family, minimum in [("valid", 4), ("damaged", 4), ("truncated", 4), ("oversized", 2), ("noncanonical", 6)]:
    assert len(fixture[family]) >= minimum
assert not any(fixture["claims"].values())
source = (root / "crates/rusty-lsl/src/short_info_query_wire.rs").read_text(encoding="utf-8")
for marker in ["LSL:shortinfo", "max_query_bytes", "max_payload_bytes", "NonCanonicalDecimal", "TrailingBytes", "try_reserve_exact"]:
    assert marker in source
for forbidden in ["TcpStream", "UdpSocket", "std::net", "multicast", "unsafe {"]:
    assert forbidden not in source
assert not re.search(r"(?i)interoperab(?:le|ility).*true", fixture_text)
print("LSLC-002C fixture, provenance, and source-boundary checks passed.")
