[CmdletBinding()]
param([switch]$SelfTestOnly)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ($PSVersionTable.PSEdition -ne "Core" -or $PSVersionTable.PSVersion -lt [version]"7.6.0") {
    throw "Release review requires PowerShell 7.6 or newer."
}

$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
Set-Location -LiteralPath $repoRoot

function Invoke-Checked([string]$Label, [scriptblock]$Command) {
    Write-Output "RELEASE-REVIEW $Label"
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "Release review failed: $Label" }
}

$policy = Get-Content -Raw "tools/validation-policy.json" | ConvertFrom-Json -AsHashtable
foreach ($profile in @("standard", "deep", "ci")) {
    if ($policy.profiles[$profile] -notcontains "release-review-contract") {
        throw "Release review contract is absent from $profile."
    }
}
foreach ($profile in @("deep", "ci")) {
    if ($policy.profiles[$profile] -notcontains "pinned-rust-180-clippy") {
        throw "Declared Rust 1.80 gate is absent from $profile."
    }
}
Write-Output "PASS release-review contract self-test"
if ($SelfTestOnly) { exit 0 }

if (@(& git status --porcelain=v1 --untracked-files=all).Count -ne 0) {
    throw "Release review requires a clean worktree."
}

$head = (& git rev-parse HEAD).Trim()
$tree = (& git rev-parse "HEAD^{tree}").Trim()
$toolchain = (& rustup run 1.80.0 rustc --version).Trim()

Invoke-Checked "Rust 1.80 public API" { rustup run 1.80.0 cargo test -p rusty-lsl --test public_api }
$priorToolchain = $env:RUSTUP_TOOLCHAIN
try {
    $env:RUSTUP_TOOLCHAIN = "1.80.0"
    Invoke-Checked "CI profile under Rust 1.80" { python ./tools/dispatch_validation.py --profile ci }
} finally {
    $env:RUSTUP_TOOLCHAIN = $priorToolchain
}
Invoke-Checked "Deep profile" { python ./tools/dispatch_validation.py --profile deep }
Invoke-Checked "Static readiness" { pwsh -NoProfile -File ./tools/Test-ReleaseCandidateReadiness.ps1 }

Write-Output "PASS release review"
Write-Output "commit=$head"
Write-Output "tree=$tree"
Write-Output "toolchain=$toolchain"
Write-Output "profiles=ci@rust-1.80.0,deep"
Write-Output "boundary=no public-main, version, tag, release, registry, default activation, or device action"
