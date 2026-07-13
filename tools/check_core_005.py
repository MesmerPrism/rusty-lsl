#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-005 descriptor/sample binding."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
BINDING_PATH = SOURCE_ROOT / "descriptor_sample.rs"
FIXTURES = ROOT / "fixtures" / "compatibility"
OVERLAY_PATH = FIXTURES / "core-005-contract-results.json"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-core-005-descriptor-sample-binding.json"
CONTRACT_ID = "descriptor-sample-shape-and-format-binding"
POSITIVE_TESTS = {
    "core_005_all_seven_exact_format_mappings_bind",
    "core_005_float_bits_and_channel_order_are_preserved",
    "core_005_integer_edges_and_channel_order_are_preserved",
    "core_005_string_exact_unicode_scalar_limit_and_empty_value_preserved",
}
DAMAGED_TESTS = {
    "core_005_descriptor_sample_channel_mismatch_has_stable_error",
    "core_005_each_input_family_has_stable_format_mismatch",
    "core_005_first_one_past_string_channel_has_stable_indexed_error",
    "core_005_validation_order_is_format_then_channel_then_string_bound",
    "core_005_zero_string_limit_has_stable_error",
}
EXPECTED_MAPPINGS = {
    "Float32": "f32",
    "Double64": "f64",
    "String": "String",
    "Int32": "i32",
    "Int16": "i16",
    "Int8": "i8",
    "Int64": "i64",
}
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
    """Bind exact positive and damaged tests without promoting STRM-000."""
    overlay = load_object(OVERLAY_PATH)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "CORE-005 overlay schema drifted")
    require(overlay.get("overlay_id") == "core-005-descriptor-sample-binding", "CORE-005 overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-core-005-descriptor-sample-binding", "CORE-005 unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "CORE-005 evidence level is inaccurate")
    require(overlay.get("implementation_status") == "bounded-local-contracts", "CORE-005 status is inaccurate")

    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-005 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog binding drifted")
    require(binding.get("case_id") == "contract-sample-shape", "CORE-005 sample-shape binding drifted")
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    interpretation = binding.get("interpretation")
    require(
        isinstance(interpretation, str)
        and "specification-only" in interpretation
        and "local Rust" in interpretation,
        "CORE-005 baseline limitation is incomplete",
    )

    results = overlay.get("contract_results")
    require(isinstance(results, list) and len(results) == 1, "CORE-005 must have one contract result")
    result = results[0]
    require(isinstance(result, dict) and result.get("contract_id") == CONTRACT_ID, "CORE-005 contract identity drifted")
    require(result.get("result") == "implemented-local-contract", "CORE-005 result is inaccurate")
    require(set(result.get("positive_tests", [])) == POSITIVE_TESTS, "CORE-005 positive coverage drifted")
    require(set(result.get("damaged_tests", [])) == DAMAGED_TESTS, "CORE-005 damaged coverage drifted")
    declared_tests = set(re.findall(r"(?m)^\s*fn\s+(core_005_[a-z0-9_]+)\s*\(", source_text()))
    require(declared_tests == POSITIVE_TESTS | DAMAGED_TESTS, "CORE-005 Rust test inventory drifted")

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-005 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-005 origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "CORE-005 license drifted")
    require(provenance.get("implementation_inputs") == [], "CORE-005 has prohibited implementation inputs")
    required_limits = {
        "numeric or string conversion, coercion, casting, parsing, formatting, or normalization",
        "byte sizing, encoding, decoding, endianness, memory layout, or wire mapping",
        "LSL protocol behavior",
        "sample or descriptor transport",
        "discovery, buffering, clocks, scheduling, recovery, providers, or runtime action",
        "source identity, authorization, admission, routing, permission, or Morphospace authority",
        "wire compatibility", "runtime support", "ecosystem compatibility", "official liblsl behavior",
    }
    require(required_limits <= set(overlay.get("does_not_prove", [])), "CORE-005 limitations are incomplete")


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
        require(
            measured.get("status") == "not-implemented" and measured.get("observation") is None,
            f"{case_id} invents a measured STRM-000 result",
        )


