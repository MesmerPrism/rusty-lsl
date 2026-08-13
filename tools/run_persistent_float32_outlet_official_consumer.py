#!/usr/bin/env python3
"""Qualify the managed Float32 outlet with a pinned official pylsl consumer."""

from __future__ import annotations

import argparse
import hashlib
import ipaddress
import json
import os
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path


PYLSL_VERSION = "1.18.2"
LIBLSL_VERSION = 117
PROTOCOL_VERSION = 110
SOURCE_ID = "rusty-lsl-interop-001-official-consumer"
RECORDS = 10
EXPECTED_VALUES = [index + 0.25 for index in range(RECORDS)]
MARKER = "RUSTY_LSL_INTEROP_001_SERVER "
RESULT_SCHEMA = "rusty.lsl.interop_001.official_consumer_qualification.v1"
SELF_TEST_SCHEMA = (
    "rusty.lsl.interop_001.official_consumer_qualification_self_test.v1"
)


def explicit_ipv4(raw: str) -> str:
    try:
        address = ipaddress.ip_address(raw)
    except ValueError as error:
        raise argparse.ArgumentTypeError("interface must be a canonical IPv4 address") from error
    if not isinstance(address, ipaddress.IPv4Address):
        raise argparse.ArgumentTypeError("interface must be IPv4")
    if address.is_unspecified or address.is_multicast or address.is_loopback:
        raise argparse.ArgumentTypeError(
            "interface must be an explicit active non-loopback unicast IPv4 address"
        )
    if str(address) != raw:
        raise argparse.ArgumentTypeError("interface must use canonical IPv4 spelling")
    return raw


def parse_server_output(output: str) -> dict[str, int]:
    payloads = [
        line.partition(MARKER)[2] for line in output.splitlines() if MARKER in line
    ]
    if len(payloads) != 1:
        raise ValueError(f"expected one server marker, observed {len(payloads)}")
    document = json.loads(payloads[0])
    required = {
        "discovery_requests",
        "accepted_consumers",
        "records",
        "closed_consumers",
    }
    if set(document) != required:
        raise ValueError("server payload fields do not match the closed contract")
    if any(not isinstance(value, int) or value < 0 for value in document.values()):
        raise ValueError("server payload counts must be non-negative integers")
    if document["discovery_requests"] < 1:
        raise ValueError("server did not handle an official resolver query")
    if document["accepted_consumers"] != 1:
        raise ValueError("server did not accept exactly one official consumer")
    if document["records"] != RECORDS:
        raise ValueError("server record count drifted")
    if document["closed_consumers"] != 1:
        raise ValueError("server did not close exactly one official consumer")
    return document


def require_exact_data(
    samples: list[list[float]],
    timestamps: list[float],
    expected_timestamps: list[float],
) -> None:
    if len(samples) != RECORDS or len(timestamps) != RECORDS:
        raise ValueError("official consumer did not receive the exact record extent")
    values: list[float] = []
    for sample in samples:
        if not isinstance(sample, list) or len(sample) != 1:
            raise ValueError("official consumer sample shape drifted from one channel")
        values.append(float(sample[0]))
    if values != EXPECTED_VALUES:
        raise ValueError("official consumer Float32 values differ from the exact oracle")
    for actual, expected in zip(timestamps, expected_timestamps, strict=True):
        if abs(float(actual) - expected) > 1e-12:
            raise ValueError("official consumer timestamps differ from the exact oracle")


def self_test() -> None:
    fixture = (
        MARKER
        + '{"discovery_requests":2,"accepted_consumers":1,"records":10,'
        + '"closed_consumers":1}'
    )
    parsed = parse_server_output(fixture)
    assert parsed["discovery_requests"] == 2
    expected_timestamps = [1000.0 + index * 0.01 for index in range(RECORDS)]
    require_exact_data(
        [[value] for value in EXPECTED_VALUES], expected_timestamps, expected_timestamps
    )
    try:
        parse_server_output(fixture + "\n" + fixture)
    except ValueError as error:
        assert "observed 2" in str(error)
    else:
        raise AssertionError("duplicate server markers were accepted")
    try:
        require_exact_data([[0.0]] * RECORDS, expected_timestamps, expected_timestamps)
    except ValueError as error:
        assert "values" in str(error)
    else:
        raise AssertionError("damaged values were accepted")


def drain_pipe(pipe, destination: list[str]) -> None:
    for line in iter(pipe.readline, ""):
        destination.append(line)
    pipe.close()


def wait_for_file(path: Path, process: subprocess.Popen[str], timeout: float) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if path.is_file():
            return
        if process.poll() is not None:
            raise RuntimeError("qualification server exited before readiness")
        time.sleep(0.01)
    raise RuntimeError("qualification server readiness deadline expired")


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--interface", type=explicit_ipv4)
    result.add_argument("--self-test", action="store_true")
    return result


