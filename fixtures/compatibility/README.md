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

All current results are `not-implemented`. Run
`tools/check_strm_000.ps1` after any edit; digest changes must be reviewed and
recorded in the provenance manifest.
