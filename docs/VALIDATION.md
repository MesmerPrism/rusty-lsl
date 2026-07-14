# Validation

LSLC-002I pins the official source revision/hash, exact port, seven displayed
spellings, evidence role, unusual spelling, and fail-closed limitations.

LSLC-002G records primary review's rejection of the otherwise passing LSLC-002F
feature head: its unit claimed an explicit extra-delimiter damaged case without
a named second-CRLF regression, and its crate-level status/ownership prose did
not name the exported response-envelope contract. The additive correction
binds both surfaces without changing encoder/parser behavior or rewriting
LSLC-002F evidence.

Run `./tools/check_lslc_002e.ps1` to validate the LSLC-002E provenance lock,
two-case bounded framing matrix, exact typed LSLC-002A/002B binding result,
raw-private boundary, and observation-only nonclaims.

## LSLC-002D focused gate

`tools/check_lslc_002d.ps1` binds the unchanged public source revision/digest,
the 65-byte arithmetic and RST-indentation distinction, rejected 002C head,
CRLF-only source shape, and exact line-ending damaged matrix. Rust tests cover
LF-only, mixed, missing CR, extra CR/LF, truncation, bounds, and decimals. This
is local candidate evidence only, not networking or interoperability proof.

## LSLC-002C focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002c.ps1`. The gate checks the provenance binding, fixture
families, four focused Rust tests, bounded query and payload surfaces,
canonical decimal rejection, borrowed-source preservation, exact final-LF
closure, and absence of networking or unsafe source markers. It proves only
the local query payload contract, not query semantics, response behavior,
endpoint meaning, discovery, networking, currentness, interoperability, or
authority.

## LSLC-002B focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002b.ps1` for closed lexical admission, entity decoding,
existing-contract delegation, damaged fixtures, and inert closure.

Run the LSLC-002A focused source-only gate with
`.\tools\check_lslc_002a.ps1`. It binds the public-safe valid, damaged,
truncated, oversized, non-canonical, malformed-closing, and character-data
fixtures; executes the six focused Rust tests; rejects transient `Vec`/format
allocation and generic-substring end-tag scanning; checks exact first-byte
errors; and verifies documentation plus inert public-boundary routing.

The gate proves only the bounded canonical borrowed shape contract. It proves
no general XML parsing, semantic field conversion, endpoint/wire
interoperability, discovery, transport, runtime, networking, device, feature,
dependency, or authority behavior.

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001z.ps1` for the LSLC-001Z accepted N/X input, P/Q/R
composition, separate-witness retention, exact document, delegated rejection,
and inert ambient-surface checks.

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001x.ps1` for the LSLC-001X accepted-input, three-witness,
allocation-preservation, complete-S, delegated-rejection, cross-owner-inference,
and ambient-surface checks.

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001v.ps1` for the LSLC-001V shared-witness, one-call,
fixed-order transport bound, allocation, lane-only, and ambient network-surface
checks.

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001u.ps1` for the LSLC-001U shared-witness, one-call,
fixed-order bound, allocation, lane-only, and ambient-acquisition checks.

Run the LSLC-001T focused source-only gate with
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001t.ps1`.
It checks three Rust tests, one-call acquisition, separate owner evidence,
exact-match rejection, version bounds, S-lane projection, documentation, and
inert dependency/feature/authority closure.

Run the focused LSLC-001S gate with
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001s.ps1`.
It checks six Rust cases, immutable O delegation, disjoint lanes, fail-closed
shape errors, documentation, and inert acquisition/runtime closure.

## LSLC-001R focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_001r.ps1`. The gate binds H/G/Q artifacts, proves the G
source remains unchanged, executes four focused Rust tests including all seven
normalized cases and the unobserved-empty-container rejection, and protects
the inert dependency, feature, runtime, and public-boundary surfaces.

## LSLC-001Q focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_001q.ps1`. The gate binds accepted H/N/P evidence, checks
the exact structural and allocation-order invariants, executes four focused
Rust tests including all seven cases, protects inert dependency and feature
surfaces, and checks public boundary routing.

## LSLC-001P focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001p.ps1`. It binds accepted B/C/E/H/O evidence, checks
twelve-node/order/allocation invariants, executes four focused Rust tests, and
preserves provider, document, runtime, feature, device, and authority closure.

