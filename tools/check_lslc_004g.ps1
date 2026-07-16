$ErrorActionPreference = "Stop"
python (Join-Path $PSScriptRoot "check_lslc_004g.py")
if ($LASTEXITCODE -ne 0) { throw "LSLC-004G checker failed." }
