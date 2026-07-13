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

`rlsl-core-007-timestamped-descriptor-chunk` is the active
source-and-validation unit. It permits only a dependency-free non-empty
composition of one validated descriptor with one of exactly seven existing
timestamped homogeneous chunk representations. The composition retains the
original chunk limits and delegates every caller-ordered sample through
CORE-006, returning the first sample index around its unchanged error. Active
workflow state does not claim actual LSL empty-chunk behavior, clock reads or
algorithms, timestamp rewriting, splitting, merging, rechunking, buffering,
queueing, conversion, encoding, wire or protocol implementation,
compatibility, oracle measurement, or runtime activation.
