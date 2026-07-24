$ErrorActionPreference = "Stop"
python "$PSScriptRoot/test_dispatch_public_boundary_policy.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python "$PSScriptRoot/dispatch_validation.py" --internal-gate public-boundary
exit $LASTEXITCODE
