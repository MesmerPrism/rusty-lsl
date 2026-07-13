# Rusty LSL project workflow

This public, project-local workspace is the control surface for bounded Rusty
LSL iteration. It does not grant runtime authority or indicate compatibility.
The closed feature lock selects no feature or module and has an empty effect
union.

Resume in this order:

1. `project.spec.json`
2. `feature.lock.json`
3. `workspace.state.json`
4. the current iteration unit, if one is named
5. only the event tail and receipts referenced by state

`rlsl-strm-000-compatibility-baseline` is proposed, not ready or active. It
must pass the portable readiness transition before implementation begins.
