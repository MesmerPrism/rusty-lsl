$ErrorActionPreference = "Stop"
python ./tools/check_lslc_003s.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python ./tools/check_lslc_003c.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --offline --locked lslc_003s
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python ./tools/dispatch_validation.py --internal-gate public-boundary
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "LSLC-003S focused activation checks passed."
