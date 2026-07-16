$ErrorActionPreference = "Stop"
python "$PSScriptRoot\test_lslc_003m.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python "$PSScriptRoot\check_lslc_003m.py"
exit $LASTEXITCODE
