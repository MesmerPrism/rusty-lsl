# Compatibility

LSLC-002E observed two synthetic loopback short-info responses from the pinned
official protocol-110 endpoint. Both shared `decimal query id + CRLF + observed
document`, and both bodies passed LSLC-002A/002B. This is bounded black-box
shape evidence, not discovery, endpoint, ecosystem, or interoperability proof.

LSLC-002D infers CRLF solely from the official document's logged 65-byte count
and its 59 displayed non-delimiter bytes. This resolves LSLC-002C's internal
62-versus-65-byte inconsistency but still does not prove endpoint or discovery
interoperability because no external endpoint or implementation was exercised.

LSLC-002C is a local candidate contract informed by the official public
network-connectivity documentation's displayed short-info query. Valid,
damaged, truncated, oversized, and noncanonical fixtures test Rusty LSL's
explicit bounds and spelling policy. No external endpoint was queried and no
response was observed, so this evidence establishes neither protocol-110 nor
discovery interoperability.

## LSLC-002B scope

The public-safe matrix binds local typed admission only. It makes no
protocol-110, endpoint, discovery, transport, or ecosystem compatibility claim.

## LSLC-002A bounded observed-document shape

LSLC-002A independently specifies a strict candidate parser for the exact
accepted LSLC-001R empty-description bytes. Public-safe fixtures cover valid,
damaged, truncated, oversized, non-canonical, malformed-closing, and invalid
character-data cases. This evidence is local shape acceptance only and makes
no broad XML, liblsl, endpoint, protocol, wire, ecosystem, runtime, discovery,
or network compatibility claim.

LSLC-001Z proves only that existing accepted N and X values compose through the
already accepted P/Q/R local policies. It does not prove endpoint bytes, wire or
ecosystem interoperability, freshness, reachability, authorization, or runtime
behavior.

LSLC-001X proves only local composition of accepted T/U/V values into one S
snapshot while separately retaining their three witnesses and preserving value
allocations. It does not prove common acquisition time, cross-owner atomicity,
freshness/currentness, endpoint operation, runtime behavior, protocol/wire
behavior, official behavior, or ecosystem compatibility.

LSLC-001V proves only local one-call grouping, exact shared transport-owner
witness matching, O-bound ordering, typed failures, allocation preservation,
and S transport-lane projection. It does not prove endpoint syntax, interface
origin, reachability, authorization, official behavior, protocol, wire,
networking, transport operation, runtime behavior, or ecosystem compatibility.

LSLC-001U proves only local one-call grouping, exact shared owner-witness
matching, O-bound ordering, typed failure, and S runtime-lane projection. It
does not prove clock format, UID uniqueness, session meaning, hostname origin,
freshness beyond owner evidence, official behavior, or runtime compatibility.

## LSLC-001T local acquisition evidence

LSLC-001T proves only the local one-call provider interface, exact owner-witness
matching, bounded opaque version preservation, typed failures, and single-value
projection for later LSLC-001S composition. It does not identify an official
implementation version, prove freshness beyond the owner's explicit witness,
or establish endpoint, protocol, wire, runtime, transport, discovery, or
ecosystem compatibility.

LSLC-001S proves only deterministic local admission of a complete, unique,
correct-lane caller snapshot into LSLC-001O. It is not evidence of fresh
provider data, platform acquisition, endpoint meaning, or runtime compatibility.

## LSLC-001R local observed-envelope result

LSLC-001R matches all seven normalized LSLC-001H public XML strings exactly
through the accepted local Q tree, including declaration, LF/tab layout,
empty-desc form, nested description indentation, and final LF. This is bounded
candidate evidence for the observed dimensions only; it does not claim raw
endpoint bytes, wire interoperability, providers, or runtime behavior.

## LSLC-001Q local ordered-element result

