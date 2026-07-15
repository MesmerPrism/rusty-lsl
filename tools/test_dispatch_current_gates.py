#!/usr/bin/env python3
"""Focused negative and ordering tests for the current-gates dispatcher."""

import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest

from dispatch_current_gates import ManifestError, dispatch, load_and_validate


class DispatcherTests(unittest.TestCase):
    def fixture(self, ids=("lslc-003g", "lslc-003h")):
        temporary = tempfile.TemporaryDirectory()
        root = Path(temporary.name)
        (root / "tools").mkdir()
        gates = []
        for order, checker_id in enumerate(ids, start=1):
            path = f"tools/check_{checker_id.replace('-', '_')}.ps1"
            (root / path).write_text("exit 0\n", encoding="utf-8")
            gates.append({"order": order, "checker_id": checker_id, "path": path})
        manifest = root / "tools/current-gates.json"
        manifest.write_text(json.dumps({"schema":"rusty.lsl.current_gates.v1","revision":1,"expected_checker_ids":list(ids),"gates":gates}) + "\n", encoding="utf-8")
        return temporary, root, manifest

    def test_order_is_exact_and_success_runs_every_gate(self):
        temporary, root, manifest = self.fixture()
        self.addCleanup(temporary.cleanup)
        seen = []
        dispatch(load_and_validate(manifest, root), root, lambda command, **_: seen.append(command[-1]) or SimpleNamespace(returncode=0))
        self.assertEqual(seen, ["tools/check_lslc_003g.ps1", "tools/check_lslc_003h.ps1"])

    def test_missing_duplicate_and_traversing_entries_reject_before_execution(self):
        for damage in ("missing", "duplicate", "traversal"):
            with self.subTest(damage=damage):
                temporary, root, manifest = self.fixture()
                self.addCleanup(temporary.cleanup)
                document = json.loads(manifest.read_text(encoding="utf-8"))
                if damage == "missing":
                    (root / document["gates"][1]["path"]).unlink()
                elif damage == "duplicate":
                    document["expected_checker_ids"][1] = document["expected_checker_ids"][0]
                    manifest.write_text(json.dumps(document), encoding="utf-8")
                else:
                    document["gates"][1]["path"] = "tools/../check_lslc_003h.ps1"
                    manifest.write_text(json.dumps(document), encoding="utf-8")
                with self.assertRaises(ManifestError):
                    load_and_validate(manifest, root)

    def test_first_failure_stops_later_gate(self):
        temporary, root, manifest = self.fixture()
        self.addCleanup(temporary.cleanup)
        seen = []
        def runner(command, **_):
            seen.append(command[-1])
            return SimpleNamespace(returncode=7)
        with self.assertRaisesRegex(RuntimeError, "lslc-003g"):
            dispatch(load_and_validate(manifest, root), root, runner)
        self.assertEqual(seen, ["tools/check_lslc_003g.ps1"])


if __name__ == "__main__":
    unittest.main()
