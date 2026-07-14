$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_002m.ps1
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002M prerequisite gate failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_002o
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002O focused Rust tests failed.' }
    cargo test --workspace --all-targets --offline --locked authority_remains_outside_the_repository
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002O ownership regression failed.' }
} finally { Pop-Location }
