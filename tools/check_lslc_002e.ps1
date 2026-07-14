$ErrorActionPreference = "Stop"
python "$PSScriptRoot\check_lslc_002e.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
