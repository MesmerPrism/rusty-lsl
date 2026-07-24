import json
from pathlib import Path
PATH = Path(__file__).resolve().parents[1] / "fixtures/compatibility/lslc-002l-clock-offset-public-documentation-evidence.json"
def require(value, message):
    if not value: raise ValueError(message)
def main():
    raw = PATH.read_bytes(); require(raw.endswith(b"\n") and not raw.endswith(b"\n\n") and b"\r" not in raw, "noncanonical fixture lines")
    data = json.loads(raw); require(data["classification"] == "public-documentation-specification-evidence-only", "classification drift")
    p = data["provenance"]; require(p["source_revision"] == "f012f8cfe8894cab0529be77dd83c91d6d95537d", "revision drift"); require(p["source_utf8_sha256"] == "35bacbc81477d7e08554e42c6fa25382622954adecded9ab2101bf2061fc883e", "hash drift"); require(not p["implementation_source_inspected"] and not p["black_box_observation_performed"], "evidence role drift")
    facts = data["documented_clock_offset_measurement"]; require(facts["default_exchange_count"] == 8 and facts["periodicity"] == "every few seconds", "measurement facts drift")
    exchange = facts["raw_exchange"]; require(exchange["round_trip_time_formula"] == "(t3-t0) - (t2-t1)", "RTT formula drift"); require(exchange["clock_offset_formula"] == "((t1-t0) + (t2-t3)) / 2", "OFS formula drift"); require(facts["measurement_selection"] == "offset whose associated RTT value was minimal", "selection drift")
    limits = data["documented_limitations"]; require(limits["timestamp_mapping_owner"] == "recipient of the data", "mapping authority drift"); require(limits["library_mapping_policy"] == "does not perform any particular such mapping", "mapping policy drift")
    require(set(data["interpretation"].values()) <= {"absent", "not-performed", "not-observed", "not-claimed"}, "interpretation widened"); print("LSLC-002L evidence checks passed.")
if __name__ == "__main__": main()
