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
