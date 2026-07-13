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
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_static_xml.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
CASES = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json"
OBSERVATIONS = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json"
G_OVERLAY = ROOT / "fixtures/compatibility/lslc-001g-contract-results.json"
K_OVERLAY = ROOT / "fixtures/compatibility/lslc-001k-stream-info-static-field-results.json"
L_OVERLAY = ROOT / "fixtures/compatibility/lslc-001l-static-numeric-spelling-results.json"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001m-static-stream-info-xml-results.json"

BOUND_SHA256 = {
    CASES: "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17",
    OBSERVATIONS: "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e",
    G_OVERLAY: "4819c8cea026c534140104c8416cddb6dd9130c3bd433857e367aed1ff7d0e74",
    K_OVERLAY: "3840fdd65dd33d56f424a3c7958b0bf9294f1b44d1e62d7476f0f48ded37511a",
    L_OVERLAY: "968c91d063cfaa18edbae44bac4245d3f4c5b5ca3f34bf83f49704315c234f4a",
}
TESTS = {
    "lslc_001m_seven_observed_static_cases_compose_and_serialize_exactly",
    "lslc_001m_absent_and_present_empty_optionals_share_only_effective_xml",
    "lslc_001m_rejections_are_typed_and_ordered",
    "lslc_001m_copy_and_consuming_access_preserve_owned_allocations",
    "lslc_001m_limit_accessors_are_exact",
}
STATIC_NAMES = ["name", "type", "channel_count", "channel_format", "source_id", "nominal_srate"]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def validate_accepted_dependencies() -> None:
    for path, expected in BOUND_SHA256.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == expected,
                f"accepted dependency changed: {path.name}")
    g = runpy.run_path(str(ROOT / "tools/check_lslc_001g.py"))
    g["validate_source"]()
    g["validate_overlay_and_history"]()
    l = runpy.run_path(str(ROOT / "tools/check_lslc_001l.py"))
    l["validate_source"]()
    l["validate_immutable_h_and_k"]()
    l["validate_overlay"]()


