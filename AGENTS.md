# Rusty LSL Agent Notes

LSLC-003P adds only a selected, run-disabled, finite IPv4-loopback composition
for exactly two homogeneous channels and exactly three ordered caller records
in double64, int32, int16, or int8. It preserves the LSLC-003O-observed
initialization, channel/record order, timestamps, and values. It does not
generalize counts, add formats, activate a feature, claim non-loopback or broad
compatibility, use devices, or grant runtime or Manifold authority.

## Work-Unit History

Chronological LSLC unit notes are preserved byte-for-byte in
[LSLC Work-Unit History](docs/history/LSLC-WORK-UNIT-HISTORY.md). They are
historical evidence and focused-check routing; the durable instructions below
govern current work.

Rusty LSL is a public Rusty Morphospace repository for an independently
authored Rust implementation of Lab Streaming Layer compatibility. Keep every
committed file portable, public-safe, and free of private paths, product names,
device identities, raw captures, credentials, signing material, or local
planning history.

Project-owned source is licensed `AGPL-3.0-or-later`.

## Purpose

Rusty LSL owns:

- backend-neutral Rust APIs for LSL-compatible metadata, discovery, samples,
  clocks, buffering, cancellation, recovery, and provider health;
- independently authored LSL protocol and runtime behavior;
- compatibility fixtures and differential evidence against official liblsl;
- observation and proposal hooks that deeper Rusty Morphospace adapters can
  consume.

Rusty LSL does not own:

- Manifold stream admission, registry revisions, subscriptions, routes, leases,
  provider epochs, authorization, or audit;
- Morphospace-native sample transport or generic stream authority;
- Quest networking, permissions, packaging, Android lifecycle, or device
  resources;
- Hostess orchestration, application policy, recording policy, or runtime
  defaults;
- commands derived directly from inbound LSL samples.

Morphospace hooks stop at typed observations and proposals. The accepting
adapter and authority remain in their owning repositories.

## Read Order

1. `README.md`
2. `morphospace/project.spec.json`
3. `morphospace/feature.lock.json`
4. `morphospace/workspace.state.json`
5. the current iteration unit, if one is named by workspace state
6. `docs/ARCHITECTURE.md`
7. `docs/COMPATIBILITY.md`
8. `docs/PROVENANCE.md`
9. `docs/VALIDATION.md`

The project-local workflow is planning and composition state, not LSL runtime
or compatibility authority. The accepted STRM-000 baseline remains historical
specification-only evidence: its planned observations are not measured and its
results remain `not-implemented`. LSLC-001A adds only an independently authored,
provenance-locked public-documentation corpus for documented stream-info
document roles and XML 1.0 character constraints. Every LSLC-001A oracle
observation and candidate result remains `not-observed` with null evidence;
exact serialization remains unresolved for a separately approved black-box
unit. LSLC-001B adds only dependency-free bounded XML 1.0 Fifth Edition legal
text and element-name value contracts. It preserves caller strings unchanged,
including representation-sensitive delimiters, and adds no escaping, parsing,
serialization, document, LSL field-mapping, protocol, wire, or runtime behavior.
LSLC-001C adds only a dependency-free bounded character-data representation
over borrowed accepted `XmlText`. Its fixed local candidate policy emits `&`,
`<`, and `>` as `&amp;`, `&lt;`, and `&gt;`, respectively, while preserving every
other legal scalar unchanged. This policy is not observed liblsl behavior and
adds no element, attribute, document, parser, LSL mapping, protocol, wire, or
runtime behavior.
LSLC-001D adds only an infallible dependency-free leaf-only composition that
moves one accepted `XmlElementName` and one accepted `XmlCharacterData` into
private two-component state. Borrowed and consuming access preserves both
components and their owned string allocations unchanged. It adds no tag
spelling, tree, document, raw-byte, parser, serializer, stream-info mapping,
protocol, wire, compatibility, or runtime behavior.
LSLC-001F adds only a dependency-free consuming one-way projection from one
accepted generic `MetadataTree` into one accepted `XmlElementTree`. `None`
classifies as a container and every `Some`, including `Some("")`, classifies
as a leaf under explicit caller-selected limits. This is local candidate
policy, not decoding, round-trip, document, stream-info, LSL field-mapping,
endpoint, compatibility, protocol, wire, or runtime behavior.
CORE-001 opens only dependency-free local
Rust contract semantics for bounded metadata and sample shape. CORE-002 adds
only finite raw source timestamps, separately typed optional derived timestamp
values, timestamped samples, and bounded chunks. CORE-003 adds only bounded
core stream descriptors, explicit nominal-rate values, and seven data-only
channel-format names. CORE-004 adds only a dependency-free parent-before-child
flat metadata-tree arena with explicit structural and Unicode scalar-value
bounds. CORE-005 adds only a dependency-free descriptor/sample binding for
exactly seven homogeneous data representations, exact descriptor format and
channel-shape checks, and bounded String channel values. CORE-006 adds only a
separate dependency-free timestamped descriptor/sample composition for those
same seven representations, delegating all sample validation to CORE-005 while
retaining raw and optional derived timestamp evidence unchanged. CORE-007 adds
only a dependency-free non-empty timestamped descriptor/chunk composition for
those same seven representations, retaining the original chunk limits and
delegating every ordered sample through CORE-006 with indexed unchanged errors.
CORE-008 adds only an infallible dependency-free composition that moves one
already validated `StreamDescriptor` and one already validated generic
`MetadataTree` into private accepted state with borrowed and consuming access.
Keep the feature lock empty and inert until a later reviewed unit and
owner-issued descriptor open an exact runtime surface.

