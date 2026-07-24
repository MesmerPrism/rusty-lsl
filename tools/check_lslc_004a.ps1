$ErrorActionPreference = "Stop"
python ./tools/check_lslc_004a.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python ./tools/dispatch_validation.py --internal-gate public-boundary
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "LSLC-004A focused observation checks passed."
