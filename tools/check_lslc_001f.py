#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free LSLC-001F one-way metadata XML projection."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/metadata_xml_projection.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001f-contract-results.json"
CORPUS = ROOT / "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json"
UNIT = ROOT / "morphospace/iteration-units/rlsl-lslc-001f-metadata-to-xml-element-tree-projection.json"
POSITIVE = {
    "lslc_001f_childless_container_and_valued_leaf_root_are_accepted",
    "lslc_001f_consuming_tree_exposes_only_accepted_target_values",
    "lslc_001f_limits_have_no_hidden_default_policy",
    "lslc_001f_name_allocations_parent_identity_and_distinct_arena_are_preserved",
    "lslc_001f_none_and_some_including_empty_project_in_order",
}
DAMAGED = {
    "lslc_001f_first_child_of_value_bearing_parent_rejects_before_components",
    "lslc_001f_later_component_failure_preserves_caller_index",
    "lslc_001f_name_then_text_then_character_data_errors_are_indexed",
    "lslc_001f_output_allocation_helper_returns_typed_error_without_panicking",
    "lslc_001f_retained_bytes_delegation_sees_represented_data",
    "lslc_001f_target_hierarchy_errors_are_delegated_unchanged",
    "lslc_001f_target_node_bound_precedes_shape_and_allocation",
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
        "pub struct MetadataXmlProjectionLimits",
        "pub enum MetadataXmlProjectionError",
        "pub fn project_metadata_tree_to_xml_element_tree(",
        "source: MetadataTree",
        "limits: MetadataXmlProjectionLimits",
        ") -> Result<XmlElementTree, MetadataXmlProjectionError>",
        "OutputAllocationFailed",
        "ValueBearingParent",
        "XmlElementTreeError::NodeLimitExceeded",
        ".try_reserve_exact(requested)",
    ):
        require(marker in implementation, f"projection invariant missing: {marker}")
    require("mod metadata_xml_projection;" in lib, "private module route missing")
    require("pub mod metadata_xml_projection" not in lib, "projection module became public")
    require("TargetNodeLimitExceeded" not in implementation,
            "projection duplicated the target hierarchy's node-limit error authority")
    require(
        re.search(
            r"return Err\(MetadataXmlProjectionError::ElementTree\(\s*"
            r"XmlElementTreeError::NodeLimitExceeded\s*\{",
            implementation,
        ) is not None,
        "target node preflight must preserve the existing hierarchy error payload",
    )
    require(
        re.search(
            r"pub struct MetadataXmlProjectionLimits\s*\{\s*"
            r"name:\s*XmlNameLimit,\s*text:\s*XmlTextLimit,\s*"
            r"character_data:\s*XmlCharacterDataLimit,\s*"
            r"element_tree:\s*XmlElementTreeLimits,\s*\}",
            implementation,
        ) is not None,
        "projection limit state must contain exactly four private accepted limits",
    )
    require(implementation.count("pub fn project_metadata_tree_to_xml_element_tree(") == 1,
            "projection public function inventory drifted")
    require(implementation.count("let mut output = Vec::new();") == 1,
            "projection must allocate exactly one distinct output arena")
    require(implementation.count("reserve_output_nodes(&mut output, node_count)?") == 1,
            "projection must reserve the output arena once")
    projection_body = implementation[
        implementation.index("pub fn project_metadata_tree_to_xml_element_tree("):
        implementation.index("fn reserve_output_nodes(")
    ]
    require("clone" not in projection_body.lower(), "projection implementation must not clone")
    public_arguments = re.findall(
        r"pub\s+(?:const\s+)?fn\s+\w+\s*\((.*?)\)\s*(?:->[^\{]+)?\{",
        implementation,
        re.DOTALL,
    )
    require(all(re.search(r"&\s*MetadataTree", arguments) is None
                for arguments in public_arguments),
            "borrowed metadata projection surface opened")
    require(not re.search(r"\b(?:TryFrom|From)\s*<", implementation),
            "From/TryFrom projection surface opened")
    forbidden = (
        "xml_element_tree_to_metadata",
        "xml_to_metadata",
        "to_metadata_tree",
        "decode",
        "round_trip",
        "roundtrip",
        "default()",
    )
    for marker in forbidden:
        require(marker not in implementation.lower(), f"forbidden projection surface opened: {marker}")

    ordered = [
        "if node_count > limits.element_tree.max_nodes()",
        "for (first_child_index, node) in source.nodes().iter().enumerate().skip(1)",
        "reserve_output_nodes(&mut output, node_count)?",
        "for (node_index, node) in source.into_nodes().into_iter().enumerate()",
        "XmlElementName::new(limits.name, name)",
        "XmlText::new(limits.text, value)",
        "XmlCharacterData::encode(limits.character_data, &text)",
        "XmlElementTree::new(limits.element_tree, output)",
    ]
    positions = [implementation.index(marker) for marker in ordered]
    require(positions == sorted(positions), "projection rejection/delegation precedence drifted")

    owning_types = ("XmlElementNodeValue", "XmlElementNodeInput", "XmlElementTree")
    xml_sources = "\n".join(
        path.read_text(encoding="utf-8").split("#[cfg(test)]", 1)[0]
        for path in (ROOT / "crates/rusty-lsl/src").glob("xml_*.rs")
    )
    for owner in owning_types:
        derived = rf"#\[derive\((?=[^\]]*\bClone\b)[^\]]*\)\]\s*pub (?:struct|enum) {owner}\b"
        manual = rf"\bimpl\s+(?:(?:core|std)::clone::)?Clone\s+for\s+{owner}\b"
        require(re.search(derived, xml_sources) is None, f"{owner} must not derive Clone")
        require(re.search(manual, xml_sources) is None, f"{owner} must not implement Clone")
    require(not re.search(r"pub\s+fn\s+\w+[^\n]*&mut", xml_sources),
            "mutable owning XML access opened")
    tests = set(re.findall(r"fn (lslc_001f_[a-z0-9_]+)\(", source))
    require(tests == POSITIVE | DAMAGED, "focused Rust test inventory drifted")


