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

`rlsl-core-006-timestamped-descriptor-sample` is the active
source-and-validation unit. It permits only a dependency-free composition of
one validated descriptor with one of exactly seven existing timestamped
homogeneous sample representations. The composition delegates format,
channel-count, and String bounds to CORE-005 and retains raw and optional
derived timestamp evidence unchanged. Active workflow state does not claim
clock reads or algorithms, timestamp rewriting, buffering, conversion,
encoding, wire or protocol implementation, compatibility, oracle measurement,
or runtime activation.
