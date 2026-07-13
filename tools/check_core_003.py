#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-003 stream-descriptor contracts."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
DESCRIPTOR_PATH = SOURCE_ROOT / "stream_descriptor.rs"
FIXTURES = ROOT / "fixtures" / "compatibility"
OVERLAY_PATH = FIXTURES / "core-003-contract-results.json"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-core-003-stream-descriptor-contracts.json"
EXPECTED_TESTS = {
    "bounded-stream-descriptor": {
        "positive": {
            "core_003_descriptor_exact_limits_preserve_all_text_and_values",
            "core_003_optional_text_is_explicit_and_empty_opaque_text_is_preserved",
        },
        "damaged": {
            "core_003_empty_name_has_stable_typed_error",
            "core_003_one_past_each_text_limit_has_stable_typed_error",
            "core_003_zero_and_one_past_channels_have_stable_typed_errors",
            "core_003_zero_limits_reject_in_argument_order",
        },
    },
    "nominal-sample-rate": {
        "positive": {"core_003_regular_rate_preserves_bits_and_irregular_is_explicit"},
        "damaged": {"core_003_invalid_regular_rates_have_stable_typed_errors"},
    },
    "channel-format-values": {
        "positive": {"core_003_all_seven_channel_formats_are_independent_data_values"},
        "damaged": set(),
    },
}
EXPECTED_FORMATS = {"Float32", "Double64", "String", "Int32", "Int16", "Int8", "Int64"}
EXPECTED_EFFECTS = {
    "activities", "assets", "commands", "inputs", "markers", "native_libraries",
    "permissions", "queries", "routes", "scenes", "services", "shaders", "streams", "tools",
}


def require(condition: bool, message: str) -> None:
    """Raise a stable validation error when an invariant is false."""
    if not condition:
        raise ValueError(message)


def load_object(path: Path) -> dict[str, Any]:
    """Load one required JSON object."""
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def source_text() -> str:
    """Return the complete Rust source surface in stable path order."""
    return "\n".join(path.read_text(encoding="utf-8") for path in sorted(SOURCE_ROOT.glob("*.rs")))


def validate_overlay_and_tests() -> None:
    """Bind exact positive/damaged tests without promoting STRM-000."""
    overlay = load_object(OVERLAY_PATH)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "CORE-003 overlay schema drifted")
    require(overlay.get("overlay_id") == "core-003-stream-descriptor-contracts", "CORE-003 overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-core-003-stream-descriptor-contracts", "CORE-003 unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "CORE-003 evidence level is inaccurate")
    require(overlay.get("implementation_status") == "bounded-local-contracts", "CORE-003 status is inaccurate")

    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-003 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog binding drifted")
    require(binding.get("case_id") == "semantic-observation-not-authority", "CORE-003 authority binding drifted")
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    interpretation = binding.get("interpretation")
    require(isinstance(interpretation, str) and "specification-only" in interpretation and "local Rust" in interpretation,
            "CORE-003 baseline limitation is incomplete")

    results = overlay.get("contract_results")
    require(isinstance(results, list), "CORE-003 contract results must be a list")
    by_id = {result.get("contract_id"): result for result in results if isinstance(result, dict)}
    require(set(by_id) == set(EXPECTED_TESTS), "CORE-003 contract-result identities drifted")
    declared_tests = set(re.findall(r"(?m)^\s*fn\s+(core_003_[a-z0-9_]+)\s*\(", source_text()))
    require(declared_tests == set().union(*(entry["positive"] | entry["damaged"] for entry in EXPECTED_TESTS.values())),
            "CORE-003 Rust test inventory drifted")
    for contract_id, expected in EXPECTED_TESTS.items():
        result = by_id[contract_id]
        require(result.get("result") == "implemented-local-contract", f"{contract_id} status is inaccurate")
        require(set(result.get("positive_tests", [])) == expected["positive"], f"{contract_id} positive coverage drifted")
        require(set(result.get("damaged_tests", [])) == expected["damaged"], f"{contract_id} damaged coverage drifted")

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-003 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-003 origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "CORE-003 license drifted")
    require(provenance.get("implementation_inputs") == [], "CORE-003 has prohibited implementation inputs")
    limitations = set(overlay.get("does_not_prove", []))
    required = {
        "XML parsing, serialization, metadata-tree mutation, or query matching",
        "discovery, resolution, recovery, or runtime identity",
        "source identifier uniqueness, authorization, routing, permission, admission, or Morphospace authority",
        "clock reading, rate measurement, scheduling, enforcement, interpolation, or rate derivation",
        "channel encoding, decoding, byte sizing, sample conversion, or wire numeric formats",
        "LSL protocol behavior", "sample or descriptor transport", "wire compatibility",
        "runtime support", "ecosystem compatibility", "official liblsl behavior",
    }
    require(required <= limitations, "CORE-003 limitations are incomplete")


