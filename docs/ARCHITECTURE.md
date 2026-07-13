# Architecture

## Current slice

The repository contains one `std`-only facade crate. Its public surface reports
`BoundedLocalContracts`, declares the repository ownership boundary, and
implements two local value families: `BoundedMetadata` under validated
`MetadataLimits`, and `Sample<T>` under validated `SampleLimits`. It does not
parse or serialize XML, open sockets, discover streams, create threads, read
clocks, allocate queues, load native libraries, or alter process or platform
state. There are no Cargo features or dependencies.

Construction validates the complete caller-provided value before returning an
accepted value. Invalid limit configurations, exceeded metadata bounds, invalid
declared channel counts, and value-count mismatches return typed deterministic
errors with stable expected/actual fields. Accepted strings and sample values
are not normalized or reordered. Protocol, runtime, testkit, oracle, and C ABI
crates remain deferred until a concrete ownership or dependency boundary
justifies a split.

The `morphospace/` directory is an inert planning and composition control
surface. Its presence does not activate code, packaging, permissions, network
access, native libraries, runtime profiles, or compatibility behavior.

The accepted STRM-000 baseline adds only specification-level compatibility cases, damaged-input
expectations, an isolated black-box oracle procedure, and deterministic
validation. These feedback-plane artifacts are neither data-plane behavior nor
runtime receipts. CORE-001 records its local unit-test results in a separate
overlay rather than rewriting that historical baseline as a measurement.

## Ownership

Rusty LSL owns independently authored, backend-neutral APIs and behavior for:

- LSL-compatible metadata and bounded metadata parsing;
- discovery observations;
- typed sample frames and chunks;
- raw source timestamps and derived clock views;
- bounded buffering, cancellation, and recovery;
- provider selection, health, and explicit fallback evidence.

Rusty LSL does not decide stream admission, identity, authorization, routes,
leases, registry revisions, product policy, platform permissions, packaging,
or application defaults. Discovery produces observations, not authority.
Inbound samples are data and cannot directly apply commands. High-rate media
does not belong in the generic sample path.

Deeper Rusty Morphospace integration stops at typed observations and
proposals. Rusty Manifold remains the authority for accepted streams,
subscriptions, routes, leases, revisions, and audit. Morphospace-native sample
transport, topology, identity, permissions, platform lifecycle, Quest and
Hostess adapters, and application policy remain in their owning repositories.

## Contract invariants

CORE-001 makes metadata collection/text limits and sample channel limits part
of validated construction. Future public contracts must likewise make bounds
part of their types or construction:
metadata size and depth, channel count, frame and chunk size, queue capacity,
timeout, retry count, and retained timestamp range. Invalid or oversized input
must return a typed error rather than trigger unbounded work.

Raw source timestamps must remain available without correction. Corrected and
smoothed time are separately identified derived views. Provider fallback must
name the selected candidate and retain the rejected candidate's failure.

Only the metadata and sample-shape construction invariants are implemented.
The remaining invariants constrain future design; none is an LSL runtime claim.

## Dependency direction

The default production closure is currently:

```text
rusty-lsl -> Rust standard library
```

Official native libraries, wrappers, oracle executables, captures, and test
endpoints must remain outside that closure. Any future dependency requires an
explicit purpose, license and provenance review, enabled-feature review, and a
validation update.

## Promotion gate

Runtime work may begin only after the compatibility behavior under test is
written independently, its fixtures have recorded provenance, oracle use is
isolated and reproducible, negative cases are named, and the resulting claim
is limited to the evidence actually collected. See `COMPATIBILITY.md`,
`PROVENANCE.md`, and `VALIDATION.md`.
