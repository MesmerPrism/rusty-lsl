# STRM-000 compatibility fixtures

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
