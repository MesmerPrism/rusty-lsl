#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the dependency-free CORE-002 timestamped-chunk contracts."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE_ROOT = ROOT / "crates" / "rusty-lsl" / "src"
FIXTURES = ROOT / "fixtures" / "compatibility"
OVERLAY_PATH = FIXTURES / "core-002-contract-results.json"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
UNIT_PATH = ROOT / "morphospace" / "iteration-units" / "rlsl-core-002-timestamped-chunk-contracts.json"
EXPECTED_TESTS = {
    "raw-and-derived-timestamp-values": {
        "positive": {
            "core_002_raw_and_optional_derived_timestamps_remain_distinct",
            "core_002_raw_remains_unchanged_beside_each_derived_kind",
            "core_002_timestamp_values_preserve_finite_bits",
        },
        "damaged": {"core_002_non_finite_timestamps_have_stable_typed_errors"},
    },
    "bounded-timestamped-chunk": {
        "positive": {
            "core_002_chunk_exact_limits_preserve_order_values_and_timestamp_pairing",
            "core_002_empty_chunk_accepts_and_retains_valid_nonzero_limits",
        },
        "damaged": {
            "core_002_chunk_inconsistent_channel_shape_has_stable_error",
            "core_002_chunk_one_past_channel_limit_has_stable_error",
            "core_002_chunk_one_past_sample_limit_has_stable_error",
            "core_002_chunk_zero_limits_reject_in_argument_order",
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


def source_text() -> str:
    """Return the complete local Rust source surface in stable path order."""
    return "\n".join(path.read_text(encoding="utf-8") for path in sorted(SOURCE_ROOT.glob("*.rs")))


def owner_source_text() -> str:
    """Return only the CORE-002 timestamp and chunk owner module."""
    return (SOURCE_ROOT / "timestamped.rs").read_text(encoding="utf-8")


def validate_overlay() -> None:
    """Bind exact CORE-002 positive and damaged tests without promoting STRM-000."""
    overlay = load_object(OVERLAY_PATH)
    require(
        overlay.get("schema") == "rusty.lsl.contract_result_overlay.v1",
        "CORE-002 overlay schema drifted",
    )
    require(
        overlay.get("overlay_id") == "core-002-timestamped-chunk-contracts",
        "CORE-002 overlay identity drifted",
    )
    require(
        overlay.get("unit_id") == "rlsl-core-002-timestamped-chunk-contracts",
        "CORE-002 overlay unit binding drifted",
    )
    require(
        overlay.get("evidence_level") == "local-rust-contract-tests",
        "CORE-002 evidence level is inaccurate",
    )
    require(
        overlay.get("implementation_status") == "bounded-local-contracts",
        "CORE-002 implementation status is inaccurate",
    )

    binding = overlay.get("baseline_binding")
    require(isinstance(binding, dict), "CORE-002 baseline binding is missing")
    require(binding.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog binding drifted")
    require(
        binding.get("case_id") == "semantic-raw-timestamp-preserved",
        "CORE-002 must bind the accepted semantic timestamp case",
    )
    require(binding.get("preserved_result") == "not-implemented", "STRM-000 history was promoted")
    interpretation = binding.get("interpretation")
    require(
        isinstance(interpretation, str)
        and "specification-only" in interpretation
        and "local Rust" in interpretation,
        "CORE-002 baseline limitation is incomplete",
    )

    results = overlay.get("contract_results")
    require(isinstance(results, list), "CORE-002 contract results must be a list")
    by_id = {result.get("contract_id"): result for result in results if isinstance(result, dict)}
    require(set(by_id) == set(EXPECTED_TESTS), "CORE-002 contract-result identities drifted")
    declared_tests = set(re.findall(r"(?m)^\s*fn\s+([a-z0-9_]+)\s*\(", source_text()))
    for contract_id, expected in EXPECTED_TESTS.items():
        result = by_id[contract_id]
        require(
            result.get("result") == "implemented-local-contract",
            f"{contract_id} status is inaccurate",
        )
        positive = set(result.get("positive_tests", []))
        damaged = set(result.get("damaged_tests", []))
        require(positive == expected["positive"], f"{contract_id} positive coverage drifted")
        require(damaged == expected["damaged"], f"{contract_id} damaged coverage drifted")
        require(positive | damaged <= declared_tests, f"{contract_id} references a missing Rust test")

    provenance = overlay.get("provenance")
    require(isinstance(provenance, dict), "CORE-002 provenance is missing")
    require(provenance.get("origin_classification") == "independently-authored", "CORE-002 origin drifted")
    require(
        provenance.get("license_expression") == "AGPL-3.0-or-later",
        "CORE-002 license expression drifted",
    )
    require(provenance.get("implementation_inputs") == [], "CORE-002 has prohibited implementation inputs")
    limitations = set(overlay.get("does_not_prove", []))
    required = {
        "clock reading or clock correction",
        "clock-correction or smoothing algorithm behavior",
        "dejittering, smoothing, interpolation, or sample-rate timestamp derivation",
        "LSL protocol behavior",
        "sample or chunk transport",
        "wire compatibility",
        "runtime support",
        "ecosystem compatibility",
        "official liblsl behavior",
    }
    require(required <= limitations, "CORE-002 limitations are incomplete")


def validate_historical_baseline() -> None:
    """Reject any compatibility promotion or invented STRM-000 measurement."""
    catalog = load_object(CATALOG_PATH)
    require(catalog.get("catalog_id") == "strm-000-baseline", "STRM-000 catalog identity drifted")
    require(catalog.get("evidence_level") == "specification-only", "STRM-000 evidence level drifted")
    cases = catalog.get("cases")
    require(isinstance(cases, list) and cases, "STRM-000 cases are missing")
    by_id = {case.get("case_id"): case for case in cases if isinstance(case, dict)}
    require("semantic-raw-timestamp-preserved" in by_id, "semantic timestamp case is missing")
    for case_id, case in by_id.items():
        require(case.get("current_result") == "not-implemented", f"{case_id} promoted STRM-000 history")
        measured = case.get("measured_result")
        require(isinstance(measured, dict), f"{case_id} measured-result role is missing")
        require(
            measured.get("status") == "not-implemented" and measured.get("observation") is None,
            f"{case_id} invents a measured STRM-000 result",
        )


def validate_source_contract() -> None:
    """Require timestamp/chunk invariants and reject runtime or clock surfaces."""
    source = owner_source_text()
    for required in (
        "pub struct RawSourceTimestamp(f64)",
        "pub enum DerivedTimestampKind",
        "ClockCorrected",
        "Smoothed",
        "pub struct DerivedTimestamp {",
        "kind: DerivedTimestampKind",
        "pub fn new(kind: DerivedTimestampKind, value: f64)",
        "pub const fn kind(self) -> DerivedTimestampKind",
        "raw_source_timestamp: RawSourceTimestamp",
        "derived_timestamp: Option<DerivedTimestamp>",
        "pub struct TimestampedChunk<T>",
        "max_samples: usize",
        "max_channels: usize",
        "InconsistentChannelShape",
        "NonFiniteTimestamp::NaN",
        "NonFiniteTimestamp::PositiveInfinity",
        "NonFiniteTimestamp::NegativeInfinity",
    ):
        require(required in source, f"CORE-002 source invariant is missing: {required}")
    require(
        re.search(
            r"#\[non_exhaustive\]\s+pub enum DerivedTimestampKind\s*\{",
            source,
        )
        is not None,
        "derived timestamp kind must remain non-exhaustive",
    )
    facade = (SOURCE_ROOT / "lib.rs").read_text(encoding="utf-8")
    require("#![forbid(unsafe_code)]" in facade, "unsafe Rust is not forbidden")
    require("#![deny(missing_docs)]" in facade, "missing documentation is not denied")

    prohibited = {
        "unsafe block": re.compile(r"(?m)^\s*unsafe\s*\{"),
        "external ABI": re.compile(r'\bextern\s+"'),
        "network API": re.compile(r"\b(?:std|core)::net\b"),
        "thread API": re.compile(r"\bstd::thread\b"),
        "process API": re.compile(r"\bstd::process\b"),
        "clock API": re.compile(r"\b(?:std|core)::time\b|\b(?:SystemTime|Instant)::now\b"),
        "FFI API": re.compile(r"\b(?:std|core)::ffi\b"),
        "filesystem API": re.compile(r"\bstd::fs\b"),
        "environment API": re.compile(r"\bstd::env\b"),
        "I/O API": re.compile(r"\bstd::io\b"),
        "synchronization API": re.compile(r"\bstd::sync\b"),
    }
    for label, pattern in prohibited.items():
        require(not pattern.search(source), f"CORE-002 opened a prohibited {label}")

    public_declarations = "\n".join(
        line for line in source.splitlines() if re.match(r"\s*pub\s+(?:struct|enum|fn|mod|trait)", line)
    )
    prohibited_surface = re.compile(
        r"(?i)\b(?:clock|correction|dejitter|smooth|interpolat|sample.?rate|timeout|queue|buffer|"
        r"inlet|outlet|xml|discover|protocol|wire|socket|thread|ffi|adapter|authority)\w*\b"
    )
    match = prohibited_surface.search(public_declarations)
    require(match is None, f"CORE-002 opened prohibited public surface: {match.group(0) if match else ''}")


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
    require(package.get("dependencies") == [], "CORE-002 permits no Cargo dependency")
    require(package.get("features") == {}, "CORE-002 permits no Cargo feature")
    require(package.get("publish") == [], "CORE-002 package must remain unpublished")

    lock = load_object(ROOT / "morphospace" / "feature.lock.json")
    require(lock.get("default_activation") == "disabled", "feature lock default activation drifted")
    require(lock.get("selected_features") == [] and lock.get("features") == [], "feature lock is not empty")
    effects = lock.get("effect_union")
    require(isinstance(effects, dict) and set(effects) == EXPECTED_EFFECTS, "effect-union shape drifted")
    require(all(value == [] for value in effects.values()), "runtime effect union must remain empty")


def validate_instruction_surface() -> None:
    """Require every declared instruction review to be complete."""
    unit = load_object(UNIT_PATH)
    require(
        unit.get("status") in {"active", "validating", "accepted"},
        "CORE-002 is outside its implementation/validation lifecycle",
    )
    surfaces = unit.get("instruction_surfaces")
    require(isinstance(surfaces, list), "CORE-002 instruction surfaces are missing")
    statuses = {surface.get("path"): surface.get("status") for surface in surfaces if isinstance(surface, dict)}
    require(statuses.get("AGENTS.md") == "complete", "AGENTS.md instruction row is not complete")
    require(statuses.get("README.md") == "complete", "README.md instruction row is not complete")
    require(
        statuses.get("<skills-root>/system-engineering/SKILL.md") == "complete",
        "system-engineering skill review is not complete",
    )


def validate_docs_and_license() -> None:
    """Require exact public limitations and AGPL provenance for new artifacts."""
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    normalized = " ".join("\n".join(path.read_text(encoding="utf-8") for path in paths).split())
    for phrase in (
        "No LSL protocol, wire, runtime, operational, or ecosystem compatibility is implemented or claimed.",
        "finite raw source timestamps",
        "optional derived timestamp",
        "caller-provided classifications only",
        "empty bounded chunk",
        "does not read clocks",
        "local Rust contract tests",
    ):
        require(phrase in normalized, f"required CORE-002 limitation is missing: {phrase}")

    workspace_manifest = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    crate_manifest = (ROOT / "crates" / "rusty-lsl" / "Cargo.toml").read_text(encoding="utf-8")
    require(
        'license = "AGPL-3.0-or-later"' in workspace_manifest,
        "workspace license expression drifted",
    )
    require("license.workspace = true" in crate_manifest, "crate license inheritance drifted")
    require(
        "Project-owned source and documentation are licensed `AGPL-3.0-or-later`."
        in (ROOT / "docs" / "PROVENANCE.md").read_text(encoding="utf-8"),
        "public provenance license statement drifted",
    )

    licensed_paths = [
        *SOURCE_ROOT.glob("*.rs"),
        ROOT / "tools" / "check_core_002.py",
        ROOT / "tools" / "check_core_002.ps1",
    ]
    for path in licensed_paths:
        text = path.read_text(encoding="utf-8")
        require("SPDX-License-Identifier: AGPL-3.0-or-later" in text, f"license drift in {path.relative_to(ROOT)}")


def main() -> int:
    """Run all deterministic CORE-002 checks."""
    validate_overlay()
    validate_historical_baseline()
    validate_source_contract()
    validate_inert_closure()
    validate_instruction_surface()
    validate_docs_and_license()
    print("CORE-002 timestamped chunk contract checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