def validate_historical_baseline() -> None:
    """Reject compatibility promotion or invented STRM-000 measurements."""
    catalog = load_object(CATALOG_PATH)
    require(catalog.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog identity drifted")
    require(catalog.get("evidence_level") == "specification-only", "STRM-000 evidence level drifted")
    cases = catalog.get("cases")
    require(isinstance(cases, list) and cases, "STRM-000 cases are missing")
    for case in cases:
        require(isinstance(case, dict), "STRM-000 case must be an object")
        case_id = case.get("case_id")
        require(case.get("current_result") == "not-implemented", f"{case_id} promoted STRM-000 history")
        measured = case.get("measured_result")
        require(isinstance(measured, dict), f"{case_id} measured-result role is missing")
        require(measured.get("status") == "not-implemented" and measured.get("observation") is None,
                f"{case_id} invents a measured STRM-000 result")


def validate_source_contract() -> None:
    """Require descriptor invariants and reject XML/runtime/authority surfaces."""
    source = source_text()
    descriptor = DESCRIPTOR_PATH.read_text(encoding="utf-8")
    for required in (
        "pub struct StreamDescriptorLimits", "max_name_code_points: usize",
        "max_content_type_code_points: usize", "max_source_id_code_points: usize",
        "max_channels: usize", "pub struct StreamDescriptor", "name: String",
        "content_type: Option<String>", "source_id: Option<String>", "channel_count: usize",
        "pub enum NominalSampleRate", "Irregular", "RegularHz(RegularSampleRate)",
        "pub struct RegularSampleRate(f64)", "pub enum ChannelFormat", "EmptyName",
        "TextLimitExceeded", "ChannelCountOutOfBounds", ".chars().count()",
    ):
        require(required in descriptor, f"CORE-003 source invariant is missing: {required}")
    require("#![forbid(unsafe_code)]" in source, "unsafe Rust is not forbidden")
    require("#![deny(missing_docs)]" in source, "missing documentation is not denied")

    format_match = re.search(r"pub enum ChannelFormat\s*\{(?P<body>.*?)\n\}", descriptor, re.DOTALL)
    require(format_match is not None, "ChannelFormat declaration is missing")
    body = format_match.group("body")
    variants = set(re.findall(r"(?m)^\s{4}([A-Z][A-Za-z0-9]*)\s*,\s*$", body))
    require(variants == EXPECTED_FORMATS, "ChannelFormat must contain exactly seven named data-only variants")
    require("=" not in body and "Undefined" not in body, "ChannelFormat gained a numeric or undefined transfer form")
    prelude = descriptor[: format_match.start()]
    require("#[repr(" not in prelude[-200:], "ChannelFormat gained a representation discriminant")

    prohibited_imports = {
        "external ABI": re.compile(r'\bextern\s+"'), "network API": re.compile(r"\b(?:std|core)::net\b"),
        "thread API": re.compile(r"\bstd::thread\b"), "process API": re.compile(r"\bstd::process\b"),
        "clock API": re.compile(r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b"),
        "FFI API": re.compile(r"\b(?:std|core)::ffi\b"), "filesystem API": re.compile(r"\bstd::fs\b"),
        "environment API": re.compile(r"\bstd::env\b"),
        "I/O API": re.compile(r"\bstd::io\b"), "synchronization API": re.compile(r"\bstd::sync\b"),
        "unsafe block": re.compile(r"(?m)^\s*unsafe\s*\{"),
    }
    for label, pattern in prohibited_imports.items():
        require(not pattern.search(source), f"CORE-003 opened a prohibited {label}")

    declarations = "\n".join(line for line in descriptor.splitlines()
                             if re.match(r"\s*(?:pub\s+|[a-z_][a-z0-9_]*:\s)", line))
    prohibited_surface = re.compile(
        r"(?i)\b(?:xml|xpath|query|metadata.?tree|discover|resolv|recover|socket|network|inlet|outlet|"
        r"transport|buffer|queue|thread|ffi|adapter|authority|admission|permission|route|runtime|"
        r"created.?at|version|outlet.?uid|session|hostname|address|port|encode|decode|byte.?size|convert)\w*\b"
    )
    match = prohibited_surface.search(declarations)
    require(match is None, f"CORE-003 opened prohibited public surface: {match.group(0) if match else ''}")


def validate_inert_closure_and_instructions() -> None:
    """Reject dependency/activation drift and require instruction completion."""
    result = subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    packages = json.loads(result.stdout).get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    require(packages[0].get("dependencies") == [], "CORE-003 permits no Cargo dependency")
    require(packages[0].get("features") == {}, "CORE-003 permits no Cargo feature")
    require(packages[0].get("publish") == [], "CORE-003 package must remain unpublished")

    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock default activation drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect-union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union must remain empty")

    unit = load_object(UNIT_PATH)
    require(unit.get("status") in {"active", "validating", "accepted"},
            "CORE-003 is outside its implementation/validation lifecycle")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list), "CORE-003 instruction surfaces are missing")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces if isinstance(surface, dict)}
    require(statuses.get("AGENTS.md") == "complete", "AGENTS.md instruction row is not complete")
    require(statuses.get("README.md") == "complete", "README.md instruction row is not complete")
    require(statuses.get("<skills-root>/system-engineering/SKILL.md") == "complete",
            "system-engineering skill review is not complete")


