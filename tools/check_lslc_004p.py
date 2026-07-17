#!/usr/bin/env python3
import copy, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-004p-official-outlet-production-requester-observation.json"
def validate(d):
    assert d["schema"]=="rusty.lsl.official_outlet_production_requester_observation.v1"
    assert d["source"]=={"rusty_lsl_commit":"3cb859d2dd4a0217bb2e2dc035047bdc4c5dfa6e","rusty_lsl_tree":"d0a3fc99b459cf771adfd9b06ce834bc2165ed7d","accepted_requester_blob":"c328a177ece4d5ac0b7cd4aeea6abc0992146cfb"}
    assert d["official"]=={"pylsl":"1.18.2","liblsl":117,"protocol":110}
    assert d["scope"]=={"family":"ipv4","group":"239.255.172.215","port":16571,"destination_selection":"caller-explicit","interface_path":"one-bounded-active-private-ipv4","platform_class":"single-windows-desktop-host","serialized_repeats":2,"queries_per_repeat":1,"responses_per_repeat":1,"maximum_deadline_milliseconds":3000}
    assert d["requester_datagrams"]==[{"sha256":"e6421e430229d8c824bad54c7028a993c61a6b22c9bea67752d7a1ff09d14a4b","byte_length":64},{"sha256":"96429fdc5a483c621fffc405a42a55f4aa022d0f059f1203620243497d53405f","byte_length":64}]
    assert d["results"]=={"official_outlet_started":["pass","pass"],"production_query_sent":["pass","pass"],"official_response_admitted":["pass","pass"],"matching_source_resolved":["pass","pass"],"response_limit_termination":["pass","pass"],"socket_cleanup":["pass","pass"],"elapsed_milliseconds":[844.0,140.0]}
    assert d["private_artifacts"]=={"driver_source_sha256":"28954e9eb26d21a830fe9591a290fedb6161e90fa2961ad0b56877a15cda64b5","successful_attempt_sha256":["de532db8899601ea28d2babee68b9451c3fc2b19ebcec3ae5c86b8e9ea13d6f5","ad8c191e8d000a070333c144ddc015650e625a61f9bc455ec48f43feaa96c2ee"],"capsule_sha256":"d6e8111d3852efc367c0f9d9c5669e9bdb511f191bea7151d4aab1ee6ee961d2","failed_attempts_preserved":1,"published":False}
    assert d["limitations"]=={"single_host":True,"single_platform":True,"production_test_harness":True,"production_bytes_changed":False,"requester_interface_selection_policy":False,"multicast_portability":False,"cross_host":False,"cross_platform":False,"quest_or_device":False,"interface_enumeration":False,"default_interface_selection":False,"ambient_fallback":False,"portable_retry_policy":False,"ipv6":False}
    assert d["claims"]=={"bounded_official_outlet_to_production_requester_observation":True,"broad_compatibility":False,"runtime_or_activation_widened":False,"routing_or_admission_authority":False,"manifold_authority":False}
    encoded=json.dumps(d,sort_keys=True)
    assert not any(x in encoded for x in ("192.168.","10.0.","Wi-Fi","InterfaceIndex","RLSL_LSLC_004P_INTERFACE","device_serial","serial_number"))
d=json.loads(FIXTURE.read_text(encoding="utf-8")); validate(d)
for route,value in [(('official','pylsl'),'1.18.1'),(('scope','group'),'239.255.172.216'),(('scope','serialized_repeats'),1),(('requester_datagrams',),[]),(('results','matching_source_resolved'),['pass','fail']),(('results','socket_cleanup'),['pass','fail']),(('private_artifacts','capsule_sha256'),'0'*64),(('limitations','default_interface_selection'),True),(('claims','broad_compatibility'),True),(('claims','manifold_authority'),True)]:
    x=copy.deepcopy(d); t=x
    for p in route[:-1]: t=t[p]
    t[route[-1]]=value
    try: validate(x)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit(f"damaged fixture accepted: {route}")
source=(ROOT/'crates/rusty-lsl/src/udp_discovery.rs').read_bytes()
marker=b'    #[test]\n    #[ignore = "requires the private pinned official LSLC-004P outlet harness"]'
prefix,sep,_=source.partition(marker); assert sep
accepted=subprocess.check_output(['git','cat-file','blob',d['source']['accepted_requester_blob']],cwd=ROOT)
assert prefix[:-1]+b'}\n'==accepted
text=source.decode('utf-8')
for value in ('lslc_004p_private_official_outlet_observation_driver','RLSL_LSLC_004P_INTERFACE','UdpDiscoveryTermination::ResponseLimit','239, 255, 172, 215'): assert value in text
for path,value in {'AGENTS.md':'LSLC-004P','README.md':'LSLC-004P','docs/COMPATIBILITY.md':'LSLC-004P','docs/PROVENANCE.md':'LSLC-004P','docs/VALIDATION.md':'check_lslc_004p.ps1','fixtures/compatibility/README.md':FIXTURE.name}.items(): assert value in (ROOT/path).read_text(encoding='utf-8'),path
print('LSLC-004P official outlet to production requester observation passed (10 damaged fixtures rejected)')
