# LSLC-001A stream-info document corpus

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
