# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repoRoot

function Invoke-Checked {
    param(
        [Parameter(Mandatory)]
        [string] $Command,

        [Parameter(ValueFromRemainingArguments)]
        [string[]] $Arguments
    )

    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed with exit code $LASTEXITCODE`: $Command $Arguments"
    }
}

try {
    Invoke-Checked cargo fmt --all --check
    $metadata = cargo metadata --offline --locked --no-deps --format-version 1 |
        ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) {
        throw 'Unable to inspect Cargo metadata.'
    }

    Invoke-Checked cargo test --workspace --all-targets --offline --locked
    Invoke-Checked python tools/check_public_boundaries.py
    Invoke-Checked git diff --check

    $packages = @($metadata.packages)
    if ($packages.Count -ne 1 -or $packages[0].name -ne 'rusty-lsl') {
        throw 'The scaffold must contain exactly one package named rusty-lsl.'
    }

    $expectedManifest = [IO.Path]::GetFullPath(
        (Join-Path $repoRoot 'crates\rusty-lsl\Cargo.toml')
    )
    $actualManifest = [IO.Path]::GetFullPath($packages[0].manifest_path)
    if ($actualManifest -ne $expectedManifest) {
        throw 'The only package must remain at crates/rusty-lsl/Cargo.toml.'
    }

    $workspaceMembers = @($metadata.workspace_members)
    if (
        $workspaceMembers.Count -ne 1 -or
        $workspaceMembers[0] -ne $packages[0].id
    ) {
        throw 'The workspace must contain exactly the rusty-lsl package.'
    }

    if ($null -eq $packages[0].publish -or @($packages[0].publish).Count -ne 0) {
        throw 'The scaffold package must remain publish = false.'
    }

    if (@($packages[0].features.PSObject.Properties).Count -ne 0) {
        throw 'The scaffold must expose no Cargo features.'
    }

    $dependencies = @($packages[0].dependencies)
    if ($dependencies.Count -ne 0) {
        throw 'The scaffold dependency closure must remain empty.'
    }

    $targets = @($packages[0].targets)
    if ($targets.Count -ne 1 -or @($targets[0].kind) -notcontains 'lib') {
        throw 'The scaffold must expose exactly one library target.'
    }

    Write-Host 'Rusty LSL scaffold checks passed.'
}
finally {
    Pop-Location
}
