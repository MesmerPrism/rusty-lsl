[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$OutputDirectory
)

$ErrorActionPreference = 'Stop'
$env:PYTHONDONTWRITEBYTECODE = '1'
$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$outputRoot = [System.IO.Path]::GetFullPath($OutputDirectory)
[System.IO.Directory]::CreateDirectory($outputRoot) | Out-Null
$receiptPath = Join-Path $outputRoot 'p6-host-qualification-v1.json'

function Invoke-Git([string[]]$Arguments) {
    $value = & git -C $repositoryRoot @Arguments
    if ($LASTEXITCODE -ne 0) { throw "git failed: $($Arguments -join ' ')" }
    return ($value -join "`n").Trim()
}

$commit = Invoke-Git @('rev-parse', 'HEAD')
$tree = Invoke-Git @('rev-parse', 'HEAD^{tree}')
$statusBefore = Invoke-Git @('status', '--porcelain=v1', '--untracked-files=all')
if ($statusBefore.Length -ne 0) { throw 'Qualification requires a clean worktree.' }

& cargo test --manifest-path (Join-Path $repositoryRoot 'Cargo.toml') -p rusty-lsl --test public_api -- --exact p6_explicit_loopback_host_lifecycle_qualification --nocapture
if ($LASTEXITCODE -ne 0) { throw 'Focused P6 host qualification test failed.' }

$commitAfter = Invoke-Git @('rev-parse', 'HEAD')
$treeAfter = Invoke-Git @('rev-parse', 'HEAD^{tree}')
$statusAfter = Invoke-Git @('status', '--porcelain=v1', '--untracked-files=all')
if ($commitAfter -ne $commit -or $treeAfter -ne $tree -or $statusAfter.Length -ne 0) {
    throw 'Repository identity or cleanliness changed during qualification.'
}

$receipt = [ordered]@{
    schema_version = 1
    qualification = 'rlsl-p6-single-quest-host-lifecycle-qualification-milestone-p66'
    result = 'pass'
    scope = 'ipv4-loopback-local-host'
    git = [ordered]@{ commit = $commit; tree = $tree; clean = $true }
    test = [ordered]@{
        package = 'rusty-lsl'
        target = 'public_api'
        name = 'p6_explicit_loopback_host_lifecycle_qualification'
    }
    evidence = [ordered]@{
        discovery_selection = 'explicit-receive-order-index-0'
        float32_records = 2
        exact_bits = $true
        bounded_recovery = $true
        clock_correction = $true
        terminal_cleanup_and_tcp_reuse = $true
    }
    does_not_prove = @('official-runtime-or-oracle', 'device', 'cross-network', 'automatic-policy', 'Manifold-authority')
}
$json = $receipt | ConvertTo-Json -Depth 8
[System.IO.File]::WriteAllText($receiptPath, $json + "`n", [System.Text.UTF8Encoding]::new($false))
Write-Output $receiptPath
