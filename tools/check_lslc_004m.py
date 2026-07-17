#!/usr/bin/env python3
import copy
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-004m-official-query-production-responder-conformance.json"


def validate(data):
    assert data["schema"] == "rusty.lsl.compatibility.lslc_004m.v1"
    assert data["source"] == {"rusty_lsl_commit":"5a7a70a257e88288951af7afef2c51ee166a0799","rusty_lsl_tree":"973adbc9e4637f36ce8113dc79c305421f382846","responder_production_prefix_sha256":"2adb4cb64fdb7e66c1615b1b0f0cb9742304cc15b0e62c2604cc3b13645a3f4b"}
    assert data["observation"] == {"unit":"LSLC-004N","datagram_sha256":"9fbeba32765d5eb6cc617cce476687fc02ee3bb434c4d4a580a3b28d43e5360c","byte_length":65,"serialized_repeats":2,"stable_scope":"two-repeats-only"}
    assert data["conformance"] == {"test_only":True,"family":"ipv4","group":"239.255.172.215","port":16571,"interface_selection":"caller-explicit-concrete-ipv4","portable_interface":"loopback","query_role_literal":"LSL:shortinfo","session_predicate_literal":"session_id='default'","line_ending":"crlf","terminal_line_ending":True,"reply_port_syntax":{"unsigned_decimal":True,"digit_count":5,"independently_selected":True},"query_id_syntax":{"unsigned_decimal":True,"digit_count":20,"independently_selected":True},"query_byte_length":65,"queries":1,"responses":1,"finite_deadline":True,"cancellation_owner_preserved":True,"request_limit_termination":True,"membership_cleanup":True,"socket_cleanup":True,"result":"pass"}
    assert data["limitations"] == {"observed_private_values_replayed":False,"exact_observed_datagram_replayed":False,"production_runtime_changed":False,"active_nonloopback_executed":False,"official_endpoint_executed":False,"cross_host":False,"cross_platform":False,"quest_or_device":False,"interface_enumeration":False,"default_interface_selection":False,"portable_retry_policy":False}
    assert data["claims"] == {"structural_query_conformance":True,"exact_private_value_conformance":False,"broad_compatibility":False,"runtime_or_activation_widened":False,"manifold_authority":False}


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
for route, value in [
    (("observation","datagram_sha256"), "0"*64), (("observation","byte_length"),64),
    (("conformance","group"),"239.255.172.216"), (("conformance","port"),16572),
    (("conformance","reply_port_syntax","digit_count"),4), (("conformance","query_id_syntax","independently_selected"),False),
    (("conformance","query_byte_length"),64), (("conformance","membership_cleanup"),False),
    (("limitations","exact_observed_datagram_replayed"),True), (("claims","broad_compatibility"),True),
]:
    damaged=copy.deepcopy(data); target=damaged
    for part in route[:-1]: target=target[part]
    target[route[-1]]=value
    try: validate(damaged)
    except (AssertionError,KeyError,TypeError): continue
    raise SystemExit(f"damaged fixture accepted: {route}")
source=(ROOT/"crates/rusty-lsl/src/short_info_discovery_responder_runtime.rs").read_bytes()
production=source.split(b"#[cfg(test)]",1)[0]
assert hashlib.sha256(production).hexdigest()==data["source"]["responder_production_prefix_sha256"]
text=source.decode("utf-8")
for marker in ("lslc_004m_observed_official_query_structure_reaches_unchanged_responder","session_id='default'","10_000_000_000_000_000_001_u64","DOCUMENTED_IPV4_MULTICAST_GROUP","ShortInfoResponderTermination::RequestLimit"):
    assert marker in text
for path,marker in {"AGENTS.md":"LSLC-004M","README.md":"LSLC-004M","docs/COMPATIBILITY.md":"LSLC-004M","docs/PROVENANCE.md":"LSLC-004M","docs/VALIDATION.md":"check_lslc_004m.ps1","fixtures/compatibility/README.md":FIXTURE.name}.items():
    assert marker in (ROOT/path).read_text(encoding="utf-8"),path
print("LSLC-004M official query structure conformance passed (10 damaged fixtures rejected)")
