# Architecture

## Current slice

The repository contains one `std`-only facade crate. Its public surface reports
`BoundedLocalContracts`, declares the repository ownership boundary, and
implements local bounded metadata, sample shape, timestamp value, timestamped
sample, chunk, core stream-descriptor, and flat metadata-tree families.
The separate descriptor/sample binding family accepts exactly seven
homogeneous `Sample<T>` representations and binds each one to the matching
data-only descriptor format and exact descriptor channel count.
The separate timestamped descriptor/sample composition family accepts the
same seven `TimestampedSample<T>` representations, moves each apart once, and
delegates the unchanged sample to `BoundDescriptorSample::new`. Its unforgeable
accepted state owns only that compact binding plus the unchanged raw source and
optional derived timestamp evidence.
The separate timestamped descriptor/chunk composition family accepts the same
seven `TimestampedChunk<T>` representations. It rejects an empty existing chunk
before delegation, retains the original chunk limits, and moves every ordered
sample exactly once through `BoundTimestampedDescriptorSample::new`. Its
unforgeable accepted state owns only those limits and the ordered CORE-006
bindings; indexed failures retain the first rejected sample location and
unchanged delegated error.
The focused stream-definition composition moves one already validated
`StreamDescriptor` and one already validated generic `MetadataTree` directly
into an unforgeable `StreamDefinition`. Its private state contains exactly
those two owned components. Borrowed access exposes each unchanged component;
consuming access returns both without cross-component validation,
interpretation, normalization, inference, cloning, or allocation by the
composition layer.
`RawSourceTimestamp` and
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

The focused `xml_value` module owns only bounded XML 1.0 Fifth Edition value
validation. `XmlTextLimit` and `XmlNameLimit` are separate explicit nonzero
Unicode scalar-value maxima. `XmlText` accepts empty input and exactly the
`Char` production. `XmlElementName` requires a `NameStartChar` followed by
zero or more `NameChar` scalars. Both accepted types privately retain the
validated limit and original `String`, expose borrowed text, and return the
same allocation through consuming access.

Text validation checks length before the first illegal scalar. Name validation
checks empty, length, start, then continuation, with scalar indexes and code
points retained in typed errors. Colon has syntax-only meaning. Ampersand,
less-than, greater-than, and `]]>` remain caller values; no representation
policy is selected. The module owns no parser, serializer, escaping, entity,
CDATA, document, byte-output, attribute, namespace, schema, query, LSL mapping,
protocol, wire, transport, runtime, or I/O API.

The focused private `xml_character_data` module composes only over borrowed
accepted `XmlText`. `XmlCharacterDataLimit` owns a nonzero encoded UTF-8 byte
maximum. `XmlCharacterData::encode` performs an exact checked-length pass,
rejects an exceeded maximum before allocation, uses `String::try_reserve_exact`,
and then writes the exact precomputed length. Its deterministic error order is
length overflow, exceeded limit with exact expected/required byte counts, then
allocation failure with the requested count.

The candidate-owned representation maps every ampersand, less-than, and
greater-than to `&amp;`, `&lt;`, and `&gt;`. All other legal input scalars, including
quotes, apostrophes, whitespace, non-ASCII scalars, and legal noncharacters,
remain unchanged. Accepted output and its limit are private; borrowed and
consuming access preserves the output allocation. This local representation is
not an element, attribute, document, parser, decoder, generic entity engine,
CDATA-section API, LSL field mapping, or endpoint serialization claim.

`MetadataTree` owns a parent-before-child flat arena. Unvalidated
`MetadataNodeInput` values use `Option<usize>` parent indices: exactly one root
at index zero has no parent, and each later node must name a strictly earlier
parent. Accepted nodes do not own children. One forward pass computes depths
and direct-child counts in vectors, so construction uses no recursive public
ownership and no recursive validation or traversal. Root depth is one.
`MetadataTreeLimits` requires nonzero maxima for nodes, depth, direct children,
name Unicode scalar values, and optional value Unicode scalar values. Required
names are nonempty; optional values preserve absence versus an empty string.
Accepted flat order, parent indices, text, whitespace, and optional-value form
are unchanged and available only through read-only or consuming accessors.

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

