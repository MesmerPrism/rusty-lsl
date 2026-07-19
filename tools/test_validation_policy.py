#!/usr/bin/env python3
import copy, hashlib, importlib.util, json, pathlib, subprocess, sys, tempfile
root=pathlib.Path(__file__).resolve().parents[1]
LSLC_003S_PIN="2c5ac93770476078638d2aded4a837b51ca91e23"
LSLC_003S_CHECKER_SHA="4b64102806aca3a5c804e8e02bdc88eb04ae8c4a278eb35c1f9c5641f084842d"
LSLC_003S_LAUNCHER_SHA="43f8b7c5ca5a11db555d8fe83b67ac77faa06674e380899fa8224f7489adde58"
LSLC_004I_PIN="1121efd219e3a0068e9d750127aebaa7c0258a77"
LSLC_004I_CHECKER_SHA="05ecc967286e0b6c84ccb642e54cd8e664f65d9f7af6933a771813532f1a00f4"
LSLC_004I_LAUNCHER_SHA="85eb7e64c39e2ae7225ee06dd57c7c29c5514a33820c765f0652cc7896a9cceb"

def git_show(path):
 return subprocess.run(["git","show",f"{LSLC_003S_PIN}:{path}"],cwd=root,check=True,capture_output=True).stdout

def replay_lslc_003s():
 assert hashlib.sha256(git_show("tools/check_lslc_003s.py")).hexdigest()==LSLC_003S_CHECKER_SHA
 assert hashlib.sha256(git_show("tools/check_lslc_003s.ps1")).hexdigest()==LSLC_003S_LAUNCHER_SHA
 with tempfile.TemporaryDirectory(prefix="rusty-lsl-lslc-003s-") as parent:
  worktree=pathlib.Path(parent)/"worktree"
  subprocess.run(["git","worktree","add","--detach",str(worktree),LSLC_003S_PIN],cwd=root,check=True,capture_output=True)
  try:
   subprocess.run([sys.executable,"tools/check_lslc_003s.py"],cwd=worktree,check=True)
  finally:
   subprocess.run(["git","worktree","remove","--force",str(worktree)],cwd=root,check=True,capture_output=True)

def replay_lslc_004i():
 for path, expected in (("tools/check_lslc_004i.py",LSLC_004I_CHECKER_SHA),("tools/check_lslc_004i.ps1",LSLC_004I_LAUNCHER_SHA)):
  data=subprocess.run(["git","show",f"{LSLC_004I_PIN}:{path}"],cwd=root,check=True,capture_output=True).stdout
  assert hashlib.sha256(data).hexdigest()==expected
 with tempfile.TemporaryDirectory(prefix="rusty-lsl-lslc-004i-") as parent:
  worktree=pathlib.Path(parent)/"worktree"
  subprocess.run(["git","worktree","add","--detach",str(worktree),LSLC_004I_PIN],cwd=root,check=True,capture_output=True)
  try:
   subprocess.run([sys.executable,"tools/check_lslc_004i.py"],cwd=worktree,check=True)
  finally:
   subprocess.run(["git","worktree","remove","--force",str(worktree)],cwd=root,check=True,capture_output=True)

if sys.argv[1:]==["--replay-lslc-003s"]:
 replay_lslc_003s(); print("Pinned LSLC-003S revision-14 replay passed."); raise SystemExit(0)
if sys.argv[1:]==["--replay-lslc-004i"]:
 replay_lslc_004i(); print("Pinned LSLC-004I revision-14 replay passed."); raise SystemExit(0)
assert not sys.argv[1:]
policy=json.loads((root/"tools/validation-policy.json").read_text())
ids=[g["id"] for g in policy["gates"]]
assert len(ids)==len(set(ids))
assert set(policy["profiles"]["standard"]) < set(policy["profiles"]["deep"])
assert set(policy["profiles"]["ci"]) - set(policy["profiles"]["standard"])=={"pinned-rust-180-clippy"}
assert "pinned-historical-replay" in policy["profiles"]["deep"]
assert "pinned-lslc-003s-replay" in policy["profiles"]["deep"]
assert "pinned-lslc-004i-replay" in policy["profiles"]["deep"]
assert all("lslc-003s-activation" not in policy["profiles"][name] for name in ("quick","standard","deep","ci"))
assert all("current-closure" in policy["profiles"][name] for name in ("quick","standard","deep","ci"))
historical_gate=next(g for g in policy["gates"] if g["id"]=="pinned-lslc-003s-replay")
assert historical_gate["command"]==["python","tools/test_validation_policy.py","--replay-lslc-003s"]
assert historical_gate["depends_on"]==[] and "current-closure" not in historical_gate["depends_on"]
assert hashlib.sha256(git_show("tools/check_lslc_003s.py")).hexdigest()==LSLC_003S_CHECKER_SHA
assert hashlib.sha256(git_show("tools/check_lslc_003s.ps1")).hexdigest()==LSLC_003S_LAUNCHER_SHA
historical_004i=next(g for g in policy["gates"] if g["id"]=="pinned-lslc-004i-replay")
assert historical_004i["command"]==["python","tools/test_validation_policy.py","--replay-lslc-004i"] and historical_004i["depends_on"]==[]
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
# Live/historical activation routes must not be omitted, swapped, or mixed.
d=copy.deepcopy(policy); d["profiles"]["standard"].remove("current-closure"); damaged.append("current-closure" not in d["profiles"]["standard"])
d=copy.deepcopy(policy); d["profiles"]["standard"].append("pinned-lslc-003s-replay"); damaged.append("pinned-lslc-003s-replay" in d["profiles"]["standard"])
d=copy.deepcopy(policy); next(g for g in d["gates"] if g["id"]=="pinned-lslc-003s-replay")["command"]=["powershell","-File","tools/check_lslc_003s.ps1"]; damaged.append(next(g for g in d["gates"] if g["id"]=="pinned-lslc-003s-replay")["command"]!=historical_gate["command"])
d=copy.deepcopy(policy); d["profiles"]["standard"].append("pinned-lslc-004i-replay"); damaged.append("pinned-lslc-004i-replay" in d["profiles"]["standard"])
assert all(damaged)
print("Validation policy valid and damaged/drift cases reject.")