LSLC-001Q executes all seven accepted case shapes through the accepted N and P
components and obtains the static, volatile, then `desc` element order observed
in LSLC-001H. The evidence proves only the compact LSLC-001G local serialization
of that hierarchy. It does not reproduce the observed declaration, whitespace,
self-closing spelling, endpoint bytes, provider values, or runtime behavior.

## LSLC-001P local candidate representation

LSLC-001P implements only the compact local volatile `info` element tree. Its
eleven leaves follow the accepted LSLC-001H order and LSLC-001C representation
policy. It does not merge static or description content or claim observed
declaration, whitespace, self-closing spelling, endpoint bytes, or complete
document compatibility.

## LSLC-001O local candidate data result

LSLC-001O binds its eleven-role inventory to the accepted LSLC-001H observed
field order and implements only bounded opaque data retention. Its ownership
classes prevent runtime-assigned identity/time/session/host data from being
conflated with transport-owned address/port data. No provider or XML/document
candidate is implemented, and the LSLC-001H complete-document candidate result
remains `not-observed`.

## LSLC-001N local candidate result

LSLC-001N executes the seven accepted case shapes through static composition,
metadata projection, description merge, and compact serialization. The nested
description order and represented values match the accepted observation, while
empty descriptions remain explicit `<desc></desc>` under local LSLC-001G
policy. This is not an observed self-closing, whitespace, volatile-field, or
complete-document compatibility result; LSLC-001H remains `not-observed` for
the candidate document.

## LSLC-001M local candidate result

LSLC-001M executes all seven accepted black-box case inputs through the local
static composition and compares each of the six represented values with the
accepted observation. Its compact serialization is candidate evidence under
the unchanged LSLC-001G policy, not a match claim for the observed declaration,
whitespace, `desc`, volatile fields, or complete `info` document. The accepted
LSLC-001H full-document candidate result remains `not-observed`.

## LSLC-001H measured serialization observation

`fixtures/compatibility/lslc-001h-stream-info-xml-observations.json` is a
separate black-box-observed overlay for one exact official Windows oracle. It
binds, but does not edit or promote, the frozen LSLC-001A specification corpus.
It covers all seven data-only formats, irregular and finite regular rates,
empty/populated optional core text, Unicode and XML-sensitive caller text,
empty `desc`, and ordered nested metadata.

The overlay records exact normalized public XML, raw-output SHA-256 values,
repeat identity, byte-positioned normalization operations, and the observed
order, whitespace, empty-element, numeric, format, character-data, and
description-placement dimensions. It is not candidate differential evidence
and does not support claims about other versions/platforms, protocol, wire,
networking, runtime, or ecosystem compatibility.

## LSLC-001G local serialization evidence

LSLC-001G proves only a borrowed, bounded, iterative projection of an accepted
local element hierarchy into a UTF-8 `String`. Explicit start/end spelling, no
inserted whitespace, depth-first traversal, and ascending arena-index sibling
order are Rusty LSL local candidate policy. Accepted LSLC-001C character data
is emitted verbatim once. Every LSLC-001A oracle observation and candidate
result remains `not-observed` with null evidence.

This evidence proves no complete XML or LSL document, parsing, decoding,
round-trip behavior, stream-info or field mapping, endpoint or official-liblsl
behavior, protocol, wire, I/O, runtime, ecosystem, or compatibility claim.

## Current evidence matrix

