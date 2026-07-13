# Architecture

## Current slice

The repository contains one `std`-only facade crate. Its public surface reports
`BoundedLocalContracts`, declares the repository ownership boundary, and
implements local bounded metadata, sample shape, timestamp value, timestamped
sample, chunk, and core stream-descriptor families. `RawSourceTimestamp` and
`DerivedTimestamp` accept
only finite `f64` values and preserve their bits. Every `DerivedTimestamp`
stores an explicit non-exhaustive `DerivedTimestampKind`: currently
`ClockCorrected` or `Smoothed`. These are caller-supplied classifications, not
algorithm implementations. `TimestampedSample<T>` always retains its raw value
and can additionally retain a distinct optional derived timestamp.
`TimestampedChunk<T>` retains explicit `ChunkLimits` for maximum sample and
channel counts; a valid nonzero limit configuration accepts an empty bounded
collection. It does not parse or serialize XML, open sockets,
discover streams, create threads, read clocks, allocate queues, load native
libraries, or alter process or platform state. There are no Cargo features or
dependencies.

`StreamDescriptor` requires a nonempty stream name, a positive bounded channel
count, and explicit nonzero maxima for name, content-type, source-id Unicode
scalar counts and channel count. Content type and source correlation are
optional bounded opaque text. Accepted text is preserved without trimming,
case folding, normalization, inference, or reordering. Source correlation has
no identity, discovery, recovery, authorization, routing, permission,
admission, or Morphospace/Manifold authority effect. The descriptor exposes no
runtime-assigned version, creation time, UID, session, host, address, or port.

`NominalSampleRate` distinguishes `Irregular` from a validated finite positive
`RegularHz` value and preserves accepted regular-rate bits. It performs no
clock read, rate measurement, scheduling, enforcement, interpolation, or rate
derivation. `ChannelFormat` has exactly seven independently named data-only
variants and assigns no protocol or wire numeric discriminants; it performs no
byte sizing, encoding, decoding, or value conversion.

Construction validates the complete caller-provided value before returning an
accepted value. Invalid limit configurations, exceeded metadata or chunk
bounds, invalid declared channel counts, value-count mismatches, non-finite
timestamps, and inconsistent chunk shapes return typed deterministic errors
with stable fields. Accepted strings, sample values, floating-point timestamp
bits, sample/time pairing, and order are not normalized or reordered. Protocol,
runtime, testkit, oracle, and C ABI crates remain deferred until a concrete
ownership or dependency boundary justifies a split.

The `morphospace/` directory is an inert planning and composition control
surface. Its presence does not activate code, packaging, permissions, network
access, native libraries, runtime profiles, or compatibility behavior.

The accepted STRM-000 baseline adds only specification-level compatibility cases, damaged-input
expectations, an isolated black-box oracle procedure, and deterministic
validation. These feedback-plane artifacts are neither data-plane behavior nor
runtime receipts. CORE-001, CORE-002, and CORE-003 record local Rust contract
tests in separate overlays rather than rewriting that historical baseline as a
measurement.

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
of validated construction. CORE-002 adds validated maximum sample and channel
counts for chunks. CORE-003 adds validated maximum Unicode scalar counts for
the required name and optional opaque text plus a validated maximum descriptor
channel count. Future public contracts must likewise make bounds part of
their types or construction:
metadata size and depth, channel count, frame and chunk size, queue capacity,
timeout, retry count, and retained timestamp range. Invalid or oversized input
must return a typed error rather than trigger unbounded work.

CORE-002 implements finite raw source timestamp retention and a separately
typed optional derived timestamp value classified as `ClockCorrected` or
`Smoothed`. A derived value cannot replace, hide, or mutate the raw value. The
kind and value are both caller-provided: the crate does not read clocks or
calculate correction, dejittering, smoothing, interpolation, or
sample-rate-derived timestamps. Provider fallback must name the selected
candidate and retain the rejected candidate's failure.

Only the metadata, sample-shape, timestamp-value, bounded-chunk, and core
stream-descriptor construction invariants are implemented. The remaining
invariants constrain future design; none is an LSL runtime claim.

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
