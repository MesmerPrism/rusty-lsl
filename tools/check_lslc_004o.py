#!/usr/bin/env python3
import copy, hashlib, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-004o-official-resolver-production-responder-observation.json"
def validate(d):
    assert d["schema"]=="rusty.lsl.official_resolver_production_responder_observation.v1"
    assert d["source"]=={"rusty_lsl_commit":"5d73d82196861570b12e5f94cbf5f6ddb803bc67","rusty_lsl_tree":"ffc2afdedb544031f867112928cc94d000676269","responder_production_prefix_sha256":"2adb4cb64fdb7e66c1615b1b0f0cb9742304cc15b0e62c2604cc3b13645a3f4b"}
    assert d["official"]=={"pylsl":"1.18.2","liblsl":117,"protocol":110}
    assert d["scope"]=={"family":"ipv4","group":"239.255.172.215","port":16571,"interface_selection":"caller-explicit-active-private-ipv4","platform_class":"single-windows-desktop-host","serialized_repeats":2,"queries_per_repeat":1,"responses_per_repeat":1,"maximum_deadline_milliseconds":12000}
    assert d["results"]=={"production_responder_started":["pass","pass"],"official_query_handled":["pass","pass"],"production_response_sent":["pass","pass"],"official_matching_source_resolved":["pass","pass"],"finite_completion":["pass","pass"],"membership_cleanup":["pass","pass"],"socket_cleanup":["pass","pass"],"elapsed_milliseconds":[1391.0,1390.0]}
    assert d["private_artifacts"]=={"driver_source_sha256":"3bf0ac5bdfd65b625e0137949735b7de8bf3881810081dfd7ebb0d40ed70a149","successful_attempt_sha256":["8158fae7cae999f2ae876c6c38132900c908937079004b6a7a0b9fcd35e507aa","b9fc0203384b5d692e4f5a55e9bdf24683988b4f7a8d06f914c5780b4dd4d0b8"],"failed_attempt_sha256":["08120a5161ac632395dcd53a4125f271cc4121c6c46d58fb251d37361491f883","1676d6d00648bb95eec6d564e2249372d6b96202040530b3fdf14925debefa1b","f98afc9d4d4e886bca14ef3dc6d6afe0c85b67efe5a9d33901b8db39b252b820"],"capsule_sha256":"e55b357bc44a590c591c1a0ee17a547ace308a6dc39a14de01fb295b9d7d5076","failed_attempts_preserved":3,"published":False}
    assert d["limitations"]=={"single_host":True,"single_platform":True,"production_test_harness":True,"production_bytes_changed":False,"official_outlet":False,"cross_host":False,"cross_platform":False,"quest_or_device":False,"interface_enumeration":False,"default_interface_selection":False,"ambient_fallback":False,"portable_retry_policy":False,"ipv6":False}
    assert d["claims"]=={"bounded_official_to_production_observation":True,"broad_compatibility":False,"runtime_or_activation_widened":False,"routing_or_admission_authority":False,"manifold_authority":False}
    enc=json.dumps(d,sort_keys=True)
    assert not any(x in enc for x in ("192.168.","10.0.","Wi-Fi","InterfaceIndex","RLSL_LSLC_004O_INTERFACE","device_serial","serial_number"))
d=json.loads(FIXTURE.read_text(encoding="utf-8")); validate(d)
for route,value in [(("official","pylsl"),"1.18.1"),(("scope","group"),"239.255.172.216"),(("scope","serialized_repeats"),1),(("results","official_matching_source_resolved"),["pass","fail"]),(("results","membership_cleanup"),["pass","fail"]),(("private_artifacts","capsule_sha256"),"0"*64),(("limitations","default_interface_selection"),True),(("limitations","cross_platform"),True),(("claims","broad_compatibility"),True),(("claims","manifold_authority"),True)]:
    x=copy.deepcopy(d); t=x
    for p in route[:-1]: t=t[p]
    t[route[-1]]=value
    try: validate(x)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit(f"damaged fixture accepted: {route}")
source=(ROOT/"crates/rusty-lsl/src/short_info_discovery_responder_runtime.rs").read_bytes()
assert hashlib.sha256(source.split(b"#[cfg(test)]",1)[0]).hexdigest()==d["source"]["responder_production_prefix_sha256"]
text=source.decode("utf-8")
for marker in ("lslc_004o_private_active_interface_production_responder","RLSL_LSLC_004O_INTERFACE","run_explicit_ipv4_multicast_short_info_responder","ShortInfoResponderTermination::RequestLimit"):
    assert marker in text
for path,marker in {"AGENTS.md":"LSLC-004O","README.md":"LSLC-004O","docs/COMPATIBILITY.md":"LSLC-004O","docs/PROVENANCE.md":"LSLC-004O","docs/VALIDATION.md":"check_lslc_004o.ps1","fixtures/compatibility/README.md":FIXTURE.name}.items(): assert marker in (ROOT/path).read_text(encoding="utf-8"),path
print("LSLC-004O official resolver to production responder observation passed (10 damaged fixtures rejected)")
