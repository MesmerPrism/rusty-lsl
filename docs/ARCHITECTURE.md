# Architecture

## Current slice

The repository currently contains one `std`-only facade crate. Its public
surface reports `ScaffoldOnly` and declares the repository ownership boundary;
it does not open sockets, discover streams, create threads, read clocks,
allocate queues, load native libraries, or alter process or platform state.
There are no Cargo features or dependencies.

This shape makes repository presence inert while leaving one reviewed public
surface from which later, evidence-backed APIs can grow. Protocol, runtime,
testkit, oracle, and C ABI crates are deferred until a concrete ownership or
dependency boundary justifies a split.

The `morphospace/` directory is an inert planning and composition control
surface. Its presence does not activate code, packaging, permissions, network
access, native libraries, runtime profiles, or compatibility behavior.

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

## Planned contract invariants

Future public contracts must make bounds part of their types or construction:
metadata size and depth, channel count, frame and chunk size, queue capacity,
timeout, retry count, and retained timestamp range. Invalid or oversized input
must return a typed error rather than trigger unbounded work.

Raw source timestamps must remain available without correction. Corrected and
smoothed time are separately identified derived views. Provider fallback must
name the selected candidate and retain the rejected candidate's failure.

No planned API is implemented by this scaffold. These invariants constrain
future design; they are not runtime claims.

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
