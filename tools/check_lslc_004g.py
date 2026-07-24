import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-004g-quest-ipv4-multicast-device-conformance.json"

def validate(data):
    assert data["schema"] == "rusty.lsl.quest_ipv4_multicast_device_conformance.v1"
    assert data["envelope"] == {
        "group": "239.255.172.215", "port": 16571, "query_count": 1,
        "response_count": 1, "finite_deadline": True,
        "same_process": False, "same_device": False,
    }
    assert data["platform"]["device_family"] == "Quest 3S"
    assert data["platform"]["device_count"] == 2
    assert data["platform"]["android_api"] == 34
    expected = {
        "join": "pass", "drop": "pass", "rejoin": "pass",
        "peer_query_received": "pass", "peer_response_received": "pass",
        "explicit_cancellation": "pass", "socket_cleanup": "pass",
        "multicast_lock_cleanup": "pass",
    }
    assert all(data["results"][key] == value for key, value in expected.items())
    assert data["results"]["target_package_fatals"] == 0
    assert data["results"]["target_packages_removed"] is True
    assert data["results"]["run_owned_forwards_after"] == 0
    assert data["results"]["run_owned_reverses_after"] == 0
    assert data["results"]["run_owned_properties_restored"] is True
    assert data["results"]["run_owned_staging_removed"] is True
    assert len(data["preserved_failures"]) == 2
    assert len(data["limitations"]) == 4
    assert len(data["hashes"]) == 7
    assert all(len(value) == 64 and set(value) <= set("0123456789abcdef") for value in data["hashes"].values())
    public = json.dumps(data).lower()
    for forbidden in ("340yc", "3487c", "logcat.txt", ".apk"):
        assert forbidden not in public

def main():
    data = json.loads(FIXTURE.read_text(encoding="utf-8"))
    validate(data)
    mutations = [
        ("group", lambda x: x["envelope"].update(group="239.0.0.1")),
        ("count", lambda x: x["envelope"].update(query_count=2)),
        ("device", lambda x: x["envelope"].update(same_device=True)),
        ("join", lambda x: x["results"].update(join="fail")),
        ("fatal", lambda x: x["results"].update(target_package_fatals=1)),
        ("cleanup", lambda x: x["results"].update(target_packages_removed=False)),
    ]
    for name, mutate in mutations:
        damaged = copy.deepcopy(data)
        mutate(damaged)
        try:
            validate(damaged)
        except AssertionError:
            continue
        raise AssertionError(f"damaged fixture accepted: {name}")
    print("LSLC-004G sanitized Quest device conformance validation passed.")

if __name__ == "__main__":
    main()
