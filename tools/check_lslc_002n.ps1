$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_002m.ps1
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002M prerequisite gate failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_002n
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002N focused Rust tests failed.' }
    cargo test --workspace --all-targets --offline --locked authority_remains_outside_the_repository
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002N ownership regression failed.' }
} finally { Pop-Location }
