import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
fixture = json.loads((root / "fixtures/compatibility/lslc-002b-observed-document-typed-admission-fixtures.json").read_text(encoding="utf-8"))
assert fixture["schema"] == "rusty.lsl.lslc_002b.typed_admission_fixtures.v1"
assert fixture["provenance"] == "independently-authored-public-safe"
assert len(fixture["accepted"]) == 7 and len(fixture["damaged"]) == 6
assert {x["channel_format"] for x in fixture["accepted"]} == {"float32", "double64", "string", "int32", "int16", "int8", "int64"}
assert len({x["nominal_srate"] for x in fixture["accepted"]}) == 6
assert not any(fixture["claims"].values())
source = (root / "crates/rusty-lsl/src/stream_info_observed_document_admission.rs").read_text(encoding="utf-8")
for marker in ["ParsedStreamInfoObservedDocument", "StreamDefinition", "StreamInfoVolatileFields", "InvalidChannelCount", "InvalidChannelFormat", "InvalidNominalSampleRate"]:
    assert marker in source
for forbidden in ["TcpStream", "UdpSocket", "std::net", "unsafe {"]:
    assert forbidden not in source
print("LSLC-002B fixture and source boundary checks passed.")
