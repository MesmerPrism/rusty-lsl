$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_002d.ps1
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002D prerequisite gate failed.' }
    powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_002j.ps1
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002J prerequisite gate failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_002k
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002K focused Rust tests failed.' }
    cargo test --workspace --all-targets --offline --locked authority_remains_outside_the_repository
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002K ownership regression failed.' }
} finally { Pop-Location }
