$ErrorActionPreference = 'Stop'
python "$PSScriptRoot/check_lslc_004i.py"
if ($LASTEXITCODE -ne 0) { throw 'LSLC-004I checker failed.' }
powershell -NoProfile -ExecutionPolicy Bypass -File "$PSScriptRoot/check_lslc_003s.ps1"
if ($LASTEXITCODE -ne 0) { throw 'LSLC-003S gate failed.' }
python "$PSScriptRoot/check_lslc_003j.py"
if ($LASTEXITCODE -ne 0) { throw 'LSLC-003J gate failed.' }
cargo test --workspace --all-targets --offline --locked runtime_activation
if ($LASTEXITCODE -ne 0) { throw 'Runtime activation tests failed.' }
