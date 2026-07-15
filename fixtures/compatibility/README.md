# STRM-000 compatibility fixtures

`lslc-002r-official-loopback-stream-handshake-observation.json` contains only
sanitized official outlet/inlet connection-setup roles, bounds, typed outcomes,
and provenance hashes. Raw packets, XML, connection rows, endpoints,
identifiers, diagnostics, binaries, environments, caches, and failed-attempt
payloads remain external.

`lslc-002l-clock-offset-public-documentation-evidence.json` records only pinned
official clock-offset documentation facts. It contains no packet capture,
clock sample, observation, candidate algorithm, or runtime result.

`lslc-002i-default-discovery-destination-evidence.json` contains only pinned
official-documentation specification facts. Parsing, selection, observation,
sockets, runtime discovery, and interoperability remain unclaimed.

`lslc-002e-short-info-response-observation.json` contains only sanitized
black-box framing facts and provenance hashes. It contains no raw response,
XML document, endpoint value, native diagnostic, or machine path.

`lslc-002d-short-info-query-crlf-correction-fixtures.json` preserves the
rejected LSLC-002C head and records the 65-byte CRLF inference separately from
RST presentation indentation, with valid and exact-offset damaged cases.

`lslc-002c-protocol-110-short-info-query-fixtures.json` contains independently
authored public-safe valid, damaged, truncated, oversized, and noncanonical
cases for the bounded local short-info query payload. Its provenance binds the
official public documentation example but makes no query-semantic, response,
socket, discovery, endpoint, currentness, interoperability, or authority claim.

LSLC-002B adds an independently authored public-safe accepted/damaged matrix
for closed parser-to-typed-observation admission, without captures, identities,
runtime evidence, or interoperability claims.

LSLC-002A records one independently authored canonical empty-description
document and deterministic valid, damaged, truncated, oversized,
non-canonical, malformed-closing, and character-data mutations in
`lslc-002a-bounded-observed-document-shape-parser-fixtures.json`. These are
local candidate fixtures, not raw endpoint captures or broad XML/liblsl/wire
interoperability evidence. Run `tools/check_lslc_002a.ps1`.

LSLC-001Z records synthetic local evidence for composing accepted N and X state
through P/Q/R while retaining three separate witnesses. It makes no endpoint,
wire, interoperability, runtime, freshness, or authority claim.

LSLC-001X records synthetic local three-owner composition evidence only. It
contains no acquired clock, host, interface, network, endpoint, authorization,
device, runtime-effect, or Manifold authority evidence.

LSLC-001V contains synthetic local transport-owner provider evidence only. It
records no acquired interface, address, port, socket, network, reachability,
authorization, device, runtime, or Manifold authority data.

LSLC-001U contains synthetic local runtime-provider evidence only; it records
no acquired clock, identity, session, host, transport, network, or device data.

LSLC-001T records synthetic local implementation-version acquisition evidence
only. It contains no acquired host, runtime, transport, network, endpoint,
device, or official-provider value.

LSLC-001S records independently authored candidate results for three-lane,
complete volatile provider snapshot admission. It contains no acquired values
and no freshness evidence.

## LSLC-001R observed document envelope

- `lslc-001r-observed-stream-info-document-envelope-results.json`: accepted
  H/G/Q bindings and the independently authored observation-bound candidate
  policy.

Run `tools/check_lslc_001r.ps1`; it proves the seven normalized envelope bytes,
not parsing, endpoint/wire compatibility, provider acquisition, or runtime.

## LSLC-001Q ordered element composition

- `lslc-001q-ordered-stream-info-element-results.json`: independently authored
  local static, volatile, then `desc` composition and boundary evidence.

Run `tools/check_lslc_001q.ps1`; it proves only compact local element order,
not observed document spelling, provider acquisition, or runtime behavior.

## LSLC-001P volatile XML composition

- `lslc-001p-volatile-stream-info-xml-results.json`: independently authored
  local volatile `info` tree policy and boundary evidence.

Run `tools/check_lslc_001p.ps1`; it proves no provider or complete-document
behavior.

## LSLC-001O volatile stream-info data contract

- `lslc-001o-volatile-stream-info-data-results.json`: independently authored
  role/class/boundary overlay for the bounded opaque accepted-data surface.

It binds accepted LSLC-001A/H/N public evidence without changing it. Run
`tools/check_lslc_001o.ps1`; the result proves no provider, XML/document,
clock/host/identity generation, address/port semantics, networking, runtime, or
Manifold authority behavior.

