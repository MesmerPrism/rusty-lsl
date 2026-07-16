# Architecture

## LSLC-003S StringSample activation descriptor

`StringSample` has its own opaque `RuntimeModuleCapability`, effective marker,
and declared `StreamHandshake` dependency. Admission rejects stale locks,
unknown or duplicate modules, mismatched markers, missing dependencies, and
absent selections. Lock revision 14 selects the descriptor while retaining
`run_activation_default: disabled`; the descriptor declares no executable
effect or permission. Capability construction stays private and all eight
prior module identities and relationships remain unchanged.

## LSLC-003P bounded record sequence

LSLC-003P composes the existing fixed-width numeric handshake and bounded
transport into three ordered timestamped records, each with exactly two
homogeneous channels. Constructors reject non-finite timestamps and the first
format mismatch before I/O. Inlet acceptance is atomic after initialization
and all three complete records arrive. Activation and authority are unchanged.

## Consumer role and plane facades

The crate root remains the compatibility facade and sole public-name source.
`contract` projects existing data and protocol contracts for data-plane
consumers; `runtime` projects existing explicit effects and activation
contracts for control/runtime consumers. Both modules contain re-exports only.
Private sibling implementation modules remain private and keep their existing
dependency direction, so the projections cannot become a second authority or
change activation and runtime behavior.

## Crate-private bounded fixed-record transport

The Float32 and fixed-width numeric sample families share one crate-private
exact-length TCP transfer helper. It owns only read/write looping, I/O-slice
timeouts, total-deadline observation, cancellation polling, peer truncation,
and socket error classification. Each runtime still owns record encoding,
format-specific initialization, activation capability consumption, its public
limits and error facade, and scope-owned socket cleanup.

LSLC-003E exposed the extraction seam but could not close changed feature
provenance inside its claimed paths, so it remains blocked. LSLC-003F retains
that history and refreshes the two feature descriptors, resolver-owned lock,
and LSLC-003C exact fingerprint binding as separate artifacts. The helper is
not public API and adds no transport selection or Manifold authority.

## Dependency-closed runtime facade composition

Runtime activation direction now follows the resolved lock: handshake feeds
both sample families; Float32 sample activation feeds queue and integrated
clock; queue feeds finite recovery. Discovery client and responder remain
dependency-free but still require their own nominal lock capability. The
facades consume opaque evidence and cannot construct or expand the lock.

## Lock-bound runtime activation

The resolved `feature.lock` remains composition evidence, not live runtime
state. `runtime_activation` independently admits only its exact accepted
fingerprint and revision plus caller-observed effective markers. The result
contains private-inner, module-nominal capabilities and a distinct
consumer-issued receipt that owns the bounded consumer identity and ordered
selected-module set. A missing module yields no capability; a requested module
without its declared dependency fails closed.

This closes lock admission only. Existing runtime facades do not yet consume
these capabilities, so LSLC-003C neither activates an effect nor claims full
runtime dependency composition. That reconciliation is the next architecture
slice. No adapter or Manifold authority participates.

## Official-compatible Float32 stream initialization

The outlet request parser retains exact role order, fixed values, and identity
fields; only `Endian-Performance` is structurally admitted as a finite positive
number inside the existing header bound. After handshake response, outlet and
inlet exchange exactly two evidence-pinned initialization records before one
caller sample. The inlet validates marker, timestamp bits, value bits, and
order before exposing caller data.

## Finite sample recovery

LSLC-002W synchronously invokes a caller operation under explicit attempt,
state, delay-slice, and deadline bounds. The caller classifies opaque failures
as retryable or terminal. The returned ordered trace makes every attempt and
termination observable; no endpoint or worker authority is absorbed.

## LSLC-002P bounded discovery runtime boundary

`udp_discovery` is a synchronous edge adapter owned by Rusty LSL. The caller
supplies the bind address, destination, accepted query bytes, response-envelope
limits, datagram/count limits, receive slice, total deadline, and cancellation
flag. The adapter owns one `UdpSocket` only for the call, observes its assigned
local address and response sources without claiming endpoint authority, and
moves admitted datagram allocations into the result before dropping the
socket. A selected lock remains inert until this explicit runtime input/call.

