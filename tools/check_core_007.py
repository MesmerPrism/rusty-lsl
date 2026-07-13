#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-007 timestamped chunk composition."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
COMPOSITION_PATH = SOURCE_ROOT / "timestamped_descriptor_chunk.rs"
CORE_006_PATH = SOURCE_ROOT / "timestamped_descriptor_sample.rs"
CORE_005_PATH = SOURCE_ROOT / "descriptor_sample.rs"
OVERLAY_PATH = ROOT / "fixtures" / "compatibility" / "core-007-contract-results.json"
CATALOG_PATH = ROOT / "fixtures" / "compatibility" / "behavior-catalog.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-core-007-timestamped-descriptor-chunk.json"
POSITIVE_TESTS = {
    "core_007_all_seven_timestamped_chunk_mappings_bind_exactly",
    "core_007_float_signed_zero_and_nan_payload_bits_are_preserved",
    "core_007_integer_edges_and_order_are_preserved",
    "core_007_multi_sample_order_and_timestamp_pairing_are_exact",
    "core_007_original_chunk_limits_survive_read_only_and_consuming_accessors",
    "core_007_string_values_order_and_allocations_are_moved_unchanged",
}
DAMAGED_TESTS = {
    "core_007_channel_mismatch_at_sample_zero_is_unchanged",
    "core_007_empty_chunk_rejects_before_sample_delegation",
    "core_007_first_failure_order_and_delegated_precedence_are_unchanged",
    "core_007_format_mismatch_at_sample_zero_is_unchanged",
    "core_007_later_string_failure_retains_sample_and_channel_indexes",
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


def validate_overlay_and_history() -> None:
    """Bind the exact tests while preserving every historical STRM-000 result."""
    overlay = load_object(OVERLAY_PATH)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "CORE-007 overlay schema drifted")
    require(overlay.get("overlay_id") == "core-007-timestamped-descriptor-chunk", "CORE-007 overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-core-007-timestamped-descriptor-chunk", "CORE-007 unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "CORE-007 evidence level drifted")
    require(overlay.get("implementation_status") == "bounded-local-contracts", "CORE-007 status drifted")
    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-007 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 binding drifted")
    require(binding.get("case_id") == "contract-sample-shape", "CORE-007 case binding drifted")
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    require("specification-only" in str(binding.get("interpretation")), "CORE-007 limitation is incomplete")

    results = overlay.get("contract_results")
    require(isinstance(results, list) and len(results) == 1, "CORE-007 must have one contract result")
    result = results[0]
    require(result.get("contract_id") == "timestamped-descriptor-chunk-composition", "CORE-007 contract identity drifted")
    require(result.get("result") == "implemented-local-contract", "CORE-007 result drifted")
    require(set(result.get("positive_tests", [])) == POSITIVE_TESTS, "CORE-007 positive coverage drifted")
    require(set(result.get("damaged_tests", [])) == DAMAGED_TESTS, "CORE-007 damaged coverage drifted")
    tests = set(re.findall(r"(?m)^\s*fn\s+(core_007_[a-z0-9_]+)\s*\(", source_text()))
    require(tests == POSITIVE_TESTS | DAMAGED_TESTS, "CORE-007 Rust test inventory drifted")

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-007 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-007 origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "CORE-007 license drifted")
    require(provenance.get("implementation_inputs") == [], "CORE-007 has prohibited implementation inputs")
    limitations = set(overlay.get("does_not_prove", []))
    for required in (
        "actual LSL empty-chunk compatibility behavior", "wire compatibility", "runtime support",
        "ecosystem compatibility", "official liblsl behavior",
    ):
        require(required in limitations, f"CORE-007 limitation is missing: {required}")

    catalog = load_object(CATALOG_PATH)
    require(catalog.get("evidence_level") == "specification-only", "STRM-000 evidence level drifted")
    cases = catalog.get("cases")
    require(isinstance(cases, list) and cases, "STRM-000 cases are missing")
    for case in cases:
        require(case.get("current_result") == "not-implemented", f"{case.get('case_id')} promoted history")
        measured = case.get("measured_result")
        require(isinstance(measured, dict), "STRM-000 measured-result role is missing")
        require(measured.get("status") == "not-implemented" and measured.get("observation") is None,
                f"{case.get('case_id')} invents a measured result")


def enum_variants(source: str, enum_name: str, wrapper: str) -> dict[str, str]:
    """Extract a closed tuple-variant family."""
    match = re.search(rf"pub enum {enum_name}\s*\{{(?P<body>.*?)\n\}}", source, re.DOTALL)
    require(match is not None, f"{enum_name} is missing")
    return dict(re.findall(rf"(?m)^\s{{4}}([A-Z][A-Za-z0-9]*)\({wrapper}<([^>]+)>\),\s*$", match.group("body")))


