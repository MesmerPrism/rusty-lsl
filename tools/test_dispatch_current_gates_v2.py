#!/usr/bin/env python3
"""Damaged-case and role-order tests for the v2 gate dispatcher."""

import copy
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest

from dispatch_current_gates import ManifestError, dispatch, load_and_validate

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "tools/current-gates-v2.json"


class V2DispatcherTests(unittest.TestCase):
    def damaged(self, mutate):
        document = json.loads(MANIFEST.read_text(encoding="utf-8"))
        mutate(document)
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        path = Path(temporary.name) / "manifest.json"
        path.write_text(json.dumps(document), encoding="utf-8")
        return path

    def test_complete_roles_validate(self):
        roles = load_and_validate(MANIFEST, ROOT)
        self.assertEqual(len(roles["historical"]), 18)
        self.assertEqual([gate["checker_id"] for gate in roles["current"]], ["lslc-003j"])

    def test_damage_fails_closed_before_execution(self):
        cases = {
            "v1-hash": lambda d: d["v1_manifest"].update(sha256="0" * 64),
            "receipt-hash": lambda d: d["historical_gates"][0].update(receipt_sha256="0" * 64),
            "pin": lambda d: d["historical_gates"][0].update(pin="0" * 40),
            "order": lambda d: d["historical_gates"][0].update(order=2),
            "traversal": lambda d: d["historical_gates"][0].update(receipt_path="../receipt.json"),
            "role-overlap": lambda d: d["current_gates"][0].update(checker_id="lslc-003h", path="tools/check_lslc_003h.ps1"),
        }
        for name, mutate in cases.items():
            with self.subTest(name=name), self.assertRaises(ManifestError):
                load_and_validate(self.damaged(mutate), ROOT)

    def test_dispatch_orders_roles_and_cleans_temporary_directories(self):
        roles = load_and_validate(MANIFEST, ROOT)
        sample = {"historical": [copy.deepcopy(roles["historical"][0])], "current": [copy.deepcopy(roles["current"][0])]}
        seen = []
        def runner(command, cwd, **_):
            seen.append((command[-1], Path(cwd)))
            return SimpleNamespace(returncode=0)
        dispatch(sample, ROOT, runner=runner, materialize=False)
        self.assertEqual([item[0] for item in seen], ["tools/check_lslc_002q.ps1", "tools/check_lslc_003j.ps1"])
        self.assertFalse(seen[0][1].exists())
        self.assertEqual(seen[1][1], ROOT)

    def test_first_failure_stops_dispatch_and_cleans(self):
        roles = load_and_validate(MANIFEST, ROOT)
        sample = {"historical": [copy.deepcopy(roles["historical"][0])], "current": []}
        seen = []
        def runner(command, cwd, **_):
            seen.append(Path(cwd))
            return SimpleNamespace(returncode=9)
        with self.assertRaisesRegex(RuntimeError, "lslc-002q"):
            dispatch(sample, ROOT, runner=runner, materialize=False)
        self.assertFalse(seen[0].exists())


if __name__ == "__main__":
    unittest.main()
