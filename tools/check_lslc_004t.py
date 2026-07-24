#!/usr/bin/env python3
import copy,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; FIXTURE=ROOT/'fixtures/compatibility/lslc-004t-requester-typed-observation-conformance.json'
def validate(d):
 assert d['schema']=='rusty.lsl.requester_typed_observation_conformance.v1'
 assert d['source']=={'claimed_head':'71def0ca40ad5461002cc2489dcd21ff53931360','claimed_tree':'3df30844f4f8f03f9ca29636abe4134e1ec1c9e1','accepted_udp_blob':'c635879b43e517a5172dd08558a7f3c295e15d93','production_prefix_sha256':'41c2393adb561669aa824e3635ae70e85b3b412892a38100fc835d3bfc8b4263','production_prefix_bytes':12473}
 assert d['scope']=={'test_only':True,'family':'ipv4','destination':'caller-explicit-loopback-peer','responses':1,'response_bytes':732,'document_bytes':711,'query_id_digits':19,'maximum_responses':1,'deadline_milliseconds':1000}
 assert d['composition']=={'udp_response_admitted':True,'retained_bytes_reparsed':True,'existing_typed_observation_admitted':True,'query_id_preserved':True,'stream_name_preserved':True,'channel_count_preserved':True,'source_address_preserved':True,'response_limit_termination':True,'socket_cleanup_rebind':True}
 assert d['independent_values']=={'all_values_independently_authored':True,'valid_nominal_rate_selected_independently':True,'exact_document_length_preserved':True,'private_oracle_values_replayed':False,'official_bytes_used':False}
 assert d['damaged']=={'noncanonical_channel_count':'existing-invalid-channel-count','delegation_unchanged':True}
 assert d['preserved_owner_behavior']=={'deadline':True,'cancellation':True,'envelope_and_structure_damage':True}
 assert d['limitations']=={'production_changed':False,'new_parser_or_admission_semantics':False,'official_byte_stability':False,'network_policy':False,'device_scope':False,'broad_compatibility':False}
 assert d['claims']=={'typed_composition_only':True,'runtime_or_activation_widened':False,'routing_or_admission_authority':False,'manifold_authority':False}
d=json.loads(FIXTURE.read_text(encoding='utf-8'));validate(d)
for route,value in [(('source','accepted_udp_blob'),'0'*40),(('scope','response_bytes'),733),(('scope','maximum_responses'),2),(('composition','query_id_preserved'),False),(('independent_values','private_oracle_values_replayed'),True),(('damaged','delegation_unchanged'),False),(('preserved_owner_behavior','cancellation'),False),(('limitations','new_parser_or_admission_semantics'),True),(('claims','typed_composition_only'),False),(('claims','manifold_authority'),True)]:
 x=copy.deepcopy(d);t=x
 for p in route[:-1]:t=t[p]
 t[route[-1]]=value
 try:validate(x)
 except (AssertionError,KeyError,TypeError):continue
 raise SystemExit(f'damaged fixture accepted: {route}')
marker=b'#[cfg(test)]\nmod tests {'
current=(ROOT/'crates/rusty-lsl/src/udp_discovery.rs').read_bytes().partition(marker)[0]
accepted=subprocess.check_output(['git','cat-file','blob',d['source']['accepted_udp_blob']],cwd=ROOT).partition(marker)[0]
assert current==accepted
assert len(current)==d['source']['production_prefix_bytes'] and hashlib.sha256(current).hexdigest()==d['source']['production_prefix_sha256']
subprocess.run(['cargo','test','-p','rusty-lsl','lslc_004t','--','--nocapture'],cwd=ROOT,check=True)
for p,m in {'AGENTS.md':'LSLC-004T','docs/COMPATIBILITY.md':'LSLC-004T','docs/PROVENANCE.md':'LSLC-004T','docs/VALIDATION.md':'check_lslc_004t.ps1','fixtures/compatibility/README.md':FIXTURE.name}.items():assert m in (ROOT/p).read_text(encoding='utf-8'),p
print('LSLC-004T requester typed-observation conformance passed (10 damaged fixtures rejected)')