def validate_exact_input_family(binding: str) -> None:
    """Require the closed seven-way Sample<T> family and one-to-one mapping."""
    enum_match = re.search(r"pub enum DescriptorSampleInput\s*\{(?P<body>.*?)\n\}", binding, re.DOTALL)
    require(enum_match is not None, "DescriptorSampleInput declaration is missing")
    variants = dict(re.findall(r"(?m)^\s{4}([A-Z][A-Za-z0-9]*)\(Sample<([^>]+)>\),\s*$", enum_match.group("body")))
    require(variants == EXPECTED_MAPPINGS, "DescriptorSampleInput must contain exactly seven Sample<T> mappings")

    arms = dict(re.findall(
        r"Self::([A-Z][A-Za-z0-9]*)\(_\)\s*=>\s*ChannelFormat::([A-Z][A-Za-z0-9]*)",
        binding,
    ))
    require(arms == {variant: variant for variant in EXPECTED_MAPPINGS}, "input-to-ChannelFormat mapping drifted")


def validate_source_contract() -> None:
    """Require binding invariants and reject conversion, wire, or runtime surfaces."""
    source = source_text()
    binding = BINDING_PATH.read_text(encoding="utf-8")
    implementation = binding.split("#[cfg(test)]", maxsplit=1)[0]
    validate_exact_input_family(binding)

    for required in (
        "pub struct DescriptorSampleLimits", "max_string_value_code_points: usize",
        "pub struct BoundDescriptorSample", "limits: DescriptorSampleLimits",
        "channel_count: usize", "channel_format: ChannelFormat",
        "sample: DescriptorSampleInput", "descriptor: &StreamDescriptor",
        "ChannelFormatMismatch", "ChannelCountMismatch", "StringValueLimitExceeded",
        "channel_index: usize", "expected_max: usize", ".chars().count()",
        "let expected_format = descriptor.channel_format()",
        "let expected_channels = descriptor.channel_count()",
    ):
        require(required in binding, f"CORE-005 source invariant is missing: {required}")

    accepted_match = re.search(r"pub struct BoundDescriptorSample\s*\{(?P<body>.*?)\n\}", binding, re.DOTALL)
    require(accepted_match is not None, "accepted binding declaration is missing")
    accepted_body = accepted_match.group("body")
    require(not re.search(r"(?m)^\s*pub\s+", accepted_body), "accepted binding fields became publicly forgeable")
    require("StreamDescriptor" not in accepted_body, "accepted binding clones or retains the full descriptor")

    require("#![forbid(unsafe_code)]" in source, "unsafe Rust is not forbidden")
    require("#![deny(missing_docs)]" in source, "missing documentation is not denied")
    prohibited_imports = {
        "external ABI": re.compile(r'\bextern\s+"'),
        "network API": re.compile(r"\b(?:std|core)::net\b"),
        "thread API": re.compile(r"\bstd::thread\b"),
        "process API": re.compile(r"\bstd::process\b"),
        "clock API": re.compile(r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b"),
        "FFI API": re.compile(r"\b(?:std|core)::ffi\b"),
        "filesystem API": re.compile(r"\bstd::fs\b"),
        "environment API": re.compile(r"\bstd::env\b"),
        "I/O API": re.compile(r"\bstd::io\b"),
        "synchronization API": re.compile(r"\bstd::sync\b"),
        "unsafe block": re.compile(r"(?m)^\s*unsafe\s*\{"),
    }
    for label, pattern in prohibited_imports.items():
        require(not pattern.search(source), f"CORE-005 opened a prohibited {label}")

    require(not re.search(r"\bimpl\s+(?:Try)?From\s*<", implementation), "CORE-005 opened a From conversion")
    require(not re.search(r"\bimpl\s+(?:Try)?Into\s*<", implementation), "CORE-005 opened an Into conversion")
    require(not re.search(r"\bas\s+(?:f32|f64|i8|i16|i32|i64|isize|usize|String|str)\b", implementation),
            "CORE-005 performs a value cast")
    require(not re.search(r"\b(?:format|print|println)\s*!\s*\(", implementation),
            "CORE-005 performs value formatting or output")
    require(not re.search(r"\.(?:parse|to_string|to_owned|clone|to_be_bytes|to_le_bytes|from_be_bytes|from_le_bytes)\s*\(", implementation),
            "CORE-005 performs conversion, encoding, or extra cloning")
    require("std::mem::transmute" not in implementation, "CORE-005 performs representation transmutation")

    public_identifiers = re.findall(
        r"(?m)^\s*pub\s+(?:const\s+)?(?:struct|enum|trait|type|fn)\s+([A-Za-z_][A-Za-z0-9_]*)"
        r"|^\s*pub\s+(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)",
        implementation,
    )
    names = {left or right for left, right in public_identifiers}
    prohibited_name = re.compile(
        r"(?i)(?:encode|decode|serializ|deserializ|parse|coerc|cast|convert|wire|protocol|runtime|"
        r"endiann|byte_size|byte_len|socket|network|transport|inlet|outlet|discover|buffer|queue|provider)"
    )
    bad_names = sorted(name for name in names if prohibited_name.search(name))
    require(not bad_names, f"CORE-005 opened prohibited public surface: {bad_names}")


def validate_inert_closure_and_instructions() -> None:
    """Reject dependency/activation drift and enforce scoped instruction completion."""
    result = subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    packages = json.loads(result.stdout).get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    require(packages[0].get("dependencies") == [], "CORE-005 permits no Cargo dependency")
    require(packages[0].get("features") == {}, "CORE-005 permits no Cargo feature")
    require(packages[0].get("publish") == [], "CORE-005 package must remain unpublished")

    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock default activation drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect-union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union must remain empty")

    unit = load_object(UNIT_PATH)
    require(unit.get("status") in {"active", "validating", "accepted"}, "CORE-005 lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list), "CORE-005 instruction surfaces are missing")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces if isinstance(surface, dict)}
    require(statuses.get("AGENTS.md") == "complete", "AGENTS.md instruction row is not complete")
    require(statuses.get("README.md") == "complete", "README.md instruction row is not complete")
    require(statuses.get("<skills-root>/system-engineering/SKILL.md") == "complete",
            "system-engineering instruction review is not complete")


def validate_docs_and_license() -> None:
    """Require exact limitations, no overclaim, and AGPL provenance."""
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    normalized = " ".join("\n".join(path.read_text(encoding="utf-8") for path in paths).split())
    for phrase in (
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
        "descriptor/sample binding", "exactly seven homogeneous", "Unicode scalar",
        "signed zero", "NaN payload", "no conversion", "local Rust contract tests",
    ):
        require(phrase in normalized, f"required CORE-005 limitation is missing: {phrase}")

    positive = re.compile(
        r"(?i)\b(?:LSL\s+)?(?:protocol|wire|runtime|operational|ecosystem)\s+"
        r"(?:compatibility|support|behavior)\s+(?:is|has been)\s+"
        r"(?:implemented|supported|proven|verified|validated)\b"
    )
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        for number, line in enumerate(lines, 1):
            if positive.search(line):
                context = " ".join(lines[max(0, number - 2):number]).lower()
                require(any(word in context for word in (" no ", " not ", "without", "isn't", "hasn't")),
                        f"compatibility overclaim in {path.relative_to(ROOT)}:{number}")

    require('license = "AGPL-3.0-or-later"' in (ROOT / "Cargo.toml").read_text(encoding="utf-8"),
            "workspace license expression drifted")
    require("license.workspace = true" in (ROOT / "crates" / "rusty-lsl" / "Cargo.toml").read_text(encoding="utf-8"),
            "crate license inheritance drifted")
    require("Project-owned source and documentation are licensed `AGPL-3.0-or-later`." in
            (ROOT / "docs" / "PROVENANCE.md").read_text(encoding="utf-8"), "provenance license drifted")
    for path in (BINDING_PATH, ROOT / "tools" / "check_core_005.py", ROOT / "tools" / "check_core_005.ps1"):
        require("SPDX-License-Identifier: AGPL-3.0-or-later" in path.read_text(encoding="utf-8"),
                f"license drift in {path.relative_to(ROOT)}")


def main() -> int:
    """Run all deterministic CORE-005 checks."""
    validate_overlay_and_tests()
    validate_historical_baseline()
    validate_source_contract()
    validate_inert_closure_and_instructions()
    validate_docs_and_license()
    print("CORE-005 descriptor/sample binding contract checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
