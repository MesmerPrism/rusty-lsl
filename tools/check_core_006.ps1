# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repoRoot
try {
    & python tools/check_core_006.py
    if ($LASTEXITCODE -ne 0) {
        throw "CORE-006 validation failed with exit code $LASTEXITCODE."
    }

    & cargo +1.80.0 test --workspace --all-targets --offline --locked core_006_
    if ($LASTEXITCODE -ne 0) {
        throw "CORE-006 contract tests failed with exit code $LASTEXITCODE."
    }
}
finally {
    Pop-Location
}
