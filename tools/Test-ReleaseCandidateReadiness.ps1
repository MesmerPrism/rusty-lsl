[CmdletBinding()]
param(
    [switch]$SelfTestOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Fail([string]$Message) {
    throw "RLSL-P8 release-candidate readiness failed: $Message"
}

function Require([bool]$Condition, [string]$Message) {
    if (-not $Condition) {
        Fail $Message
    }
}

function Require-Match([string]$Text, [string]$Pattern, [string]$Message) {
    Require ([regex]::IsMatch($Text, $Pattern, [Text.RegularExpressions.RegexOptions]::Multiline)) $Message
}

function Test-WorkspaceManifest([string]$Text) {
    Require-Match $Text '^\[workspace\]\s*$' "workspace manifest lacks [workspace]"
    Require-Match $Text '^\[workspace\.package\]\s*$' "workspace manifest lacks [workspace.package]"
    Require-Match $Text '^\s*version\s*=\s*"0\.0\.0"\s*$' "workspace candidate version must remain 0.0.0"
    Require-Match $Text '^\s*repository\s*=\s*"https://github\.com/MesmerPrism/rusty-lsl"\s*$' "workspace repository identity drifted"
    Require (-not [regex]::IsMatch($Text, '^\s*default\s*=', 'Multiline')) "workspace manifest must not define a default feature"
}

function Test-CrateManifest([string]$Text) {
    Require-Match $Text '^\s*name\s*=\s*"rusty-lsl"\s*$' "crate package identity drifted"
    Require-Match $Text '^\s*version\.workspace\s*=\s*true\s*$' "crate version must remain workspace-owned"
    Require-Match $Text '^\s*publish\s*=\s*false\s*$' "crate must remain unpublished"
    Require (-not [regex]::IsMatch($Text, '^\[features\]\s*$', 'Multiline')) "release candidate must remain feature-free"
    Require (-not [regex]::IsMatch($Text, '^\s*default\s*=', 'Multiline')) "crate must not define a default feature"
}

function Test-Policy([hashtable]$Policy) {
    Require ($Policy.schema -eq "rusty.lsl.validation_policy.v1") "validation policy schema drifted"
    Require ($Policy.authority -eq "tools/validation-policy.json") "validation policy authority drifted"
    Require ($Policy.revision -is [long] -or $Policy.revision -is [int]) "validation policy revision is not an integer"
    Require ($Policy.revision -gt 0) "validation policy revision must be positive"
    foreach ($profile in @("quick", "standard", "deep", "ci")) {
        Require ($Policy.profiles.ContainsKey($profile)) "validation policy lacks $profile profile"
        Require ($Policy.profiles[$profile].Count -gt 0) "validation profile $profile is empty"
    }
    Require ($Policy.gates.Count -gt 0) "validation policy has no gates"
    $ids = @($Policy.gates | ForEach-Object { $_.id })
    Require (($ids | Select-Object -Unique).Count -eq $ids.Count) "validation gate IDs are not unique"
    foreach ($gate in $Policy.gates) {
        Require ($gate.state -eq "current") "validation gate $($gate.id) is not current"
        Require ($gate.command.Count -gt 0) "validation gate $($gate.id) has no command"
    }
    foreach ($profile in $Policy.profiles.Keys) {
        foreach ($gateId in $Policy.profiles[$profile]) {
            Require ($ids -contains $gateId) "profile $profile references unknown gate $gateId"
        }
    }
}

function Test-Document([string]$Name, [string]$Text, [string[]]$RequiredPatterns) {
    Require ($Text.Trim().Length -gt 0) "$Name is empty"
    foreach ($pattern in $RequiredPatterns) {
        Require-Match $Text $pattern "$Name lacks required current release evidence: $pattern"
    }
}

function Invoke-SelfTests {
    $workspace = @'
[workspace]
members = ["crates/rusty-lsl"]
[workspace.package]
version = "0.0.0"
repository = "https://github.com/MesmerPrism/rusty-lsl"
'@
    $crate = @'
[package]
name = "rusty-lsl"
version.workspace = true
publish = false
'@
    Test-WorkspaceManifest $workspace
    Test-CrateManifest $crate

    $damagedCases = @(
        @{ Name = "version"; Action = { Test-WorkspaceManifest ($workspace -replace '0\.0\.0', '1.0.0') } },
        @{ Name = "publication"; Action = { Test-CrateManifest ($crate -replace 'publish = false', 'publish = true') } },
        @{ Name = "default activation"; Action = { Test-CrateManifest ($crate + "`n[features]`ndefault = []") } },
        @{ Name = "missing evidence"; Action = { Test-Document "damaged" "# Release" @('default-disabled') } }
    )
    foreach ($case in $damagedCases) {
        $rejected = $false
        try {
            & $case.Action
        } catch {
            $rejected = $true
        }
        Require $rejected "self-test did not reject damaged $($case.Name)"
    }
    Write-Output "PASS self-test: positive manifest checks and 4 damaged cases"
}

Invoke-SelfTests
if ($SelfTestOnly) {
    exit 0
}

Require ($PSVersionTable.PSEdition -eq "Core") "PowerShell Core is required"
Require ($PSVersionTable.PSVersion -ge [version]"7.6.0") "PowerShell 7.6 or newer is required"

$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
Set-Location -LiteralPath $repoRoot

function Invoke-Git([string[]]$Arguments) {
    $output = & git @Arguments 2>&1
    if ($LASTEXITCODE -ne 0) {
        Fail "git $($Arguments -join ' ') failed: $($output -join ' ')"
    }
    return @($output)
}

$topLevel = (@(Invoke-Git @("rev-parse", "--show-toplevel")))[0].Trim()
Require ([IO.Path]::GetFullPath($topLevel) -eq $repoRoot) "script is not running from its repository root"
Require (@(Invoke-Git @("status", "--porcelain=v1", "--untracked-files=all")).Count -eq 0) "worktree is not clean"

$branch = (@(Invoke-Git @("symbolic-ref", "--quiet", "--short", "HEAD")))[0].Trim()
Require ($branch.StartsWith("codex/")) "candidate must remain on an isolated codex/* feature branch"
Require ($branch -notin @("main", "master", "public-main")) "candidate must not be public main"
$parents = @((@(Invoke-Git @("show", "-s", "--format=%P", "HEAD")))[0].Trim() -split '\s+' | Where-Object { $_ })
Require ($parents.Count -eq 1) "candidate HEAD must have exactly one parent"

$requiredPaths = @(
    "Cargo.toml",
    "Cargo.lock",
    "crates/rusty-lsl/Cargo.toml",
    "README.md",
    "docs/ARCHITECTURE.md",
    "docs/COMPATIBILITY.md",
    "docs/LSL-PRODUCTION-ROADMAP.md",
    "docs/P6_HOST_QUALIFICATION.md",
    "docs/RELEASE_CANDIDATE.md",
    "docs/SUPPLY_CHAIN.md",
    "docs/VALIDATION.md",
    "tools/Test-ReleaseCandidateReadiness.ps1",
    "tools/dispatch_validation.py",
    "tools/validation-policy.json"
)
foreach ($path in $requiredPaths) {
    Invoke-Git @("ls-files", "--error-unmatch", "--", $path) | Out-Null
    Require (Test-Path -LiteralPath $path -PathType Leaf) "required tracked path is absent: $path"
}

Test-WorkspaceManifest (Get-Content -LiteralPath "Cargo.toml" -Raw)
Test-CrateManifest (Get-Content -LiteralPath "crates/rusty-lsl/Cargo.toml" -Raw)

$policy = Get-Content -LiteralPath "tools/validation-policy.json" -Raw | ConvertFrom-Json -AsHashtable
Test-Policy $policy
$allowedExecutables = @("cargo", "git", "powershell", "pwsh", "python", "python3")
foreach ($gate in $policy.gates) {
    Require ($allowedExecutables -contains $gate.command[0]) "gate $($gate.id) uses an unreviewed command executable"
    foreach ($argument in $gate.command) {
        if ($argument -match '^(tools|validation|fixtures|docs|crates|morphospace)/.+\.[A-Za-z0-9]+$') {
            $normalized = $argument -replace '/', [IO.Path]::DirectorySeparatorChar
            Require (Test-Path -LiteralPath $normalized -PathType Leaf) "gate $($gate.id) command path is absent: $argument"
            Invoke-Git @("ls-files", "--error-unmatch", "--", $argument) | Out-Null
        }
    }
}

Test-Document "README.md" (Get-Content "README.md" -Raw) @(
    'default-disabled',
    'Production Roadmap'
)
Test-Document "docs/ARCHITECTURE.md" (Get-Content "docs/ARCHITECTURE.md" -Raw) @(
    'single coherent public outlet/inlet session',
    'explicit activation'
)
Test-Document "docs/COMPATIBILITY.md" (Get-Content "docs/COMPATIBILITY.md" -Raw) @(
    '^# Compatibility\s*$',
    '[Nn]ot implemented and not claimed'
)
Test-Document "docs/LSL-PRODUCTION-ROADMAP.md" (Get-Content "docs/LSL-PRODUCTION-ROADMAP.md" -Raw) @(
    'P8: stable promotion, public-main integration review, and versioned release readiness',
    'Default activation stays disabled'
)
Test-Document "docs/P6_HOST_QUALIFICATION.md" (Get-Content "docs/P6_HOST_QUALIFICATION.md" -Raw) @(
    'P6 local host qualification',
    'fails closed unless the worktree is clean'
)
Test-Document "docs/VALIDATION.md" (Get-Content "docs/VALIDATION.md" -Raw) @(
    'sole current validation-policy authority',
    'dispatch_validation\.py --profile quick'
)
Test-Document "docs/RELEASE_CANDIDATE.md" (Get-Content "docs/RELEASE_CANDIDATE.md" -Raw) @(
    'Test-ReleaseCandidateReadiness\.ps1',
    'does not integrate public main',
    'does not version, tag, release, or publish',
    'specifically authorized'
)

$head = (@(Invoke-Git @("rev-parse", "HEAD")))[0].Trim()
$tree = (@(Invoke-Git @("rev-parse", "HEAD^{tree}")))[0].Trim()
Write-Output "PASS RLSL-P8 release-candidate readiness"
Write-Output "branch=$branch"
Write-Output "commit=$head"
Write-Output "tree=$tree"
Write-Output "policy=$($policy.schema);revision=$($policy.revision);authority=$($policy.authority)"
Write-Output "boundary=feature-branch readiness only; no integration, versioning, tagging, release, or publication"
