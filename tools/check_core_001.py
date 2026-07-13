#!/usr/bin/env python3
"""Validate the dependency-free CORE-001 local contract implementation."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures" / "compatibility"
OVERLAY_PATH = FIXTURES / "core-001-contract-results.json"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
CASE_TESTS = {
    "contract-metadata-bounds": {
        "positive": {"contract_metadata_bounds_exact_limit_and_values_unchanged"},
        "damaged": {
            "contract_metadata_bounds_one_past_description_limit_rejected",
            "contract_metadata_bounds_one_past_field_limit_rejected",
            "contract_metadata_bounds_one_past_text_limit_rejected",
            "contract_metadata_bounds_zero_limits_rejected_deterministically",
        },
    },
    "contract-sample-shape": {
        "positive": {"contract_sample_shape_exact_limit_and_values_unchanged"},
        "damaged": {
            "contract_sample_shape_channel_mismatch_has_stable_payload",
            "contract_sample_shape_one_past_limit_rejected",
            "contract_sample_shape_zero_declared_channels_rejected",
            "contract_sample_shape_zero_limit_rejected",
        },
    },
}
EXPECTED_EFFECTS = {
    "activities",
    "assets",
    "commands",
    "inputs",
    "markers",
    "native_libraries",
    "permissions",
    "queries",
    "routes",
    "scenes",
    "services",
    "shaders",
    "streams",
    "tools",
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


def validate_overlay() -> None:
    """Bind the two accepted cases to exact positive and damaged Rust tests."""
    overlay = load_object(OVERLAY_PATH)
    require(
        overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1",
        "CORE-001 overlay schema drifted",
    )
    require(
        overlay.get("overlay_id") == "core-001-bounded-contract-model",
        "CORE-001 overlay identity drifted",
    )
    require(
        overlay.get("unit_id") == "rlsl-core-001-bounded-contract-model",
        "CORE-001 overlay unit binding drifted",
    )
    require(
        overlay.get("evidence_level") == "local-rust-contract-tests",
        "CORE-001 evidence level is inaccurate",
    )
    require(
        overlay.get("implementation_status") == "bounded-local-contracts",
        "CORE-001 implementation status is inaccurate",
    )
    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-001 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog binding drifted")
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    require(
        isinstance(binding.get("interpretation"), str) and binding["interpretation"].strip(),
        "CORE-001 baseline interpretation is missing",
    )

    results = overlay.get("case_results")
    require(isinstance(results, list), "CORE-001 case results must be a list")
    by_id = {result.get("case_id"): result for result in results if isinstance(result, dict)}
    require(set(by_id) == set(CASE_TESTS), "CORE-001 must bind exactly the two accepted case IDs")

    source = "\n".join(
        path.read_text(encoding="utf-8") for path in sorted(SOURCE_ROOT.glob("*.rs"))
    )
    declared_tests = set(re.findall(r"(?m)^\s*fn\s+([a-z0-9_]+)\s*\(", source))
    for case_id, expected in CASE_TESTS.items():
        result = by_id[case_id]
        require(result.get("result") == "implemented-local-contract", f"{case_id} status is inaccurate")
        positive = set(result.get("positive_tests", []))
        damaged = set(result.get("damaged_tests", []))
        require(positive == expected["positive"], f"{case_id} positive coverage drifted")
        require(damaged == expected["damaged"], f"{case_id} damaged coverage drifted")
        require(positive | damaged <= declared_tests, f"{case_id} references a missing Rust test")

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-001 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-001 origin drifted")
    require(
        provenance.get("license_expression") == "AGPL-3.0-or-later",
        "CORE-001 license expression drifted",
    )
    require(provenance.get("implementation_inputs") == [], "CORE-001 has prohibited implementation inputs")
    limitations = set(overlay.get("does_not_prove", []))
    require(
        {"LSL protocol behavior", "wire compatibility", "runtime support", "ecosystem compatibility"}
        <= limitations,
        "CORE-001 limitations are incomplete",
    )


def validate_historical_baseline() -> None:
    """Keep accepted STRM-000 case results specification-only and unmeasured."""
    catalog = load_object(CATALOG_PATH)
    require(catalog.get("evidence_level") == "specification-only", "STRM-000 evidence level drifted")
    cases = {
        case.get("case_id"): case
        for case in catalog.get("cases", [])
        if isinstance(case, dict)
    }
    for case_id in CASE_TESTS:
        case = cases.get(case_id)
        require(isinstance(case, dict), f"STRM-000 is missing {case_id}")
        require(case.get("current_result") == "not-implemented", f"{case_id} rewrote STRM-000 history")
        measured = case.get("measured_result")
        require(isinstance(measured, dict), f"{case_id} measured-result role is missing")
        require(
            measured.get("status") == "not-implemented" and measured.get("observation") is None,
            f"{case_id} invents a measured STRM-000 result",
        )


def validate_source_boundary() -> None:
    """Reject unsafe, protocol/runtime surface additions, and status drift."""
    source = "\n".join(
        path.read_text(encoding="utf-8") for path in sorted(SOURCE_ROOT.glob("*.rs"))
    )
    require("#![forbid(unsafe_code)]" in source, "unsafe Rust is not forbidden")
    require("#![deny(missing_docs)]" in source, "missing documentation is not denied")
    require(
        "ImplementationStatus::BoundedLocalContracts" in source,
        "public implementation status does not name CORE-001",
    )
    prohibited = {
        "unsafe block": re.compile(r"(?m)^\s*unsafe\s*\{"),
        "external ABI": re.compile(r"\bextern\s+\""),
        "network API": re.compile(r"\b(?:std|core)::net\b"),
        "thread API": re.compile(r"\bstd::thread\b"),
        "process API": re.compile(r"\bstd::process\b"),
        "clock API": re.compile(r"\b(?:std|core)::time\b"),
        "FFI API": re.compile(r"\b(?:std|core)::ffi\b"),
        "filesystem API": re.compile(r"\bstd::fs\b"),
        "environment API": re.compile(r"\bstd::env\b"),
        "I/O API": re.compile(r"\bstd::io\b"),
        "synchronization API": re.compile(r"\bstd::sync\b"),
    }
    for label, pattern in prohibited.items():
        require(not pattern.search(source), f"CORE-001 opened a prohibited {label}")


def validate_inert_closure() -> None:
    """Reject dependency, feature, package, or activation drift."""
    result = subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    metadata = json.loads(result.stdout)
    packages = metadata.get("packages", [])
    require(len(packages) == 1 and packages[0].get("name") == "rusty-lsl", "package closure drifted")
    package = packages[0]
    require(package.get("dependencies") == [], "CORE-001 permits no Cargo dependency")
    require(package.get("features") == {}, "CORE-001 permits no Cargo feature")
    require(package.get("publish") == [], "CORE-001 package must remain unpublished")

    project = load_object(ROOT / "morphospace" / "project.spec.json")
    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    composition = project.get("composition", {})
    require(composition.get("selected_features") == [], "project feature activation drifted")
    require(composition.get("selected_modules") == [], "project module activation drifted")
    require(composition.get("allowed_permissions") == [], "project permission activation drifted")
    require(lock.get("default_activation") == "disabled", "feature lock default activation drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect-union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union must remain empty")


def validate_no_overclaim() -> None:
    """Require exact disclaimers and reject unqualified positive broad claims."""
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    text = "\n".join(path.read_text(encoding="utf-8") for path in paths)
    normalized = " ".join(text.split())
    for phrase in (
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
        "local Rust contract semantics",
    ):
        require(phrase in normalized, f"required CORE-001 limitation is missing: {phrase}")
    positive = re.compile(
        r"(?i)\b(?:LSL\s+)?(?:protocol|wire|runtime|operational|ecosystem)\s+"
        r"(?:compatibility|support|behavior)\s+(?:is|has been)\s+"
        r"(?:implemented|supported|proven|verified|validated)\b"
    )
    negations = {"no", "not", "without", "isn't", "hasn't"}
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        for number, line in enumerate(lines, 1):
            if positive.search(line):
                context = " ".join(lines[max(0, number - 2) : number])
                words = set(re.findall(r"[a-z]+(?:n't)?", context.lower()))
                require(bool(words & negations), f"compatibility overclaim in {path.relative_to(ROOT)}:{number}")


def main() -> int:
    """Run all deterministic CORE-001 checks."""
    validate_overlay()
    validate_historical_baseline()
    validate_source_boundary()
    validate_inert_closure()
    validate_no_overclaim()
    print("CORE-001 bounded local contract checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
