import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
fixture = json.loads((root / "fixtures/compatibility/lslc-003e-bounded-fixed-record-transport-core.json").read_text())
helper = (root / "crates/rusty-lsl/src/bounded_fixed_record_transport.rs").read_text()
lib = (root / "crates/rusty-lsl/src/lib.rs").read_text()
float32 = (root / "crates/rusty-lsl/src/timestamped_float32_sample_runtime.rs").read_text()
fixed = (root / "crates/rusty-lsl/src/fixed_width_numeric_sample_runtime.rs").read_text()
assert fixture["schema"] == "rusty.lsl.lslc_003e.bounded_fixed_record_transport.v1"
assert fixture["visibility"] == "crate-private"
assert "mod bounded_fixed_record_transport;" in lib
assert "pub mod bounded_fixed_record_transport" not in lib
assert "pub(crate) fn read_exact_bounded" in helper
assert "pub(crate) fn write_exact_bounded" in helper
for source in (float32, fixed):
    assert "read_exact_bounded" in source and "write_exact_bounded" in source
    assert "BoundedFixedRecordError" in source
assert fixture["historical_outcome"] == "blocked-before-dependency-closed-binding"
assert fixture["does_not_own"][-1] == "Manifold authority"
print("LSLC-003E bounded fixed-record transport evidence passed")