def validate_transitive_mapping(composition: str) -> None:
    """Require CORE-007 -> CORE-006 -> CORE-005 -> ChannelFormat closure."""
    require(enum_variants(composition, "TimestampedDescriptorChunkInput", "TimestampedChunk") == EXPECTED_MAPPINGS,
            "CORE-007 input must contain exactly seven TimestampedChunk<T> mappings")
    chunk_arms = dict(re.findall(
        r"TimestampedDescriptorChunkInput::([A-Z][A-Za-z0-9]*)\(chunk\).*?"
        r"TimestampedDescriptorSampleInput::([A-Z][A-Za-z0-9]*),", composition, re.DOTALL))
    require(chunk_arms == {variant: variant for variant in EXPECTED_MAPPINGS}, "CORE-007-to-CORE-006 mapping drifted")

    core_006 = CORE_006_PATH.read_text(encoding="utf-8")
    require(enum_variants(core_006, "TimestampedDescriptorSampleInput", "TimestampedSample") == EXPECTED_MAPPINGS,
            "CORE-006 input family drifted")
    sample_arms = dict(re.findall(
        r"Self::([A-Z][A-Za-z0-9]*)\(timestamped\).*?DescriptorSampleInput::([A-Z][A-Za-z0-9]*)\(sample\)",
        core_006, re.DOTALL))
    require(sample_arms == {variant: variant for variant in EXPECTED_MAPPINGS}, "CORE-006-to-CORE-005 mapping drifted")

    core_005 = CORE_005_PATH.read_text(encoding="utf-8")
    require(enum_variants(core_005, "DescriptorSampleInput", "Sample") == EXPECTED_MAPPINGS, "CORE-005 input family drifted")
    format_arms = dict(re.findall(
        r"Self::([A-Z][A-Za-z0-9]*)\(_\)\s*=>\s*ChannelFormat::([A-Z][A-Za-z0-9]*)", core_005))
    require(format_arms == {variant: variant for variant in EXPECTED_MAPPINGS}, "transitive ChannelFormat mapping drifted")


def validate_delegation_and_state(composition: str) -> None:
    """Require one ordered generic delegation, indexed errors, and compact state."""
    implementation = composition.split("#[cfg(test)]", maxsplit=1)[0]
    require(implementation.count("BoundTimestampedDescriptorSample::new(") == 1,
            "CORE-007 must have exactly one source delegation call")
    helper = re.search(r"fn bind_chunk<T>\((?P<body>.*?)\n\s{4}\}", implementation, re.DOTALL)
    require(helper is not None and "BoundTimestampedDescriptorSample::new(" in helper.group("body"),
            "delegation must live in the generic per-sample path")
    body = helper.group("body")
    positions = [body.find(marker) for marker in (
        "let samples = chunk.into_samples()", "if samples.is_empty()", "Vec::with_capacity(samples.len())",
        "samples.into_iter().enumerate()", "BoundTimestampedDescriptorSample::new(", "bound_samples.push(bound)",
    )]
    require(all(position >= 0 for position in positions) and positions == sorted(positions),
            "non-empty-first or caller-order delegation drifted")
    require("sample_index" in body and "SampleRejected" in body and ".map_err(" in body,
            "indexed delegated error mapping is missing")
    require(implementation.count("Vec::with_capacity(") == 1, "accepted state must use one explicit Vec allocation")
    require(".collect::<Vec" not in implementation and ".clone(" not in implementation,
            "CORE-007 creates an intermediate vector or clones values")

    accepted = re.search(r"pub struct BoundTimestampedDescriptorChunk\s*\{(?P<body>.*?)\n\}", composition, re.DOTALL)
    require(accepted is not None, "accepted CORE-007 state is missing")
    fields = dict(re.findall(r"(?m)^\s{4}([a-z][a-z0-9_]*):\s*([^,]+),\s*$", accepted.group("body")))
    require(fields == {"chunk_limits": "ChunkLimits", "bound_samples": "Vec<BoundTimestampedDescriptorSample>"},
            "accepted CORE-007 state is not compact")
    require(not re.search(r"(?m)^\s*pub\s+", accepted.group("body")), "accepted fields became forgeable")
    for accessor in ("pub const fn chunk_limits(&self) -> ChunkLimits", "pub fn bound_samples(&self)", "pub fn into_parts(self)"):
        require(accessor in composition, f"CORE-007 accessor is missing: {accessor}")
    error = re.search(r"pub enum TimestampedDescriptorChunkError\s*\{(?P<body>.*?)\n\}", composition, re.DOTALL)
    require(error is not None and "EmptyChunk" in error.group("body"), "typed EmptyChunk error is missing")
    require("SampleRejected" in error.group("body") and "sample_index: usize" in error.group("body")
            and "error: DescriptorSampleError" in error.group("body"), "typed indexed delegated error is missing")

    for duplicated in (
        "descriptor.channel_format()", "descriptor.channel_count()", ".chars().count()",
        "ChannelFormatMismatch {", "ChannelCountMismatch {", "StringValueLimitExceeded {",
        "RawSourceTimestamp::new(", "DerivedTimestamp::new(",
    ):
        require(duplicated not in implementation, f"CORE-007 duplicated lower-layer validation: {duplicated}")


