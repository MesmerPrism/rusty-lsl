# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
fixture = root / "fixtures/compatibility/lslc-002v-bounded-sample-queue-backpressure-cancellation-runtime.json"
data = json.loads(fixture.read_text(encoding="utf-8"))
assert data["schema"] == "rusty.lsl.lslc_002v.bounded_sample_queue_runtime.v1"
assert data["queue"] == {"ownership":"caller-owned", "capacity":"explicit-nonzero", "ordering":"FIFO", "allocation":"fallible-before-exposure"}
assert data["backpressure"]["try_push"] == "full-retains-sample"
assert data["termination"]["owned_worker"] is False
assert data["composition"]["timestamp_bits"] == data["composition"]["value_bits"] == "exact"
assert all(value is False for value in data["claims"].values())
source = (root / "crates/rusty-lsl/src/bounded_sample_queue_runtime.rs").read_text(encoding="utf-8")
for token in ["try_reserve", "Condvar", "AtomicBool", "Full(TimestampedSample<f32>)", "notify_all"]:
    assert token in source
for forbidden in ["tokio", "unsafe {", "retry", "reconnect", "Manifold"]:
    assert forbidden not in source
print("LSLC-002V bounded queue boundary validation passed")
