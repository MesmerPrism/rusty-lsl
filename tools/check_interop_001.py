#!/usr/bin/env python3
import copy
import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/rlsl-interop-001-managed-persistent-float32-outlet.json"
COMMIT = re.compile(r"[0-9a-f]{40}")
SHA256 = re.compile(r"[0-9a-f]{64}")

SOURCE_PATHS = {
    "persistent_outlet_sha256": "crates/rusty-lsl/src/persistent_float32_outlet.rs",
    "managed_service_sha256": "crates/rusty-lsl/src/persistent_float32_outlet_service.rs",
    "stream_handshake_sha256": "crates/rusty-lsl/src/stream_handshake.rs",
    "official_driver_sha256": "tools/run_persistent_float32_outlet_official_consumer.py",
    "comparison_driver_sha256": "tools/run_polar_stream_sender_ab_benchmark.py",
}


def validate(data):
    assert set(data) == {
        "schema", "source", "official", "scope", "official_results",
        "polar_comparison", "private_artifacts", "limitations", "claims",
        "private_exclusions",
    }
    assert data["schema"] == "rusty.lsl.interop_001.managed_persistent_float32_outlet.v1"
    source = data["source"]
    assert set(source) == {"rusty_lsl_commit", "rusty_lsl_tree", *SOURCE_PATHS}
    assert source["rusty_lsl_commit"] == "893cdc4d6e542086643f3c7d7429629b2bb19af4"
    assert COMMIT.fullmatch(source["rusty_lsl_tree"])
    assert all(SHA256.fullmatch(source[key]) for key in SOURCE_PATHS)
    assert data["official"] == {
        "package": "pylsl", "package_version": "1.18.2",
        "library_version": 117, "protocol_version": 110,
        "native_library_sha256": "8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff",
        "diagnostic_source_inspected": True,
        "diagnostic_source_revision": "64988c6a14b8dc3b3f270ece58eab4f480bfab43",
        "implementation_source_copied_or_translated": False,
    }
    assert data["scope"] == {
        "platform_class": "single-windows-desktop-host",
        "interface_selection": "caller-explicit-active-private-ipv4",
        "serialized_repeats": 2, "outlets": 1, "consumers": 1,
        "channels": 1, "records": 10,
    }
    assert data["official_results"] == {
        "discovery_requests": [2, 2],
        "official_source_resolved": ["pass", "pass"],
        "persistent_handshake": ["pass", "pass"],
        "exact_float32_values": ["pass", "pass"],
        "exact_source_timestamps": ["pass", "pass"],
        "bounded_close": ["pass", "pass"],
    }
    comparison = data["polar_comparison"]
    assert comparison == {
        "polar_stream_revision": "5e13f64c6247f3ff5c5f919711d53edc1a7c8da3",
        "polar_sender_source_sha256": "beeb63ede432949e1d39723e95eabbdff4e21765b3d034b662632631dcbe9579",
        "channels": 1, "records_per_chunk": 10, "warmup_pushes": 100,
        "measured_pushes": 1000, "rusty_median_ns": 14200,
        "rusty_p95_ns": 23500, "liblsl_median_ns": 4200,
        "liblsl_p95_ns": 5900, "rusty_to_liblsl_median": 3.380952,
        "rusty_to_liblsl_p95": 3.983051, "transport_units_equal": False,
        "current_rusty_speed_advantage": False,
    }
    private = data["private_artifacts"]
    assert set(private) == {
        "official_repeat_sha256", "rusty_benchmark_sha256",
        "polar_benchmark_sha256", "comparison_sha256", "published",
    }
    assert len(private["official_repeat_sha256"]) == 2
    assert all(SHA256.fullmatch(value) for value in private["official_repeat_sha256"])
    assert all(SHA256.fullmatch(private[key]) for key in (
        "rusty_benchmark_sha256", "polar_benchmark_sha256", "comparison_sha256"
    ))
    assert private["published"] is False
    assert data["limitations"] == {
        "single_host": True, "single_platform": True, "pull_sample_only": True,
        "background_service": False, "default_interface_selection": False,
        "multi_outlet_discovery": False, "cross_host": False,
        "cross_platform": False, "recovery_parity": False,
        "device_or_h10": False, "ble_to_recorder_latency": False,
        "stable_or_release_ready": False,
    }
    assert data["claims"] == {
        "bounded_official_consumer_path": True,
        "descriptive_sender_comparison": True,
        "broad_liblsl_equivalence": False, "production_suitability": False,
        "universal_performance_advantage": False,
        "runtime_default_activated": False, "manifold_authority": False,
    }
    assert data["private_exclusions"] == [
        "machine-paths", "interface-name-address-index", "endpoints",
        "raw-output", "environment", "machine-identity",
    ]
    encoded = json.dumps(data, sort_keys=True)
    drive_prefixes = tuple(letter + ":" + chr(92) for letter in ("S", "C"))
    assert not any(value in encoded for value in (
        *drive_prefixes, "192.168.", "10.0.", "InterfaceIndex",
        "computer_name", "user_name", "device_serial", "serial_number",
    ))


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
damaged = [
    (("official", "package_version"), "1.18.1"),
    (("official", "diagnostic_source_inspected"), False),
    (("official", "implementation_source_copied_or_translated"), True),
    (("scope", "serialized_repeats"), 1),
    (("official_results", "exact_float32_values"), ["pass", "fail"]),
    (("polar_comparison", "polar_stream_revision"), "0" * 40),
    (("polar_comparison", "current_rusty_speed_advantage"), True),
    (("private_artifacts", "published"), True),
    (("limitations", "device_or_h10"), True),
    (("claims", "broad_liblsl_equivalence"), True),
    (("claims", "production_suitability"), True),
    (("claims", "manifold_authority"), True),
]
for route, value in damaged:
    candidate = copy.deepcopy(data)
    candidate[route[0]][route[1]] = value
    try:
        validate(candidate)
    except (AssertionError, KeyError, TypeError):
        continue
    raise SystemExit(f"damaged fixture accepted: {route}")

revision = data["source"]["rusty_lsl_commit"]
tree = subprocess.check_output(
    ["git", "-C", str(ROOT), "rev-parse", f"{revision}^{{tree}}"], text=True
).strip()
assert tree == data["source"]["rusty_lsl_tree"]
for key, path in SOURCE_PATHS.items():
    content = subprocess.check_output(["git", "-C", str(ROOT), "show", f"{revision}:{path}"])
    assert hashlib.sha256(content).hexdigest() == data["source"][key], path

routes = {
    "AGENTS.md": "run_persistent_float32_outlet_official_consumer.py",
    "README.md": "14.2 µs median",
    "docs/ARCHITECTURE.md": "Caller-polled persistent Float32 discovery service",
    "docs/COMPATIBILITY.md": "5e13f64c6247f3ff5c5f919711d53edc1a7c8da3",
    "docs/LSL-PRODUCTION-ROADMAP.md": "no current Rusty LSL sender-occupancy advantage",
    "docs/STABLE_PUBLIC_API.md": "PersistentFloat32OutletService",
    "docs/VALIDATION.md": "check_interop_001.py",
}
for path, marker in routes.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8"), path

print("INTEROP-001 managed persistent Float32 outlet evidence passed (12 damaged fixtures rejected)")
