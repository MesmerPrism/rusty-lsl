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
compatibility result. Run
`tools/check_strm_000.ps1` after any edit; digest changes must be reviewed and
recorded in the provenance manifest. Run `tools/check_core_001.ps1` for the
overlay and local-contract implementation.
