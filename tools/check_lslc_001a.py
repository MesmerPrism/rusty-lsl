#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the LSLC-001A public-documentation specification corpus."""

from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
CORPUS_PATH = ROOT / "fixtures" / "compatibility" / "lslc-001a-stream-info-document-corpus.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-lslc-001a-stream-info-document-corpus.json"
EXPECTED_URLS = {
    "liblsl-stream-info-api-doc": "https://labstreaminglayer.readthedocs.io/projects/liblsl/ref/streaminfo.html",
    "w3c-xml-1-0-fifth-edition": "https://www.w3.org/TR/xml/",
}
EXPECTED_CLAIMS = {
    "lsl-stream-info-document-top-level",
    "lsl-stream-info-core-child-roles",
    "lsl-stream-info-extended-description-role",
    "lsl-stream-info-runtime-misc-role",
    "lsl-stream-name-required-nonempty",
    "lsl-stream-channel-count-positive",
    "xml-well-formed-document-role",
    "xml-legal-character-domain",
    "xml-name-grammar-role",
    "xml-character-data-escape-ampersand",
    "xml-character-data-escape-less-than",
    "xml-character-data-forbidden-close-delimiter",
}
POSITIVE_CASES = {
    "spec-info-top-level",
    "spec-core-child-roles",
    "spec-extended-desc-role",
    "spec-runtime-misc-separation",
    "spec-xml-legal-character-domain",
    "spec-xml-character-data-handling",
    "spec-bounded-input-policy",
}
DAMAGED_CASES = {
    "damaged-malformed-document",
    "damaged-illegal-xml-character",
    "damaged-invalid-element-name",
    "damaged-missing-stream-name",
    "damaged-nonpositive-channel-count",
    "damaged-duplicate-core-role",
    "damaged-excessive-depth",
    "damaged-excessive-node-count",
    "damaged-excessive-text",
}
PRESERVED_FILES = {
    "fixtures/compatibility/behavior-catalog.json": "e31d1ce0dceaec294ae932479815fa6072b35263f680df0ec5046dfc3ab602ee",
    "fixtures/compatibility/negative-case-matrix.json": "1f88d6557eed93525fc52c76024b7531c79acc95cfe28302529abe5d95e3a6a1",
    "fixtures/compatibility/baseline-provenance.json": "a71c13ef2dff7e4fa26abf5f6fe93ccc02a0c96349e950342970465955545ff9",
    "fixtures/compatibility/core-001-contract-results.json": "8e9a7bc0a6625c56c7da87e02d69e574304a48b329c93841984f19f9e44a3435",
    "fixtures/compatibility/core-002-contract-results.json": "c33517a3fb9c4b671ae7b56ed1fe6eb68adf855495ae54e0a332b528c6a273cd",
    "fixtures/compatibility/core-003-contract-results.json": "f64ed921b2576992a05ddd09bc02718b34455e0c441019d7bb8aee5e9fe60049",
    "fixtures/compatibility/core-004-contract-results.json": "664afa2f2d91ff7d1ed7355d31705fabdee5a80968a3eb15e2d9766388b2f1c6",
    "fixtures/compatibility/core-005-contract-results.json": "f56211fcb3abb50c1e170e112f338fad4791189f05c1866c01e4ffd8675abd0e",
    "fixtures/compatibility/core-006-contract-results.json": "3707cf05680c794c70141cf49acee7babf518b27488a4240e32e10414fd4e392",
    "fixtures/compatibility/core-007-contract-results.json": "8fb3b5f18a4ed95edb5d10bee097573e948d6b58c9d745540726416132a454ea",
    "fixtures/compatibility/core-008-contract-results.json": "8685b40dc5b3bb5ff68e3daf1c0d0be9daf4746aa52e6dad964eb2e7572f4d23",
}
EXPECTED_EFFECTS = {
    "activities", "assets", "commands", "inputs", "markers", "native_libraries",
    "permissions", "queries", "routes", "scenes", "services", "shaders", "streams", "tools",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load_object(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def validate_sources_and_claims(corpus: dict[str, Any]) -> None:
    require(corpus.get("schema") == "rusty.lsl.compatibility.stream_info_document_corpus.v1", "corpus schema drifted")
    require(corpus.get("corpus_id") == "lslc-001a-stream-info-document-corpus", "corpus identity drifted")
    require(corpus.get("unit_id") == "rlsl-lslc-001a-stream-info-document-corpus", "unit binding drifted")
    require(corpus.get("evidence_level") == "public-documentation-specification-only", "evidence level drifted")
    require(corpus.get("origin_classification") == "independently-authored", "origin classification drifted")
    require(corpus.get("access_date") == "2026-07-13", "corpus access date drifted")
    boundary = corpus.get("source_boundary")
    require(isinstance(boundary, dict), "source boundary is missing")
    require(boundary == {
        "source_code_used": False,
        "implementation_input": False,
        "endpoint_output_captured": False,
        "allowed_sources_only": True,
    }, "source boundary drifted")

    sources = corpus.get("sources")
    require(isinstance(sources, list) and len(sources) == 2, "exactly two documentation sources are required")
    by_id = {source.get("source_id"): source for source in sources if isinstance(source, dict)}
    require(set(by_id) == set(EXPECTED_URLS), "source inventory drifted")
    all_source_claims: set[str] = set()
    for source_id, url in EXPECTED_URLS.items():
        source = by_id[source_id]
        require(source.get("url") == url, f"source URL drifted: {source_id}")
        require(source.get("access_date") == "2026-07-13", f"access date drifted: {source_id}")
        require(source.get("role") == "public-documentation", f"source role drifted: {source_id}")
        require(source.get("source_code_used") is False, f"source code entered corpus: {source_id}")
        require(source.get("implementation_input") is False, f"implementation input entered corpus: {source_id}")
        for field in ("title", "publisher", "authority"):
            require(isinstance(source.get(field), str) and source[field].strip(), f"{field} is missing: {source_id}")
        claim_ids = source.get("extracted_claim_ids")
        require(isinstance(claim_ids, list) and len(claim_ids) == len(set(claim_ids)), f"source claims must be unique: {source_id}")
        all_source_claims.update(claim_ids)
    require(all_source_claims == EXPECTED_CLAIMS, "source claim inventory drifted")

    claims = corpus.get("extracted_claims")
    require(isinstance(claims, list), "extracted claims are missing")
    claim_by_id = {claim.get("claim_id"): claim for claim in claims if isinstance(claim, dict)}
    require(set(claim_by_id) == EXPECTED_CLAIMS, "extracted claim records drifted")
    for claim_id, claim in claim_by_id.items():
        require(claim.get("source_id") in EXPECTED_URLS, f"claim source is invalid: {claim_id}")
        summary = claim.get("independent_summary")
        require(isinstance(summary, str) and summary.strip(), f"claim summary is missing: {claim_id}")


def validate_cases_and_serialization(corpus: dict[str, Any]) -> None:
    roles = corpus.get("document_roles")
    require(isinstance(roles, dict), "document role inventory is missing")
    require(roles.get("ordering_semantics") == "none-recorded", "document roles assert ordering")
    require(roles.get("top_level") == "info", "top-level document role drifted")
    require(set(roles.get("core_children", [])) == {
        "name", "type", "channel_count", "nominal_srate", "channel_format", "source_id",
    }, "core child role inventory drifted")
    require(roles.get("extended_description") == "desc", "extended-description role drifted")
    require(set(roles.get("runtime_misc_children", [])) == {
        "version", "created_at", "uid", "session_id", "v4address", "v4data_port",
        "v4service_port", "v6address", "v6data_port", "v6service_port",
    }, "runtime/misc role inventory drifted")

    bounds = corpus.get("bounded_input_policy")
    require(isinstance(bounds, dict), "bounded-input policy is missing")
    require(bounds.get("policy_owner") == "rusty-lsl", "bound ownership drifted")
    require({key: bounds.get(key) for key in (
        "max_document_characters", "max_depth", "max_nodes",
        "max_element_name_characters", "max_character_data_characters_per_node",
    )} == {
        "max_document_characters": 65536,
        "max_depth": 16,
        "max_nodes": 256,
        "max_element_name_characters": 128,
        "max_character_data_characters_per_node": 4096,
    }, "bounded-input values drifted")

    resolution = corpus.get("serialization_resolution")
    require(isinstance(resolution, dict), "serialization resolution is missing")
    require(resolution.get("status") == "unresolved", "serialization was resolved without an oracle unit")
    require(resolution.get("next_authority") == "separately-approved-black-box-oracle-unit", "serialization authority drifted")
    require(set(resolution.get("unresolved_dimensions", [])) == {
        "bytes", "element-order", "whitespace", "empty-element-form",
        "numeric-spelling", "channel-format-wire-spelling",
    }, "unresolved serialization dimensions drifted")

    cases = corpus.get("cases")
    require(isinstance(cases, list), "cases are missing")
    by_id = {case.get("case_id"): case for case in cases if isinstance(case, dict)}
    require(set(by_id) == POSITIVE_CASES | DAMAGED_CASES, "case inventory drifted")
    require(len(by_id) == len(cases), "case identifiers must be unique")
    for case_id, case in by_id.items():
        require(set(case) == {"case_id", "case_class", "specification", "oracle_observation", "candidate_result"}, f"case roles drifted: {case_id}")
        expected_class = "positive-specification" if case_id in POSITIVE_CASES else "damaged-specification"
        require(case.get("case_class") == expected_class, f"case class drifted: {case_id}")
        specification = case.get("specification")
        require(isinstance(specification, dict), f"specification is missing: {case_id}")
        require(set(specification) == {"claim_ids", "role"}, f"specification shape drifted: {case_id}")
        require(set(specification.get("claim_ids", [])) <= EXPECTED_CLAIMS, f"unknown claim binding: {case_id}")
        require(isinstance(specification.get("role"), str) and specification["role"].strip(), f"specification role is empty: {case_id}")
        for evidence_role in ("oracle_observation", "candidate_result"):
            evidence = case.get(evidence_role)
            require(evidence == {"status": "not-observed", "evidence": None}, f"{evidence_role} invents evidence: {case_id}")

    character_data_role = by_id["spec-xml-character-data-handling"]["specification"]["role"]
    require("not emitted unchanged" in character_data_role, "CDATA close-delimiter representation boundary drifted")
    require("no escaping, splitting, or rejection policy is selected" in character_data_role, "corpus selected an implementation policy")

    serialized = json.dumps(cases, sort_keys=True).lower()
    for prohibited in ("input_xml", "output_xml", "serialized_bytes", "oracle_bytes", "candidate_bytes"):
        require(prohibited not in serialized, f"case inventory contains exact serialization material: {prohibited}")
    limitations = set(corpus.get("does_not_prove", []))
    for limitation in (
        "XML parsing or serialization implementation",
        "exact liblsl serialization",
        "official liblsl behavior",
        "candidate behavior",
        "wire or ecosystem compatibility",
        "runtime support",
    ):
        require(limitation in limitations, f"corpus limitation is missing: {limitation}")


def validate_preserved_history_and_source() -> None:
    for relative, expected in PRESERVED_FILES.items():
        path = ROOT / relative
        require(path.is_file(), f"preserved file is missing: {relative}")
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        require(actual == expected, f"preserved file changed: {relative}")

    catalog = load_object(ROOT / "fixtures" / "compatibility" / "behavior-catalog.json")
    for case in catalog.get("cases", []):
        require(case.get("current_result") == "not-implemented", f"STRM-000 history promoted: {case.get('case_id')}")
        require(case.get("measured_result") == {"status": "not-implemented", "observation": None}, f"STRM-000 observation changed: {case.get('case_id')}")


def validate_inert_closure() -> None:
    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout)
    packages = metadata.get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package inventory drifted")
    require(packages[0].get("dependencies") == [], "dependency closure is not empty")
    require(packages[0].get("features") == {}, "Cargo features are not empty")
    lib_source = (ROOT / "crates" / "rusty-lsl" / "src" / "lib.rs").read_text(encoding="utf-8")
    require("#![forbid(unsafe_code)]" in lib_source, "unsafe-code prohibition drifted")

    project = load_object(ROOT / "morphospace" / "project.spec.json")
    composition = project.get("composition")
    require(isinstance(composition, dict), "project composition is missing")
    require(composition.get("selected_features") == [] and composition.get("selected_modules") == [], "project activates a feature or module")
    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock default drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union is not empty")


def validate_instruction_and_docs() -> None:
    unit = load_object(UNIT_PATH)
    require(unit.get("status") in {"active", "validating", "accepted"}, "LSLC-001A lifecycle state is invalid")
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list) and len(surfaces) == 4, "instruction rows drifted")
    require(all(surface.get("status") == "complete" for surface in surfaces if isinstance(surface, dict)), "instruction review is incomplete")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces if isinstance(surface, dict)}
    for path in ("AGENTS.md", "README.md", "<skills-root>/system-engineering/SKILL.md", "<skills-root>/rusty-morphospace-context/SKILL.md"):
        require(statuses.get(path) == "complete", f"instruction surface is incomplete: {path}")

    required_text = {
        "AGENTS.md": "LSLC-001A",
        "README.md": "LSLC-001A",
        "docs/COMPATIBILITY.md": "LSLC-001A",
        "docs/PROVENANCE.md": "LSLC-001A",
        "docs/VALIDATION.md": "check_lslc_001a.ps1",
        "fixtures/compatibility/README.md": "lslc-001a-stream-info-document-corpus.json",
        "morphospace/README.md": "rlsl-lslc-001a-stream-info-document-corpus",
        "docs/CORPUS.md": "separately approved clean black-box oracle unit",
    }
    for relative, marker in required_text.items():
        require(marker in (ROOT / relative).read_text(encoding="utf-8"), f"documentation route is missing: {relative}")


def main() -> int:
    corpus = load_object(CORPUS_PATH)
    validate_sources_and_claims(corpus)
    validate_cases_and_serialization(corpus)
    validate_preserved_history_and_source()
    validate_inert_closure()
    validate_instruction_and_docs()
    print("LSLC-001A corpus checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
