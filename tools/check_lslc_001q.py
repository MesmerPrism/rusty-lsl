# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_ordered_xml.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001q-ordered-stream-info-element-results.json"
BOUND = {
    "case_manifest": (ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json", "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17"),
    "observation_overlay": (ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json", "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e"),
    "description_xml_overlay": (ROOT / "fixtures/compatibility/lslc-001n-description-xml-results.json", "ee8071b2ecc204e8dcd5de7e0d588d02b500a248e6ba1fa41e8a034eeaf61fd7"),
    "description_xml_receipt": (ROOT / "morphospace/receipts/rlsl-lslc-001n-standard-validation.json", "8c6ab97b4697d5e42baedc6e4330b76ad2cda043ab507e2be97d037542a273a6"),
    "volatile_xml_overlay": (ROOT / "fixtures/compatibility/lslc-001p-volatile-stream-info-xml-results.json", "57456fb3149dd1649ec94e68ee280e194311c77a2e7486f8700531628764e3b1"),
    "volatile_xml_receipt": (ROOT / "morphospace/receipts/rlsl-lslc-001p-standard-validation.json", "afff9a3aa55f7e76b3e443dd967f58ae5562f65b60bd6c3fcd57c5bc48e58716"),
}
ORDER = ["name","type","channel_count","channel_format","source_id","nominal_srate","version","created_at","uid","session_id","hostname","v4address","v4data_port","v4service_port","v6address","v6data_port","v6service_port","desc"]
TESTS = {
    "lslc_001q_seven_cases_preserve_exact_static_volatile_desc_order",
    "lslc_001q_only_description_parents_receive_volatile_offset",
    "lslc_001q_component_values_move_without_cloning",
    "lslc_001q_target_bound_rejects_before_merged_allocation",
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
    require("mod stream_info_ordered_xml;" in lib and "pub mod stream_info_ordered_xml" not in lib,
            "private ordered XML module route drifted")
    for marker in ("const STATIC_NODE_COUNT: usize = 7", "const VOLATILE_NODE_COUNT: usize = 12",
                   "const VOLATILE_LEAF_COUNT", "validate_static_description_shape",
                   "validate_volatile_shape", "total > limits.max_nodes()",
                   ".try_reserve_exact(total)", "drain(..STATIC_NODE_COUNT)",
                   "skip(1)", "checked_add(VOLATILE_LEAF_COUNT)", "XmlElementTree::new"):
        require(marker in implementation, f"ordered composition invariant missing: {marker}")
    for forbidden in ("clone()", "std::time", "SystemTime", "std::net", ".parse(", "unsafe"):
        require(forbidden not in implementation, f"document/provider/runtime surface opened: {forbidden}")
    require(implementation.index("total > limits.max_nodes()") < implementation.index(".try_reserve_exact(total)"),
            "target node bound must precede merged allocation")
    tests = set(re.findall(r"fn (lslc_001q_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused Rust test inventory drifted")

    overlay = load(OVERLAY)
    bindings = overlay.get("accepted_artifact_bindings", {})
    for key, (path, digest) in BOUND.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == digest, f"accepted dependency changed: {path.name}")
        require(bindings.get(key, {}).get("sha256") == digest, f"overlay binding drifted: {key}")
    policy = overlay.get("fixed_candidate_policy", {})
    require(policy.get("root") == "info" and policy.get("direct_child_order") == ORDER
            and policy.get("description_parent_remap") == "add-eleven-only-to-non-root-description-parents"
            and policy.get("document_status") == "local-element-tree-only",
            "fixed ordered-element policy drifted")
    require(set(overlay.get("focused_rust_tests", [])) == TESTS, "overlay test inventory drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [], "external implementation input entered")

    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_ordered_xml::tests::lslc_001q_"], cwd=ROOT,
        check=True, capture_output=True, text=True,
    )
    require("4 passed" in result.stdout, "focused LSLC-001Q Rust tests did not all pass")
    docs = {"AGENTS.md":"check_lslc_001q.ps1", "README.md":"StreamInfoOrderedXml",
            "docs/ARCHITECTURE.md":"stream_info_ordered_xml", "docs/COMPATIBILITY.md":"LSLC-001Q",
            "docs/CORPUS.md":"LSLC-001Q", "docs/PROVENANCE.md":"lslc-001q-ordered-stream-info-element-results.json",
            "docs/VALIDATION.md":"check_lslc_001q.ps1", "fixtures/compatibility/README.md":"LSLC-001Q",
            "morphospace/README.md":"rlsl-lslc-001q-ordered-stream-info-element-composition"}
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    protected = ["Cargo.toml","Cargo.lock","crates/rusty-lsl/Cargo.toml","morphospace/feature.lock.json","morphospace/project.spec.json",
                 *[str(path.relative_to(ROOT)) for path, _ in BOUND.values()]]
    status = subprocess.run(["git","status","--porcelain=v1","--",*protected], cwd=ROOT, check=True, capture_output=True, text=True).stdout
    require(not status.strip(), "protected dependency, lock, or accepted evidence changed")


def main() -> int:
    validate()
    print("LSLC-001Q ordered stream-info element composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
