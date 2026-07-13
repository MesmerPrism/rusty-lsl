# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

from __future__ import annotations

import hashlib
import json
import re
import runpy
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_static_fields.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
CORE_008 = ROOT / "fixtures/compatibility/core-008-contract-results.json"
CASES = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json"
OBSERVATIONS = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json"
PROVENANCE = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-provenance.json"
DRIVER_PS1 = ROOT / "tools/oracle/Invoke-Lslc001hOracle.ps1"
DRIVER_PY = ROOT / "tools/oracle/lslc_001h_capture.py"
HISTORICAL_CHECKER = ROOT / "tools/check_lslc_001h.py"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001k-stream-info-static-field-results.json"
UNIT = ROOT / "morphospace/iteration-units/rlsl-lslc-001k-stream-info-static-fields.json"

BOUND_SHA256 = {
    CORE_008: "8685b40dc5b3bb5ff68e3daf1c0d0be9daf4746aa52e6dad964eb2e7572f4d23",
    CASES: "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17",
    OBSERVATIONS: "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e",
    PROVENANCE: "7f1c7c80a4c749fda6303fa22cb54b0378b1894996b434b45e2969459f7f63c2",
}
DRIVER_SHA256 = {
    DRIVER_PS1: "edf07ba073c7947558ac32c38e608fbbd3d344c715b88baa37512cfa5cd37e0f",
    DRIVER_PY: "0e064fcd78f4352268cf37e6be8edd1510bfbfa2cbe029c9ecce83d8a9a25b40",
}
HISTORICAL_CHECKER_SHA256 = "5ece4b44aed40ff1c657ff7d2451473ada79f586f70aefe675a5b1aa5fcf55c8"
ROLE_ORDER = ["name", "type", "channel_count", "channel_format", "source_id", "nominal_srate"]
FORMAT_MAP = {
    "cf_float32": ("Float32", "float32"),
    "cf_double64": ("Double64", "double64"),
    "cf_string": ("String", "string"),
    "cf_int32": ("Int32", "int32"),
    "cf_int16": ("Int16", "int16"),
    "cf_int8": ("Int8", "int8"),
    "cf_int64": ("Int64", "int64"),
}
TESTS = {
    "lslc_001k_fixed_six_role_order_is_exact",
    "lslc_001k_option_forms_remain_distinct_from_effective_empty_values",
    "lslc_001k_all_channel_formats_map_exactly_and_totally",
    "lslc_001k_seven_observed_semantic_cases_execute_exactly",
    "lslc_001k_original_and_effective_rate_views_remain_separate",
    "lslc_001k_borrowing_preserves_identity_and_generic_metadata",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def canonical_lf_source(source: bytes, label: str) -> bytes:
    without_crlf = source.replace(b"\r\n", b"")
    require(b"\r" not in without_crlf, f"lone carriage return in bound driver: {label}")
    if b"\r\n" in source:
        require(b"\n" not in without_crlf, f"mixed LF/CRLF bound driver: {label}")
        return source.replace(b"\r\n", b"\n")
    return source


def validate_preserved_black_box_contracts() -> None:
    checker_source = canonical_lf_source(
        HISTORICAL_CHECKER.read_bytes(), HISTORICAL_CHECKER.name,
    )
    require(hashlib.sha256(checker_source).hexdigest() == HISTORICAL_CHECKER_SHA256,
            "accepted LSLC-001H/J checker changed")
    historical = runpy.run_path(str(HISTORICAL_CHECKER))
    historical["validate_history_and_corpus"]()
    cases = historical["validate_cases"]()
    historical["validate_observations"](cases)
    historical["validate_provenance_and_driver"]()


def validate_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = LIB.read_text(encoding="utf-8")
    require("mod stream_info_static_fields;" in lib, "private module route missing")
    require("pub mod stream_info_static_fields" not in lib, "module became public")
    require(re.search(
        r"pub struct StreamInfoStaticFields<'a>\s*\{\s*definition:\s*&'a StreamDefinition,\s*\}",
        implementation,
    ) is not None, "borrowed view must contain exactly one private definition reference")
    for marker in (
        "pub const ORDER: [Self; 6]",
        "pub fn content_type(&self) -> Option<&'a str>",
        "pub fn effective_type(&self) -> &'a str",
        "pub fn source_id(&self) -> Option<&'a str>",
        "pub fn effective_source_id(&self) -> &'a str",
        "pub const fn nominal_sample_rate(&self) -> NominalSampleRate",
        "pub const fn effective_nominal_srate(&self) -> f64",
        "NominalSampleRate::Irregular => 0.0",
        "pub const fn extended_metadata(&self) -> &'a MetadataTree",
    ):
        require(marker in implementation, f"static projection invariant missing: {marker}")
    for variant, spelling in FORMAT_MAP.values():
        require(
            f'ChannelFormat::{variant} => "{spelling}"' in implementation,
            f"channel-format mapping missing: {variant}",
        )
    for forbidden in ("String", "Vec<", "format!", ".clone(", "to_owned(", "to_string("):
        if forbidden == "String":
            require("String" not in re.search(
                r"pub struct StreamInfoStaticFields<'a>\s*\{(?P<body>.*?)\}",
                implementation,
                re.S,
            ).group("body"), "borrowed view acquired owned text")
        else:
            require(forbidden not in implementation, f"allocation surface opened: {forbidden}")
    tests = set(re.findall(r"fn (lslc_001k_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused Rust test inventory drifted")


def validate_overlay() -> None:
    for path, expected in BOUND_SHA256.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == expected,
                f"accepted artifact changed: {path.name}")
    for path, expected in DRIVER_SHA256.items():
        canonical = canonical_lf_source(path.read_bytes(), path.name)
        require(hashlib.sha256(canonical).hexdigest() == expected,
                f"accepted canonical-LF driver changed: {path.name}")
    cases = load(CASES)
    observations = load(OBSERVATIONS)
    overlay = load(OVERLAY)
    bindings = overlay.get("accepted_artifact_bindings", {})
    require(bindings.get("stream_definition_contract", {}).get("sha256") ==
            BOUND_SHA256[CORE_008], "CORE-008 binding drifted")
    require(observations.get("candidate_result") == {"status": "not-observed", "evidence": None},
            "LSLC-001H full-document candidate result changed")
    require(overlay.get("static_role_order") == ROLE_ORDER, "static role order drifted")
    results = overlay.get("candidate_semantic_results", [])
    inputs = cases.get("positive_cases", [])
    require(len(results) == len(inputs) == 7, "exact seven-case semantic matrix required")
    require([row.get("case_id") for row in results] == [row.get("case_id") for row in inputs],
            "semantic result case order drifted")
    observed_ids = [row.get("case_id") for row in observations.get("observations", [])]
    require(observed_ids == [row.get("case_id") for row in inputs],
            "accepted observation case binding drifted")
    for source, result in zip(inputs, results):
        fields = result.get("fields", {})
        variant, spelling = FORMAT_MAP[source["channel_format_symbol"]]
        require(fields.get("name") == source["name"], f"name mismatch: {source['case_id']}")
        require(fields.get("original_type") == {"form": "present", "value": source["type"]},
                f"type form mismatch: {source['case_id']}")
        require(fields.get("effective_type") == source["type"], f"effective type mismatch: {source['case_id']}")
        require(fields.get("channel_count") == source["channel_count"], f"count mismatch: {source['case_id']}")
        require(fields.get("original_channel_format") == variant, f"format mismatch: {source['case_id']}")
        require(fields.get("channel_format_spelling") == spelling, f"spelling mismatch: {source['case_id']}")
        require(fields.get("original_source_id") == {"form": "present", "value": source["source_id"]},
                f"source form mismatch: {source['case_id']}")
        require(fields.get("effective_source_id") == source["source_id"], f"effective source mismatch: {source['case_id']}")
        expected_rate = ({"form": "irregular"} if source["nominal_srate"] == 0.0 else
                         {"form": "regular_hz", "value": source["nominal_srate"]})
        require(fields.get("original_nominal_srate") == expected_rate,
                f"rate form mismatch: {source['case_id']}")
        require(fields.get("effective_nominal_srate") == source["nominal_srate"],
                f"effective rate mismatch: {source['case_id']}")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "implementation source inputs entered")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_static_fields::tests::lslc_001k_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("6 passed" in result.stdout, "focused LSLC-001K tests did not all pass")
    unit = load(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit state invalid")
    require(all(row.get("status") == "complete" for row in unit.get("instruction_surfaces", [])),
            "instruction review incomplete")
    docs = {
        "AGENTS.md": "check_lslc_001k.ps1",
        "README.md": "StreamInfoStaticFields",
        "docs/ARCHITECTURE.md": "stream_info_static_fields",
        "docs/COMPATIBILITY.md": "LSLC-001K",
        "docs/CORPUS.md": "LSLC-001K",
        "docs/PROVENANCE.md": "lslc-001k-stream-info-static-field-results.json",
        "docs/VALIDATION.md": "check_lslc_001k.ps1",
        "morphospace/README.md": "rlsl-lslc-001k-stream-info-static-fields",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"),
                f"documentation route missing: {path}")
    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout)
    package = metadata["packages"][0]
    require(len(metadata["packages"]) == 1 and package["dependencies"] == [],
            "dependency closure drifted")
    require(package["features"] == {} and package["publish"] == [],
            "feature/publication closure drifted")
    lock = load(ROOT / "morphospace/feature.lock.json")
    require(lock.get("features") == [] and lock.get("selected_features") == [],
            "feature lock drifted")
    require(all(not value for value in lock.get("effect_union", {}).values()),
            "effect closure is not inert")


def main() -> int:
    validate_source()
    validate_preserved_black_box_contracts()
    validate_overlay()
    validate_execution_docs_and_closure()
    print("LSLC-001K borrowed static stream-info field checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