## LSLC-001H StreamInfo XML observation

- `lslc-001h-stream-info-xml-cases.json`: independently authored bounded
  positive and damaged case manifest.
- `lslc-001h-stream-info-xml-observations.json`: exact pinned-oracle
  black-box observations and public-safe normalized XML.
- `lslc-001h-stream-info-xml-provenance.json`: wheel/library/tool/environment,
  external raw-output digest, normalization, and failure-policy provenance.

Raw XML, native stderr, wheel, DLL, virtual environment, and cache artifacts
are external-only. Run `tools/check_lslc_001h.ps1` for the focused gate.

Run `tools/check_lslc_001g.ps1` for the LSLC-001G borrowed bounded serializer,
fixed explicit-tag and hierarchy-order policy, exact byte/allocation
precedence, source preservation, local overlay, historical preservation, and
inert-closure checks. Its overlay proves no complete document, parsing,
decoding, LSL mapping, endpoint, protocol, wire, I/O, runtime, ecosystem,
compatibility, or official-liblsl behavior.

These small JSON documents are independently authored, deterministic planning
fixtures. They contain behavior specifications, bounded damaged cases, and a
provenance record. They contain no measured interoperability result, captured
traffic, native binary, protocol constant, or implementation-derived byte
sequence.

`behavior-catalog.json` separates the specified behavior, a planned observable,
and the absent measured result. `negative-case-matrix.json` assigns an expected
classification to each bounded damaged input. The expected classification is a
test expectation, not an oracle observation. `baseline-provenance.json` binds
both inputs by SHA-256 and records the source-input prohibitions.

