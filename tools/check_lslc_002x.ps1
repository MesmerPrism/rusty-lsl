# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    python tools/check_lslc_002x.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002X observation check failed.' }
    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { throw 'Public-boundary check failed.' }
    Write-Host 'LSLC-002X focused observation checks passed.'
} finally { Pop-Location }
