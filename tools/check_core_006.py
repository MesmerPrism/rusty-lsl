#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-006 timestamped binding composition."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
COMPOSITION_PATH = SOURCE_ROOT / "timestamped_descriptor_sample.rs"
CORE_005_PATH = SOURCE_ROOT / "descriptor_sample.rs"
FIXTURES = ROOT / "fixtures" / "compatibility"
OVERLAY_PATH = FIXTURES / "core-006-contract-results.json"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
UNIT_PATH = (
    ROOT
    / "morphospace"
    / "iteration-units"
    / "rlsl-core-006-timestamped-descriptor-sample.json"
)
CONTRACT_ID = "timestamped-descriptor-sample-composition"
POSITIVE_TESTS = {
    "core_006_all_seven_timestamped_format_mappings_bind_exactly",
    "core_006_float_nan_payloads_and_timestamp_pairing_survive_consumption",
    "core_006_integer_edges_and_order_are_preserved",
    "core_006_raw_only_and_both_derived_kinds_preserve_exact_bits",
    "core_006_string_exact_bound_and_accessors_preserve_pairing",
}
DAMAGED_TESTS = {
    "core_006_delegated_validation_precedence_is_unchanged",
    "core_006_delegates_channel_mismatch_without_rewriting_error",
    "core_006_delegates_first_one_past_string_error_exactly",
    "core_006_delegates_format_mismatch_without_rewriting_error",
    "core_006_zero_string_limit_retains_delegated_typed_error",
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
    "activities",
    "assets",
    "commands",
    "inputs",
    "markers",
    "native_libraries",
    "permissions",
    "queries",
    "routes",
    "scenes",
    "services",
    "shaders",
    "streams",
    "tools",
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
    return "\n".join(
        path.read_text(encoding="utf-8") for path in sorted(SOURCE_ROOT.glob("*.rs"))
    )


def validate_overlay_and_tests() -> None:
    """Bind the exact CORE-006 test inventory without promoting STRM-000."""
    overlay = load_object(OVERLAY_PATH)
    require(
        overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1",
        "CORE-006 overlay schema drifted",
    )
    require(
        overlay.get("overlay_id") == "core-006-timestamped-descriptor-sample",
        "CORE-006 overlay identity drifted",
    )
    require(
        overlay.get("unit_id") == "rlsl-core-006-timestamped-descriptor-sample",
        "CORE-006 unit binding drifted",
    )
    require(
        overlay.get("evidence_level") == "local-rust-contract-tests",
        "CORE-006 evidence level is inaccurate",
    )
    require(
        overlay.get("implementation_status") == "bounded-local-contracts",
        "CORE-006 status is inaccurate",
    )

    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-006 baseline binding is missing")
    require(
        binding.get("catalog_id") == "strm-000-baseline",
        "STRM-000 catalog binding drifted",
    )
    require(
        binding.get("case_id") == "semantic-raw-timestamp-preserved",
        "CORE-006 timestamp binding drifted",
    )
    require(
        binding.get("preserved_result") == "not-implemented",
        "STRM-000 history was promoted",
    )
    interpretation = binding.get("interpretation")
    require(
        isinstance(interpretation, str)
        and "specification-only" in interpretation
        and "local Rust" in interpretation,
        "CORE-006 baseline limitation is incomplete",
    )

    results = overlay.get("contract_results")
    require(
        isinstance(results, list) and len(results) == 1,
        "CORE-006 must have one contract result",
    )
    result = results[0]
    require(
        isinstance(result, dict) and result.get("contract_id") == CONTRACT_ID,
        "CORE-006 contract identity drifted",
    )
    require(
        result.get("result") == "implemented-local-contract",
        "CORE-006 result is inaccurate",
    )
    require(
        set(result.get("positive_tests", [])) == POSITIVE_TESTS,
        "CORE-006 positive coverage drifted",
    )
    require(
        set(result.get("damaged_tests", [])) == DAMAGED_TESTS,
        "CORE-006 damaged coverage drifted",
    )
    declared_tests = set(
        re.findall(r"(?m)^\s*fn\s+(core_006_[a-z0-9_]+)\s*\(", source_text())
    )
    require(
        declared_tests == POSITIVE_TESTS | DAMAGED_TESTS,
        "CORE-006 Rust test inventory drifted",
    )

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-006 provenance is missing")
    require(
        provenance.get("origin_classification") == "independently-authored",
        "CORE-006 origin drifted",
    )
    require(
        provenance.get("license_expression") == "AGPL-3.0-or-later",
        "CORE-006 license drifted",
    )
    require(
        provenance.get("implementation_inputs") == [],
        "CORE-006 has prohibited implementation inputs",
    )
    required_limits = {
        "clock reading, clock correction, dejittering, smoothing, interpolation, or timestamp derivation",
        "timestamp sorting, rewriting, scheduling, buffering, or postprocessing",
        "numeric or string conversion, coercion, casting, parsing, formatting, or normalization",
        "byte sizing, encoding, decoding, endianness, memory layout, or wire mapping",
        "LSL protocol behavior",
        "sample, descriptor, or timestamp transport",
        "discovery, recovery, providers, or runtime action",
        "source identity, authorization, admission, routing, permission, or Morphospace authority",
        "wire compatibility",
        "runtime support",
        "ecosystem compatibility",
        "official liblsl behavior",
    }
    require(
        required_limits <= set(overlay.get("does_not_prove", [])),
        "CORE-006 limitations are incomplete",
    )


def validate_historical_baseline() -> None:
    """Reject compatibility promotion or invented STRM-000 measurements."""
    catalog = load_object(CATALOG_PATH)
    require(
        catalog.get("catalog_id") == "strm-000-baseline",
        "STRM-000 catalog identity drifted",
    )
    require(
        catalog.get("evidence_level") == "specification-only",
        "STRM-000 evidence level drifted",
    )
    cases = catalog.get("cases")
    require(isinstance(cases, list) and cases, "STRM-000 cases are missing")
    for case in cases:
        require(isinstance(case, dict), "STRM-000 case must be an object")
        case_id = case.get("case_id")
        require(
            case.get("current_result") == "not-implemented",
            f"{case_id} promoted STRM-000 history",
        )
        measured = case.get("measured_result")
        require(isinstance(measured, dict), f"{case_id} measured-result role is missing")
        require(
            measured.get("status") == "not-implemented"
            and measured.get("observation") is None,
            f"{case_id} invents a measured STRM-000 result",
        )


def validate_exact_input_family(composition: str) -> None:
    """Require the closed seven-way input and transitive format mappings."""
    enum_match = re.search(
        r"pub enum TimestampedDescriptorSampleInput\s*\{(?P<body>.*?)\n\}",
        composition,
        re.DOTALL,
    )
    require(enum_match is not None, "TimestampedDescriptorSampleInput is missing")
    variants = dict(
        re.findall(
            r"(?m)^\s{4}([A-Z][A-Za-z0-9]*)\(TimestampedSample<([^>]+)>\),\s*$",
            enum_match.group("body"),
        )
    )
    require(
        variants == EXPECTED_MAPPINGS,
        "input must contain exactly seven TimestampedSample<T> mappings",
    )

    arms = dict(
        re.findall(
            r"Self::([A-Z][A-Za-z0-9]*)\(timestamped\).*?"
            r"DescriptorSampleInput::([A-Z][A-Za-z0-9]*)\(sample\)",
            composition,
            re.DOTALL,
        )
    )
    require(
        arms == {variant: variant for variant in EXPECTED_MAPPINGS},
        "TimestampedSample-to-CORE-005 mapping drifted",
    )

    core_005 = CORE_005_PATH.read_text(encoding="utf-8")
    core_005_enum = re.search(
        r"pub enum DescriptorSampleInput\s*\{(?P<body>.*?)\n\}",
        core_005,
        re.DOTALL,
    )
    require(core_005_enum is not None, "CORE-005 input family is missing")
    core_005_variants = dict(
        re.findall(
            r"(?m)^\s{4}([A-Z][A-Za-z0-9]*)\(Sample<([^>]+)>\),\s*$",
            core_005_enum.group("body"),
        )
    )
    require(
        core_005_variants == EXPECTED_MAPPINGS,
        "CORE-005 input family no longer matches the seven CORE-006 families",
    )
    format_arms = dict(
        re.findall(
            r"Self::([A-Z][A-Za-z0-9]*)\(_\)\s*=>\s*"
            r"ChannelFormat::([A-Z][A-Za-z0-9]*)",
            core_005,
        )
    )
    require(
        format_arms == {variant: variant for variant in EXPECTED_MAPPINGS},
        "CORE-006-to-CORE-005-to-ChannelFormat mapping drifted",
    )


def validate_core_005_delegation(composition: str) -> None:
    """Require one delegation and reject copied or weakened CORE-005 checks."""
    implementation = composition.split("#[cfg(test)]", maxsplit=1)[0]
    require(
        implementation.count("BoundDescriptorSample::new(") == 1,
        "CORE-006 must delegate exactly once to BoundDescriptorSample::new",
    )
    require(
        "Result<Self, DescriptorSampleError>" in implementation,
        "CORE-006 must return the delegated CORE-005 error type",
    )
    for prohibited in (
        ".chars().count()",
        "descriptor.channel_format()",
        "descriptor.channel_count()",
        "RawSourceTimestamp::new(",
        "DerivedTimestamp::new(",
        "TimestampedSample::new(",
        "ChannelFormatMismatch {",
        "ChannelCountMismatch {",
        "StringValueLimitExceeded {",
    ):
        require(
            prohibited not in implementation,
            f"CORE-006 duplicated CORE-005 validation: {prohibited}",
        )

    core_005 = CORE_005_PATH.read_text(encoding="utf-8").split(
        "#[cfg(test)]", maxsplit=1
    )[0]
    ordered_markers = (
        "let expected_format = descriptor.channel_format()",
        "if actual_format != expected_format",
        "let expected_channels = descriptor.channel_count()",
        "if actual_channels != expected_channels",
        "if let DescriptorSampleInput::String",
    )
    positions = [core_005.find(marker) for marker in ordered_markers]
    require(
        all(position >= 0 for position in positions) and positions == sorted(positions),
        "CORE-005 format-channel-string validation precedence was weakened",
    )


def validate_accepted_state(composition: str) -> None:
    """Require compact private state that preserves the exact pairing."""
    accepted = re.search(
        r"pub struct BoundTimestampedDescriptorSample\s*\{(?P<body>.*?)\n\}",
        composition,
        re.DOTALL,
    )
    require(accepted is not None, "accepted CORE-006 state is missing")
    body = accepted.group("body")
    fields = dict(
        re.findall(r"(?m)^\s{4}([a-z][a-z0-9_]*):\s*([^,]+),\s*$", body)
    )
    require(
        fields
        == {
            "bound_sample": "BoundDescriptorSample",
            "raw_source_timestamp": "RawSourceTimestamp",
            "derived_timestamp": "Option<DerivedTimestamp>",
        },
        "accepted CORE-006 state must contain only the binding and timestamp evidence",
    )
    require(
        not re.search(r"(?m)^\s*pub\s+", body),
        "accepted CORE-006 fields became publicly forgeable",
    )
    for accessor in (
        "pub const fn bound_sample(&self) -> &BoundDescriptorSample",
        "pub const fn raw_source_timestamp(&self) -> RawSourceTimestamp",
        "pub const fn derived_timestamp(&self) -> Option<DerivedTimestamp>",
        "pub fn into_parts(",
    ):
        require(accessor in composition, f"CORE-006 accessor is missing: {accessor}")


def validate_no_algorithms_or_runtime(composition: str) -> None:
    """Reject timestamp rewriting, conversion, encoding, and runtime APIs."""
    implementation = composition.split("#[cfg(test)]", maxsplit=1)[0]
    source = source_text()
    require("#![forbid(unsafe_code)]" in source, "unsafe Rust is not forbidden")
    require("#![deny(missing_docs)]" in source, "missing documentation is not denied")
    prohibited_apis = {
        "external ABI": re.compile(r'\bextern\s+"'),
        "network API": re.compile(r"\b(?:std|core)::net\b"),
        "thread API": re.compile(r"\bstd::thread\b"),
        "process API": re.compile(r"\bstd::process\b"),
        "clock API": re.compile(
            r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b"
        ),
        "FFI API": re.compile(r"\b(?:std|core)::ffi\b"),
        "filesystem API": re.compile(r"\bstd::fs\b"),
        "environment API": re.compile(r"\bstd::env\b"),
        "I/O API": re.compile(r"\bstd::io\b"),
        "synchronization API": re.compile(r"\bstd::sync\b"),
        "unsafe block": re.compile(r"(?m)^\s*unsafe\s*\{"),
    }
    for label, pattern in prohibited_apis.items():
        require(not pattern.search(source), f"CORE-006 opened a prohibited {label}")

    prohibited_operations = {
        "timestamp value read": re.compile(
            r"(?:raw_source_timestamp|derived_timestamp)\s*\.\s*(?:value|to_bits)\s*\("
        ),
        "sorting": re.compile(r"\.(?:sort|sort_by|sort_by_key|sort_unstable)\s*\("),
        "numeric cast": re.compile(
            r"\bas\s+(?:f32|f64|i8|i16|i32|i64|isize|usize)\b"
        ),
        "conversion trait": re.compile(r"\bimpl\s+(?:Try)?(?:From|Into)\s*<"),
        "parsing or encoding": re.compile(
            r"\.(?:parse|to_string|to_owned|clone|to_be_bytes|to_le_bytes|"
            r"from_be_bytes|from_le_bytes)\s*\("
        ),
        "formatting": re.compile(r"\b(?:format|print|println)\s*!\s*\("),
    }
    for label, pattern in prohibited_operations.items():
        require(
            not pattern.search(implementation),
            f"CORE-006 performs prohibited {label}",
        )
    require(
        "std::mem::transmute" not in implementation,
        "CORE-006 performs representation transmutation",
    )

    public_identifiers = re.findall(
        r"(?m)^\s*pub\s+(?:const\s+)?(?:struct|enum|trait|type|fn)\s+"
        r"([A-Za-z_][A-Za-z0-9_]*)|^\s*pub\s+(?:const\s+)?fn\s+"
        r"([A-Za-z_][A-Za-z0-9_]*)",
        implementation,
    )
    names = {left or right for left, right in public_identifiers}
    prohibited_name = re.compile(
        r"(?i)(?:correct|dejitter|smooth|interpolat|derive_timestamp|sort|rewrite|"
        r"schedule|encode|decode|serializ|deserializ|parse|coerc|cast|convert|wire|"
        r"protocol|runtime|endiann|byte_size|byte_len|socket|network|transport|inlet|"
        r"outlet|discover|buffer|queue|provider)"
    )
    bad_names = sorted(name for name in names if prohibited_name.search(name))
    require(not bad_names, f"CORE-006 opened prohibited public surface: {bad_names}")


def validate_inert_closure_and_instructions() -> None:
    """Reject dependency/activation drift and enforce scoped instruction status."""
    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--offline",
            "--locked",
            "--no-deps",
            "--format-version",
            "1",
        ],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    packages = json.loads(result.stdout).get("packages", [])
    require(
        len(packages) == 1 and packages[0].get("name") == "rusty-lsl",
        "package closure drifted",
    )
    require(packages[0].get("dependencies") == [], "CORE-006 permits no dependency")
    require(packages[0].get("features") == {}, "CORE-006 permits no Cargo feature")
    require(packages[0].get("publish") == [], "CORE-006 package must remain unpublished")

    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(
        lock.get("default_activation") == "disabled",
        "feature lock default activation drifted",
    )
    require(
        lock.get("selected_features") == [] and lock.get("features") == [],
        "feature lock is not empty",
    )
    effects = lock.get("effect_union")
    require(
        isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS,
        "effect-union shape drifted",
    )
    require(
        all(value == [] for value in effects.values()),
        "runtime effect union must remain empty",
    )

    unit = load_object(UNIT_PATH)
    require(
        unit.get("status") in {"active", "validating", "accepted"},
        "CORE-006 lifecycle state is invalid",
    )
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list), "CORE-006 instruction surfaces are missing")
    statuses = {
        surface.get("path"): surface.get("status")
        for surface in surfaces
        if isinstance(surface, dict)
    }
    require(
        statuses.get("AGENTS.md") == "complete",
        "AGENTS.md instruction row is not complete",
    )
    require(
        statuses.get("README.md") == "complete",
        "README.md instruction row is not complete",
    )
    require(
        statuses.get("<skills-root>/system-engineering/SKILL.md") == "complete",
        "system-engineering instruction review is not complete",
    )


