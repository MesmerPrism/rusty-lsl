param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    python .\tools\check_lslc_003i.py
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    cargo test -p rusty-lsl timestamped_float32_sample_runtime
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally { Pop-Location }
