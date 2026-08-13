#!/usr/bin/env python3
"""Measure and compare the current Polar Stream and Rusty LSL sender paths."""

from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import platform
import subprocess
import sys
import time
from pathlib import Path


POLAR_SCHEMA = "rusty.lsl.interop_001.polar_stream_liblsl_benchmark.v1"
RUSTY_SCHEMA = "rusty.lsl.persistent_float32_outlet_benchmark.v1"
COMPARISON_SCHEMA = "rusty.lsl.interop_001.sender_ab_comparison.v1"
SELF_TEST_SCHEMA = "rusty.lsl.interop_001.sender_ab_comparison_self_test.v1"
POLAR_SOURCE = "crates/polar-h10-output/src/lsl.rs"
PYLSL_VERSION = "1.18.2"
LIBLSL_VERSION = 117
PROTOCOL_VERSION = 110


def bounded_integer(name: str, minimum: int, maximum: int):
    def parse(raw: str) -> int:
        try:
            value = int(raw)
        except ValueError as error:
            raise argparse.ArgumentTypeError(f"{name} must be an integer") from error
        if not minimum <= value <= maximum:
            raise argparse.ArgumentTypeError(
                f"{name} must be in {minimum}..={maximum}"
            )
        return value

    return parse


def run(command: list[str], root: Path) -> str:
    completed = subprocess.run(
        command,
        cwd=root,
        text=True,
        encoding="utf-8",
        errors="strict",
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"{' '.join(command)} failed with exit {completed.returncode}:\n"
            f"{(completed.stdout + completed.stderr).strip()}"
        )
    return completed.stdout.strip()


def percentile_nearest_rank(samples: list[int], percentile: int) -> int:
    if not samples:
        raise ValueError("at least one timing sample is required")
    ordered = sorted(samples)
    rank = (len(ordered) * percentile + 99) // 100
    return ordered[rank - 1]


def host_document(root: Path) -> dict[str, str]:
    return {
        "system": platform.system(),
        "release": platform.release(),
        "machine": platform.machine(),
        "rustc": run(["rustc", "--version"], root),
    }


def read_json(path: Path) -> dict[str, object]:
    document = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(document, dict):
        raise ValueError(f"{path.name} must contain one JSON object")
    return document


def validate_polar_source(source: bytes) -> None:
    required = [
        b"lsl_push_chunk_ftp",
        b"lsl_local_clock",
        b"fn push_notification",
        b"self.scratch.as_ptr()",
    ]
    if any(fragment not in source for fragment in required):
        raise ValueError("Polar Stream revision does not contain the admitted chunk path")


