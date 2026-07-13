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

`rlsl-core-008-stream-definition-composition` permits only an infallible
dependency-free composition that directly moves one already validated stream
descriptor and one already validated generic metadata tree into private
accepted state.
Borrowed and consuming access preserves both components exactly. The
composition adds no allocation, clone, error or limit family, cross-component
validation, XML or `desc`-root interpretation, runtime identity, discovery,
networking, clocks, buffering, provider, adapter, authority, wire or protocol
implementation, compatibility, oracle measurement, or runtime activation.