`DescriptorSampleInput` is public unvalidated binding input. It owns one
already validated `Sample<T>` for exactly one of `f32`, `f64`, `String`, `i32`,
`i16`, `i8`, or `i64`. `BoundDescriptorSample` cannot be publicly forged: it
stores private accepted fields containing the unchanged sample and only a
compact descriptor-shape snapshot of channel count and `ChannelFormat`.
Construction borrows the validated descriptor and does not clone it.
`DescriptorSampleLimits` requires a nonzero maximum Unicode scalar-value count
for each String channel. Validation reports format mismatch, channel-count
mismatch, or the first oversized String channel deterministically. It performs
no conversion, casting, parsing, formatting, normalization, inference, byte
sizing, encoding, decoding, endianness, wire mapping, or runtime action.

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
runtime receipts. CORE-001, CORE-002, CORE-003, CORE-004, CORE-005, CORE-006,
CORE-007, and CORE-008
record local Rust contract tests in separate overlays rather than rewriting
that historical baseline as a measurement.

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

CORE-004 makes flat metadata-tree node count, root/parent structure, depth,
direct child fanout, required names, and optional-value text bounds part of
validated construction. It does not define XML names or documents, parsing,
serialization, escaping, namespaces, attributes, entities, schemas, queries,
mutation, discovery, protocol, wire, transport, runtime, or authority behavior.

CORE-005 makes exact descriptor format, exact descriptor/sample channel count,
and per-String-channel Unicode scalar bounds part of accepted construction.
Accepted strings, integer values, and floating-point bits including signed zero
and NaN payloads remain unchanged and ordered.

CORE-006 composes, but does not replace, the CORE-002 and CORE-005 contracts.
It preserves mandatory raw source timestamp bits, optional derived absence or
presence, derived kind and bits, homogeneous values and order, and their exact
pairing. Format, then channel count, then per-String Unicode scalar validation
remains owned by CORE-005 and returns its unchanged typed errors. CORE-006 does
not clone values, read clocks, derive or rewrite timestamps, sort, schedule,
buffer, convert, encode, transport, or perform runtime work.

CORE-007 composes, but does not replace, the CORE-002, CORE-005, and CORE-006
contracts. It adds only local non-empty acceptance, original `ChunkLimits`
retention, caller-order iteration, and zero-based first-failure indexing.
Format, channel count, String bounds, values, raw and derived timestamp
evidence, and their pairings remain owned by the delegated contracts. CORE-007
does not clone values, read clocks, calculate timestamps, sort, rewrite, split,
merge, rechunk, buffer, queue, schedule, convert, encode, transport, or perform
runtime work. Its empty rejection is not LSL compatibility evidence.

CORE-008 composes, but does not replace or reinterpret, the CORE-003 and
CORE-004 contracts. `StreamDefinition` retains the complete validated
descriptor and complete validated tree rather than copying snapshots or
creating parallel limits. Its infallible constructor moves both components
directly and adds no error family or validation order. The generic
metadata-tree root is not an LSL `desc` element. CORE-008 adds no XML document
shape, channel metadata convention, runtime identity, version, creation time,
UID, session, host, address, port, fingerprint, recovery, discovery, clock,
buffer, provider, adapter, authority, protocol, wire, or runtime behavior.

LSLC-001B implements only XML legal-text and element-name value invariants.
Text and name maxima count Unicode scalar values rather than bytes or grapheme
clusters. Accepted allocation and content are unchanged. The local contract
does not interpret the generic metadata tree, create XML nodes or documents,
or choose how accepted caller values are represented.

LSLC-001C composes over, but does not replace, the LSLC-001B `XmlText`
contract. It neither revalidates nor mutates the source and owns only the fixed
local three-character replacement policy plus exact bounded output allocation.
Its global greater-than replacement is candidate policy, not oracle evidence.

CORE-002 implements finite raw source timestamp retention and a separately
typed optional derived timestamp value classified as `ClockCorrected` or
`Smoothed`. A derived value cannot replace, hide, or mutate the raw value. The
kind and value are both caller-provided: the crate does not read clocks or
calculate correction, dejittering, smoothing, interpolation, or
sample-rate-derived timestamps. Provider fallback must name the selected
candidate and retain the rejected candidate's failure.

Only the metadata, sample-shape, timestamp-value, bounded-chunk, core
stream-descriptor, flat metadata-tree, descriptor/sample binding, and
timestamped descriptor/sample, non-empty descriptor/chunk, and
stream-definition composition
construction invariants are
implemented. The remaining
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
