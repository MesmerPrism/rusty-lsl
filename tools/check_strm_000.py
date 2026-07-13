#!/usr/bin/env python3
"""Validate the source-only STRM-000 compatibility and provenance baseline."""

from __future__ import annotations

import hashlib
import json
import math
import re
import subprocess
from collections import Counter
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures" / "compatibility"
CATALOG_PATH = FIXTURES / "behavior-catalog.json"
NEGATIVE_PATH = FIXTURES / "negative-case-matrix.json"
MANIFEST_PATH = FIXTURES / "baseline-provenance.json"
CLASSES = {"contract", "semantic-bridge", "wire", "operational-ecosystem"}
ORIGIN_CLASSES = {
    "independently-authored",
    "generated",
    "black-box-observed",
    "adapted",
    "copied",
}
EXPECTED_PROHIBITIONS = {
    "liblsl-source",
    "rlsl-source",
    "implementation-derived-protocol-bytes",
    "private-captures",
    "native-binaries",
}
EXPECTED_CLASSIFICATIONS = {
    "incomplete-observation",
    "reject-authority-escalation",
    "reject-evidence-loss",
    "reject-input",
    "reject-semantic-loss",
    "timeout",
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
NEGATIONS = {"no", "not", "neither", "without", "doesn't", "isn't"}


def require(condition: bool, message: str) -> None:
    """Raise a stable validation error when an invariant is false."""
    if not condition:
        raise ValueError(message)


def load_object(path: Path) -> dict[str, Any]:
    """Load one required JSON object."""
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def sha256(path: Path) -> str:
    """Return a lowercase SHA-256 digest."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def nonempty_text(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate_bounds(value: Any, context: str) -> None:
    """Require a non-empty, finite JSON object of explicit scalar bounds."""
    require(isinstance(value, dict) and value, f"{context} needs bounded inputs")
    for name, bound in value.items():
        require(nonempty_text(name), f"{context} has an empty bound name")
        scalar = isinstance(bound, (int, str, bool)) or (
            isinstance(bound, float) and math.isfinite(bound)
        )
        require(scalar, f"{context} bound '{name}' must be a finite scalar")


def validate_catalog(catalog: dict[str, Any]) -> set[str]:
    """Validate class coverage and the specification/plan/result separation."""
    require(
        catalog.get("schema") == "rusty.lsl.compatibility.behavior_catalog.v1",
        "behavior catalog schema drifted",
    )
    require(catalog.get("evidence_level") == "specification-only", "catalog overclaims evidence")
    cases = catalog.get("cases")
    require(isinstance(cases, list) and cases, "behavior catalog must contain cases")
    ids: list[str] = []
    class_counts: Counter[str] = Counter()
    for case in cases:
        require(isinstance(case, dict), "each behavior case must be an object")
        case_id = case.get("case_id")
        compatibility_class = case.get("compatibility_class")
        require(nonempty_text(case_id), "behavior case needs an identity")
        require(compatibility_class in CLASSES, f"{case_id} has an unknown compatibility class")
        specification = case.get("specification")
        planned = case.get("planned_observation")
        measured = case.get("measured_result")
        require(isinstance(specification, dict), f"{case_id} needs a specification")
        require(nonempty_text(specification.get("behavior")), f"{case_id} needs specified behavior")
        validate_bounds(specification.get("bounded_inputs"), case_id)
        require(isinstance(planned, dict), f"{case_id} needs a planned observation")
        require(nonempty_text(planned.get("endpoint")), f"{case_id} needs a planned endpoint")
        require(nonempty_text(planned.get("observable")), f"{case_id} needs a planned observable")
        require(case.get("current_result") == "not-implemented", f"{case_id} overclaims its result")
        require(isinstance(measured, dict), f"{case_id} needs a measured-result role")
        require(measured.get("status") == "not-implemented", f"{case_id} measured status overclaims")
        require(measured.get("observation") is None, f"{case_id} must not contain a measured observation")
        ids.append(case_id)
        class_counts[compatibility_class] += 1
    require(len(ids) == len(set(ids)), "behavior case identities must be unique")
    require(set(class_counts) == CLASSES, "all four compatibility classes are required")
    require(all(class_counts[name] >= 2 for name in CLASSES), "each compatibility class needs two cases")
    return set(ids)


def validate_negative_matrix(matrix: dict[str, Any], case_ids: set[str]) -> set[str]:
    """Validate bounded damaged cases and their catalog relationships."""
    require(
        matrix.get("schema") == "rusty.lsl.compatibility.negative_case_matrix.v1",
        "negative-case matrix schema drifted",
    )
    require(matrix.get("evidence_level") == "specification-only", "negative matrix overclaims evidence")
    cases = matrix.get("negative_cases")
    require(isinstance(cases, list) and cases, "negative-case matrix must not be empty")
    ids: list[str] = []
    covered: set[str] = set()
    for case in cases:
        require(isinstance(case, dict), "each negative case must be an object")
        negative_id = case.get("negative_case_id")
        case_id = case.get("case_id")
        require(nonempty_text(negative_id), "negative case needs an identity")
        require(case_id in case_ids, f"{negative_id} references an unknown behavior case")
        validate_bounds(case.get("bounded_input"), negative_id)
        require(
            case.get("expected_oracle_classification") in EXPECTED_CLASSIFICATIONS,
            f"{negative_id} has an unknown expected classification",
        )
        require(case.get("current_result") == "not-implemented", f"{negative_id} overclaims its result")
        require(case.get("measured_observation") is None, f"{negative_id} must not contain an observation")
        ids.append(negative_id)
        covered.add(case_id)
    require(len(ids) == len(set(ids)), "negative-case identities must be unique")
    require(covered == case_ids, "every behavior case needs a negative case")
    return set(ids)


def validate_manifest(
    manifest: dict[str, Any], case_ids: set[str], negative_ids: set[str]
) -> None:
    """Validate provenance fields, digests, and prohibited implementation inputs."""
    require(manifest.get("schema") == "rusty.lsl.provenance.manifest.v1", "manifest schema drifted")
    require(manifest.get("origin_classification") in ORIGIN_CLASSES, "manifest origin is invalid")
    require(nonempty_text(manifest.get("license_expression")), "manifest license is missing")
    require(nonempty_text(manifest.get("created_on")), "manifest creation date is missing")
    toolchain = manifest.get("toolchain")
    environment = manifest.get("environment")
    require(isinstance(toolchain, dict), "toolchain is missing")
    require(
        set(toolchain) == {"manifest_writer", "validator", "oracle_endpoint", "oracle_version"}
        and all(nonempty_text(value) for value in toolchain.values()),
        "toolchain fields are incomplete",
    )
    require(isinstance(environment, dict), "environment is missing")
    require(
        set(environment)
        == {
            "execution_state",
            "operating_system",
            "architecture",
            "locale",
            "time_zone",
            "network_topology",
        }
        and all(nonempty_text(value) for value in environment.values()),
        "environment fields are incomplete",
    )
    require(manifest.get("observations") == [], "STRM-000 must contain no measured observations")
    require(manifest.get("implementation_inputs") == [], "implementation inputs are prohibited")
    require(
        set(manifest.get("source_input_prohibitions", [])) == EXPECTED_PROHIBITIONS,
        "source-input prohibitions drifted",
    )
    require(manifest.get("case_ids") == sorted(case_ids), "manifest case identities must be exact and sorted")
    require(
        manifest.get("negative_case_ids") == sorted(negative_ids),
        "manifest negative-case identities must be exact and sorted",
    )
    require(isinstance(manifest.get("normalization"), dict), "normalization contract is missing")
    require(manifest["normalization"].get("performed") is False, "baseline normalization must be absent")
    require(
        isinstance(manifest.get("does_not_prove"), list) and manifest["does_not_prove"],
        "manifest limitations are missing",
    )

    artifacts = manifest.get("artifacts")
    require(isinstance(artifacts, list) and len(artifacts) == 2, "manifest must bind two source fixtures")
    expected_paths = {
        "fixtures/compatibility/behavior-catalog.json": CATALOG_PATH,
        "fixtures/compatibility/negative-case-matrix.json": NEGATIVE_PATH,
    }
    seen: set[str] = set()
    for artifact in artifacts:
        require(isinstance(artifact, dict), "manifest artifact must be an object")
        relative = artifact.get("path")
        require(relative in expected_paths, f"manifest references unexpected artifact '{relative}'")
        require(relative not in seen, f"manifest repeats artifact '{relative}'")
        require(artifact.get("origin_classification") in ORIGIN_CLASSES, f"{relative} origin is invalid")
        require(nonempty_text(artifact.get("license_expression")), f"{relative} license is missing")
        require(artifact.get("sha256") == sha256(expected_paths[relative]), f"{relative} digest is stale")
        seen.add(relative)
    require(seen == set(expected_paths), "manifest artifact relationships are incomplete")


def validate_public_fixture_shape() -> None:
    """Reject binary/capture-like fixtures and protocol-implementation payload keys."""
    allowed_suffixes = {".json", ".md"}
    prohibited_key = re.compile(r"(?i)(?:wire|protocol|packet|header|frame)[_-]?(?:bytes|constant|magic)")
    for path in FIXTURES.rglob("*"):
        if not path.is_file():
            continue
        require(path.suffix.lower() in allowed_suffixes, f"prohibited fixture type: {path.name}")
        data = path.read_bytes()
        require(b"\0" not in data, f"binary fixture is prohibited: {path.name}")
        if path.suffix.lower() == ".json":
            value = json.loads(data.decode("utf-8"))
            stack = [value]
            while stack:
                current = stack.pop()
                if isinstance(current, dict):
                    for key, child in current.items():
                        require(not prohibited_key.search(key), f"{path.name} contains prohibited key '{key}'")
                        stack.append(child)
                elif isinstance(current, list):
                    stack.extend(current)


def validate_overclaims() -> None:
    """Reject unqualified positive compatibility or runtime-support claims."""
    pattern = re.compile(
        r"(?i)\b(?:wire|ecosystem|protocol|runtime)\s+(?:compatibility|support)\s+"
        r"(?:is|has been)\s+(?:implemented|claimed|proven|verified|validated|supported)\b"
    )
    paths = [ROOT / "README.md", ROOT / "AGENTS.md", *(ROOT / "docs").glob("*.md")]
    for path in paths:
        for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            if not pattern.search(line):
                continue
            words = set(re.findall(r"[a-z]+(?:n't)?", line.lower()))
            require(bool(words & NEGATIONS), f"overclaim in {path.relative_to(ROOT)}:{number}")
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    require(
        "No protocol, runtime, wire, or ecosystem compatibility is implemented\n"
        "or claimed." in readme,
        "README compatibility disclaimer drifted",
    )


def validate_inert_closure() -> None:
    """Reject Cargo dependency/features or project-lock activation drift."""
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
    require(package.get("dependencies") == [], "STRM-000 permits no Cargo dependency")
    require(package.get("features") == {}, "STRM-000 permits no Cargo feature")
    require(package.get("publish") == [], "STRM-000 package must remain unpublished")

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


def main() -> int:
    """Run all deterministic STRM-000 checks."""
    catalog = load_object(CATALOG_PATH)
    negative = load_object(NEGATIVE_PATH)
    manifest = load_object(MANIFEST_PATH)
    case_ids = validate_catalog(catalog)
    negative_ids = validate_negative_matrix(negative, case_ids)
    validate_manifest(manifest, case_ids, negative_ids)
    validate_public_fixture_shape()
    validate_overclaims()
    validate_inert_closure()
    print("STRM-000 compatibility baseline checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
