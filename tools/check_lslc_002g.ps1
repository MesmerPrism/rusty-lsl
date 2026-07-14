$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_002f.ps1
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002F rolling gate failed.' }
    $response = Get-Content -Raw crates/rusty-lsl/src/short_info_response_envelope.rs
    $facade = Get-Content -Raw crates/rusty-lsl/src/lib.rs
    if (-not $response.Contains('lslc_002g_extra_crlf_rejects_at_body_start_with_unchanged_document_error')) { throw 'Explicit extra-CRLF regression is absent.' }
    if (-not $facade.Contains('bounded local short-info response-envelope contract')) { throw 'Public ownership declaration omits the response envelope.' }
    Write-Host 'LSLC-002G correction checks passed.'
} finally { Pop-Location }
