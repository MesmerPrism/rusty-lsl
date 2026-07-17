#!/usr/bin/env python3
import copy, hashlib, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/"fixtures/compatibility/lslc-004s-official-response-production-requester-conformance.json"
def validate(d):
    assert d["schema"]=="rusty.lsl.official_response_production_requester_conformance.v1"
    assert d["source"]=={"claimed_head":"ad125d09caca98ca64b4a1ff666c86e380f7d2f4","claimed_tree":"08cba8f6215ebc36e1f5b01e27f302842d5963da","accepted_production_blob":"f13c039180139b7f9879340e0ee7e98a6f440eba","accepted_production_sha256":"d1ed62275916b0e5ada9b332b4c2a4af122fd0c84d09cc6f672d8e1138975619"}
    assert d["evidence_basis"]=={"fixture":"lslc-004r-official-outlet-response-datagram-observation.json","response_byte_length":732,"document_byte_length":711,"envelope_query_id_digit_count":19,"line_delimiter":"crlf","document_encoding":"utf-8","xml_declaration":True,"root":"info","ordered_roles":["name","type","channel_count","channel_format","source_id","nominal_srate","version","created_at","uid","session_id","hostname","v4address","v4data_port","v4service_port","v6address","v6data_port","v6service_port","desc"]}
    assert d["independent_construction"]=={"all_values_independently_authored":True,"private_oracle_values_replayed":False,"official_response_hash_matched":False,"official_response_bytes_in_test":False,"query_id_value_private_or_observed":False}
    assert d["scope"]=={"family":"ipv4","destination_selection":"caller-explicit-loopback-test-peer","queries":1,"responses":1,"maximum_response_bytes":732,"maximum_document_bytes":711,"maximum_responses":1,"total_deadline_milliseconds":1000,"receive_slice_milliseconds":10}
    assert d["positive"]=={"production_requester_admission":"pass","query_id_correlation":"pass","response_bytes_retained":"pass","source_retained":"pass","response_limit_termination":"pass","deadline_termination":"pass","pre_send_cancellation":"pass","socket_cleanup_rebind":"pass"}
    assert d["damaged"]=={"invalid_envelope_delimiter":"reject","one_past_response_length":"reject","first_role_name_drift":"reject","role_order_drift":"reject","later_role_name_drift":"reject","missing_terminal_lf":"reject"}
    assert d["limitations"]=={"test_only":True,"loopback_only":True,"official_endpoint_executed":False,"official_byte_or_value_stability":False,"production_bytes_changed":False,"interface_policy":False,"other_groups_or_families":False,"retry_policy":False,"device_scope":False,"broad_compatibility":False}
    assert d["claims"]=={"structural_conformance_only":True,"runtime_or_activation_widened":False,"routing_or_admission_authority":False,"manifold_authority":False}
d=json.loads(FIXTURE.read_text(encoding="utf-8")); validate(d)
for route,value in [(('source','accepted_production_blob'),'0'*40),(('evidence_basis','response_byte_length'),731),(('evidence_basis','ordered_roles',0),'type'),(('independent_construction','private_oracle_values_replayed'),True),(('scope','maximum_document_bytes'),712),(('positive','deadline_termination'),'skip'),(('damaged','missing_terminal_lf'),'accept'),(('limitations','official_byte_or_value_stability'),True),(('claims','structural_conformance_only'),False),(('claims','manifold_authority'),True)]:
    x=copy.deepcopy(d); t=x
    for part in route[:-1]: t=t[part]
    t[route[-1]]=value
    try: validate(x)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit(f"damaged fixture accepted: {route}")
source=(ROOT/'crates/rusty-lsl/src/udp_discovery.rs').read_bytes(); marker=b'    fn lslc_004s_independent_document() -> String {'
prefix,sep,_=source.partition(marker); assert sep
accepted=subprocess.check_output(['git','cat-file','blob',d['source']['accepted_production_blob']],cwd=ROOT)
assert prefix.rstrip()+b'\n}\n'==accepted
assert hashlib.sha256(accepted).hexdigest()==d['source']['accepted_production_sha256']
subprocess.run(['cargo','test','-p','rusty-lsl','lslc_004s','--','--nocapture'],cwd=ROOT,check=True)
for path,text in {'AGENTS.md':'LSLC-004S','docs/COMPATIBILITY.md':'LSLC-004S','docs/PROVENANCE.md':'LSLC-004S','docs/VALIDATION.md':'check_lslc_004s.ps1','fixtures/compatibility/README.md':FIXTURE.name}.items(): assert text in (ROOT/path).read_text(encoding='utf-8'),path
print('LSLC-004S response structure conformance passed (10 damaged fixtures rejected)')
