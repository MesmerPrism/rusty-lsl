#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free LSLC-001E container/leaf hierarchy."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/xml_element_tree.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001e-contract-results.json"
CORPUS = ROOT / "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json"
UNIT = ROOT / "morphospace/iteration-units/rlsl-lslc-001e-xml-container-leaf-tree.json"
POSITIVE = {
    "lslc_001e_exact_limits_preserve_hierarchy_and_component_values",
    "lslc_001e_original_vector_and_component_allocations_are_preserved",
    "lslc_001e_root_may_be_a_leaf_but_a_leaf_cannot_parent",
}
DAMAGED = {
    "lslc_001e_allocation_and_overflow_helpers_return_typed_errors",
    "lslc_001e_depth_and_child_one_past_errors_follow_parent_kind",
    "lslc_001e_empty_node_and_root_parent_precedence_is_stable",
    "lslc_001e_parent_identity_errors_are_distinct_and_indexed",
    "lslc_001e_retained_utf8_byte_bound_is_exact_and_last",
    "lslc_001e_zero_limits_reject_in_argument_order",
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
    for marker in (
        "pub enum XmlElementTreeBound", "pub struct XmlElementTreeLimits",
        "pub enum XmlElementNodeValue", "pub struct XmlElementNodeInput",
        "pub struct XmlElementTree", "pub enum XmlElementTreeError",
        "nodes: Vec<XmlElementNodeInput>", ".try_reserve_exact(requested)",
        "ScratchAllocationFailed", "RetainedBytesOverflow",
        "RetainedBytesLimitExceeded", "LeafParent",
    ):
        require(marker in implementation, f"source invariant missing: {marker}")
    owning_types = ("XmlElementNodeValue", "XmlElementNodeInput", "XmlElementTree")
    for owner in owning_types:
        derived = rf"#\[derive\((?=[^\]]*\bClone\b)[^\]]*\)\]\s*pub (?:struct|enum) {owner}\b"
        manual = rf"\bimpl\s+(?:(?:core|std)::clone::)?Clone\s+for\s+{owner}\b"
        require(re.search(derived, implementation) is None, f"{owner} must not derive Clone")
        require(re.search(manual, implementation) is None, f"{owner} must not implement Clone")
    require(
        re.search(
            r"pub struct XmlElementTree\s*\{\s*"
            r"limits:\s*XmlElementTreeLimits,\s*"
            r"nodes:\s*Vec<XmlElementNodeInput>,\s*\}",
            implementation,
        )
        is not None,
        "accepted tree state must contain only private limits and the original node arena",
    )
    require("pub fn nodes(&self) -> &[XmlElementNodeInput]" in implementation,
            "read-only arena accessor missing")
    require(not re.search(r"pub\s+fn\s+\w+[^\n]*&mut", implementation),
            "mutable public access opened")
    require(implementation.count("let mut scratch = Vec::new();") == 1,
            "validation must use one scratch Vec")
    ordered = [
        "if nodes.is_empty()", "if nodes.len() > limits.max_nodes",
        "if let Some(parent_index) = nodes[0].parent_index",
        "reserve_scratch(&mut scratch, nodes.len())?", "for node_index in 1..nodes.len()",
        "let retained_bytes = retained_bytes(&nodes)?",
        "if retained_bytes > limits.max_retained_bytes",
    ]
    positions = [implementation.index(item) for item in ordered]
    require(positions == sorted(positions), "top-level rejection precedence drifted")
    loop = implementation[positions[4]:positions[5]]
    inner = ["ExtraRoot", "ParentOutOfRange", "ParentIsSelf", "ParentIsForward",
             "LeafParent", "DepthLimitExceeded", "ChildLimitExceeded"]
    inner_positions = [loop.index(item) for item in inner]
    require(inner_positions == sorted(inner_positions), "per-node precedence drifted")
    require("mod xml_element_tree;" in LIB.read_text(encoding="utf-8"), "private module route missing")
    tests = set(re.findall(r"fn (lslc_001e_[a-z0-9_]+)\(", source))
    require(tests == POSITIVE | DAMAGED, "focused Rust test inventory drifted")


def validate_overlay_and_history() -> None:
    overlay = load(OVERLAY)
    require(overlay.get("overlay_id") == "lslc-001e-xml-container-leaf-tree", "overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-lslc-001e-xml-container-leaf-tree", "unit binding drifted")
    result = overlay.get("contract_results", [{}])[0]
    require(set(result.get("positive_tests", [])) == POSITIVE, "positive tests drifted")
    require(set(result.get("damaged_tests", [])) == DAMAGED, "damaged tests drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [], "implementation inputs entered")
    for case in load(CORPUS).get("cases", []):
        require(case.get("oracle_observation") == {"status": "not-observed", "evidence": None},
                f"oracle role changed: {case.get('case_id')}")
        require(case.get("candidate_result") == {"status": "not-observed", "evidence": None},
                f"candidate role changed: {case.get('case_id')}")
    base = ROOT / "fixtures/compatibility"
    for name, expected in PRESERVED.items():
        require(hashlib.sha256((base / name).read_bytes()).hexdigest() == expected,
                f"historical evidence changed: {name}")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "+1.80.0", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "xml_element_tree::tests::lslc_001e_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("9 passed" in result.stdout, "focused LSLC-001E tests did not all pass")
    unit = load(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit state invalid")
    require(all(row.get("status") == "complete" for row in unit.get("instruction_surfaces", [])),
            "instruction review incomplete")
    docs = {
        "AGENTS.md": "check_lslc_001e.ps1", "README.md": "XmlElementTreeLimits",
        "docs/ARCHITECTURE.md": "xml_element_tree", "docs/COMPATIBILITY.md": "LSLC-001E",
        "docs/CORPUS.md": "LSLC-001E", "docs/PROVENANCE.md": "lslc-001e-contract-results.json",
        "docs/VALIDATION.md": "check_lslc_001e.ps1",
        "fixtures/compatibility/README.md": "check_lslc_001e.ps1",
        "morphospace/README.md": "rlsl-lslc-001e-xml-container-leaf-tree",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    metadata = json.loads(subprocess.run(
        ["cargo", "+1.80.0", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout)
    package = metadata["packages"][0]
    require(len(metadata["packages"]) == 1 and package["dependencies"] == [], "dependency closure drifted")
    require(package["features"] == {} and package["publish"] == [], "feature/publication closure drifted")
    lock = load(ROOT / "morphospace/feature.lock.json")
    require(lock.get("features") == [] and lock.get("selected_features") == [], "feature lock drifted")
    require(all(not value for value in lock.get("effect_union", {}).values()), "effect closure is not inert")


def main() -> int:
    validate_source()
    validate_overlay_and_history()
    validate_execution_docs_and_closure()
    print("LSLC-001E XML container/leaf hierarchy checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
