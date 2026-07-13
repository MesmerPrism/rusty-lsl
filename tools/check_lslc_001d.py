#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free LSLC-001D leaf-element composition."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates" / "rusty-lsl" / "src" / "xml_leaf_element.rs"
LIB = ROOT / "crates" / "rusty-lsl" / "src" / "lib.rs"
OVERLAY = ROOT / "fixtures" / "compatibility" / "lslc-001d-contract-results.json"
CORPUS = ROOT / "fixtures" / "compatibility" / "lslc-001a-stream-info-document-corpus.json"
UNIT = ROOT / "morphospace" / "iteration-units" / "rlsl-lslc-001d-xml-leaf-element-composition.json"
TESTS = {
    "positive": {
        "lslc_001d_empty_character_data_and_colon_name_are_preserved",
        "lslc_001d_into_parts_preserves_both_owned_allocations",
        "lslc_001d_unicode_and_represented_reference_text_are_preserved",
    },
    "damaged": {
        "lslc_001d_damaged_character_data_remains_representation_authority_rejection",
        "lslc_001d_damaged_name_remains_name_authority_rejection",
    },
}
PRESERVED = {
    "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json":
        "68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c",
    "fixtures/compatibility/lslc-001b-contract-results.json":
        "67ad7f72a1ef8c6474234de36a6864aa049b6e59170a17a7b4745f1ee51cd1b9",
    "fixtures/compatibility/lslc-001c-contract-results.json":
        "01d03280c76c9bd08476564e20ad7a80513e17bdb0ec56fbbedfc728b35ce7a3",
    "fixtures/compatibility/behavior-catalog.json":
        "e31d1ce0dceaec294ae932479815fa6072b35263f680df0ec5046dfc3ab602ee",
    "fixtures/compatibility/baseline-provenance.json":
        "a71c13ef2dff7e4fa26abf5f6fe93ccc02a0c96349e950342970465955545ff9",
    "fixtures/compatibility/negative-case-matrix.json":
        "1f88d6557eed93525fc52c76024b7531c79acc95cfe28302529abe5d95e3a6a1",
    "fixtures/compatibility/core-001-contract-results.json":
        "8e9a7bc0a6625c56c7da87e02d69e574304a48b329c93841984f19f9e44a3435",
    "fixtures/compatibility/core-002-contract-results.json":
        "c33517a3fb9c4b671ae7b56ed1fe6eb68adf855495ae54e0a332b528c6a273cd",
    "fixtures/compatibility/core-003-contract-results.json":
        "f64ed921b2576992a05ddd09bc02718b34455e0c441019d7bb8aee5e9fe60049",
    "fixtures/compatibility/core-004-contract-results.json":
        "664afa2f2d91ff7d1ed7355d31705fabdee5a80968a3eb15e2d9766388b2f1c6",
    "fixtures/compatibility/core-005-contract-results.json":
        "f56211fcb3abb50c1e170e112f338fad4791189f05c1866c01e4ffd8675abd0e",
    "fixtures/compatibility/core-006-contract-results.json":
        "3707cf05680c794c70141cf49acee7babf518b27488a4240e32e10414fd4e392",
    "fixtures/compatibility/core-007-contract-results.json":
        "8fb3b5f18a4ed95edb5d10bee097573e948d6b58c9d745540726416132a454ea",
    "fixtures/compatibility/core-008-contract-results.json":
        "8685b40dc5b3bb5ff68e3daf1c0d0be9daf4746aa52e6dad964eb2e7572f4d23",
}
EXPECTED_EFFECTS = {
    "activities", "assets", "commands", "inputs", "markers", "native_libraries",
    "permissions", "queries", "routes", "scenes", "services", "shaders",
    "streams", "tools",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load_object(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def brace_body(source: str, marker_pattern: str) -> str:
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


def validate_source_contract() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    body = brace_body(implementation, r"pub\s+struct\s+XmlLeafElement\b")
    fields = dict(re.findall(r"(?m)^\s*([a-z][a-z0-9_]*)\s*:\s*([^,\n]+)\s*,\s*$", body))
    require(fields == {
        "name": "XmlElementName",
        "character_data": "XmlCharacterData",
    }, "XmlLeafElement must own exactly the two accepted components")
    require(not re.search(r"(?m)^\s*pub(?:\([^)]*\))?\s+", body),
            "XmlLeafElement fields must remain private")
    require(re.search(
        r"#\[derive\((?=[^\]]*\bClone\b)[^\]]*\)\]\s*"
        r"pub\s+struct\s+XmlLeafElement\b",
        implementation,
    ) is None, "XmlLeafElement must not expose allocation-cloning Clone")

    constructor = brace_body(
        implementation,
        r"pub\s+fn\s+new\s*\(\s*name\s*:\s*XmlElementName\s*,\s*"
        r"character_data\s*:\s*XmlCharacterData\s*\)\s*->\s*Self",
    )
    require(re.sub(r"\s+", "", constructor) == "Self{name,character_data,}",
            "constructor must move both accepted values directly into state")
    for forbidden in (
        "Result<", "Error", "Limit", ".clone(", "to_owned(", "String::", "Vec::",
        "collect(", "with_capacity(", "XmlElementName::new", "XmlCharacterData::encode",
    ):
        require(forbidden not in constructor, f"constructor contains prohibited operation: {forbidden}")

    signatures = (
        r"pub\s+const\s+fn\s+name\s*\(\s*&self\s*\)\s*->\s*&XmlElementName",
        r"pub\s+const\s+fn\s+character_data\s*\(\s*&self\s*\)\s*->\s*&XmlCharacterData",
        r"pub\s+fn\s+into_parts\s*\(\s*self\s*\)\s*->\s*"
        r"\(\s*XmlElementName\s*,\s*XmlCharacterData\s*\)",
    )
    for signature in signatures:
        require(re.search(signature, implementation) is not None,
                f"LSLC-001D accessor is missing: {signature}")

    public_names = set(re.findall(
        r"(?m)^pub\s+(?:const\s+)?(?:struct|enum|type|fn)\s+([A-Za-z_][A-Za-z0-9_]*)",
        implementation,
    ))
    require(public_names == {"XmlLeafElement"},
            f"LSLC-001D opened an extra public type or function: {sorted(public_names)}")
    for pattern, label in (
        (r"(?m)^\s*unsafe\s*\{", "unsafe block"),
        (r"\b(?:std|core)::(?:fs|io|net|process|thread|ffi|sync|time)\b", "effect API"),
        (r"\bextern\s+\"", "external ABI"),
    ):
        require(re.search(pattern, implementation) is None, f"LSLC-001D opened prohibited {label}")

    lib = LIB.read_text(encoding="utf-8")
    require("mod xml_leaf_element;" in lib and "pub mod xml_leaf_element" not in lib,
            "leaf-element module must remain private")
    require("pub use xml_leaf_element::XmlLeafElement;" in lib,
            "XmlLeafElement facade export is missing")
    declared = set(re.findall(r"(?m)^\s*fn\s+(lslc_001d_[a-z0-9_]+)\s*\(", source))
    require(declared == TESTS["positive"] | TESTS["damaged"],
            "focused LSLC-001D test inventory drifted")


def validate_overlay_and_preservation() -> None:
    overlay = load_object(OVERLAY)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "overlay schema drifted")
    require(overlay.get("overlay_id") == "lslc-001d-xml-leaf-element-composition", "overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-lslc-001d-xml-leaf-element-composition", "unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "evidence level drifted")

    binding = overlay.get("corpus_binding", {})
    require(binding.get("corpus_id") == "lslc-001a-stream-info-document-corpus", "corpus binding drifted")
    require(binding.get("case_ids") == [
        "spec-xml-character-data-handling", "damaged-invalid-element-name",
    ], "corpus case binding drifted")
    require(binding.get("preserved_oracle_status") == "not-observed", "oracle role was promoted")
    require(binding.get("preserved_candidate_status") == "not-observed", "candidate role was promoted")

    components = overlay.get("accepted_component_bindings")
    require(components == [
        {
            "overlay_id": "lslc-001b-xml-name-text-contracts",
            "contract_id": "bounded-xml-element-name",
            "public_input_type": "XmlElementName",
        },
        {
            "overlay_id": "lslc-001c-xml-character-data-representation",
            "contract_id": "bounded-xml-character-data-representation",
            "public_input_type": "XmlCharacterData",
        },
    ], "accepted component authority drifted")
    policy = overlay.get("candidate_policy", {})
    require(policy.get("classification") == "preserved-lslc-001c-local-candidate-policy",
            "LSLC-001C policy classification drifted")
    require(policy.get("observed_liblsl_behavior") is False,
            "candidate policy was promoted to observed behavior")

    results = overlay.get("contract_results")
    require(isinstance(results, list) and len(results) == 1, "contract result identity drifted")
    result = results[0]
    require(result.get("contract_id") == "xml-leaf-element-composition", "contract id drifted")
    require(result.get("result") == "implemented-local-contract", "local result drifted")
    for role in ("positive", "damaged"):
        require(set(result.get(f"{role}_tests", [])) == TESTS[role],
                f"{role} test identities drifted")

    provenance = overlay.get("provenance", {})
    require(provenance.get("origin_classification") == "independently-authored", "origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "license drifted")
    require(provenance.get("implementation_inputs") == [], "prohibited implementation input entered")
    require(set(provenance.get("technical_specification_inputs", [])) == {
        "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json",
        "fixtures/compatibility/lslc-001b-contract-results.json",
        "fixtures/compatibility/lslc-001c-contract-results.json",
    }, "technical input boundary drifted")

    corpus = load_object(CORPUS)
    for case in corpus.get("cases", []):
        require(case.get("oracle_observation") == {"status": "not-observed", "evidence": None},
                f"LSLC-001A oracle role changed: {case.get('case_id')}")
        require(case.get("candidate_result") == {"status": "not-observed", "evidence": None},
                f"LSLC-001A candidate role changed: {case.get('case_id')}")
    for relative, expected in PRESERVED.items():
        require(hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == expected,
                f"historical evidence changed: {relative}")


def validate_tests_docs_instructions_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "+1.80.0", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "xml_leaf_element::tests::lslc_001d_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("5 passed" in result.stdout, "focused LSLC-001D Rust tests did not all pass")

    unit = load_object(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list) and len(surfaces) == 4, "instruction rows drifted")
    require(all(row.get("status") == "complete" for row in surfaces if isinstance(row, dict)),
            "instruction review is incomplete")

    required_docs = {
        "AGENTS.md": "LSLC-001D",
        "README.md": "XmlLeafElement",
        "docs/ARCHITECTURE.md": "leaf-only",
        "docs/COMPATIBILITY.md": "LSLC-001D",
        "docs/CORPUS.md": "LSLC-001D",
        "docs/PROVENANCE.md": "lslc-001d-contract-results.json",
        "docs/VALIDATION.md": "check_lslc_001d.ps1",
        "fixtures/compatibility/README.md": "lslc-001d-contract-results.json",
        "morphospace/README.md": "rlsl-lslc-001d-xml-leaf-element-composition",
    }
    for relative, marker in required_docs.items():
        require(marker in (ROOT / relative).read_text(encoding="utf-8"),
                f"documentation route is missing: {relative}")

    metadata = json.loads(subprocess.run(
        ["cargo", "+1.80.0", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout)
    packages = metadata.get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    require(packages[0].get("dependencies") == [], "dependency closure is not empty")
    require(packages[0].get("features") == {}, "Cargo features are not empty")
    require(packages[0].get("publish") == [], "package must remain unpublished")
    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect closure shape drifted")
    require(all(value == [] for value in effects.values()), "effect closure is not inert")


def main() -> int:
    validate_source_contract()
    validate_overlay_and_preservation()
    validate_tests_docs_instructions_and_closure()
    print("LSLC-001D XML leaf-element composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
