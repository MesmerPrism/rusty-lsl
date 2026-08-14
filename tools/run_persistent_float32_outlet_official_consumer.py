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
LIBLSL_SOURCE_REVISION = "64988c6a14b8dc3b3f270ece58eab4f480bfab43"
SOURCE_ID = "rusty-lsl-interop-001-official-consumer"
RECORDS = 10
EXPECTED_VALUES = [index + 0.25 for index in range(RECORDS)]
MARKER = "RUSTY_LSL_INTEROP_001_SERVER "
RESULT_SCHEMA = "rusty.lsl.interop_001.official_consumer_qualification.v1"
SELF_TEST_SCHEMA = (
    "rusty.lsl.interop_001.official_consumer_qualification_self_test.v1"
)
MULTI_MARKER = "RUSTY_LSL_POLAR_001_MULTI_OUTLET "
POLAR_OFFICIAL_MARKER = "RUSTY_LSL_POLAR_001_OFFICIAL_SINGLE "
MULTI_SELF_TEST_SCHEMA = (
    "rusty.lsl.polar_001.multi_outlet_official_consumer_self_test.v1"
)
POLAR_OFFICIAL_SCHEMA = "rusty.lsl.polar_001.official_consumer_qualification.v1"
ECG_RECORDS = 73
ACC_RECORDS = 36
ECG_NAME = "Rusty LSL Polar ECG 130"
ACC_NAME = "Rusty LSL Polar ACC 200"
ECG_SOURCE_ID = "rusty-lsl-polar-ecg-130"
ACC_SOURCE_ID = "rusty-lsl-polar-acc-200"
ECG_UID = "71000000-0000-4000-8000-000000000130"
ACC_UID = "71000000-0000-4000-8000-000000000200"


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


def parse_multi_server_output(output: str) -> dict[str, int]:
    payloads = [
        line.partition(MULTI_MARKER)[2]
        for line in output.splitlines()
        if MULTI_MARKER in line
    ]
    if len(payloads) != 1:
        raise ValueError(f"expected one multi-outlet marker, observed {len(payloads)}")
    document = json.loads(payloads[0])
    expected = {
        "outlets": 2,
        "discovery_queries": 1,
        "discovery_responses": 2,
        "timedata_queries": 2,
        "accepted_consumers": 2,
        "ecg_records": ECG_RECORDS,
        "acc_records": ACC_RECORDS,
        "closed_consumers": 2,
    }
    if set(document) != set(expected):
        raise ValueError("multi-outlet payload fields do not match the closed contract")
    if any(not isinstance(value, int) or value < 0 for value in document.values()):
        raise ValueError("multi-outlet payload counts must be non-negative integers")
    for field, minimum in expected.items():
        if field in {"discovery_queries", "discovery_responses"}:
            if document[field] < minimum:
                raise ValueError(f"multi-outlet {field} was below {minimum}")
        elif document[field] != minimum:
            raise ValueError(f"multi-outlet {field} drifted from {minimum}")
    return document


def require_multi_exact_data(
    ecg: list[list[float]], acc: list[list[float]]
) -> None:
    if len(ecg) != ECG_RECORDS or any(len(sample) != 1 for sample in ecg):
        raise ValueError("ECG notification shape drifted from 73 one-channel records")
    if len(acc) != ACC_RECORDS or any(len(sample) != 3 for sample in acc):
        raise ValueError("ACC notification shape drifted from 36 three-channel records")
    if [sample[0] for sample in ecg] != [float(index) for index in range(ECG_RECORDS)]:
        raise ValueError("ECG values differ from the exact oracle")
    expected_acc = [
        [float(index), float(index + 1), float(index + 2)]
        for index in range(ACC_RECORDS)
    ]
    if acc != expected_acc:
        raise ValueError("ACC values differ from the exact oracle")


