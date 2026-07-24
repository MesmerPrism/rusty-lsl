import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-003o-multichannel-numeric-record-sequence-observation.json"
FORMATS = {
    "double64": (8, 25, [[16777221.0, -16777222.0], [16777219.0, -16777220.0]]),
    "int32": (4, 17, [[65541, -65542], [65539, -65540]]),
    "int16": (2, 13, [[261, -262], [259, -260]]),
    "int8": (1, 11, [[5, -6], [3, -4]]),
}
HEX = set("0123456789abcdef")


def validate(document):
    assert document["schema"] == "rusty.lsl.lslc_003o.multichannel_numeric_record_sequence_observation.v1"
    assert document["official"] == {
        "package": "pylsl",
        "version": "1.18.2",
        "library_version": 117,
        "protocol_version": 110,
        "implementation_source_used": False,
    }
    assert document["bounds"] == {
        "address_family": "ipv4-loopback-unicast",
        "formats": 4,
        "directions_per_format": 2,
        "channels": 2,
        "initialization_records": 2,
        "caller_records": 3,
        "repeat_runs": 2,
        "finite_deadlines": True,
    }
    assert set(document["formats"]) == set(FORMATS)
    for name, (width, record_width, initialization) in FORMATS.items():
        evidence = document["formats"][name]
        assert evidence == {
            "value_width_bytes": width,
            "record_width_bytes": record_width,
            "initialization_values": initialization,
            "official_outlet_to_private_inlet": "pass",
            "private_outlet_to_official_inlet": "pass",
        }
    assert set(document["observed_dimensions"].values()) == {"pass"}
    negative = document["negative_observation"]
    for role in ["truncation", "extra_record", "malformed_record", "cancellation"]:
        assert negative[role] == "not-run-observation-only"
    assert "later implementation responsibility" in negative["reason"]
    for value in document["provenance"].values():
        assert len(value) == 64 and set(value) <= HEX
    attempts = document["preserved_non_acceptance_attempts"]
    assert [item["observed_package_version"] for item in attempts] == ["1.18.1", "1.18.1"]
    assert all("version-drift" in item["classification"] for item in attempts)
    assert all(len(item["raw_sha256"]) == 64 and set(item["raw_sha256"]) <= HEX for item in attempts)
    assert document["recommendation"] == "bounded-multichannel-multirecord-fixed-width-runtime"
    assert all(value is False for value in document["claims"].values())


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)

damaged = []
for mutation in [
    lambda value: value["bounds"].update(channels=1),
    lambda value: value["formats"]["int16"].update(record_width_bytes=12),
    lambda value: value["formats"]["int8"].update(initialization_values=[[5, 6], [3, 4]]),
    lambda value: value["formats"]["double64"].update(private_outlet_to_official_inlet="not-observed"),
    lambda value: value["observed_dimensions"].update(channel_order="unknown"),
    lambda value: value["negative_observation"].update(truncation="pass"),
    lambda value: value["claims"].update(production_implementation=True),
    lambda value: value["provenance"].update(driver_sha256="0" * 63),
]:
    candidate = copy.deepcopy(data)
    mutation(candidate)
    try:
        validate(candidate)
        damaged.append(False)
    except (AssertionError, KeyError, TypeError):
        damaged.append(True)
assert all(damaged)

text = FIXTURE.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml", "<info>", "appdata", "\\users\\", "127.0.0.1", "private-003o"]:
    assert forbidden not in text
assert text.endswith("\n")

required_routes = {
    "README.md": "LSLC-003O",
    "AGENTS.md": "LSLC-003O",
    "docs/COMPATIBILITY.md": "LSLC-003O",
    "docs/PROVENANCE.md": "LSLC-003O",
    "docs/VALIDATION.md": "check_lslc_003o.ps1",
    "fixtures/compatibility/README.md": "lslc-003o-multichannel-numeric-record-sequence-observation.json",
}
for path, marker in required_routes.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8")

print("LSLC-003O bounded multichannel record-sequence observation passed")
