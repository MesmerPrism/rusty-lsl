import hashlib
import json
import re
from pathlib import Path, PurePosixPath

root = Path(__file__).resolve().parents[1]
lock = json.loads((root / "morphospace/feature.lock.json").read_text())
fixture = json.loads((root / "fixtures/compatibility/lslc-003f-dependency-closed-bounded-record-transport-correction.json").read_text())
activation_fixture = json.loads((root / "fixtures/compatibility/lslc-003c-lock-bound-runtime-activation-capability.json").read_text())
runtime = (root / "crates/rusty-lsl/src/runtime_activation.rs").read_text()
blocked = json.loads((root / "morphospace/iteration-units/rlsl-lslc-003e-bounded-fixed-record-transport-core.json").read_text())
events = [json.loads(line) for line in (root / "morphospace/iteration-events.jsonl").read_text().splitlines() if line]
assert fixture["schema"] == "rusty.lsl.lslc_003f.dependency_closed_transport_correction.v1"
assert fixture["lock"] == {"revision": lock["revision"], "fingerprint": lock["lock_fingerprint"]}
assert activation_fixture["accepted_lock"] == fixture["lock"]
assert f'"{lock["lock_fingerprint"]}"' in runtime
assert f"ACCEPTED_FEATURE_LOCK_REVISION: u64 = {lock['revision']}" in runtime
features = {item["feature_id"]: item for item in lock["features"]}
for feature_id in fixture["changed_features"]:
    item = features[feature_id]
    descriptor_path = item["descriptor"]["path"]
    portable = PurePosixPath(descriptor_path)
    assert "\\" not in descriptor_path and not portable.is_absolute()
    assert not re.match(r"^[A-Za-z]:/", descriptor_path)
    assert all(part not in ("", ".", "..") for part in portable.parts)
    assert hashlib.sha256((root / item["descriptor"]["source_path"]).read_bytes()).hexdigest() == item["descriptor"]["source_sha256"]
assert blocked["status"] == "blocked"
assert any(event["event_id"].endswith("validation-blocked-0364") and event["unit_id"] == blocked["unit_id"] for event in events)
assert fixture["nonclaims"][-1] == "Manifold authority"
print("LSLC-003F dependency-closed transport correction evidence passed")
