# Rusty LSL Agent Notes

Rusty LSL is a public Rusty Morphospace repository for an independently
authored Rust implementation of Lab Streaming Layer compatibility. Keep every
committed file portable, public-safe, and free of private paths, product names,
device identities, raw captures, credentials, signing material, or local
planning history.

Project-owned source is licensed `AGPL-3.0-or-later`.

## Purpose

Rusty LSL owns:

- backend-neutral Rust APIs for LSL-compatible metadata, discovery, samples,
  clocks, buffering, cancellation, recovery, and provider health;
- independently authored LSL protocol and runtime behavior;
- compatibility fixtures and differential evidence against official liblsl;
- observation and proposal hooks that deeper Rusty Morphospace adapters can
  consume.

Rusty LSL does not own:

- Manifold stream admission, registry revisions, subscriptions, routes, leases,
  provider epochs, authorization, or audit;
- Morphospace-native sample transport or generic stream authority;
- Quest networking, permissions, packaging, Android lifecycle, or device
  resources;
- Hostess orchestration, application policy, recording policy, or runtime
  defaults;
- commands derived directly from inbound LSL samples.

Morphospace hooks stop at typed observations and proposals. The accepting
adapter and authority remain in their owning repositories.

## Read Order

1. `README.md`
2. `morphospace/project.spec.json`
3. `morphospace/feature.lock.json`
4. `morphospace/workspace.state.json`
5. the current iteration unit, if one is named by workspace state
6. `docs/ARCHITECTURE.md`
7. `docs/COMPATIBILITY.md`
8. `docs/PROVENANCE.md`
9. `docs/VALIDATION.md`

The project-local workflow is planning and composition state, not LSL runtime
or compatibility authority. The accepted STRM-000 baseline remains historical
specification-only evidence: its planned observations are not measured and its
results remain `not-implemented`. CORE-001 opens only dependency-free local
Rust contract semantics for bounded metadata and sample shape. CORE-002 adds
only finite raw source timestamps, separately typed optional derived timestamp
values, timestamped samples, and bounded chunks. CORE-003 adds only bounded
core stream descriptors, explicit nominal-rate values, and seven data-only
channel-format names. CORE-004 adds only a dependency-free parent-before-child
flat metadata-tree arena with explicit structural and Unicode scalar-value
bounds. CORE-005 adds only a dependency-free descriptor/sample binding for
exactly seven homogeneous data representations, exact descriptor format and
channel-shape checks, and bounded String channel values. CORE-006 adds only a
separate dependency-free timestamped descriptor/sample composition for those
same seven representations, delegating all sample validation to CORE-005 while
retaining raw and optional derived timestamp evidence unchanged. CORE-007 adds
only a dependency-free non-empty timestamped descriptor/chunk composition for
those same seven representations, retaining the original chunk limits and
delegating every ordered sample through CORE-006 with indexed unchanged errors.
CORE-008 adds only an infallible dependency-free composition that moves one
already validated `StreamDescriptor` and one already validated generic
`MetadataTree` into private accepted state with borrowed and consuming access.
Keep the feature lock empty and inert until a later reviewed unit and
owner-issued descriptor open an exact runtime surface.

## Provenance And Compatibility

- Do not copy or translate liblsl or rLSL source.
- Do not use rLSL source as an implementation input.
- Official liblsl is an MIT-licensed compatibility oracle and reference
  endpoint, not a source template.
- Keep specification, planned observation, and measured result separate. For
  the STRM-000 baseline every current result is `not-implemented` and every
  measured observation is absent.
- Record every fixture or observation as independently authored, generated,
  black-box observed, adapted, or copied. Copied material requires an explicit
  license and notice review.
- Do not claim clean-room implementation, wire compatibility, ecosystem
  compatibility, or runtime support without the named process and evidence.
- Keep official native libraries and wrappers outside the default production
  dependency closure.
- The repository is source-only. Its local constructors have no runtime,
  package, permission, network, authority, or feature-activation effect.

## Architecture Rules

- Start with one `std`-only facade crate. Split protocol, runtime, testkit,
  oracle, or C-ABI crates only when a reviewed ownership boundary requires it.
- Keep `unsafe_code = "forbid"` until a separately reviewed FFI or platform
  adapter demonstrates a need.
- Keep metadata, frames, channel counts, chunks, queues, timeouts, retries, and
  retained ranges explicitly bounded.
- CORE-001 constructors validate complete inputs before returning a value,
  reject invalid zero limit configurations, preserve accepted caller values,
  and report stable expected/actual error payloads.
- CORE-002 preserves every accepted raw source timestamp bit-for-bit beside any
  separately labelled derived value. `ClockCorrected` and `Smoothed` are
  caller-provided classifications only, not implemented algorithms. It rejects
  non-finite timestamps, invalid chunk maxima, one-past maxima, and inconsistent
  channel shapes atomically. An empty chunk is valid under nonzero maxima.
- CORE-003 requires a nonempty stream name and explicit nonzero Unicode scalar
  and channel maxima. Optional content type and source correlation are bounded
  opaque text preserved exactly. Source correlation is never runtime identity,
  discovery, recovery, authorization, routing, permission, admission, or
  Morphospace/Manifold authority.
