import json
from pathlib import Path
p=Path(__file__).resolve().parents[1]/"fixtures/compatibility/lslc-003b-fixed-width-numeric-sample-runtime-family.json";d=json.loads(p.read_text())
assert d["schema"]=="rusty.lsl.lslc_003b.fixed_width_numeric_sample_runtime_family.v1"
assert {k:v["width"] for k,v in d["formats"].items()}=={"double64":8,"int32":4,"int16":2,"int8":1}
assert all(v["forward"]==v["reverse"]=="pass" for v in d["formats"].values())
assert d["runtime"]=={"records":1,"channels":1,"initialization_records":2,"bounded":True}
assert all(len(h)==64 for h in d["provenance"].values()) and all(v is False for v in d["claims"].values())
text=p.read_text().lower();assert not any(x in text for x in ["<?xml","127.0.0.1","private-003b","\\users\\"])
print("LSLC-003B fixed-width numeric runtime evidence passed")
