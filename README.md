# Rusty LSL

## LSLC-001G bounded element-tree serialization

`XmlElementSerialization::serialize` borrows an accepted `XmlElementTree` and
returns one byte-bounded owned UTF-8 `String`. Fixed local policy emits explicit
start and end tags for every node, inserts no whitespace, visits container
children depth-first with siblings in ascending original arena index, and
copies accepted `XmlCharacterData` verbatim without decoding or re-escaping.
Exact checked length and limit rejection precede one exact fallible traversal-
frame-stack reserve and one exact fallible output reserve. The frames index
direct-child and next-sibling links once, so traversal is linear in node count.
The source remains owned by the
caller; accepted output exposes only its limit, borrowed text, and
allocation-preserving consuming access.

This local string projection is not a complete XML or LSL stream-info
document. It assigns no `info` or `desc` role, field mapping, endpoint or
official-liblsl behavior, round-trip claim, protocol, wire, I/O, runtime,
adapter, provider, or authority meaning. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001g.ps1`
for the focused gate.

Rusty LSL is an independently authored Rust implementation of Lab Streaming
Layer compatibility. It is designed for the existing LSL ecosystem and for
explicit, typed integration with Rusty Morphospace.

Status: source-only crate with dependency-free bounded metadata, sample-shape,
finite raw source timestamp, optional derived timestamp, and timestamped-chunk
contracts, plus bounded core stream descriptors, a bounded parent-before-child
flat metadata-tree arena, a data-only descriptor/sample binding, a timestamped
descriptor/sample composition, a non-empty timestamped descriptor/chunk
composition, an infallible local stream-definition composition, and the
accepted specification-level STRM-000 baseline, the LSLC-001A
public-documentation specification corpus, and the LSLC-001B bounded XML
legal-text and element-name value contracts, plus the LSLC-001C bounded local
XML character-data representation, LSLC-001D leaf-only two-component
composition, LSLC-001E bounded container/leaf hierarchy, the LSLC-001F
consuming metadata-to-element-tree projection, and LSLC-001G bounded borrowed
element-tree serialization. No LSL
protocol, wire, runtime, operational, or ecosystem compatibility is implemented
or claimed. Every historical STRM-000 catalog and damaged-case result remains
`not-implemented`, and no official-liblsl observation has been measured.
Every LSLC-001A oracle observation and candidate result is `not-observed` with
null evidence.

LSLC-001B adds separate validated nonzero Unicode scalar-value limits for
`XmlText` and `XmlElementName`. `XmlText` may be empty and accepts exactly the
XML 1.0 Fifth Edition `Char` production. `XmlElementName` is nonempty and
accepts the complete `NameStartChar` and `NameChar` productions. Typed errors
retain deterministic expected/actual counts or the first rejected scalar's
zero-based index and code point. Accepted values expose their limit, borrowed
text, and consuming `String` access while preserving the original allocation
and exact contents.

Ampersand, less-than, greater-than, and caller text containing `]]>` remain
unchanged accepted values. Colon is accepted name syntax only and grants no
namespace meaning. This slice adds no escaping, entity or CDATA policy,
parsing, serialization, byte output, document assembly, LSL field mapping,
protocol, transport, wire, runtime, or compatibility behavior.

LSLC-001C separately borrows an accepted `XmlText` and produces
`XmlCharacterData` under a fixed candidate-owned policy: `&`, `<`, and `>`
become `&amp;`, `&lt;`, and `&gt;`; every other legal scalar remains byte-for-byte
unchanged in UTF-8. Thus `&<>]]>` becomes `&amp;&lt;&gt;]]&gt;` and `&amp;` becomes
`&amp;amp;`. Global greater-than escaping is local policy, not observed liblsl
behavior.

`XmlCharacterDataLimit` is a nonzero encoded UTF-8 byte maximum. Encoding
calculates the exact length with checked arithmetic, rejects output exceeding
the maximum before allocation, reserves fallibly, and returns typed overflow,
limit, or allocation errors in that order. Private accepted state exposes `limit()`, `as_str()`, and
allocation-preserving `into_string()`. This does not add an XML element,
attribute, document, parser, decoder, generic entity engine, CDATA-section, LSL
mapping, protocol, wire, transport, adapter, provider, or runtime API.

LSLC-001D adds `XmlLeafElement`, an infallible composition of exactly one
accepted `XmlElementName` and one accepted `XmlCharacterData`. Construction
moves both components directly into private state. Borrowed `name()` and
`character_data()` access and consuming `into_parts()` preserve their limits,
exact contents, and owned string allocations without cloning, allocation,
revalidation, re-encoding, normalization, or interpretation.

The value is leaf-only contract state, not tag syntax or a serialized XML
element. It adds no raw-string entrypoint, start/end/empty-element spelling,
attributes, children, mixed content, parent, root, tree, or document structure,
namespace meaning, parser, serializer, raw bytes, metadata-tree or stream-info
mapping, protocol, wire, runtime, or compatibility claim. Colon remains name
syntax only, and global greater-than escaping remains LSLC-001C local candidate
policy rather than observed liblsl behavior.

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

LSLC-001A adds a separate provenance-locked corpus of independently worded
public-documentation claims and bounded future test roles. It covers the
documented stream-info top-level, core, extended-description, and runtime/misc
roles plus the XML 1.0 legal-character and character-data constraints named in
[`docs/CORPUS.md`](docs/CORPUS.md). It contains no XML payload, endpoint output,
parser, serializer, or observed compatibility result. Exact serialization is
unresolved pending a separately approved black-box oracle unit.

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

LSLC-001E adds a dependency-free bounded parent-before-child hierarchy over
accepted XML components. `XmlElementNodeValue` contains either a name-only
container or an accepted `XmlLeafElement`; index zero is the sole root, every
later parent is strictly earlier and a container, and root-one depth and direct
child counts are checked iteratively.

`XmlElementTreeLimits` bounds nodes, depth, children per container, and retained
UTF-8 bytes. Retained bytes are the checked sum of owned container names, leaf
names, and represented character data; they are an arena resource count, not
serialized or wire size. Validation fallibly reserves one private scratch
`Vec`. Accepted `XmlElementTree` state keeps the original caller node vector
and component allocations, provides read-only inspection and consuming
recovery, and exposes neither mutable access nor `Clone` on owning candidate
node, value, or tree types.

The hierarchy is not a complete XML tree or document and assigns no mixed
content, tag spelling, serialization order, raw bytes, parser, mutation,
`MetadataTree` conversion, `info`/`desc` role, stream-info mapping, endpoint,
protocol, wire, runtime, authority, or compatibility meaning.

LSLC-001F adds
`project_metadata_tree_to_xml_element_tree(source, limits)` as the sole public
projection. It consumes an accepted `MetadataTree`; maps `None` to a
name-only container and every `Some`, including empty text, to a represented
leaf; preserves node order, parent indexes, root identity, and name `String`
allocations; and delegates a distinct fallibly reserved candidate arena to
`XmlElementTree`. `MetadataXmlProjectionLimits` contains only caller-selected
accepted XML name, text, character-data, and element-tree limits.

Target node count rejects before the first child whose parent is
value-bearing, then output reservation precedes per-node name, optional text,
and character-data validation. Final hierarchy errors are retained unchanged.
This partial local classification is one-way only: there is no borrowed or
reverse conversion, `From`/`TryFrom` or default policy, decoding, logical-text
recovery, round-trip claim, document production, stream-info or LSL mapping,
endpoint representation, protocol, wire, runtime, or compatibility behavior.
