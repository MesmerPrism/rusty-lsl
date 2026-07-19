# Rusty LSL Production Roadmap

## Completion definition

Rusty LSL is complete only when it provides a production-quality,
independently authored, pure-Rust LSL implementation with a coherent public
lifecycle, bounded native transport, required compatibility evidence, typed
advisory Morphospace integration, host and Quest qualification, stable-module
promotion, and a reviewed public integration/release path. An unpublished
feature branch, an exhausted test queue, or isolated capability facades do not
satisfy this definition.

Default activation stays disabled. A selected feature lock and explicit
caller runtime input remain necessary, accepted receipts remain identity-bound,
and public/private and provenance boundaries remain unchanged. Rusty LSL never
acquires Manifold admission, routing, lease, authorization, revision, or audit
authority.

## Priority order

1. P1: coherent public outlet/inlet session API and shared bounded record/chunk engine.
2. P2: all declared formats, channel counts, record counts, samples, and chunks.
3. P3: discovery, resolution, stream-info, connect, close, and cleanup lifecycle.
4. P4: clocking, requested post-processing, buffering, backpressure, recovery, and health.
5. P5: typed Morphospace observations and proposals with Manifold non-authority.
6. P6: representative native host and Rust-on-Quest qualification.
7. P7: secondary ecosystem compatibility selected by demonstrated adoption need.
8. P8: stable promotion, public-main integration review, and versioned release readiness.

## First production successor

`rlsl-lslc-007b-bounded-float32-session-engine` is the first P1 production
unit. It will add a reusable bounded Float32 outlet/inlet session engine that
supports the already accepted one-record and two-record chunk verticals behind
one lifecycle, while preserving existing public facades, activation receipts,
exact timestamp/value ownership, cancellation/deadline separation, terminal
cleanup, and immediate port reuse.

007B does not generalize all formats or arbitrary shapes, activate anything by
default, change discovery policy, add automatic recovery, claim broader
compatibility, perform device work, or acquire Manifold authority. Those remain
separate later units.

## Unit guard

Keep one canonical current unit. At every accepted/published boundary, persist
exact Git/workspace/receipt/cleanup state and review product value and
architecture before declaring the next unit. Compatibility-only micro-units
must be directly required by an active production acceptance gate and may not
replace the next production slice.