There is no interface enumeration, multicast membership, address selection,
retry loop, background worker, provider/currentness inference, or Manifold
state transition. Loopback behavior and cleanup evidence are separate from the
next official-endpoint interoperability unit.

LSLC-002E separates the observed response transport envelope from the existing
document contracts: an uninterpreted canonical decimal query-identifier prefix,
one CRLF delimiter, then an unchanged body accepted by LSLC-002A and LSLC-002B.
The official endpoint is evidence producer only. A later unit may implement
bounded local envelope parsing; correlation and networking remain separate.

## LSLC-002D corrective boundary

The wire-shape leaf now owns CRLF delimiters only. LSLC-002C's LF-only
candidate remains rejected historical evidence. No runtime or authority role
changes: the artifact is still inert bounded data, not endpoint selection,
network execution, discovery state, interoperability, or Manifold authority.

## LSLC-002C query byte-shape boundary

The short-info query module is a data-contract leaf. It owns exact local bytes,
field and payload maxima, canonical unsigned-decimal spelling, and borrowed
shape admission. The query text is nonempty printable ASCII and remains opaque;
the nonzero `u16` return port and `u64` query identifier remain uninterpreted.
It neither chooses nor opens an endpoint. Response documents and every network,
resolver, timing, provider, activation, and Manifold authority stay beyond this
boundary.

## LSLC-002B typed observation boundary

LSLC-002B is the consuming boundary from fixed LSLC-002A parsed state to the
existing static-definition and volatile-field contracts. Representation is
decoded without manufacturing acquisition evidence or authority.

## LSLC-002A observed document shape boundary

The private `stream_info_observed_document_parser` module scans one borrowed
`str` under an explicit nonzero byte maximum. It recognizes only the exact
empty-description LSLC-001R surface: fixed declaration and layout, seventeen
ordered leaves, represented character data, fixed end tags, and the final
empty `desc`/root suffix. Fixed borrowed end-tag spellings and a fixed
seventeen-range array eliminate transient and retained structural allocation.
Accepted values remain byte ranges into the unchanged source.

The scanner neither decodes represented data nor constructs semantic stream
state. It is not general XML, endpoint, protocol, wire, discovery, transport,
runtime, network, feature, device, or Manifold authority.

## LSLC-001Z local document facade

`stream_info_three_owner_observed_document` closes the accepted N/X-to-R local
dependency chain through P and Q. Its accepted state keeps the implementation,
runtime, and transport witnesses separate from the owned document; intermediate
accepted snapshot and element-tree state is not exposed as a new authority.

## LSLC-001X three-owner composition boundary

`stream_info_three_owner_snapshot` is a consuming composition edge above the
accepted T/U/V acquisition contracts and S admission. Owner evidence remains
three nominal witness objects; value allocations alone move into S's fixed
implementation, runtime, and transport lanes. The module performs no provider
call and establishes no relation among owner epochs or revisions. Acquisition,
activation, platform/network behavior, and Manifold authority remain outside.

## LSLC-001V transport-owner boundary

`stream_info_transport_provider` keeps six same-owner endpoint strings atomic
at the evidence boundary: one call, one shared identity/epoch/revision witness,
and six opaque allocations. It applies only the accepted O transport bound and
can project only S's transport lane. Complete S admission, platform endpoint
acquisition, interface inspection, address/port semantics, sockets, networking,
reachability, authorization, activation, and Manifold authority remain separate.

## LSLC-001U runtime-owner boundary

`stream_info_runtime_provider` keeps four same-owner values atomic at the
provider evidence boundary: one call, one witness, four opaque allocations.
This prevents mixed provider epochs or revisions inside the runtime lane.
Complete LSLC-001S admission and every platform acquisition mechanism remain
separate.

## LSLC-001T implementation-version provider boundary

