# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_volatile_xml.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001p-volatile-stream-info-xml-results.json"
BOUND = {
    "xml_value_overlay": (ROOT / "fixtures/compatibility/lslc-001b-contract-results.json", "67ad7f72a1ef8c6474234de36a6864aa049b6e59170a17a7b4745f1ee51cd1b9"),
    "character_data_overlay": (ROOT / "fixtures/compatibility/lslc-001c-contract-results.json", "01d03280c76c9bd08476564e20ad7a80513e17bdb0ec56fbbedfc728b35ce7a3"),
    "element_tree_overlay": (ROOT / "fixtures/compatibility/lslc-001e-contract-results.json", "32a0d2bb2a83bb84bc126e142996fb56c5bee744b86fb7e2753d69f72f99a9f3"),
    "observation_overlay": (ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json", "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e"),
    "volatile_data_overlay": (ROOT / "fixtures/compatibility/lslc-001o-volatile-stream-info-data-results.json", "8eb56a23ae2de109368d2c5ec9279a3e5794f402d0257ceb56c1d5da73bf9346"),
    "volatile_data_receipt": (ROOT / "morphospace/receipts/rlsl-lslc-001o-standard-validation.json", "3e11bc5600a33de60015b32987f14d2acebd191b40ada6b135dae156f135cf49"),
}
ROLES = ["version","created_at","uid","session_id","hostname","v4address","v4data_port","v4service_port","v6address","v6data_port","v6service_port"]
TESTS = {
    "lslc_001p_exact_order_and_representation_are_compact_and_local",
    "lslc_001p_source_is_borrowed_unchanged_and_copies_are_distinct",
    "lslc_001p_target_node_bound_rejects_before_projection_allocation",
    "lslc_001p_first_text_failure_retains_fixed_node_index",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.name} must be an object")
    return value


def validate() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = (ROOT / "crates/rusty-lsl/src/lib.rs").read_text(encoding="utf-8")
    require("mod stream_info_volatile_xml;" in lib and "pub mod stream_info_volatile_xml" not in lib,
            "private volatile XML module route drifted")
    for marker in ("const NODE_COUNT: usize = 12", 'const ROOT_NAME: &str = "info"',
                   "const FIELD_NAMES: [&str; 11]", "limits.tree.max_nodes() < NODE_COUNT",
                   "nodes.try_reserve_exact(NODE_COUNT)", "StreamInfoVolatileFields::roles()",
                   "XmlCharacterData::encode", "XmlElementTree::new"):
        require(marker in implementation, f"volatile XML invariant missing: {marker}")
    for forbidden in ("clone()", "std::time", "SystemTime", "std::net", ".parse(", "unsafe"):
        require(forbidden not in implementation, f"provider/runtime surface opened: {forbidden}")
    require(implementation.index("limits.tree.max_nodes() < NODE_COUNT") < implementation.index("nodes.try_reserve_exact(NODE_COUNT)"),
            "target node bound must precede arena allocation")
    tests = set(re.findall(r"fn (lslc_001p_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused Rust test inventory drifted")

    overlay = load(OVERLAY)
    bindings = overlay.get("accepted_artifact_bindings", {})
    for key, (path, digest) in BOUND.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == digest, f"accepted dependency changed: {path.name}")
        require(bindings.get(key, {}).get("sha256") == digest, f"overlay binding drifted: {key}")
    policy = overlay.get("fixed_candidate_policy", {})
    require(policy.get("root") == "info" and policy.get("node_count") == 12
            and policy.get("direct_leaf_order") == ROLES
            and policy.get("document_status") == "local-element-tree-only",
            "fixed volatile XML policy drifted")
    require(set(overlay.get("focused_rust_tests", [])) == TESTS, "overlay test inventory drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [], "external implementation input entered")

    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_volatile_xml::tests::lslc_001p_"], cwd=ROOT,
        check=True, capture_output=True, text=True,
    )
    require("4 passed" in result.stdout, "focused LSLC-001P Rust tests did not all pass")
    docs = {"AGENTS.md":"check_lslc_001p.ps1", "README.md":"StreamInfoVolatileXml",
            "docs/ARCHITECTURE.md":"stream_info_volatile_xml", "docs/COMPATIBILITY.md":"LSLC-001P",
            "docs/CORPUS.md":"LSLC-001P", "docs/PROVENANCE.md":"lslc-001p-volatile-stream-info-xml-results.json",
            "docs/VALIDATION.md":"check_lslc_001p.ps1", "fixtures/compatibility/README.md":"LSLC-001P",
            "morphospace/README.md":"rlsl-lslc-001p-volatile-stream-info-xml-composition"}
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    protected = ["Cargo.toml","Cargo.lock","crates/rusty-lsl/Cargo.toml","morphospace/feature.lock.json","morphospace/project.spec.json",
                 *[str(path.relative_to(ROOT)) for path, _ in BOUND.values()]]
    status = subprocess.run(["git","status","--porcelain=v1","--",*protected], cwd=ROOT, check=True, capture_output=True, text=True).stdout
    require(not status.strip(), "protected dependency, lock, or accepted evidence changed")


def main() -> int:
    validate()
    print("LSLC-001P bounded volatile stream-info XML composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
