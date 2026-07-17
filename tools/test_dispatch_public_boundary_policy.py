#!/usr/bin/env python3
import hashlib
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIRECT = ROOT / "tools/check_public_boundaries.py"
assert hashlib.sha256(DIRECT.read_bytes()).hexdigest() == "b0adb8a1d8622b5c8414ffd7273c76511b701fd29cf3cf3145fa642e2e6ba301"
for relative, expected in {
    "morphospace/receipts/rlsl-lslc-003m-standard-validation.json": "b3c6693d54d856b2868a4497da5e13e0bdb8b21ece9d066c8559da90b9c09dd7",
    "morphospace/receipts/rlsl-lslc-004j-standard-validation.json": "edc017486efc0b37530ae7212bbdf82abe9ec453d0ab0999e56de29171cf6ac0",
}.items():
    data = (ROOT / relative).read_bytes()
    assert hashlib.sha256(data).hexdigest() == expected
    assert data and not data.endswith((b"\n", b"\r"))
result = subprocess.run(
    [sys.executable, str(ROOT / "tools/dispatch_validation.py"), "--self-test-public-boundary-policy"],
    cwd=ROOT,
    check=False,
)
raise SystemExit(result.returncode)
