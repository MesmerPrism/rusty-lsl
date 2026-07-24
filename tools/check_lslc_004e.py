#!/usr/bin/env python3
import copy,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-004e-explicit-loopback-multicast-responder-runtime.json"
def validate(data):
    assert data["schema"]=="rusty.lsl.compatibility.lslc_004e.v1"
    assert data["composition"]=={"address_family":"ipv4","group":"239.255.172.215","port":16571,"explicit_interface":"127.0.0.1","required_module":"short-info-discovery-responder","queries":1,"responses":1,"finite_deadline":True,"cancellation_preserved":True,"membership_cleanup":True,"socket_cleanup":True,"synthetic_loopback":"pass","non_loopback_rejection":"pass"}
    assert data["limitations"]=={"platform":"windows-single-platform","configurable_group":False,"interface_enumeration":False,"default_interface_selection":False,"portable_retry_policy":False}
    assert all(not value for value in data["claims"].values())
data=json.loads(FIXTURE.read_text(encoding="utf-8"));validate(data)
for route,value in [(("composition","group"),"239.255.172.216"),(("composition","port"),16572),(("composition","queries"),2),(("composition","explicit_interface"),"0.0.0.0"),(("limitations","platform"),"portable"),(("claims","generic_membership_policy"),True)]:
    damaged=copy.deepcopy(data);target=damaged
    for part in route[:-1]:target=target[part]
    target[route[-1]]=value
    try:validate(damaged)
    except (AssertionError,KeyError,TypeError):continue
    raise SystemExit("damaged fixture accepted")
source=(ROOT/"crates/rusty-lsl/src/short_info_discovery_responder_runtime.rs").read_text(encoding="utf-8")
for marker in ["run_explicit_loopback_multicast_short_info_responder","DOCUMENTED_IPV4_MULTICAST_GROUP","DOCUMENTED_IPV4_MULTICAST_PORT","NonLoopbackMulticastInterface","run_short_info_responder_on_socket","lslc_002z_cancellation_deadline_limits_and_cleanup_are_finite","lslc_004e_exact_group_explicit_loopback_serves_one_query_and_cleans_up"]:assert marker in source
routes={"AGENTS.md":"LSLC-004E","README.md":"LSLC-004E","docs/ARCHITECTURE.md":"LSLC-004E","docs/COMPATIBILITY.md":"LSLC-004E","docs/VALIDATION.md":"check_lslc_004e.ps1","fixtures/compatibility/README.md":FIXTURE.name}
for path,marker in routes.items():assert marker in (ROOT/path).read_text(encoding="utf-8"),path
print("LSLC-004E explicit-loopback multicast responder passed (6 damaged fixtures rejected)")
