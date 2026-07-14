$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
python (Join-Path $PSScriptRoot "check_lslc_002b.py") $root
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --manifest-path (Join-Path $root "Cargo.toml") --workspace --all-targets --offline --locked stream_info_observed_document_admission
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "LSLC-002B focused checks passed."
