import copy,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; FIXTURE=ROOT/"fixtures/compatibility/lslc-004h-active-interface-rust-multicast-observation.json"
def check(d):
 assert d["schema"]=="rusty.lsl.active_interface_rust_multicast_observation.v1"
 assert d["source"]=={"rusty_lsl_commit":"6a5747437dd510d4ca48fa77c4339027074b63ad","rusty_lsl_tree":"f322d6c05d901ac85d363478f92dba10cdade45e","requester_source_sha256":"1133224e6b741cd8136a518022d7bafdf1cc205aedc82e953c03130b5f171b55","responder_source_sha256":"afd4cc0cf9f370ba62ffda60b7e85cb460688c200ab66ae374c1c4da6e15f034"}
 assert d["scope"]=={"family":"ipv4","group":"239.255.172.215","port":16571,"interface_selection":"caller-explicit-active-private-ipv4","platform_class":"single-windows-desktop-host","repeats":2,"queries_per_repeat":1,"responses_per_repeat":1,"total_deadline_milliseconds":3000}
 assert d["results"]=={"query_response":["pass","pass"],"query_bytes":[39,39],"response_bytes":[512,512],"pre_io_cancellation":["pass","pass"],"no_response_deadline":["pass","pass"],"membership_cleanup":["pass","pass"],"socket_cleanup":["pass","pass"]}
 p=d["private_artifacts"]
 for k in ("driver_source_sha256","driver_binary_sha256","attempt_record_sha256"): assert len(p[k])==64 and int(p[k],16)>=0
 assert p["failed_attempts_preserved"]==2 and p["published"] is False
 assert d["limitations"]=={"single_host":True,"single_platform":True,"official_endpoint":False,"quest_or_device":False,"production_non_loopback_responder":False,"portable_interface_or_retry_policy":False}
 assert d["claims"]=={"observation_only":True,"production_bytes_changed":False,"runtime_or_activation_widened":False,"manifold_authority":False}
 text=json.dumps(d,sort_keys=True); assert not any(x in text for x in ("192.168.","Wi-Fi","InterfaceIndex","quest-pair","serial"))
data=json.loads(FIXTURE.read_text(encoding="utf-8"));check(data)
for a,b,v in [("scope","group","239.255.172.216"),("scope","port",16572),("scope","repeats",1),("results","query_response",["pass","fail"]),("limitations","single_host",False),("claims","production_bytes_changed",True)]:
 bad=copy.deepcopy(data);bad[a][b]=v
 try:check(bad)
 except AssertionError:pass
 else:raise AssertionError(f"damage accepted: {a}.{b}")
for route,marker in {"AGENTS.md":"LSLC-004H","README.md":"LSLC-004H","docs/COMPATIBILITY.md":"LSLC-004H","docs/PROVENANCE.md":"LSLC-004H","docs/VALIDATION.md":"check_lslc_004h.ps1","fixtures/compatibility/README.md":FIXTURE.name}.items(): assert marker in (ROOT/route).read_text(encoding="utf-8")
print("LSLC-004H active-interface Rust multicast observation passed (6 damaged fixtures rejected)")
