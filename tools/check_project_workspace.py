#!/usr/bin/env python3
"""Validate the inert Rusty LSL project-workspace projection."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "morphospace"
UNIT_ID = "rlsl-strm-000-compatibility-baseline"
EMPTY_EFFECTS = {
    "activities",
    "assets",
    "commands",
    "inputs",
    "markers",
    "native_libraries",
    "permissions",
    "queries",
    "routes",
    "scenes",
    "services",
    "shaders",
    "streams",
    "tools",
}


def load_json(relative: str) -> dict[str, Any]:
    """Load one required workspace JSON object."""
    value = json.loads((WORKSPACE / relative).read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"{relative} must contain one JSON object")
    return value


def require(condition: bool, message: str) -> None:
    """Raise a stable validation error when one invariant is false."""
    if not condition:
        raise ValueError(message)


def main() -> int:
    """Validate the bootstrap state without claiming portable acceptance."""
    project = load_json("project.spec.json")
    lock = load_json("feature.lock.json")
    state = load_json("workspace.state.json")
    unit = load_json(f"iteration-units/{UNIT_ID}.json")

    documents = (project, lock, state, unit)
    require(
        all(document.get("project_id") == "rusty-lsl" for document in documents),
        "every workspace document must target rusty-lsl",
    )
    require(project.get("schema", "").endswith("project_spec.v2"), "project spec must be v2")
    require(lock.get("schema", "").endswith("feature_lock.v2"), "feature lock must be v2")
    require(state.get("schema", "").endswith("workspace_state.v2"), "state must be v2")
    require(unit.get("status") == "proposed", "STRM-000 must remain proposed")
    require(state.get("current_unit") is None, "bootstrap must have no current unit")
    require(state.get("next_ready_unit") is None, "bootstrap must have no ready unit")

    composition = project.get("composition", {})
    require(not composition.get("selected_features"), "no feature may be selected")
    require(not composition.get("selected_modules"), "no module may be selected")
    require(not composition.get("allowed_permissions"), "no permission may be allowed")
    require(not project.get("modules"), "no module is registered by the scaffold")

    require(lock.get("default_activation") == "disabled", "lock default must be disabled")
    require(not lock.get("selected_features"), "feature lock must select nothing")
    require(not lock.get("features"), "feature lock must contain no feature")
    fingerprint = lock.get("lock_fingerprint", "")
    require(bool(re.fullmatch(r"[0-9a-f]{64}", fingerprint)), "lock fingerprint is invalid")
    effects = lock.get("effect_union", {})
    require(set(effects) == EMPTY_EFFECTS, "effect-union keys drifted")
    require(all(not value for value in effects.values()), "effect union must remain empty")

    event_lines = [
        line
        for line in (WORKSPACE / "iteration-events.jsonl")
        .read_text(encoding="utf-8")
        .splitlines()
        if line
    ]
    require(len(event_lines) == 1, "bootstrap must contain exactly one event")
    event = json.loads(event_lines[0])
    require(event.get("sequence") == 1, "bootstrap event sequence must be one")
    require(event.get("event_type") == "decision", "bootstrap event must be a decision")
    require(event.get("unit_id") == UNIT_ID, "bootstrap event must name STRM-000")
    require(
        state.get("last_event_id") == event.get("event_id"),
        "workspace state must point to the bootstrap event",
    )

    print("Project-workspace check passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
