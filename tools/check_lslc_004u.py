#!/usr/bin/env python3
import copy,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];F=ROOT/'fixtures/compatibility/lslc-004u-typed-udp-response-projection.json'
def validate(d):
 assert d['schema']=='rusty.lsl.typed_udp_response_projection.v1'
 assert d['source']=={'claimed_head':'1c2a87e1417f29757283485b4598494a87e2dfea','claimed_tree':'641d2ca14a025571fe430ba4e4d59e81200601a3','unchanged_udp_blob':'81fdbbbda57dbe0f25020926dda1cab3b70fed69'}
 assert d['contract']=={'input':'borrowed-accepted-udp-response','caller_envelope_limits':True,'caller_admission_limits':True,'output':'owned-typed-observation-plus-source','source_preserved':True,'response_not_consumed':True}
 assert d['delegation']=={'invalid_utf8':'valid-up-to','envelope':'existing-error','typed_admission':'existing-error','noncanonical_channel_count':'invalid-channel-count'}
 assert d['positive']=={'query_id':19,'stream_name':'projected','channel_count':1,'source':'independent-loopback-test-value','into_parts_preserved':True}
 assert d['implementation']=={'dependency_free':True,'unsafe':False,'io':False,'activation':False,'udp_runtime_changed':False,'new_parsing_semantics':False,'implicit_limits':False}
 assert d['limitations']=={'network_policy':False,'device_scope':False,'broad_compatibility':False}
 assert d['claims']=={'local_projection_only':True,'routing_or_admission_authority':False,'manifold_authority':False}
d=json.loads(F.read_text(encoding='utf-8'));validate(d)
for route,value in [(('source','unchanged_udp_blob'),'0'*40),(('contract','caller_envelope_limits'),False),(('contract','source_preserved'),False),(('delegation','typed_admission'),'new-error'),(('positive','query_id'),20),(('implementation','io'),True),(('implementation','udp_runtime_changed'),True),(('implementation','implicit_limits'),True),(('claims','local_projection_only'),False),(('claims','manifold_authority'),True)]:
 x=copy.deepcopy(d);t=x
 for p in route[:-1]:t=t[p]
 t[route[-1]]=value
 try:validate(x)
 except (AssertionError,KeyError,TypeError):continue
 raise SystemExit(f'damaged fixture accepted: {route}')
assert subprocess.check_output(['git','hash-object','crates/rusty-lsl/src/udp_discovery.rs'],cwd=ROOT).decode().strip()==d['source']['unchanged_udp_blob']
subprocess.run(['cargo','test','-p','rusty-lsl','lslc_004u','--','--nocapture'],cwd=ROOT,check=True)
subprocess.run(['cargo','test','-p','rusty-lsl','--test','public_api','lslc_004u','--','--nocapture'],cwd=ROOT,check=True)
for p,m in {'AGENTS.md':'LSLC-004U','README.md':'LSLC-004U','docs/ARCHITECTURE.md':'LSLC-004U','docs/COMPATIBILITY.md':'LSLC-004U','docs/PROVENANCE.md':'LSLC-004U','docs/VALIDATION.md':'check_lslc_004u.ps1','fixtures/compatibility/README.md':F.name}.items():assert m in (ROOT/p).read_text(encoding='utf-8'),p
print('LSLC-004U typed UDP response projection passed (10 damaged fixtures rejected)')
