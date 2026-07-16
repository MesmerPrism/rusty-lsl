#!/usr/bin/env python3
"""Exact README archive and current-router gate."""
import hashlib, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = "9cd21a10b7b75eb645a3bbb05d6d3dcf36e6338c"
SOURCE_SHA = "b2961e1df0896e0d528754c7c17135d85fbc93a8fbeb63979145fce2376ddb87"
ROUTES = [b"](AGENTS.md)", b"docs/ARCHITECTURE.md", b"docs/COMPATIBILITY.md", b"docs/PROVENANCE.md", b"docs/VALIDATION.md", b"docs/CORPUS.md", b"docs/ORACLE.md", b"docs/LICENSING.md", b"docs/SUPPLY_CHAIN.md", b"morphospace/README.md", b"fixtures/compatibility/README.md", b"docs/history/LSLC-WORK-UNIT-HISTORY.md", b"docs/history/README-THROUGH-LSLC-003L.md"]
CAPABILITIES = [b"bounded sample queue", b"finite sample recovery", b"fixed-width numeric sample transport", b"integrated clock correction", b"short-info discovery responder", b"stream handshake", b"timestamped Float32 sample transport", b"UDP discovery"]
REQUIRED = [b"disabled by default", b"Selection is not activation", b"Manifold stream", b"not current runtime or\ninteroperability claims"]
FORBIDDEN = [b"clean-room implementation", b"fully compatible", b"production-ready", b"enabled by default"]

class RouterError(RuntimeError): pass
def sha(data): return hashlib.sha256(data).hexdigest()
def validate(source, archive, readme):
    if sha(source) != SOURCE_SHA: raise RouterError("accepted README source hash mismatch")
    if archive != source: raise RouterError("README archive differs from accepted bytes")
    if len(readme.splitlines()) > 100: raise RouterError("README exceeds concise router budget")
    for marker in ROUTES + CAPABILITIES + REQUIRED:
        if readme.count(marker) != 1: raise RouterError(f"required marker count mismatch: {marker!r}")
    for marker in FORBIDDEN:
        if marker in readme: raise RouterError(f"forbidden overclaim: {marker!r}")
    if b"S:\\" in readme or b"C:\\Users\\" in readme: raise RouterError("private path in README")
def main():
    source=subprocess.run(["git","show",f"{BASE}:README.md"],cwd=ROOT,capture_output=True,check=True).stdout
    validate(source,(ROOT/"docs/history/README-THROUGH-LSLC-003L.md").read_bytes(),(ROOT/"README.md").read_bytes())
    print("LSLC-003M exact README archive and current router passed.")
if __name__ == "__main__":
    try: main()
    except (RouterError,OSError,subprocess.SubprocessError) as e: print(f"LSLC-003M failed: {e}",file=sys.stderr); sys.exit(1)
