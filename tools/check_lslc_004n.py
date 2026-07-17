#!/usr/bin/env python3
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures/compatibility/lslc-004n-official-query-datagram-observation.json"


def validate(data):
    assert data["schema"] == "rusty.lsl.official_query_datagram_observation.v1"
    assert data["source"] == {"rusty_lsl_commit":"682d8b4cc1497c24cb497d10d361726033700639","rusty_lsl_tree":"df0b0160ea996c04a5f027f5545b35800fe0ac5e"}
    assert data["official"] == {"pylsl":"1.18.2","liblsl":117,"protocol":110}
    assert data["scope"] == {"family":"ipv4","group":"239.255.172.215","port":16571,"interface_selection":"caller-explicit-active-private-ipv4","platform_class":"single-windows-desktop-host","serialized_repeats":2,"captures_per_repeat":1,"maximum_deadline_milliseconds":3000,"response_sent":False}
    expected = {"sha256":"9fbeba32765d5eb6cc617cce476687fc02ee3bb434c4d4a580a3b28d43e5360c","byte_length":65}
    assert data["datagrams"] == [expected, expected]
    assert data["structure"] == {"encoding":"utf-8-ascii-subset","line_count":3,"line_ending":"crlf","terminal_line_ending":True,"line_byte_lengths":[13,20,26],"line_1":{"literal":"LSL:shortinfo"},"line_2":{"field":"session_id","operator":"equals","quoting":"single","literal_value":"default"},"line_3":{"separator":"single-space","tokens":[{"role":"reply-port","syntax":"unsigned-decimal","digit_count":5,"value_published":False},{"role":"query-id","syntax":"unsigned-decimal","digit_count":20,"value_published":False}]}}
    assert data["repeat_relationship"] == {"complete_datagrams_equal":True,"hashes_equal":True,"lengths_equal":True,"structures_equal":True,"claim_scope":"two-serialized-repeats-only"}
    assert data["results"] == {"capture_before_response":["pass","pass"],"finite_resolver_completion":["pass","pass"],"membership_cleanup":["pass","pass"],"socket_cleanup":["pass","pass"],"elapsed_milliseconds":[781.0,765.0]}
    private = data["private_artifacts"]
    assert private == {
        "driver_source_sha256":"f7672a0d9fb5d2ef71a32fa5ef49b17b97ba1ba87bd5a8696b056bc5af903d81",
        "attempt_record_sha256":["3517aa40164d5d7ab3169a2d16c348e921f2b8b1c293e4c3f9a5f65d92549047","27014f29fd53012a6fe0b9710e49f580ad8e4c0fc291917959a5456d7a39d7a0"],
        "capsule_sha256":"030c51fa72471e8d42770722ec4e9789fdc7a741734bad42d4460a731b79b63e",
        "failed_attempts_preserved":2,
        "published":False,
    }
    assert data["limitations"] == {"single_host":True,"single_platform":True,"two_repeat_stability_only":True,"reply_port_and_query_id_values_private":True,"production_runtime_executed":False,"official_outlet":False,"cross_host":False,"quest_or_device":False,"portable_interface_or_retry_policy":False}
    assert data["claims"] == {"observation_only":True,"addressable_exact_datagram_hashes":True,"raw_datagrams_published":False,"production_bytes_changed":False,"runtime_or_activation_widened":False,"broad_compatibility":False,"manifold_authority":False}
    encoded = json.dumps(data, sort_keys=True)
    forbidden = ("192.168.", "10.0.", "Wi-Fi", "InterfaceIndex", "16520821047081189859", "private-host", "device_serial", "serial_number")
    assert not any(value in encoded for value in forbidden)


data = json.loads(FIXTURE.read_text(encoding="utf-8"))
validate(data)
mutations = [
    (("datagrams", 0, "sha256"), "0" * 64),
    (("datagrams", 1, "byte_length"), 64),
    (("structure", "line_count"), 4),
    (("structure", "line_3", "tokens", 1, "digit_count"), 19),
    (("official", "pylsl"), "1.18.1"),
    (("scope", "serialized_repeats"), 1),
    (("repeat_relationship", "claim_scope"), "portable-stability"),
    (("private_artifacts", "capsule_sha256"), "f" * 64),
    (("limitations", "two_repeat_stability_only"), False),
    (("claims", "raw_datagrams_published"), True),
]
for route, value in mutations:
    damaged = copy.deepcopy(data)
    target = damaged
    for part in route[:-1]:
        target = target[part]
    target[route[-1]] = value
    try:
        validate(damaged)
    except (AssertionError, KeyError, TypeError):
        continue
    raise SystemExit(f"damaged fixture accepted: {route}")
for path, marker in {
    "AGENTS.md":"LSLC-004N", "README.md":"LSLC-004N", "docs/COMPATIBILITY.md":"LSLC-004N",
    "docs/PROVENANCE.md":"LSLC-004N", "docs/VALIDATION.md":"check_lslc_004n.ps1",
    "fixtures/compatibility/README.md":FIXTURE.name,
}.items():
    assert marker in (ROOT / path).read_text(encoding="utf-8"), path
print("LSLC-004N official query datagram observation passed (10 damaged fixtures rejected)")
