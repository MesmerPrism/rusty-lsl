# P65 stream lifecycle advisory composition

## Decision

The canonical P65 facade composes the frozen whole-session observation and
caller-authored advisory candidates without merging their authority. The caller
supplies finite observation limits, exact lifecycle facts, an explicit textual
to opaque identity binding, and the complete advisory draft. Construction first
validates the whole observation, then compares every cross-candidate binding,
then asks the proposal owner to validate its own envelope.

The public root and `runtime` facades expose the same concrete data types and
`compose_morphospace_stream_lifecycle_advisory` function. The successful value
retains exact admitted facts and the unchanged subordinate proposal. It exposes
no apply, activation, or lifecycle operation.

## Transactional binding

The composition requires byte-exact caller, source ID, session ID, and stream
UID provenance; exact proposal caller/authorship and expected opaque discovery,
session, and stream values; the selected receive-order response index; and the
P64 source, execution, finite budget, and committed-cycle extent. The private
observation owner separately rejects identity, peer, lifecycle-order, cycle,
report, record, processing/execution, terminal, close, cleanup, recovery, loss,
or health contradictions. Proposal-envelope refusal retains its complete draft.
No partial successful composition is returned on any refusal.

`NotExposedByOwner`, `NotObserved`, and
`NotReportedByAcceptedOwners` remain exact evidence states. The facade does not
infer cleanup from close, zero loss from absent loss evidence, health from
processing, or recovery from execution.

## Authority boundary

The result is immutable CPU/data-only advisory evidence with the sole
`InertAdvisoryOnly` disposition. It does not discover, select, connect,
transfer, process, complete, close, retry, recover, schedule, mutate or admit
queues, choose routes, admit peers, authorize, lease, audit, activate, inspect
devices, or perform background work. Existing discovery, lifecycle,
post-processing, recovery, queue, cleanup, and activation owners remain
subordinate and unchanged. Manifold retains stream, schema, command, route,
admission, authorization, lease, revision, scheduling, and audit authority.