def validate_overlay_and_history() -> None:
    overlay = load(OVERLAY)
    require(overlay.get("overlay_id") == "lslc-001f-metadata-to-xml-element-tree-projection",
            "overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-lslc-001f-metadata-to-xml-element-tree-projection",
            "unit binding drifted")
    results = overlay.get("contract_results", [])
    require(len(results) == 1, "overlay must contain exactly one contract result")
    require(set(results[0].get("positive_tests", [])) == POSITIVE, "positive tests drifted")
    require(set(results[0].get("damaged_tests", [])) == DAMAGED, "damaged tests drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "implementation inputs entered")
    require(overlay.get("projection", {}).get("direction") ==
            "one-way-only-with-no-decoding-or-round-trip-claim", "one-way policy drifted")
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
         "metadata_xml_projection::tests::lslc_001f_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("12 passed" in result.stdout, "focused LSLC-001F tests did not all pass")
    unit = load(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit state invalid")
    require(all(row.get("status") == "complete" for row in unit.get("instruction_surfaces", [])),
            "instruction review incomplete")
    docs = {
        "AGENTS.md": "check_lslc_001f.ps1",
        "README.md": "project_metadata_tree_to_xml_element_tree",
        "docs/ARCHITECTURE.md": "metadata_xml_projection",
        "docs/COMPATIBILITY.md": "LSLC-001F",
        "docs/CORPUS.md": "LSLC-001F",
        "docs/PROVENANCE.md": "lslc-001f-contract-results.json",
        "docs/VALIDATION.md": "check_lslc_001f.ps1",
        "fixtures/compatibility/README.md": "check_lslc_001f.ps1",
        "morphospace/README.md": "rlsl-lslc-001f-metadata-to-xml-element-tree-projection",
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
    require(len(package["targets"]) == 1 and "lib" in package["targets"][0]["kind"],
            "target closure drifted")
    lock = load(ROOT / "morphospace/feature.lock.json")
    require(lock.get("features") == [] and lock.get("selected_features") == [],
            "feature lock drifted")
    require(all(not value for value in lock.get("effect_union", {}).values()),
            "effect closure is not inert")


def main() -> int:
    validate_source()
    validate_overlay_and_history()
    validate_execution_docs_and_closure()
    print("LSLC-001F metadata-to-XML-element-tree projection checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
