import hashlib
import json
import re
from pathlib import Path, PurePosixPath

root = Path(__file__).resolve().parents[1]
lock = json.loads((root / "morphospace/feature.lock.json").read_text())
fixture = json.loads((root / "fixtures/compatibility/lslc-003d-dependency-closed-runtime-activation-composition.json").read_text())
assert fixture["schema"] == "rusty.lsl.lslc_003d.dependency_closed_activation.v1"
features = {item["module_id"]: item for item in lock["features"]}
assert {name: item["dependencies"] for name, item in features.items()} == fixture["composition"]
for name, item in features.items():
    descriptor_path = item["descriptor"]["path"]
    portable_path = PurePosixPath(descriptor_path)
    assert "\\" not in descriptor_path
    assert not portable_path.is_absolute()
    assert not re.match(r"^[A-Za-z]:/", descriptor_path)
    assert all(part not in ("", ".", "..") for part in portable_path.parts)
    assert (root / "morphospace" / portable_path).resolve().is_relative_to(
        (root / "morphospace").resolve()
    )
    source_path = root / item["descriptor"]["source_path"]
    assert hashlib.sha256(source_path.read_bytes()).hexdigest() == item["descriptor"]["source_sha256"]
    source = source_path.read_text()
    assert "RuntimeModuleCapability" in source
    assert "feature: &str" not in source and "marker: &str" not in source
assert fixture["runtime_behavior"] == "unchanged"
assert fixture["nonclaims"][-1] == "Manifold authority"
print("LSLC-003D dependency-closed activation evidence passed")
