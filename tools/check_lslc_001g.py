#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate bounded borrowed LSLC-001G element-tree serialization."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/xml_element_serialization.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001g-contract-results.json"
CORPUS = ROOT / "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json"
UNIT = ROOT / "morphospace/iteration-units/rlsl-lslc-001g-bounded-xml-element-tree-serialization.json"
POSITIVE = {
    "lslc_001g_borrowed_serialization_preserves_source_components_and_allocations",
    "lslc_001g_core_004_through_lslc_001f_composes_into_serialization",
    "lslc_001g_deep_tree_is_iterative_and_bounded",
    "lslc_001g_exact_limit_borrow_and_consuming_output_preserve_allocation",
    "lslc_001g_leaf_root_and_empty_container_use_explicit_tags",
    "lslc_001g_non_preorder_arena_serializes_depth_first_with_index_ordered_siblings",
    "lslc_001g_unicode_colon_and_character_data_are_emitted_verbatim_once",
}
DAMAGED = {
    "lslc_001g_checked_length_overflow_retains_node_index",
    "lslc_001g_one_past_output_limit_reports_exact_required_bytes",
    "lslc_001g_output_allocation_failure_retains_requested_bytes",
    "lslc_001g_traversal_stack_allocation_failure_retains_requested_count",
    "lslc_001g_zero_limit_has_stable_counts",
}
PRESERVED = {
    "baseline-provenance.json": "a71c13ef2dff7e4fa26abf5f6fe93ccc02a0c96349e950342970465955545ff9",
    "behavior-catalog.json": "e31d1ce0dceaec294ae932479815fa6072b35263f680df0ec5046dfc3ab602ee",
    "core-001-contract-results.json": "8e9a7bc0a6625c56c7da87e02d69e574304a48b329c93841984f19f9e44a3435",
    "core-002-contract-results.json": "c33517a3fb9c4b671ae7b56ed1fe6eb68adf855495ae54e0a332b528c6a273cd",
    "core-003-contract-results.json": "f64ed921b2576992a05ddd09bc02718b34455e0c441019d7bb8aee5e9fe60049",
    "core-004-contract-results.json": "664afa2f2d91ff7d1ed7355d31705fabdee5a80968a3eb15e2d9766388b2f1c6",
    "core-005-contract-results.json": "f56211fcb3abb50c1e170e112f338fad4791189f05c1866c01e4ffd8675abd0e",
    "core-006-contract-results.json": "3707cf05680c794c70141cf49acee7babf518b27488a4240e32e10414fd4e392",
    "core-007-contract-results.json": "8fb3b5f18a4ed95edb5d10bee097573e948d6b58c9d745540726416132a454ea",
    "core-008-contract-results.json": "8685b40dc5b3bb5ff68e3daf1c0d0be9daf4746aa52e6dad964eb2e7572f4d23",
    "lslc-001a-stream-info-document-corpus.json": "68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c",
    "lslc-001b-contract-results.json": "67ad7f72a1ef8c6474234de36a6864aa049b6e59170a17a7b4745f1ee51cd1b9",
    "lslc-001c-contract-results.json": "01d03280c76c9bd08476564e20ad7a80513e17bdb0ec56fbbedfc728b35ce7a3",
    "lslc-001d-contract-results.json": "8eb7256810956983bcf9763eaf31a8d47f2f7fc40000fc4ce1801bc5d7997ca7",
    "lslc-001e-contract-results.json": "32a0d2bb2a83bb84bc126e142996fb56c5bee744b86fb7e2753d69f72f99a9f3",
    "lslc-001f-contract-results.json": "22887579e874624ec13fb68365ccdb835d40bc42249c55053fcedb3606476438",
    "negative-case-matrix.json": "1f88d6557eed93525fc52c76024b7531c79acc95cfe28302529abe5d95e3a6a1",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def validate_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = LIB.read_text(encoding="utf-8")
    for marker in (
        "pub struct XmlElementSerializationLimit",
        "pub struct XmlElementSerialization",
        "pub enum XmlElementSerializationError",
        "pub fn serialize(",
        "source: &XmlElementTree",
        "LengthOverflow",
        "OutputLimitExceeded",
        "TraversalStackAllocationFailed",
        "OutputAllocationFailed",
        "output.push_str(leaf.character_data().as_str())",
        "build_traversal_stack(source, &mut stack)",
        "for child_index in 1..source.nodes().len()",
    ):
        require(marker in implementation, f"serialization invariant missing: {marker}")
    require("mod xml_element_serialization;" in lib, "private module route missing")
    require("pub mod xml_element_serialization" not in lib, "serialization module became public")
    require(re.search(
        r"pub struct XmlElementSerializationLimit\s*\{\s*max_output_bytes:\s*usize,\s*\}",
        implementation,
    ) is not None, "limit must contain exactly one private byte maximum")
    require(re.search(
        r"pub struct XmlElementSerialization\s*\{\s*limit:\s*"
        r"XmlElementSerializationLimit,\s*output:\s*String,\s*\}", implementation,
    ) is not None, "accepted state must contain exactly limit and String")
    require(implementation.count("reserve_traversal_stack(&mut stack, requested_stack)?") == 1,
            "traversal stack must be reserved exactly once")
    require(implementation.count("build_traversal_stack(source, &mut stack);") == 1,
            "traversal links must be indexed exactly once")
    require(implementation.count("reserve_output(&mut output, required)?") == 1,
            "output String must be reserved exactly once")
    require(implementation.count("let mut stack = Vec::new();") == 1,
            "serializer must own exactly one traversal scratch Vec")
    require(implementation.count("let mut output = String::new();") == 1,
            "serializer must own exactly one output String")
    require(implementation.count("Vec::new()") == 1,
            "additional traversal scratch allocation opened")
    require(implementation.count("String::new()") == 1,
            "additional output String allocation opened")
    body = implementation[implementation.index("pub fn serialize("):
                          implementation.index("/// Returns the selected output-byte limit.")]
    ordered = [
        "let required = exact_output_bytes(source.nodes())?",
        "if required > limit.max_output_bytes",
        "reserve_traversal_stack(&mut stack, requested_stack)?",
        "build_traversal_stack(source, &mut stack);",
        "reserve_output(&mut output, required)?",
    ]
    positions = [body.index(marker) for marker in ordered]
    require(positions == sorted(positions), "length/limit/allocation precedence drifted")
    require("clone" not in body.lower(), "serializer must not clone source state")
    require("write!" not in body and "format!" not in body,
            "hidden formatting allocation or spelling policy opened")
    require("/>" not in implementation, "self-closing element spelling opened")
    require("(1..source.nodes().len()).rev()" not in implementation,
            "quadratic per-container arena rescanning returned")
    require(re.search(r"\b(?:Self|XmlElementSerialization)::serialize\s*\(", body) is None,
            "recursive serialization call opened")
    tests = set(re.findall(r"fn (lslc_001g_[a-z0-9_]+)\(", source))
    require(tests == POSITIVE | DAMAGED, "focused Rust test inventory drifted")


def validate_overlay_and_history() -> None:
    overlay = load(OVERLAY)
    require(overlay.get("overlay_id") == "lslc-001g-bounded-xml-element-tree-serialization",
            "overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-lslc-001g-bounded-xml-element-tree-serialization",
            "unit binding drifted")
    results = overlay.get("contract_results", [])
    require(len(results) == 1, "overlay must contain exactly one contract result")
    require(set(results[0].get("positive_tests", [])) == POSITIVE, "positive tests drifted")
    require(set(results[0].get("damaged_tests", [])) == DAMAGED, "damaged tests drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "implementation inputs entered")
    for case in load(CORPUS).get("cases", []):
        require(case.get("oracle_observation") == {"status": "not-observed", "evidence": None},
                f"oracle role changed: {case.get('case_id')}")
        require(case.get("candidate_result") == {"status": "not-observed", "evidence": None},
                f"candidate role changed: {case.get('case_id')}")
    for name, expected in PRESERVED.items():
        actual = hashlib.sha256((ROOT / "fixtures/compatibility" / name).read_bytes()).hexdigest()
        require(actual == expected, f"historical evidence changed: {name}")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "+1.80.0", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "xml_element_serialization::tests::lslc_001g_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("12 passed" in result.stdout, "focused LSLC-001G tests did not all pass")
    unit = load(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit state invalid")
    require(all(row.get("status") == "complete" for row in unit.get("instruction_surfaces", [])),
            "instruction review incomplete")
    docs = {
        "AGENTS.md": "check_lslc_001g.ps1",
        "README.md": "XmlElementSerialization",
        "docs/ARCHITECTURE.md": "xml_element_serialization",
        "docs/COMPATIBILITY.md": "LSLC-001G",
        "docs/CORPUS.md": "LSLC-001G",
        "docs/PROVENANCE.md": "lslc-001g-contract-results.json",
        "docs/VALIDATION.md": "check_lslc_001g.ps1",
        "fixtures/compatibility/README.md": "check_lslc_001g.ps1",
        "morphospace/README.md": "rlsl-lslc-001g-bounded-xml-element-tree-serialization",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"),
                f"documentation route missing: {path}")
    metadata = json.loads(subprocess.run(
        ["cargo", "+1.80.0", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
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
    validate_overlay_and_history()
    validate_execution_docs_and_closure()
    print("LSLC-001G bounded XML element-tree serialization checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
