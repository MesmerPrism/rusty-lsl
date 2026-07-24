$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python tools/check_lslc_003d.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003D evidence failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_003d
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003D tests failed.' }
    cargo test --workspace --all-targets --offline --locked
    if ($LASTEXITCODE -ne 0) { throw 'Runtime parity tests failed.' }
    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { throw 'Public-boundary check failed.' }
}
finally { Pop-Location }
