# Validation

## Entry point

Run the full source-only gate from the repository root:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1
```

The script runs formatting, locked offline metadata inspection, locked offline
tests, the STRM-000 compatibility/provenance gate, the LSLC-001A corpus gate,
the LSLC-001B XML value-contract gate,
the LSLC-001C XML character-data representation gate,
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

Future compatibility claims require focused positive and damaged fixtures,
oracle versioning, normalized differential results, and platform details. Live
or external evidence must remain separate from source validation and must name
its cleanup and reproducibility limits.

When the portable Rusty Morphospace work-environment repository is available,
also run its `Test-WorkflowContracts.ps1` against `morphospace/`. The local
checker is a repository gate; it does not replace portable lifecycle or
transition validation.