## LSLC-001O focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001o.ps1`. The gate binds the accepted A/H/N artifacts,
checks the fixed eleven-role order and three disjoint classes, executes all five
focused Rust tests, validates the local overlay and stale-status reconciliation,
and preserves dependency, feature, publication, provider, representation,
runtime, device, and authority closure.

## LSLC-001N focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_001n.ps1`. The gate checks explicit `desc` admission,
checked offset and allocation order, immutable dependency bindings, all five
focused Rust tests and seven candidate cases, documentation, protected
surfaces, and the inert dependency/feature/publication closure.

## LSLC-001M focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_001m.ps1`. The gate binds accepted dependency artifacts,
checks the seven-node and fallible-allocation source invariants, executes all
five focused Rust tests, verifies the seven exact candidate static values and
compact serializations, checks documentation and protected surfaces, and
confirms the empty dependency/feature/publication closure.

## LSLC-001J shallow-checkout protected-surface correction

The focused LSLC-001H gate no longer requires historical revision `9650de4`.
It requires the exact 21-entry binary protected-tree manifest at current
`HEAD`, SHA-256
`ee776163e904ea3c6eb336dd1855d12f0def3e257634272e0c33e7b6e784d8e1`,
then independently requires no staged or unstaged protected changes and no
untracked protected paths. Git command or repository setup failures reject.

Deterministic tests use removable local file clones outside the source
worktree. They prove that the overlaid checker passes in a one-commit shallow
clone with `9650de4` absent, while tracked worktree content, staged index
content, ordinary and ignored untracked protected paths, and an exact committed
manifest mutation each reject at the intended boundary. They make no network,
fetch, unshallow, oracle, device, or source-worktree mutation.
The branch-independent clone route also passes when its source checkout is
detached, matching the GitHub Actions execution shape.

Runs `29276386135` and `29278122366` remain distinct failed pre-fix attempts.
The latter had already passed all 134 Rust tests and LSLC-001A through
LSLC-001G before encountering the missing-history failure. A passing corrected
gate proves only the pinned source-tree and checkout-state validation boundary;
it does not prove implementation semantics, candidate XML, protocol, wire,
runtime, dependency, platform, compatibility, publication, or authority
behavior.

## LSLC-001I portable driver-source correction

The LSLC-001H focused gate reads the two bound driver sources from the current
working tree and compares their canonical LF SHA-256 values with the unchanged
provenance bindings. Complete LF and byte-equivalent complete CRLF sources are
accepted. Mixed LF/CRLF, lone carriage returns, and non-line-ending mutations
are rejected by deterministic in-memory checks for both drivers. Only CRLF
pairs are canonicalized; no other byte is decoded, trimmed, or normalized.

GitHub Actions run `29276386135` is the preserved failed pre-fix integration
attempt that exposed complete CRLF checkout materialization. A passing gate
proves only checkout-portable validation of the two bound text sources. It does
not rerun the oracle or change/prove any observation, capture, candidate XML,
protocol, wire, runtime, dependency, platform, or authority behavior.

## LSLC-001H oracle observation gate

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001h.ps1
```

The focused gate verifies all historical fixture digests, the frozen corpus
binding, case coverage and bounds, two-capture identity, exact public XML and
normalization records, oracle/package/library identities, public-boundary
scans, typed failure coverage, external-artifact exclusion, inert production
closure, documentation routing, and main-account-only skill closure. It does
not rerun the external oracle.

To acquire and capture the pinned oracle separately:

```powershell
pwsh -NoProfile -File .\tools\oracle\Invoke-Lslc001hOracle.ps1 -Mode Capture
```

That command writes only below its explicit external root.

## LSLC-001G focused gate

Run:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001g.ps1
```

The focused gate checks twelve local Rust tests, exact byte and allocation
precedence, iterative traversal, source ownership, the independent overlay,
historical fixture digests, corpus null states, inert closure, and public
routing. It proves no complete document, parser, decoder, LSL mapping, endpoint
or official behavior, protocol, wire, I/O, runtime, or compatibility.

## Entry point

