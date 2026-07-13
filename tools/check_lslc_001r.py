# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_observed_document.rs"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001r-observed-stream-info-document-envelope-results.json"
COMPACT_SOURCE = ROOT / "crates/rusty-lsl/src/xml_element_serialization.rs"
BOUND = {
    "case_manifest": (ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json", "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17"),
    "observation_overlay": (ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json", "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e"),
    "compact_serialization_overlay": (ROOT / "fixtures/compatibility/lslc-001g-contract-results.json", "4819c8cea026c534140104c8416cddb6dd9130c3bd433857e367aed1ff7d0e74"),
    "compact_serialization_receipt": (ROOT / "morphospace/receipts/rlsl-lslc-001g-standard-validation.json", "3fc8f4e4d0c9f09a859cf6b8ff80a3a47f648796215110c5d287ae129b6821e1"),
    "ordered_element_overlay": (ROOT / "fixtures/compatibility/lslc-001q-ordered-stream-info-element-results.json", "9e8bbef9f1122a568983ef33b035302e8fd8836da60e5cb172a0a41ea60d6bde"),
    "ordered_element_receipt": (ROOT / "morphospace/receipts/rlsl-lslc-001q-standard-validation.json", "4eacd6ee1f24c713ba97aad975825a7bfb91018b255fda9f930cf177866cb50c"),
}
TESTS = {
    "lslc_001r_seven_normalized_observations_match_exact_bytes",
    "lslc_001r_exact_limit_borrow_and_consuming_output_preserve_allocations",
    "lslc_001r_one_past_limit_reports_exact_required_bytes",
    "lslc_001r_childless_non_desc_container_fails_closed",
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
    require("mod stream_info_observed_document;" in lib and "pub mod stream_info_observed_document" not in lib,
            "private observed-document module route drifted")
    for marker in ('const DECLARATION: &str = "<?xml version=\\"1.0\\"?>\\n"',
                   "const DESCRIPTION_ROOT_INDEX: usize = 18", "frames.try_reserve_exact(nodes.len())",
                   "validate_empty_containers", "exact_output_bytes", "required > limit.max_output_bytes",
                   "output.try_reserve_exact(required)", 'output.push_str(" />\\n")',
                   "StreamInfoObservedDocumentError::UnsupportedEmptyContainer"):
        require(marker in implementation, f"observed envelope invariant missing: {marker}")
    for forbidden in ("XmlElementSerialization::serialize", "clone()", "std::time", "SystemTime", "std::net", ".parse(", "unsafe"):
        require(forbidden not in implementation, f"compact/provider/runtime boundary opened: {forbidden}")
    require(implementation.index("required > limit.max_output_bytes") < implementation.index("output.try_reserve_exact(required)"),
            "output bound must precede output allocation")
    require(hashlib.sha256(COMPACT_SOURCE.read_bytes()).hexdigest() == "3b46413a31213c1945551a597b6a2d44b294681d854456460e0ef1c4b2072c37",
            "LSLC-001G compact serializer source changed")
    tests = set(re.findall(r"fn (lslc_001r_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused Rust test inventory drifted")

    overlay = load(OVERLAY)
    bindings = overlay.get("accepted_artifact_bindings", {})
    for key, (path, digest) in BOUND.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == digest, f"accepted dependency changed: {path.name}")
        require(bindings.get(key, {}).get("sha256") == digest, f"overlay binding drifted: {key}")
    policy = overlay.get("fixed_candidate_policy", {})
    require(policy.get("line_endings") == "LF-only"
            and policy.get("empty_desc") == "<desc />"
            and policy.get("other_empty_containers") == "typed-fail-closed"
            and policy.get("compact_serializer") == "unchanged-separate-lslc-001g",
            "fixed observed envelope policy drifted")
    require(set(overlay.get("focused_rust_tests", [])) == TESTS, "overlay test inventory drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [], "external implementation input entered")

    observations = load(BOUND["observation_overlay"][0]).get("observations", [])
    require(len(observations) == 7, "accepted observation inventory drifted")
    require(all(item.get("public_xml_utf8", "").startswith('<?xml version="1.0"?>\n<info>\n')
                and item.get("public_xml_utf8", "").endswith("</info>\n") for item in observations),
            "accepted declaration or final LF evidence drifted")
    require(sum(item.get("observed_dimensions", {}).get("empty_desc_form") == "<desc />" for item in observations) == 6,
            "accepted empty-desc evidence drifted")

    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_observed_document::tests::lslc_001r_"], cwd=ROOT,
        check=True, capture_output=True, text=True,
    )
    require("4 passed" in result.stdout, "focused LSLC-001R Rust tests did not all pass")
    docs = {"AGENTS.md":"check_lslc_001r.ps1", "README.md":"StreamInfoObservedDocument",
            "docs/ARCHITECTURE.md":"stream_info_observed_document", "docs/COMPATIBILITY.md":"LSLC-001R",
            "docs/CORPUS.md":"LSLC-001R", "docs/PROVENANCE.md":"lslc-001r-observed-stream-info-document-envelope-results.json",
            "docs/VALIDATION.md":"check_lslc_001r.ps1", "fixtures/compatibility/README.md":"LSLC-001R",
            "morphospace/README.md":"rlsl-lslc-001r-observed-stream-info-document-envelope"}
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"), f"documentation route missing: {path}")
    protected = ["Cargo.toml","Cargo.lock","crates/rusty-lsl/Cargo.toml","crates/rusty-lsl/src/xml_element_serialization.rs",
                 "morphospace/feature.lock.json","morphospace/project.spec.json", *[str(path.relative_to(ROOT)) for path, _ in BOUND.values()]]
    status = subprocess.run(["git","status","--porcelain=v1","--",*protected], cwd=ROOT, check=True, capture_output=True, text=True).stdout
    require(not status.strip(), "protected dependency, compact serializer, lock, or accepted evidence changed")


def main() -> int:
    validate()
    print("LSLC-001R observed stream-info document-envelope checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
