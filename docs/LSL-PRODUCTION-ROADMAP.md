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

The persistent-outlet sequence advances P1/P2/P3 with a caller-owned Float32
listener, reusable flat chunk submission, bounded retained consumer fan-out,
multiple independent outlets, and an explicit-interface caller-polled
discovery service. One pinned official liblsl consumer has resolved, opened,
pulled ten exact records, and closed on one Windows host. The same-host Polar
comparison shows no current Rusty LSL sender-occupancy advantage.

DEVICE-001 closes one Windows H10 transport prerequisite: full-rate ECG and
accelerometer notification shapes were captured, a real ECG frame reached a
pinned official consumer through Rusty, truthful 130/200 Hz metadata is now
composable and admissible, and H10-shaped sender occupancy is recorded. This
supports a bounded Polar adapter pilot but does not recommend replacing
liblsl. Current Polar Stream end-to-end qualification remains blocked by its
Windows input wrapper emitting no PMD notifications despite a working direct
protocol path.

POLAR-001 closes the bounded source prerequisite for multiple concurrently
discoverable Float32 outlets: one shared discovery socket, per-outlet timedata
in one source-clock domain, round-robin admissions, fail-fast slow-consumer
eviction, Polar rate/metadata composition, health counters, and deterministic
cleanup. It adds no ambient worker or application policy.

INTEROP-002 closes the device-free simultaneous official-inlet host gate. One
pinned pylsl 1.18.2/liblsl 1.17.7 ECG inlet and one ACC inlet open concurrently,
receive exact 73x1 and 36x3 `pull_chunk` payloads with caller timestamps, and
close within bounds. Exact auxiliary full-info requests use separate bounded
capacity from the single admitted data consumer on each outlet. This is not
generic multi-consumer, predicate-filter, device, LabRecorder, cross-host, or
cross-platform conformance.

The next production gates are a default-off application adapter,
BLE-to-recorder/LabRecorder qualification, non-loopback/cross-host and
Linux/macOS coverage,
reconnect/recovery behavior, arbitrary stream shapes, application integration,
and an explicit licensing decision. Stability, version, tag, registry
publication, and P8 release authority remain separately reviewed.

## Unit guard

Keep one canonical current unit. At every accepted/published boundary, persist
exact Git/workspace/receipt/cleanup state and review product value and
architecture before declaring the next unit. Compatibility-only micro-units
must be directly required by an active production acceptance gate and may not
replace the next production slice.
