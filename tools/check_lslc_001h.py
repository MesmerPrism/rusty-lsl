#!/usr/bin/env python3
# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Validate the bounded LSLC-001H black-box observation artifacts."""

from __future__ import annotations

import argparse
import ast
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/compatibility"
CASES = FIXTURES / "lslc-001h-stream-info-xml-cases.json"
OBSERVATIONS = FIXTURES / "lslc-001h-stream-info-xml-observations.json"
PROVENANCE = FIXTURES / "lslc-001h-stream-info-xml-provenance.json"
CORPUS = FIXTURES / "lslc-001a-stream-info-document-corpus.json"
UNIT = ROOT / "morphospace/iteration-units/rlsl-lslc-001h-stream-info-xml-black-box-observation.json"
DRIVER_PS1 = ROOT / "tools/oracle/Invoke-Lslc001hOracle.ps1"
DRIVER_PY = ROOT / "tools/oracle/lslc_001h_capture.py"

PROTECTED_PATHS = (
    "crates/rusty-lsl",
    "Cargo.toml",
    "Cargo.lock",
    "morphospace/feature.lock.json",
    "morphospace/project.spec.json",
)
PROTECTED_TREE_ENTRY_COUNT = 21
PROTECTED_TREE_SHA256 = "ee776163e904ea3c6eb336dd1855d12f0def3e257634272e0c33e7b6e784d8e1"
PROTECTED_TREE_MANIFEST = (
    b"100644 blob 9fa8457b15ab8c9afba84f82f6cc3f67a03f86de\tCargo.lock\n"
    b"100644 blob bdd26437cd3daa0be2df2a2c171753f642571a47\tCargo.toml\n"
    b"100644 blob a92227a58889233599fbe9984f83ec16e79c7bfa\tcrates/rusty-lsl/Cargo.toml\n"
    b"100644 blob c19f5a5afe7e86c40d497bfb13d534ed19a7c253\tcrates/rusty-lsl/src/descriptor_sample.rs\n"
    b"100644 blob fc5e85063843fd2691cf8453cd4b1ff2ae675458\tcrates/rusty-lsl/src/lib.rs\n"
    b"100644 blob ab0b4c30bb6084219a1f784bea7f313cd6f6bbeb\tcrates/rusty-lsl/src/metadata.rs\n"
    b"100644 blob b5cba5e834cebffbc434a3f71c23d63117537915\tcrates/rusty-lsl/src/metadata_tree.rs\n"
    b"100644 blob 5cd8d7e633ba2adc2b22e6c8498c6dcad9025816\tcrates/rusty-lsl/src/metadata_xml_projection.rs\n"
    b"100644 blob efb3ab6f9e1814b8bd77b8cb4c3f4b59d5c248bd\tcrates/rusty-lsl/src/sample.rs\n"
    b"100644 blob e5fd01c90552529ff384cd3e4d373a947c38b8ba\tcrates/rusty-lsl/src/stream_definition.rs\n"
    b"100644 blob 5b91ca6ec59876745628273ef939e744ef274588\tcrates/rusty-lsl/src/stream_descriptor.rs\n"
    b"100644 blob 9e158dcfd3aa8b410fa66a182d043edb10784470\tcrates/rusty-lsl/src/timestamped.rs\n"
    b"100644 blob 76b03c689b9c1d1a8b1ffff602550f45d652c034\tcrates/rusty-lsl/src/timestamped_descriptor_chunk.rs\n"
    b"100644 blob 00a8ccd6a501ad3d0f6d4e25a5897e09c4c0281f\tcrates/rusty-lsl/src/timestamped_descriptor_sample.rs\n"
    b"100644 blob 853cbc7db2e3fd37c0272f52f9771f131fa341e1\tcrates/rusty-lsl/src/xml_character_data.rs\n"
    b"100644 blob de632cc491b30400bd6fb5de19c566176e287083\tcrates/rusty-lsl/src/xml_element_serialization.rs\n"
    b"100644 blob 55ee4b72ac8cee73dae771a58c1f97b03cad1aed\tcrates/rusty-lsl/src/xml_element_tree.rs\n"
    b"100644 blob feadc048441143860491b74a4d6b5b13124bcc73\tcrates/rusty-lsl/src/xml_leaf_element.rs\n"
    b"100644 blob a80e0c7a29088396c8806cd8d48bb80e791680dd\tcrates/rusty-lsl/src/xml_value.rs\n"
    b"100644 blob 569129e9d8fab591ca0210d4b02ded568c80efbe\tmorphospace/feature.lock.json\n"
    b"100644 blob 062b62efe799ed301ef3f0a04b0c21dab205d1f7\tmorphospace/project.spec.json\n"
)
NESTED_PROTECTED_SURFACE_PROBE = "RUSTY_LSL_LSLC_001J_NESTED_PROBE"

