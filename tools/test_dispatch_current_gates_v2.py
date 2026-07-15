#!/usr/bin/env python3
"""Damaged-case and role-order tests for the v2 gate dispatcher."""

import copy
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
import subprocess

from dispatch_current_gates import ManifestError, _remove_owned_worktree, dispatch, load_and_validate

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
            "truncated-v1-prefix": lambda d: d["historical_gates"].pop(0),
            "reordered-v1-prefix": lambda d: d["historical_gates"].__setitem__(slice(0, 2), [d["historical_gates"][1], d["historical_gates"][0]]),
            "duplicate-appended-id": lambda d: d["historical_gates"].append({**d["historical_gates"][-1], "order": 19}),
        }
        for name, mutate in cases.items():
            with self.subTest(name=name), self.assertRaises(ManifestError):
                load_and_validate(self.damaged(mutate), ROOT)

    @staticmethod
    def append_lslc_003i(document):
        document["historical_gates"].append({
            "order": 19,
            "checker_id": "lslc-003i",
            "role": "historical",
            "pin": "468d9f04dc5a82d6692095723f450b24c27bb070",
            "receipt_path": "morphospace/receipts/rlsl-lslc-003i-blocked-validation.json",
            "receipt_sha256": "f87e04588e5953662d5ea22c2fae0adbe893607dab01d4f0a55d78866352cac4",
            "launcher_path": "tools/check_lslc_003i.ps1",
            "launcher_sha256": "2c9eddbc1c4990680ad53005266e3f9db3e9c43fd3e98f0bc4aea64652446292",
            "companion_path": "tools/check_lslc_003i.py",
            "companion_sha256": "10be9ea585983a05038755c66d0472a66338cf93898de55cdd9f56c62bd383db",
        })

    def test_valid_append_then_replace_current_scenario(self):
        path = self.damaged(self.append_lslc_003i)
        roles = load_and_validate(path, ROOT)
        self.assertEqual(roles["historical"][-1]["checker_id"], "lslc-003i")
        self.assertEqual(roles["current"][0]["checker_id"], "lslc-003j")

    def test_invalid_promoted_pin_and_receipt_fail_closed(self):
        for damage in ("pin", "receipt"):
            def mutate(document, damage=damage):
                self.append_lslc_003i(document)
                if damage == "pin":
                    document["historical_gates"][-1]["pin"] = "0" * 40
                else:
                    document["historical_gates"][-1]["receipt_sha256"] = "0" * 64
            with self.subTest(damage=damage), self.assertRaises(ManifestError):
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

    def test_real_materialization_removes_directory_and_registry_entry(self):
        roles = load_and_validate(MANIFEST, ROOT)
        sample = {"historical": [copy.deepcopy(roles["historical"][0])], "current": []}
        dispatch(sample, ROOT, materialize=True)

    def test_removal_failure_is_not_hidden_by_directory_cleanup(self):
        temporary = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: temporary.exists() and temporary.rmdir())
        def failed_remove(*_, **__):
            return SimpleNamespace(returncode=7, stdout="", stderr="synthetic removal failure")
        with self.assertRaisesRegex(RuntimeError, "removal failed"):
            _remove_owned_worktree(ROOT, temporary, failed_remove)
        self.assertTrue(temporary.exists())

    def test_registry_leak_fails_after_successful_remove_result(self):
        temporary = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: temporary.exists() and temporary.rmdir())
        calls = []
        def leaked_registry(command, **_):
            calls.append(command)
            if command[2:4] == ["remove", "--force"]:
                return SimpleNamespace(returncode=0, stdout="", stderr="")
            return SimpleNamespace(returncode=0, stdout=f"worktree {temporary.resolve()}\nHEAD 0\ndetached\n", stderr="")
        with self.assertRaisesRegex(RuntimeError, "registry entry remains"):
            _remove_owned_worktree(ROOT, temporary, leaked_registry)
        self.assertTrue(temporary.exists())

    def test_live_checker_survives_post_acceptance_current_unit_null(self):
        workspace = json.loads((ROOT / "morphospace/workspace.state.json").read_text(encoding="utf-8"))
        workspace["current_unit"] = None
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        path = Path(temporary.name) / "workspace.state.json"
        path.write_text(json.dumps(workspace), encoding="utf-8")
        result = subprocess.run(["python", "tools/check_lslc_003j.py", str(path)], cwd=ROOT, check=False, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()
