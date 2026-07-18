#!/usr/bin/env python3
import copy,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];F=ROOT/'fixtures/compatibility/lslc-004v-typed-udp-discovery-run.json'
def validate(d):
 assert d['schema']=='rusty.lsl.typed_udp_discovery_run.v1'
 assert d['source']=={'claimed_head':'5f699d56716fdf1a112f075c81f50596ae46dede','claimed_tree':'c7fc822a348bfdad6ef1b6951726bafacee27f3e','unchanged_udp_blob':'e4df037ff44263207532764040ae07b8e1b65e36','unchanged_projection_blob':'762f4aa3b40b417659d5a817c0e46b1dce4b0125'}
 assert d['contract']=={'activation':'existing-udp-capability','configuration':'existing-caller-explicit-udp-config','caller_envelope_limits':True,'caller_admission_limits':True,'output':'owned-typed-discovery-run','local_address_preserved':True,'termination_preserved':True,'receive_order_preserved':True,'source_preserved':True}
 assert d['delegation']=={'udp':'existing-error','typed_response':'existing-error-plus-zero-based-index','allocation':'requested-response-capacity'}
 assert d['positive']=={'response_count':1,'query_id':19,'stream_name':'typed-run','channel_count':1,'termination':'response-limit','cleanup':'socket-rebind-by-existing-owner'}
 assert d['negative']=={'noncanonical_channel_count':'response-index-zero-invalid-channel-count','pre_io_cancellation':'cancelled-empty-run'}
 assert d['implementation']=={'dependency_free':True,'unsafe':False,'new_udp_policy':False,'new_parsing_semantics':False,'filtering':False,'deduplication':False,'retry_policy':False,'activation_changed':False}
 assert d['lanes']=={'official_oracle':'unchanged-separate','sanitized_derivation':'unchanged-separate','rust_host':'synthetic-loopback-only','android_java':'not-executed','rust_on_quest':'not-executed'}
 assert d['claims']=={'bounded_typed_run_only':True,'broad_compatibility':False,'routing_or_admission_authority':False,'manifold_authority':False}
d=json.loads(F.read_text(encoding='utf-8'));validate(d)
for route,value in [(('source','unchanged_udp_blob'),'0'*40),(('source','unchanged_projection_blob'),'0'*40),(('contract','caller_admission_limits'),False),(('contract','receive_order_preserved'),False),(('delegation','typed_response'),'unindexed'),(('positive','termination'),'deadline'),(('negative','pre_io_cancellation'),'io-opened'),(('implementation','filtering'),True),(('lanes','rust_on_quest'),'passed'),(('claims','manifold_authority'),True)]:
 x=copy.deepcopy(d);t=x
 for p in route[:-1]:t=t[p]
 t[route[-1]]=value
 try:validate(x)
 except (AssertionError,KeyError,TypeError):continue
 raise SystemExit(f'damaged fixture accepted: {route}')
assert subprocess.check_output(['git','hash-object','crates/rusty-lsl/src/udp_discovery.rs'],cwd=ROOT).decode().strip()==d['source']['unchanged_udp_blob']
assert subprocess.check_output(['git','hash-object','crates/rusty-lsl/src/typed_udp_discovery_response.rs'],cwd=ROOT).decode().strip()==d['source']['unchanged_projection_blob']
subprocess.run(['cargo','test','-p','rusty-lsl','lslc_004v','--','--nocapture'],cwd=ROOT,check=True)
subprocess.run(['cargo','test','-p','rusty-lsl','--test','public_api','lslc_004v','--','--nocapture'],cwd=ROOT,check=True)
for p,m in {'AGENTS.md':'LSLC-004V','README.md':'LSLC-004V','docs/ARCHITECTURE.md':'LSLC-004V','docs/COMPATIBILITY.md':'LSLC-004V','docs/PROVENANCE.md':'LSLC-004V','docs/VALIDATION.md':'check_lslc_004v.ps1','fixtures/compatibility/README.md':F.name}.items():assert m in (ROOT/p).read_text(encoding='utf-8'),p
print('LSLC-004V typed UDP discovery run passed (10 damaged fixtures rejected)')
