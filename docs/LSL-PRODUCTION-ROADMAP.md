# Rusty LSL Production Roadmap

## Completion definition

Rusty LSL is complete only when it provides a production-quality,
independently authored, pure-Rust LSL implementation with a coherent public
lifecycle, bounded native transport, required compatibility evidence, typed
advisory Morphospace integration, host and Quest qualification, stable-module
promotion, and a reviewed release path. Public-main source integration, an
exhausted test queue, or isolated capability facades do not satisfy this
definition. The integrated source remains a `0.0.0`, `publish = false`,
default-inert candidate rather than a stable, tagged, or registry-published
release.

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
8. P8: stable promotion and versioned release readiness after completed
   public-main source integration.

## Integrated production sequence

The production sequence through P68 is integrated on public `main`. It includes
the bounded multi-format lifecycle, discovery, processing, recovery,
observability, advisory, host-qualification, and release-review source
surfaces described by the authoritative architecture, compatibility,
validation, and history documents.

That integration does not activate anything by default, establish arbitrary
shape or ecosystem compatibility, complete host-to-Quest qualification, make
the public API stable, approve a version, or authorize a tag, registry
publication, or Manifold authority. Those remain distinct reviewed boundaries.

## Unit guard

Keep one canonical current unit. At every accepted/published boundary, persist
exact Git/workspace/receipt/cleanup state and review product value and
architecture before declaring the next unit. Compatibility-only micro-units
must be directly required by an active production acceptance gate and may not
replace the next production slice.