def main() -> int:
    arguments = parser().parse_args()
    if arguments.self_test:
        self_test()
        print(json.dumps({"schema": SELF_TEST_SCHEMA, "status": "pass"}, sort_keys=True))
        return 0
    if arguments.interface is None:
        parser().error("--interface is required unless --self-test is used")

    import pylsl

    if pylsl.__version__ != PYLSL_VERSION:
        raise ValueError(f"pylsl version drifted from {PYLSL_VERSION}")
    if pylsl.library_version() != LIBLSL_VERSION:
        raise ValueError(f"liblsl version drifted from {LIBLSL_VERSION}")
    if pylsl.protocol_version() != PROTOCOL_VERSION:
        raise ValueError(f"protocol version drifted from {PROTOCOL_VERSION}")
    native_library = Path(pylsl.lib.lib._name).resolve()
    native_sha256 = hashlib.sha256(native_library.read_bytes()).hexdigest()

    root = Path(__file__).resolve().parents[1]
    with tempfile.TemporaryDirectory(prefix="rusty-lsl-interop-001-") as raw_temp:
        temp = Path(raw_temp)
        ready = temp / "ready.json"
        consumer_ready = temp / "consumer-ready"
        acknowledgement = temp / "ack"
        environment = os.environ.copy()
        environment.update(
            {
                "RUSTY_LSL_INTEROP_INTERFACE": arguments.interface,
                "RUSTY_LSL_INTEROP_READY_FILE": str(ready),
                "RUSTY_LSL_INTEROP_CONSUMER_READY_FILE": str(consumer_ready),
                "RUSTY_LSL_INTEROP_ACK_FILE": str(acknowledgement),
            }
        )
        process = subprocess.Popen(
            [
                "cargo",
                "test",
                "--quiet",
                "-p",
                "rusty-lsl",
                "--lib",
                "persistent_float32_outlet_official_consumer::interop_001_official_consumer_qualification_server",
                "--",
                "--exact",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ],
            cwd=root,
            env=environment,
            text=True,
            encoding="utf-8",
            errors="strict",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        assert process.stdout is not None and process.stderr is not None
        stdout: list[str] = []
        stderr: list[str] = []
        stdout_thread = threading.Thread(target=drain_pipe, args=(process.stdout, stdout))
        stderr_thread = threading.Thread(target=drain_pipe, args=(process.stderr, stderr))
        stdout_thread.start()
        stderr_thread.start()
        inlet = None
        consumer_error: Exception | None = None
        try:
            wait_for_file(ready, process, 30.0)
            ready_document = json.loads(ready.read_text(encoding="utf-8"))
            if ready_document != {
                "schema": "rusty.lsl.interop_001.server_ready.v1",
                "source_id": SOURCE_ID,
                "channels": 1,
                "records": RECORDS,
            }:
                raise ValueError("qualification server readiness shape drifted")
            streams = pylsl.resolve_byprop("source_id", SOURCE_ID, minimum=1, timeout=8.0)
            exact = [stream for stream in streams if stream.source_id() == SOURCE_ID]
            if len(exact) != 1:
                raise ValueError(
                    f"expected one exact official resolver result, observed {len(exact)}"
                )
            inlet = pylsl.StreamInlet(exact[0], max_buflen=10, recover=False)
            base_timestamp = pylsl.local_clock()
            expected_timestamps = [
                base_timestamp + index * 0.01 for index in range(RECORDS)
            ]
            consumer_ready.write_text(repr(base_timestamp) + "\n", encoding="utf-8")
            inlet.open_stream(timeout=5.0)
            samples: list[list[float]] = []
            timestamps: list[float] = []
            deadline = time.monotonic() + 5.0
            while len(samples) < RECORDS and time.monotonic() < deadline:
                sample, timestamp = inlet.pull_sample(
                    timeout=max(0.0, deadline - time.monotonic())
                )
                if timestamp:
                    samples.append(sample)
                    timestamps.append(timestamp)
            require_exact_data(samples, timestamps, expected_timestamps)
            acknowledgement.write_text("pass\n", encoding="utf-8")
        except Exception as error:  # preserve server diagnostics with consumer failures
            consumer_error = error
        finally:
            if inlet is not None:
                inlet.close_stream()
            try:
                return_code = process.wait(timeout=10.0)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
                raise RuntimeError("qualification server did not terminate")
            stdout_thread.join()
            stderr_thread.join()
        output = "".join(stdout + stderr)
        if consumer_error is not None:
            raise RuntimeError(
                f"official consumer failed: {consumer_error}\n"
                f"qualification server exit {return_code}:\n{output.strip()}"
            ) from consumer_error
        if return_code != 0:
            raise RuntimeError(
                f"qualification server failed with exit {return_code}:\n{output.strip()}"
            )
        server = parse_server_output(output)

    result = {
        "schema": RESULT_SCHEMA,
        "revision": subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=root,
            text=True,
            encoding="utf-8",
            errors="strict",
            capture_output=True,
            check=True,
        ).stdout.strip(),
        "working_tree_dirty": bool(
            subprocess.run(
                ["git", "status", "--porcelain"],
                cwd=root,
                text=True,
                encoding="utf-8",
                errors="strict",
                capture_output=True,
                check=True,
            ).stdout.strip()
        ),
        "official": {
            "package": "pylsl",
            "package_version": PYLSL_VERSION,
            "library_version": LIBLSL_VERSION,
            "protocol_version": PROTOCOL_VERSION,
            "native_library_sha256": native_sha256,
            "implementation_source_used": False,
        },
        "scope": {
            "platform_class": "single-windows-desktop-host",
            "interface_selection": "caller-explicit-active-private-ipv4",
            "channels": 1,
            "records": RECORDS,
            "consumers": 1,
        },
        "result": {
            "official_source_resolved": "pass",
            "persistent_handshake": "pass",
            "exact_float32_values": "pass",
            "exact_source_timestamps": "pass",
            "bounded_close": "pass",
            "discovery_requests": server["discovery_requests"],
        },
        "limitations": {
            "single_host": True,
            "single_platform": True,
            "cross_host": False,
            "device": False,
            "background_service": False,
            "default_interface_selection": False,
            "broad_liblsl_equivalence": False,
        },
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
