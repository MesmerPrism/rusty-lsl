default: check

fmt:
    cargo fmt --all --check

metadata:
    cargo metadata --offline --locked --no-deps --format-version 1

test:
    cargo test --workspace --all-targets --offline --locked

boundaries:
    python tools/check_public_boundaries.py

check:
    powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1