def multi_outlet_self_test() -> None:
    fixture = MULTI_MARKER + json.dumps(
        {
            "outlets": 2,
            "discovery_queries": 3,
            "discovery_responses": 6,
            "timedata_queries": 2,
            "accepted_consumers": 2,
            "ecg_records": ECG_RECORDS,
            "acc_records": ACC_RECORDS,
            "closed_consumers": 2,
        },
        sort_keys=True,
        separators=(",", ":"),
    )
    parsed = parse_multi_server_output(fixture)
    assert parsed["discovery_responses"] == 6
    ecg = [[float(index)] for index in range(ECG_RECORDS)]
    acc = [
        [float(index), float(index + 1), float(index + 2)]
        for index in range(ACC_RECORDS)
    ]
    require_multi_exact_data(ecg, acc)
    try:
        parse_multi_server_output(fixture + "\n" + fixture)
    except ValueError as error:
        assert "observed 2" in str(error)
    else:
        raise AssertionError("duplicate multi-outlet markers were accepted")
    try:
        require_multi_exact_data(ecg, acc[:-1])
    except ValueError as error:
        assert "ACC notification shape" in str(error)
    else:
        raise AssertionError("damaged ACC record extent was accepted")
    for role, channels, rate_hz, records in (
        ("ecg", 1, 130, ECG_RECORDS),
        ("acc", 3, 200, ACC_RECORDS),
    ):
        official_fixture = POLAR_OFFICIAL_MARKER + json.dumps(
            {
                "role": role,
                "channels": channels,
                "rate_hz": rate_hz,
                "records": records,
                "discovery_requests": 2,
                "accepted_consumers": 1,
                "closed_consumers": 1,
            },
            sort_keys=True,
            separators=(",", ":"),
        )
        assert parse_polar_official_server_output(official_fixture, role)["role"] == role
        damaged_official = official_fixture.replace(
            f'"channels":{channels}', f'"channels":{channels + 1}'
        )
        try:
            parse_polar_official_server_output(damaged_official, role)
        except ValueError as error:
            assert "channels" in str(error)
        else:
            raise AssertionError("damaged official Polar shape was accepted")

    class FakeStream:
        def __init__(self, **overrides: object) -> None:
            self.fields = {
                "name": ECG_NAME,
                "type": "ECG",
                "channels": 1,
                "rate": 130.0,
                "format": 1,
                "source_id": ECG_SOURCE_ID,
                "uid": ECG_UID,
                **overrides,
            }

        def name(self):
            return self.fields["name"]

        def type(self):
            return self.fields["type"]

        def channel_count(self):
            return self.fields["channels"]

        def nominal_srate(self):
            return self.fields["rate"]

        def channel_format(self):
            return self.fields["format"]

        def source_id(self):
            return self.fields["source_id"]

        def uid(self):
            return self.fields["uid"]

    class FakePylsl:
        cf_float32 = 1

    exact = FakeStream()
    assert len(exact_polar_candidates([exact, exact], "ecg", FakePylsl)) == 1
    assert len(
        exact_polar_candidates(
            [exact, FakeStream(uid="different-exact-stream")], "ecg", FakePylsl
        )
    ) == 2
    assert exact_polar_candidates([], "ecg", FakePylsl) == []
    for drift in (
        {"name": "wrong"},
        {"type": "wrong"},
        {"channels": 3},
        {"rate": 200.0},
        {"format": 2},
        {"source_id": "wrong"},
    ):
        assert exact_polar_candidates([FakeStream(**drift)], "ecg", FakePylsl) == []


def parse_polar_official_server_output(output: str, role: str) -> dict[str, object]:
    payloads = [
        line.partition(POLAR_OFFICIAL_MARKER)[2]
        for line in output.splitlines()
        if POLAR_OFFICIAL_MARKER in line
    ]
    if len(payloads) != 1:
        raise ValueError(
            f"expected one official Polar marker, observed {len(payloads)}"
        )
    document = json.loads(payloads[0])
    channels = 1 if role == "ecg" else 3
    rate_hz = 130 if role == "ecg" else 200
    records = ECG_RECORDS if role == "ecg" else ACC_RECORDS
    exact = {
        "role": role,
        "channels": channels,
        "rate_hz": rate_hz,
        "records": records,
        "accepted_consumers": 1,
        "closed_consumers": 1,
    }
    required = {*exact, "discovery_requests"}
    if set(document) != required:
        raise ValueError("official Polar payload fields drifted")
    for field, expected in exact.items():
        if document[field] != expected:
            raise ValueError(f"official Polar {field} drifted from {expected}")
    if not isinstance(document["discovery_requests"], int) or document["discovery_requests"] < 1:
        raise ValueError("official broad discovery did not receive the outlet")
    return document