## Provenance And Compatibility

- Do not copy or translate liblsl or rLSL source.
- Do not use rLSL source as an implementation input.
- Official liblsl is an MIT-licensed compatibility oracle and reference
  endpoint, not a source template.
- Keep specification, planned observation, and measured result separate. For
  the STRM-000 baseline every current result is `not-implemented` and every
  measured observation is absent.
- Record every fixture or observation as independently authored, generated,
  black-box observed, adapted, or copied. Copied material requires an explicit
  license and notice review.
- Do not claim clean-room implementation, wire compatibility, ecosystem
  compatibility, or runtime support without the named process and evidence.
- LSLC-001A public-documentation cases keep specification, oracle observation,
  and candidate result separate. Its bounds are Rusty LSL test policy, not
  liblsl limits, and it implements no XML behavior.
- Keep official native libraries and wrappers outside the default production
  dependency closure.
- LSLC-003O records only sanitized black-box evidence for two channels and
  three ordered caller records across four already observed numeric formats.
  Its private raw runs, requests, endpoints, diagnostics, environment, and
  binary stay outside the repository; the evidence adds no runtime code,
  activation, String/int64 behavior, damaged-peer policy, or broad compatibility.
- The repository is source-only. Its local constructors have no runtime,
  package, permission, network, authority, or feature-activation effect.

## Architecture Rules

- Start with one `std`-only facade crate. Split protocol, runtime, testkit,
  oracle, or C-ABI crates only when a reviewed ownership boundary requires it.
- Keep `unsafe_code = "forbid"` until a separately reviewed FFI or platform
  adapter demonstrates a need.
- LSLC-001O keeps the eleven volatile values as opaque caller-owned text under
  three explicit class bounds. The role inventory and class mapping are data
  contracts only; they do not confer provider, representation, endpoint,
  runtime, identity, transport, security, recovery, or authority meaning.
- LSLC-001Q consumes only accepted N and P trees. It validates their fixed
  shapes, shares the `info` root, retains six static and eleven volatile leaves
  before `desc`, offsets only description-internal parents by eleven, and
  delegates all final hierarchy bounds to `XmlElementTree`.
- LSLC-001Q is local element composition only. It does not own a declaration,
  observed whitespace or self-closing policy, complete-document bytes,
  provider acquisition, runtime values, transport, activation, or authority.
- LSLC-001R borrows accepted Q state and owns only the H-observed declaration,
  LF/tab layout, empty fixed `desc` spelling, and final LF. Other childless
  containers reject as unobserved rather than inheriting that spelling.
- LSLC-001R does not modify or generalize LSLC-001G. Its owned string is local
  observation-bound candidate evidence, not endpoint, wire, provider, runtime,
  transport, device, feature, or authority proof.
- LSLC-001B uses separate nonzero Unicode scalar-value maxima for XML text and
  element names. Text accepts exactly the XML 1.0 Fifth Edition `Char`
  production; names accept the complete `NameStartChar` and `NameChar`
  productions. Accepted strings and allocations remain unchanged behind
  private fields with borrowed and consuming access.
- XML text length rejects before its first indexed illegal scalar. Element-name
  rejection order is empty, length, invalid start, then first invalid
  continuation. Colon is syntax only and grants no namespace interpretation.
- LSLC-001B accepts ampersand, less-than, greater-than, and `]]>` as caller
  values. It owns no representation policy, escaping, entity selection, CDATA
  handling, parsing, serialization, byte output, document assembly, attributes,
  namespaces, schemas, queries, or canonicalization.