`stream_info_implementation_version_provider` is an explicitly invoked edge
adapter for one implementation-owned value. Output contains separate opaque
version data and owner-issued provider identity/epoch/revision evidence. The
adapter calls the caller-selected provider once, requires exact evidence match,
applies the LSLC-001O implementation text bound, and can move only the version
into an LSLC-001S lane value. LSLC-001S still owns complete three-lane
admission. No clock, registry, runtime lane, transport lane, ambient inspection,
socket, activation, or Manifold authority enters.

The `stream_info_volatile_snapshot` module is a one-shot candidate-to-accepted
composition layer above LSLC-001O. Its three lanes preserve implementation,
runtime, and transport ownership without implementing any provider or claiming
freshness; actual acquisition remains a later owner-adapter concern.

## LSLC-001R observation-bound document representation

`stream_info_observed_document` is a specialized borrowed representation layer
above accepted LSLC-001Q. It owns only the observed declaration, LF/tab layout,
empty-desc spelling, and final LF. A bounded iterative frame table supplies
depth-first traversal; checked exact length precedes limit rejection and exact
output allocation. Childless non-desc containers reject as outside the
observed structural domain.

LSLC-001G remains the generic compact explicit-tag serializer. Neither layer
owns parsing, endpoints, providers, transport, runtime, or authority.

## LSLC-001Q ordered element composition layer

`stream_info_ordered_xml` is a consuming structural layer above the accepted
LSLC-001N and LSLC-001P element trees. It validates their fixed `info` shapes,
shares the static component root, inserts the eleven volatile leaves after the
six static leaves, and moves `desc` plus its descendants after them. Only
description-internal parents are offset by eleven before final delegation to
`XmlElementTree`; values are neither cloned nor re-represented.

This layer owns no complete-document spelling and no observation, semantic
data acquisition, provider, protocol, transport, runtime, or authority role.

## LSLC-001P volatile XML representation layer

`stream_info_volatile_xml` is a one-way borrowed projection from accepted
LSLC-001O data into an owned `info` element tree. It applies explicit existing
name, text, represented-byte, and tree limits, checks the twelve-node target
bound before allocation, and retains fixed node-indexed delegated errors.
Provider acquisition and complete-document representation remain separate.

## LSLC-001O volatile accepted-data layer

`stream_info_volatile_fields` is a dependency-free data layer below any XML
representation or provider. It retains eleven opaque caller-owned strings in
the LSLC-001H observed order. `StreamInfoVolatileFieldRole` supplies the fixed
inventory, while `StreamInfoVolatileFieldClass` separates implementation-
assigned version data, runtime-assigned creation/identity/session/host data,
and transport-owned address/port data.

`StreamInfoVolatileFieldLimits` validates three nonzero Unicode scalar maxima
in class order. `StreamInfoVolatileFields` then validates the complete input in
role order and owns only those limits plus the unchanged input. It performs no
allocation, clone, normalization, parsing, inference, provider acquisition,
clock or host read, identity generation, address or port interpretation, XML
operation, transport, runtime activation, or authority action.

## LSLC-001N description composition

`stream_info_description_xml` is a consuming arena merge above LSLC-001M and a
separate LSLC-001F projection. Admission requires an already accepted
container root named exactly `desc`; this keeps generic metadata semantics with
the caller and prevents ambient reinterpretation. The merged arena preserves
values and allocations and delegates final structural bounds to
`XmlElementTree`. No runtime or complete-document owner is introduced.

## LSLC-001M static XML composition

`stream_info_static_xml` is a leaf composition layer above the accepted static
semantic and lexical projections and below any description, volatile-field, or
complete-document policy. `StreamInfoStaticXml::compose` first closes the
LSLC-001L numeric domain, then reserves exactly seven nodes and delegates fixed
names, copied logical text, represented character data, and hierarchy checks
to the accepted XML contracts. Accepted state owns only its explicit limits
and `XmlElementTree`; it retains no runtime or authority handle.

## LSLC-001H black-box observation boundary

