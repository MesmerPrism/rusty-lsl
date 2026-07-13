# Compatibility

## Current evidence matrix

| Surface | Implementation status | Current evidence |
| --- | --- | --- |
| Local bounded metadata construction | Implemented | CORE-001 Rust unit tests; no XML behavior |
| Metadata XML | Not implemented | Specification cases only |
| Discovery and resolution | Not implemented | No case or measurement |
| Local sample-shape construction | Implemented | CORE-001 Rust unit tests; no transport behavior |
| Local timestamp value and sample construction | Implemented | CORE-002 Rust unit tests; caller-provided finite values and derived kinds only |
| Local bounded timestamped chunks | Implemented | CORE-002 Rust unit tests, including empty bounded collections; no buffering or transport behavior |
| Sample and chunk transport | Not implemented | Specification cases only |
| Channel formats | Not implemented | No case or measurement |
| Clock-sourced or protocol timestamps | Not implemented | Specification cases only |
| Clock correction and smoothing | Not implemented | No case or measurement |
| Buffering and post-processing | Not implemented | No case or measurement |
| Cancellation and timeouts | Not implemented | Damaged-case expectation only |
| Loss detection and recovery | Not implemented | Specification cases only |
| Provider health and fallback | Not implemented | Specification cases only |
| C ABI or language wrappers | Not implemented | No case or measurement |
| Wire compatibility | Not implemented and not claimed | Planned observations only |
| Operational/ecosystem compatibility | Not implemented and not claimed | Planned observations only |

The CORE-001 and CORE-002 tests prove only local Rust contract semantics for the
named constructors on the tested toolchain. CORE-002 validates caller-provided
finite values and preserves explicit `ClockCorrected` or `Smoothed`
classifications; it does not read clocks, derive timestamps, or implement those
algorithms. These tests do not prove LSL behavior, interoperability,
performance, or platform support.

## Compatibility classes

Compatibility evidence is classified at four distinct levels:

- **Contract compatibility:** Rust types and API behavior represent the named
  metadata, sample, error, timeout, and lifecycle cases.
- **Semantic-bridge compatibility:** explicit adapters preserve meaning across
  LSL observations and downstream Morphospace proposals without transferring
  authority.
- **Wire compatibility:** independently implemented peers exchange the named
  protocol cases with specified versions and negative evidence.
- **Operational/ecosystem compatibility:** documented applications, wrappers,
  platforms, recovery paths, and long-running behavior pass their named gates.

Only the named local contract slices are implemented, and only as local Rust
API behavior. No LSL protocol, wire, runtime, operational, or ecosystem
compatibility is implemented or claimed. The canonical STRM-000 catalog is
`fixtures/compatibility/behavior-catalog.json`; it remains accepted historical
specification-only evidence with at least two bounded cases for each class.
The separate `core-001-contract-results.json` overlay binds local unit tests to
the two CORE-001 case IDs. The `core-002-contract-results.json` overlay binds
local timestamp preservation to `semantic-raw-timestamp-preserved` and records
the bounded-chunk contract without turning either into a measured oracle result.
Evidence at one level must not be promoted into a broader claim.

Each case has three deliberately separate roles:

- `specification` states independently authored behavior and bounds;
- `planned_observation` names a future endpoint and observable;
- `measured_result` records evidence only after a reviewed run.

For STRM-000, `current_result` and `measured_result.status` remain
`not-implemented`, and each measured observation remains null. CORE-001 and
CORE-002 status lives only in their result overlays.

## Compatibility method

Compatibility work must proceed from an independently written behavior case,
not from a translation of another implementation. Each case must identify:

1. the observable behavior and bounded inputs;
2. an independently authored or generated valid fixture;
3. at least one malformed, oversized, stale, or interrupted case where
   relevant;
4. the Rusty LSL result, initially and currently `not-implemented`;
5. an optional black-box result from official liblsl as the oracle endpoint;
6. the exact versions, commands, platforms, and limitations of the comparison.

The damaged-case catalog is
`fixtures/compatibility/negative-case-matrix.json`. Its classifications are
expected outcomes for future tests, not measured oracle results. Cases are
bounded without embedding wire constants, native artifacts, captures, or
implementation-derived protocol bytes.

Agreement on one case supports only that case. A collection of passing cases
does not by itself establish wire or ecosystem compatibility.

## Oracle isolation

Official liblsl may be invoked only by explicit compatibility tooling. It must
not enter the default dependency graph, production binaries, generated source,
or implementation logic. Oracle input and normalized output may become
fixtures only after their origin, license implications, generation command,
and review are recorded in `PROVENANCE.md` or an adjacent fixture manifest.
The reproducible procedure and normalized classification vocabulary are in
`ORACLE.md`.

rLSL source is not an implementation input and must not be used to construct
tests, APIs, protocol behavior, or runtime code.

## Claim changes

A row may move from `Not implemented` only in the same change that adds the
named implementation, positive and damaged tests, provenance records, and
validation instructions. Broader labels such as wire-compatible,
ecosystem-compatible, or supported require an explicitly reviewed definition
and evidence set; they must not be inferred from build success.
