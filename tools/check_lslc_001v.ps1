# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
[CmdletBinding()] param()
$ErrorActionPreference='Stop'; Set-StrictMode -Version Latest
$root=Split-Path -Parent $PSScriptRoot; Push-Location $root
try { python tools/check_lslc_001v.py; if($LASTEXITCODE -ne 0){throw "LSLC-001V checker failed."} } finally { Pop-Location }