The oracle is a repository tool, never a production dependency or provider.
Its PowerShell layer owns the explicit external root, exact wheel acquisition,
hash and architecture checks, dependency-free isolated installation, bounded
process capture, and append-only failure history. Its Python layer imports the
pinned distribution only at capture time, calls documented public
`StreamInfo` and metadata-element APIs, takes two bounded `as_xml()`
snapshots of each unchanged object, and verifies exact repeat identity.

Raw XML, stderr, wheel, DLL, environment, and cache files remain external.
Only public-safe XML enters the append-only overlay. Ten runtime or
machine-specific text ranges are replaced by byte position; markup, whitespace,
core character data, numeric/format spelling, and description structure are
otherwise unchanged. This observation plane neither feeds the LSLC-001G local
serializer nor opens a StreamDefinition mapping, endpoint, runtime, provider,
adapter, discovery, networking, inlet, outlet, or Manifold authority plane.

## LSLC-001G element-tree serialization

The private `xml_element_serialization` module borrows one accepted
`XmlElementTree` and derives one owned UTF-8 `String` under a separate explicit
nonzero output-byte maximum. A checked length pass accounts for two copies of
each accepted name, five tag-punctuation bytes per node, and leaf character
data. Limit rejection occurs before allocation, followed by one exact fallible
reserve for a node-count-bounded traversal-frame stack and one exact fallible reserve
for the output string.

One forward pass over the accepted parent indexes records each direct-child and
next-sibling link in that stack. The serializer then performs depth-first linear
traversal without recursion, emitting siblings by ascending original arena
index. Start and end tags are always explicit, no whitespace is inserted, and
accepted character data is copied verbatim. The module neither consumes nor
mutates the source and owns no parsing, decoding, document, stream-info, LSL
mapping, endpoint, protocol, wire, I/O, runtime, adapter, provider, or authority
behavior.

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

The focused private `xml_leaf_element` module adds only a leaf-only composition
over one accepted `XmlElementName` and one accepted `XmlCharacterData`.
`XmlLeafElement` owns exactly those two private components. Its infallible
constructor moves both directly, borrowed access returns each unchanged, and
`into_parts` returns both with their existing limits, contents, and string
allocations preserved. Validation remains owned by LSLC-001B and representation
remains owned by LSLC-001C.

This composition assigns no tag spelling, namespace meaning, tree position,
document role, metadata-tree meaning, or stream-info mapping. It adds no raw
string or byte entrypoint, allocation, error or limit family, attributes,
children, mixed content, roots, parser, serializer, protocol, wire, transport,
runtime, or compatibility behavior.

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

The private `xml_element_tree` module is a bounded parent-before-child arena
over accepted component values. Accepted state contains exactly its limits and
the original caller node `Vec`; it allocates no replacement arena. One private
scratch `Vec` is fallibly reserved before per-node validation and iteratively
tracks root-one depth and child counts. Checked retained bytes cover only owned
container names, leaf names, and represented character data.

This arena is not recursive public ownership, a general DOM, mixed content, a
document, serialization order, raw-byte output, `MetadataTree` conversion, or
an LSL `info`, `desc`, or stream-info projection.

## Dependency direction

LSLC-002O is an explicit pure numeric adapter from existing raw timestamp plus
finite offset values to the existing derived timestamp label. It owns no
provider, policy, history, automatic post-processing, or activation edge.


LSLC-002N consumes only already evaluated M results. Its single bounded scan
retains the input vector and records an index; acquisition, scheduling,
history, correction, and runtime layers remain absent.


LSLC-002M is an allocation-free numeric data/formula leaf. Raw finite values
are admitted separately from fallible arithmetic results. No provider,
packet, clock, scheduling, filter, mapping, or activation edge is introduced.


LSLC-002K is a data-plane proposal composition from the accepted query wire
and one caller-selected documented destination label. It has no control-plane
selection policy and no runtime/effect edge; any socket adoption remains a
separate closed activation unit.


LSLC-002J adds a closed data-only leaf beside the existing protocol candidates.
It returns static source spellings and a numeric documented port; it has no
dependency on address, socket, interface, discovery, clock, or runtime layers.
Those later layers may consume this evidence-backed candidate only through
separately reviewed contracts and activation units.


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

