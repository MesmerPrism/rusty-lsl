#!/usr/bin/env python3
import copy,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];PATH=ROOT/"fixtures/compatibility/lslc-004c-ipv4-multicast-discovery-observation.json"
def validate(d):
 assert d["schema"]=="rusty.lsl.compatibility.lslc_004c.v1"
 assert d["official"]=={"package":"pylsl","version":"1.18.2","library_version":117,"protocol_version":110,"implementation_source_used":False}
 assert d["scope"]=={"family":"ipv4","group":"239.255.172.215","port":16571,"interface_selection":"explicit-loopback","platform_class":"single-windows-desktop-host","directions":2,"repeats":2,"resolver_calls_per_repeat":1,"private_query_attempts_per_repeat":1,"deadline_seconds":3}
 assert all(v=="pass" for v in d["membership"].values())
 f=d["official_resolver_to_private_member"];assert f["outcome"]=="pass" and f["query_datagrams_per_repeat"]==[3,3] and f["latest_observed_query_ms"]<=1265 and f["call_elapsed_upper_ms"]<=1797 and f["retry_interpretation"]=="not-inferred" and len(f["query_sha256"])==3
 r=d["private_multicast_query_to_official_outlet"];assert r["outcome"]=="pass" and r["attempts_per_repeat"]==[1,1] and r["matching_responses_per_repeat"]==[1,1] and len(r["query_sha256"])==2 and len(r["response_sha256"])==2
 for v in [*d["provenance"].values()]:
  if isinstance(v,list): assert len(v)==2 and all(len(x)==64 for x in v)
  else: assert len(v)==64
 assert all(v is False for v in d["excluded_private_evidence"].values());assert all(v is False for v in d["limitations"].values())
d=json.loads(PATH.read_text(encoding="utf-8"));validate(d)
for route,value in [(('scope','group'),'224.0.0.1'),(('scope','family'),'ipv6'),(('scope','repeats'),1),(('membership','preflight_join_receive_drop_rejoin'),'fail'),(('official_resolver_to_private_member','retry_interpretation'),'portable-policy'),(('limitations','production_runtime'),True)]:
 x=copy.deepcopy(d);t=x
 for p in route[:-1]:t=t[p]
 t[route[-1]]=value
 try:validate(x)
 except (AssertionError,KeyError,TypeError):continue
 raise SystemExit("damaged fixture accepted")
routes={"AGENTS.md":"LSLC-004C","README.md":"LSLC-004C","docs/COMPATIBILITY.md":"LSLC-004C","docs/PROVENANCE.md":"LSLC-004C","docs/VALIDATION.md":"check_lslc_004c.ps1","fixtures/compatibility/README.md":PATH.name}
for p,m in routes.items():assert m in (ROOT/p).read_text(encoding="utf-8"),p
for p in [PATH,Path(__file__)]:assert p.read_bytes().endswith(b"\n")
print("LSLC-004C IPv4 multicast discovery observation passed")
