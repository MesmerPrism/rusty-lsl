#!/usr/bin/env python3
import copy, hashlib, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-004d-ipv4-multicast-discovery-runtime-conformance.json"
SOURCE=ROOT/"crates/rusty-lsl/src/udp_discovery.rs"
def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_004d.v1"
    assert data["production_baseline_revision"]=="b8407facecbc1e24e7759dafed52c9af90d8e4bc"
    assert data["production_prefix_sha256"]=="41c2393adb561669aa824e3635ae70e85b3b412892a38100fc835d3bfc8b4263"
    assert data["composition"]=={"address_family":"ipv4","group":"239.255.172.215","port":16571,"interface":"127.0.0.1","explicit_bind":True,"joined_synthetic_peer":True,"queries":1,"responses":1,"finite_deadline":True,"cancellation_preserved":True,"membership_cleanup":True,"socket_cleanup":True,"synthetic_loopback":"pass"}
    assert data["limitations"]=={"platform":"windows-single-platform","portable_retry_policy":False,"interface_enumeration":False,"default_interface_selection":False,"non_loopback":False}
    assert all(not value for value in data["claims"].values())
data=json.loads(FIXTURE.read_text(encoding="utf-8"));validate(data)
for route,value in [(("composition","group"),"239.255.172.216"),(("composition","port"),16572),(("composition","queries"),2),(("composition","cancellation_preserved"),False),(("limitations","platform"),"portable"),(("claims","production_change"),True)]:
    damaged=copy.deepcopy(data);target=damaged
    for part in route[:-1]:target=target[part]
    target[route[-1]]=value
    try:validate(damaged)
    except (AssertionError,KeyError,TypeError):continue
    raise SystemExit("damaged fixture accepted")
current=SOURCE.read_bytes().split(b"#[cfg(test)]",1)[0]
baseline=subprocess.run(["git","show",f"{data['production_baseline_revision']}:crates/rusty-lsl/src/udp_discovery.rs"],cwd=ROOT,check=True,capture_output=True).stdout.split(b"#[cfg(test)]",1)[0]
assert current==baseline,"production prefix changed"
assert hashlib.sha256(current).hexdigest()==data["production_prefix_sha256"]
source=SOURCE.read_text(encoding="utf-8")
for marker in ["lslc_004d_explicit_loopback_requester_composes_with_one_joined_peer","lslc_002p_pre_send_and_blocked_receive_cancellation_are_bounded","lslc_002p_deadline_releases_the_caller_selected_port"]:assert marker in source
routes={"AGENTS.md":"LSLC-004D","README.md":"LSLC-004D","docs/ARCHITECTURE.md":"LSLC-004D","docs/COMPATIBILITY.md":"LSLC-004D","docs/VALIDATION.md":"check_lslc_004d.ps1","fixtures/compatibility/README.md":FIXTURE.name}
for path,marker in routes.items():assert marker in (ROOT/path).read_text(encoding="utf-8"),path
print("LSLC-004D multicast requester runtime conformance passed (6 damaged fixtures rejected)")
