#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free LSLC-001B XML value contracts."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates" / "rusty-lsl" / "src" / "xml_value.rs"
LIB = ROOT / "crates" / "rusty-lsl" / "src" / "lib.rs"
OVERLAY = ROOT / "fixtures" / "compatibility" / "lslc-001b-contract-results.json"
CORPUS = ROOT / "fixtures" / "compatibility" / "lslc-001a-stream-info-document-corpus.json"
UNIT = ROOT / "morphospace" / "iteration-units" / "rlsl-lslc-001b-xml-name-text-contracts.json"
EXPECTED_TESTS = {
    "bounded-xml-legal-text": {
        "positive": {
            "lslc_001b_text_char_production_boundaries",
            "lslc_001b_text_empty_delimiters_and_allocation_are_preserved",
            "lslc_001b_text_noncharacter_distinction_is_exact",
        },
        "damaged": {"lslc_001b_text_limits_indexes_and_precedence_are_scalar_based"},
    },
    "bounded-xml-element-name": {
        "positive": {
            "lslc_001b_colon_is_accepted_as_syntax_only",
            "lslc_001b_name_start_range_boundaries",
        },
        "damaged": {
            "lslc_001b_name_char_additional_boundaries_and_adjacent_failures",
            "lslc_001b_name_limits_indexes_precedence_and_allocation_are_exact",
            "lslc_001b_name_start_adjacent_failures",
        },
    },
}
XML_CHAR_TABLE = {
    (0x9, 0x9), (0xA, 0xA), (0xD, 0xD), (0x20, 0xD7FF),
    (0xE000, 0xFFFD), (0x10000, 0x10FFFF),
}
NAME_START_TABLE = {
    (0x3A, 0x3A), (0x41, 0x5A), (0x5F, 0x5F), (0x61, 0x7A),
    (0xC0, 0xD6), (0xD8, 0xF6), (0xF8, 0x2FF), (0x370, 0x37D),
    (0x37F, 0x1FFF), (0x200C, 0x200D), (0x2070, 0x218F),
    (0x2C00, 0x2FEF), (0x3001, 0xD7FF), (0xF900, 0xFDCF),
    (0xFDF0, 0xFFFD), (0x10000, 0xEFFFF),
}
NAME_CHAR_ADDITIONS = {
    (0x2D, 0x2D), (0x2E, 0x2E), (0x30, 0x39), (0xB7, 0xB7),
    (0x300, 0x36F), (0x203F, 0x2040),
}
PRESERVED = {
    "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json":
        "68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c",
    "fixtures/compatibility/behavior-catalog.json":
        "e31d1ce0dceaec294ae932479815fa6072b35263f680df0ec5046dfc3ab602ee",
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


def function_body(source: str, name: str) -> str:
    match = re.search(rf"const\s+fn\s+{re.escape(name)}\b.*?\{{(.*?)\n\}}", source, re.DOTALL)
    require(match is not None, f"missing range function: {name}")
    return match.group(1) if match else ""


def range_table(body: str) -> set[tuple[int, int]]:
    compact = re.sub(r"\s+", "", body)
    tokens = re.findall(r"0x[0-9A-Fa-f]+(?:\.\.=0x[0-9A-Fa-f]+)?", compact)
    table: set[tuple[int, int]] = set()
    for token in tokens:
        parts = token.split("..=")
        start = int(parts[0], 16)
        end = int(parts[-1], 16)
        table.add((start, end))
    return table


def validate_range_tables_and_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    require(range_table(function_body(source, "is_xml_char")) == XML_CHAR_TABLE,
            "XML Char range table drifted")
    require(range_table(function_body(source, "is_name_start_char")) == NAME_START_TABLE,
            "NameStartChar range table drifted")
    name_char = function_body(source, "is_name_char")
    require("is_name_start_char(character)" in name_char,
            "NameChar no longer includes the complete NameStartChar table")
    require(range_table(name_char) == NAME_CHAR_ADDITIONS, "NameChar addition table drifted")

    for declaration, fields in {
        "XmlText": {"limit: XmlTextLimit", "text: String"},
        "XmlElementName": {"limit: XmlNameLimit", "name: String"},
    }.items():
        match = re.search(rf"pub\s+struct\s+{declaration}\s*\{{(.*?)\}}", source, re.DOTALL)
        require(match is not None, f"accepted value is missing: {declaration}")
        body = match.group(1) if match else ""
        require(not re.search(r"(?m)^\s*pub(?:\([^)]*\))?\s+", body),
                f"accepted fields must remain private: {declaration}")
        require(fields <= {line.strip().rstrip(",") for line in body.splitlines()},
                f"accepted field shape drifted: {declaration}")

    for required in (
        "pub struct XmlTextLimit", "pub struct XmlNameLimit", "pub struct XmlText",
        "pub struct XmlElementName", "pub enum XmlTextError", "pub enum XmlNameError",
        "pub const fn max_code_points", "pub const fn limit", "pub fn as_str",
        "pub fn into_string", "text.chars().count()", "name.chars().count()",
        "IllegalCharacter", "InvalidStart", "InvalidContinuation", "code_point: character as u32",
    ):
        require(required in source, f"required LSLC-001B source invariant is missing: {required}")

    public_lines = "\n".join(
        line for line in source.splitlines()
        if re.match(r"\s*pub\s+(?:struct|enum|fn|mod|trait)", line)
    )
    prohibited_surface = re.compile(
        r"(?i)\b(?:parse|parser|serializ|escape|entity|reference|document|output|writer|"
        r"namespace|attribute|schema|query|protocol|wire|transport|runtime|socket|ffi)\w*\b"
    )
    match = prohibited_surface.search(public_lines)
    require(match is None, f"LSLC-001B opened prohibited public surface: {match.group(0) if match else ''}")
    for pattern, label in (
        (r"(?m)^\s*unsafe\s*\{", "unsafe block"),
        (r"\b(?:std|core)::(?:fs|io|net|process|thread|ffi|sync|time)\b", "effect API"),
        (r"\bextern\s+\"", "external ABI"),
    ):
        require(re.search(pattern, source) is None, f"LSLC-001B opened prohibited {label}")

    lib = LIB.read_text(encoding="utf-8")
    require("mod xml_value;" in lib, "focused XML value module is not private")
    require("pub mod xml_value" not in lib, "XML value implementation module must remain private")
    for public_type in (
        "XmlElementName", "XmlNameError", "XmlNameLimit", "XmlText", "XmlTextError", "XmlTextLimit",
    ):
        require(public_type in lib, f"facade re-export is missing: {public_type}")


def validate_overlay_and_corpus() -> None:
    overlay = load_object(OVERLAY)
    require(overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1", "overlay schema drifted")
    require(overlay.get("overlay_id") == "lslc-001b-xml-name-text-contracts", "overlay identity drifted")
    require(overlay.get("unit_id") == "rlsl-lslc-001b-xml-name-text-contracts", "unit binding drifted")
    require(overlay.get("evidence_level") == "local-rust-contract-tests", "evidence level drifted")
    binding = overlay.get("corpus_binding", {})
    require(binding.get("corpus_id") == "lslc-001a-stream-info-document-corpus", "corpus binding drifted")
    require(binding.get("preserved_oracle_status") == "not-observed", "oracle role was promoted")
    require(binding.get("preserved_candidate_status") == "not-observed", "candidate role was promoted")

    source = SOURCE.read_text(encoding="utf-8")
    declared = set(re.findall(r"(?m)^\s*fn\s+(lslc_001b_[a-z0-9_]+)\s*\(", source))
    results = overlay.get("contract_results")
    require(isinstance(results, list), "contract results are missing")
    by_id = {result.get("contract_id"): result for result in results if isinstance(result, dict)}
    require(set(by_id) == set(EXPECTED_TESTS), "contract result identities drifted")
    for contract_id, expected in EXPECTED_TESTS.items():
        result = by_id[contract_id]
        require(result.get("result") == "implemented-local-contract", f"result drifted: {contract_id}")
        positive = set(result.get("positive_tests", []))
        damaged = set(result.get("damaged_tests", []))
        require(positive == expected["positive"], f"positive tests drifted: {contract_id}")
        require(damaged == expected["damaged"], f"damaged tests drifted: {contract_id}")
        require(positive | damaged <= declared, f"overlay references missing tests: {contract_id}")

    provenance = overlay.get("provenance", {})
    require(provenance.get("origin_classification") == "independently-authored", "origin drifted")
    require(provenance.get("implementation_inputs") == [], "prohibited implementation input entered")
    require(set(provenance.get("technical_specification_inputs", [])) == {
        "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json",
        "https://www.w3.org/TR/xml/",
    }, "technical specification inputs drifted")

    corpus = load_object(CORPUS)
    cases = corpus.get("cases")
    require(isinstance(cases, list), "LSLC-001A cases are missing")
    for case in cases:
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
         "xml_value::tests::lslc_001b_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("9 passed" in result.stdout, "focused LSLC-001B Rust tests did not all pass")

    unit = load_object(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"}, "unit lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list) and len(surfaces) == 4, "instruction rows drifted")
    require(all(row.get("status") == "complete" for row in surfaces if isinstance(row, dict)),
            "instruction review is incomplete")

    required_docs = {
        "AGENTS.md": "LSLC-001B",
        "README.md": "LSLC-001B",
        "docs/ARCHITECTURE.md": "XmlElementName",
        "docs/COMPATIBILITY.md": "LSLC-001B",
        "docs/CORPUS.md": "LSLC-001B",
        "docs/PROVENANCE.md": "lslc-001b-contract-results.json",
        "docs/VALIDATION.md": "check_lslc_001b.ps1",
        "fixtures/compatibility/README.md": "lslc-001b-contract-results.json",
        "morphospace/README.md": "rlsl-lslc-001b-xml-name-text-contracts",
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
    validate_range_tables_and_source()
    validate_overlay_and_corpus()
    validate_tests_docs_instructions_and_closure()
    print("LSLC-001B XML name/text contract checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
