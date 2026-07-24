$ErrorActionPreference='Stop'; $root=Split-Path -Parent $PSScriptRoot; Push-Location $root
try { python tools/check_lslc_003a.py; if($LASTEXITCODE-ne 0){throw 'LSLC-003A evidence failed.'}; python tools/check_public_boundaries.py; if($LASTEXITCODE-ne 0){throw 'Public boundary failed.'}; Write-Host 'LSLC-003A focused observation checks passed.' } finally { Pop-Location }
