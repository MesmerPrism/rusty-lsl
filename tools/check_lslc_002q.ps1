# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$root = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $root
try {
    python tools/check_lslc_002q.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    cargo test --workspace --all-targets --offline --locked udp_discovery
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    Write-Host 'LSLC-002Q focused observation checks passed.'
}
finally {
    Pop-Location
}
