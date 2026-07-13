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
SOURCE = ROOT / "crates/rusty-lsl/src/stream_info_static_numeric_spellings.rs"
LIB = ROOT / "crates/rusty-lsl/src/lib.rs"
CASES = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json"
OBSERVATIONS = ROOT / "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json"
K_OVERLAY = ROOT / "fixtures/compatibility/lslc-001k-stream-info-static-field-results.json"
OVERLAY = ROOT / "fixtures/compatibility/lslc-001l-static-numeric-spelling-results.json"
K_CHECKER = ROOT / "tools/check_lslc_001k.py"

BOUND_SHA256 = {
    CASES: "398adef9dab9fc7aed44991168734dbc29b270616833586acbe0b3b48f8d9d17",
    OBSERVATIONS: "2b1aaa4ce3faa20722386c224e70dd7b8252fecd94b6e4437280af2bb4c5ab1e",
    K_OVERLAY: "3840fdd65dd33d56f424a3c7958b0bf9294f1b44d1e62d7476f0f48ded37511a",
}
EXPECTED = [
    ("irregular-float32-empty-optionals-desc", "1", "0.000000000000000"),
    ("regular-double64-populated-optionals", "2", "100.0000000000000"),
    ("regular-string-unicode-sensitive-nested-desc", "3", "59.94000000000000"),
    ("regular-int32-unit-rate", "4", "1.000000000000000"),
    ("regular-int16-fractional-rate", "5", "256.5000000000000"),
    ("irregular-int8", "6", "0.000000000000000"),
    ("regular-int64-large-fractional-rate", "7", "1000000.250000000"),
]
TESTS = {
    "lslc_001l_seven_observed_numeric_cases_execute_exactly",
    "lslc_001l_unsupported_regular_rates_fail_closed_with_exact_bits",
    "lslc_001l_source_remains_borrowed_unchanged_and_reusable",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def validate_immutable_h_and_k() -> None:
    for path, expected in BOUND_SHA256.items():
        require(hashlib.sha256(path.read_bytes()).hexdigest() == expected,
                f"accepted artifact changed: {path.name}")

    historical = runpy.run_path(str(K_CHECKER))
    historical["validate_source"]()
    historical["validate_preserved_black_box_contracts"]()
    historical["validate_overlay"]()
    historical["validate_execution_docs_and_closure"]()


def validate_source() -> None:
    source = SOURCE.read_text(encoding="utf-8")
    implementation = source.split("#[cfg(test)]", 1)[0]
    lib = LIB.read_text(encoding="utf-8")
    require("mod stream_info_static_numeric_spellings;" in lib, "private module route missing")
    require("pub mod stream_info_static_numeric_spellings" not in lib, "module became public")
    require(re.search(
        r"pub struct StreamInfoStaticNumericSpellings<'fields, 'definition>\s*\{\s*"
        r"_fields:\s*&'fields StreamInfoStaticFields<'definition>,\s*"
        r"channel_count:\s*String,\s*nominal_srate:\s*String,\s*\}",
        implementation,
    ) is not None, "projection must retain exactly one borrowed view and two owned strings")
    for marker in (
        "const MAX_CHANNEL_COUNT_BYTES: usize = 20;",
        "const NUMERIC_SPELLING_BYTES: usize = 17;",
        'const IRREGULAR_SPELLING: &str = "0.000000000000000";',
        "pub fn channel_count(&self) -> &str",
        "pub fn nominal_srate(&self) -> &str",
        "try_reserve_exact(digits)",
        "try_reserve_exact(source.len())",
        "UnsupportedRegularNominalSrate",
        "actual_bits: bits",
    ):
        require(marker in implementation, f"numeric projection invariant missing: {marker}")
    for _, _, spelling in EXPECTED:
        require(f'"{spelling}"' in implementation,
                f"accepted spelling missing from Rust source: {spelling}")
    for forbidden in ("format!", "to_string(", "to_owned(", "unsafe"):
        require(forbidden not in implementation, f"unbounded/general formatting surface opened: {forbidden}")
    require(
        implementation.index("let nominal_source = nominal_srate_spelling")
        < implementation.index("let channel_count = spell_channel_count"),
        "unsupported regular-rate domain must reject before either owned allocation",
    )
    tests = set(re.findall(r"fn (lslc_001l_[a-z0-9_]+)\(", source))
    require(tests == TESTS, "focused Rust test inventory drifted")


def observed_numeric_text(xml: str, tag: str) -> str:
    match = re.search(fr"<{tag}>([^<]*)</{tag}>", xml)
    require(match is not None, f"observed public XML omits {tag}")
    return match.group(1)


def validate_overlay() -> None:
    cases = load(CASES)
    observations = load(OBSERVATIONS)
    overlay = load(OVERLAY)
    require(observations.get("candidate_result") == {"status": "not-observed", "evidence": None},
            "LSLC-001H full-document candidate result changed")
    bindings = overlay.get("accepted_artifact_bindings", {})
    require(bindings.get("semantic_projection_overlay", {}).get("sha256") == BOUND_SHA256[K_OVERLAY],
            "LSLC-001K binding drifted")
    require(bindings.get("case_manifest", {}).get("sha256") == BOUND_SHA256[CASES],
            "LSLC-001H case binding drifted")
    require(bindings.get("observation_overlay", {}).get("sha256") == BOUND_SHA256[OBSERVATIONS],
            "LSLC-001H observation binding drifted")

    inputs = cases.get("positive_cases", [])
    observed = observations.get("observations", [])
    results = overlay.get("candidate_numeric_results", [])
    require(len(inputs) == len(observed) == len(results) == len(EXPECTED),
            "exact seven-case numeric matrix required")
    for case, observation, result, expected in zip(inputs, observed, results, EXPECTED):
        case_id, channel_count, nominal_srate = expected
        require(case.get("case_id") == observation.get("case_id") == result.get("case_id") == case_id,
                f"case order or identity drifted: {case_id}")
        require(str(case.get("channel_count")) == channel_count, f"case channel count drifted: {case_id}")
        public_xml = observation.get("public_xml_utf8", "")
        require(observed_numeric_text(public_xml, "channel_count") == channel_count,
                f"observed channel-count spelling drifted: {case_id}")
        require(observed_numeric_text(public_xml, "nominal_srate") == nominal_srate,
                f"observed nominal-rate spelling drifted: {case_id}")
        require(result == {
            "case_id": case_id,
            "result": "implemented-local-numeric-lexical-projection",
            "channel_count": channel_count,
            "nominal_srate": nominal_srate,
        }, f"candidate numeric result drifted: {case_id}")

    policy = overlay.get("fixed_policy", {})
    require(policy == {
        "channel_count_max_bytes": 20,
        "nominal_srate_bytes": 17,
        "irregular_spelling": "0.000000000000000",
        "accepted_regular_f64_values": [100.0, 59.94, 1.0, 256.5, 1000000.25],
        "unsupported_regular_policy": "reject-with-unchanged-f64-bits",
    }, "fixed fail-closed policy drifted")
    require(overlay.get("provenance", {}).get("implementation_inputs") == [],
            "external implementation input entered")


def validate_execution_docs_and_closure() -> None:
    result = subprocess.run(
        ["cargo", "test", "--workspace", "--all-targets", "--offline", "--locked",
         "stream_info_static_numeric_spellings::tests::lslc_001l_"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    )
    require("3 passed" in result.stdout, "focused LSLC-001L Rust tests did not all pass")
    docs = {
        "AGENTS.md": "check_lslc_001l.ps1",
        "README.md": "StreamInfoStaticNumericSpellings",
        "docs/ARCHITECTURE.md": "stream_info_static_numeric_spellings",
        "docs/COMPATIBILITY.md": "LSLC-001L",
        "docs/CORPUS.md": "LSLC-001L",
        "docs/PROVENANCE.md": "lslc-001l-static-numeric-spelling-results.json",
        "docs/VALIDATION.md": "check_lslc_001l.ps1",
        "morphospace/README.md": "rlsl-lslc-001l-static-numeric-spellings",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"),
                f"documentation route missing: {path}")

    protected = [
        "Cargo.toml", "Cargo.lock", "crates/rusty-lsl/Cargo.toml",
        "morphospace/feature.lock.json", "morphospace/project.spec.json",
        "morphospace/workspace.state.json", "morphospace/iteration-events.jsonl",
        "morphospace/iteration-units/rlsl-lslc-001l-static-numeric-spellings.json",
        "fixtures/compatibility/lslc-001h-stream-info-xml-cases.json",
        "fixtures/compatibility/lslc-001h-stream-info-xml-observations.json",
        "fixtures/compatibility/lslc-001h-stream-info-xml-provenance.json",
        "fixtures/compatibility/lslc-001k-stream-info-static-field-results.json",
        "tools/check_lslc_001h.py", "tools/check_lslc_001h.ps1",
        "tools/check_lslc_001k.py", "tools/check_lslc_001k.ps1",
    ]
    status = subprocess.run(
        ["git", "status", "--porcelain=v1", "--", *protected], cwd=ROOT,
        check=True, capture_output=True, text=True,
    ).stdout
    require(not status.strip(), "protected Cargo, lifecycle, fixture, or checker path changed")

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
    validate_immutable_h_and_k()
    validate_overlay()
    validate_execution_docs_and_closure()
    print("LSLC-001L bounded static numeric spelling checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
