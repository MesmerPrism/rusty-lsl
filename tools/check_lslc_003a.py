import json
from pathlib import Path
p=Path(__file__).resolve().parents[1]/"fixtures/compatibility/lslc-003a-fixed-width-sample-format-interoperability-observation.json"
d=json.loads(p.read_text(encoding="utf-8"))
assert d["schema"]=="rusty.lsl.lslc_003a.fixed_width_sample_format_interoperability_observation.v1"
assert d["official"]["implementation_source_used"] is False
forward=d["directions"]["official_outlet_to_private_inlet"]
for name,width in {"double64":8,"int32":4,"int16":2,"int8":1}.items():
    assert forward[name]["width_bytes"]==width
    assert all(forward[name][k]=="pass" for k in ["marker","record_width","initialization_timestamp","sample_timestamp","sample_value"])
    assert forward[name]["initialization_values"]=="format-specific"
assert forward["int64"]["outcome"]=="public-binding-unavailable"
assert set(d["directions"]["private_outlet_to_official_inlet"].values())=={"test-pattern-mismatch"}
for h in d["provenance"].values(): assert len(h)==64 and all(c in "0123456789abcdef" for c in h)
assert all(v is False for v in d["claims"].values())
text=p.read_text(encoding="utf-8").lower()
for bad in ["<?xml","<info>","127.0.0.1","private-003a","appdata","\\users\\"]: assert bad not in text
print("LSLC-003A fixed-width interoperability observation passed")
