# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repoRoot
try {
    python tools/check_lslc_003j.py
    if ($LASTEXITCODE -ne 0) { throw "LSLC-003J checker failed with exit code $LASTEXITCODE" }
}
finally { Pop-Location }
