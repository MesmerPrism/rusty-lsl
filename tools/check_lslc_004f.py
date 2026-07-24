#!/usr/bin/env python3
import copy,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-004f-exact-multicast-discovery-composition-conformance.json"
SOURCES={"requester":"crates/rusty-lsl/src/udp_discovery.rs","responder":"crates/rusty-lsl/src/short_info_discovery_responder_runtime.rs"}
def validate(data):
 assert data["schema"]=="rusty.lsl.compatibility.lslc_004f.v1"
 assert data["production_baseline_revision"]=="a3f4b1d2eeaa1d5c50ebcb31fa782fcbb8c72874"
 assert data["composition"]=={"address_family":"ipv4","group":"239.255.172.215","port":16571,"interface":"127.0.0.1","queries":1,"responses":1,"requester_module":"udp-discovery","responder_module":"short-info-discovery-responder","finite_deadline":True,"cancellation_preserved":True,"cleanup":True,"production_prefixes_unchanged":True,"synthetic_loopback":"pass"}
 assert all(not value for value in data["claims"].values())
data=json.loads(FIXTURE.read_text(encoding="utf-8"));validate(data)
for route,value in [(("composition","group"),"239.255.172.216"),(("composition","queries"),2),(("composition","responses"),2),(("composition","production_prefixes_unchanged"),False),(("claims","production_change"),True),(("claims","devices"),True)]:
 damaged=copy.deepcopy(data);target=damaged
 for part in route[:-1]:target=target[part]
 target[route[-1]]=value
 try:validate(damaged)
 except (AssertionError,KeyError,TypeError):continue
 raise SystemExit("damaged fixture accepted")
for role,path in SOURCES.items():
 current=(ROOT/path).read_bytes().split(b"#[cfg(test)]",1)[0]
 baseline=subprocess.run(["git","show",f"{data['production_baseline_revision']}:{path}"],cwd=ROOT,check=True,capture_output=True).stdout.split(b"#[cfg(test)]",1)[0]
 assert current==baseline,f"{role} production prefix changed"
 assert hashlib.sha256(current).hexdigest()==data[f"{role}_prefix_sha256"]
source=(ROOT/SOURCES["responder"]).read_text(encoding="utf-8")
assert "lslc_004f_unchanged_requester_and_responder_compose_exactly_once" in source
routes={"AGENTS.md":"LSLC-004F","README.md":"LSLC-004F","docs/ARCHITECTURE.md":"LSLC-004F","docs/COMPATIBILITY.md":"LSLC-004F","docs/VALIDATION.md":"check_lslc_004f.ps1","fixtures/compatibility/README.md":FIXTURE.name}
for path,marker in routes.items():assert marker in (ROOT/path).read_text(encoding="utf-8"),path
print("LSLC-004F exact multicast discovery composition passed (6 damaged fixtures rejected)")
