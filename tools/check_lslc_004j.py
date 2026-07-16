#!/usr/bin/env python3
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-004j-explicit-ipv4-interface-multicast-responder.json"


def validate(data):
    assert data["schema"] == "rusty.lsl.compatibility.lslc_004j.v1"
    assert data["source"] == {
        "rusty_lsl_commit": "e37141c4e553a3db3af98a9511ff433235a94de6",
        "rusty_lsl_tree": "174659f58e61afdca4efa18fcb50ccee2e2a557d",
        "runtime_source_sha256": "e440886751f40e34e3ff80b4708e5bb3dd6391e0a5b526f89bf7f9489f9536ef",
        "public_api_source_sha256": "03c15e92a36045e465bb777002238c7072e7aa2b0812a2458c02c3e35a1ab93f",
    }
    assert data["composition"] == {
        "address_family": "ipv4",
        "group": "239.255.172.215",
        "port": 16571,
        "interface_selection": "caller-explicit-concrete-ipv4",
        "required_module": "short-info-discovery-responder",
        "repeats": 2,
        "queries_per_repeat": 1,
        "responses_per_repeat": 1,
        "finite_deadline": True,
        "cancellation_preserved": True,
        "membership_cleanup": True,
        "socket_cleanup": True,
        "active_interface_private_conformance": ["pass", "pass"],
        "loopback_wrapper_preserved": "pass",
        "nonconcrete_rejection": {
            "unspecified": "pass",
            "multicast": "pass",
            "broadcast": "pass",
        },
    }
    private = data["private_artifacts"]
    for key in ("driver_source_sha256", "driver_binary_sha256", "attempt_record_sha256"):
        assert len(private[key]) == 64 and int(private[key], 16) >= 0
    assert private["failed_attempts_preserved"] == 1
    assert private["published"] is False
    assert data["limitations"] == {
        "platform": "windows-single-host",
        "official_endpoint": False,
        "quest_or_device": False,
        "cross_host": False,
        "interface_enumeration": False,
        "default_interface_selection": False,
        "ambient_fallback": False,
        "portable_retry_policy": False,
        "ipv6": False,
    }
    assert all(value is False for value in data["claims"].values())
    encoded = json.dumps(data, sort_keys=True)
    assert not any(value in encoded for value in ("192.168.", "10.0.", "Wi-Fi", "InterfaceIndex", "serial"))


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
for route, value in [
    (("composition", "group"), "239.255.172.216"),
    (("composition", "port"), 16572),
    (("composition", "interface_selection"), "default"),
    (("composition", "repeats"), 1),
    (("composition", "nonconcrete_rejection", "unspecified"), "fail"),
    (("limitations", "interface_enumeration"), True),
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

source = (ROOT / "crates/rusty-lsl/src/short_info_discovery_responder_runtime.rs").read_text(encoding="utf-8")
for marker in (
    "run_explicit_ipv4_multicast_short_info_responder",
    "run_explicit_loopback_multicast_short_info_responder",
    "NonConcreteMulticastInterface",
    "interface.is_unspecified()",
    "interface.is_multicast()",
    "interface == Ipv4Addr::BROADCAST",
    "run_short_info_responder_on_socket",
    "lslc_004j_concrete_interface_entry_point_preserves_loopback_composition",
    "lslc_004j_nonconcrete_interfaces_reject_before_io",
):
    assert marker in source
routes = {
    "AGENTS.md": "LSLC-004J",
    "README.md": "LSLC-004J",
    "docs/ARCHITECTURE.md": "LSLC-004J",
    "docs/COMPATIBILITY.md": "LSLC-004J",
    "docs/VALIDATION.md": "check_lslc_004j.ps1",
    "fixtures/compatibility/README.md": FIXTURE.name,
}
for path, marker in routes.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8"), path
print("LSLC-004J explicit concrete IPv4 responder passed (6 damaged fixtures rejected)")
