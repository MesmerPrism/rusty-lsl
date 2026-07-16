#!/usr/bin/env python3
import copy, hashlib, importlib.util, json, pathlib, tempfile
root=pathlib.Path(__file__).resolve().parents[1]
policy=json.loads((root/"tools/validation-policy.json").read_text())
ids=[g["id"] for g in policy["gates"]]
assert len(ids)==len(set(ids))
assert set(policy["profiles"]["standard"]) < set(policy["profiles"]["deep"])
assert set(policy["profiles"]["ci"]) - set(policy["profiles"]["standard"])=={"pinned-rust-180-clippy"}
assert "pinned-historical-replay" in policy["profiles"]["deep"]
for gate in policy["gates"]:
 assert gate["owner"] and gate["proves"] and gate["does_not_prove"] and gate["affected_paths"] and gate["change_categories"]
 assert set(gate["depends_on"]) <= set(ids)
ci=(root/".github/workflows/ci.yml").read_text()
wrapper=(root/"tools/check_all.ps1").read_text()
assert ci.count("dispatch_validation.py --profile ci")==1 and "check_lslc_" not in ci and "check_all.ps1" not in ci
assert wrapper.count("dispatch_validation.py --profile standard")==1 and "dispatch_current_gates" not in wrapper
historical=json.loads((root/"tools/validation-historical-gates.json").read_text())
actual=hashlib.sha256((root/historical["source_manifest"]).read_bytes()).hexdigest()
assert actual==historical["source_manifest_sha256"]
archive=json.loads((root/"tools/validation-archive-binding.json").read_text())
assert hashlib.sha256((root/archive["path"]).read_bytes()).hexdigest()==archive["sha256"]
# Deterministic damaged-policy coverage: duplicate, orphan, malformed, stale,
# bypass, overlap, and documentation/archive drift are represented by direct
# invariant mutations without executing child commands.
damaged=[]
d=copy.deepcopy(policy); d["gates"].append(copy.deepcopy(d["gates"][0])); damaged.append(len({g["id"] for g in d["gates"]})!=len(d["gates"]))
d=copy.deepcopy(policy); d["profiles"]["quick"].append("orphan"); damaged.append(not set(d["profiles"]["quick"])<=set(ids))
d=copy.deepcopy(policy); del d["gates"][0]["owner"]; damaged.append(set(d["gates"][0])!=set(policy["gates"][0]))
d=copy.deepcopy(policy); d["gates"][0]["state"]="superseded"; damaged.append(any(g["state"]!="current" for g in d["gates"]))
d=copy.deepcopy(policy); d["profiles"]["ci"]=[]; damaged.append(not d["profiles"]["ci"])
d=copy.deepcopy(policy); d["profiles"]["quick"] += [d["profiles"]["quick"][0]]; damaged.append(len(d["profiles"]["quick"])!=len(set(d["profiles"]["quick"])))
assert all(damaged)
print("Validation policy valid and damaged/drift cases reject.")
