param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python .\tools\check_lslc_003h.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    python .\tools\test_dispatch_current_gates.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    python .\tools\check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally { Pop-Location }