- LSLC-001C borrows an already validated `XmlText` without consuming,
  mutating, reinterpreting, or revalidating it. A separate nonzero maximum
  counts encoded UTF-8 bytes. Exact checked length precedes limit rejection,
  which precedes a non-panicking fallible reserve; typed errors retain
  `LengthOverflow`, exact expected/required bounds, or the requested allocation.
- Character-data accepted state is private and exposes only its limit,
  borrowed encoded text, and consuming allocation-preserving `String` access.
  Quotes and apostrophes remain literal. No generic entity engine, CDATA
  section, decoder, document assembly, or exact endpoint representation is
  implied.
- LSLC-001D accepts only the existing `XmlElementName` and `XmlCharacterData`
  types. Its infallible constructor moves them directly without cloning,
  allocation, validation, re-encoding, normalization, or interpretation.
- `XmlLeafElement` has exactly two private fields and exposes only borrowed
  `name` and `character_data` access plus allocation-preserving `into_parts`.
  Colon remains syntax only, and existing greater-than escaping remains
  LSLC-001C local candidate policy rather than observed liblsl behavior.
- LSLC-001D adds no raw-string entrypoint, limits, errors, tag spelling,
  attributes, children, mixed content, trees, roots, documents, namespaces,
  raw bytes, parsing, serialization, or LSL field mapping.
- LSLC-001E accepts one root at index zero and requires every later node to
  name a strictly earlier container parent. Leaves cannot parent children.
  Four nonzero maxima bound nodes, root-one depth, direct children per
  container, and retained UTF-8 bytes across owned container names, leaf names,
  and represented character data. Retained bytes are an arena resource bound,
  not serialized or wire size.
- Hierarchy rejection order is empty arena, node bound, root-parent shape, one
  fallible scratch reservation, then each later node in caller order for
  parent identity, parent kind, depth, and child bound, followed by checked
  retained-byte calculation and its bound. Failures are typed and non-panicking.
- Accepted `XmlElementTree` state owns only its limits and the original
  candidate-node `Vec`. Owning candidate node, value, and tree types are not
  `Clone` and expose no mutable access. The hierarchy grants no mixed-content,
  complete-document, tag-spelling, serialization, `MetadataTree`, stream-info,
  `info`, `desc`, protocol, wire, compatibility, or runtime meaning.
- LSLC-001F rejects the target node bound first, then the first child in caller
  order whose parent has a value, then fallibly reserves one exact distinct
  output arena. It projects nodes in order through XML name validation,
  optional text validation, character-data representation, and unchanged
  `XmlElementTree` delegation.
- The projection consumes the source without cloning. It preserves name
  allocations and parent/order identity, while accepted character data owns
  the separate LSLC-001C represented-string allocation. It exposes no borrowed
  or reverse projection, `From`/`TryFrom`, default limits, decoder, mutable XML
  ownership, source recovery, or round-trip claim.
- Keep metadata, frames, channel counts, chunks, queues, timeouts, retries, and
  retained ranges explicitly bounded.
- CORE-001 constructors validate complete inputs before returning a value,
  reject invalid zero limit configurations, preserve accepted caller values,
  and report stable expected/actual error payloads.
- CORE-002 preserves every accepted raw source timestamp bit-for-bit beside any
  separately labelled derived value. `ClockCorrected` and `Smoothed` are
  caller-provided classifications only, not implemented algorithms. It rejects
  non-finite timestamps, invalid chunk maxima, one-past maxima, and inconsistent
  channel shapes atomically. An empty chunk is valid under nonzero maxima.
- CORE-003 requires a nonempty stream name and explicit nonzero Unicode scalar
  and channel maxima. Optional content type and source correlation are bounded
  opaque text preserved exactly. Source correlation is never runtime identity,
  discovery, recovery, authorization, routing, permission, admission, or
  Morphospace/Manifold authority.
- A regular nominal sample rate must be finite and positive and preserves its
  accepted floating-point bits; irregular is a separate explicit form. These
  values do not read clocks, measure, schedule, enforce, interpolate, or derive
  rates.
- `ChannelFormat` has exactly seven data-only variants: `Float32`, `Double64`,
  `String`, `Int32`, `Int16`, `Int8`, and `Int64`. They have no wire numeric
  discriminants and perform no byte sizing, encoding, decoding, or conversion.
- CORE-004 accepts exactly one root at index zero and requires every later node
  to name a strictly earlier parent. Root depth is one. Total nodes, depth,
  direct children per node, name Unicode scalar values, and optional value
  Unicode scalar values have explicit nonzero maxima.