| Surface | Implementation status | Current evidence |
| --- | --- | --- |
| Local bounded metadata construction | Implemented | CORE-001 Rust unit tests; no XML behavior |
| Metadata XML | Not implemented | Specification cases only |
| Stream-info document corpus | Specification only | LSLC-001A public-documentation roles; oracle and candidate evidence not observed |
| Stream-info XML black-box observation | Observed for seven bounded cases | LSLC-001H pinned Windows binary observation; separate complete-document candidate remains not observed |
| Local static and description stream-info element composition | Implemented | LSLC-001M/N compact local element trees only; no declaration, observed whitespace, volatile fields, or complete document |
| Local volatile stream-info data | Implemented | LSLC-001O bounded opaque values with explicit implementation/runtime/transport classes; no provider, XML, parsing, endpoint, or runtime behavior |
| Local volatile stream-info XML and ordered element composition | Implemented | LSLC-001P/Q compact local element trees; exact static, volatile, then `desc` order without declaration, observed whitespace, provider, or complete-document behavior |
| Local observed stream-info document envelope | Implemented | LSLC-001R seven normalized H strings; separate from compact G and without endpoint, wire, provider, or runtime claims |
| Local XML legal-text and element-name values | Implemented | LSLC-001B Rust unit tests; bounded scalar/name validation only, with no representation or document behavior |
| Local XML character-data representation | Implemented | LSLC-001C Rust unit tests; bounded candidate-owned `&`, `<`, and `>` replacement only, with no document or endpoint-byte claim |
| Local XML leaf-only composition | Implemented | LSLC-001D Rust unit tests; exact accepted name plus character-data ownership only, with no tag, tree, document, mapping, or endpoint-byte claim |
| Local XML container/leaf hierarchy | Implemented | LSLC-001E Rust unit tests; bounded caller-owned parent-before-child accepted-component arena only, with no complete-document, serialization, mapping, or endpoint-byte claim |
| Local XML element-tree string serialization | Implemented | LSLC-001G Rust unit tests; bounded borrowed local spelling and hierarchy order only, with no complete-document, LSL mapping, endpoint, oracle, wire, or compatibility claim |
| Discovery and resolution | Not implemented | No case or measurement |
| Local sample-shape construction | Implemented | CORE-001 Rust unit tests; no transport behavior |
| Local timestamp value and sample construction | Implemented | CORE-002 Rust unit tests; caller-provided finite values and derived kinds only |
| Local bounded timestamped chunks | Implemented | CORE-002 Rust unit tests, including empty bounded collections; no buffering or transport behavior |
| Local core stream descriptors | Implemented | CORE-003 Rust unit tests; bounded opaque text and values only, with no XML, discovery, or runtime identity |
| Local bounded metadata tree | Implemented | CORE-004 Rust unit tests; flat parent-before-child arena only, with no XML or runtime behavior |
| Local descriptor/sample binding | Implemented | CORE-005 Rust unit tests; exact homogeneous format and channel shape only, with bounded String values and no conversion or runtime behavior |
| Local timestamped descriptor/sample composition | Implemented | CORE-006 Rust unit tests; exact composition of existing validated values through CORE-005, with no clock, algorithm, transport, or runtime behavior |
| Local non-empty timestamped descriptor/chunk composition | Implemented | CORE-007 Rust unit tests; ordered composition through CORE-006 with original chunk limits and indexed delegated errors; no actual LSL empty-chunk compatibility claim |
| Local stream-definition composition | Implemented | CORE-008 Rust unit tests; lossless composition of existing validated descriptor and generic metadata-tree values only, with no XML interpretation or runtime behavior |
| Sample and chunk transport | Not implemented | Specification cases only |
| Local channel-format names | Implemented | CORE-003 Rust unit tests; exactly seven data-only variants with no wire numeric mapping or conversion |
| Protocol or wire channel formats | Not implemented | No case or measurement |
| Clock-sourced or protocol timestamps | Not implemented | Specification cases only |
| Clock correction and smoothing | Not implemented | No case or measurement |
| Buffering and post-processing | Not implemented | No case or measurement |
| Cancellation and timeouts | Not implemented | Damaged-case expectation only |
| Loss detection and recovery | Not implemented | Specification cases only |
| Provider health and fallback | Not implemented | Specification cases only |
| C ABI or language wrappers | Not implemented | No case or measurement |
| Wire compatibility | Not implemented and not claimed | Planned observations only |
| Operational/ecosystem compatibility | Not implemented and not claimed | Planned observations only |