Run the full source-only gate from the repository root:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1
```

The script runs formatting, locked offline metadata inspection, locked offline
tests, the STRM-000 compatibility/provenance gate, the LSLC-001A corpus gate,
the LSLC-001B XML value-contract gate,
the LSLC-001C XML character-data representation gate,
the LSLC-001D XML leaf-element composition gate,
the LSLC-001E XML container/leaf hierarchy gate,
the CORE-001, CORE-002, CORE-003, CORE-004, CORE-005, CORE-006, CORE-007, and CORE-008 local-contract gates, the
public-boundary and text-hygiene checker, the dependency-free local
project-workspace checker, and Git whitespace checks.

Run the focused baseline gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_strm_000.ps1
```

Run the focused LSLC-001A corpus gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001a.ps1
```

Run the focused LSLC-001B XML value-contract gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001b.ps1
```

Run the focused LSLC-001C XML character-data representation gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001c.ps1
```

Run the focused LSLC-001D XML leaf-element composition gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001d.ps1
```

Run the focused LSLC-001E XML container/leaf hierarchy gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001e.ps1
```

Run the focused bounded-contract gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_001.ps1
```

Run the focused timestamped-chunk gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_002.ps1
```

Run the focused stream-descriptor gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_003.ps1
```

Run the focused bounded metadata-tree gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_004.ps1
```

Run the focused descriptor/sample binding gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_005.ps1
```

Run the focused timestamped descriptor/sample composition gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_006.ps1
```

Run the focused timestamped descriptor/chunk composition gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_007.ps1
```

