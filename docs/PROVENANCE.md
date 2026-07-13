# Provenance

## LSLC-001I canonical driver-source digest basis

The two existing LSLC-001H driver bindings retain their original SHA-256
values and now explicitly identify `canonical-lf-source-bytes` as their digest
basis. The checker reads current working-tree bytes, accepts complete LF or a
byte-equivalent complete CRLF checkout, replaces only CRLF pairs with LF, and
hashes those canonical bytes. It rejects mixed LF/CRLF, lone carriage returns,
and any other source-byte mutation rather than treating general text or
semantic equivalence as provenance identity.

This is a validation-provenance correction for the failed pre-fix GitHub
Actions run `29276386135`, not a new observation or normalization of captured
evidence. Both oracle drivers, all accepted observations and raw artifact
digests, the capture window, normalization records, external identities,
limitations, and historical fixture digests remain unchanged. The deterministic
damaged checks operate only on in-memory copies and do not rerun the oracle.

## LSLC-001H black-box observation

`fixtures/compatibility/lslc-001h-stream-info-xml-provenance.json` records the
official wheel URL/version/SHA-256, public library version, native DLL digest,
Python identity, bounded command vectors, sanitized environment, driver and
case/corpus bindings, external stdout/stderr and capture-record digests,
raw-output hashes, normalization policy, and append-only typed failure policy.
No wheel, DLL, environment, cache, raw XML, native diagnostics, or package/
native source is committed or used as implementation input.

`fixtures/compatibility/lslc-001h-stream-info-xml-observations.json` is
`black-box-observed`; the case manifest and tools are independently authored.
The observed overlay neither changes the `independently-authored` frozen
LSLC-001A corpus nor promotes local LSLC-001C/G candidate policy.

## LSLC-001G local overlay

`fixtures/compatibility/lslc-001g-contract-results.json` is a separate
independently authored local-results overlay. Its technical inputs are the
accepted LSLC-001A through LSLC-001F contract artifacts. No liblsl, rLSL,
wrapper, application, generated, protocol, or other implementation source
supplied implementation input, and it contains no endpoint output. It
preserves every historical digest and every LSLC-001A oracle/candidate null
state. It proves only bounded borrowed local string serialization, not complete
documents, parsing, decoding, LSL mapping, protocol, wire, runtime, ecosystem,
compatibility, or official-liblsl behavior.

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

`fixtures/compatibility/core-005-contract-results.json` is a separate
independently authored local-results overlay. It binds exact positive and
damaged local Rust tests for descriptor/sample format and channel-shape
matching, per-String-channel Unicode scalar bounds, and unchanged homogeneous
values. Its source, tests, and gate were authored from this repository's public
unit requirements without external implementation input. It proves no
conversion, encoding, layout, wire, protocol, transport, runtime, ecosystem,
or official-liblsl behavior.

`fixtures/compatibility/core-006-contract-results.json` is a separate
independently authored local-results overlay. It binds exact positive and
damaged local Rust tests for composing the existing timestamped-sample and
descriptor/sample contracts while retaining their exact values and delegated
errors. Its source, tests, and gate were authored from this repository's public
unit requirements without external implementation input. It proves no clock
read or timestamp algorithm, rewriting, buffering, conversion, encoding,
layout, wire, protocol, transport, runtime, ecosystem, or official-liblsl
behavior.

`fixtures/compatibility/core-007-contract-results.json` is a separate
independently authored local-results overlay. It binds exact positive and
damaged local Rust tests for composing an existing non-empty timestamped chunk
with a descriptor through CORE-006 while retaining original limits, order,
pairings, and indexed unchanged delegated errors. Its source, tests, and gate
were authored from this repository's public unit requirements without external
implementation input. It proves no actual LSL empty-chunk behavior, clock or
timestamp algorithm, splitting, merging, rechunking, buffering, queueing,
conversion, encoding, layout, wire, protocol, transport, runtime, ecosystem,
or official-liblsl behavior.

