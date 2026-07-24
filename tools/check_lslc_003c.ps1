$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python tools/check_lslc_003c.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003C evidence failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_003c
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003C tests failed.' }
    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { throw 'Public-boundary check failed.' }
}
finally {
    Pop-Location
}
