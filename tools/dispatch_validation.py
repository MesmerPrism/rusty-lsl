#!/usr/bin/env python3
import importlib.util, json, pathlib, subprocess, sys
root=pathlib.Path(__file__).resolve().parents[1]
policy_path=root/"tools/validation-policy.json"
def fail(msg): raise SystemExit("Validation policy rejected: "+msg)
def exact_immutable_receipt_at_head(relative_text,data):
 if not relative_text.startswith("morphospace/receipts/"): return False
 working=subprocess.run(["git","hash-object",f"--path={relative_text}","--stdin"],cwd=root,input=data,capture_output=True)
 head=subprocess.run(["git","rev-parse",f"HEAD:{relative_text}"],cwd=root,capture_output=True)
 return working.returncode==0 and head.returncode==0 and working.stdout.strip()==head.stdout.strip()
def self_test_public_boundary_policy(module):
 immutable=[
  "morphospace/receipts/rlsl-lslc-003m-standard-validation.json",
  "morphospace/receipts/rlsl-lslc-004j-standard-validation.json",
 ]
 for relative_text in immutable:
  data=(root/relative_text).read_bytes()
  if data.endswith((b"\n",b"\r")) or not exact_immutable_receipt_at_head(relative_text,data): fail("immutable receipt identity self-test")
 if exact_immutable_receipt_at_head(immutable[1],(root/immutable[1]).read_bytes()+b"damage"): fail("modified receipt accepted")
 if exact_immutable_receipt_at_head("morphospace/receipts/untracked-newline-less.json",b"{}"): fail("new receipt accepted")
 if exact_immutable_receipt_at_head("README.md",(root/immutable[0]).read_bytes()): fail("non-receipt accepted")
 damage=["C"+":\\private\\record.json","token"+"=example-secret-value","trailing "]
 if any(not module.content_violations(value) for value in damage): fail("content damage accepted")
 if not module.is_build_artifact(pathlib.Path("target/private.exe")): fail("build artifact accepted")
try: p=json.loads(policy_path.read_text(encoding="utf-8"))
except Exception as e: fail(str(e))
if p.get("schema")!="rusty.lsl.validation_policy.v1" or p.get("authority")!="tools/validation-policy.json": fail("schema or authority")
gates=p.get("gates"); profiles=p.get("profiles")
if not isinstance(gates,list) or not isinstance(profiles,dict): fail("shape")
ids=[g.get("id") for g in gates]
if len(ids)!=len(set(ids)) or any(g.get("state")!="current" for g in gates): fail("duplicate ID or noncurrent gate")
known=set(ids)
for name,members in profiles.items():
 if len(members)!=len(set(members)) or not set(members)<=known: fail("profile duplicate or orphan: "+name)
for g in gates:
 required={"id","owner","state","command","depends_on","change_categories","affected_paths","proves","does_not_prove","environment"}
 if set(g)!=required or not all(g[k] for k in ("owner","command","change_categories","affected_paths","proves","does_not_prove")): fail("malformed gate: "+str(g.get("id")))
 if not set(g["depends_on"])<=known: fail("orphan dependency")
if len(sys.argv)==2 and sys.argv[1]=="--self-test-public-boundary-policy":
 spec=importlib.util.spec_from_file_location("public_boundary",root/"tools/check_public_boundaries.py")
 module=importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
 module.self_test(); self_test_public_boundary_policy(module)
 print("Dispatcher public-boundary policy self-test passed."); raise SystemExit(0)
if len(sys.argv)==3 and sys.argv[1:]==["--internal-gate","public-boundary"]:
 spec=importlib.util.spec_from_file_location("public_boundary",root/"tools/check_public_boundaries.py")
 module=importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
 module.self_test(); violations=[]
 for path in module.repository_files():
  relative=path.relative_to(root); relative_text=relative.as_posix()
  violations.extend(f"{relative}: path contains {name}" for name in module.content_violations(relative_text) if name!="trailing whitespace")
  if module.is_build_artifact(relative): violations.append(f"{relative}: tracked build artifact"); continue
  data=path.read_bytes()
  if b"\0" in data: continue
  text_value=data.decode("utf-8",errors="strict")
  violations.extend(f"{relative}: {name}" for name in module.content_violations(text_value))
  if text_value and not text_value.endswith(("\n","\r")):
   if not exact_immutable_receipt_at_head(relative_text,data): violations.append(f"{relative}: missing terminal newline")
 if violations: fail("public boundary: "+"; ".join(violations))
 print("Current public boundary passed with exact immutable-evidence handling."); raise SystemExit(0)
profile="standard"
if len(sys.argv)==3 and sys.argv[1]=="--profile": profile=sys.argv[2]
elif len(sys.argv)!=1: fail("usage: --profile quick|standard|deep|ci")
if profile not in profiles: fail("unknown profile")
by_id={g["id"]:g for g in gates}; done=set()
def run(gid):
 if gid in done:return
 g=by_id[gid]
 for dep in g["depends_on"]: run(dep)
 print("VALIDATION-GATE",gid,flush=True)
 try:r=subprocess.run(g["command"],cwd=root,timeout=g["environment"]["timeout_seconds"])
 except subprocess.TimeoutExpired: fail("timeout: "+gid)
 if r.returncode: fail("failed: "+gid)
 done.add(gid)
for gid in profiles[profile]:run(gid)
print("Validation profile passed:",profile)
