# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param(
    [ValidateSet('Capture')]
    [string] $Mode = 'Capture',
    [string] $ExternalRoot = (Join-Path $env:LOCALAPPDATA 'Temp\rusty-lsl-lslc-001h-oracle'),
    [string] $PythonCommand = 'python'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$wheelFile = 'pylsl-1.18.2-py2.py3-none-win_amd64.whl'
$wheelUrl = 'https://files.pythonhosted.org/packages/8a/a8/8ccb88b3cd65140e57966472038baa0cebb56bde5e86ad51852faed99cad/pylsl-1.18.2-py2.py3-none-win_amd64.whl'
$wheelSha256 = '3ea2693417c7d79766cebf967250fde78aa1a3ad2b198e40246d36f549dbfde1'
$pylslVersion = '1.18.2'
$libraryVersion = 117
$nativeSha256 = '8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff'
$maxProcessOutputBytes = 65536

function Add-FailureRecord {
    param([string] $Stage, [string] $Classification, [string] $Detail)
    $history = Join-Path $ExternalRoot 'failure-history.jsonl'
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $history) | Out-Null
    $record = [ordered]@{
        schema = 'rusty.lsl.oracle.failure.v1'
        recorded_at_utc = [DateTimeOffset]::UtcNow.ToString('o')
        stage = $Stage
        classification = $Classification
        detail = $Detail
    }
    ($record | ConvertTo-Json -Compress) | Add-Content -LiteralPath $history -Encoding utf8
}

function Assert-Hash {
    param([string] $Path, [string] $Expected, [string] $Stage)
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "[$Stage] required artifact is missing"
    }
    $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
    if ($actual -ne $Expected) {
        throw "[$Stage] SHA-256 mismatch"
    }
}

$resolvedRepo = [IO.Path]::GetFullPath($repoRoot)
$resolvedExternal = [IO.Path]::GetFullPath($ExternalRoot)
if ($resolvedExternal.StartsWith($resolvedRepo, [StringComparison]::OrdinalIgnoreCase)) {
    throw '[external-root] oracle artifacts must remain outside the repository'
}

try {
    if (-not $IsWindows) {
        throw '[python-architecture] LSLC-001H requires Windows'
    }
    New-Item -ItemType Directory -Force -Path $resolvedExternal | Out-Null
    $processTemp = Join-Path $resolvedExternal 'process-temp'
    $pipCache = Join-Path $resolvedExternal 'pip-cache'
    New-Item -ItemType Directory -Force -Path $processTemp, $pipCache | Out-Null
    $env:TEMP = $processTemp
    $env:TMP = $processTemp
    $env:PIP_CACHE_DIR = $pipCache
    $wheelDir = Join-Path $resolvedExternal 'wheelhouse'
    $venv = Join-Path $resolvedExternal 'venv'
    $run = Join-Path $resolvedExternal 'capture'
    New-Item -ItemType Directory -Force -Path $wheelDir, $run | Out-Null

    $probe = & $PythonCommand -c "import json,platform,struct; print(json.dumps({'version':platform.python_version(),'machine':platform.machine(),'bits':struct.calcsize('P')*8}))"
    if ($LASTEXITCODE -ne 0) { throw '[python-version] Python probe failed' }
    $pythonIdentity = $probe | ConvertFrom-Json
    if ($pythonIdentity.machine -ne 'AMD64' -or $pythonIdentity.bits -ne 64) {
        throw '[python-architecture] Python must be Windows AMD64'
    }

    $wheel = Join-Path $wheelDir $wheelFile
    if (-not (Test-Path -LiteralPath $wheel -PathType Leaf)) {
        Invoke-WebRequest -UseBasicParsing -Uri $wheelUrl -OutFile $wheel
    }
    Assert-Hash -Path $wheel -Expected $wheelSha256 -Stage 'wheel-digest'

    $venvPython = Join-Path $venv 'Scripts\python.exe'
    if (-not (Test-Path -LiteralPath $venvPython -PathType Leaf)) {
        & $PythonCommand -m venv $venv
        if ($LASTEXITCODE -ne 0) { throw '[python-version] virtual environment creation failed' }
    }
    & $venvPython -m pip install --disable-pip-version-check --no-cache-dir --no-deps --no-index --force-reinstall $wheel
    if ($LASTEXITCODE -ne 0) { throw '[pylsl-version] isolated no-dependency installation failed' }

    $dlls = @(Get-ChildItem -LiteralPath $venv -Recurse -File -Filter 'lsl.dll')
    if ($dlls.Count -ne 1) { throw '[native-library-presence] expected exactly one lsl.dll' }
    Assert-Hash -Path $dlls[0].FullName -Expected $nativeSha256 -Stage 'native-library-digest'

    $stdout = Join-Path $run 'driver-stdout.bin'
    $stderr = Join-Path $run 'driver-stderr.bin'
    $arguments = @(
        (Join-Path $PSScriptRoot 'lslc_001h_capture.py'),
        '--case-manifest', (Join-Path $repoRoot 'fixtures\compatibility\lslc-001h-stream-info-xml-cases.json'),
        '--output-dir', $run,
        '--native-dll', $dlls[0].FullName,
        '--failure-history', (Join-Path $resolvedExternal 'failure-history.jsonl'),
        '--expected-python-version', $pythonIdentity.version,
        '--expected-pylsl-version', $pylslVersion,
        '--expected-library-version', $libraryVersion,
        '--expected-native-sha256', $nativeSha256
    )
    $process = Start-Process -FilePath $venvPython -ArgumentList $arguments -Wait -PassThru -WindowStyle Hidden -RedirectStandardOutput $stdout -RedirectStandardError $stderr
    if ((Get-Item -LiteralPath $stdout).Length -gt $maxProcessOutputBytes -or (Get-Item -LiteralPath $stderr).Length -gt $maxProcessOutputBytes) {
        throw '[capture-output-bound] oracle process output exceeded its bound'
    }
    if ($process.ExitCode -ne 0) {
        throw "[oracle-process-exit] oracle exited $($process.ExitCode)"
    }
    $record = Join-Path $run 'capture-record.json'
    if (-not (Test-Path -LiteralPath $record -PathType Leaf)) {
        throw '[evidence-shape] capture record is missing'
    }
    $parsed = Get-Content -Raw -LiteralPath $record | ConvertFrom-Json
    if ($parsed.schema -ne 'rusty.lsl.oracle.external_capture.v1' -or $parsed.classification -ne 'accepted') {
        throw '[evidence-shape] capture record identity drifted'
    }
    Write-Host "LSLC-001H oracle capture passed. External record: $record"
}
catch {
    $message = $_.Exception.Message
    $stage = if ($message -match '^\[([^]]+)\]') { $Matches[1] } else { 'oracle-process-exit' }
    $classification = if ($stage -match '^(python|wheel|pylsl|native-library)') { 'setup-failure' } else { 'provider-failure' }
    Add-FailureRecord -Stage $stage -Classification $classification -Detail ($message -replace [regex]::Escape($resolvedExternal), '<external-root>')
    throw
}
