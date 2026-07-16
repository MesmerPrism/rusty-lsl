$ErrorActionPreference = 'Stop'
python "$PSScriptRoot/check_lslc_004h.py"
if ($LASTEXITCODE -ne 0) { throw 'LSLC-004H checker failed.' }
python "$PSScriptRoot/dispatch_validation.py" --internal-gate public-boundary
if ($LASTEXITCODE -ne 0) { throw 'Public-boundary checker failed.' }