The CORE-001, CORE-002, CORE-003, and CORE-004 tests prove only local Rust
contract semantics for the named constructors on the tested toolchain. CORE-002 validates caller-provided
finite values and preserves explicit `ClockCorrected` or `Smoothed`
classifications; it does not read clocks, derive timestamps, or implement those
algorithms. These tests do not prove LSL behavior, interoperability,
performance, or platform support. CORE-003 proves only bounded construction,
opaque optional text retention, explicit irregular or finite positive regular
rate values, and the seven named channel-format values. Source correlation is
not identity or authority, and no clock, XML, discovery, recovery, transport,
encoding, conversion, or runtime behavior is exercised.
CORE-004 proves only bounded flat-arena construction, exactly one root,
strictly earlier parents, iterative depth and child accounting, Unicode scalar
text bounds, and preservation of absent-versus-empty optional values. It proves
no XML, document, query, mutation, discovery, transport, protocol, wire,
runtime, or ecosystem behavior.
CORE-005 proves only local binding of seven homogeneous `Sample<T>` families to
an exact validated descriptor shape, String Unicode scalar bounds, stable
errors, and unchanged value order and floating-point bits. It proves no
conversion, layout, encoding, wire, protocol, transport, runtime, or ecosystem
behavior.
CORE-006 proves only local composition of seven homogeneous
`TimestampedSample<T>` families with an exact descriptor binding, preservation
of raw and optional derived timestamp evidence, and unchanged delegated
CORE-005 errors. It proves no clock read or algorithm, timestamp derivation or
rewriting, buffering, conversion, encoding, protocol, transport, runtime, or
ecosystem behavior.
CORE-007 proves only local non-empty composition of seven homogeneous
`TimestampedChunk<T>` families with an exact descriptor, original chunk-limit
retention, caller-order CORE-006 delegation, exact evidence pairing, and the
first failing sample index around an unchanged delegated error. It proves no
actual LSL empty-chunk behavior, clock read or algorithm, timestamp rewriting,
splitting, merging, rechunking, buffering, queueing, conversion, encoding,
protocol, transport, runtime, or ecosystem behavior.

CORE-008 proves only local infallible composition of one already validated
descriptor and one already validated generic metadata tree, including borrowed
and consuming preservation of all component values and existing allocations.
It proves no cross-component interpretation or validation, XML or LSL `desc`
meaning, channel metadata convention, runtime identity, discovery, transport,
protocol, wire, runtime, authority, or ecosystem behavior.

LSLC-001B proves only local bounded validation against the XML 1.0 Fifth
Edition `Char`, `NameStartChar`, and `NameChar` productions, stable validation
precedence, and exact caller-string retention. It does not prove escaping,
parsing, serialization, document construction, LSL field mapping, endpoint
output, protocol, wire, runtime, oracle, or ecosystem behavior.

LSLC-001C proves only that borrowed accepted `XmlText` is represented under a
bounded local policy that emits `&amp;`, `&lt;`, and `&gt;`, preserves all other legal
UTF-8 input, checks exact byte length before fallible allocation, and retains
stable typed errors and private accepted state. The global greater-than choice
is not observed liblsl behavior. These tests prove no element, attribute,
document, parser, LSL mapping, exact endpoint bytes, protocol, wire, runtime,
oracle, or ecosystem behavior.

LSLC-001D proves only that one accepted `XmlElementName` and one accepted
`XmlCharacterData` can be moved into and recovered from a private two-component
`XmlLeafElement` without changing their allocations or state. It does not
spell a tag, create a tree or document, map stream-info fields, emit bytes, or
measure official-liblsl behavior. Every LSLC-001A oracle observation and
candidate result remains `not-observed` with null evidence.

## Compatibility classes

LSLC-001A is a separate specification corpus, not a CORE overlay or an
implementation status change. Its cases cover documented stream-info document
roles, XML 1.0 character constraints, and repository-owned input bounds. Each
case has separate `specification`, `oracle_observation`, and `candidate_result`
roles. Every observation and candidate result is `not-observed` with null
evidence. Exact bytes, order, whitespace, empty-element form, numeric spelling,
and channel-format wire spelling remain unresolved for a separately approved
black-box oracle unit.

