#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Bounded LSLC-001H black-box capture through documented pylsl APIs only."""

from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import os
import platform
import re
import struct
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

RUNTIME_FIELDS = (
    "created_at", "uid", "session_id", "hostname", "v4address",
    "v4data_port", "v4service_port", "v6address", "v6data_port",
    "v6service_port",
)
REPLACEMENTS = {name: f"NORMALIZED_{name.upper()}" for name in RUNTIME_FIELDS}
FORMAT_SPELLINGS = {
    "cf_float32": "float32", "cf_double64": "double64", "cf_string": "string",
    "cf_int32": "int32", "cf_int16": "int16", "cf_int8": "int8", "cf_int64": "int64",
}


class OracleFailure(Exception):
    def __init__(self, stage: str, detail: str) -> None:
        super().__init__(detail)
        self.stage = stage
        self.detail = detail


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def require(condition: bool, stage: str, detail: str) -> None:
    if not condition:
        raise OracleFailure(stage, detail)


def load_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise OracleFailure("evidence-shape", f"unable to load JSON: {error.__class__.__name__}") from error
    require(isinstance(value, dict), "evidence-shape", "JSON root is not an object")
    return value


def append_failure(path: Path, stage: str, detail: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    record = {
        "schema": "rusty.lsl.oracle.failure.v1",
        "recorded_at_utc": datetime.now(timezone.utc).isoformat(),
        "stage": stage,
        "classification": "setup-failure" if stage.startswith(("python-", "wheel-", "pylsl-", "native-library")) else "provider-failure",
        "detail": detail,
    }
    with path.open("a", encoding="utf-8", newline="\n") as stream:
        stream.write(json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n")


def loaded_dll_path() -> Path:
    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel32.GetModuleHandleW.argtypes = [ctypes.c_wchar_p]
    kernel32.GetModuleHandleW.restype = ctypes.c_void_p
    kernel32.GetModuleFileNameW.argtypes = [ctypes.c_void_p, ctypes.c_wchar_p, ctypes.c_uint32]
    kernel32.GetModuleFileNameW.restype = ctypes.c_uint32
    handle = kernel32.GetModuleHandleW("lsl.dll")
    require(bool(handle), "native-library-presence", "lsl.dll is not loaded")
    buffer = ctypes.create_unicode_buffer(32768)
    length = kernel32.GetModuleFileNameW(handle, buffer, len(buffer))
    require(length > 0 and length < len(buffer), "native-library-presence", "loaded lsl.dll path is unavailable")
    return Path(buffer.value).resolve()


def validate_case_set(case_set: dict[str, Any]) -> None:
    require(case_set.get("schema") == "rusty.lsl.compatibility.stream_info_xml_black_box_cases.v1", "evidence-shape", "case schema drifted")
    bounds = case_set.get("bounds")
    require(isinstance(bounds, dict), "evidence-shape", "bounds missing")
    cases = case_set.get("positive_cases")
    require(isinstance(cases, list) and 0 < len(cases) <= bounds.get("max_cases", 0), "evidence-shape", "case count is invalid")
    require(bounds.get("captures_per_case") == 2, "evidence-shape", "exactly two captures are required")
    require({case.get("channel_format_symbol") for case in cases if isinstance(case, dict)} == set(FORMAT_SPELLINGS), "evidence-shape", "format coverage drifted")
    for case in cases:
        require(isinstance(case, dict), "evidence-shape", "case is not an object")
        for key in ("case_id", "name", "type", "channel_count", "nominal_srate", "channel_format_symbol", "source_id", "description"):
            require(key in case, "evidence-shape", f"case field missing: {key}")
        require(isinstance(case["channel_count"], int) and case["channel_count"] > 0, "evidence-shape", "channel count is invalid")
        require(isinstance(case["nominal_srate"], (int, float)) and case["nominal_srate"] >= 0, "evidence-shape", "nominal rate is invalid")
        require(isinstance(case["description"], list) and len(case["description"]) <= bounds["max_desc_nodes"], "evidence-shape", "description bound exceeded")
        for value in (case["name"], case["type"], case["source_id"]):
            require(isinstance(value, str) and len(value) <= bounds["max_text_code_points"], "evidence-shape", "core text bound exceeded")
        for entry in case["description"]:
            require(isinstance(entry, dict) and set(entry) == {"path", "name", "value"}, "evidence-shape", "description entry shape drifted")
            require(isinstance(entry["path"], list) and len(entry["path"]) <= bounds["max_desc_depth"], "evidence-shape", "description depth exceeded")


def normalize(raw: bytes) -> tuple[bytes, list[dict[str, Any]]]:
    operations: list[dict[str, Any]] = []
    matches: list[tuple[int, int, bytes, str]] = []
    for field in RUNTIME_FIELDS:
        pattern = re.compile(rb"<" + field.encode("ascii") + rb">([^<]*)</" + field.encode("ascii") + rb">")
        found = list(pattern.finditer(raw))
        require(len(found) == 1, "normalization", f"runtime field occurrence drifted: {field}")
        match = found[0]
        replacement = REPLACEMENTS[field].encode("ascii")
        matches.append((match.start(1), match.end(1), replacement, field))
    normalized = bytearray(raw)
    for start, end, replacement, field in sorted(matches, reverse=True):
        original_length = end - start
        normalized[start:end] = replacement
        operations.append({
            "field": field,
            "raw_start_byte": start,
            "raw_end_byte_exclusive": end,
            "raw_length_bytes": original_length,
            "replacement_utf8": replacement.decode("ascii"),
            "reason": "runtime-or-machine-specific-character-data-outside-observed-dimensions",
        })
    operations.reverse()
    return bytes(normalized), operations


def safe_public_scan(data: bytes) -> None:
    require(len(data) <= 16384, "capture-output-bound", "normalized XML exceeds public bound")
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError as error:
        raise OracleFailure("public-boundary", "normalized XML is not UTF-8") from error
    lowered = text.lower()
    prohibited_literals = [os.environ.get("USERNAME", ""), os.environ.get("COMPUTERNAME", ""), str(Path.home()), str(sys.prefix)]
    for value in prohibited_literals:
        if value:
            require(value.lower() not in lowered, "public-boundary", "normalized XML contains a local identity or path")
    for pattern in (
        r"[a-zA-Z]:[\\/]", r"\\\\[^\\\s]+[\\/]", r"(?:\d{1,3}\.){3}\d{1,3}",
        r"\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b",
        r"(?i)(?:password|credential|secret|token|private[_-]?key)\s*[:=]",
    ):
        require(re.search(pattern, text) is None, "public-boundary", "normalized XML contains prohibited public content")


def element_text(xml: bytes, name: str) -> str:
    match = re.search(rb"<" + name.encode("ascii") + rb">([^<]*)</" + name.encode("ascii") + rb">", xml)
    require(match is not None, "evidence-shape", f"missing simple element: {name}")
    return match.group(1).decode("utf-8")


def direct_info_children(xml: bytes) -> list[str]:
    tokens = re.finditer(rb"<(/?)([A-Za-z_][A-Za-z0-9_]*)(?:\s*/)?>", xml)
    stack: list[str] = []
    children: list[str] = []
    for token in tokens:
        closing = token.group(1) == b"/"
        name = token.group(2).decode("ascii")
        self_closing = token.group(0).rstrip().endswith(b"/>")
        if closing:
            require(stack and stack[-1] == name, "evidence-shape", "markup nesting drifted")
            stack.pop()
        else:
            if stack == ["info"]:
                children.append(name)
            if not self_closing:
                stack.append(name)
    require(not stack, "evidence-shape", "markup is unbalanced")
    return children


def summarize(xml: bytes, case: dict[str, Any]) -> dict[str, Any]:
    children = direct_info_children(xml)
    desc_tokens = re.findall(rb"<desc(?:\s*/)?>", xml)
    require(len(desc_tokens) == 1, "evidence-shape", "desc occurrence drifted")
    whitespace = re.findall(rb">([\x09\x0a\x0d\x20]+)<", xml)
    return {
        "direct_info_child_order": children,
        "whitespace_between_markup": "present" if whitespace else "absent",
        "whitespace_run_sha256": sha256(b"\x00".join(whitespace)),
        "empty_desc_form": desc_tokens[0].decode("ascii") if not case["description"] else None,
        "channel_count_spelling": element_text(xml, "channel_count"),
        "nominal_srate_spelling": element_text(xml, "nominal_srate"),
        "channel_format_spelling": element_text(xml, "channel_format"),
        "expected_channel_format_spelling": FORMAT_SPELLINGS[case["channel_format_symbol"]],
        "desc_direct_child_index": children.index("desc"),
        "character_data_inputs": {
            "name": case["name"], "type": case["type"], "source_id": case["source_id"],
            "description_values": [entry["value"] for entry in case["description"]],
        },
        "character_data_spellings": {
            "name": element_text(xml, "name"),
            "type": element_text(xml, "type"),
            "source_id": element_text(xml, "source_id"),
            "description_values": [element_text(xml, entry["name"]) for entry in case["description"]],
        },
        "description_leaf_order": [entry["name"] for entry in case["description"]],
    }


def append_description(info: Any, entries: list[dict[str, Any]]) -> None:
    desc = info.desc()
    cache: dict[tuple[str, ...], Any] = {(): desc}
    for entry in entries:
        current: tuple[str, ...] = ()
        for segment in entry["path"]:
            next_path = current + (segment,)
            if next_path not in cache:
                cache[next_path] = cache[current].append_child(segment)
            current = next_path
        cache[current].append_child_value(entry["name"], entry["value"])


def capture(args: argparse.Namespace) -> int:
    output = Path(args.output_dir).resolve()
    output.mkdir(parents=True, exist_ok=True)
    history = Path(args.failure_history).resolve()
    started = datetime.now(timezone.utc)
    try:
        require(os.name == "nt", "python-architecture", "oracle requires Windows")
        require(struct.calcsize("P") * 8 == 64 and platform.machine().upper() == "AMD64", "python-architecture", "oracle requires AMD64 Python")
        require(platform.python_version() == args.expected_python_version, "python-version", "Python version drifted from setup probe")
        case_path = Path(args.case_manifest).resolve()
        case_set = load_object(case_path)
        validate_case_set(case_set)

        import pylsl

        require(pylsl.__version__ == args.expected_pylsl_version, "pylsl-version", "pylsl version drifted")
        library_version = pylsl.library_version()
        require(library_version == args.expected_library_version, "native-library-version", "public liblsl version drifted")
        dll = Path(args.native_dll).resolve()
        require(dll.is_file(), "native-library-presence", "native library is missing")
        require(loaded_dll_path() == dll, "native-library-presence", "loaded native library path differs from verified artifact")
        dll_sha = sha256(dll.read_bytes())
        require(dll_sha == args.expected_native_sha256, "native-library-digest", "native library digest drifted")

        observations: list[dict[str, Any]] = []
        for case in case_set["positive_cases"]:
            symbol = case["channel_format_symbol"]
            require(hasattr(pylsl, symbol), "evidence-shape", f"public format symbol is missing: {symbol}")
            info = pylsl.StreamInfo(
                case["name"], case["type"], case["channel_count"],
                case["nominal_srate"], getattr(pylsl, symbol), case["source_id"],
            )
            append_description(info, case["description"])
            raw_captures = [info.as_xml().encode("utf-8"), info.as_xml().encode("utf-8")]
            for raw in raw_captures:
                require(len(raw) <= case_set["bounds"]["max_xml_bytes_per_capture"], "capture-output-bound", f"raw XML exceeds bound: {case['case_id']}")
            require(raw_captures[0] == raw_captures[1], "capture-repeat", f"raw captures differ: {case['case_id']}")
            normalized: list[bytes] = []
            operation_sets: list[list[dict[str, Any]]] = []
            raw_refs: list[dict[str, Any]] = []
            for index, raw in enumerate(raw_captures, start=1):
                raw_path = output / "raw" / f"{case['case_id']}-capture-{index}.xml"
                raw_path.parent.mkdir(parents=True, exist_ok=True)
                raw_path.write_bytes(raw)
                public, operations = normalize(raw)
                safe_public_scan(public)
                normalized.append(public)
                operation_sets.append(operations)
                raw_refs.append({"capture_index": index, "sha256": sha256(raw), "byte_length": len(raw), "external_file_name": raw_path.name})
            require(normalized[0] == normalized[1], "capture-repeat", f"normalized captures differ: {case['case_id']}")
            require(operation_sets[0] == operation_sets[1], "capture-repeat", f"normalization operations differ: {case['case_id']}")
            observations.append({
                "case_id": case["case_id"],
                "classification": "accepted",
                "captures": raw_refs,
                "repeat_comparison": "raw-and-normalized-byte-identical",
                "normalization_operations": operation_sets[0],
                "public_xml_utf8": normalized[0].decode("utf-8"),
                "public_xml_sha256": sha256(normalized[0]),
                "public_xml_byte_length": len(normalized[0]),
                "observed_dimensions": summarize(normalized[0], case),
            })

        record = {
            "schema": "rusty.lsl.oracle.external_capture.v1",
            "unit_id": case_set["unit_id"],
            "started_at_utc": started.isoformat(),
            "finished_at_utc": datetime.now(timezone.utc).isoformat(),
            "classification": "accepted",
            "environment": {
                "os_family": "Windows", "architecture": "AMD64", "python_implementation": platform.python_implementation(),
                "python_version": platform.python_version(), "python_bits": 64,
            },
            "oracle": {
                "pylsl_version": pylsl.__version__, "liblsl_public_library_version": library_version,
                "native_dll_sha256": dll_sha,
            },
            "case_manifest_sha256": sha256(case_path.read_bytes()),
            "observations": observations,
        }
        (output / "capture-record.json").write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
        return 0
    except OracleFailure as error:
        append_failure(history, error.stage, error.detail)
        print(json.dumps({"stage": error.stage, "classification": "failed"}, separators=(",", ":")), file=sys.stderr)
        return 2
    except Exception as error:
        append_failure(history, "oracle-process-exit", error.__class__.__name__)
        print(json.dumps({"stage": "oracle-process-exit", "classification": "failed"}, separators=(",", ":")), file=sys.stderr)
        return 3


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--case-manifest", required=True)
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--native-dll", required=True)
    parser.add_argument("--failure-history", required=True)
    parser.add_argument("--expected-python-version", required=True)
    parser.add_argument("--expected-pylsl-version", required=True)
    parser.add_argument("--expected-library-version", required=True, type=int)
    parser.add_argument("--expected-native-sha256", required=True)
    return parser.parse_args()


if __name__ == "__main__":
    raise SystemExit(capture(parse_args()))
