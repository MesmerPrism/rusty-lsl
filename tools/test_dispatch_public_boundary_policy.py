#!/usr/bin/env python3
import hashlib
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIRECT = ROOT / "tools/check_public_boundaries.py"
assert hashlib.sha256(DIRECT.read_bytes()).hexdigest() == "b0adb8a1d8622b5c8414ffd7273c76511b701fd29cf3cf3145fa642e2e6ba301"
for relative, expected in {
    "morphospace/receipts/rlsl-lslc-003m-standard-validation.json": "d979a92da01cf3b7c46844335c8612cb4d0aeacf64201b9f9e9a40997ac15d5b",
    "morphospace/receipts/rlsl-lslc-004j-standard-validation.json": "edc017486efc0b37530ae7212bbdf82abe9ec453d0ab0999e56de29171cf6ac0",
}.items():
    tracked = subprocess.run(
        ["git", "show", f"HEAD:{relative}"],
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
    )
    data = tracked.stdout
    assert hashlib.sha256(data).hexdigest() == expected
    assert data and not data.endswith((b"\n", b"\r"))
result = subprocess.run(
    [sys.executable, str(ROOT / "tools/dispatch_validation.py"), "--self-test-public-boundary-policy"],
    cwd=ROOT,
    check=False,
)
raise SystemExit(result.returncode)
