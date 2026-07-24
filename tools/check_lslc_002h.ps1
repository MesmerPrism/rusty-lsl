$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo test --workspace --all-targets --offline --locked typed_short_info_response_observation
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002H Rust tests failed.' }
    $source = Get-Content -Raw crates/rusty-lsl/src/typed_short_info_response_observation.rs
    foreach ($forbidden in @('UdpSocket','TcpStream','std::net','unsafe {','Manifold')) {
        if ($source.Contains($forbidden)) { throw "LSLC-002H forbidden surface: $forbidden" }
    }
    Write-Host 'LSLC-002H focused checks passed.'
} finally { Pop-Location }