def measure_polar(arguments: argparse.Namespace) -> dict[str, object]:
    import pylsl

    if pylsl.__version__ != PYLSL_VERSION:
        raise ValueError(f"pylsl version drifted from {PYLSL_VERSION}")
    if pylsl.library_version() != LIBLSL_VERSION:
        raise ValueError(f"liblsl version drifted from {LIBLSL_VERSION}")
    if pylsl.protocol_version() != PROTOCOL_VERSION:
        raise ValueError(f"protocol version drifted from {PROTOCOL_VERSION}")

    polar_root = arguments.polar_repository.resolve()
    revision = run(["git", "rev-parse", f"{arguments.polar_revision}^{{commit}}"], polar_root)
    source = subprocess.run(
        ["git", "show", f"{revision}:{POLAR_SOURCE}"],
        cwd=polar_root,
        capture_output=True,
        check=True,
    ).stdout
    validate_polar_source(source)
    native_library = Path(pylsl.lib.lib._name).resolve()

    source_id = f"rusty-lsl-interop-001-polar-{time.monotonic_ns()}"
    info = pylsl.StreamInfo(
        "Rusty LSL Interop 001 Polar reference",
        "qualification",
        arguments.channels,
        100.0,
        pylsl.cf_float32,
        source_id,
    )
    outlet = pylsl.StreamOutlet(info, chunk_size=0, max_buffered=360)
    streams = pylsl.resolve_byprop("source_id", source_id, minimum=1, timeout=5.0)
    exact = [stream for stream in streams if stream.source_id() == source_id]
    if len(exact) != 1:
        raise RuntimeError(f"expected one Polar reference stream, observed {len(exact)}")
    inlet = pylsl.StreamInlet(exact[0], max_buflen=10, recover=False)
    inlet.open_stream(timeout=5.0)
    if not outlet.wait_for_consumers(5.0):
        raise RuntimeError("official liblsl outlet did not observe its connected inlet")

    value_count = arguments.channels * arguments.records
    values = (ctypes.c_float * value_count)(
        *(float(index % 65_536) + 0.25 for index in range(value_count))
    )
    push_chunk = pylsl.lib.lib.lsl_push_chunk_ftp
    local_clock = pylsl.lib.lib.lsl_local_clock

    def push_once() -> None:
        timestamp = local_clock()
        result = push_chunk(
            outlet.obj,
            values,
            ctypes.c_ulong(value_count),
            ctypes.c_double(timestamp),
            ctypes.c_int(1),
        )
        if result != 0:
            raise RuntimeError(f"lsl_push_chunk_ftp failed with {result}")

    for _ in range(arguments.warmup):
        push_once()
    samples: list[int] = []
    for _ in range(arguments.iterations):
        started = time.perf_counter_ns()
        push_once()
        samples.append(time.perf_counter_ns() - started)
    inlet.close_stream()

    return {
        "schema": POLAR_SCHEMA,
        "polar_stream_revision": revision,
        "polar_sender_source_sha256": hashlib.sha256(source).hexdigest(),
        "host": host_document(polar_root),
        "official": {
            "package": "pylsl",
            "package_version": PYLSL_VERSION,
            "library_version": LIBLSL_VERSION,
            "protocol_version": PROTOCOL_VERSION,
            "native_library_sha256": hashlib.sha256(native_library.read_bytes()).hexdigest(),
        },
        "mode": "release-native-library",
        "channels": arguments.channels,
        "records_per_chunk": arguments.records,
        "warmup_pushes": arguments.warmup,
        "measured_pushes": arguments.iterations,
        "sample_count": arguments.iterations * arguments.records,
        "median_push_chunk_ns": percentile_nearest_rank(samples, 50),
        "p95_push_chunk_ns": percentile_nearest_rank(samples, 95),
        "transport_unit": (
            "one lsl_local_clock call and one lsl_push_chunk_ftp call with one "
            "connected official inlet"
        ),
        "interpretation": "descriptive-non-gating",
    }


def require_integer(document: dict[str, object], field: str) -> int:
    value = document.get(field)
    if not isinstance(value, int) or isinstance(value, bool) or value < 0:
        raise ValueError(f"{field} must be a non-negative integer")
    return value


def compare(rusty: dict[str, object], polar: dict[str, object]) -> dict[str, object]:
    if rusty.get("schema") != RUSTY_SCHEMA or polar.get("schema") != POLAR_SCHEMA:
        raise ValueError("benchmark input schema drifted")
    if rusty.get("working_tree_dirty") is not False:
        raise ValueError("Rusty LSL benchmark must bind a clean revision")
    for field in [
        "channels",
        "records_per_chunk",
        "warmup_pushes",
        "measured_pushes",
        "sample_count",
    ]:
        if require_integer(rusty, field) != require_integer(polar, field):
            raise ValueError(f"benchmark dimension mismatch: {field}")
    rusty_host = rusty.get("host")
    polar_host = polar.get("host")
    if not isinstance(rusty_host, dict) or not isinstance(polar_host, dict):
        raise ValueError("benchmark host evidence is missing")
    host_fields = ["system", "release", "machine"]
    if any(rusty_host.get(field) != polar_host.get(field) for field in host_fields):
        raise ValueError("benchmarks were not recorded on the exact same host class")
    rusty_median = require_integer(rusty, "median_push_chunk_ns")
    rusty_p95 = require_integer(rusty, "p95_push_chunk_ns")
    polar_median = require_integer(polar, "median_push_chunk_ns")
    polar_p95 = require_integer(polar, "p95_push_chunk_ns")
    if not all([rusty_median, rusty_p95, polar_median, polar_p95]):
        raise ValueError("benchmark percentiles must be nonzero")
    official = polar.get("official")
    if not isinstance(official, dict) or official.get("library_version") != LIBLSL_VERSION:
        raise ValueError("Polar result does not bind official liblsl 1.17")
    return {
        "schema": COMPARISON_SCHEMA,
        "subjects": {
            "rusty_lsl_revision": rusty.get("revision"),
            "polar_stream_revision": polar.get("polar_stream_revision"),
            "polar_sender_source_sha256": polar.get("polar_sender_source_sha256"),
            "official": official,
        },
        "host": {field: rusty_host[field] for field in host_fields},
        "subject_toolchains": {
            "rusty_lsl_rustc": rusty_host.get("rustc"),
            "polar_stream_rustc": polar_host.get("rustc"),
        },
        "dimensions": {
            field: rusty[field]
            for field in [
                "channels",
                "records_per_chunk",
                "warmup_pushes",
                "measured_pushes",
                "sample_count",
            ]
        },
        "rusty_lsl": {
            "median_ns": rusty_median,
            "p95_ns": rusty_p95,
            "transport_unit": rusty.get("transport_unit"),
        },
        "polar_stream_liblsl": {
            "median_ns": polar_median,
            "p95_ns": polar_p95,
            "transport_unit": polar.get("transport_unit"),
        },
        "ratios": {
            "rusty_to_liblsl_median": round(rusty_median / polar_median, 6),
            "rusty_to_liblsl_p95": round(rusty_p95 / polar_p95, 6),
        },
        "interpretation": (
            "same-host descriptive sender occupancy; transport units differ in timestamp "
            "handling and this is not BLE-to-recorder latency or a universal speed claim"
        ),
    }


