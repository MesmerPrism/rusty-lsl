# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param(
    [string] $ExternalRoot
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repoRoot
try {
    $arguments = @('tools/check_lslc_001h.py')
    if ($PSBoundParameters.ContainsKey('ExternalRoot')) {
        $arguments += @('--external-root', $ExternalRoot)
    }
    & python @arguments
    if ($LASTEXITCODE -ne 0) {
        throw "LSLC-001H validation failed with exit code $LASTEXITCODE."
    }
}
finally {
    Pop-Location
}
