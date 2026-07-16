#!/usr/bin/env python3
import json, pathlib, subprocess, sys
root = pathlib.Path(__file__).resolve().parents[1]
r = subprocess.run(["cargo","metadata","--offline","--locked","--no-deps","--format-version","1"],cwd=root,capture_output=True,text=True)
if r.returncode: sys.exit(r.returncode)
m=json.loads(r.stdout); p=m["packages"]
assert len(p)==1 and p[0]["name"]=="rusty-lsl" and p[0]["publish"]==[]
assert p[0]["features"]=={} and p[0]["dependencies"]==[]
assert pathlib.Path(p[0]["manifest_path"]).resolve()==(root/"crates/rusty-lsl/Cargo.toml").resolve()
assert m["workspace_members"]==[p[0]["id"]]
targets=p[0]["targets"]
assert len([t for t in targets if "lib" in t["kind"]])==1
assert [t["name"] for t in targets if "test" in t["kind"]]==["public_api"]
print("Cargo composition shape passed.")
