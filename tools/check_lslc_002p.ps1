# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo test --workspace --all-targets --offline --locked lslc_002p
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002P loopback tests failed.' }

    $source = Get-Content -LiteralPath './crates/rusty-lsl/src/udp_discovery.rs' -Raw
    foreach ($required in @('UdpSocket', 'AtomicBool', 'try_reserve_exact', 'checked_add', 'set_read_timeout', 'ParsedShortInfoResponseEnvelope')) {
        if (-not $source.Contains($required)) { throw "LSLC-002P source lacks required bounded surface '$required'." }
    }
    foreach ($forbidden in @('join_multicast', 'set_broadcast(true)', 'TcpStream', 'unsafe {', 'tokio', 'Manifold')) {
        if ($source.Contains($forbidden)) { throw "LSLC-002P source contains forbidden expansion '$forbidden'." }
    }

    $lock = Get-Content -LiteralPath './morphospace/feature.lock.json' -Raw | ConvertFrom-Json
    if (@($lock.selected_features).Count -ne 1 -or [string]$lock.selected_features[0] -cne 'udp-discovery') {
        throw 'LSLC-002P feature lock must select only udp-discovery.'
    }
    if (@($lock.effect_union.permissions).Count -ne 1 -or [string]$lock.effect_union.permissions[0] -cne 'network access') {
        throw 'LSLC-002P feature lock must open only network access permission.'
    }
    foreach ($name in @('services','activities','queries','tools','assets','shaders','native_libraries','commands','routes','streams','scenes','markers')) {
        if (@($lock.effect_union.$name).Count -ne 0) { throw "LSLC-002P lock unexpectedly opens $name." }
    }
    if (@($lock.effect_union.inputs) -notcontains 'explicit caller UDP discovery configuration') {
        throw 'LSLC-002P lock lacks the explicit runtime input.'
    }

    python tools/check_public_boundaries.py
    if ($LASTEXITCODE -ne 0) { throw 'LSLC-002P public boundary failed.' }
} finally {
    Pop-Location
}