- A regular nominal sample rate must be finite and positive and preserves its
  accepted floating-point bits; irregular is a separate explicit form. These
  values do not read clocks, measure, schedule, enforce, interpolate, or derive
  rates.
- `ChannelFormat` has exactly seven data-only variants: `Float32`, `Double64`,
  `String`, `Int32`, `Int16`, `Int8`, and `Int64`. They have no wire numeric
  discriminants and perform no byte sizing, encoding, decoding, or conversion.
- CORE-004 accepts exactly one root at index zero and requires every later node
  to name a strictly earlier parent. Root depth is one. Total nodes, depth,
  direct children per node, name Unicode scalar values, and optional value
  Unicode scalar values have explicit nonzero maxima.
- Metadata-tree names are required and nonempty. Optional values preserve
  `None` versus `Some("")`; accepted node order, parent indices, names, and
  optional values are retained exactly behind private fields and read-only
  accessors. Validation and storage use a flat arena with no recursive public
  ownership or recursive validation/traversal.
- CORE-004 implements no XML syntax, parsing, serialization, escaping,
  namespaces, attributes, entities, schemas, queries, protocol behavior,
  discovery, runtime, transport, or tree mutation.
- CORE-005 reuses validated `StreamDescriptor` and `Sample<T>` values. Its
  accepted state privately retains only the exact descriptor channel count and
  data-only format plus the unchanged owned sample; it does not copy the full
  descriptor into each binding.
- Descriptor/sample construction checks format, then channel count, then String
  values in zero-based channel order. Its nonzero String maximum counts Unicode
  scalar values, accepts empty strings, and reports the first oversized channel
  with stable expected/actual counts. Numeric values retain order and exact
  bits, including signed zero and NaN payloads.
- CORE-005 performs no conversion, casting, parsing, formatting, normalization,
  inference, byte sizing, encoding, decoding, endianness, wire mapping,
  allocation beyond owned contract state, or runtime action.
- CORE-006 moves one existing `TimestampedSample<T>` apart without cloning or
  recalculation, delegates its sample unchanged to `BoundDescriptorSample::new`,
  and privately retains the accepted binding plus the exact raw source and
  optional derived timestamp evidence. Its public input family has exactly the
  same seven type-to-format mappings as CORE-005.
- CORE-006 adds no timestamp algorithm, clock read, correction, smoothing,
  dejittering, interpolation, sorting, rewriting, scheduling, buffering,
  conversion, encoding, transport, protocol, wire, or runtime action.
- CORE-007 rejects an empty existing `TimestampedChunk<T>` before delegation,
  then moves each sample in caller order exactly once through
  `BoundTimestampedDescriptorSample::new`. Accepted private state contains only
  the original `ChunkLimits` and the ordered accepted sample bindings.
- CORE-007 reports the zero-based first failing sample and unchanged
  `DescriptorSampleError`. It duplicates no CORE-005/006 format, channel-count,
  String-bound, or timestamp validation and performs no splitting, merging,
  rechunking, sorting, rewriting, buffering, queueing, scheduling, conversion,
  encoding, transport, protocol, wire, or runtime action.
- CORE-008 `StreamDefinition` privately owns exactly one existing descriptor
  and one existing metadata tree. Construction moves both directly, performs
  no allocation, clone, validation, normalization, inference, or interpretation,
  and exposes only `descriptor`, `extended_metadata`, and consuming `into_parts`
  access.
- CORE-008 does not make the generic metadata-tree root an LSL `desc` element
  and adds no XML/document shape, channel conventions, runtime identity,
  discovery, protocol, transport, provider, adapter, or authority behavior.
- Timestamp value constructors do not read clocks or calculate correction,
  dejittering, smoothing, interpolation, or sample-rate timestamp derivation.
- Discovery is observation, never identity, authorization, or activation.
- No inbound sample may apply a command directly.
- No high-rate media belongs in the generic LSL sample path.
- Provider fallback is explicit and preserves the failed candidate evidence.

## Worktree And Agent Policy

Use one writer per branch and worktree. Account-specific or delegated work must
use a dedicated linked worktree and a `codex/*` branch. The main checkout is
the integration and review surface; delegated agents must not write there.

A handoff records the baseline commit, branch, allowed paths, non-scope,
commands run, results, unresolved risks, and rollback point.

## Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

For compatibility-baseline edits, also run the focused gate directly:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_strm_000.ps1
```

For bounded-contract edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_001.ps1
```

For timestamped-chunk contract edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_002.ps1
```

For stream-descriptor contract edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_003.ps1
```

For bounded metadata-tree contract edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_004.ps1
```

For descriptor/sample binding edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_005.ps1
```

For timestamped descriptor/sample composition edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_006.ps1
```

For timestamped descriptor/chunk composition edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_007.ps1
```

For stream-definition composition edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_core_008.ps1
```

The gates prove only the source-level baseline, local Rust contract semantics,
and inert dependency/activation closure. They do not prove protocol behavior,
interoperability, clock behavior, transport, or runtime support.
