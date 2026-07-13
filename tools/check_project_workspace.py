#!/usr/bin/env python3
"""Validate the inert Rusty LSL project-workspace lifecycle projection."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "morphospace"
PROJECT_ID = "rusty-lsl"
UNIT_STATUSES = {"proposed", "ready", "active", "validating", "blocked", "accepted"}
IN_FLIGHT_STATUSES = {"active", "validating"}
EVENT_TYPES = {
    "state-transition",
    "decision",
    "extraction",
    "validation",
    "commit",
    "push",
    "promotion",
    "blocker",
}
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


def load_json(path: Path) -> dict[str, Any]:
    """Load one required workspace JSON object."""
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"{path.relative_to(WORKSPACE)} must contain one JSON object")
    return value


def require(condition: bool, message: str) -> None:
    """Raise a stable validation error when one invariant is false."""
    if not condition:
        raise ValueError(message)


def load_units() -> dict[str, dict[str, Any]]:
    """Load every declared unit without assuming a bootstrap unit identity."""
    units: dict[str, dict[str, Any]] = {}
    for path in sorted((WORKSPACE / "iteration-units").glob("*.json")):
        unit = load_json(path)
        unit_id = unit.get("unit_id")
        require(isinstance(unit_id, str) and unit_id, f"{path.name} has no unit_id")
        require(unit_id not in units, f"duplicate unit_id '{unit_id}'")
        require(unit.get("project_id") == PROJECT_ID, f"unit '{unit_id}' targets another project")
        require(unit.get("status") in UNIT_STATUSES, f"unit '{unit_id}' has an invalid status")
        require(isinstance(unit.get("acceptance"), list) and unit["acceptance"], f"unit '{unit_id}' has no acceptance gates")
        require(isinstance(unit.get("validation"), list) and unit["validation"], f"unit '{unit_id}' has no validation profiles")
        surfaces = unit.get("instruction_surfaces")
        require(isinstance(surfaces, list), f"unit '{unit_id}' has invalid instruction surfaces")
        for surface in surfaces:
            require(surface.get("status") in {"planned", "complete"}, f"unit '{unit_id}' has invalid instruction status")
        units[unit_id] = unit
    require(bool(units), "workspace must contain at least one iteration unit")
    for unit_id, unit in units.items():
        for prerequisite in unit.get("prerequisites", []):
            require(prerequisite in units, f"unit '{unit_id}' has missing prerequisite '{prerequisite}'")
    return units


def load_events(units: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    """Validate the append-only event projection used by local state."""
    lines = [
        line
        for line in (WORKSPACE / "iteration-events.jsonl")
        .read_text(encoding="utf-8")
        .splitlines()
        if line
    ]
    require(bool(lines), "workspace event log must not be empty")
    events: list[dict[str, Any]] = []
    event_ids: set[str] = set()
    previous_sequence = 0
    for line in lines:
        event = json.loads(line)
        require(isinstance(event, dict), "each event line must contain one object")
        event_id = event.get("event_id")
        require(isinstance(event_id, str) and event_id, "event has no identity")
        require(event_id not in event_ids, f"duplicate event_id '{event_id}'")
        require(event.get("schema") in {
            "rusty.morphospace.workflow.iteration_event.v1",
            "rusty.morphospace.workflow.iteration_event.v2",
        }, f"event '{event_id}' has an invalid schema")
        require(event.get("project_id") == PROJECT_ID, f"event '{event_id}' targets another project")
        require(event.get("event_type") in EVENT_TYPES, f"event '{event_id}' has an invalid type")
        sequence = event.get("sequence")
        require(isinstance(sequence, int) and sequence > previous_sequence, "event sequences must increase")
        unit_id = event.get("unit_id")
        require(unit_id is None or unit_id in units, f"event '{event_id}' references a missing unit")
        require(isinstance(event.get("receipts"), list), f"event '{event_id}' has invalid receipts")
        events.append(event)
        event_ids.add(event_id)
        previous_sequence = sequence
    return events


def effective_in_flight(
    units: dict[str, dict[str, Any]], events: list[dict[str, Any]]
) -> set[str]:
    """Account for the portable workflow's additive supersession projection."""
    superseded: set[str] = set()
    pattern = re.compile(r"^(?P<old>[a-z0-9][a-z0-9-]{1,63})-superseded-by-(?P<new>[a-z0-9][a-z0-9-]{1,63})$")
    for event in events:
        match = pattern.fullmatch(str(event.get("event_id", "")))
        if not match:
            continue
        old_id = match.group("old")
        new_id = match.group("new")
        require(event.get("event_type") == "state-transition", "supersession must be a state transition")
        require(event.get("unit_id") == old_id, "supersession must target the old unit")
        require(old_id in units and new_id in units, "supersession references a missing unit")
        require(units[old_id].get("status") in IN_FLIGHT_STATUSES, "superseded unit is not in flight")
        require(units[new_id].get("status") in IN_FLIGHT_STATUSES | {"accepted"}, "replacement unit is invalid")
        superseded.add(old_id)
    return {
        unit_id
        for unit_id, unit in units.items()
        if unit.get("status") in IN_FLIGHT_STATUSES and unit_id not in superseded
    }


