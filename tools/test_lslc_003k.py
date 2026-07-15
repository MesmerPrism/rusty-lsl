#!/usr/bin/env python3
"""Damaged-baseline tests for LSLC-003K."""

import copy
import importlib.util
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("check_lslc_003k", ROOT / "tools" / "check_lslc_003k.py")
policy = importlib.util.module_from_spec(SPEC)
assert SPEC.loader
SPEC.loader.exec_module(policy)


class ClippyBaselineTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.document = json.loads(policy.BASELINE.read_text(encoding="utf-8"))

    def test_real_baseline_is_canonical(self):
        policy.validate_document(copy.deepcopy(self.document))

    def assert_invalid(self, mutate):
        damaged = copy.deepcopy(self.document)
        mutate(damaged)
        with self.assertRaises(policy.PolicyError):
            policy.validate_document(damaged)

    def test_missing_diagnostic_rejected(self):
        self.assert_invalid(lambda doc: doc["diagnostics"].pop())

    def test_changed_diagnostic_rejected(self):
        def mutate(doc):
            doc["diagnostics"][0]["message"] += " damaged"
        damaged = copy.deepcopy(self.document)
        mutate(damaged)
        with self.assertRaises(policy.PolicyError):
            policy.compare(damaged, self.document["diagnostics"], [319, 350])

    def test_extra_duplicate_rejected(self):
        def mutate(doc):
            doc["diagnostics"].append(copy.deepcopy(doc["diagnostics"][-1]))
            doc["coded_diagnostic_count"] += 1
        self.assert_invalid(mutate)

    def test_reordered_baseline_rejected(self):
        def mutate(doc):
            doc["diagnostics"][0], doc["diagnostics"][-1] = doc["diagnostics"][-1], doc["diagnostics"][0]
        self.assert_invalid(mutate)

    def test_new_runtime_diagnostic_rejected(self):
        actual = copy.deepcopy(self.document["diagnostics"])
        extra = copy.deepcopy(actual[-1])
        extra["message"] += " new"
        actual.append(extra)
        actual.sort(key=policy.canonical)
        with self.assertRaises(policy.PolicyError):
            policy.compare(self.document, actual, [319, 351])

    def test_wrong_toolchain_rejected(self):
        self.assert_invalid(lambda doc: doc["toolchain"].update({"rustc_release": "1.81.0"}))


if __name__ == "__main__":
    unittest.main()