Run the focused stream-definition composition gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_core_008.ps1
```

## Acceptance criteria

The source-only slice passes when:

- the crate builds and tests without third-party dependencies;
- unsafe Rust is forbidden;
- the public status is exactly `BoundedLocalContracts`;
- the only package remains unpublished at `crates/rusty-lsl`, exposes no Cargo
  feature, and has exactly one library target;
- repository content passes the tested public-boundary rules;
- all four compatibility classes have bounded cases, all current results remain
  `not-implemented`, and specification/planned/measured roles remain separate;
- LSLC-001A records exactly the two approved public-documentation sources and
  the independently worded claim inventory, with no source-code or
  implementation-input classification;
- its seven positive and nine damaged cases keep specification, oracle
  observation, and candidate result separate; every observation and result is
  `not-observed` with null evidence;
- its corpus-owned input limits remain explicit, exact serialization remains
  unresolved, historical STRM-000 files and CORE overlays retain their baseline
  digests, all instruction rows are complete, and lifecycle parsing accepts
  only active, validating, or accepted state;
- the LSLC-001B overlay binds only local Rust tests to accepted LSLC-001A case
  roles while every corpus oracle observation and candidate result remains
  `not-observed` with null evidence;
- separate nonzero XML text and name limits count Unicode scalar values; text
  accepts exactly the XML 1.0 Fifth Edition `Char` production and names accept
  the complete `NameStartChar` and `NameChar` productions;
- text length precedes illegal-scalar rejection, name rejection follows empty,
  length, invalid start, then first invalid continuation, and indexed errors
  retain scalar indexes and code points;
- accepted XML values retain their private original `String` allocation and
  exact content, including ampersand, less-than, greater-than, and `]]>`;
- LSLC-001B opens no escaping, parsing, serialization, output, document,
  namespace, field-mapping, dependency, feature, unsafe, I/O, protocol, wire,
  transport, or runtime surface;
- the LSLC-001C overlay binds exactly seven focused local Rust tests only to
  the accepted LSLC-001A character-data role and LSLC-001B `XmlText` contract,
  while all corpus oracle/candidate evidence remains unchanged and null;
- `XmlCharacterDataLimit` rejects zero and bounds exact encoded UTF-8 bytes;
  encoding borrows without source mutation or revalidation, uses checked
  arithmetic, checks the maximum before `try_reserve_exact`, writes the exact
  precomputed length, and reports `LengthOverflow`, exact `LimitExceeded`, then
  typed `AllocationFailed` in stable order;
- the fixed local policy emits every `&`, `<`, and `>` as `&amp;`, `&lt;`, and
  `&gt;`, preserves every other legal scalar and output allocation, and labels
  global greater-than escaping as candidate policy rather than observed
  behavior;
- LSLC-001C opens no element, attribute, document, declaration, comment,
  processing-instruction, CDATA-section, parser, decoder, generic entity,
  namespace, schema, query, MetadataTree mapping, LSL mapping, exact endpoint,
  dependency, feature, unsafe, I/O, protocol, wire, transport, runtime,
  adapter, provider, FFI, device, or authority surface;
- the LSLC-001D overlay binds exactly five focused local Rust tests to the
  existing accepted `XmlElementName` and `XmlCharacterData` contracts while
  all LSLC-001A oracle/candidate evidence remains unchanged and null;
- `XmlLeafElement` owns exactly two private fields, has an infallible direct
  move constructor, borrowed component accessors, and allocation-preserving
  `into_parts`, with no raw-string entrypoint, limit, error, allocation,
  clone, validation, re-encoding, normalization, or interpretation policy;
- LSLC-001D opens no tag spelling, attribute, child, parent, mixed-content,
  root, tree, document, namespace, raw-byte, parser, serializer, decoder,
  metadata-tree, stream-info mapping, dependency, feature, unsafe, I/O,
  protocol, wire, transport, runtime, adapter, provider, FFI, device,
  authority, or compatibility-claim surface;
- the separate CORE-001 overlay binds exactly `contract-metadata-bounds` and
  `contract-sample-shape` to exact-limit, one-past-limit, malformed/zero-bound,
  channel-mismatch, stable-error, and unchanged-value tests;
- the separate CORE-002 overlay binds exact local Rust contract tests for finite
  raw and optional derived timestamp values, explicit `ClockCorrected` and
  `Smoothed` classification, bit preservation, raw/derived coexistence, empty
  chunk acceptance under valid nonzero limits, exact chunk maxima, one-past
  maxima, zero maxima, inconsistent shapes, stable error payloads, and unchanged
  sample/time pairing and order;
- CORE-002 opens no clock-reading, correction, dejittering, smoothing,
  interpolation, sample-rate derivation, buffering, transport, protocol, or
  runtime surface;
- the separate CORE-003 overlay binds exact local Rust tests for a nonempty
  stream name, exact and one-past Unicode scalar text bounds, optional bounded
  opaque text, source correlation, zero and one-past channels, malformed
  limits, explicit irregular rate, bit-preserving finite positive regular rate,
  stable rate errors, and exactly seven channel-format names;
- CORE-003 opens no XML/query/tree mutation, discovery, resolution, runtime
  identity, recovery, clock, scheduling, transport, buffering, encoding,
  conversion, wire numeric format, adapter, FFI, or authority surface;
- the separate CORE-004 overlay binds the exact focused positive and damaged
  Rust tests for exactly one root, strictly earlier parents, exact and one-past
  node/depth/child/text bounds, nonzero limits, required nonempty names,
  Unicode scalar counts, deep chains, child fanout, stable indexed errors, and
  absent-versus-empty optional values;
- CORE-004 retains a flat parent-before-child arena with private accepted
  fields and read-only accessors, no recursive public ownership or recursive
  validation/traversal, and no XML syntax, parsing, serialization, query,
  mutation, protocol, discovery, transport, runtime, adapter, dependency,
  feature, unsafe, authority, or compatibility-claim surface;
- the separate CORE-005 overlay binds exact tests for all seven homogeneous
  input mappings, each input-family format mismatch, descriptor/sample channel
  mismatch, nonzero String limits, exact and one-past Unicode scalar bounds,
  first-channel error indexing, empty String preservation, order preservation,
  signed zero and NaN payload bit preservation, integer edges, and stable error
  payloads;
- CORE-005 retains private accepted fields and only a compact descriptor-shape
  snapshot plus the owned validated sample, with no conversion, casting,
  parsing, formatting, normalization, inference, byte sizing, encoding,
  decoding, endianness, wire mapping, allocation beyond owned contract state,
  runtime action, dependency, feature, or unsafe surface;
- the separate CORE-006 overlay binds exact tests for all seven timestamped
  homogeneous mappings, raw-only and both derived kinds, raw and derived signed
  zero and finite bit patterns, sample NaN payloads, integer edges, exact and
  one-past String bounds, format and channel mismatch, delegated validation
  precedence and errors, consuming and read-only accessors, and exact
  timestamp/sample pairing;
- CORE-006 retains private accepted fields containing only a
  `BoundDescriptorSample` plus unchanged raw and optional derived timestamp
  evidence, delegates exactly once to `BoundDescriptorSample::new`, duplicates
  no CORE-005 validation, and opens no clock, timestamp algorithm, sorting,
  rewriting, scheduling, buffering, conversion, encoding, dependency, feature,
  unsafe, transport, protocol, wire, or runtime surface;
- the separate CORE-007 overlay binds exact tests for all seven timestamped
  chunk mappings, original `ChunkLimits`, multi-sample order and pairing, raw
  only and both derived kinds, signed-zero and finite timestamp bits, f32/f64
  signed zero and NaN payloads, integer edges, String allocation/value/order
  preservation, deterministic empty rejection, sample-zero format and channel
  mismatch, later indexed String failure, and first-failure delegated
  precedence;
- CORE-007 retains private accepted fields containing only the original
  `ChunkLimits` and ordered `Vec<BoundTimestampedDescriptorSample>`, rejects
  emptiness before sample delegation, delegates exactly once per sample through
  the single generic call to `BoundTimestampedDescriptorSample::new`, preserves
  unchanged indexed `DescriptorSampleError` values, and duplicates no lower
  validation or clock, algorithm, sorting, rewriting, splitting, merging,
  rechunking, buffering, queueing, runtime, conversion, dependency, feature,
  unsafe, transport, protocol, or wire surface;
- the separate CORE-008 overlay binds exact tests for borrowed and consuming
  access, all seven channel-format variants, irregular and exact-bit regular
  nominal rates, descriptor limits and Unicode/optional text, metadata limits,
  nontrivial parent-before-child node order, Unicode names and values,
  absent-versus-empty optional values, and preservation of existing owned
  allocations across composition;
- CORE-008 retains private accepted fields containing exactly one complete
  `StreamDescriptor` and one complete `MetadataTree`; its infallible constructor
  moves both directly and adds no `Result`, error or limit family, allocation,
  clone, normalization, inference, cross-component validation, XML or `desc`
  interpretation, channel convention, runtime identity, dependency, feature,
  unsafe, discovery, transport, provider, adapter, authority, protocol, wire,
  or runtime surface;
- the damaged matrix, provenance fields, artifact digests, case relationships,
  source-input prohibitions, and oracle isolation contract remain valid;
- the project-local workspace remains well-formed, source-only, and inert;
- every visible source file passes whole-tree trailing-whitespace and terminal
  newline checks, including untracked files before commit;
- `git diff --check` reports no additional Git whitespace errors.

The script rejects any dependency in `cargo metadata`. Build and development
dependencies require a future review and a corresponding gate change rather
than silent addition.

## Evidence limits

A passing source-only gate proves that this revision satisfies the local Rust
contract semantics, historical specification-level STRM-000 checks, the
LSLC-001A public-documentation corpus invariants, and inert
closure checks in the local Rust and PowerShell environment. It does not prove
clock or nominal-rate behavior, timestamp or rate derivation, sample, chunk, or
descriptor transport, metadata-tree XML/document behavior, source identity or
authority, channel encoding or conversion, actual LSL empty-chunk behavior,
stream-definition XML/document meaning or cross-component semantics,
protocol behavior, wire interoperability, ecosystem compatibility, network behavior,
performance, numeric or String conversion, memory layout, native-library safety,
platform support, official-liblsl behavior,
or publication readiness.

The LSLC-001A gate does not prove XML parsing, serialization, exact endpoint
output, oracle behavior, candidate behavior, protocol or query behavior, wire
compatibility, or ecosystem compatibility.

The LSLC-001B gate proves only the local bounded XML text/name value contracts
and inert source closure. It does not prove representation policy, escaping,
CDATA or entity handling, parsing, serialization, document well-formedness,
LSL field mapping, exact bytes, official endpoint behavior, protocol, wire,
transport, runtime, or ecosystem compatibility.

The LSLC-001C gate proves only the local bounded character-data representation,
its source-value composition, fixed candidate replacements, typed allocation
path, and inert source closure. It does not prove document well-formedness,
LSL field mapping, exact endpoint output, official behavior, protocol, wire,
transport, runtime, or ecosystem compatibility.

The LSLC-001D gate proves only exact local composition of the two existing
accepted component types, borrowed access, consuming recovery, allocation
preservation, and inert source closure. It does not prove XML tag or document
syntax, LSL field mapping, exact endpoint output, official behavior, protocol,
wire, transport, runtime, or ecosystem compatibility.

The LSLC-001E gate proves only the local bounded container/leaf hierarchy,
deterministic iterative validation, ownership preservation, and inert source
closure. Its nine focused tests cover exact and one-past bounds, root/parent
shape, leaf parents, allocation and overflow helpers, precedence, and vector
and component allocation preservation. It does not prove complete XML or
document behavior, serialization or raw bytes, `MetadataTree` conversion,
stream-info mapping, exact endpoint output, official behavior, protocol, wire,
transport, runtime, or ecosystem compatibility.

Future compatibility claims require focused positive and damaged fixtures,
oracle versioning, normalized differential results, and platform details. Live
or external evidence must remain separate from source validation and must name
its cleanup and reproducibility limits.

When the portable Rusty Morphospace work-environment repository is available,
also run its `Test-WorkflowContracts.ps1` against `morphospace/`. The local
checker is a repository gate; it does not replace portable lifecycle or
transition validation.

## LSLC-002O focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002o.ps1`. It covers finite offset/input preservation,
ClockCorrected labelling, non-finite offset and sum rejection, ownership, and
LSLC-002L/M prerequisites. It proves no automatic or runtime behavior.

