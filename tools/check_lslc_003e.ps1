$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python tools/check_lslc_003e.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003E structural evidence failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_003e
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-003E helper tests failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_002t
    if ($LASTEXITCODE -ne 0) { throw 'Float32 facade parity failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_003b
    if ($LASTEXITCODE -ne 0) { throw 'Fixed-width facade parity failed.' }
    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { throw 'Public-boundary check failed.' }
}
finally { Pop-Location }
