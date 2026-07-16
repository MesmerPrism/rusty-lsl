$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    python tools/check_lslc_003o.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003O evidence failed.' }
    python tools/dispatch_validation.py --internal-gate public-boundary
    if ($LASTEXITCODE -ne 0) { throw 'Public boundary failed.' }
    Write-Host 'LSLC-003O focused observation checks passed.'
}
finally {
    Pop-Location
}
