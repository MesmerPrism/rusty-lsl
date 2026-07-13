#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-004 flat metadata-tree contracts."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
TREE_PATH = SOURCE_ROOT / "metadata_tree.rs"
FIXTURES = ROOT / "fixtures" / "compatibility"
OVERLAY_PATH = FIXTURES / "core-004-contract-results.json"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-core-004-bounded-metadata-tree.json"
CONTRACT_ID = "bounded-parent-before-child-metadata-tree"
POSITIVE_TESTS = {
    "core_004_absent_and_empty_values_remain_distinct",
    "core_004_child_fanout_at_limit",
    "core_004_deep_parent_chain_at_depth_limit",
    "core_004_tree_exact_limits_preserve_flat_state",
    "core_004_unicode_scalar_counts_not_bytes",
}
DAMAGED_TESTS = {
    "core_004_empty_arena_has_stable_error",
    "core_004_empty_name_has_stable_indexed_error",
    "core_004_extra_root_has_stable_indexed_error",
    "core_004_invalid_root_parent_has_stable_error",
    "core_004_one_past_child_limit_has_stable_indexed_error",
    "core_004_one_past_depth_limit_has_stable_indexed_error",
    "core_004_one_past_node_limit_has_stable_error",
    "core_004_one_past_text_limits_have_stable_indexed_errors",
    "core_004_self_forward_and_out_of_range_parents_are_distinct",
    "core_004_zero_limits_reject_in_argument_order",
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
    """Bind every focused test without promoting the historical baseline."""
    overlay = load_object(OVERLAY_PATH)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "CORE-004 overlay schema drifted")
    require(overlay.get("overlay_id") == "core-004-bounded-metadata-tree", "CORE-004 overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-core-004-bounded-metadata-tree", "CORE-004 unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "CORE-004 evidence level is inaccurate")
    require(overlay.get("implementation_status") == "bounded-local-contracts", "CORE-004 status is inaccurate")

    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-004 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog binding drifted")
    require(binding.get("case_id") == "contract-metadata-bounds", "CORE-004 metadata binding drifted")
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    interpretation = binding.get("interpretation")
    require(
        isinstance(interpretation, str)
        and "specification-only" in interpretation
        and "local Rust" in interpretation,
        "CORE-004 baseline limitation is incomplete",
    )

    results = overlay.get("contract_results")
    require(isinstance(results, list) and len(results) == 1, "CORE-004 must have one contract result")
    result = results[0]
    require(isinstance(result, dict) and result.get("contract_id") == CONTRACT_ID, "CORE-004 contract identity drifted")
    require(result.get("result") == "implemented-local-contract", "CORE-004 result is inaccurate")
    require(set(result.get("positive_tests", [])) == POSITIVE_TESTS, "CORE-004 positive coverage drifted")
    require(set(result.get("damaged_tests", [])) == DAMAGED_TESTS, "CORE-004 damaged coverage drifted")
    declared_tests = set(re.findall(r"(?m)^\s*fn\s+(core_004_[a-z0-9_]+)\s*\(", source_text()))
    require(declared_tests == POSITIVE_TESTS | DAMAGED_TESTS, "CORE-004 Rust test inventory drifted")

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-004 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-004 origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "CORE-004 license drifted")
    require(provenance.get("implementation_inputs") == [], "CORE-004 has prohibited implementation inputs")
    required_limits = {
        "XML syntax, parsing, serialization, escaping, namespaces, schemas, or queries",
        "recursive tree ownership, recursive traversal, or tree mutation",
        "LSL metadata document or protocol behavior",
        "discovery, resolution, recovery, or runtime identity",
        "sample, metadata, or descriptor transport",
        "wire compatibility", "runtime support", "ecosystem compatibility", "official liblsl behavior",
    }
    require(required_limits <= set(overlay.get("does_not_prove", [])), "CORE-004 limitations are incomplete")


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


def struct_body(source: str, name: str) -> str:
    """Return the simple named-struct body used by this source contract."""
    match = re.search(rf"pub struct {name}\s*\{{(?P<body>.*?)\n\}}", source, re.DOTALL)
    require(match is not None, f"{name} declaration is missing")
    return match.group("body")


