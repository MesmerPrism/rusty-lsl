param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python .\tools\check_lslc_003g.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    cargo test --workspace --all-targets --offline --locked --test public_api
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    python .\tools\check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally {
    Pop-Location
}
