#!/usr/bin/env python3
"""Run the descriptive persistent Float32 chunk-outlet microbenchmark."""

from __future__ import annotations

import argparse
import json
import os
import platform
import subprocess
import sys
from pathlib import Path


MARKER = "RUSTY_LSL_PERSISTENT_FLOAT32_OUTLET_SAMPLES "
RESULT_SCHEMA = "rusty.lsl.persistent_float32_outlet_benchmark.v1"
SELF_TEST_SCHEMA = "rusty.lsl.persistent_float32_outlet_benchmark_self_test.v1"
MAX_PUSHES = 1_000_000
MAX_BYTES = 1_073_741_824


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


def percentile_nearest_rank(samples: list[int], percentile: int) -> int:
    if not samples:
        raise ValueError("at least one benchmark sample is required")
    if not 1 <= percentile <= 100:
        raise ValueError("percentile must be in 1..=100")
    ordered = sorted(samples)
    rank = (len(ordered) * percentile + 99) // 100
    return ordered[rank - 1]


def parse_benchmark_output(output: str) -> dict[str, object]:
    payloads = [
        line.partition(MARKER)[2] for line in output.splitlines() if MARKER in line
    ]
    if len(payloads) != 1:
        raise ValueError(f"expected one benchmark marker, observed {len(payloads)}")
    document = json.loads(payloads[0])
    required = {"channels", "records", "warmup", "iterations", "samples_ns"}
    if set(document) != required:
        raise ValueError("benchmark payload fields do not match the closed contract")
    samples = document["samples_ns"]
    if not isinstance(samples, list) or not samples:
        raise ValueError("benchmark payload needs at least one timing sample")
    if any(not isinstance(sample, int) or sample < 0 for sample in samples):
        raise ValueError("benchmark timing samples must be non-negative integers")
    return document


def run(command: list[str], root: Path, env: dict[str, str] | None = None) -> str:
    completed = subprocess.run(
        command,
        cwd=root,
        env=env,
        text=True,
        encoding="utf-8",
        errors="strict",
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        detail = (completed.stdout + completed.stderr).strip()
        raise RuntimeError(
            f"{' '.join(command)} failed with exit {completed.returncode}:\n{detail}"
        )
    return completed.stdout + completed.stderr


def self_test() -> None:
    fixture = (
        "test output\n"
        + MARKER
        + '{"channels":2,"records":3,"warmup":1,"iterations":5,'
        + '"samples_ns":[5,1,100,3,2]}\n'
    )
    parsed = parse_benchmark_output(fixture)
    samples = parsed["samples_ns"]
    assert isinstance(samples, list)
    assert percentile_nearest_rank(samples, 50) == 3
    assert percentile_nearest_rank(samples, 95) == 100
    try:
        parse_benchmark_output(fixture + fixture)
    except ValueError as error:
        assert "observed 2" in str(error)
    else:
        raise AssertionError("duplicate benchmark markers were accepted")
    try:
        bounded_integer("channels", 1, 4)("5")
    except argparse.ArgumentTypeError:
        pass
    else:
        raise AssertionError("out-of-range benchmark argument was accepted")


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument(
        "--channels", type=bounded_integer("channels", 1, 4096), default=1
    )
    result.add_argument(
        "--records", type=bounded_integer("records", 1, 4096), default=10
    )
    result.add_argument(
        "--warmup", type=bounded_integer("warmup", 0, 100_000), default=20
    )
    result.add_argument(
        "--iterations",
        type=bounded_integer("iterations", 1, 100_000),
        default=200,
    )
    result.add_argument("--self-test", action="store_true")
    return result


def main() -> int:
    arguments = parser().parse_args()
    if arguments.self_test:
        self_test()
        print(json.dumps({"schema": SELF_TEST_SCHEMA, "status": "pass"}, sort_keys=True))
        return 0

    pushes = arguments.warmup + arguments.iterations
    if pushes > MAX_PUSHES:
        parser().error(f"benchmark pushes must not exceed {MAX_PUSHES}")
    transport_bytes = pushes * arguments.records * (9 + arguments.channels * 4)
    if transport_bytes > MAX_BYTES:
        parser().error(f"benchmark transport bytes must not exceed {MAX_BYTES}")
    root = Path(__file__).resolve().parents[1]
    environment = os.environ.copy()
    environment.update(
        {
            "RUSTY_LSL_PERSISTENT_BENCH_CHANNELS": str(arguments.channels),
            "RUSTY_LSL_PERSISTENT_BENCH_RECORDS": str(arguments.records),
            "RUSTY_LSL_PERSISTENT_BENCH_WARMUP": str(arguments.warmup),
            "RUSTY_LSL_PERSISTENT_BENCH_ITERATIONS": str(arguments.iterations),
        }
    )
    output = run(
        [
            "cargo",
            "test",
            "--quiet",
            "-p",
            "rusty-lsl",
            "--release",
            "--lib",
            "persistent_float32_outlet_benchmark::perf_002_persistent_float32_outlet_benchmark",
            "--",
            "--exact",
            "--ignored",
            "--nocapture",
        ],
        root,
        environment,
    )
    measured = parse_benchmark_output(output)
    expected = {
        "channels": arguments.channels,
        "records": arguments.records,
        "warmup": arguments.warmup,
        "iterations": arguments.iterations,
    }
    for field, value in expected.items():
        if measured[field] != value:
            raise ValueError(f"benchmark payload {field} differs from the requested bound")
    samples = measured["samples_ns"]
    assert isinstance(samples, list)
    if len(samples) != arguments.iterations:
        raise ValueError("benchmark payload sample extent differs from iterations")

    result = {
        "schema": RESULT_SCHEMA,
        "revision": run(["git", "rev-parse", "HEAD"], root).strip(),
        "working_tree_dirty": bool(run(["git", "status", "--porcelain"], root).strip()),
        "host": {
            "system": platform.system(),
            "release": platform.release(),
            "machine": platform.machine(),
            "rustc": run(["rustc", "--version"], root).strip(),
        },
        "mode": "release",
        "channels": arguments.channels,
        "records_per_chunk": arguments.records,
        "warmup_pushes": arguments.warmup,
        "measured_pushes": arguments.iterations,
        "sample_count": arguments.iterations * arguments.records,
        "median_push_chunk_ns": percentile_nearest_rank(samples, 50),
        "p95_push_chunk_ns": percentile_nearest_rank(samples, 95),
        "transport_unit": "one reusable encoded Float32 chunk and one contiguous loopback TCP write with caller-supplied timestamps",
        "interpretation": "descriptive-non-gating",
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