## LSLC-002N focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002n.ps1`. It covers zero limit, empty, exact and oversized
batches, minimum and equal-minimum selection, vector-allocation preservation,
ownership routing, and the LSLC-002M prerequisite. It proves no acquisition,
clock, scheduling, history, correction, currentness, or runtime behavior.

## LSLC-002M focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002m.ps1`. It executes finite valid formulas, signed-zero
bit preservation, first non-finite input roles, every reachable first
non-finite arithmetic stage, ownership routing, and the LSLC-002L evidence
gate. It proves no packet, clock, filtering, correction, or runtime behavior.

## LSLC-002L focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002l.ps1`. It rejects provenance, exact-formula, selection,
limitation, evidence-role, or non-scope drift. It performs no network or clock
operation and proves no candidate or runtime behavior.

## LSLC-002K focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002k.ps1`. It checks borrowed and consuming allocation/data
preservation, the public ownership declaration, and both prerequisite focused
gates. It proves no address, selection, socket, send, discovery runtime,
reachability, interoperability, activation, or authority behavior.

## LSLC-002J focused gate

Run `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_002j.ps1`. The gate executes the closed inventory, static
storage, and public ownership regressions and reuses LSLC-002I provenance
validation. It proves no address interpretation, networking, discovery runtime,
reachability, interoperability, activation, or authority behavior.