def validate_docs_and_license() -> None:
    """Require exact public limitations and AGPL provenance for new artifacts."""
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    normalized = " ".join("\n".join(path.read_text(encoding="utf-8") for path in paths).split())
    for phrase in (
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
        "nonempty stream name", "optional bounded opaque text", "Unicode scalar",
        "source correlation", "finite positive regular", "exactly seven", "local Rust contract tests",
    ):
        require(phrase in normalized, f"required CORE-003 limitation is missing: {phrase}")
    require("Project-owned source and documentation are licensed `AGPL-3.0-or-later`." in
            (ROOT / "docs" / "PROVENANCE.md").read_text(encoding="utf-8"),
            "public provenance license statement drifted")
    require('license = "AGPL-3.0-or-later"' in (ROOT / "Cargo.toml").read_text(encoding="utf-8"),
            "workspace license expression drifted")
    require("license.workspace = true" in (ROOT / "crates" / "rusty-lsl" / "Cargo.toml").read_text(encoding="utf-8"),
            "crate license inheritance drifted")
    for path in [DESCRIPTOR_PATH, ROOT / "tools" / "check_core_003.py", ROOT / "tools" / "check_core_003.ps1"]:
        require("SPDX-License-Identifier: AGPL-3.0-or-later" in path.read_text(encoding="utf-8"),
                f"license drift in {path.relative_to(ROOT)}")


def main() -> int:
    """Run all deterministic CORE-003 checks."""
    validate_overlay_and_tests()
    validate_historical_baseline()
    validate_source_contract()
    validate_inert_closure_and_instructions()
    validate_docs_and_license()
    print("CORE-003 stream descriptor contract checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
