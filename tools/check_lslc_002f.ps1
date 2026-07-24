$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo test --workspace --all-targets --offline --locked
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002F Rust tests failed.' }
    $source = Get-Content -Raw crates/rusty-lsl/src/short_info_response_envelope.rs
    foreach ($forbidden in @('UdpSocket','TcpStream','unsafe {','std::net','Manifold')) {
        if ($source.Contains($forbidden)) { throw "LSLC-002F forbidden surface: $forbidden" }
    }
    Write-Host 'LSLC-002F focused checks passed.'
} finally { Pop-Location }
