# Validation

## Entry point

Run the full source-only gate from the repository root:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1
```

The script runs formatting, locked offline metadata inspection, locked offline
tests, the STRM-000 compatibility/provenance gate, the CORE-001, CORE-002, and
CORE-003 local-contract gates, the public-boundary and text-hygiene checker, the
dependency-free local project-workspace checker, and Git whitespace checks.

Run the focused baseline gate with:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_strm_000.ps1
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
contract semantics, historical specification-level STRM-000 checks, and inert
closure checks in the local Rust and PowerShell environment. It does not prove
clock or nominal-rate behavior, timestamp or rate derivation, sample, chunk, or
descriptor transport, source identity or authority, channel encoding or
conversion, protocol behavior, wire interoperability, ecosystem compatibility, network behavior,
performance, native-library safety, platform support, official-liblsl behavior,
or publication readiness.

Future compatibility claims require focused positive and damaged fixtures,
oracle versioning, normalized differential results, and platform details. Live
or external evidence must remain separate from source validation and must name
its cleanup and reproducibility limits.

When the portable Rusty Morphospace work-environment repository is available,
also run its `Test-WorkflowContracts.ps1` against `morphospace/`. The local
checker is a repository gate; it does not replace portable lifecycle or
transition validation.
