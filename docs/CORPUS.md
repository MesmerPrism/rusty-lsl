# LSLC-001A stream-info document corpus

## LSLC-001H append-only observation

LSLC-001H leaves this corpus byte-for-byte frozen at SHA-256
`68331a7a5ae6d0767ae9d2eb2d317d3673595fa04352087e88d6ff1506faaa2c`.
Specification cases retain null `not-observed` oracle and candidate roles.
The new bounded synthetic cases, provenance, and measured official-oracle
results live in three separate `lslc-001h-*` artifacts. Observed endpoint
bytes therefore do not rewrite specification, LSLC-001C/G local candidate
policy, or any future candidate result.

## LSLC-001G local serialization binding

LSLC-001G borrows only the accepted LSLC-001E hierarchy and preserves accepted
LSLC-001B through LSLC-001D components. Its separate overlay records explicit
start/end tags, no inserted whitespace, depth-first traversal, ascending
arena-index sibling order, and verbatim represented character data as local
candidate policy. It does not edit this corpus: every oracle observation and
candidate result remains `not-observed` with null evidence.

The bounded string projection assigns no complete-document, stream-info,
`info`, `desc`, LSL field-mapping, endpoint, official-liblsl, parsing, decoding,
round-trip, protocol, wire, I/O, runtime, or compatibility role.

LSLC-001A is an independently authored specification corpus. It records a
small set of public documentation claims and bounded future test roles; it is
not an XML implementation, endpoint capture, serialization fixture, or
compatibility result.

## Provenance and clean-source use

The only external technical inputs are the public liblsl Stream Info API page
and the W3C XML 1.0 Fifth Edition Recommendation. The corpus records their
exact URLs, titles, publisher or authority roles, and the access date. Only
concise claim identifiers and independently worded summaries enter the
repository. No liblsl, rLSL, wrapper, application, test, build, vendored,
generated, or protocol implementation source was inspected or used. No
endpoint output, native artifact, capture, or implementation-derived byte
sequence was collected.

## Evidence roles

Every case keeps three roles separate:

- `specification` states a public-documentation or repository-owned test role;
- `oracle_observation` is reserved for a separately approved black-box run;
- `candidate_result` is reserved for later independently implemented behavior.

All LSLC-001A oracle observations and candidate results are `not-observed` with
null evidence. Historical STRM-000 results remain `not-implemented`, and the
CORE-001 through CORE-008 overlays remain separate local Rust contract
evidence.

## Taxonomy and bounds

Positive cases cover the documented `info` top-level role, the six documented
core child roles, the extended `desc` role, separation of runtime/misc roles
from caller core state, the XML legal-character domain, required character-data
handling, and the repository's bounded-input policy.

Character-data constraints apply to a future XML representation, not directly
to caller values. In particular, a caller value containing `]]>` is not declared
invalid by this corpus; a later implementation unit must choose and test a
well-formed representation policy without claiming observed liblsl behavior.

Damaged cases cover malformed documents, illegal XML characters, invalid
element names, missing required stream names, nonpositive channel counts,
duplicate singleton core roles, excessive depth, excessive node count, and
excessive text. Only the nonempty-name and positive-channel-count cases are
labelled documented stream-constructor invariants. Other damaged cases are XML
specification roles or independently selected future-harness policies, never
observed liblsl behavior.

The corpus bounds a future input to 65,536 document characters, depth 16, 256
nodes, 128 Unicode scalar values per element name, and 4,096 Unicode scalar
values of character data per node. These resource limits belong to Rusty LSL;
they are not claims about liblsl.

## Promotion gates

Exact bytes, element order, whitespace, empty-element form, numeric spelling,
and channel-format wire spelling remain unresolved. Resolving any of them
requires a separately approved clean black-box oracle unit with named endpoint
version, bounded commands, provenance, observations, limitations, and review.
Parser or serializer work requires its own implementation unit, positive and
damaged tests, and dependency/activation review. No case may be promoted to a
compatibility claim until specification, oracle evidence, and candidate result
are independently present and the claimed evidence tier is explicitly
accepted.