def validate_closed_surface(composition: str) -> None:
    """Reject algorithms, runtime APIs, dependency drift, and instruction drift."""
    implementation = composition.split("#[cfg(test)]", maxsplit=1)[0]
    source = source_text()
    require("#![forbid(unsafe_code)]" in source and "#![deny(missing_docs)]" in source, "crate safety lints drifted")
    prohibited = {
        "external ABI": r'\bextern\s+"', "network": r"\b(?:std|core)::net\b", "thread": r"\bstd::thread\b",
        "process": r"\bstd::process\b", "clock": r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b",
        "FFI": r"\b(?:std|core)::ffi\b", "filesystem": r"\bstd::fs\b", "I/O": r"\bstd::io\b",
        "synchronization": r"\bstd::sync\b", "unsafe block": r"(?m)^\s*unsafe\s*\{",
    }
    for label, pattern in prohibited.items():
        require(not re.search(pattern, source), f"CORE-007 opened a prohibited {label} API")
    operations = {
        "sorting": r"\.(?:sort|sort_by|sort_by_key|sort_unstable)\s*\(",
        "numeric cast": r"\bas\s+(?:f32|f64|i8|i16|i32|i64|isize|usize)\b",
        "conversion trait": r"\bimpl\s+(?:Try)?(?:From|Into)\s*<",
        "parsing or encoding": r"\.(?:parse|to_string|to_owned|clone|to_be_bytes|to_le_bytes|from_be_bytes|from_le_bytes)\s*\(",
        "formatting": r"\b(?:format|print|println)\s*!\s*\(",
    }
    for label, pattern in operations.items():
        require(not re.search(pattern, implementation), f"CORE-007 performs prohibited {label}")
    public_names = {left or right for left, right in re.findall(
        r"(?m)^\s*pub\s+(?:const\s+)?(?:struct|enum|trait|type|fn)\s+([A-Za-z_][A-Za-z0-9_]*)|"
        r"^\s*pub\s+(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", implementation)}
    banned_name = re.compile(r"(?i)(?:correct|dejitter|smooth|interpolat|sort|rewrite|split|merge|rechunk|schedule|buffer|queue|encode|decode|convert|wire|protocol|runtime|socket|network|transport|discover|provider)")
    require(not [name for name in public_names if banned_name.search(name)], "CORE-007 opened a prohibited public surface")

    metadata = subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True)
    packages = json.loads(metadata.stdout).get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    require(packages[0].get("dependencies") == [] and packages[0].get("features") == {}, "dependency or feature closure drifted")
    require(packages[0].get("publish") == [], "package must remain unpublished")
    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock activation drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS and all(value == [] for value in effects.values()),
            "runtime effect union must remain empty")

    unit = load_object(UNIT_PATH)
    require(
        unit.get("status") in {"active", "validating", "accepted"},
        "CORE-007 lifecycle state is invalid",
    )
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list) and len(surfaces) == 3, "CORE-007 instruction rows are incomplete")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces}
    require(statuses.get("AGENTS.md") == "complete", "AGENTS.md instruction row is not complete")
    require(statuses.get("README.md") == "complete", "README.md instruction row is not complete")
    require(statuses.get("<skills-root>/system-engineering/SKILL.md") == "complete",
            "system-engineering instruction review is not complete")


def validate_docs_and_license() -> None:
    """Require precise public claims and AGPL provenance."""
    docs = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    normalized = " ".join("\n".join(path.read_text(encoding="utf-8") for path in docs).split())
    for phrase in (
        "timestamped descriptor/chunk composition", "non-empty", "original chunk limits",
        "BoundTimestampedDescriptorSample::new", "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
    ):
        require(phrase in normalized, f"required CORE-007 claim is missing: {phrase}")
    require('license = "AGPL-3.0-or-later"' in (ROOT / "Cargo.toml").read_text(encoding="utf-8"), "workspace license drifted")
    require("Project-owned source and documentation are licensed `AGPL-3.0-or-later`." in (ROOT / "docs" / "PROVENANCE.md").read_text(encoding="utf-8"), "provenance license drifted")
    for path in (COMPOSITION_PATH, ROOT / "tools" / "check_core_007.py", ROOT / "tools" / "check_core_007.ps1"):
        require("SPDX-License-Identifier: AGPL-3.0-or-later" in path.read_text(encoding="utf-8"), f"license drift in {path.relative_to(ROOT)}")


def main() -> int:
    """Run all deterministic CORE-007 checks."""
    composition = COMPOSITION_PATH.read_text(encoding="utf-8")
    validate_overlay_and_history()
    validate_transitive_mapping(composition)
    validate_delegation_and_state(composition)
    validate_closed_surface(composition)
    validate_docs_and_license()
    print("CORE-007 timestamped descriptor/chunk composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
