# P64 requested-processing execution advisory

## Decision

P64 exposes one coherent public composition of the two frozen candidates. It
first produces the exact bounded observation from a completed P63 composition
or from a stopped P63 batch plus the exact committed-prefix supervision. It
then derives an immutable advisory proposal only after caller-owned provenance
matches that observation.

The observation input and provenance independently bind opaque `u128` source
and execution identities, allowing substitution to fail closed. Provenance also
binds the exact finite budget, the committed-cycle count, and each cycle's report count, total
execution extent, first and last completed prefixes, remaining extent, and
current index. The identities remain uninterpreted. Distinct cycles remain
distinct executions.

## Transactional refusal

Observation refusal, identity drift, budget drift, committed-cycle extent
drift, or the first exact per-cycle fact drift returns no proposal and no
partial observation. Observation bounds are explicit and nonzero. Existing
P62 and P63 values remain borrowed and unchanged.

On success the proposal retains the exact observation and exposes its opaque
identity, budget, committed-cycle count, and one descriptive classification:
all committed cycles complete, an incomplete committed cycle is present, or no
cycle committed before refusal. The classification is not a request to act.

## Authority boundary

The composition is CPU/data-only, advisory-only, and default-inert. It defines
no Manifold schema, command, admission, route, lease, revision, authorization,
scheduling, or audit authority. It performs no execution, retry, recovery
policy, queue work, clock work, discovery, socket I/O, storage, device or
Makepad work, background work, or activation. Existing explicit activation
remains default-disabled.
