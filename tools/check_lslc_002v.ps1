# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    python tools/check_lslc_002v.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002V boundary check failed.' }
    cargo test --workspace --all-targets --offline --locked lslc_002v
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002V runtime tests failed.' }
    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { throw 'Public-boundary check failed.' }
    Write-Host 'LSLC-002V focused checks passed.'
} finally { Pop-Location }