## LSLC-001F focused gate

Run:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001f.ps1
```

The LSLC-001F gate proves only the local consuming one-way projection,
None/Some classification, deterministic indexed failures, deliberate output
and represented-string allocation ownership, and unchanged final hierarchy
delegation. Its twelve focused tests cover classification, shape rejection,
component and target bounds, precedence, exact limits, allocation failure, and
ownership. It does not prove reverse conversion, decoding, round trips,
document or serialization behavior, stream-info or LSL mapping, endpoint or
official behavior, protocol, wire, transport, runtime, or compatibility.

## LSLC-001L focused gate

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001l.ps1
```

The gate reuses the complete accepted LSLC-001K validator, including its
immutable LSLC-001H corpus, case, observation, provenance, and driver checks.
It then checks the borrowed-plus-two-string source shape, bounded fallible
reserves, closed regular-rate bit policy, all seven overlay rows and observed
numeric texts, three focused Rust tests, documentation routes, protected-file
cleanliness, and inert dependency/feature/publication closure. It does not
execute the external oracle or validate XML construction, complete-document
bytes, volatile fields, protocol, wire, runtime, or broad floating-point
compatibility.

## LSLC-001K focused gate

Run:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001k.ps1
```

The LSLC-001K gate checks the exact single-reference view shape, fixed six-role
order, original and effective option/rate separation, all seven format
mappings, pointer/source preservation tests, direct execution of the seven-case
semantic matrix, exact LSLC-001H artifact bindings, seven-case local-results
matrix, documentation routing, and inert dependency and feature closure. The
rolling gate reuses the full immutable LSLC-001H corpus, case, observation,
provenance, and driver validators. It deliberately does not reapply LSLC-001J's
current-HEAD protected-tree pin after an authorized source unit; the accepted
LSLC-001J receipt preserves that validation-only result. The gate does not prove
XML, numeric formatting, runtime fields, protocol, wire, transport, I/O,
adapters, providers, devices, or authority.
