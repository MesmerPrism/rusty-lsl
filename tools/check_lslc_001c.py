#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free LSLC-001C character-data representation."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates" / "rusty-lsl" / "src" / "xml_character_data.rs"
LIB = ROOT / "crates" / "rusty-lsl" / "src" / "lib.rs"
OVERLAY = ROOT / "fixtures" / "compatibility" / "lslc-001c-contract-results.json"
CORPUS = ROOT / "fixtures" / "compatibility" / "lslc-001a-stream-info-document-corpus.json"
LSLC_001B_OVERLAY = ROOT / "fixtures" / "compatibility" / "lslc-001b-contract-results.json"
UNIT = ROOT / "morphospace" / "iteration-units" / "rlsl-lslc-001c-xml-character-data-representation.json"
EXPECTED_TESTS = {
    "positive": {
        "lslc_001c_close_delimiter_and_reference_like_text_are_represented_literally",
        "lslc_001c_consuming_access_preserves_output_allocation",
        "lslc_001c_empty_and_each_fixed_escape_are_exact",
        "lslc_001c_quotes_apostrophes_whitespace_and_unicode_are_preserved",
        "lslc_001c_source_is_unchanged_and_reusable",
    },
    "damaged": {
        "lslc_001c_encoded_byte_bounds_are_exact",
        "lslc_001c_invalid_limit_and_error_precedence_are_stable",
    },
}
PRESERVED = {
    "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json":
        "68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c",
    "fixtures/compatibility/lslc-001b-contract-results.json":
        "67ad7f72a1ef8c6474234de36a6864aa049b6e59170a17a7b4745f1ee51cd1b9",
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


def struct_body(source: str, name: str) -> str:
    match = re.search(rf"pub\s+struct\s+{re.escape(name)}\s*\{{(.*?)\}}", source, re.DOTALL)
    require(match is not None, f"missing accepted value: {name}")
    return match.group(1) if match else ""


def validate_source_contract() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]

    for name, fields in {
        "XmlCharacterDataLimit": {"max_encoded_bytes: usize"},
        "XmlCharacterData": {"limit: XmlCharacterDataLimit", "encoded: String"},
    }.items():
        body = struct_body(implementation, name)
        require(not re.search(r"(?m)^\s*pub(?:\([^)]*\))?\s+", body),
                f"accepted fields must remain private: {name}")
        require(fields == {line.strip().rstrip(",") for line in body.splitlines() if line.strip()},
                f"accepted field shape drifted: {name}")

    for required in (
        "pub struct XmlCharacterDataLimit", "pub struct XmlCharacterData",
        "pub enum XmlCharacterDataError", "pub const fn new(max_encoded_bytes: usize)",
        "text: &XmlText", "pub const fn max_encoded_bytes", "pub const fn limit",
        "pub fn as_str", "pub fn into_string", "LengthOverflow",
        "LimitExceeded", "expected_max: usize", "required: usize",
        "AllocationFailed", "requested: usize", ".checked_add(length)",
        ".try_reserve_exact(requested)", "reserve_exact(&mut encoded, required)",
        "debug_assert_eq!(encoded.len(), required)",
    ):
        require(required in implementation, f"required LSLC-001C source invariant is missing: {required}")
    require("reserve_exact(&mut encoded, usize::MAX)" in source,
            "allocation-failure mapping is not exercised by the focused tests")

    replacements = {
        "const AMPERSAND: &str = \"&amp;\";": "'&' => encoded.push_str(AMPERSAND)",
        "const LESS_THAN: &str = \"&lt;\";": "'<' => encoded.push_str(LESS_THAN)",
        "const GREATER_THAN: &str = \"&gt;\";": "'>' => encoded.push_str(GREATER_THAN)",
    }
    for constant, write_arm in replacements.items():
        require(constant in implementation and write_arm in implementation,
                f"fixed replacement drifted: {constant}")

    length_index = implementation.index("let required = encoded_length(text.as_str())?;")
    limit_index = implementation.index("if required > limit.max_encoded_bytes")
    reserve_index = implementation.index("reserve_exact(&mut encoded, required)")
    write_index = implementation.index("for character in text.as_str().chars()")
    require(length_index < limit_index < reserve_index < write_index,
            "required LengthOverflow -> LimitExceeded -> AllocationFailed -> write order drifted")
    require("XmlText::new" not in implementation and "text.into_string" not in implementation,
            "borrowed XmlText must not be consumed or revalidated")

    public_lines = "\n".join(
        line for line in implementation.splitlines()
        if re.match(r"\s*pub\s+(?:struct|enum|fn|mod|trait)", line)
    )
    prohibited_surface = re.compile(
        r"(?i)\b(?:element|attribute|declaration|comment|processing|cdata|parse|parser|"
        r"decode|entity|document|namespace|schema|query|canonical|metadata|protocol|wire|"
        r"discovery|transport|clock|inlet|outlet|runtime|adapter|provider|ffi)\w*\b"
    )
    match = prohibited_surface.search(public_lines)
    require(match is None, f"LSLC-001C opened prohibited public surface: {match.group(0) if match else ''}")
    for pattern, label in (
        (r"(?m)^\s*unsafe\s*\{", "unsafe block"),
        (r"\b(?:std|core)::(?:fs|io|net|process|thread|ffi|sync|time)\b", "effect API"),
        (r"\bextern\s+\"", "external ABI"),
    ):
        require(re.search(pattern, implementation) is None, f"LSLC-001C opened prohibited {label}")

    lib = LIB.read_text(encoding="utf-8")
    require("mod xml_character_data;" in lib, "focused character-data module is not private")
    require("pub mod xml_character_data" not in lib, "character-data module must remain private")
    for public_type in ("XmlCharacterData", "XmlCharacterDataError", "XmlCharacterDataLimit"):
        require(public_type in lib, f"facade re-export is missing: {public_type}")


