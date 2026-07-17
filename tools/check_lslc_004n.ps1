$ErrorActionPreference = "Stop"
python "$PSScriptRoot/check_lslc_004n.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python "$PSScriptRoot/dispatch_validation.py" --internal-gate public-boundary
exit $LASTEXITCODE
