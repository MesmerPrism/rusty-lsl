# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

from __future__ import annotations

import hashlib
import json
import re
import runpy
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_description_xml.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
CASES = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json"
OBSERVATIONS = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json"
F_OVERLAY = ROOT / "fixtures/compatibility/lslc-001f-contract-results.json"
G_OVERLAY = ROOT / "fixtures/compatibility/lslc-001g-contract-results.json"
M_OVERLAY = ROOT / "fixtures/compatibility/lslc-001m-static-stream-info-xml-results.json"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001n-description-xml-results.json"
BOUND_SHA256 = {
    CASES: "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17",
    OBSERVATIONS: "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e",
    F_OVERLAY: "22887579e874624ec13fb68365ccdb835d40bc42249c55053fcedb3606476438",
    G_OVERLAY: "4819c8cea026c534140104c8416cddb6dd9130c3bd433857e367aed1ff7d0e74",
    M_OVERLAY: "f95d4a1387d6291bee930bbb7893c0e00ec83a3791c82958885bb74c618baad3",
}
TESTS = {
    "lslc_001n_seven_observed_cases_place_desc_after_static_fields_exactly",
    "lslc_001n_root_contract_rejects_leaf_and_non_desc_container",
    "lslc_001n_none_and_some_empty_survive_remap_with_exact_parent_order",
    "lslc_001n_target_node_bound_rejects_before_merged_allocation",
    "lslc_001n_component_allocations_move_and_consuming_tree_preserves_arena",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def validate_dependencies() -> None:
    for path, expected in BOUND_SHA256.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == expected,
                f"accepted dependency changed: {path.name}")
    for checker, functions in [
        ("tools/check_lslc_001f.py", ("validate_source", "validate_overlay_and_history")),
        ("tools/check_lslc_001g.py", ("validate_source", "validate_overlay_and_history")),
        ("tools/check_lslc_001m.py", ("validate_source", "validate_accepted_dependencies", "validate_overlay")),
    ]:
        historical = runpy.run_path(str(ROOT / checker))
        for function in functions:
            historical[function]()


def validate_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = LIB.read_text(encoding="utf-8")
    require("mod stream_info_description_xml;" in lib, "private description XML module route missing")
    require("pub mod stream_info_description_xml" not in lib, "description XML module became public")
    for marker in (
        "const STATIC_NODE_COUNT: usize = 7;",
        'const DESCRIPTION_ROOT_NAME: &str = "desc";',
        "DescriptionRootNotContainer",
        "DescriptionRootNameMismatch",
        "static_count.checked_add(description_count)",
        "if total > limits.max_nodes()",
        "merged.try_reserve_exact(total)",
        "static_count.checked_add(parent_index)",
        "XmlElementTree::new(limits, merged)",
    ):
        require(marker in implementation, f"description composition invariant missing: {marker}")
    for forbidden in ("clone()", "to_owned()", "to_string()", "unsafe", "recursive"):
        require(forbidden not in implementation, f"forbidden ownership/runtime surface opened: {forbidden}")
    require(implementation.index("root_name.as_str() != DESCRIPTION_ROOT_NAME")
            < implementation.index("merged.try_reserve_exact(total)"),
            "explicit desc admission must precede merged allocation")
    require(implementation.index("if total > limits.max_nodes()")
            < implementation.index("merged.try_reserve_exact(total)"),
            "target node bound must precede merged allocation")
    tests = set(re.findall(r"fn (lslc_001n_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused LSLC-001N Rust test inventory drifted")


def validate_overlay() -> None:
    cases = load(CASES).get("positive_cases", [])
    observations_doc = load(OBSERVATIONS)
    observations = observations_doc.get("observations", [])
    overlay = load(OVERLAY)
    results = overlay.get("candidate_description_results", [])
    require(observations_doc.get("candidate_result") == {"status":"not-observed","evidence":None},
            "accepted full-document candidate role changed")
    bindings = overlay.get("accepted_artifact_bindings", {})
    for key, path in {
        "case_manifest": CASES, "observation_overlay": OBSERVATIONS,
        "metadata_projection_overlay": F_OVERLAY, "serialization_overlay": G_OVERLAY,
        "static_xml_overlay": M_OVERLAY,
    }.items():
        require(bindings.get(key, {}).get("sha256") == BOUND_SHA256[path],
                f"overlay binding drifted: {key}")
    require(len(cases) == len(observations) == len(results) == 7,
            "exact seven-case description matrix required")
    for case, observation, result in zip(cases, observations, results):
        case_id = case.get("case_id")
        require(case_id == observation.get("case_id") == result.get("case_id"),
                f"case identity or order drifted: {case_id}")
        compact = result.get("compact_desc", "")
        require(compact.startswith("<desc>") and compact.endswith("</desc>"),
                f"description envelope drifted: {case_id}")
        require("\n" not in compact and "<desc />" not in compact,
                f"observed whitespace or self-closing policy leaked: {case_id}")
        expected_nonempty = bool(case.get("description"))
        require((compact != "<desc></desc>") == expected_nonempty,
                f"description emptiness drifted: {case_id}")
        if expected_nonempty:
            spellings = observation.get("observed_dimensions", {}).get(
                "character_data_spellings", {}).get("description_values", [])
            for spelling in spellings:
                require(spelling in compact, f"represented description value missing: {case_id}")
        require(result.get("result") == "implemented-local-description-element-composition",
                f"candidate result label drifted: {case_id}")
    policy = overlay.get("fixed_candidate_policy", {})
    require(policy.get("description_root_index") == 7
            and policy.get("static_predecessor") == "nominal_srate"
            and policy.get("empty_description_serialization") == "<desc></desc>",
            "fixed description policy drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "external implementation input entered")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_description_xml::tests::lslc_001n_"], cwd=ROOT,
        check=True, capture_output=True, text=True,
    )
    require("5 passed" in result.stdout, "focused LSLC-001N Rust tests did not all pass")
    docs = {
        "AGENTS.md":"check_lslc_001n.ps1", "README.md":"StreamInfoDescriptionXml",
        "docs/ARCHITECTURE.md":"stream_info_description_xml", "docs/COMPATIBILITY.md":"LSLC-001N",
        "docs/CORPUS.md":"LSLC-001N", "docs/PROVENANCE.md":"lslc-001n-description-xml-results.json",
        "docs/VALIDATION.md":"check_lslc_001n.ps1",
        "morphospace/README.md":"rlsl-lslc-001n-description-xml-composition",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    protected = ["Cargo.toml","Cargo.lock","crates/rusty-lsl/Cargo.toml",
                 "morphospace/feature.lock.json","morphospace/project.spec.json",
                 *[str(path.relative_to(ROOT)) for path in BOUND_SHA256]]
    status = subprocess.run(["git","status","--porcelain=v1","--",*protected], cwd=ROOT,
                            check=True, capture_output=True, text=True).stdout
    require(not status.strip(), "protected dependency, lock, or accepted fixture changed")
    metadata = json.loads(subprocess.run(
        ["cargo","metadata","--offline","--locked","--no-deps","--format-version","1"],
        cwd=ROOT, check=True, capture_output=True, text=True).stdout)
    package = metadata["packages"][0]
    require(len(metadata["packages"]) == 1 and package["dependencies"] == [], "dependency closure drifted")
    require(package["features"] == {} and package["publish"] == [], "feature/publication closure drifted")


def main() -> int:
    validate_source()
    validate_dependencies()
    validate_overlay()
    validate_execution_docs_and_closure()
    print("LSLC-001N bounded description XML composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
