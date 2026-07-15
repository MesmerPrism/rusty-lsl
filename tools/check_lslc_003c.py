import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
lock = json.loads((root / "morphospace/feature.lock.json").read_text())
fixture_path = root / "fixtures/compatibility/lslc-003c-lock-bound-runtime-activation-capability.json"
fixture = json.loads(fixture_path.read_text())
source = (root / "crates/rusty-lsl/src/runtime_activation.rs").read_text()

assert fixture["schema"] == "rusty.lsl.lslc_003c.activation_fixture.v1"
assert fixture["accepted_lock"] == {"revision": lock["revision"], "fingerprint": lock["lock_fingerprint"]}
assert lock["default_activation"] == "disabled"
assert lock["activation_rule"] == "selected-lock-and-runtime-input"
assert f'"{lock["lock_fingerprint"]}"' in source
assert f"ACCEPTED_FEATURE_LOCK_REVISION: u64 = {lock['revision']};" in source

features = {feature["module_id"]: feature for feature in lock["features"]}
assert set(features) == set(lock["selected_features"])
for module_id, feature in features.items():
    assert f'"{module_id}"' in source
    assert f'"{feature["activation"]["effective_marker"]}"' in source
    for dependency in feature["dependencies"]:
        assert f'"{dependency}"' in source

assert fixture["damaged"] == ["stale-lock-fingerprint", "unknown-module", "duplicate-module", "effective-marker-mismatch", "missing-declared-dependency", "empty-consumer-id", "oversized-consumer-id"]
assert fixture["nonclaims"] == ["runtime effect execution", "existing runtime facade composition", "device behavior", "adapter readiness", "Manifold authority"]
assert "pub struct RuntimeModuleCapability" in source and "_private: ()" in source
assert "pub struct RuntimeActivationReceipt" in source
assert "pub enum RuntimeActivationOutcome" in source and "Accepted" in source
assert "pub fn admit_runtime_activation" in source
assert "std::net" not in source and "std::process" not in source and "unsafe" not in source
public_text = fixture_path.read_text().lower()
assert not any(token in public_text for token in ["127.0.0.1", "private-"])
print("LSLC-003C lock-bound runtime activation evidence passed")
