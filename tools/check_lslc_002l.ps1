$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try { python ./tools/check_lslc_002l.py; if ($LASTEXITCODE -ne 0) { throw 'LSLC-002L evidence checker failed.' } } finally { Pop-Location }
