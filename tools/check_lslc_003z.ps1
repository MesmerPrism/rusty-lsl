$ErrorActionPreference = "Stop"
python "$PSScriptRoot/check_lslc_003z.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --offline --locked lslc_003z
exit $LASTEXITCODE
