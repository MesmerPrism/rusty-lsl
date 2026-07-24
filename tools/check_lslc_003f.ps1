$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python tools/check_lslc_003f.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003F binding evidence failed.' }
    powershell -NoProfile -ExecutionPolicy Bypass -File tools/check_lslc_003c.ps1
    if ($LASTEXITCODE -ne 0) { throw 'Exact activation binding failed.' }
    powershell -NoProfile -ExecutionPolicy Bypass -File tools/check_lslc_003e.ps1
    if ($LASTEXITCODE -ne 0) { throw 'Transport facade parity failed.' }
}
finally { Pop-Location }