`fixtures/compatibility/core-008-contract-results.json` is a separate
independently authored local-results overlay. It binds exact local Rust tests
for composing one existing validated stream descriptor with one existing
validated generic metadata tree while retaining both contracts unchanged. Its
source, tests, and gate were authored from this repository's public unit
requirements without external implementation input. It gives the generic root
no XML or LSL `desc` meaning and proves no discovery, transport, protocol,
wire, runtime identity, authority, ecosystem, or official-liblsl behavior.

## Required fixture record

`fixtures/compatibility/lslc-001a-stream-info-document-corpus.json` is the
independently authored LSLC-001A public-documentation corpus. Its only external
technical references are the exact recorded liblsl Stream Info API page and
W3C XML 1.0 Fifth Edition Recommendation, both accessed on 2026-07-13. The
corpus retains concise claim identifiers and independently worded summaries;
`source_code_used` and `implementation_input` are false for both records. It
contains no copied prose, implementation source, endpoint output, XML payload,
or serialization observation. Its bounds are local future-harness policy and
do not describe liblsl limits or behavior.

`fixtures/compatibility/lslc-001b-contract-results.json` is a separate
independently authored local-results overlay. Its only technical specification
inputs are the accepted LSLC-001A corpus and the recorded W3C XML 1.0 Fifth
Edition Recommendation. The source, tests, and checker use no liblsl, rLSL,
wrapper, application, test, build, vendored, generated, or protocol
implementation source and contain no endpoint output. The overlay records only
bounded Rust value validation and preserves every LSLC-001A oracle/candidate
role unchanged. It proves no representation, document, LSL, protocol, wire,
runtime, ecosystem, or official-liblsl behavior.

`fixtures/compatibility/lslc-001c-contract-results.json` is a separate
independently authored local-results overlay. Its only technical inputs are the
accepted LSLC-001A character-data role and LSLC-001B bounded legal-text value
contract. Its source, tests, and checker use no liblsl, rLSL, wrapper,
application, test, build, vendored, generated, protocol, or other
implementation source and contain no endpoint output. The overlay labels the
fixed `&`, `<`, and global `>` replacements as Rusty LSL local candidate
policy, preserves all LSLC-001A oracle/candidate roles unchanged, and proves no
document, LSL mapping, protocol, wire, runtime, ecosystem, or official-liblsl
behavior.

`fixtures/compatibility/lslc-001d-contract-results.json` is a separate
independently authored local-results overlay. Its only technical inputs are the
accepted LSLC-001A roles and the LSLC-001B/LSLC-001C accepted component
contracts. Its source, tests, and checker use no liblsl, rLSL, wrapper,
application, test, build, vendored, generated, protocol, or other
implementation source and contain no endpoint output. The overlay preserves
all LSLC-001A oracle/candidate roles unchanged and proves only exact local
two-component composition, not tag, tree, document, LSL mapping, protocol,
wire, runtime, ecosystem, or official-liblsl behavior.

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

`fixtures/compatibility/lslc-001e-contract-results.json` is a separate
independently authored local-results overlay. Its only technical inputs are the
accepted LSLC-001A structural roles and LSLC-001B through LSLC-001D component
contracts. Its source, tests, and checker use no liblsl, rLSL, wrapper,
application, generated, protocol, or other implementation source and contain
no endpoint output. It preserves every historical digest and all LSLC-001A
oracle/candidate roles unchanged. It proves only the bounded local hierarchy,
not document, serialization, LSL mapping, protocol, wire, runtime, ecosystem,
compatibility, or official-liblsl behavior.

## Review gate

A dependency or reference artifact may enter only after its task-specific
purpose and license are recorded. A Cargo lockfile proves resolution, not
dependency acceptance. Any material whose origin or redistribution rights are
unclear remains outside the repository until resolved.

`fixtures/compatibility/lslc-001f-contract-results.json` is a separate
independently authored local-results overlay. Its only technical inputs are the
accepted CORE-004 and LSLC-001A through LSLC-001E contract artifacts. No
liblsl, rLSL, wrapper, application, generated, protocol, or other
implementation source supplied implementation input, and the overlay contains
no endpoint output. It preserves every historical digest and all LSLC-001A
oracle/candidate roles unchanged. It proves only the bounded local consuming
projection, not decoding, round trips, documents, serialization, LSL mapping,
protocol, wire, runtime, ecosystem, compatibility, or official-liblsl behavior.