CORPUS_SHA256 = "68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c"
WHEEL_SHA256 = "3ea2693417c7d79766cebf967250fde78aa1a3ad2b198e40246d36f549dbfde1"
DLL_SHA256 = "8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff"
DRIVER_PS1_SHA256 = "edf07ba073c7947558ac32c38e608fbbd3d344c715b88baa37512cfa5cd37e0f"
DRIVER_PY_SHA256 = "0e064fcd78f4352268cf37e6be8edd1510bfbfa2cbe029c9ecce83d8a9a25b40"
FORMATS = {
    "cf_float32": "float32", "cf_double64": "double64", "cf_string": "string",
    "cf_int32": "int32", "cf_int16": "int16", "cf_int8": "int8", "cf_int64": "int64",
}
INFO_ORDER = [
    "name", "type", "channel_count", "channel_format", "source_id",
    "nominal_srate", "version", "created_at", "uid", "session_id",
    "hostname", "v4address", "v4data_port", "v4service_port",
    "v6address", "v6data_port", "v6service_port", "desc",
]
RUNTIME_FIELDS = [
    "created_at", "uid", "session_id", "hostname", "v4address",
    "v4data_port", "v4service_port", "v6address", "v6data_port",
    "v6service_port",
]
DAMAGED_STAGES = {
    "wheel-digest", "python-architecture", "pylsl-version",
    "native-library-presence", "native-library-digest", "oracle-process-exit",
    "capture-output-bound", "capture-repeat", "evidence-shape", "public-boundary",
}
PRESERVED = {
    "baseline-provenance.json": "a71c13ef2dff7e4fa26abf5f6fe93ccc02a0c96349e950342970465955545ff9",
    "behavior-catalog.json": "e31d1ce0dceaec294ae932479815fa6072b35263f680df0ec5046dfc3ab602ee",
    "core-001-contract-results.json": "8e9a7bc0a6625c56c7da87e02d69e574304a48b329c93841984f19f9e44a3435",
    "core-002-contract-results.json": "c33517a3fb9c4b671ae7b56ed1fe6eb68adf855495ae54e0a332b528c6a273cd",
    "core-003-contract-results.json": "f64ed921b2576992a05ddd09bc02718b34455e0c441019d7bb8aee5e9fe60049",
    "core-004-contract-results.json": "664afa2f2d91ff7d1ed7355d31705fabdee5a80968a3eb15e2d9766388b2f1c6",
    "core-005-contract-results.json": "f56211fcb3abb50c1e170e112f338fad4791189f05c1866c01e4ffd8675abd0e",
    "core-006-contract-results.json": "3707cf05680c794c70141cf49acee7babf518b27488a4240e32e10414fd4e392",
    "core-007-contract-results.json": "8fb3b5f18a4ed95edb5d10bee097573e948d6b58c9d745540726416132a454ea",
    "core-008-contract-results.json": "8685b40dc5b3bb5ff68e3daf1c0d0be9daf4746aa52e6dad964eb2e7572f4d23",
    "lslc-001a-stream-info-document-corpus.json": CORPUS_SHA256,
    "lslc-001b-contract-results.json": "67ad7f72a1ef8c6474234de36a6864aa049b6e59170a17a7b4745f1ee51cd1b9",
    "lslc-001c-contract-results.json": "01d03280c76c9bd08476564e20ad7a80513e17bdb0ec56fbbedfc728b35ce7a3",
    "lslc-001d-contract-results.json": "8eb7256810956983bcf9763eaf31a8d47f2f7fc40000fc4ce1801bc5d7997ca7",
    "lslc-001e-contract-results.json": "32a0d2bb2a83bb84bc126e142996fb56c5bee744b86fb7e2753d69f72f99a9f3",
    "lslc-001f-contract-results.json": "22887579e874624ec13fb68365ccdb835d40bc42249c55053fcedb3606476438",
    "lslc-001g-contract-results.json": "4819c8cea026c534140104c8416cddb6dd9130c3bd433857e367aed1ff7d0e74",
    "negative-case-matrix.json": "1f88d6557eed93525fc52c76024b7531c79acc95cfe28302529abe5d95e3a6a1",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run_git(root: Path, arguments: list[str], label: str) -> subprocess.CompletedProcess[bytes]:
    try:
        result = subprocess.run(
            ["git", *arguments], cwd=root, check=False, capture_output=True,
        )
    except OSError as error:
        raise ValueError(f"Git setup failed during {label}") from error
    require(result.returncode == 0, f"Git command failed during {label}")
    return result


def validate_protected_surface(root: Path) -> None:
    manifest = run_git(
        root,
        ["ls-tree", "-r", "--full-tree", "HEAD", "--", *PROTECTED_PATHS],
        "protected HEAD tree manifest",
    ).stdout
    require(
        len(manifest.splitlines()) == PROTECTED_TREE_ENTRY_COUNT,
        "protected HEAD tree entry count drifted",
    )
    require(
        hashlib.sha256(manifest).hexdigest() == PROTECTED_TREE_SHA256,
        "protected HEAD tree manifest digest drifted",
    )
    require(manifest == PROTECTED_TREE_MANIFEST,
            "protected HEAD tree path, mode, object, or output bytes drifted")

    tracked = subprocess.run(
        ["git", "diff", "--quiet", "HEAD", "--", *PROTECTED_PATHS],
        cwd=root, check=False, capture_output=True,
    )
    require(tracked.returncode in {0, 1},
            "Git command failed during protected working tree/index comparison")
    require(tracked.returncode == 0, "protected working tree or index changed")

    untracked = run_git(
        root,
        ["ls-files", "--others", "--", *PROTECTED_PATHS],
        "protected untracked-path inventory",
    ).stdout
    require(not untracked, "untracked protected path entered the repository")


def validate_protected_surface_damaged_checks() -> None:
    def run_checker(
        clone: Path, label: str, expected_message: str | None = None,
    ) -> None:
        environment = os.environ.copy()
        environment[NESTED_PROTECTED_SURFACE_PROBE] = "1"
        result = subprocess.run(
            [sys.executable, "tools/check_lslc_001h.py"],
            cwd=clone, check=False, capture_output=True, env=environment,
        )
        combined = result.stdout + result.stderr
        if expected_message is None:
            require(
                result.returncode == 0,
                f"one-commit shallow checker probe failed: {label}: "
                f"{combined[-2000:].decode('utf-8', errors='replace')}",
            )
        else:
            require(result.returncode != 0,
                    f"damaged protected-surface probe was accepted: {label}")
            require(
                expected_message.encode("utf-8") in combined,
                f"damaged protected-surface probe failed at the wrong boundary: {label}",
            )

    with tempfile.TemporaryDirectory(prefix="rusty-lsl-lslc-001j-") as temporary:
        temporary_root = Path(temporary)
        clone = temporary_root / "shallow"
        clone_result = subprocess.run(
            [
                "git", "clone", "--quiet", "--depth", "1", "--single-branch",
                ROOT.resolve().as_uri(), str(clone),
            ],
            check=False, capture_output=True,
        )
        require(clone_result.returncode == 0, "one-commit local shallow clone failed")
        shutil.copyfile(Path(__file__), clone / "tools/check_lslc_001h.py")

        shallow = run_git(
            clone, ["rev-parse", "--is-shallow-repository"], "shallow identity",
        ).stdout.strip()
        commit_count = run_git(
            clone, ["rev-list", "--count", "HEAD"], "shallow commit count",
        ).stdout.strip()
        require(shallow == b"true" and commit_count == b"1",
                "local clone is not an exact one-commit shallow checkout")
        historical = subprocess.run(
            ["git", "cat-file", "-e", "9650de4^{commit}"],
            cwd=clone, check=False, capture_output=True,
        )
        require(historical.returncode != 0,
                "historical revision unexpectedly entered the shallow clone")
        run_checker(clone, "clean shallow checkout")

        protected_file = clone / "Cargo.toml"
        with protected_file.open("ab") as stream:
            stream.write(b"\n# lslc-001j-working-tree-probe\n")
        run_checker(
            clone, "tracked working-tree drift",
            "protected working tree or index changed",
        )
        run_git(
            clone,
            ["restore", "--source=HEAD", "--staged", "--worktree", "--", "Cargo.toml"],
            "working-tree probe restoration",
        )

        with protected_file.open("ab") as stream:
            stream.write(b"\n# lslc-001j-index-probe\n")
        run_git(clone, ["add", "--", "Cargo.toml"], "index probe staging")
        run_checker(
            clone, "staged index drift", "protected working tree or index changed",
        )
        run_git(
            clone,
            ["restore", "--source=HEAD", "--staged", "--worktree", "--", "Cargo.toml"],
            "index probe restoration",
        )

        untracked = clone / "crates/rusty-lsl/lslc-001j-untracked-probe"
        untracked.write_bytes(b"synthetic untracked protected-path probe\n")
        run_checker(
            clone, "untracked protected path",
            "untracked protected path entered the repository",
        )
        untracked.unlink()

        ignored = clone / "crates/rusty-lsl/src/lib.rs.bk"
        ignored.write_bytes(b"synthetic ignored protected-path probe\n")
        run_git(
            clone,
            ["check-ignore", "--quiet", "--", "crates/rusty-lsl/src/lib.rs.bk"],
            "ignored protected-path probe identity",
        )
        run_checker(
            clone, "ignored untracked protected path",
            "untracked protected path entered the repository",
        )
        ignored.unlink()

        with protected_file.open("ab") as stream:
            stream.write(b"\n# lslc-001j-manifest-probe\n")
        run_git(clone, ["add", "--", "Cargo.toml"], "manifest probe staging")
        run_git(
            clone,
            [
                "-c", "user.name=Rusty LSL Validation",
                "-c", "user.email=validation.invalid",
                "commit", "--quiet", "--no-verify", "-m", "LSLC-001J manifest probe",
            ],
            "manifest probe commit",
        )
        run_checker(
            clone, "exact HEAD manifest mutation",
            "protected HEAD tree manifest digest drifted",
        )


def canonical_lf_bound_driver_source(source: bytes, label: str) -> bytes:
    without_crlf = source.replace(b"\r\n", b"")
    require(b"\r" not in without_crlf, f"lone carriage return in bound driver: {label}")
    if b"\r\n" in source:
        require(b"\n" not in without_crlf,
                f"mixed LF/CRLF line endings in bound driver: {label}")
        return source.replace(b"\r\n", b"\n")
    return source


def validate_bound_driver_source(
    path: Path, expected_sha256: str,
) -> bytes:
    label = path.relative_to(ROOT).as_posix()
    working_tree_source = path.read_bytes()
    canonical = canonical_lf_bound_driver_source(working_tree_source, label)
    require(hashlib.sha256(canonical).hexdigest() == expected_sha256,
            f"canonical LF driver digest binding drifted: {label}")

    complete_crlf = canonical.replace(b"\n", b"\r\n")
    require(canonical_lf_bound_driver_source(canonical, f"{label}:lf-check") == canonical,
            f"complete LF driver canonicalization drifted: {label}")
    require(canonical_lf_bound_driver_source(
        complete_crlf, f"{label}:crlf-check",
    ) == canonical, f"complete CRLF driver canonicalization drifted: {label}")
    require(hashlib.sha256(canonical_lf_bound_driver_source(
        complete_crlf, f"{label}:crlf-digest-check",
    )).hexdigest() == expected_sha256,
            f"complete CRLF driver digest equivalence drifted: {label}")

    require(canonical.count(b"\n") >= 2,
            f"bound driver needs two lines for mixed-ending validation: {label}")
    mixed = canonical.replace(b"\n", b"\r\n", 1)
    lone_cr = canonical.replace(b"\n", b"\r", 1)
    for damaged, damage in ((mixed, "mixed"), (lone_cr, "lone-cr")):
        try:
            canonical_lf_bound_driver_source(damaged, f"{label}:{damage}-check")
        except ValueError:
            pass
        else:
            raise ValueError(f"damaged driver line endings were accepted: {label}:{damage}")

    mutation_index = next(
        (index for index, byte in enumerate(canonical) if byte not in (0x0A, 0x0D)),
        None,
    )
    require(mutation_index is not None,
            f"bound driver has no non-line-ending byte to mutate: {label}")
    mutated = bytearray(canonical)
    mutated[mutation_index] ^= 0x01
    mutated_canonical = canonical_lf_bound_driver_source(
        bytes(mutated), f"{label}:content-mutation-check",
    )
    require(hashlib.sha256(mutated_canonical).hexdigest() != expected_sha256,
            f"non-line-ending driver mutation retained canonical digest: {label}")
    return canonical


def raw_element_text(xml: bytes, name: str) -> str:
    pattern = re.compile(
        rb"<" + re.escape(name.encode("ascii")) + rb">([^<]*)</"
        + re.escape(name.encode("ascii")) + rb">"
    )
    matches = list(pattern.finditer(xml))
    require(len(matches) == 1, f"simple element occurrence drifted: {name}")
    return matches[0].group(1).decode("utf-8")


def description_records(element: ET.Element, path: tuple[str, ...] = ()) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for child in list(element):
        if list(child):
            records.extend(description_records(child, path + (child.tag,)))
        else:
            records.append({
                "path": list(path),
                "name": child.tag,
                "value": child.text or "",
            })
    return records


def validate_public_xml(item: dict[str, Any], case: dict[str, Any]) -> None:
    case_id = case["case_id"]
    public_xml = item["public_xml_utf8"]
    encoded = public_xml.encode("utf-8")
    try:
        root = ET.fromstring(public_xml)
    except ET.ParseError as error:
        raise ValueError(f"public XML is not well formed: {case_id}") from error
    require(root.tag == "info", f"public XML root drifted: {case_id}")
    children = list(root)
    child_order = [child.tag for child in children]
    require(child_order == INFO_ORDER, f"public XML direct child order drifted: {case_id}")
    require(len(children) == len(INFO_ORDER), f"public XML child count drifted: {case_id}")

    by_name = {child.tag: child for child in children}
    require((by_name["name"].text or "") == case["name"], f"decoded name drifted: {case_id}")
    require((by_name["type"].text or "") == case["type"], f"decoded type drifted: {case_id}")
    require((by_name["source_id"].text or "") == case["source_id"],
            f"decoded source_id drifted: {case_id}")
    for field in RUNTIME_FIELDS:
        require((by_name[field].text or "") == f"NORMALIZED_{field.upper()}",
                f"normalized runtime field drifted: {case_id}:{field}")

    observed = item["observed_dimensions"]
    require(observed.get("direct_info_child_order") == child_order,
            f"claimed direct order does not match public XML: {case_id}")
    require(raw_element_text(encoded, "channel_count") ==
            observed.get("channel_count_spelling") == str(case["channel_count"]),
            f"channel count spelling does not match public XML: {case_id}")
    require(raw_element_text(encoded, "nominal_srate") ==
            observed.get("nominal_srate_spelling"),
            f"nominal rate spelling does not match public XML: {case_id}")
    require(raw_element_text(encoded, "channel_format") ==
            observed.get("channel_format_spelling") == FORMATS[case["channel_format_symbol"]],
            f"channel format spelling does not match public XML: {case_id}")

    whitespace = re.findall(rb">([\x09\x0a\x0d\x20]+)<", encoded)
    whitespace_digest = hashlib.sha256(b"\x00".join(whitespace)).hexdigest()
    require(bool(whitespace) and observed.get("whitespace_between_markup") == "present",
            f"inter-element whitespace does not match public XML: {case_id}")
    require(observed.get("whitespace_run_sha256") == whitespace_digest,
            f"whitespace digest does not match public XML: {case_id}")

    desc = by_name["desc"]
    actual_description = description_records(desc)
    require(actual_description == case["description"],
            f"nested description structure/order drifted: {case_id}")
    require(observed.get("description_leaf_order") ==
            [entry["name"] for entry in actual_description],
            f"claimed description order does not match public XML: {case_id}")
    require(observed.get("desc_direct_child_index") == child_order.index("desc"),
            f"claimed desc placement does not match public XML: {case_id}")
    if case["description"]:
        require(observed.get("empty_desc_form") is None,
                f"populated desc was classified as empty: {case_id}")
    else:
        require(re.search(rb"<desc />", encoded) is not None and not list(desc),
                f"empty desc spelling does not match public XML: {case_id}")
        require(observed.get("empty_desc_form") == "<desc />",
                f"claimed empty desc spelling drifted: {case_id}")

    spellings = observed.get("character_data_spellings", {})
    require(spellings.get("name") == raw_element_text(encoded, "name"),
            f"name character-data spelling does not match public XML: {case_id}")
    require(spellings.get("type") == raw_element_text(encoded, "type"),
            f"type character-data spelling does not match public XML: {case_id}")
    require(spellings.get("source_id") == raw_element_text(encoded, "source_id"),
            f"source_id character-data spelling does not match public XML: {case_id}")
    require(spellings.get("description_values") ==
            [raw_element_text(encoded, entry["name"]) for entry in case["description"]],
            f"description character-data spelling does not match public XML: {case_id}")
    require(observed.get("character_data_inputs") == {
        "name": case["name"],
        "type": case["type"],
        "source_id": case["source_id"],
        "description_values": [entry["value"] for entry in case["description"]],
    }, f"claimed character-data inputs drifted: {case_id}")


def validate_history_and_corpus() -> None:
    for name, expected in PRESERVED.items():
        require(digest(FIXTURES / name) == expected, f"accepted historical artifact changed: {name}")
    corpus = load(CORPUS)
    for case in corpus.get("cases", []):
        require(case.get("oracle_observation") == {"status": "not-observed", "evidence": None},
                f"frozen corpus oracle role changed: {case.get('case_id')}")
        require(case.get("candidate_result") == {"status": "not-observed", "evidence": None},
                f"frozen corpus candidate role changed: {case.get('case_id')}")


def validate_cases() -> dict[str, dict[str, Any]]:
    cases = load(CASES)
    require(cases.get("schema") == "rusty.lsl.compatibility.stream_info_xml_black_box_cases.v1",
            "case schema drifted")
    bounds = cases.get("bounds", {})
    require(bounds == {
        "max_cases": 16, "captures_per_case": 2, "max_xml_bytes_per_capture": 16384,
        "max_stdout_bytes": 65536, "max_stderr_bytes": 65536, "max_desc_nodes": 32,
        "max_desc_depth": 8, "max_text_code_points": 512,
    }, "case bounds drifted")
    positive = cases.get("positive_cases", [])
    by_id = {case.get("case_id"): case for case in positive}
    require(len(by_id) == len(positive) == 7, "positive case inventory drifted")
    require({case.get("channel_format_symbol") for case in positive} == set(FORMATS),
            "seven data-only format cases are required")
    rates = {case.get("nominal_srate") for case in positive}
    require(0.0 in rates and {1.0, 59.94, 100.0, 256.5, 1000000.25} <= rates,
            "irregular or representative regular rates are missing")
    require(any(case.get("type") == "" and case.get("source_id") == "" for case in positive),
            "empty optional core text is missing")
    require(any(case.get("type") and case.get("source_id") for case in positive),
            "populated optional core text is missing")
    sensitive = by_id.get("regular-string-unicode-sensitive-nested-desc", {})
    require("Ω" in sensitive.get("name", "") and "&" in sensitive.get("name", "") and
            "<" in sensitive.get("name", "") and ">" in sensitive.get("name", ""),
            "Unicode/XML-sensitive core text is missing")
    require([entry.get("name") for entry in sensitive.get("description", [])] ==
            ["first", "second", "third"], "ordered nested description drifted")
    require(len(sensitive["description"][-1].get("path", [])) == 2,
            "nested description depth case is missing")
    stages = {case.get("expected_stage") for case in cases.get("damaged_cases", [])}
    require(stages == DAMAGED_STAGES, "damaged failure-stage coverage drifted")
    require(isinstance(cases.get("does_not_prove"), list) and cases["does_not_prove"],
            "case limitations are missing")
    return by_id


def validate_observations(cases: dict[str, dict[str, Any]]) -> dict[str, Any]:
    overlay = load(OBSERVATIONS)
    require(overlay.get("schema") ==
            "rusty.lsl.compatibility.stream_info_xml_black_box_observations.v1",
            "observation schema drifted")
    require(overlay.get("origin_classification") == "black-box-observed",
            "observation classification drifted")
    require(overlay.get("corpus_binding", {}).get("sha256") == CORPUS_SHA256,
            "frozen corpus digest binding drifted")
    require(overlay.get("case_manifest_binding", {}).get("sha256") == digest(CASES),
            "case manifest digest binding drifted")
    oracle = overlay.get("oracle_binding", {})
    require(oracle.get("distribution_version") == "1.18.2" and
            oracle.get("wheel_sha256") == WHEEL_SHA256, "wheel identity drifted")
    require(oracle.get("liblsl_public_library_version") == 117 and
            oracle.get("native_dll_sha256") == DLL_SHA256, "native identity drifted")
    require(oracle.get("python_version") == "3.12.10" and
            oracle.get("python_architecture") == "AMD64" and oracle.get("python_bits") == 64,
            "Python identity drifted")
    policy = overlay.get("capture_policy", {})
    require(policy.get("outlets_created") == policy.get("inlets_created") ==
            policy.get("discovery_calls") == policy.get("network_calls") == 0,
            "runtime or network behavior entered the observation")
    require(policy.get("captures_per_case") == 2 and
            policy.get("raw_evidence_location") == "external-only",
            "capture or raw-evidence policy drifted")

    observations = overlay.get("observations", [])
    by_id = {item.get("case_id"): item for item in observations}
    require(set(by_id) == set(cases) and len(by_id) == len(observations),
            "observation inventory drifted")
    for case_id, case in cases.items():
        item = by_id[case_id]
        captures = item.get("captures")
        require(isinstance(captures, list) and len(captures) == 2,
                f"two captures are required: {case_id}")
        require(captures[0]["sha256"] == captures[1]["sha256"] and
                captures[0]["byte_length"] == captures[1]["byte_length"],
                f"raw repeat mismatch: {case_id}")
        require(item.get("repeat_comparison") == "raw-and-normalized-byte-identical",
                f"repeat classification drifted: {case_id}")
        public_xml = item.get("public_xml_utf8")
        require(isinstance(public_xml, str), f"public XML missing: {case_id}")
        encoded = public_xml.encode("utf-8")
        require(len(encoded) == item.get("public_xml_byte_length") <= 16384,
                f"public XML bound or length drifted: {case_id}")
        require(hashlib.sha256(encoded).hexdigest() == item.get("public_xml_sha256"),
                f"public XML digest drifted: {case_id}")
        require("<info>" in public_xml and public_xml.endswith("</info>\n"),
                f"bounded info document shape drifted: {case_id}")
        validate_public_xml(item, case)
        for pattern in (
            r"[A-Za-z]:[\\/]", r"\\\\[^\\\s]+[\\/]", r"(?:\d{1,3}\.){3}\d{1,3}",
            r"(?i)(?:password|credential|secret|token|private[_-]?key)\s*[:=]",
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b",
        ):
            require(re.search(pattern, public_xml) is None,
                    f"prohibited public content entered XML: {case_id}")
        operations = item.get("normalization_operations")
        require(isinstance(operations, list) and [op.get("field") for op in operations] ==
                RUNTIME_FIELDS, f"normalization operation inventory drifted: {case_id}")
        positions = [op.get("raw_start_byte") for op in operations]
        require(all(isinstance(value, int) and value >= 0 for value in positions) and
                positions == sorted(positions), f"normalization positions drifted: {case_id}")
        for operation in operations:
            require(operation.get("raw_end_byte_exclusive") >= operation["raw_start_byte"],
                    f"normalization range drifted: {case_id}")
            require(operation.get("raw_length_bytes") ==
                    operation["raw_end_byte_exclusive"] - operation["raw_start_byte"],
                    f"normalization length drifted: {case_id}")
            require(operation.get("replacement_utf8") ==
                    f"NORMALIZED_{operation['field'].upper()}",
                    f"normalization replacement drifted: {case_id}")

        observed = item.get("observed_dimensions", {})
        require(observed.get("direct_info_child_order") == INFO_ORDER,
                f"direct info order drifted: {case_id}")
        require(observed.get("whitespace_between_markup") == "present",
                f"whitespace observation drifted: {case_id}")
        require(observed.get("desc_direct_child_index") == 17,
                f"desc placement drifted: {case_id}")
        require(observed.get("channel_count_spelling") == str(case["channel_count"]),
                f"channel count spelling drifted: {case_id}")
        require(observed.get("channel_format_spelling") ==
                FORMATS[case["channel_format_symbol"]], f"format spelling drifted: {case_id}")
        if case["description"]:
            require(observed.get("empty_desc_form") is None and
                    observed.get("description_leaf_order") == ["first", "second", "third"],
                    "populated nested desc observation drifted")
        else:
            require(observed.get("empty_desc_form") == "<desc />",
                    f"empty desc form drifted: {case_id}")

    sensitive = by_id["regular-string-unicode-sensitive-nested-desc"]["observed_dimensions"]
    spellings = sensitive.get("character_data_spellings", {})
    require(spellings.get("name") == "unicode-Ω-中-&amp;-&lt;-greater-&gt;",
            "core character-data representation drifted")
    require(spellings.get("type") == "text-&amp;-&lt;-greater-&gt;-\"-'",
            "quote/apostrophe character-data representation drifted")
    require(spellings.get("description_values") == [
        "alpha-α-&amp;-&lt;-greater-&gt;-\"-'",
        "beta-β-]]&gt;",
        "tail-尾-&amp;-&lt;-greater-&gt;",
    ], "nested character-data representation drifted")
    require(overlay.get("candidate_result") == {"status": "not-observed", "evidence": None},
            "future candidate role was promoted")
    require(isinstance(overlay.get("does_not_prove"), list) and overlay["does_not_prove"],
            "observation limitations are missing")
    return overlay


def validate_provenance_and_driver() -> dict[str, Any]:
    provenance = load(PROVENANCE)
    require(provenance.get("schema") == "rusty.lsl.provenance.oracle_observation.v1",
            "provenance schema drifted")
    boundary = provenance.get("source_boundary", {})
    require(boundary == {
        "source_code_used": False, "implementation_input": False,
        "package_or_native_source_inspected": False,
        "public_documentation_and_black_box_returns_only": True,
    }, "source boundary drifted")
    require(provenance.get("implementation_inputs") == [], "implementation input entered")
    require(provenance.get("distribution", {}).get("wheel_sha256") == WHEEL_SHA256,
            "provenance wheel digest drifted")
    require(provenance.get("native_library", {}).get("dll_sha256") == DLL_SHA256,
            "provenance DLL digest drifted")
    bindings = provenance.get("bindings", {})
    require(bindings.get("corpus", {}).get("sha256") == CORPUS_SHA256,
            "provenance corpus binding drifted")
    require(bindings.get("case_manifest", {}).get("sha256") == digest(CASES),
            "provenance case binding drifted")
    expected_driver_bindings = {
        "powershell_driver": (DRIVER_PS1, DRIVER_PS1_SHA256),
        "python_driver": (DRIVER_PY, DRIVER_PY_SHA256),
    }
    canonical_driver_sources: dict[str, bytes] = {}
    for binding_name, (path, expected_sha256) in expected_driver_bindings.items():
        binding = bindings.get(binding_name, {})
        require(binding.get("sha256") == expected_sha256,
                f"unchanged driver SHA-256 binding drifted: {binding_name}")
        require(binding.get("digest_basis") == "canonical-lf-source-bytes",
                f"canonical LF digest basis missing: {binding_name}")
        canonical_driver_sources[binding_name] = validate_bound_driver_source(
            path, expected_sha256,
        )
    external = provenance.get("external_evidence", {})
    require(external.get("raw_xml_committed") is False and external.get("raw_outputs"),
            "raw evidence boundary drifted")
    require(external.get("stdout_bytes", 65537) <= 65536 and
            external.get("stderr_bytes", 65537) <= 65536,
            "bounded process evidence drifted")
    raw_inventory = {
        (item.get("case_id"), item.get("capture_index")):
        (item.get("sha256"), item.get("byte_length"))
        for item in external.get("raw_outputs", [])
    }
    require(len(raw_inventory) == 14, "raw provenance inventory drifted")
    observations = load(OBSERVATIONS).get("observations", [])
    expected_raw = {
        (item["case_id"], capture["capture_index"]):
        (capture["sha256"], capture["byte_length"])
        for item in observations for capture in item["captures"]
    }
    require(raw_inventory == expected_raw,
            "raw provenance inventory does not match observations")
    normalization = provenance.get("normalization", {})
    for dimension in (
        "core-field order", "all markup and tag spelling", "all whitespace",
        "empty-element form", "channel-count and nominal-rate spelling",
        "channel-format spelling", "caller-supplied character-data bytes",
        "desc placement and nested metadata order",
    ):
        require(dimension in normalization.get("preserved_dimensions", []),
                f"normalization preservation is missing: {dimension}")
    failure = provenance.get("failure_policy", {})
    require(failure.get("append_only") is True and
            failure.get("successful_capture_must_not_remove_prior_failures") is True,
            "failure-history preservation drifted")
    require(DAMAGED_STAGES <= set(failure.get("typed_stages", [])),
            "typed failure stages drifted")

    ps1 = canonical_driver_sources["powershell_driver"].decode("utf-8")
    py = canonical_driver_sources["python_driver"].decode("utf-8")
    ast.parse(py)
    for marker in (
        "--no-deps", "--no-index", WHEEL_SHA256, DLL_SHA256, "$maxProcessOutputBytes",
        "failure-history.jsonl", "Start-Process", "RedirectStandardError",
    ):
        require(marker in ps1, f"PowerShell fail-closed marker missing: {marker}")
    for stage in DAMAGED_STAGES | {"python-version", "native-library-version"}:
        require(stage in ps1 or stage in py, f"typed failure stage missing from driver: {stage}")
    for prohibited in (
        "StreamOutlet(", "StreamInlet(", "resolve_stream", "resolve_byprop",
        "resolve_bypred", "local_clock(", "time_correction(", "pull_sample(",
        "push_sample(", "push_chunk(",
    ):
        require(prohibited not in py, f"prohibited LSL call entered driver: {prohibited}")
    require("import pylsl" in py and "pylsl.StreamInfo(" in py and
            ".as_xml()" in py and ".desc()" in py,
            "documented public black-box calls are missing")

    tracked = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard"],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.splitlines()
    for path in tracked:
        lowered = path.lower()
        require(not lowered.endswith((".whl", ".dll", ".exe", ".pyd", ".pyc")),
                f"native/package artifact entered repository: {path}")
        require("/venv/" not in f"/{lowered}/" and "__pycache__" not in lowered,
                f"environment/cache artifact entered repository: {path}")
    return provenance


def validate_external_evidence(
    external_root: Path, overlay: dict[str, Any], provenance: dict[str, Any],
) -> None:
    root = external_root.resolve()
    require(root != ROOT and ROOT not in root.parents,
            "external evidence root must remain outside the repository")
    capture_dir = root / "capture"
    external = provenance["external_evidence"]

    def verify(path: Path, expected_sha: str, expected_bytes: int, label: str) -> bytes:
        require(path.is_file(), f"external artifact missing: {label}")
        data = path.read_bytes()
        require(len(data) == expected_bytes, f"external artifact length drifted: {label}")
        require(hashlib.sha256(data).hexdigest() == expected_sha,
                f"external artifact digest drifted: {label}")
        return data

    record_bytes = verify(
        capture_dir / "capture-record.json",
        external["capture_record_sha256"], external["capture_record_bytes"],
        "capture-record",
    )
    verify(capture_dir / "driver-stdout.bin", external["stdout_sha256"],
           external["stdout_bytes"], "driver-stdout")
    verify(capture_dir / "driver-stderr.bin", external["stderr_sha256"],
           external["stderr_bytes"], "driver-stderr")
    record = json.loads(record_bytes.decode("utf-8"))
    require(record.get("schema") == "rusty.lsl.oracle.external_capture.v1" and
            record.get("classification") == "accepted",
            "external capture-record identity drifted")
    require(record.get("case_manifest_sha256") == digest(CASES),
            "external capture-record case binding drifted")
    require(record.get("observations") == overlay.get("observations"),
            "committed observations differ from external capture record")
    window = overlay.get("capture_window", {})
    require(record.get("started_at_utc") == window.get("started_at_utc") and
            record.get("finished_at_utc") == window.get("finished_at_utc"),
            "capture window differs from external record")

    wheel = root / "wheelhouse" / provenance["distribution"]["wheel_filename"]
    require(wheel.is_file() and digest(wheel) == WHEEL_SHA256,
            "external wheel identity drifted")
    dlls = list((root / "venv").rglob("lsl.dll"))
    require(len(dlls) == 1 and digest(dlls[0]) == DLL_SHA256,
            "external native DLL identity drifted")

    raw_provenance = {
        (entry["case_id"], entry["capture_index"]): entry
        for entry in external["raw_outputs"]
    }
    for item in overlay["observations"]:
        for capture in item["captures"]:
            key = (item["case_id"], capture["capture_index"])
            provenance_entry = raw_provenance[key]
            raw_path = capture_dir / "raw" / capture["external_file_name"]
            raw = verify(raw_path, capture["sha256"], capture["byte_length"],
                         f"raw:{key[0]}:{key[1]}")
            require((capture["sha256"], capture["byte_length"]) ==
                    (provenance_entry["sha256"], provenance_entry["byte_length"]),
                    f"external raw provenance drifted: {key}")
            normalized = bytearray(raw)
            for operation in reversed(item["normalization_operations"]):
                field = operation["field"]
                pattern = re.compile(
                    rb"<" + field.encode("ascii") + rb">([^<]*)</"
                    + field.encode("ascii") + rb">"
                )
                matches = list(pattern.finditer(raw))
                require(len(matches) == 1, f"external runtime field drifted: {key}:{field}")
                match = matches[0]
                require((match.start(1), match.end(1)) ==
                        (operation["raw_start_byte"], operation["raw_end_byte_exclusive"]),
                        f"normalization range does not match raw evidence: {key}:{field}")
                normalized[match.start(1):match.end(1)] = operation["replacement_utf8"].encode("ascii")
            require(bytes(normalized) == item["public_xml_utf8"].encode("utf-8"),
                    f"normalized public XML does not reconstruct from raw evidence: {key}")

    for directory in (root / "process-temp", root / "pip-cache"):
        require(directory.is_dir() and not any(directory.iterdir()),
                f"external temporary/cache directory is not empty: {directory.name}")
    history = root / "failure-history.jsonl"
    if provenance["failure_policy"]["accepted_capture_failure_count"] == 0:
        require(not history.exists() or history.stat().st_size == 0,
                "accepted external root contains unreported failures")


def validate_docs_and_instructions() -> None:
    unit = load(UNIT)
    require(unit.get("status") in {"active", "validating", "accepted"},
            "unit lifecycle state is invalid")
    statuses = {row.get("path"): row.get("status") for row in unit.get("instruction_surfaces", [])}
    require(statuses.get("AGENTS.md") == statuses.get("README.md") == "complete",
            "updated repository instruction surfaces are incomplete")
    skill_statuses = {
        statuses.get("<skills-root>/system-engineering/SKILL.md"),
        statuses.get("<skills-root>/rusty-morphospace-context/SKILL.md"),
    }
    require(skill_statuses <= {"planned", "complete"} and None not in skill_statuses,
            "installed skill review status is invalid")
    if unit.get("status") == "accepted":
        require(all(row.get("status") == "complete" for row in unit.get("instruction_surfaces", [])),
                "accepted unit has incomplete instruction review")
    docs = {
        "AGENTS.md": "LSLC-001H", "README.md": "LSLC-001H",
        "docs/ARCHITECTURE.md": "black-box observation boundary",
        "docs/COMPATIBILITY.md": "lslc-001h-stream-info-xml-observations.json",
        "docs/CORPUS.md": "LSLC-001H append-only observation",
        "docs/ORACLE.md": "pylsl 1.18.2",
        "docs/PROVENANCE.md": "lslc-001h-stream-info-xml-provenance.json",
        "docs/VALIDATION.md": "check_lslc_001h.ps1",
        "fixtures/compatibility/README.md": "lslc-001h-stream-info-xml-cases.json",
        "morphospace/README.md": "rlsl-lslc-001h-stream-info-xml-black-box-observation",
    }
    for path, marker in docs.items():
        require(marker in (ROOT / path).read_text(encoding="utf-8"),
                f"documentation route missing: {path}")
    require("check_lslc_001h.ps1" in (ROOT / "tools/check_all.ps1").read_text(encoding="utf-8"),
            "aggregate gate does not route LSLC-001H")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--external-root", type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    validate_protected_surface(ROOT)
    validate_history_and_corpus()
    cases = validate_cases()
    overlay = validate_observations(cases)
    provenance = validate_provenance_and_driver()
    if args.external_root is not None:
        validate_external_evidence(args.external_root, overlay, provenance)
    validate_docs_and_instructions()
    if os.environ.get(NESTED_PROTECTED_SURFACE_PROBE) != "1":
        validate_protected_surface_damaged_checks()
        print(
            "LSLC-001J one-commit shallow and damaged protected-surface checks passed."
        )
    print("LSLC-001H black-box observation checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