## LSLC-001F metadata XML projection

The private `metadata_xml_projection` module is a one-way adapter between the
accepted CORE-004 arena and LSLC-001B through LSLC-001E values. Its sole public
entry point consumes `MetadataTree` plus four explicit accepted limit values.
It checks target node count, scans for the first child of a value-bearing
parent, reserves one distinct output arena exactly, projects names and present
values in caller order, and delegates the completed arena unchanged to
`XmlElementTree`.

Absent values become containers; present values, including empty strings,
become leaves. Name allocations move into accepted XML names, while represented
character data owns the separate LSLC-001C allocation. The module owns no
reverse conversion, decoding, defaults, mutable XML state, document or
serialization behavior, LSL mapping, protocol, wire, or runtime authority.

## LSLC-001L static numeric lexical projection

The private `stream_info_static_numeric_spellings` module retains one borrowed
`StreamInfoStaticFields` reference and two private owned strings. Construction
first selects nominal-rate text from the closed observed policy, before either
allocation, then converts the channel count through a fixed 20-byte stack
buffer. The irregular form and each of the five bit-exact accepted regular
forms select one 17-byte spelling. Any other regular bits reject before either
allocation. Each accepted output performs one exact fallible reserve; accepted
state exposes only borrowed channel-count and nominal-rate text.

The source remains unchanged. There is no generic float formatter,
exponent/locale/rounding policy, XML node or document ownership, `desc`
mapping, volatile-field surface, protocol, wire, runtime, adapter, provider,
device, feature, effect, or authority behavior.

## LSLC-001K static semantic projection

The private `stream_info_static_fields` module is an allocation-free borrowed
view over one accepted `StreamDefinition`. It owns no copied descriptor or
metadata state. Original option and nominal-rate forms remain observable beside
their explicit effective views, and channel-format spelling is a total mapping
over the existing seven variants. Borrowed extended metadata remains generic;
the adapter assigns no XML, `desc`, document, runtime, transport, or authority
role.
## LSLC-002S bounded TCP connection setup

The stream-handshake feature owns one call-scoped listener or connector. The
caller owns endpoint selection and identity input; the selected lock plus exact
runtime marker opens the effect. Header allocation, each blocking slice, the
total call, cancellation observation, and socket lifetime are finite. Accepted
request and response headers do not open sample, clock, queue, recovery, or
authority planes.
## LSLC-002T one-record data plane

An internal continuation retains the already admitted handshake socket only
inside the composed call. One fixed-size record carries a marker, little-endian
finite raw timestamp, and one `float32` value. The public handshake-only calls
still close immediately; the composed sample calls also close after exactly one
record. Sample I/O has its own finite slice/deadline and cancellation checks.
## LSLC-002U explicit clock data plane

One synchronous call owns one UDP socket and an explicit exchange count. The
caller owns bind/peer selection and the clock provider/domain. Each admitted
response echoes the outstanding identifier and `t0`; the caller supplies `t3`
after receipt. Existing M/N/O contracts retain formula, selection, and mapping
authority. No periodic worker, offset history, drift, or smoothing is opened.
## Bounded sample queue

The LSLC-002V queue is a caller-owned data-plane buffer. A nonzero capacity is
reserved before exposure. Immediate operations report full or empty; blocking
operations poll cancellation within an explicit finite slice and total
deadline. Close wakes all callers and permits buffered FIFO drain. Thread
creation and recovery policy remain caller concerns and outside this unit.
# LSLC-002Z responder ownership

The caller owns activation, bind address, finite limits, accepted document,
and cancellation. The responder owns one socket for one synchronous call,
admits canonical queries, derives the response identifier from that query, and
drops the socket on every return path. It acquires no endpoint-selection or
ambient authority.
# LSLC-003B

One closed value enum owns the four observed widths. Explicit activation,
caller identity/listener or peer, finite deadlines, cancellation, and
scope-owned sockets bound the data-plane call.
