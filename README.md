# Rusty LSL

Rusty LSL is an independently authored, pure-Rust implementation of bounded
Lab Streaming Layer compatibility surfaces for Rusty Morphospace.

## Status

The source candidate is integrated on public `main`. It remains version
`0.0.0`, `publish = false`, and is neither a stable API nor a tagged,
registry-published release. Integration is a source milestone, not release or
acceptance evidence.

Runtime capability selection and activation remain separate. The project lock
selects candidate capability families, but activation is disabled by default
and requires the accepted lock plus explicit caller runtime input. No default
Cargo feature activates runtime behavior; the complete activation model remains
default-disabled.

## Scope and authority

The crate provides bounded contracts and explicitly activated candidate
runtime compositions for stream descriptions and metadata, discovery,
connection and record transfer, timestamps, clock correction, queues,
recovery, and typed Morphospace observations and proposals. Exact supported
claims and their evidence limits live in the documents below.

Rusty LSL does not own Manifold stream admission, routing, leases,
authorization, revisions, or audit. Source integration and passing local tests
do not by themselves prove stable API, broad ecosystem interoperability,
portable non-loopback behavior, host-to-Quest qualification, or release
readiness.

Project-owned source is licensed AGPL-3.0-or-later. Official liblsl is used
only as a pinned black-box compatibility oracle, never as an implementation
template. rLSL source is not an implementation input.

## Float32 sender state and measurement

An accepted bounded Float32 outlet session allocates one exact-size encoding
buffer before caller-record transfer and reuses it for initialization and every
declared record. Its bounded writer also retains the current socket write
timeout and changes it only when the effective timeout changes; cancellation
and a fresh total deadline are still checked for every record.

`python ./tools/run_float32_sender_benchmark.py` runs a descriptive release-mode
loopback microbenchmark and emits one JSON record with the revision, dirty-state
flag, host, dimensions, median, and p95. It is not a performance gate or a claim
of persistent outlets, chunk submission, background discovery, multi-consumer
fanout, application latency, or liblsl parity.

## Project documents

- [Production Roadmap](docs/LSL-PRODUCTION-ROADMAP.md) — completion criteria,
  remaining release boundary, and priority order.
- [Architecture](docs/ARCHITECTURE.md) — component and authority boundaries.
- [Compatibility](docs/COMPATIBILITY.md) — supported and unresolved
  interoperability claims.
- [Stable public API](docs/STABLE_PUBLIC_API.md) — current stability contract
  and limitations.
- [Validation](docs/VALIDATION.md) and
  [validation policy](tools/validation-policy.json) — current gates and what
  they prove.
- [Provenance](docs/PROVENANCE.md), [corpus](docs/CORPUS.md), and
  [oracle policy](docs/ORACLE.md) — independent authorship and evidence
  discipline.
- [Project workflow](morphospace/README.md) — bounded work-unit lifecycle and
  inert-by-default composition state.
- [Worktree classification](docs/WORKTREE-CLASSIFICATION.md) — non-mutating
  snapshot of registered worktree overlays at this integration boundary.

## Preserved history

- [README at P68 integration](docs/history/README-AT-P68-INTEGRATION.md)
  preserves the pre-compaction first-hop chronology.
- [LSLC work-unit history](docs/history/LSLC-WORK-UNIT-HISTORY.md) and
  [README through accepted LSLC-003L](docs/history/README-THROUGH-LSLC-003L.md)
  retain earlier accepted-unit context.

Historical descriptions and validation receipts retain their original scope;
they are not current release or interoperability claims.

Current focused evidence routes: LSLC-003O, LSLC-003P, LSLC-003Q, LSLC-003S,
LSLC-003T, LSLC-003U, LSLC-003V, LSLC-003W, LSLC-003X, LSLC-003Y, LSLC-003Z,
LSLC-004A, LSLC-004B, LSLC-004C, LSLC-004D, LSLC-004E, LSLC-004F, LSLC-004G,
LSLC-004H, LSLC-004J, LSLC-004K, LSLC-004M, LSLC-004N, LSLC-004O, LSLC-004P,
LSLC-004R, LSLC-004S, LSLC-004T, LSLC-004U, and LSLC-004V. These identifiers
route to the compatibility, provenance, validation, and fixture documents;
they do not widen the summary claims above.
