# Rusty LSL

Rusty LSL is an independently authored Rust implementation of Lab Streaming
Layer compatibility. It is designed for the existing LSL ecosystem and for
explicit, typed integration with Rusty Morphospace.

Status: source-only crate with dependency-free bounded metadata, sample-shape,
finite raw source timestamp, optional derived timestamp, and timestamped-chunk
contracts, plus bounded core stream descriptors, a bounded parent-before-child
flat metadata-tree arena, a data-only descriptor/sample binding, a timestamped
descriptor/sample composition, a non-empty timestamped descriptor/chunk
composition, an infallible local stream-definition composition, and the
accepted specification-level STRM-000
baseline. No LSL
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

CORE-004 adds an unvalidated flat node-input type and atomically constructs a
bounded `MetadataTree`. Exactly one root is required at index zero, root depth
is one, and every later node must name a parent index strictly less than its own
index. Nonzero limits bound total nodes, depth, direct children per node, name
Unicode scalar values, and optional value Unicode scalar values. Names are
required and nonempty. Optional values preserve `None` versus `Some("")`.
Accepted node order, parent indices, names, and values are retained exactly in
private accepted fields with read-only accessors. Storage, validation, and
inspection are flat and iterative, with no recursive public ownership or
recursive validation/traversal.

CORE-004 adds no XML syntax, parser, serializer, namespace, attribute, entity,
schema, query, mutation, document assembly, protocol, wire, discovery,
networking, transport, runtime, clock, provider, adapter, or authority behavior.
Its tests prove only the named local Rust contract semantics.

CORE-005 adds a separate descriptor/sample binding module. Its public
unvalidated input family contains exactly seven homogeneous representations:
`Sample<f32>`, `Sample<f64>`, `Sample<String>`, `Sample<i32>`, `Sample<i16>`,
`Sample<i8>`, and `Sample<i64>`, mapped one-to-one to the existing data-only
`ChannelFormat` values. Construction reuses the validated `Sample<T>` and
`StreamDescriptor` contracts, requires the descriptor's exact format and
channel count, and stores only a compact validated shape snapshot rather than
copying the full descriptor.

An explicit nonzero limit bounds each String channel by Unicode scalar values.
Empty strings and all accepted text remain exact. Numeric values remain in
order and preserve exact integer values and floating-point bits, including
signed zero and chosen NaN payloads. CORE-005 performs no conversion, casting,
parsing, formatting, normalization, inference, byte sizing, encoding, decoding,
endianness, wire mapping, or runtime action.

CORE-006 adds a separate timestamped descriptor/sample composition whose
public unvalidated input family contains exactly seven homogeneous
`TimestampedSample<T>` variants corresponding to the existing data-only
formats. Construction moves the timestamped sample apart once and delegates
the unchanged sample to `BoundDescriptorSample::new`, preserving CORE-005
format, descriptor/sample channel-count, String Unicode-scalar validation,
error payloads, and format → channel-count → String-bound precedence.

Accepted state privately owns the resulting binding plus the mandatory raw
source timestamp and optional derived timestamp evidence. It preserves raw
source timestamp bits, optional derived `None` versus `Some`, derived kind and
bits, sample values and order, and their exact pairing without cloning or
recalculation. This composition reads no clock and performs no timestamp
derivation, correction, smoothing, dejittering, interpolation, sorting,
rewriting, scheduling, buffering, conversion, encoding, transport, protocol,
wire, or runtime action.

CORE-007 adds a separate timestamped descriptor/chunk composition whose public
input family contains exactly seven homogeneous `TimestampedChunk<T>` variants.
It rejects an empty existing chunk before delegation, retains the original
chunk limits exactly, and moves every sample in caller order exactly once
through `BoundTimestampedDescriptorSample::new`. Accepted private state owns
only those original limits and the ordered CORE-006 bindings. The first
delegated failure reports its zero-based sample index and unchanged
`DescriptorSampleError`.

This local non-empty requirement is not a claim about actual LSL empty-chunk
compatibility. CORE-007 performs no clock read or timestamp algorithm, sorting,
rewriting, splitting, merging, rechunking, buffering, queueing, scheduling,
conversion, encoding, XML, discovery, networking, transport, protocol, wire,
or runtime action.

CORE-008 adds a focused `StreamDefinition` aggregate that moves one already
validated `StreamDescriptor` and one already validated `MetadataTree` directly
into private accepted state. Borrowed `descriptor()` and `extended_metadata()`
accessors and consuming `into_parts()` preserve both complete components,
including their limits, exact optional text forms, nominal-rate bits, channel
format, flat node order, parent indices, Unicode names, and optional values.
The infallible constructor adds no error or limit family, allocation, clone,
normalization, inference, or cross-component validation.

The generic metadata-tree root gains no LSL `desc`-element meaning. This slice
adds no XML/document assembly, channel metadata convention, runtime identity,
version, creation time, UID, session, host, address, port, fingerprint,
recovery, discovery, networking, clock, buffering, provider, adapter,
authority, protocol, wire, or runtime behavior.

The separate STRM-000 baseline
continues to distinguish independently authored specifications, planned
black-box observations, and measured results; only the first exists today. See
[`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md),
[`docs/ORACLE.md`](docs/ORACLE.md), and the deterministic public fixtures under
[`fixtures/compatibility/`](fixtures/compatibility/README.md).

Development is bounded by the public project-local control surface under
[`morphospace/`](morphospace/README.md). Its lock selects no feature or module
and permits no runtime effect. CORE-008 bounds this local implementation and
validation slice without asserting a particular workflow lifecycle state. The
separate CORE-001 and CORE-002 overlays, the separate CORE-003 and CORE-004
local-results overlays, and the separate CORE-005, CORE-006, CORE-007, and
CORE-008 local-results overlays report only local Rust contract tests. Activity
and local unit-test results are not LSL interoperability or runtime evidence.