def self_test() -> None:
    host = {"system": "Windows", "release": "11", "machine": "AMD64", "rustc": "x"}
    shared = {
        "host": host,
        "channels": 1,
        "records_per_chunk": 10,
        "warmup_pushes": 2,
        "measured_pushes": 4,
        "sample_count": 40,
    }
    rusty = {
        **shared,
        "schema": RUSTY_SCHEMA,
        "revision": "a" * 40,
        "working_tree_dirty": False,
        "median_push_chunk_ns": 20,
        "p95_push_chunk_ns": 30,
        "transport_unit": "rusty",
    }
    polar = {
        **shared,
        "schema": POLAR_SCHEMA,
        "polar_stream_revision": "b" * 40,
        "polar_sender_source_sha256": "c" * 64,
        "official": {"library_version": LIBLSL_VERSION},
        "median_push_chunk_ns": 5,
        "p95_push_chunk_ns": 10,
        "transport_unit": "polar",
    }
    result = compare(rusty, polar)
    assert result["ratios"] == {
        "rusty_to_liblsl_median": 4.0,
        "rusty_to_liblsl_p95": 3.0,
    }
    damaged = dict(polar, records_per_chunk=9)
    try:
        compare(rusty, damaged)
    except ValueError as error:
        assert "records_per_chunk" in str(error)
    else:
        raise AssertionError("dimension mismatch was accepted")


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    mode = result.add_mutually_exclusive_group(required=True)
    mode.add_argument("--measure-polar", action="store_true")
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--rusty-result", type=Path)
    result.add_argument("--polar-result", type=Path)
    result.add_argument("--polar-repository", type=Path)
    result.add_argument("--polar-revision", default="origin/main")
    result.add_argument("--channels", type=bounded_integer("channels", 1, 4096), default=1)
    result.add_argument("--records", type=bounded_integer("records", 1, 4096), default=10)
    result.add_argument("--warmup", type=bounded_integer("warmup", 0, 100_000), default=100)
    result.add_argument("--iterations", type=bounded_integer("iterations", 1, 100_000), default=1000)
    return result


def main() -> int:
    arguments = parser().parse_args()
    if arguments.self_test:
        self_test()
        print(json.dumps({"schema": SELF_TEST_SCHEMA, "status": "pass"}, sort_keys=True))
        return 0
    if arguments.measure_polar:
        if arguments.polar_repository is None:
            parser().error("--measure-polar requires --polar-repository")
        print(json.dumps(measure_polar(arguments), sort_keys=True, separators=(",", ":")))
        return 0
    if arguments.polar_result is None:
        parser().error("comparison requires --polar-result")
    print(
        json.dumps(
            compare(read_json(arguments.rusty_result), read_json(arguments.polar_result)),
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
