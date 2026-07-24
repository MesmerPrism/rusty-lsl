# Copyright (C) 2026 Rusty LSL contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
import json
from pathlib import Path
root=Path(__file__).resolve().parents[1]
path=root/"fixtures/compatibility/lslc-002z-bounded-short-info-discovery-responder-interoperability.json"
data=json.loads(path.read_text(encoding="utf-8"))
assert data["schema"]=="rusty.lsl.lslc_002z.bounded_short_info_discovery_responder_interoperability.v1"
assert data["official"]=={"package":"pylsl","version":"1.18.2","library_version":117,"protocol_version":110,"implementation_source_used":False}
assert data["runtime"]["activation"]=="explicit" and data["runtime"]["transport"]=="ipv4-loopback-udp"
assert data["runtime"]["accepted_requests"]==2 and data["runtime"]["official_resolution"]=="pass"
for value in data["provenance"].values(): assert len(value)==64 and all(c in "0123456789abcdef" for c in value)
assert all(value is False for value in data["claims"].values())
text=path.read_text(encoding="utf-8").lower()
for forbidden in ["<?xml","<info>","127.0.0.1","private-002z","appdata","\\users\\"]: assert forbidden not in text
print("LSLC-002Z bounded responder interoperability evidence passed")
