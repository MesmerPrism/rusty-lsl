$ErrorActionPreference = "Stop"
python ./tools/check_lslc_003v.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --offline --locked lslc_003v
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python ./tools/dispatch_validation.py --internal-gate public-boundary
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "LSLC-003V focused runtime conformance checks passed."