def validate_source_contract() -> None:
    """Require flat-arena invariants and reject recursive/XML/runtime surfaces."""
    source = source_text()
    tree = TREE_PATH.read_text(encoding="utf-8")
    for required in (
        "pub struct MetadataNodeInput", "parent_index: Option<usize>", "name: String",
        "value: Option<String>", "pub struct MetadataNode", "pub struct MetadataTreeLimits",
        "max_nodes: usize", "max_depth: usize", "max_children_per_node: usize",
        "max_name_code_points: usize", "max_value_code_points: usize", "pub struct MetadataTree",
        "nodes: Vec<MetadataNode>", "EmptyArena", "RootHasParent", "ExtraRoot",
        "ParentOutOfRange", "ParentIsSelf", "ParentIsForward", "NodeLimitExceeded",
        "DepthLimitExceeded", "ChildLimitExceeded", "EmptyName", "TextLimitExceeded",
        ".chars().count()", "depths[parent_index] + 1",
    ):
        require(required in tree, f"CORE-004 source invariant is missing: {required}")
    require("#![forbid(unsafe_code)]" in source, "unsafe Rust is not forbidden")
    require("#![deny(missing_docs)]" in source, "missing documentation is not denied")

    for name in ("MetadataNodeInput", "MetadataNode"):
        body = struct_body(tree, name)
        require("MetadataNode" not in body, f"{name} recursively owns a metadata node")
        require(not re.search(r"(?m)^\s*pub\s+\w+\s*:", body), f"{name} fields must remain private")
        require(not re.search(r"\b(?:Box|Rc|Arc|Vec|LinkedList|VecDeque)\s*<", body),
                f"{name} gained recursive or collection ownership")
    tree_body = struct_body(tree, "MetadataTree")
    require(not re.search(r"(?m)^\s*pub\s+\w+\s*:", tree_body), "MetadataTree fields must remain private")
    require(not re.search(r"\b(?:Box|Rc|Arc)\s*<\s*MetadataNode", tree), "metadata nodes gained recursive ownership")

    prohibited_imports = {
        "external ABI": re.compile(r'\bextern\s+"'), "network API": re.compile(r"\b(?:std|core)::net\b"),
        "thread API": re.compile(r"\bstd::thread\b"), "process API": re.compile(r"\bstd::process\b"),
        "clock API": re.compile(r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b"),
        "FFI API": re.compile(r"\b(?:std|core)::ffi\b"), "filesystem API": re.compile(r"\bstd::fs\b"),
        "environment API": re.compile(r"\bstd::env\b"), "I/O API": re.compile(r"\bstd::io\b"),
        "synchronization API": re.compile(r"\bstd::sync\b"), "unsafe block": re.compile(r"(?m)^\s*unsafe\s*\{"),
    }
    for label, pattern in prohibited_imports.items():
        require(not pattern.search(source), f"CORE-004 opened a prohibited {label}")

    declarations = "\n".join(
        line for line in tree.splitlines()
        if re.match(r"\s*(?:pub\s+(?:struct|enum|fn|mod|trait)|pub\s+const\s+fn)", line)
    )
    prohibited_surface = re.compile(
        r"(?i)\b(?:xml|xpath|query|parser|serializ|namespace|attribute|escape|entity|schema|"
        r"discover|resolv|recover|socket|network|inlet|outlet|transport|buffer|queue|thread|ffi|"
        r"adapter|authority|admission|permission|route|runtime|protocol|wire|clock|provider)\w*\b"
    )
    match = prohibited_surface.search(declarations)
    require(match is None, f"CORE-004 opened prohibited public surface: {match.group(0) if match else ''}")


def validate_inert_closure_and_instructions() -> None:
    """Reject dependency/activation drift and enforce scoped instruction completion."""
    result = subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    packages = json.loads(result.stdout).get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    require(packages[0].get("dependencies") == [], "CORE-004 permits no Cargo dependency")
    require(packages[0].get("features") == {}, "CORE-004 permits no Cargo feature")
    require(packages[0].get("publish") == [], "CORE-004 package must remain unpublished")

    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock default activation drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect-union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union must remain empty")

    unit = load_object(UNIT_PATH)
    require(unit.get("status") in {"active", "validating", "accepted"}, "CORE-004 lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list), "CORE-004 instruction surfaces are missing")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces if isinstance(surface, dict)}
    require(statuses.get("AGENTS.md") == "complete", "AGENTS.md instruction row is not complete")
    require(statuses.get("README.md") == "complete", "README.md instruction row is not complete")
    require(statuses.get("<skills-root>/system-engineering/SKILL.md") == "complete",
            "system-engineering instruction review is not complete")


def validate_docs_and_license() -> None:
    """Require public limitations, no overclaim, and AGPL provenance."""
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    normalized = " ".join("\n".join(path.read_text(encoding="utf-8") for path in paths).split())
    for phrase in (
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
        "parent-before-child", "exactly one root", "root depth is one", "optional values",
        "Unicode scalar", "no recursive", "local Rust contract tests",
    ):
        require(phrase in normalized, f"required CORE-004 limitation is missing: {phrase}")
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
    for path in (TREE_PATH, ROOT / "tools" / "check_core_004.py", ROOT / "tools" / "check_core_004.ps1"):
        require("SPDX-License-Identifier: AGPL-3.0-or-later" in path.read_text(encoding="utf-8"),
                f"license drift in {path.relative_to(ROOT)}")


def main() -> int:
    """Run all deterministic CORE-004 checks."""
    validate_overlay_and_tests()
    validate_historical_baseline()
    validate_source_contract()
    validate_inert_closure_and_instructions()
    validate_docs_and_license()
    print("CORE-004 bounded metadata tree contract checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
