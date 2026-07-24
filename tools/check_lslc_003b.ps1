$ErrorActionPreference='Stop';$r=Split-Path -Parent $PSScriptRoot;Push-Location $r
try{python tools/check_lslc_003b.py;if($LASTEXITCODE-ne 0){throw 'evidence failed'};cargo test --workspace --all-targets --offline --locked lslc_003b;if($LASTEXITCODE-ne 0){throw 'tests failed'};python tools/check_public_boundaries.py;if($LASTEXITCODE-ne 0){throw 'boundary failed'}}finally{Pop-Location}