def validate_overlay_and_preservation() -> None:
    overlay = load_object(OVERLAY)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "overlay schema drifted")
    require(overlay.get("overlay_id") == "lslc-001c-xml-character-data-representation", "overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-lslc-001c-xml-character-data-representation", "unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "evidence level drifted")

    binding = overlay.get("corpus_binding", {})
    require(binding.get("corpus_id") == "lslc-001a-stream-info-document-corpus", "corpus binding drifted")
    require(binding.get("case_ids") == ["spec-xml-character-data-handling"],
            "overlay must bind only the accepted character-data role")
    require(binding.get("preserved_oracle_status") == "not-observed", "oracle role was promoted")
    require(binding.get("preserved_candidate_status") == "not-observed", "candidate role was promoted")

    value_binding = overlay.get("validated_value_binding", {})
    require(value_binding.get("overlay_id") == "lslc-001b-xml-name-text-contracts", "LSLC-001B binding drifted")
    require(value_binding.get("contract_id") == "bounded-xml-legal-text", "XmlText contract binding drifted")
    require(value_binding.get("public_input_type") == "XmlText", "validated input type drifted")

    policy = overlay.get("candidate_policy", {})
    require(policy.get("owner") == "rusty-lsl", "candidate policy owner drifted")
    require(policy.get("classification") == "local-candidate-policy", "candidate policy classification drifted")
    require(policy.get("observed_liblsl_behavior") is False, "local policy was promoted to observed behavior")
    require(policy.get("replacements") == {"&": "&amp;", "<": "&lt;", ">": "&gt;"},
            "overlay replacement policy drifted")

    source = SOURCE.read_text(encoding="utf-8")
    declared = set(re.findall(r"(?m)^\s*fn\s+(lslc_001c_[a-z0-9_]+)\s*\(", source))
    results = overlay.get("contract_results")
    require(isinstance(results, list) and len(results) == 1, "contract result identity drifted")
    result = results[0]
    require(result.get("contract_id") == "bounded-xml-character-data-representation", "contract id drifted")
    require(result.get("result") == "implemented-local-contract", "local result drifted")
    for role in ("positive", "damaged"):
        actual = set(result.get(f"{role}_tests", []))
        require(actual == EXPECTED_TESTS[role], f"{role} test identities drifted")
        require(actual <= declared, f"overlay references missing {role} tests")
    require(declared == EXPECTED_TESTS["positive"] | EXPECTED_TESTS["damaged"],
            "focused LSLC-001C test inventory drifted")

    provenance = overlay.get("provenance", {})
    require(provenance.get("origin_classification") == "independently-authored", "origin drifted")
    require(provenance.get("license_expression") == "AGPL-3.0-or-later", "license drifted")
    require(provenance.get("implementation_inputs") == [], "prohibited implementation input entered")
    require(set(provenance.get("technical_specification_inputs", [])) == {
        "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json",
        "fixtures/compatibility/lslc-001b-contract-results.json",
    }, "technical input boundary drifted")

    corpus = load_object(CORPUS)
    for case in corpus.get("cases", []):
        require(case.get("oracle_observation") == {"status": "not-observed", "evidence": None},
                f"LSLC-001A oracle role changed: {case.get('case_id')}")
        require(case.get("candidate_result") == {"status": "not-observed", "evidence": None},
                f"LSLC-001A candidate role changed: {case.get('case_id')}")
    prior = load_object(LSLC_001B_OVERLAY)
    require(prior.get("overlay_id") == "lslc-001b-xml-name-text-contracts", "LSLC-001B overlay drifted")
    for relative, expected in PRESERVED.items():
        require(hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == expected,
                f"historical evidence changed: {relative}")


def validate_tests_docs_instructions_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "+1.80.0", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "xml_character_data::tests::lslc_001c_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("7 passed" in result.stdout, "focused LSLC-001C Rust tests did not all pass")

    unit = load_object(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list) and len(surfaces) == 4, "instruction rows drifted")
    require(all(row.get("status") == "complete" for row in surfaces if isinstance(row, dict)),
            "instruction review is incomplete")

    required_docs = {
        "AGENTS.md": "LSLC-001C",
        "README.md": "XmlCharacterData",
        "docs/ARCHITECTURE.md": "XmlCharacterDataLimit",
        "docs/COMPATIBILITY.md": "LSLC-001C",
        "docs/CORPUS.md": "LSLC-001C",
        "docs/PROVENANCE.md": "lslc-001c-contract-results.json",
        "docs/VALIDATION.md": "check_lslc_001c.ps1",
        "fixtures/compatibility/README.md": "lslc-001c-contract-results.json",
        "morphospace/README.md": "rlsl-lslc-001c-xml-character-data-representation",
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
    print("LSLC-001C XML character-data representation checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