The separate `lslc-001b-contract-results.json` overlay binds local Rust tests
to the LSLC-001A legal-character, character-data, invalid-name, and bound roles
without changing any LSLC-001A oracle observation or candidate result. Those
roles remain `not-observed` with null evidence.

The separate `lslc-001c-contract-results.json` overlay binds only the accepted
LSLC-001A character-data role and the LSLC-001B validated `XmlText` contract.
It records the three fixed replacements as local candidate policy while every
LSLC-001A oracle observation and candidate result remains `not-observed` with
null evidence.

The separate `lslc-001d-contract-results.json` overlay binds local Rust tests
only to the accepted LSLC-001B element-name and LSLC-001C character-data
contracts. It preserves the LSLC-001C representation as local candidate policy
and leaves every LSLC-001A oracle observation and candidate result
`not-observed` with null evidence.

Compatibility evidence is classified at four distinct levels:

- **Contract compatibility:** Rust types and API behavior represent the named
  metadata, sample, error, timeout, and lifecycle cases.
- **Semantic-bridge compatibility:** explicit adapters preserve meaning across
  LSL observations and downstream Morphospace proposals without transferring
  authority.
- **Wire compatibility:** independently implemented peers exchange the named
  protocol cases with specified versions and negative evidence.
- **Operational/ecosystem compatibility:** documented applications, wrappers,
  platforms, recovery paths, and long-running behavior pass their named gates.

Only the named local contract slices are implemented, and only as local Rust
API behavior. No LSL protocol, wire, runtime, operational, or ecosystem
compatibility is implemented or claimed. The canonical STRM-000 catalog is
`fixtures/compatibility/behavior-catalog.json`; it remains accepted historical
specification-only evidence with at least two bounded cases for each class.
The separate `core-001-contract-results.json` overlay binds local unit tests to
the two CORE-001 case IDs. The `core-002-contract-results.json` overlay binds
local timestamp preservation to `semantic-raw-timestamp-preserved` and records
the bounded-chunk contract without turning either into a measured oracle result.
The separate `core-003-contract-results.json` overlay binds local descriptor,
nominal-rate, and channel-format tests to CORE-003 while preserving
`semantic-observation-not-authority` as `not-implemented` historical evidence.
The separate `core-004-contract-results.json` overlay binds only flat bounded
metadata-tree tests to CORE-004 while preserving `contract-metadata-bounds` as
`not-implemented` historical specification evidence.
The separate `core-005-contract-results.json` overlay binds only local
descriptor/sample binding tests to CORE-005 while preserving
`contract-sample-shape` as `not-implemented` historical specification evidence.
The separate `core-006-contract-results.json` overlay binds only local
timestamped descriptor/sample composition tests to CORE-006 while preserving
`semantic-raw-timestamp-preserved` as `not-implemented` historical
specification evidence.
The separate `core-007-contract-results.json` overlay binds only local
timestamped descriptor/chunk composition tests to CORE-007 while preserving
`contract-sample-shape` as `not-implemented` historical specification evidence.
The separate `core-008-contract-results.json` overlay binds only local
stream-definition composition tests to CORE-008 while preserving
`contract-metadata-bounds` as `not-implemented` historical specification
evidence.
Evidence at one level must not be promoted into a broader claim.

Each case has three deliberately separate roles:

- `specification` states independently authored behavior and bounds;
- `planned_observation` names a future endpoint and observable;
- `measured_result` records evidence only after a reviewed run.

For STRM-000, `current_result` and `measured_result.status` remain
`not-implemented`, and each measured observation remains null. CORE-001,
CORE-002, CORE-003, CORE-004, CORE-005, CORE-006, CORE-007, and CORE-008 status lives only in their
result overlays.

