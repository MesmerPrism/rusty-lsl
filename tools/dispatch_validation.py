#!/usr/bin/env python3
import importlib.util, json, pathlib, subprocess, sys
root=pathlib.Path(__file__).resolve().parents[1]
policy_path=root/"tools/validation-policy.json"
def fail(msg): raise SystemExit("Validation policy rejected: "+msg)
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
   historical_base="68db7ce0f92ef9cab633d0b54c35744a7814db53"
   is_receipt=relative_text.startswith("morphospace/receipts/")
   existed_at_base=is_receipt and subprocess.run(["git","cat-file","-e",f"{historical_base}:{relative_text}"],cwd=root,capture_output=True).returncode==0
   unchanged_since_base=existed_at_base and subprocess.run(["git","diff","--quiet",historical_base,"HEAD","--",relative_text],cwd=root).returncode==0
   if not unchanged_since_base: violations.append(f"{relative}: missing terminal newline")
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
