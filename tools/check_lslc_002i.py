import json
from pathlib import Path

PATH = Path(__file__).resolve().parents[1] / "fixtures/compatibility/lslc-002i-default-discovery-destination-evidence.json"

def require(value, message):
    if not value: raise ValueError(message)

def main():
    raw = PATH.read_bytes()
    require(raw.endswith(b"\n") and not raw.endswith(b"\n\n") and b"\r" not in raw, "noncanonical fixture lines")
    data = json.loads(raw)
    require(data["classification"] == "public-documentation-specification-evidence-only", "classification drift")
    p = data["provenance"]
    require(p["source_revision"] == "f012f8cfe8894cab0529be77dd83c91d6d95537d", "revision drift")
    require(p["source_utf8_sha256"] == "b96b5976d569018713187b73b3b83b0c7136f8d128b46e5184fa41e9c8536294", "hash drift")
    require(not p["implementation_source_inspected"] and not p["black_box_observation_performed"], "evidence role drift")
    facts = data["documented_default_settings"]
    require((facts["transport_spelling"], facts["port_decimal_spelling"], facts["relationship_spelling"]) == ("UDP", "16571", "broadcasts and/or multicast"), "default facts drift")
    expected = [("FF02:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", True),("FF05:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", False),("FF08113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", True),("FF0E:113D:6FDD:2C17:A643:FFE2:1BD1:3CD2", True),("224.0.0.1", False),("224.0.0.183", False),("239.255.172.215", False)]
    require([(x["spelling"], x["source_parenthesized"]) for x in facts["multicast_displayed"]] == expected, "destination drift")
    require(set(data["interpretation"].values()) <= {"not-performed","not-claimed","not-observed","absent"}, "interpretation widened")
    print("LSLC-002I evidence checks passed.")

if __name__ == "__main__": main()