## Compatibility method

Compatibility work must proceed from an independently written behavior case,
not from a translation of another implementation. Each case must identify:

1. the observable behavior and bounded inputs;
2. an independently authored or generated valid fixture;
3. at least one malformed, oversized, stale, or interrupted case where
   relevant;
4. the Rusty LSL result, initially and currently `not-implemented`;
5. an optional black-box result from official liblsl as the oracle endpoint;
6. the exact versions, commands, platforms, and limitations of the comparison.

The damaged-case catalog is
`fixtures/compatibility/negative-case-matrix.json`. Its classifications are
expected outcomes for future tests, not measured oracle results. Cases are
bounded without embedding wire constants, native artifacts, captures, or
implementation-derived protocol bytes.

Agreement on one case supports only that case. A collection of passing cases
does not by itself establish wire or ecosystem compatibility.

LSLC-001E proves only deterministic local validation and ownership preservation
for a bounded caller-owned container/leaf arena. Retained UTF-8 bytes are a
component-storage resource count, not serialized bytes. It proves no mixed
content, complete XML/document shape, caller-order serialization, `MetadataTree`
conversion, stream-info mapping, endpoint output, official behavior, protocol,
wire, runtime, or compatibility. Every LSLC-001A oracle observation and
candidate result remains `not-observed` with null evidence.

## Oracle isolation

Official liblsl may be invoked only by explicit compatibility tooling. It must
not enter the default dependency graph, production binaries, generated source,
or implementation logic. Oracle input and normalized output may become
fixtures only after their origin, license implications, generation command,
and review are recorded in `PROVENANCE.md` or an adjacent fixture manifest.
The reproducible procedure and normalized classification vocabulary are in
`ORACLE.md`.

rLSL source is not an implementation input and must not be used to construct
tests, APIs, protocol behavior, or runtime code.

## Claim changes

A row may move from `Not implemented` only in the same change that adds the
named implementation, positive and damaged tests, provenance records, and
validation instructions. Broader labels such as wire-compatible,
ecosystem-compatible, or supported require an explicitly reviewed definition
and evidence set; they must not be inferred from build success.

## LSLC-001F local projection evidence

LSLC-001F proves only the dependency-free consuming projection from one
accepted generic metadata arena to one accepted container/leaf hierarchy.
`None` versus `Some`, represented character data, allocation ownership, and
failure precedence are Rusty LSL local candidate policy. Every LSLC-001A oracle
observation and candidate result remains `not-observed` with null evidence.

The evidence makes no reverse, decoding, round-trip, document, serialization,
stream-info, LSL field-mapping, endpoint, official-liblsl, protocol, wire,
runtime, ecosystem, or compatibility claim.

## LSLC-001L local numeric lexical evidence

LSLC-001L binds the immutable LSLC-001H observation and LSLC-001K semantic
overlays, then directly executes their seven accepted inputs in Rust. The only
candidate results are channel counts `1` through `7` and nominal-rate spellings
`0.000000000000000`, `100.0000000000000`, `59.94000000000000`,
`1.000000000000000`, `256.5000000000000`, and `1000000.250000000`.

Regular-rate acceptance is bit-exact and closed to those five observed values.
This evidence does not establish arbitrary fixed-decimal, exponent, locale,
shortest-round-trip, rounding, XML, complete-document, endpoint, protocol,
wire, runtime, or ecosystem compatibility.

## LSLC-001K local semantic-field evidence

LSLC-001K binds the seven accepted LSLC-001H inputs and observations in a
separate local-results overlay. It records only the candidate's six borrowed
static semantic fields, original option/rate forms, effective empty/zero views,
and seven format symbols. The accepted XML observations, runtime placeholders,
description hierarchy, and full-document `candidate_result` remain unchanged.
This evidence proves neither XML or numeric lexical representation nor broader
official-liblsl, wire, runtime, or ecosystem compatibility.
