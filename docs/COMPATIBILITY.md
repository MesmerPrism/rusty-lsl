# Compatibility

## Current claim matrix

| Surface | Status | Evidence |
| --- | --- | --- |
| Metadata model and XML | Not implemented | None |
| Discovery and resolution | Not implemented | None |
| Sample and chunk transport | Not implemented | None |
| Channel formats | Not implemented | None |
| Source and local timestamps | Not implemented | None |
| Clock correction and smoothing | Not implemented | None |
| Buffering and post-processing | Not implemented | None |
| Cancellation and timeouts | Not implemented | None |
| Loss detection and recovery | Not implemented | None |
| Provider health and fallback | Not implemented | None |
| C ABI or language wrappers | Not implemented | None |
| Wire compatibility | Not claimed | None |
| Ecosystem compatibility | Not claimed | None |

The compiling crate proves only that the inert facade builds on the tested
toolchain. It does not prove LSL behavior, interoperability, performance, or
platform support.

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

No level is implemented or claimed by this scaffold. Evidence at one level
must not be promoted into a broader claim.

## Compatibility method

Compatibility work must proceed from an independently written behavior case,
not from a translation of another implementation. Each case must identify:

1. the observable behavior and bounded inputs;
2. an independently authored or generated valid fixture;
3. at least one malformed, oversized, stale, or interrupted case where
   relevant;
4. the Rusty LSL result;
5. an optional black-box result from official liblsl as the oracle endpoint;
6. the exact versions, commands, platforms, and limitations of the comparison.

Agreement on one case supports only that case. A collection of passing cases
does not by itself establish wire or ecosystem compatibility.

## Oracle isolation

Official liblsl may be invoked only by explicit compatibility tooling. It must
not enter the default dependency graph, production binaries, generated source,
or implementation logic. Oracle input and normalized output may become
fixtures only after their origin, license implications, generation command,
and review are recorded in `PROVENANCE.md` or an adjacent fixture manifest.

rLSL source is not an implementation input and must not be used to construct
tests, APIs, protocol behavior, or runtime code.

## Claim changes

A row may move from `Not implemented` only in the same change that adds the
named implementation, positive and damaged tests, provenance records, and
validation instructions. Broader labels such as wire-compatible,
ecosystem-compatible, or supported require an explicitly reviewed definition
and evidence set; they must not be inferred from build success.
