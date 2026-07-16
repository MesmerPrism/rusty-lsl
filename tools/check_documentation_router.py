#!/usr/bin/env python3
import hashlib, json, pathlib, sys
root=pathlib.Path(__file__).resolve().parents[1]
agents=(root/"AGENTS.md").read_text(encoding="utf-8")
readme=(root/"README.md").read_text(encoding="utf-8")
validation=(root/"docs/VALIDATION.md").read_text(encoding="utf-8")
required=["tools/validation-policy.json","tools/dispatch_validation.py","VALIDATION-THROUGH-LSLC-003M.md"]
expected_counts={required[0]:2,required[1]:3,required[2]:2}
if any(validation.count(x)!=expected_counts[x] for x in required): sys.exit("validation router drift")
if "LSLC Work-Unit History" not in agents or "README-THROUGH-LSLC-003L" not in readme: sys.exit("first-hop archive route drift")
archive=root/"docs/history/VALIDATION-THROUGH-LSLC-003M.md"
binding=json.loads((root/"tools/validation-archive-binding.json").read_text())
if hashlib.sha256(archive.read_bytes()).hexdigest()!=binding["sha256"]: sys.exit("validation archive drift")
print("Documentation router and exact archive passed.")