def expected_polar_stream(role: str) -> dict[str, object]:
    return {
        "ecg": {
            "name": ECG_NAME,
            "type": "ECG",
            "channels": 1,
            "rate": 130.0,
            "source_id": ECG_SOURCE_ID,
        },
        "acc": {
            "name": ACC_NAME,
            "type": "ACC",
            "channels": 3,
            "rate": 200.0,
            "source_id": ACC_SOURCE_ID,
        },
    }[role]


def exact_stream_matches(stream, expected: dict[str, object], pylsl) -> bool:
    return (
        stream.name() == expected["name"]
        and stream.type() == expected["type"]
        and stream.channel_count() == expected["channels"]
        and stream.nominal_srate() == expected["rate"]
        and stream.channel_format() == pylsl.cf_float32
        and stream.source_id() == expected["source_id"]
    )


def exact_polar_candidates(streams, role: str, pylsl) -> list[object]:
    matches: dict[str, object] = {}
    expected = expected_polar_stream(role)
    for stream in streams:
        if exact_stream_matches(stream, expected, pylsl):
            matches[stream.uid()] = stream
    return list(matches.values())


def resolve_exact_polar_stream(pylsl, role: str):
    matches: dict[str, object] = {}
    deadline = time.monotonic() + 10.0
    while time.monotonic() < deadline:
        for stream in exact_polar_candidates(
            pylsl.resolve_streams(wait_time=1.0), role, pylsl
        ):
            matches[stream.uid()] = stream
        if len(matches) == 1:
            break
    if len(matches) != 1:
        raise ValueError(f"expected one exact {role} candidate, observed {len(matches)}")
    return next(iter(matches.values()))


def pull_exact_chunk(inlet, records: int) -> tuple[list[list[float]], list[float]]:
    samples: list[list[float]] = []
    timestamps: list[float] = []
    deadline = time.monotonic() + 10.0
    while len(samples) < records and time.monotonic() < deadline:
        sample, timestamp = inlet.pull_sample(
            timeout=max(0.0, deadline - time.monotonic())
        )
        if timestamp:
            samples.append(sample)
            timestamps.append(timestamp)
    return samples, timestamps


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


