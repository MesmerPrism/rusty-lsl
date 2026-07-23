# Stable public API promotion contract

## Status

RLSL-P8 is a **feature-branch release candidate**, not a released stable-version
promise. The executable contract in
`crates/rusty-lsl/tests/stable_public_api.rs` demonstrates that representative
already-public production lifecycle APIs compose from an external crate at the
candidate commit. It is evidence for release review; it does not create a
versioned compatibility commitment, publish a crate, change a version, tag a
release, or enable runtime behavior by default.

## Promoted release-candidate surface

The candidate contract promotes the following coherent, representative slice
of the crate-root public facade:

- explicit receipt-bound runtime activation through `admit_runtime_activation`,
  `RuntimeActivationSelection`, `RuntimeModule`, and the accepted feature-lock
  constants;
- bounded UDP discovery through `run_udp_discovery`, its activation,
  configuration, limits, query-wire, response, and termination contracts, with
  caller-selected receive-order index;
- exact one-channel, two-record Float32 transfer through
  `run_timestamped_float32_two_record_chunk_outlet` and
  `run_timestamped_float32_two_record_chunk_inlet`, including public sample,
  timestamp, chunk, handshake, activation, limit, and report values;
- caller-bounded Float32 processing through
  `run_bounded_float32_recovery_clock_queue`, finite-recovery policy and
  activation, integrated clock-correction configuration and activation, and
  bounded queue admission/pop/close;
- explicit lifecycle cleanup observable as completed peer joins, immediate UDP
  and TCP address reuse, drained queue ownership, and explicit queue closure.

The test uses the published crate-root facade exactly as an external consumer
would. Private modules and private lifecycle engines are not part of the
promotion.

## Compatibility and stability policy

Before a stable Rusty LSL version is released, this candidate surface may still
change through normal review. Any material change to a promoted name, signature,
ownership boundary, error/termination classification, activation requirement,
boundedness rule, or cleanup behavior must update this document and the
executable contract together and be reviewed as an intentional compatibility
change.

A future released stable-version promise requires an explicit release decision
and versioned policy. At that point, compatible maintenance should preserve the
documented public meanings and external-consumer compilation expected by the
release's compatibility policy. This candidate alone does not define a
permanent semantic-versioning horizon.

## Exclusions

The promotion does not cover every public symbol or every supported format and
shape. In particular, it excludes:

- generic or private codec/session strategies and private engine structure;
- automatic discovery, response filtering, selection, retries, scheduling,
  routing, queue policy, recovery policy, clock source, or interface choice;
- ambient or default activation, background work, persistent connections, or
  unbounded operation;
- all-format parity, arbitrary channel/record counts, device behavior, multicast
  portability, official liblsl interoperability, performance, or wire-protocol
  completeness;
- Morphospace or Manifold execution, admission, command, lease, authorization,
  audit, stream, or lifecycle authority.

## Forbidden claims

Passing the contract must not be described as a released stable API, a crates.io
publication, official LSL compatibility, comprehensive protocol conformance,
automatic policy, production deployment approval, device qualification,
zero-copy behavior, performance evidence, or authority beyond the exact
caller-owned bounded lifecycle exercised here. Loopback cleanup and address
reuse are host evidence only, not a portable timing or network guarantee.
