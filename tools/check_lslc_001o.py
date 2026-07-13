# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_volatile_fields.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
CORPUS = ROOT / "fixtures/compatibility/lslc-001a-stream-info-document-corpus.json"
CASES = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json"
OBSERVATIONS = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json"
PROVENANCE = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-provenance.json"
N_RECEIPT = ROOT / "morphospace/receipts/rlsl-lslc-001n-standard-validation.json"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001o-volatile-stream-info-data-results.json"
BOUND_SHA256 = {
    CORPUS: "68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c",
    CASES: "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17",
    OBSERVATIONS: "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e",
    PROVENANCE: "7f1c7c80a4c749fda6303fa22cb54b0378b1894996b434b45e2969459f7f63c2",
    N_RECEIPT: "8c6ab97b4697d5e42baedc6e4330b76ad2cda043ab507e2be97d037542a273a6",
}
ROLES = [
    "version", "created_at", "uid", "session_id", "hostname", "v4address",
    "v4data_port", "v4service_port", "v6address", "v6data_port", "v6service_port",
]
CLASSES = {
    "implementation-assigned": ["version"],
    "runtime-assigned": ["created_at", "uid", "session_id", "hostname"],
    "transport-owned": [
        "v4address", "v4data_port", "v4service_port",
        "v6address", "v6data_port", "v6service_port",
    ],
}
TESTS = {
    "lslc_001o_role_order_and_classes_are_exact_and_disjoint",
    "lslc_001o_exact_limits_preserve_empty_unicode_and_original_allocations",
    "lslc_001o_zero_limits_reject_in_class_order",
    "lslc_001o_one_past_values_reject_in_fixed_role_order",
    "lslc_001o_values_remain_opaque_without_provider_or_semantic_interpretation",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def validate_dependencies_and_observed_roles() -> None:
    for path, expected in BOUND_SHA256.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == expected,
                f"accepted dependency changed: {path.name}")
    observations = load(OBSERVATIONS)
    require(observations.get("candidate_result") == {"status": "not-observed", "evidence": None},
            "accepted complete-document candidate role changed")
    rows = observations.get("observations", [])
    require(len(rows) == 7, "exact seven accepted observations required")
    for row in rows:
        field_order = row.get("observed_dimensions", {}).get("direct_info_child_order", [])
        start = field_order.index("version")
        require(field_order[start:start + len(ROLES)] == ROLES,
                f"volatile field order drifted: {row.get('case_id')}")


def validate_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = LIB.read_text(encoding="utf-8")
    require("mod stream_info_volatile_fields;" in lib, "private volatile data module route missing")
    require("pub mod stream_info_volatile_fields" not in lib, "volatile data module became public")
    for marker in (
        "ImplementationAssigned", "RuntimeAssigned", "TransportOwned",
        "const ROLES: [StreamInfoVolatileFieldRole; 11]", "Self::Version",
        "Self::CreatedAt", "Self::Hostname", "Self::V4Address", "Self::V6ServicePort",
        "if actual == 0", "for role in Self::ROLES", ".chars().count()",
        "TextLimitExceeded", "Ok(Self { limits, input })",
    ):
        require(marker in implementation, f"volatile data invariant missing: {marker}")
    for forbidden in ("unsafe", "std::time", "SystemTime", "std::net", ".parse(", "Xml", "clone()"):
        require(forbidden not in implementation, f"provider, representation, or runtime surface opened: {forbidden}")
    require(implementation.index("ImplementationAssigned") < implementation.index("RuntimeAssigned")
            < implementation.index("TransportOwned"), "limit class order drifted")
    tests = set(re.findall(r"fn (lslc_001o_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused LSLC-001O Rust test inventory drifted")


def validate_overlay() -> None:
    overlay = load(OVERLAY)
    bindings = overlay.get("accepted_artifact_bindings", {})
    expected_paths = {
        "documentation_corpus": CORPUS,
        "case_manifest": CASES,
        "observation_overlay": OBSERVATIONS,
        "observation_provenance": PROVENANCE,
        "accepted_description_receipt": N_RECEIPT,
    }
    for key, path in expected_paths.items():
        require(bindings.get(key, {}).get("sha256") == BOUND_SHA256[path],
                f"overlay binding drifted: {key}")
    require(overlay.get("fixed_role_order") == ROLES, "overlay role order drifted")
    require(overlay.get("field_classes") == CLASSES, "overlay field classes drifted")
    policy = overlay.get("candidate_policy", {})
    require(policy.get("value_kind") == "opaque-caller-owned-utf8-text"
            and policy.get("count_unit") == "unicode-scalar-values"
            and policy.get("semantic_validation") == "none",
            "opaque bounded candidate policy drifted")
    require(set(overlay.get("focused_rust_tests", [])) == TESTS,
            "overlay focused test inventory drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "external implementation input entered")
    require(len(overlay.get("does_not_prove", [])) >= 5, "boundary limitations are incomplete")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_volatile_fields::tests::lslc_001o_"], cwd=ROOT,
        check=True, capture_output=True, text=True,
    )
    require("5 passed" in result.stdout, "focused LSLC-001O Rust tests did not all pass")
    docs = {
        "AGENTS.md": "check_lslc_001o.ps1",
        "README.md": "StreamInfoVolatileFields",
        "docs/ARCHITECTURE.md": "stream_info_volatile_fields",
        "docs/COMPATIBILITY.md": "LSLC-001O",
        "docs/CORPUS.md": "LSLC-001O",
        "docs/PROVENANCE.md": "lslc-001o-volatile-stream-info-data-results.json",
        "docs/VALIDATION.md": "check_lslc_001o.ps1",
        "fixtures/compatibility/README.md": "LSLC-001O",
        "morphospace/README.md": "rlsl-lslc-001o-volatile-stream-info-data-contract",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"),
                f"documentation route missing: {path}")
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    require("no official-liblsl observation has been measured" not in readme,
            "stale pre-LSLC-001H status prose remains")
    protected = [
        "Cargo.toml", "Cargo.lock", "crates/rusty-lsl/Cargo.toml",
        "morphospace/feature.lock.json", "morphospace/project.spec.json",
        *[str(path.relative_to(ROOT)) for path in BOUND_SHA256],
    ]
    status = subprocess.run(
        ["git", "status", "--porcelain=v1", "--", *protected], cwd=ROOT,
        check=True, capture_output=True, text=True,
    ).stdout
    require(not status.strip(), "protected dependency, lock, or accepted evidence changed")
    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout)
    package = metadata["packages"][0]
    require(len(metadata["packages"]) == 1 and package["dependencies"] == [],
            "dependency closure drifted")
    require(package["features"] == {} and package["publish"] == [],
            "feature or publication closure drifted")


def main() -> int:
    validate_source()
    validate_dependencies_and_observed_roles()
    validate_overlay()
    validate_execution_docs_and_closure()
    print("LSLC-001O bounded volatile stream-info data checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