def validate_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = LIB.read_text(encoding="utf-8")
    require("mod stream_info_static_xml;" in lib, "private static XML module route missing")
    require("pub mod stream_info_static_xml" not in lib, "static XML module became public")
    for marker in (
        "const NODE_COUNT: usize = 7;",
        'const ROOT_NAME: &str = "info";',
        '"channel_count"',
        '"channel_format"',
        '"nominal_srate"',
        "StreamInfoStaticNumericSpellings::new(fields)",
        "nodes.try_reserve_exact(NODE_COUNT)",
        "copied.try_reserve_exact(requested)",
        "XmlText::new(limit, copied)",
        "XmlCharacterData::encode",
        "XmlElementTree::new(limits.tree, nodes)",
    ):
        require(marker in implementation, f"static XML invariant missing: {marker}")
    for forbidden in ("format!", ".to_owned()", ".to_string()", "unsafe"):
        require(forbidden not in implementation, f"unbounded or forbidden surface opened: {forbidden}")
    require(
        implementation.index("StreamInfoStaticNumericSpellings::new(fields)")
        < implementation.index("nodes.try_reserve_exact(NODE_COUNT)"),
        "numeric domain must reject before XML arena allocation",
    )
    tests = set(re.findall(r"fn (lslc_001m_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused LSLC-001M Rust test inventory drifted")


def element_text(xml: str, tag: str) -> str:
    match = re.search(fr"<{tag}>(.*?)</{tag}>", xml)
    require(match is not None, f"XML omits {tag}")
    return match.group(1)


def validate_overlay() -> None:
    cases = load(CASES).get("positive_cases", [])
    observations_doc = load(OBSERVATIONS)
    observations = observations_doc.get("observations", [])
    overlay = load(OVERLAY)
    results = overlay.get("candidate_static_xml_results", [])
    require(observations_doc.get("candidate_result") == {"status": "not-observed", "evidence": None},
            "accepted full-document candidate role changed")
    bindings = overlay.get("accepted_artifact_bindings", {})
    expected_bindings = {
        "case_manifest": CASES,
        "observation_overlay": OBSERVATIONS,
        "element_serialization_overlay": G_OVERLAY,
        "semantic_projection_overlay": K_OVERLAY,
        "numeric_spelling_overlay": L_OVERLAY,
    }
    for key, path in expected_bindings.items():
        require(bindings.get(key, {}).get("sha256") == BOUND_SHA256[path],
                f"overlay binding drifted: {key}")
    require(len(cases) == len(observations) == len(results) == 7,
            "exact seven-case static matrix required")
    for case, observation, result in zip(cases, observations, results):
        case_id = case.get("case_id")
        require(case_id == observation.get("case_id") == result.get("case_id"),
                f"case identity or order drifted: {case_id}")
        compact = result.get("compact_xml", "")
        require(compact.startswith("<info><name>") and compact.endswith("</nominal_srate></info>"),
                f"compact static envelope drifted: {case_id}")
        require("<?xml" not in compact and "<desc" not in compact and "\n" not in compact,
                f"complete-document or whitespace policy leaked: {case_id}")
        positions = [compact.index(f"<{name}>") for name in STATIC_NAMES]
        require(positions == sorted(positions), f"static child order drifted: {case_id}")
        observed = observation.get("public_xml_utf8", "")
        for name in STATIC_NAMES:
            require(element_text(compact, name) == element_text(observed, name),
                    f"static represented value drifted for {case_id}: {name}")
        require(result.get("result") == "implemented-local-static-element-composition",
                f"candidate result label drifted: {case_id}")
    shape = overlay.get("fixed_candidate_shape", {})
    require(shape.get("root") == "info" and shape.get("direct_children") == STATIC_NAMES,
            "fixed static tree shape drifted")
    require(shape.get("node_count") == 7 and shape.get("root_one_depth") == 2
            and shape.get("root_direct_children") == 6, "fixed static bounds drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "external implementation input entered")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_static_xml::tests::lslc_001m_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("5 passed" in result.stdout, "focused LSLC-001M Rust tests did not all pass")
    docs = {
        "AGENTS.md": "check_lslc_001m.ps1",
        "README.md": "StreamInfoStaticXml",
        "docs/ARCHITECTURE.md": "stream_info_static_xml",
        "docs/COMPATIBILITY.md": "LSLC-001M",
        "docs/CORPUS.md": "LSLC-001M",
        "docs/PROVENANCE.md": "lslc-001m-static-stream-info-xml-results.json",
        "docs/VALIDATION.md": "check_lslc_001m.ps1",
        "morphospace/README.md": "rlsl-lslc-001m-static-stream-info-xml-composition",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"),
                f"documentation route missing: {path}")
    protected = [
        "Cargo.toml", "Cargo.lock", "crates/rusty-lsl/Cargo.toml",
        "morphospace/feature.lock.json", "morphospace/project.spec.json",
        "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json",
        "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json",
        "fixtures/compatibility/lslc-001g-contract-results.json",
        "fixtures/compatibility/lslc-001k-stream-info-static-field-results.json",
        "fixtures/compatibility/lslc-001l-static-numeric-spelling-results.json",
    ]
    status = subprocess.run(
        ["git", "status", "--porcelain=v1", "--", *protected], cwd=ROOT,
        check=True, capture_output=True, text=True,
    ).stdout
    require(not status.strip(), "protected dependency, lock, or accepted fixture changed")
    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--offline", "--locked", "--no-deps", "--format-version", "1"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout)
    package = metadata["packages"][0]
    require(len(metadata["packages"]) == 1 and package["dependencies"] == [],
            "dependency closure drifted")
    require(package["features"] == {} and package["publish"] == [],
            "feature/publication closure drifted")


def main() -> int:
    validate_source()
    validate_accepted_dependencies()
    validate_overlay()
    validate_execution_docs_and_closure()
    print("LSLC-001M bounded static stream-info XML composition checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
