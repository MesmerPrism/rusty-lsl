$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
python (Join-Path $PSScriptRoot "check_lslc_002d.py") $root
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --manifest-path (Join-Path $root "Cargo.toml") --workspace --all-targets --offline --locked short_info_query_wire
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "LSLC-002D focused checks passed."