All STRM-000 baseline results remain `not-implemented`.
`lslc-001a-stream-info-document-corpus.json` is a separate LSLC-001A
public-documentation specification corpus. It records two exact source records,
seven positive roles, nine damaged/planned-test roles, and explicit local
bounds. Every oracle observation and candidate result is `not-observed` with
null evidence. It contains no XML payload or endpoint output, and exact
serialization remains unresolved.
`lslc-001b-contract-results.json` is a separate independently authored local
Rust contract overlay for bounded XML legal-text and element-name values. It
binds accepted LSLC-001A case roles without changing their `not-observed` null
oracle/candidate evidence and proves no escaping, parsing, serialization,
document, LSL, protocol, wire, runtime, ecosystem, or official-liblsl behavior.
`lslc-001c-contract-results.json` separately binds seven focused local Rust
tests only to the accepted LSLC-001A character-data role and LSLC-001B
validated `XmlText` contract. It records fixed `&`, `<`, and global `>`
replacement as local candidate policy, preserves all corpus oracle/candidate
roles unchanged, and proves no element, document, LSL mapping, exact endpoint,
protocol, wire, runtime, ecosystem, or official-liblsl behavior.
`lslc-001d-contract-results.json` separately binds five focused local Rust
tests to the accepted LSLC-001B element-name and LSLC-001C character-data
contracts. It records only exact leaf-only two-component composition, preserves
all corpus oracle/candidate roles and LSLC-001C candidate policy unchanged,
and proves no tag, tree, document, LSL mapping, exact endpoint, protocol, wire,
runtime, ecosystem, or official-liblsl behavior.
`core-001-contract-results.json` is a separate independently authored overlay
for local Rust unit-test results tied to `contract-metadata-bounds` and
`contract-sample-shape`; it is not an oracle measurement or an LSL
compatibility result. `core-002-contract-results.json` separately records local
Rust contract tests for finite raw and optional derived timestamp values,
explicit `ClockCorrected` and `Smoothed` classifications, and bounded
timestamped chunks including the empty-collection case. The classifications do
not implement either algorithm. The overlay preserves the historical semantic
timestamp case as `not-implemented` and proves no clock, transport, protocol,
wire, runtime, ecosystem, or official-liblsl behavior.
`core-003-contract-results.json` records only local Rust tests for bounded core
stream descriptors, nominal-rate values, and seven data-only channel-format
names. Its optional source correlation is opaque caller data, not identity,
discovery, recovery, routing, permission, admission, or authority. It proves no
XML, transport, protocol, wire, runtime, ecosystem, or official-liblsl behavior.
`core-004-contract-results.json` records only local Rust tests for a bounded
parent-before-child flat metadata-tree arena. It preserves root/parent indexes,
node order, names, and absent-versus-empty optional values without XML
interpretation. It proves no XML, document, query, mutation, discovery,
transport, protocol, wire, runtime, ecosystem, or official-liblsl behavior.
`core-005-contract-results.json` records only local Rust tests for binding one
of exactly seven homogeneous validated samples to an exact descriptor format
and channel count, with bounded String channel values. It preserves all
historical STRM-000 results and proves no conversion, encoding, layout,
transport, protocol, wire, runtime, ecosystem, or official-liblsl behavior.
`core-006-contract-results.json` records only local Rust tests for composing
one of exactly seven existing timestamped homogeneous samples with an exact
CORE-005 descriptor/sample binding. It preserves raw and optional derived
timestamp evidence and unchanged delegated errors. It proves no clock read,
timestamp algorithm or rewriting, buffering, conversion, encoding, transport,
protocol, wire, runtime, ecosystem, or official-liblsl behavior.
`core-007-contract-results.json` records only local Rust tests for composing
one of exactly seven existing timestamped homogeneous chunks with an exact
descriptor by delegating every ordered sample through CORE-006. It retains the
original chunk limits, pairings, and indexed unchanged errors, and rejects an
empty chunk locally before delegation. It proves no actual LSL empty-chunk
behavior, clock or timestamp algorithm, splitting, merging, rechunking,
buffering, queueing, conversion, encoding, transport, protocol, wire, runtime,
ecosystem, or official-liblsl behavior.
`core-008-contract-results.json` records only local Rust tests for composing
one already validated stream descriptor with one already validated generic
metadata tree. It preserves both component contracts exactly and every
historical STRM-000 result. It gives the generic root no XML or LSL `desc`
meaning and proves no discovery, transport, protocol, wire, runtime,
ecosystem, authority, or official-liblsl behavior.
Run
`tools/check_strm_000.ps1` after any edit; digest changes must be reviewed and
recorded in the provenance manifest. Run `tools/check_core_001.ps1` for the
CORE-001 overlay and `tools/check_core_002.ps1` for the CORE-002 overlay and
timestamped-chunk implementation. Run `tools/check_core_003.ps1` for the
CORE-003 overlay and stream-descriptor implementation.
Run `tools/check_core_004.ps1` for the CORE-004 overlay and bounded flat
metadata-tree implementation.
Run `tools/check_core_005.ps1` for the CORE-005 overlay and descriptor/sample
binding implementation.
Run `tools/check_core_006.ps1` for the CORE-006 overlay and timestamped
descriptor/sample composition implementation.
Run `tools/check_core_007.ps1` for the CORE-007 overlay and timestamped
descriptor/chunk composition implementation.
Run `tools/check_core_008.ps1` for the CORE-008 overlay and stream-definition
composition implementation.
Run `tools/check_lslc_001a.ps1` for the LSLC-001A corpus and its clean-source,
role-separation, historical-preservation, and inert-closure checks.
Run `tools/check_lslc_001b.ps1` for the LSLC-001B range tables, value privacy,
test overlay, corpus separation, and inert-closure checks.
Run `tools/check_lslc_001c.ps1` for the LSLC-001C exact replacements, byte
bounds, typed fallible allocation path, test overlay, historical preservation,
and inert-closure checks.
Run `tools/check_lslc_001d.ps1` for the LSLC-001D private two-component shape,
direct move construction, allocation preservation, component-authority tests,
historical preservation, and inert-closure checks.
Run `tools/check_lslc_001e.ps1` for the LSLC-001E structural and resource
precedence, fallible scratch allocation, ownership preservation, local overlay,
historical preservation, and inert-closure checks. The overlay proves no
complete XML/document, serialization, LSL mapping, endpoint, protocol, wire,
runtime, ecosystem, compatibility, or official-liblsl behavior.
Run `tools/check_lslc_001f.ps1` for the LSLC-001F one-way consuming projection,
fixed None/container and Some/leaf classification, indexed precedence,
allocation ownership, historical preservation, and inert-closure checks. Its
overlay proves no reverse, decoding, round-trip, document, serialization, LSL
mapping, endpoint, protocol, wire, runtime, ecosystem, compatibility, or
official-liblsl behavior.
`lslc-002s-bounded-stream-handshake-runtime.json` binds sanitized request and
response framing roles plus independently authored bounded Rust loopback
runtime outcomes. Raw wire artifacts remain private.
`lslc-002t-bounded-timestamped-float32-sample-runtime.json` binds the sanitized
one-record timestamp/value observation and independent bounded Rust outcome.
Raw wire artifacts remain private.
