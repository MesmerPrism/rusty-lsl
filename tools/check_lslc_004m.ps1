$ErrorActionPreference = "Stop"
python "$PSScriptRoot/check_lslc_004m.py"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo test --workspace --all-targets --offline --locked lslc_004m_observed_official_query_structure_reaches_unchanged_responder
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python "$PSScriptRoot/dispatch_validation.py" --internal-gate public-boundary
exit $LASTEXITCODE
