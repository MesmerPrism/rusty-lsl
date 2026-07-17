$ErrorActionPreference = "Stop"
python "$PSScriptRoot/check_lslc_004t.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python "$PSScriptRoot/dispatch_validation.py" --internal-gate public-boundary
exit $LASTEXITCODE
