#!/usr/bin/env python3
"""Damaged extraction tests for LSLC-003L."""

import importlib.util
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("check_lslc_003l", ROOT / "tools/check_lslc_003l.py")
gate = importlib.util.module_from_spec(SPEC)
assert SPEC.loader
SPEC.loader.exec_module(gate)


class HistoryExtractionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.original = subprocess.run(
            ["git", "show", f"{gate.BASE}:AGENTS.md"], cwd=ROOT, capture_output=True, check=True
        ).stdout
        cls.archive = (ROOT / "docs/history/LSLC-WORK-UNIT-HISTORY.md").read_bytes()
        cls.current = (ROOT / "AGENTS.md").read_bytes()
        cls.readme = (ROOT / "README.md").read_bytes()
        cls.validation = (ROOT / "docs/VALIDATION.md").read_bytes()

    def test_live_extraction(self):
        gate.validate(self.original, self.archive, self.current, self.readme, self.validation)

    def assert_archive_rejected(self, archive):
        with self.assertRaises(gate.ExtractionError):
            gate.validate(self.original, archive, self.current, self.readme, self.validation)

    def test_omitted_byte_rejected(self):
        self.assert_archive_rejected(self.archive[:-1])

    def test_changed_byte_rejected(self):
        self.assert_archive_rejected(b"X" + self.archive[1:])

    def test_duplicate_payload_rejected(self):
        self.assert_archive_rejected(self.archive + self.archive)

    def test_reordered_sections_rejected(self):
        marker = self.archive.find(b"\n## LSLC-003J")
        self.assertGreater(marker, 0)
        self.assert_archive_rejected(self.archive[marker + 1 :] + self.archive[: marker + 1])

    def test_wrong_boundary_rejected(self):
        damaged = self.original.replace(b"## LSLC-003K ", b"## LSLC-003X ", 1)
        with self.assertRaises(gate.ExtractionError):
            gate.split_original(damaged)

    def test_missing_router_link_rejected(self):
        with self.assertRaises(gate.ExtractionError):
            gate.validate(self.original, self.archive, self.current, self.readme.replace(b"docs/history/LSLC-WORK-UNIT-HISTORY.md", b"missing"), self.validation)


if __name__ == "__main__":
    unittest.main()