def validate_lifecycle(
    state: dict[str, Any], units: dict[str, dict[str, Any]], events: list[dict[str, Any]]
) -> None:
    """Validate proposed through accepted lifecycle projections without freezing one stage."""
    in_flight = effective_in_flight(units, events)
    require(len(in_flight) <= 1, "workspace has more than one effective in-flight unit")
    current = state.get("current_unit")
    if current is None:
        require(not in_flight, "workspace state omits an in-flight unit")
    else:
        require(current in units, "workspace state references a missing current unit")
        require(units[current].get("status") in IN_FLIGHT_STATUSES, "current unit is not active or validating")
        require(in_flight == {current}, "current unit does not match the effective in-flight unit")

    next_ready = state.get("next_ready_unit")
    if next_ready is not None:
        require(next_ready in units, "workspace state references a missing next-ready unit")
        require(units[next_ready].get("status") == "ready", "next-ready unit is not ready")

    event_ids = {event["event_id"] for event in events}
    require(state.get("last_event_id") in event_ids, "workspace state references a missing last event")
    require(state.get("last_event_id") == events[-1]["event_id"], "workspace state does not reference the event tail")


def validate_inert_lock(project: dict[str, Any], lock: dict[str, Any], state: dict[str, Any]) -> None:
    """Keep the scaffold's composition and effect closure closed in every lifecycle stage."""
    composition = project.get("composition", {})
    require(not composition.get("selected_features"), "no feature may be selected")
    require(not composition.get("selected_modules"), "no module may be selected")
    require(not composition.get("allowed_permissions"), "no permission may be allowed")
    require(not project.get("modules"), "no module may be registered by the scaffold")

    require(lock.get("default_activation") == "disabled", "lock default must be disabled")
    require(not lock.get("selected_features"), "feature lock must select nothing")
    require(not lock.get("features"), "feature lock must contain no feature")
    fingerprint = lock.get("lock_fingerprint", "")
    require(bool(re.fullmatch(r"[0-9a-f]{64}", fingerprint)), "lock fingerprint is invalid")
    effects = lock.get("effect_union", {})
    require(set(effects) == EMPTY_EFFECTS, "effect-union keys drifted")
    require(all(not value for value in effects.values()), "effect union must remain empty")

    registry = state.get("module_registry", {})
    require(registry.get("lock_revision") == lock.get("revision"), "module registry lock revision drifted")
    require(registry.get("lock_fingerprint") == fingerprint, "module registry fingerprint drifted")
    require(not registry.get("modules"), "module registry must remain empty")


def main() -> int:
    """Validate the legitimate lifecycle while retaining a closed feature lock."""
    project = load_json(WORKSPACE / "project.spec.json")
    lock = load_json(WORKSPACE / "feature.lock.json")
    state = load_json(WORKSPACE / "workspace.state.json")
    units = load_units()
    events = load_events(units)

    documents = (project, lock, state)
    require(all(document.get("project_id") == PROJECT_ID for document in documents), "workspace project IDs drifted")
    require(project.get("schema", "").endswith("project_spec.v2"), "project spec must be v2")
    require(lock.get("schema", "").endswith("feature_lock.v2"), "feature lock must be v2")
    require(state.get("schema", "").endswith("workspace_state.v2"), "state must be v2")
    require(isinstance(state.get("plan_revision"), int) and state["plan_revision"] >= 1, "plan revision is invalid")

    validate_lifecycle(state, units, events)
    validate_inert_lock(project, lock, state)
    print("Project-workspace lifecycle and inert-lock checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
