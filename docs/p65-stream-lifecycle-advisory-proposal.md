# P65 whole-session lifecycle advisory proposal

## Decision

P65 defines one caller-authored, bounded, typed proposal describing a desired
whole-session Float32 lifecycle and requested P60 timestamp post-processing.
The proposal binds four independent identity surfaces: an opaque proposal
identity, opaque caller/authorship provenance, the exact P64 source/execution
observation identity and extent, and caller-expected discovery-source,
session, stream, and selected receive-order response identity. None of these
values is interpreted as authentication, native identity, admission, or
authority.

The caller explicitly describes whether the session is intended to complete
or stop without completion, whether completion or caller-owned report-free
close is expected, and whether timestamp handling is pass-through, monotonic,
or de-jitter. Stateful post-processing retains the exact caller-supplied
history count and floating-point configuration bits. P65 validates the same
finite P60 configuration envelope but does not construct or invoke the P60
processor.

## Bounds, preconditions, and transactional refusal

Every proposal supplies nonzero maxima for cycles, records, recovery attempts,
queue admissions, and preconditions. These are proposal-envelope facts, not
permission to perform the bounded work. The bound P64 observation budget must
be nonzero and no larger than the proposal cycle maximum; its committed extent
must fit its own budget. Preconditions are an explicit, nonempty, bounded
ordered list.

Construction rejects every zero work maximum, observation/budget expansion,
empty or excessive preconditions, invalid P60 stateful configuration, a
completion request paired with report-free close, and opposing preconditions.
Opposing pairs cover selected-response identity, session start state,
caller-owned activation, and Manifold stream authority. On every refusal the
complete owning draft is returned unchanged; no partially accepted proposal
exists. Successful construction retains the same draft and adds only
`InertAdvisoryOnly`.

P65 deliberately does not compare its caller statements with sockets,
discovery responses, P54 preflight owners, P60 processing state, P63 batches,
or P64 observations. The integrating caller must perform exact comparisons
before construction. This avoids manufacturing native facts or turning an
advisory value into an oracle.

## Authority boundary

The proposal is data-only, default-inert, advisory-only, and non-applying. It
does not execute, connect, transfer, complete, close, recover, retry, schedule,
mutate or admit to queues, choose routes, select or admit peers, grant leases,
authorize, audit, activate, inspect devices, perform background work, or infer
native facts. It defines no Manifold schema or command. Manifold retains stream
authority; existing discovery, selected-connection, session, processing,
recovery, clock, queue, cancellation, cleanup, and activation owners remain
unchanged.

## Facade integration contract

The owned Rust file is intentionally not registered. The integrator may add a
private module declaration and deliberately chosen re-exports in facade-owned
paths. Any facade must:

1. obtain exact P64 `source()`, `execution()`, `budget_cycles()`, and
   `committed_cycle_count()` facts from one already accepted advisory;
2. obtain discovery source, session, stream, and receive-order selection only
   from caller-owned selected-discovery evidence, without normalization or
   inference;
3. translate the caller's exact P60 request into the typed intent, preserving
   stateful floating-point bits;
4. require the caller to supply proposal identity, provenance, every budget,
   lifecycle intent, and ordered preconditions; and
5. return P65 refusal plus the complete draft without invoking any lifecycle,
   processing, recovery, queue, activation, or Manifold operation.

No convenience facade may synthesize defaults, read native state, accept the
proposal, or add an apply/activate path. A standalone `include!` harness is
sufficient for source-local qualification until the integrator owns wiring.