def run_polar_official_qualification(
    interface: str, role: str, pylsl, native_sha256: str, root: Path
) -> int:
    channels = 1 if role == "ecg" else 3
    records = ECG_RECORDS if role == "ecg" else ACC_RECORDS
    rate_hz = 130 if role == "ecg" else 200
    with tempfile.TemporaryDirectory(prefix="rusty-lsl-polar-official-") as raw_temp:
        temp = Path(raw_temp)
        ready = temp / "ready.json"
        consumer_ready = temp / "consumer-ready"
        acknowledgement = temp / "ack"
        environment = os.environ.copy()
        environment.update(
            {
                "RUSTY_LSL_INTEROP_INTERFACE": interface,
                "RUSTY_LSL_INTEROP_READY_FILE": str(ready),
                "RUSTY_LSL_INTEROP_CONSUMER_READY_FILE": str(consumer_ready),
                "RUSTY_LSL_INTEROP_ACK_FILE": str(acknowledgement),
                "RUSTY_LSL_POLAR_ROLE": role,
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
                "persistent_float32_outlet_official_consumer::polar_001_single_official_consumer_qualification_server",
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
        consumer_stage = "server-readiness"
        try:
            wait_for_file(ready, process, 30.0)
            ready_document = json.loads(ready.read_text(encoding="utf-8"))
            if ready_document != {
                "schema": "rusty.lsl.polar_001.official_single_ready.v1",
                "role": role,
                "channels": channels,
                "records": records,
            }:
                raise ValueError("official Polar readiness shape drifted")
            consumer_stage = "broad-resolution"
            stream = resolve_exact_polar_stream(pylsl, role)
            inlet = pylsl.StreamInlet(stream, max_buflen=10, recover=False)
            base_timestamp = pylsl.local_clock()
            consumer_ready.write_text(repr(base_timestamp) + "\n", encoding="utf-8")
            consumer_stage = f"{role}-open"
            inlet.open_stream(timeout=10.0)
            consumer_stage = f"{role}-pull"
            samples, timestamps = pull_exact_chunk(inlet, records)
            consumer_stage = "value-validation"
            if role == "ecg":
                expected_samples = [[float(index)] for index in range(ECG_RECORDS)]
            else:
                expected_samples = [
                    [float(index), float(index + 1), float(index + 2)]
                    for index in range(ACC_RECORDS)
                ]
            if samples != expected_samples:
                raise ValueError(f"{role} values or record shape drifted")
            consumer_stage = "timestamp-validation"
            expected_timestamps = [
                base_timestamp + index / rate_hz for index in range(records)
            ]
            for actual, expected in zip(timestamps, expected_timestamps, strict=True):
                if abs(float(actual) - expected) > 1e-12:
                    raise ValueError(f"{role} source timestamp drifted")
            consumer_stage = "acknowledgement"
            acknowledgement.write_text("pass\n", encoding="utf-8")
        except Exception as error:
            consumer_error = RuntimeError(f"{consumer_stage}: {error}")
        finally:
            if inlet is not None:
                inlet.close_stream()
            try:
                return_code = process.wait(timeout=10.0)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
                raise RuntimeError("official Polar server did not terminate")
            stdout_thread.join()
            stderr_thread.join()
        output = "".join(stdout + stderr)
        if consumer_error is not None:
            raise RuntimeError(
                f"official Polar consumer failed: {consumer_error}\n"
                f"qualification server exit {return_code}:\n{output.strip()}"
            ) from consumer_error
        if return_code != 0:
            raise RuntimeError(
                f"official Polar server failed with exit {return_code}:\n{output.strip()}"
            )
        server = parse_polar_official_server_output(output, role)

    revision = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=root,
        text=True,
        encoding="utf-8",
        errors="strict",
        stdout=subprocess.PIPE,
        check=True,
    ).stdout.strip()
    result = {
        "schema": POLAR_OFFICIAL_SCHEMA,
        "revision": revision,
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
            "implementation_source_inspected": False,
            "implementation_source_copied_or_translated": False,
        },
        "scope": {
            "platform_class": "single-windows-desktop-host",
            "interface_selection": "caller-explicit-active-private-ipv4",
            "discovery": "official broad resolver with exact client-side identity matching",
            "outlet": {
                "role": role,
                "channels": channels,
                "rate_hz": rate_hz,
                "records": records,
            },
        },
        "result": {
            "exact_candidates": "pass",
            "persistent_handshakes": "pass",
            "initialization": "pass",
            "exact_float32_values": "pass",
            "exact_source_timestamps": "pass",
            "bounded_close": "pass",
            "discovery_requests": server["discovery_requests"],
        },
        "limitations": {
            "query_predicate_evaluation": False,
            "single_host": True,
            "single_platform": True,
            "cross_host": False,
            "device": False,
            "browser": False,
            "broad_liblsl_equivalence": False,
        },
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--interface", type=explicit_ipv4)
    result.add_argument("--self-test", action="store_true")
    result.add_argument("--multi-outlet-self-test", action="store_true")
    result.add_argument("--polar-role", choices=("ecg", "acc"))
    return result


def main() -> int:
    arguments = parser().parse_args()
    selected_modes = sum(
        [
            arguments.self_test,
            arguments.multi_outlet_self_test,
            arguments.polar_role is not None,
        ]
    )
    if selected_modes > 1:
        parser().error("select only one qualification or self-test mode")
    if arguments.multi_outlet_self_test:
        multi_outlet_self_test()
        print(json.dumps({"schema": MULTI_SELF_TEST_SCHEMA, "status": "pass"}, sort_keys=True))
        return 0
    if arguments.self_test:
        self_test()
        print(json.dumps({"schema": SELF_TEST_SCHEMA, "status": "pass"}, sort_keys=True))
        return 0
    if arguments.interface is None:
        parser().error("--interface is required unless a self-test is used")

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
    if arguments.polar_role is not None:
        return run_polar_official_qualification(
            arguments.interface, arguments.polar_role, pylsl, native_sha256, root
        )
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
            "diagnostic_source_inspected": True,
            "diagnostic_source_revision": LIBLSL_SOURCE_REVISION,
            "implementation_source_copied_or_translated": False,
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
