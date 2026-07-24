#!/usr/bin/env python3
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-004k-active-interface-official-resolver-observation.json"


def validate(data):
    assert data["schema"] == "rusty.lsl.active_interface_official_resolver_observation.v1"
    assert data["source"] == {
        "rusty_lsl_commit": "3cc46662983ab8216dafc885004f929f6d00e14a",
        "rusty_lsl_tree": "8e8d8512d1dd2fda9b415333a24324635958436c",
        "requester_source_sha256": "1133224e6b741cd8136a518022d7bafdf1cc205aedc82e953c03130b5f171b55",
        "responder_source_sha256": "e440886751f40e34e3ff80b4708e5bb3dd6391e0a5b526f89bf7f9489f9536ef",
    }
    assert data["official"] == {"pylsl": "1.18.2", "liblsl": 117, "protocol": 110}
    assert data["scope"] == {
        "family": "ipv4",
        "group": "239.255.172.215",
        "port": 16571,
        "interface_selection": "caller-explicit-active-private-ipv4",
        "platform_class": "single-windows-desktop-host",
        "repeats": 2,
        "queries_per_repeat": 1,
        "responses_per_repeat": 1,
        "maximum_deadline_milliseconds": 3000,
    }
    assert data["results"] == {
        "official_query_received": ["pass", "pass"],
        "independent_response_sent": ["pass", "pass"],
        "official_matching_source_resolved": ["pass", "pass"],
        "membership_cleanup": ["pass", "pass"],
        "socket_cleanup": ["pass", "pass"],
        "elapsed_milliseconds": [31.0, 15.0],
    }
    private = data["private_artifacts"]
    hashes = [private[key] for key in ("driver_source_sha256", "configuration_sha256", "attempt_record_sha256")] + private["raw_attempt_sha256"]
    assert all(len(value) == 64 and int(value, 16) >= 0 for value in hashes)
    assert private["failed_attempts_preserved"] == 2 and private["published"] is False
    assert data["limitations"] == {"single_host": True, "single_platform": True, "production_runtime_executed": False, "official_outlet": False, "cross_host": False, "quest_or_device": False, "portable_interface_or_retry_policy": False}
    assert data["claims"] == {"observation_only": True, "production_bytes_changed": False, "runtime_or_activation_widened": False, "broad_compatibility": False, "manifold_authority": False}
    encoded = json.dumps(data, sort_keys=True)
    assert not any(value in encoded for value in ("192.168.", "10.0.", "Wi-Fi", "InterfaceIndex", "private-host", "serial"))


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
for route, value in [
    (("official", "pylsl"), "1.18.1"),
    (("scope", "group"), "239.255.172.216"),
    (("scope", "repeats"), 1),
    (("results", "official_matching_source_resolved"), ["pass", "fail"]),
    (("limitations", "single_host"), False),
    (("claims", "production_bytes_changed"), True),
]:
    damaged = copy.deepcopy(data)
    target = damaged
    for part in route[:-1]:
        target = target[part]
    target[route[-1]] = value
    try:
        validate(damaged)
    except (AssertionError, KeyError, TypeError):
        continue
    raise SystemExit(f"damaged fixture accepted: {'.'.join(route)}")
for path, marker in {
    "AGENTS.md": "LSLC-004K",
    "README.md": "LSLC-004K",
    "docs/COMPATIBILITY.md": "LSLC-004K",
    "docs/PROVENANCE.md": "LSLC-004K",
    "docs/VALIDATION.md": "check_lslc_004k.ps1",
    "fixtures/compatibility/README.md": FIXTURE.name,
}.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8"), path
print("LSLC-004K active-interface official resolver observation passed (6 damaged fixtures rejected)")
