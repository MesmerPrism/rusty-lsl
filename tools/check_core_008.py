#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-008 stream-definition composition."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
COMPOSITION_PATH = SOURCE_ROOT / "stream_definition.rs"
LIB_PATH = SOURCE_ROOT / "lib.rs"
OVERLAY_PATH = ROOT / "fixtures" / "compatibility" / "core-008-contract-results.json"
CATALOG_PATH = ROOT / "fixtures" / "compatibility" / "behavior-catalog.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-core-008-stream-definition-composition.json"
TESTS = {
    "core_008_all_seven_channel_formats_survive_composition",
    "core_008_borrow_access_preserves_all_component_values",
    "core_008_construction_moves_existing_allocations_unchanged",
    "core_008_into_parts_preserves_irregular_descriptor_and_tree_order",
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


def brace_body(source: str, marker_pattern: str) -> str:
    """Return a balanced brace body after a whitespace-tolerant marker."""
    marker = re.search(marker_pattern, source, re.DOTALL)
    require(marker is not None, f"source marker is missing: {marker_pattern}")
    opening = source.find("{", marker.end())
    require(opening >= 0, "source marker has no body")
    depth = 0
    for index in range(opening, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return source[opening + 1:index]
    raise ValueError("source body has unbalanced braces")


def validate_overlay_and_history() -> None:
    """Bind local tests while preserving every historical STRM-000 result."""
    overlay = load_object(OVERLAY_PATH)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "CORE-008 overlay schema drifted")
    require(overlay.get("overlay_id") == "core-008-stream-definition-composition", "CORE-008 overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-core-008-stream-definition-composition", "CORE-008 unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "CORE-008 evidence level drifted")
    require(overlay.get("implementation_status") == "bounded-local-contracts", "CORE-008 status drifted")
    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-008 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 binding drifted")
    require(binding.get("case_id") == "contract-metadata-bounds", "CORE-008 case binding drifted")
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    require("local-only" in str(binding.get("interpretation")), "CORE-008 local-only limitation is missing")
    results = overlay.get("contract_results")
    require(isinstance(results, list) and len(results) == 1, "CORE-008 must have one contract result")
    result = results[0]
    require(result.get("contract_id") == "stream-definition-composition", "CORE-008 contract identity drifted")
    require(result.get("result") == "implemented-local-contract", "CORE-008 result drifted")
    require(set(result.get("positive_tests", [])) == TESTS, "CORE-008 test binding drifted")
    require(result.get("damaged_tests") == [], "infallible CORE-008 composition must not invent rejection tests")
    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-008 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-008 origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "CORE-008 license drifted")
    require(provenance.get("implementation_inputs") == [], "CORE-008 has prohibited implementation inputs")
    limitations = set(overlay.get("does_not_prove", []))
    for required in (
        "XML or an LSL desc-element interpretation",
        "discovery, networking, transport, protocol, wire, provider, or runtime behavior",
        "wire compatibility",
        "runtime support",
        "ecosystem compatibility",
        "official liblsl behavior",
    ):
        require(required in limitations, f"CORE-008 limitation is missing: {required}")

    catalog = load_object(CATALOG_PATH)
    require(catalog.get("evidence_level") == "specification-only", "STRM-000 evidence level drifted")
    cases = catalog.get("cases")
    require(isinstance(cases, list) and cases, "STRM-000 cases are missing")
    for case in cases:
        require(case.get("current_result") == "not-implemented", f"{case.get('case_id')} promoted history")
        measured = case.get("measured_result")
        require(isinstance(measured, dict), "STRM-000 measured-result role is missing")
        require(
            measured.get("status") == "not-implemented" and measured.get("observation") is None,
            f"{case.get('case_id')} invents a measured result",
        )


def validate_api_and_tests(composition: str) -> None:
    """Require the exact compact aggregate and rustfmt-independent API shape."""
    implementation = composition.split("#[cfg(test)]", maxsplit=1)[0]
    struct_body = brace_body(implementation, r"pub\s+struct\s+StreamDefinition\b")
    fields = dict(re.findall(r"(?m)^\s*([a-z][a-z0-9_]*)\s*:\s*([^,\n]+)\s*,\s*$", struct_body))
    require(
        fields == {"descriptor": "StreamDescriptor", "extended_metadata": "MetadataTree"},
        "StreamDefinition must own exactly one descriptor and metadata tree",
    )
    require(not re.search(r"(?m)^\s*pub(?:\([^)]*\))?\s+", struct_body), "StreamDefinition fields became forgeable")

    constructor = brace_body(
        implementation,
        r"pub\s+fn\s+new\s*\(\s*descriptor\s*:\s*StreamDescriptor\s*,\s*extended_metadata\s*:\s*MetadataTree\s*\)\s*->\s*Self",
    )
    compact = re.sub(r"\s+", "", constructor)
    require(compact == "Self{descriptor,extended_metadata,}", "constructor must move both values directly into state")
    for forbidden in ("Result<", "Error", "Limits", ".clone(", "to_owned(", "Vec::", "String::", "collect(", "with_capacity("):
        require(forbidden not in constructor, f"constructor contains prohibited operation: {forbidden}")

    signatures = (
        r"pub\s+const\s+fn\s+descriptor\s*\(\s*&self\s*\)\s*->\s*&StreamDescriptor",
        r"pub\s+const\s+fn\s+extended_metadata\s*\(\s*&self\s*\)\s*->\s*&MetadataTree",
        r"pub\s+fn\s+into_parts\s*\(\s*self\s*\)\s*->\s*\(\s*StreamDescriptor\s*,\s*MetadataTree\s*\)",
    )
    for signature in signatures:
        require(re.search(signature, implementation) is not None, f"CORE-008 accessor is missing: {signature}")

    lib = LIB_PATH.read_text(encoding="utf-8")
    require(re.search(r"mod\s+stream_definition\s*;", lib) is not None, "stream_definition module is not declared")
    require(re.search(r"pub\s+use\s+stream_definition\s*::\s*StreamDefinition\s*;", lib) is not None, "StreamDefinition is not re-exported")
    declared = set(re.findall(r"(?m)^\s*fn\s+(core_008_[a-z0-9_]+)\s*\(", composition))
    require(declared == TESTS, "CORE-008 Rust test inventory drifted")
    for format_name in ("Float32", "Double64", "String", "Int32", "Int16", "Int8", "Int64"):
        require(f"ChannelFormat::{format_name}" in composition, f"CORE-008 test omits {format_name}")


def validate_closed_surface(composition: str) -> None:
    """Reject new runtime, interpretation, dependency, or activation surfaces."""
    implementation = composition.split("#[cfg(test)]", maxsplit=1)[0]
    source = "\n".join(path.read_text(encoding="utf-8") for path in sorted(SOURCE_ROOT.glob("*.rs")))
    require("#![forbid(unsafe_code)]" in source and "#![deny(missing_docs)]" in source, "crate safety lints drifted")
    prohibited_apis = {
        "external ABI": r'\bextern\s+"', "network": r"\b(?:std|core)::net\b",
        "thread": r"\bstd::thread\b", "process": r"\bstd::process\b",
        "clock": r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b",
        "FFI": r"\b(?:std|core)::ffi\b", "filesystem": r"\bstd::fs\b",
        "I/O": r"\bstd::io\b", "synchronization": r"\bstd::sync\b",
        "unsafe block": r"(?m)^\s*unsafe\s*\{",
    }
    for label, pattern in prohibited_apis.items():
        require(not re.search(pattern, source), f"CORE-008 opened a prohibited {label} API")
    public_names = set(re.findall(r"(?m)^pub\s+(?:const\s+)?(?:struct|enum|type|fn)\s+([A-Za-z_][A-Za-z0-9_]*)", implementation))
    require(public_names == {"StreamDefinition"}, f"CORE-008 opened an extra public type or function: {sorted(public_names)}")
    forbidden_names = re.compile(
        r"(?i)(?:xml|desc_element|runtime|version|creation|uid|session|host|address|port|fingerprint|recovery|discover|network|socket|inlet|outlet|transport|buffer|queue|clock|protocol|wire|provider|adapter|authority)"
    )
    field_and_method_names = set(re.findall(r"(?m)^\s*(?:pub\s+(?:const\s+)?)?(?:fn\s+)?([a-z][a-z0-9_]*)\s*(?::|\()", implementation))
    bad_names = sorted(name for name in field_and_method_names if forbidden_names.search(name))
    require(not bad_names, f"CORE-008 opened prohibited meaning: {bad_names}")

    metadata = subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    packages = json.loads(metadata.stdout).get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    require(packages[0].get("dependencies") == [], "CORE-008 permits no dependency")
    require(packages[0].get("features") == {}, "CORE-008 permits no Cargo feature")
    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock default drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union is not empty")


def validate_instruction_and_document_surfaces() -> None:
    """Require all scoped instruction rows and local-only claims."""
    unit = load_object(UNIT_PATH)
    require(unit.get("status") in {"active", "validating", "accepted"}, "CORE-008 lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list), "CORE-008 instruction surfaces are missing")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces if isinstance(surface, dict)}
    for path in ("AGENTS.md", "README.md", "<skills-root>/system-engineering/SKILL.md"):
        require(statuses.get(path) == "complete", f"instruction surface is not complete: {path}")

    normalized = " ".join(
        "\n".join(path.read_text(encoding="utf-8") for path in [ROOT / "AGENTS.md", ROOT / "README.md", *(ROOT / "docs").glob("*.md")]).split()
    )
    for phrase in (
        "stream-definition composition",
        "already validated",
        "generic metadata-tree root",
        "local Rust contract tests",
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
    ):
        require(phrase in normalized, f"required CORE-008 claim is missing: {phrase}")
    for path in (COMPOSITION_PATH, ROOT / "tools" / "check_core_008.py", ROOT / "tools" / "check_core_008.ps1"):
        require("SPDX-License-Identifier: AGPL-3.0-or-later" in path.read_text(encoding="utf-8"), f"license drift in {path.relative_to(ROOT)}")


def main() -> int:
    """Run all deterministic CORE-008 checks."""
    composition = COMPOSITION_PATH.read_text(encoding="utf-8")
    validate_overlay_and_history()
    validate_api_and_tests(composition)
    validate_closed_surface(composition)
    validate_instruction_and_document_surfaces()
    print("CORE-008 stream-definition composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
