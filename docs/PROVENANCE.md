# Provenance

## Policy

Project-owned source and documentation are licensed `AGPL-3.0-or-later`.
Contributors must author implementation code independently. Do not copy or
translate liblsl or rLSL source, and do not use rLSL source as an
implementation input.

Official liblsl is an MIT-licensed black-box compatibility oracle and reference
endpoint. Its source is not a template. Native libraries, headers, wrappers,
and executables are not production dependencies and are not stored in this
repository by default.

## Artifact classifications

Every fixture, observation, or imported artifact must use one classification:

- `independently-authored`: written from the repository's public behavior
  requirements without copying implementation source;
- `generated`: produced by a named generator from recorded inputs;
- `black-box-observed`: captured from a named public endpoint using recorded
  commands and versions, without inspecting implementation source;
- `adapted`: transformed from a named artifact under a compatible license;
- `copied`: retained verbatim from a named artifact under a compatible license.

`adapted` and `copied` require license and notice review before commit.
Generated and observed material must record whether normalization can discard
semantically relevant details.

## Current inventory

| Paths | Classification | Origin | License |
| --- | --- | --- | --- |
| `crates/**` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `docs/**`, `README.md`, `AGENTS.md` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `tools/**`, Cargo manifests | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `Cargo.lock`, `.gitignore`, `.gitattributes` | generated or independently-authored | Cargo or Rusty LSL contributors | AGPL-3.0-or-later |
| `.github/workflows/**`, `Justfile` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `fixtures/compatibility/**` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `morphospace/**` | generated then independently reviewed and adapted | Rusty Morphospace workflow and Rusty LSL contributors | AGPL-3.0-or-later |
| `THIRD_PARTY_NOTICES.md` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `LICENSE` | copied | Free Software Foundation license text | AGPL-3.0 |

The STRM-000 compatibility fixtures contain only independently authored
specifications, planned observations, damaged-input expectations, and their
digest manifest. There are no measured oracle outputs, captures, third-party
source files, native binaries, vendored dependencies, or generated protocol
artifacts.

`fixtures/compatibility/core-001-contract-results.json` is an independently
authored local-result overlay. It binds Rust unit-test names to the two accepted
STRM-000 contract case identifiers while leaving the accepted baseline and its
digest manifest unchanged. It records no external input or observation and
proves no XML, protocol, wire, runtime, ecosystem, or official-liblsl behavior.

`fixtures/compatibility/core-002-contract-results.json` is a separate
independently authored local-result overlay. It binds exact positive and damaged
local Rust contract tests to raw/derived timestamp values and bounded
timestamped chunks. Its semantic timestamp binding preserves the STRM-000
`not-implemented` result. The `ClockCorrected` and `Smoothed` kinds are
independently authored classifications of caller-provided values, not copied or
observed algorithm behavior. It uses no external implementation input and
proves no clock behavior, timestamp derivation, correction or smoothing
algorithm, transport, protocol, wire, runtime, ecosystem, or official-liblsl
behavior.

`fixtures/compatibility/core-003-contract-results.json` is a separate
independently authored local-results overlay. It binds exact positive and
damaged local Rust tests for bounded descriptors, nominal sample-rate values,
and seven data-only channel-format names. Optional content type and source
correlation are opaque caller inputs; no external identity or authority meaning
was imported. The implementation and overlay use no liblsl, rLSL, generated
protocol material, or other implementation source as input. They prove no XML,
discovery, recovery, timing algorithm, transport, protocol, wire, runtime,
ecosystem, or official-liblsl behavior.

`fixtures/compatibility/core-004-contract-results.json` is a separate
independently authored local-results overlay. It binds exact positive and
damaged local Rust tests for a bounded parent-before-child flat metadata-tree
arena. Its API, tests, and validation gate were authored from this repository's
public unit requirements without liblsl or rLSL source, generated protocol
material, external observations, or copied inputs. It proves no XML, document,
query, mutation, discovery, transport, protocol, wire, runtime, ecosystem, or
official-liblsl behavior.

## Required fixture record

Each fixture family must have an adjacent machine-readable manifest recording:

- stable fixture identifier and classification;
- author or upstream project and canonical public source;
- creation or retrieval date and exact version;
- generator or observation command and normalized inputs;
- license expression, notice requirement, and reviewer decision;
- content digest for external or generated inputs;
- intended compatibility case and what the artifact does not prove.

`rusty.lsl.provenance.manifest.v1` requires these machine-readable roles:

- manifest and case-catalog identity;
- origin classification and SPDX license expression for the manifest and each
  artifact;
- SHA-256 for each referenced artifact;
- toolchain and sanitized environment identity;
- sorted case and negative-case identities;
- normalized observations, empty until an oracle run is reviewed;
- an explicit implementation-input list and source-input prohibitions;
- normalization behavior and a non-empty `does_not_prove` list.

The deterministic baseline instance is
`fixtures/compatibility/baseline-provenance.json`. A measured observation must
add exact oracle release and digest, driver and tool versions, bounded command
arguments, execution environment, case identity, classifications, timestamps,
output digests, and limitations. It must never replace the specification or
erase a failed or setup-failed observation.

Do not commit local filesystem paths, host or device identities, endpoint
addresses from private runs, raw captures, credentials, signing material, or
private planning history.

## Review gate

A dependency or reference artifact may enter only after its task-specific
purpose and license are recorded. A Cargo lockfile proves resolution, not
dependency acceptance. Any material whose origin or redistribution rights are
unclear remains outside the repository until resolved.
