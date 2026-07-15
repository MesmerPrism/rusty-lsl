# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json
from pathlib import Path
root = Path(__file__).resolve().parents[1]
data = json.loads((root / "fixtures/compatibility/lslc-002w-finite-sample-recovery-runtime.json").read_text(encoding="utf-8"))
assert data["schema"] == "rusty.lsl.lslc_002w.finite_sample_recovery_runtime.v1"
assert data["recovery"]["classification"] == "caller-labelled"
assert data["composition"]["timestamp_bits"] == data["composition"]["value_bits"] == "exact"
assert all(value is False for value in data["claims"].values())
source = (root / "crates/rusty-lsl/src/finite_sample_recovery_runtime.rs").read_text(encoding="utf-8")
for token in ["max_attempts", "max_states", "RetryableFailure", "TerminalFailure", "AtomicBool", "total_deadline"]: assert token in source
for forbidden in ["tokio", "unsafe {", "TcpStream", "UdpSocket", "Manifold"]: assert forbidden not in source
print("LSLC-002W finite recovery boundary validation passed")
