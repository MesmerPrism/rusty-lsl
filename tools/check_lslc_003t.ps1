$ErrorActionPreference = "Stop"
python ./tools/check_lslc_003t.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --offline --locked lslc_003t
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python ./tools/dispatch_validation.py --internal-gate public-boundary
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Output "LSLC-003T focused runtime checks passed."
