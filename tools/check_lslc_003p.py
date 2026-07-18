#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-003p-bounded-multichannel-record-sequence-runtime.json"
SOURCE = ROOT / "crates/rusty-lsl/src/fixed_width_numeric_sample_runtime.rs"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
require(fixture["bounds"]["channel_count"] == 2, "channel bound drifted")
require(fixture["bounds"]["caller_record_count"] == 3, "record bound drifted")
require(fixture["bounds"]["formats"] == ["double64", "int32", "int16", "int8"], "format closure drifted")
require(not any(fixture["claims"].values()), "an excluded claim became true")

source = SOURCE.read_text(encoding="utf-8")
for token in (
    "FixedWidthNumericPairRecord",
    "FixedWidthNumericRecordSequence",
    "run_fixed_width_numeric_sequence_outlet",
    "run_fixed_width_numeric_sequence_inlet",
    "SequenceFormatMismatch",
    "lslc_003p_four_formats_preserve_two_channels_three_records_and_cleanup",
    "lslc_003p_shape_format_timestamp_and_truncation_fail_closed",
    "lslc_003p_truncation_is_addressable_for_every_accepted_width",
    "lslc_003p_width_shift_retains_marker_error_ownership_and_cleanup",
    "lslc_003p_caller_cancellation_precedes_deadline_and_teardown_repeats",
):
    require(token in source, f"missing runtime evidence: {token}")

routes = {
    "AGENTS.md": "LSLC-003P",
    "README.md": "LSLC-003P",
    "docs/ARCHITECTURE.md": "LSLC-003P",
    "docs/COMPATIBILITY.md": "LSLC-003P",
    "docs/PROVENANCE.md": "LSLC-003P",
    "docs/VALIDATION.md": "check_lslc_003p.ps1",
    "fixtures/compatibility/README.md": FIXTURE.name,
}
for path, marker in routes.items():
    require(marker in (ROOT / path).read_text(encoding="utf-8"), f"missing route: {path}")

for path in (FIXTURE, SOURCE, ROOT / "tools/check_lslc_003p.py"):
    require(path.read_bytes().endswith(b"\n"), f"missing terminal newline: {path.relative_to(ROOT)}")

print("LSLC-003P bounded multichannel record-sequence runtime passed")
