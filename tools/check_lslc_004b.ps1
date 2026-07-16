$ErrorActionPreference = "Stop"
python "$PSScriptRoot/check_lslc_004b.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --offline --locked lslc_004b
exit $LASTEXITCODE
