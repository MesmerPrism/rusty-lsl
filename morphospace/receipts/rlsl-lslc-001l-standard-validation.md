# LSLC-001L standard validation evidence

- Unit: `rlsl-lslc-001l-static-numeric-spellings`
- Project: `rusty-lsl`
- Branch: `codex/rlsl-lslc-001l-numeric-spellings`
- Claimed-base revision: `a9683760e9874ae79e34418c6a66604142c7d8d2`
- Validated revision: `d0edfa9f8d1e3cc8473efd9e2558e55743005a3e`
- Validation tier: `standard`
- Completed at: `2026-07-13T21:19:12.8031282Z`
- Device requirement: `forbidden`

## Accepted implementation

The validated tree adds one dependency-free bounded lexical projection over a
borrowed accepted `StreamInfoStaticFields`. It owns only the channel-count and
nominal-rate strings. Regular-rate domain validation precedes either owned
allocation. Channel-count spelling uses a fixed 20-byte stack buffer, and each
accepted owned result performs one fallible exact reserve.

The exact accepted nominal-rate domain is closed to the five observed regular
`f64` bit patterns plus the irregular form. All other regular values return the
typed `UnsupportedRegularNominalSrate` error with unchanged bits. No generic
float formatter, exponent policy, locale policy, shortest-round-trip policy,
or broader numeric compatibility is present.

## Exact seven-case matrix

The focused Rust test directly executes all seven accepted LSLC-001H/K inputs:

1. channel count `1`, irregular -> `0.000000000000000`
2. channel count `2`, `100.0` -> `100.0000000000000`
3. channel count `3`, `59.94` -> `59.94000000000000`
4. channel count `4`, `1.0` -> `1.000000000000000`
5. channel count `5`, `256.5` -> `256.5000000000000`
6. channel count `6`, irregular -> `0.000000000000000`
7. channel count `7`, `1000000.25` -> `1000000.250000000`

The source view and `StreamDefinition` remain borrowed, unchanged, and
reusable. A separate test exercises `usize::MAX` channel-count spelling and the
20-byte bound. Unsupported regular-rate tests retain exact rejected bits.

## Validation results

### Formatting

Command: `cargo fmt --all --check`

Result: pass.

### Focused LSLC-001L gate

Command:
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_001l.ps1`

Result: pass. The gate validated the source shape, allocation and rejection
precedence, exact spellings, three focused Rust tests, H/K artifact bindings,
complete historical validators, documentation routes, protected-path
cleanliness, and dependency/feature/publication closure.

### Full owner-repository gate

Command:
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1`

Result: pass.

- Rust tests: 143 passed, 0 failed.
- STRM-000: pass.
- LSLC-001A through LSLC-001G: pass.
- LSLC-001K: pass.
- LSLC-001L: pass.
- CORE-001 through CORE-008: pass.
- Public-boundary check: pass.
- Project-workspace lifecycle and inert-lock checks: pass.
- Source-only closure: pass.

### Portable workflow contracts

Command:
`powershell -NoProfile -ExecutionPolicy Bypass -File
<work-environment-root>/scripts/Test-WorkflowContracts.ps1
-WorkspaceRoot <project-root>/morphospace`

Result: pass.

## Independent mutation checks

Three in-memory mutations were tested without changing repository bytes:

- a damaged accepted LSLC-001K overlay digest was rejected;
- channel-count allocation moved before regular-rate domain validation was
  rejected;
- the first accepted nominal-rate spelling changed to `0` was rejected.

## Scope and provenance audit

- Exactly 14 implementation paths changed from the claimed revision.
- Every path is inside the declared LSLC-001L repository envelope.
- No staged changes remained before the implementation commit.
- Cargo manifests and lock, project and feature locks, lifecycle state/events,
  the active unit, H/K accepted fixtures, and H/K checkers were unchanged by
  the implementation commit.
- The LSLC-001L overlay binds exact accepted H case, H observation, and K
  semantic-overlay SHA-256 values.
- Implementation inputs remain empty; no liblsl, pylsl, rLSL, wrapper,
  application, vendored, generated, or external source implementation was
  inspected or used.
- Both PowerShell files use complete CRLF, while Rust, Python, JSON, and
  Markdown files retain LF per `.gitattributes`.

## Instruction impact

- `AGENTS.md`: updated and reviewed complete.
- `README.md`: updated and reviewed complete.
- `system-engineering` skill: reviewed; unchanged because its existing
  authority and fail-closed contract guidance already covers the slice.
- `rusty-morphospace-context` skill: reviewed; unchanged because its existing
  Rusty LSL routing, public boundary, and Manifold authority guidance already
  covers the slice.

## Preserved failure history

- The secondary worker initially used `f64::next_up` / `next_down`; Rust 1.80
  does not provide those methods. Tests were changed to explicit adjacent bit
  patterns and then passed.
- Main review found that channel-count allocation preceded unsupported-rate
  domain rejection while the overlay claimed allocation occurred after domain
  validation. Construction and the focused checker were corrected so domain
  rejection precedes both allocations.
- Main scope audit found one out-of-envelope fixture-index README edit. That
  worker-added section was removed; the accepted overlay remains documented in
  declared public documentation paths.
- Main review normalized the new and modified PowerShell drivers to the CRLF
  policy required by `.gitattributes`.
- An optional `cargo clippy -- -D warnings` probe remains red because the
  repository has a pre-existing pedantic lint backlog outside this unit. After
  local cleanup, the probe reported zero diagnostics in the new LSLC-001L
  source file. Clippy is not a declared LSLC-001L acceptance gate.
- The first post-acceptance full check ran while lifecycle paths were
  intentionally dirty. All 143 Rust tests passed, then the focused checker
  correctly rejected the dirty protected paths. The acceptance ledger was
  committed before the meaningful clean-tree rerun.
- The clean-tree post-acceptance check then found that this validation log
  contained a local absolute workflow-tool path. The path was replaced with a
  portable placeholder before publication and the bound artifact hash was
  updated before the final clean-tree rerun.

## Explicit non-execution and authority closure

No external oracle, network, source-repository implementation inspection,
device, Quest, ADB, package, runtime, protocol, wire, socket, discovery,
inlet, outlet, clock, buffering, recovery, FFI, adapter, provider, or external
integration action was run or added.

No Manifold admission, identity, registry, subscription, route, lease,
revision, replay, authorization, audit, command, session, topology, transport,
or product-policy authority was added.

## Verdict

Standard source-only validation passed. LSLC-001L is eligible for the explicit
validating, validation-pass, and acceptance lifecycle transitions.