## LSLC-001B local contract binding

LSLC-001B consumes only the accepted legal-character and name-grammar roles as
technical specification input for dependency-free local Rust value contracts.
Its separate overlay binds tests to those roles but does not edit this corpus:
all oracle observations and candidate results remain `not-observed` with null
evidence. The value layer accepts representation-sensitive caller text
unchanged. Escaping, CDATA handling, parsing, serialization, document assembly,
LSL field mapping, exact output, and oracle comparison remain deferred.

## LSLC-001C local representation binding

LSLC-001C binds only the accepted `spec-xml-character-data-handling` role and
the LSLC-001B bounded `XmlText` value contract. Its separate overlay records a
Rusty LSL candidate policy that replaces every `&`, `<`, and `>` with `&amp;`,
`&lt;`, and `&gt;`. Global greater-than replacement is explicitly local policy,
not observed liblsl behavior. The accepted corpus is unchanged: every oracle
observation and candidate result remains `not-observed` with null evidence.

The local result proves no element or document assembly, LSL field mapping,
exact endpoint bytes, oracle result, protocol, wire, transport, or runtime
behavior. Those roles remain separately gated.

## LSLC-001D local composition binding

LSLC-001D composes only the accepted LSLC-001B `XmlElementName` and LSLC-001C
`XmlCharacterData` values. Its separate overlay records independently authored
local Rust tests for exact two-component ownership and recovery. It does not
edit this corpus: every oracle observation and candidate result remains
`not-observed` with null evidence, and LSLC-001C greater-than replacement
remains local candidate policy.

The composition proves no tag spelling, element tree, root or document shape,
raw bytes, parsing, serialization, stream-info mapping, exact endpoint output,
oracle behavior, protocol, wire, transport, or runtime behavior.

## LSLC-001E local hierarchy binding

LSLC-001E composes accepted LSLC-001B through LSLC-001D values into a bounded
caller-owned parent-before-child arena. Its separate overlay records only
independently authored local Rust structural and resource tests. It does not
edit this corpus: every oracle observation and candidate result remains
`not-observed` with null evidence. The hierarchy assigns no stream-info,
`info`, `desc`, complete-document, serialization, or endpoint role.

## LSLC-001F local projection binding

LSLC-001F consumes the accepted CORE-004 metadata arena and composes only
accepted LSLC-001B through LSLC-001E component contracts. Its separate overlay
records a local one-way `None`-to-container and `Some`-to-leaf policy, including
empty present text, plus deterministic bounds, allocation, and failure
precedence. It does not edit this corpus: every oracle observation and
candidate result remains `not-observed` with null evidence.

The projection assigns no reverse or round-trip behavior, complete-document,
serialization, stream-info, `info`, `desc`, LSL field-mapping, endpoint,
official-liblsl, protocol, wire, transport, runtime, or compatibility role.

## LSLC-001L local numeric-spelling binding

`fixtures/compatibility/lslc-001l-static-numeric-spelling-results.json` is a
separate candidate overlay over the immutable LSLC-001H case/observation set
and LSLC-001K semantic overlay. It records only the exact two numeric lexical
fields for all seven cases, the 20-byte channel-count bound, the fixed 17-byte
nominal-rate outputs, and the closed five-value regular-rate policy. It does
not modify public observed XML or the unresolved full-document candidate
result.

## LSLC-001K local static-field binding

LSLC-001K uses the accepted LSLC-001H seven-case manifest and observation
overlay only as public black-box semantic-field evidence. Its separate overlay
does not edit the frozen corpus or LSLC-001H artifacts. It records a local
borrowed `StreamDefinition` projection and keeps XML construction, `desc`
interpretation, numeric formatting, runtime fields, protocol, and transport
outside the candidate result.