- Metadata-tree names are required and nonempty. Optional values preserve
  `None` versus `Some("")`; accepted node order, parent indices, names, and
  optional values are retained exactly behind private fields and read-only
  accessors. Validation and storage use a flat arena with no recursive public
  ownership or recursive validation/traversal.
- CORE-004 implements no XML syntax, parsing, serialization, escaping,
  namespaces, attributes, entities, schemas, queries, protocol behavior,
  discovery, runtime, transport, or tree mutation.
- CORE-005 reuses validated `StreamDescriptor` and `Sample<T>` values. Its
  accepted state privately retains only the exact descriptor channel count and
  data-only format plus the unchanged owned sample; it does not copy the full
  descriptor into each binding.
- Descriptor/sample construction checks format, then channel count, then String
  values in zero-based channel order. Its nonzero String maximum counts Unicode
  scalar values, accepts empty strings, and reports the first oversized channel
  with stable expected/actual counts. Numeric values retain order and exact
  bits, including signed zero and NaN payloads.
- CORE-005 performs no conversion, casting, parsing, formatting, normalization,
  inference, byte sizing, encoding, decoding, endianness, wire mapping,
  allocation beyond owned contract state, or runtime action.
- CORE-006 moves one existing `TimestampedSample<T>` apart without cloning or
  recalculation, delegates its sample unchanged to `BoundDescriptorSample::new`,
  and privately retains the accepted binding plus the exact raw source and
  optional derived timestamp evidence. Its public input family has exactly the
  same seven type-to-format mappings as CORE-005.
- CORE-006 adds no timestamp algorithm, clock read, correction, smoothing,
  dejittering, interpolation, sorting, rewriting, scheduling, buffering,
  conversion, encoding, transport, protocol, wire, or runtime action.
- CORE-007 rejects an empty existing `TimestampedChunk<T>` before delegation,
  then moves each sample in caller order exactly once through
  `BoundTimestampedDescriptorSample::new`. Accepted private state contains only
  the original `ChunkLimits` and the ordered accepted sample bindings.
- CORE-007 reports the zero-based first failing sample and unchanged
  `DescriptorSampleError`. It duplicates no CORE-005/006 format, channel-count,
  String-bound, or timestamp validation and performs no splitting, merging,
  rechunking, sorting, rewriting, buffering, queueing, scheduling, conversion,
  encoding, transport, protocol, wire, or runtime action.
- CORE-008 `StreamDefinition` privately owns exactly one existing descriptor
  and one existing metadata tree. Construction moves both directly, performs
  no allocation, clone, validation, normalization, inference, or interpretation,
  and exposes only `descriptor`, `extended_metadata`, and consuming `into_parts`
  access.
- CORE-008 does not make the generic metadata-tree root an LSL `desc` element
  and adds no XML/document shape, channel conventions, runtime identity,
  discovery, protocol, transport, provider, adapter, or authority behavior.
- Timestamp value constructors do not read clocks or calculate correction,
  dejittering, smoothing, interpolation, or sample-rate timestamp derivation.
- Discovery is observation, never identity, authorization, or activation.
- No inbound sample may apply a command directly.
- No high-rate media belongs in the generic LSL sample path.
- Provider fallback is explicit and preserves the failed candidate evidence.

## Worktree And Agent Policy

Use one writer per branch and worktree. Account-specific or delegated work must
use a dedicated linked worktree and a `codex/*` branch. The main checkout is
the integration and review surface; delegated agents must not write there.

A handoff records the baseline commit, branch, allowed paths, non-scope,
commands run, results, unresolved risks, and rollback point.

## Validation

The sole current policy authority is `tools/validation-policy.json`; CI and
local wrappers must not carry independent gate inventories. Use the facade:

```text
python ./tools/dispatch_validation.py --profile quick
python ./tools/dispatch_validation.py --profile standard
python ./tools/dispatch_validation.py --profile deep
```

`tools/check_all.ps1` remains the compatibility wrapper for `standard`.
`docs/VALIDATION.md` routes the policy, exact historical guide, inventory, and
ADR. Receipts are execution evidence, never policy authority.

Every future unit declares validation impact as `none` with a specific
justification, `implementation-only`, or `policy`. Unit-specific gates become
pinned historical evidence after acceptance; durable invariants move to stable
owner-named current gate IDs. Profile, claim, limitation, or state changes
require an explicit machine-readable semantic delta. The policy's
`does_not_prove` fields remain binding evidence limits.
