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

Project-owned source is licensed AGPL-3.0-or-later. Official liblsl is used as
a pinned compatibility endpoint, never as an implementation template. Public
liblsl 1.17.7 receiver/server source was inspected only to diagnose endpoint
connection roles; no source was copied or translated. rLSL source is not an
implementation input.

## Persistent Float32 chunk outlet

`PersistentFloat32Outlet` is a caller-owned, explicitly activated, long-lived
Float32 TCP outlet. Construction fixes the channel count and allocates one
bounded reusable flat chunk buffer plus a bounded consumer registry. The caller
polls for pending consumers and calls `push_chunk(&[f32],
&[RawSourceTimestamp], ...)`; one call encodes the complete interleaved chunk
once and performs one contiguous bounded write per retained consumer. Multiple
independent outlets and multiple consumers per outlet are supported without a
crate-owned thread, timer, discovery policy, or allocation in `push_chunk`.

This is a public candidate API, not default activation, a stable-version
promise, automatic discovery, arbitrary-format support, recovery policy, or a
claim of official liblsl/non-loopback/device interoperability. Those remain
separate qualification boundaries.

`PersistentFloat32OutletService` now composes that outlet with the existing
short-info responder. The caller supplies one concrete IPv4 interface and owns
the poll loop. Two pinned pylsl 1.18.2/liblsl 1.17 runs resolved the service
and received an exact one-channel, ten-record Float32 chunk and timestamps.
This qualifies that narrow Windows-host path, not background operation,
automatic interface selection, recovery parity, arbitrary shapes,
cross-platform/device behavior, stable API, or release readiness.

The same-host Polar-shaped comparison at Rusty LSL `893cdc4` and Polar Stream
`5e13f64` measured 14.2 µs median / 23.5 µs p95 for Rusty LSL and 4.2 µs /
5.9 µs for current Polar Stream/liblsl. Polar includes one `lsl_local_clock`
plus one native chunk call; Rusty LSL receives caller-supplied timestamps and
encodes ten records before one TCP write. This is descriptive sender occupancy,
not BLE-to-recorder latency, but establishes no current Rusty LSL speed
advantage.

DEVICE-001 adds one physical Windows Polar H10 qualification. The reference
capture preserved 130 Hz ECG as 73-record notifications and nominal 200 Hz
three-axis accelerometer data as 36-record notifications. One exact captured
ECG notification then resolved through the managed Rusty outlet and reached a
pinned official liblsl inlet with exact Float32 values and timestamps. The
candidate also composes and admits truthful `130.0000000000000` and
`200.0000000000000` nominal-rate metadata.

At those observed notification shapes, five repeated sender microbenchmarks
showed no Rusty speed advantage: ECG median-of-medians was 13.4 µs for both
senders, while accelerometer chunks measured 13.9 µs for Rusty and 7.0 µs for
liblsl. This qualifies the Rusty transport for a bounded Polar adapter pilot;
it is not a recommendation to replace liblsl in production. The current Polar
Stream Windows input wrapper connected but emitted no PMD notifications in two
runs, while its direct protocol path streamed successfully; that separate
Polar-side defect prevents a current-repository end-to-end integration claim.

## Float32 sender state and measurement

An accepted bounded Float32 outlet session allocates one exact-size encoding
buffer before caller-record transfer and reuses it for initialization and every
declared record. Its bounded writer also retains the current socket write
timeout and changes it only when the effective timeout changes; cancellation
and a fresh total deadline are still checked for every record.

`python ./tools/run_float32_sender_benchmark.py` runs a descriptive release-mode
loopback microbenchmark and emits one JSON record with the revision, dirty-state
flag, host, dimensions, median, and p95. It remains the historical finite-session
baseline and is not a performance gate or a claim about the persistent outlet,
application latency, or liblsl parity.

`python ./tools/run_persistent_float32_outlet_benchmark.py` separately measures
one reusable chunk submission over one already established loopback consumer.
Its result is descriptive and host-bound; it does not include connection,
discovery, BLE, recorder, scheduling, or recovery latency.

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
