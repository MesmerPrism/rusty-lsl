# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repoRoot
try {
    python tools/check_lslc_001n.py
    if ($LASTEXITCODE -ne 0) { throw "LSLC-001N checker failed with exit code $LASTEXITCODE." }
}
finally { Pop-Location }