def validate_docs_and_license() -> None:
    """Require scoped claims, exact limitations, and AGPL provenance."""
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    normalized = " ".join(
        "\n".join(path.read_text(encoding="utf-8") for path in paths).split()
    )
    for phrase in (
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
        "timestamped descriptor/sample composition",
        "exactly seven homogeneous",
        "BoundDescriptorSample::new",
        "raw source timestamp bits",
        "local Rust contract tests",
    ):
        require(phrase in normalized, f"required CORE-006 limitation is missing: {phrase}")

    positive = re.compile(
        r"(?i)\b(?:LSL\s+)?(?:protocol|wire|runtime|operational|ecosystem)\s+"
        r"(?:compatibility|support|behavior)\s+(?:is|has been)\s+"
        r"(?:implemented|supported|proven|verified|validated)\b"
    )
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        for number, line in enumerate(lines, 1):
            if positive.search(line):
                context = " ".join(lines[max(0, number - 2) : number]).lower()
                require(
                    any(
                        word in context
                        for word in (" no ", " not ", "without", "isn't", "hasn't")
                    ),
                    f"compatibility overclaim in {path.relative_to(ROOT)}:{number}",
                )

    require(
        'license = "AGPL-3.0-or-later"'
        in (ROOT / "Cargo.toml").read_text(encoding="utf-8"),
        "workspace license expression drifted",
    )
    require(
        "license.workspace = true"
        in (ROOT / "crates" / "rusty-lsl" / "Cargo.toml").read_text(
            encoding="utf-8"
        ),
        "crate license inheritance drifted",
    )
    require(
        "Project-owned source and documentation are licensed `AGPL-3.0-or-later`."
        in (ROOT / "docs" / "PROVENANCE.md").read_text(encoding="utf-8"),
        "provenance license drifted",
    )
    for path in (COMPOSITION_PATH, ROOT / "tools" / "check_core_006.py", ROOT / "tools" / "check_core_006.ps1"):
        require(
            "SPDX-License-Identifier: AGPL-3.0-or-later"
            in path.read_text(encoding="utf-8"),
            f"license drift in {path.relative_to(ROOT)}",
        )


def main() -> int:
    """Run all deterministic CORE-006 checks."""
    composition = COMPOSITION_PATH.read_text(encoding="utf-8")
    validate_overlay_and_tests()
    validate_historical_baseline()
    validate_exact_input_family(composition)
    validate_core_005_delegation(composition)
    validate_accepted_state(composition)
    validate_no_algorithms_or_runtime(composition)
    validate_inert_closure_and_instructions()
    validate_docs_and_license()
    print("CORE-006 timestamped descriptor/sample composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
