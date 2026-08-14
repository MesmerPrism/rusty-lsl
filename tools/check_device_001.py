#!/usr/bin/env python3
"""Validate the sanitized DEVICE-001 physical Polar H10 evidence."""

from __future__ import annotations

import copy
import hashlib
import json
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/rlsl-device-001-polar-h10-publisher-qualification.json"
COMMIT = re.compile(r"[0-9a-f]{40}")
SHA256 = re.compile(r"[0-9a-f]{64}")
SOURCE_PATHS = {
    "rate_admission_sha256": "crates/rusty-lsl/src/stream_info_observed_document_admission.rs",
    "rate_composition_sha256": "crates/rusty-lsl/src/stream_info_static_numeric_spellings.rs",
    "persistent_outlet_sha256": "crates/rusty-lsl/src/persistent_float32_outlet.rs",
    "managed_service_sha256": "crates/rusty-lsl/src/persistent_float32_outlet_service.rs",
}


def validate(data: dict[str, object]) -> None:
    assert set(data) == {
        "schema", "source", "device_session", "rusty_official_consumer",
        "sender_benchmark", "polar_input_observation", "private_artifacts",
        "claims", "private_exclusions",
    }
    assert data["schema"] == "rusty.lsl.device_001.polar_h10_publisher_qualification.v1"
    source = data["source"]
    assert set(source) == {
        "rusty_lsl_commit", "rusty_lsl_tree", "polar_stream_executed_revision",
        "polar_stream_current_readback", "polar_input_tree",
        "polar_lsl_sender_blob", *SOURCE_PATHS,
    }
    assert all(COMMIT.fullmatch(source[key]) for key in (
        "rusty_lsl_commit", "rusty_lsl_tree", "polar_stream_executed_revision",
        "polar_stream_current_readback", "polar_input_tree", "polar_lsl_sender_blob",
    ))
    assert all(SHA256.fullmatch(source[key]) for key in SOURCE_PATHS)

    session = data["device_session"]
    assert session["device_class"] == "Polar H10"
    assert session["platform_class"] == "single-windows-desktop-host"
    assert session["negotiated_pdu_bytes"] == 232
    assert session["observed_connection_interval_ms"] == 15.0
    assert session["ecg"] == {
        "nominal_rate_hz": 130, "estimated_rate_hz": 130.1975,
        "frames": 39, "samples": 2847, "records_per_frame": 73, "channels": 1,
    }
    assert session["accelerometer"] == {
        "nominal_rate_hz": 200, "estimated_rate_hz": 202.5561,
        "frames": 112, "samples": 4032, "records_per_frame": 36, "channels": 3,
    }
    assert session["heart_rate_notifications"] == 24

    official = data["rusty_official_consumer"]
    assert official == {
        "package": "pylsl", "package_version": "1.18.2", "library_version": 117,
        "protocol_version": 110,
        "native_library_sha256": "8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff",
        "nominal_rate_hz": 130, "records": 73, "discovery_requests": 5,
        "official_source_resolved": "pass", "persistent_handshake": "pass",
        "exact_captured_float32_values": "pass", "exact_source_timestamps": "pass",
        "bounded_close": "pass",
    }
    benchmark = data["sender_benchmark"]
    assert benchmark["interpretation"] == "descriptive-not-ble-to-recorder-latency"
    assert benchmark["warmup_pushes"] == 100
    assert benchmark["measured_pushes_per_repeat"] == 1000
    assert benchmark["repeats"] == 5
    assert benchmark["ecg"] == {
        "channels": 1, "records_per_chunk": 73,
        "rusty_median_of_medians_ns": 13400, "rusty_median_p95_ns": 19300,
        "liblsl_median_of_medians_ns": 13400, "liblsl_median_p95_ns": 18300,
        "rusty_to_liblsl_median_ratio": 1.0,
    }
    assert benchmark["accelerometer"] == {
        "channels": 3, "records_per_chunk": 36,
        "rusty_median_of_medians_ns": 13900, "rusty_median_p95_ns": 17400,
        "liblsl_median_of_medians_ns": 7000, "liblsl_median_p95_ns": 9500,
        "rusty_to_liblsl_median_ratio": 1.985714,
    }
    assert benchmark["current_rusty_speed_advantage"] is False
    assert benchmark["transport_units_equal"] is False
    assert benchmark["estimated_extra_rusty_sender_occupancy_ns_per_second"] == 38823

    polar = data["polar_input_observation"]
    assert polar == {
        "direct_protocol_control_replies": "pass",
        "direct_protocol_pmd_notifications": 109,
        "direct_protocol_heart_rate_notifications": 15,
        "full_input_wrapper_connects": True,
        "full_input_wrapper_pmd_notifications": 0,
        "full_input_wrapper_runs": 2,
        "classification": "polar-stream-windows-input-wrapper-defect-outside-rusty-lsl",
    }
    private = data["private_artifacts"]
    assert set(private) == {"session_sha256", "ecg_sha256", "accelerometer_sha256", "published"}
    assert all(SHA256.fullmatch(private[key]) for key in (
        "session_sha256", "ecg_sha256", "accelerometer_sha256",
    ))
    assert private["published"] is False
    assert data["claims"] == {
        "one_windows_h10_full_rate_capture": True,
        "truthful_130_and_200_hz_metadata": True,
        "real_ecg_frame_to_official_consumer": True,
        "h10_shaped_sender_comparison": True,
        "rusty_transport_suitable_for_bounded_polar_adapter_pilot": True,
        "rusty_faster_than_liblsl": False,
        "current_polar_stream_end_to_end_integration": False,
        "production_replacement_recommended": False,
        "broad_liblsl_equivalence": False,
        "stable_or_release_ready": False,
    }
    assert data["private_exclusions"] == [
        "device-identity", "participant-identity", "machine-paths",
        "network-endpoints", "raw-samples", "raw-logs",
    ]
    encoded = json.dumps(data, sort_keys=True).lower()
    local_drive_prefixes = tuple(letter + ":" + chr(92) for letter in ("s", "c"))
    assert not any(fragment in encoded for fragment in (
        *local_drive_prefixes, "192.168.", "device_address", "device_serial",
        "sensor_timestamp_ns", "microvolts", "x_mg", "y_mg", "z_mg",
    ))


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
damaged = [
    (("device_session", "negotiated_pdu_bytes"), 23),
    (("device_session", "ecg"), {**data["device_session"]["ecg"], "records_per_frame": 1}),
    (("rusty_official_consumer", "exact_captured_float32_values"), "fail"),
    (("sender_benchmark", "current_rusty_speed_advantage"), True),
    (("sender_benchmark", "estimated_extra_rusty_sender_occupancy_ns_per_second"), 1),
    (("polar_input_observation", "full_input_wrapper_pmd_notifications"), 1),
    (("private_artifacts", "published"), True),
    (("claims", "production_replacement_recommended"), True),
    (("claims", "broad_liblsl_equivalence"), True),
]
for route, value in damaged:
    candidate = copy.deepcopy(data)
    candidate[route[0]][route[1]] = value
    try:
        validate(candidate)
    except (AssertionError, KeyError, TypeError):
        continue
    raise SystemExit(f"damaged DEVICE-001 fixture accepted: {route}")

revision = data["source"]["rusty_lsl_commit"]
tree = subprocess.check_output(
    ["git", "-C", str(ROOT), "rev-parse", f"{revision}^{{tree}}"], text=True
).strip()
assert tree == data["source"]["rusty_lsl_tree"]
for key, path in SOURCE_PATHS.items():
    content = subprocess.check_output(["git", "-C", str(ROOT), "show", f"{revision}:{path}"])
    assert hashlib.sha256(content).hexdigest() == data["source"][key], path

routes = {
    "AGENTS.md": "check_device_001.py",
    "README.md": "DEVICE-001",
    "docs/COMPATIBILITY.md": "130.1975 Hz",
    "docs/LSL-PRODUCTION-ROADMAP.md": "bounded Polar adapter pilot",
    "docs/VALIDATION.md": "rlsl-device-001-polar-h10-publisher-qualification.json",
    "fixtures/compatibility/README.md": "DEVICE-001",
}
for path, marker in routes.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8"), path

print("DEVICE-001 Polar H10 publisher evidence passed (9 damaged fixtures rejected)")
