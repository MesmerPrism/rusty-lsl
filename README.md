# Rusty LSL

Rusty LSL is an independently authored Rust implementation of Lab Streaming
Layer compatibility. It is designed for the existing LSL ecosystem and for
explicit, typed integration with Rusty Morphospace.

Status: source-only crate with dependency-free bounded metadata, sample-shape,
finite raw source timestamp, optional derived timestamp, and timestamped-chunk
contracts, plus bounded core stream descriptors and the accepted
specification-level STRM-000 baseline. No LSL
protocol, wire, runtime, operational, or ecosystem compatibility is implemented
or claimed. Every historical STRM-000 catalog and damaged-case result remains
`not-implemented`, and no official-liblsl observation has been measured.

The architecture keeps LSL interoperability at a data-plane edge:

- Rusty LSL owns LSL-compatible metadata, discovery, samples, clocks, recovery,
  and backend-neutral Rust APIs.
- Rusty Manifold remains the authority for accepted Morphospace streams,
  subscriptions, routes, leases, revisions, and audit.
- Platform and operator adapters remain outside this repository.
- Deeper Morphospace hooks emit observations and proposals; they do not bypass
  the owning authority.

Project-owned public source is licensed `AGPL-3.0-or-later`. Official liblsl
is an MIT-licensed interoperability oracle, not a source template. rLSL source
is not an implementation input.

CORE-001 implements only local Rust contract semantics: validated nonzero
limits, atomic construction, deterministic bound and channel-mismatch errors,
and preservation of accepted caller-provided values. It adds no XML, protocol,
transport, runtime, or authority behavior. CORE-002 adds finite raw source
timestamps, separately typed optional derived timestamp values explicitly
classified as `ClockCorrected` or `Smoothed`, timestamped samples, and chunks
with explicit maximum sample and channel counts. Those kinds classify only
caller-provided values; they do not implement either algorithm. Accepted
floating-point bits, sample values, per-sample timestamp pairing, and order are
preserved. Valid nonzero limits accept an empty bounded chunk. Non-finite
timestamps, zero maxima, one-past maxima, and inconsistent sample shapes return
typed deterministic errors. This local slice does not read clocks or calculate
correction, dejittering, smoothing, interpolation, or sample-rate-derived
timestamps.

CORE-003 adds a bounded descriptor with a required nonempty stream name,
optional bounded opaque text for content type and source correlation, a
positive bounded channel count, an explicit nominal sample rate, and one of
exactly seven data-only channel formats. All text maxima count Unicode scalar
values. Accepted name, content type, and source correlation text is preserved
exactly without trimming, case folding, normalization, inference, or
reordering. Optional source correlation remains caller data only; it is not a
global identity, outlet UID, discovery or recovery decision, route,
permission, admission, or Morphospace/Manifold authority.

`NominalSampleRate` is explicitly irregular or a validated finite positive
regular-Hz value that preserves its accepted floating-point bits. It does not
read clocks, measure, schedule, enforce, interpolate, or derive rates.
`ChannelFormat` names `Float32`, `Double64`, `String`, `Int32`, `Int16`, `Int8`,
and `Int64`; these variants have no protocol or wire numeric discriminants and
do not size, encode, decode, or convert samples. CORE-003 adds no XML,
metadata-tree queries or mutation, discovery, networking, transport, buffering,
runtime identity, adapters, FFI, or external authority.

The separate STRM-000 baseline
continues to distinguish independently authored specifications, planned
black-box observations, and measured results; only the first exists today. See
[`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md),
[`docs/ORACLE.md`](docs/ORACLE.md), and the deterministic public fixtures under
[`fixtures/compatibility/`](fixtures/compatibility/README.md).

Development is bounded by the public project-local control surface under
[`morphospace/`](morphospace/README.md). Its lock selects no feature or module
and permits no runtime effect. Workflow state records CORE-003 as active for
this bounded implementation and validation slice. The separate CORE-001 and
CORE-002 overlays and the separate CORE-003 local-results overlay report only
local Rust contract tests. Activity and local unit-test results are not LSL
interoperability or runtime evidence.
