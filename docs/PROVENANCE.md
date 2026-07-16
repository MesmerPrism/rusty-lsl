# Provenance

## LSLC-003Q private observation provenance

The independently authored private driver and configuration are bound by
SHA-256 `90c5ed742ac555ef8e2695702f236cf9b9b60b2a77748c0ee2474a32c07c3bf1`
and `41d41bb8b3df2c4988ffac7b6b80cef79dac462289a908de532a7b04b0cc681c`.
Two successful raw results remain private at hashes
`8f9d6258574ff9765a47cf6ade6da353c8086d377ff28df208851e4beb2961d6`
and `db901839eeaa78f924ecf501167f609323427304fffd29b8b8a59a8e2bb9ba03`.
The pinned official binary hash is
`8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff`.
Raw bytes, endpoints, diagnostics, and the exploratory probe remain private;
official and rLSL source were not inspected.

## LSLC-003P

The implementation is independently authored from the accepted LSLC-003B
runtime and LSLC-003O sanitized black-box observations. Official source and
rLSL source were not inspected or used. Private drivers, raw records,
endpoints, and diagnostics remain outside the repository.

## LSLC-003O private observation provenance

The public fixture binds the independently authored driver, configuration,
pinned official binary, and two pinned raw result files by SHA-256. Two prior
successful behavioral runs under package version 1.18.1 are retained privately
and classified as non-acceptance evidence because they did not match the
accepted 1.18.2 oracle package. Official implementation source was not read.
Raw records, request/response bytes, documents, endpoints, identifiers,
diagnostics, environment paths, binaries, caches, and harness details remain
private.

## LSLC-002Q official responder observation

The independently authored external driver exercised the accepted LSLC-002P
Rust client against the same pinned official PyPI Windows AMD64 `pylsl 1.18.2`
wheel, library version 117, protocol version 110, and native DLL digest already
recorded by LSLC-002E. Only documented `StreamInfo` and `StreamOutlet` APIs
were invoked; implementation source was not inspected. The public fixture
binds the private orchestration driver, Rust driver, and raw observation by
SHA-256. The initial private compile-only harness failure is retained as failed
attempt evidence. Raw packets, XML, endpoint/runtime values, diagnostics,
binaries, environments, and caches remain external.

LSLC-002L binds `docs/info/time_synchronization.rst` at public revision
`f012f8cfe8894cab0529be77dd83c91d6d95537d` and exact UTF-8 SHA-256
`35bacbc81477d7e08554e42c6fa25382622954adecded9ab2101bf2061fc883e`.
Only documentation specification facts are reduced; implementation links in
the document were not followed and no black-box observation was performed.

LSLC-002I reuses the pinned official network-connectivity document and exact
UTF-8 SHA-256 already admitted by LSLC-002C/D, extracting only displayed
default-settings UDP port and destination spellings. No implementation source
or black-box endpoint was inspected.

## LSLC-002E response observation

The observation used the official PyPI Windows AMD64 `pylsl 1.18.2` wheel
already identified by SHA-256
`3ea2693417c7d79766cebf967250fde78aa1a3ad2b198e40246d36f549dbfde1`,
public library version 117, protocol version 110, and native DLL SHA-256
`8156d0021794135ce217821cae0e99912753d86d8519e349756d13d99e0292ff`.
An independently authored external driver used documented `StreamInfo` and
`StreamOutlet` APIs and a bounded loopback-only UDP probe. No implementation
source was inspected. Raw artifacts, diagnostics, and local values remain
external; their hashes bind the sanitized fixture.

## LSLC-002D correction

The pinned public RST source and SHA-256 remain unchanged. Its indentation
belongs to RST presentation. The logged packet count is 65 bytes; the three
displayed content lines contribute 59 bytes, and three CRLF delimiters
contribute six. LSLC-002D records that explicit inference and retains the
rejected LSLC-002C LF-only fixture and commits as additive failure evidence.

## LSLC-002C public specification intake

The sole external input is the public
`docs/info/network-connectivity.rst` file at labstreaminglayer revision
`f012f8cfe8894cab0529be77dd83c91d6d95537d`, which displays a 65-byte
`LSL:shortinfo` query example. The fixture records its URL, revision, UTF-8
digest, and displayed bytes. All additional boundaries and damaged cases are
independently authored Rusty LSL candidate policy. No installed liblsl, rLSL,
implementation source, native library, raw capture, or external oracle was
used.

## LSLC-002B

The fixture matrix is independently authored from accepted public
LSLC-001H/K/L/R/002A evidence. No liblsl or rLSL implementation source is an
input.

## LSLC-002A parser fixtures

LSLC-002A's fixture document and mutations are independently authored from the
accepted public LSLC-001R shape contract and XML 1.0 character constraints.
They contain synthetic values only. No liblsl/rLSL source, raw capture,
endpoint value, native binary, network traffic, device evidence, or private
machine context was used or committed. The cases are candidate validation
evidence, not oracle observation or interoperability measurement.

LSLC-001Z is independently authored from the accepted N, P, Q, R, and X public
contracts. Its fixtures are synthetic local contract evidence; no external
implementation source, endpoint, network, device, or raw capture is used.

LSLC-001X is independently authored from accepted T/U/V/S contracts. Its
providers, values, owner identities, epochs, and revisions are synthetic local
fixtures. No external implementation source, host/interface/network state,
native library, or raw capture is an input.

LSLC-001V is independently authored from accepted O/S/T/U contracts. Its test
provider, witness, and six endpoint strings are synthetic. No external source,
interface, socket, network, native library, raw capture, or implementation
observation is an input.

LSLC-001U is independently authored from accepted O/S/T contracts. Its test
providers, witnesses, and four values are synthetic. No external source,
clock, environment, host, network, native library, or raw capture is an input.

## LSLC-001T

LSLC-001T is independently authored from accepted LSLC-001O/S contracts and
repository ownership rules. No liblsl or rLSL source, binary observation,
native library, host value, network value, or external implementation input was
used. Focused-test providers and their evidence are synthetic local fixtures.

LSLC-001S is independently authored local candidate policy derived only from
accepted LSLC-001O role/class evidence. It uses no external implementation
source and makes no measured freshness or acquisition claim.

## LSLC-001R local observation-bound overlay

`lslc-001r-observed-stream-info-document-envelope-results.json` is an
independently authored candidate overlay bound to accepted black-box H evidence
and accepted local G/Q artifacts. Official binary observations supply exact
behavioral facts only; no official or third-party source is an implementation
input.

## LSLC-001Q local composition overlay

`lslc-001q-ordered-stream-info-element-results.json` is independently authored
and hash-binds accepted H/N/P public evidence. Official observations supply
compatibility facts only; no official or third-party source is an
implementation input. The implementation is an original consuming arena merge.

## LSLC-001P local representation overlay

`fixtures/compatibility/lslc-001p-volatile-stream-info-xml-results.json` is
independently authored and binds accepted XML value/tree, observation, and
volatile-data artifacts. Its implementation inputs are empty; it records local
representation policy only, not provider or complete-document behavior.

## LSLC-001O local data overlay

`fixtures/compatibility/lslc-001o-volatile-stream-info-data-results.json` is an
independently authored local-results overlay. It hash-binds the frozen
LSLC-001A corpus, accepted LSLC-001H case/observation/provenance artifacts, and
accepted LSLC-001N receipt. Its implementation inputs are empty. The role order
comes from public-safe black-box observations; the three ownership classes,
bounds, validation order, and opaque retention policy are local candidate
contracts rather than observed provider or runtime behavior.

## LSLC-001N

`fixtures/compatibility/lslc-001n-description-xml-results.json` is an
independently authored local-results overlay over hash-bound accepted H, F, G,
and M artifacts. Its implementation-input list is empty; no external
implementation source was inspected or used.

## LSLC-001M

`fixtures/compatibility/lslc-001m-static-stream-info-xml-results.json` is an
independently authored local-results overlay. Its technical inputs are the
hash-bound accepted H, G, K, and L public artifacts. Implementation inputs are
empty: no liblsl, pylsl, rLSL, wrapper, application, generated, or external
implementation source was used.

## LSLC-001J protected-tree identity

The protected production and activation surface is identified from current
`HEAD`, without an ancestor: the exact binary stdout of `git ls-tree -r
--full-tree HEAD` over the Rust crate, both Cargo files, the feature lock, and
the project specification has 21 entries and SHA-256
`ee776163e904ea3c6eb336dd1855d12f0def3e257634272e0c33e7b6e784d8e1`.
The pinned manifest binds every path, file mode, blob identity, separator, and
line ending. Separate HEAD comparisons cover the combined index and working
tree, and an untracked-path inventory covers every protected root.

GitHub Actions runs `29276386135` and `29278122366` remain separate failed
pre-fix evidence. Run `29278122366` passed all 134 Rust tests and LSLC-001A
through LSLC-001G before failing on absent shallow-checkout history at
`9650de4`. Disposable local file clones provide only synthetic validation
evidence: one depth-1 pass with that revision absent, plus rejected tracked,
staged, untracked, and committed manifest mutations. They are removed after
each gate and supply no implementation input.

This correction changes no accepted oracle evidence or LSLC-001I driver
binding and proves no source semantics, candidate output, protocol, wire,
runtime, dependency, platform, compatibility, publication, or authority
behavior.

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

`fixtures/compatibility/lslc-001l-static-numeric-spelling-results.json` is an
independently authored local-results overlay. Its technical inputs are the
accepted LSLC-001H cases and public observations plus the accepted LSLC-001K
semantic overlay. Its implementation inputs are empty: no oracle, wrapper,
liblsl, rLSL, application, generated, vendored, or external implementation
source was inspected or used. The overlay preserves observation and candidate
separation and claims only the two recorded lexical fields.

`fixtures/compatibility/lslc-001k-stream-info-static-field-results.json` is an
independently authored local-results overlay. It binds the exact accepted
LSLC-001H case-manifest and observation-overlay SHA-256 values while leaving
those artifacts unchanged. Public black-box observations supply only the six
static role order and seven format spellings; no liblsl, rLSL, wrapper,
application, generated, protocol, build, vendored, or test source supplied an
implementation input. The overlay contains semantic values only, not captured
endpoint bytes or volatile runtime data.
# LSLC-002R black-box provenance

The observation uses the exact pinned `pylsl 1.18.2` Windows AMD64 wheel,
public library version 117, protocol version 110, and previously recorded
wheel/native-library digests. The loopback configuration is derived only from
the pinned official public `docs/info/lslapicfg.rst` document at revision
`f012f8cfe8894cab0529be77dd83c91d6d95537d`; no implementation source was
inspected or translated.

The independently authored private driver, configuration, and raw observation
are bound by SHA-256 in the sanitized fixture. Raw XML, connection rows,
endpoint and runtime values, native diagnostics, binary/environment/cache
paths, and failed attempts stay outside Git. Their digests establish exact
provenance without publishing their contents.
## LSLC-002S black-box framing provenance

The pinned `pylsl 1.18.2` Windows AMD64 wheel, public library/protocol versions,
and loaded native-library digest match the accepted LSLC-002R endpoint. Driver
and raw-output SHA-256 values bind two private active probes. No implementation
source was inspected, and raw request/response bytes, XML, identifiers,
endpoints, diagnostics, binaries, environments, caches, and local paths are
not committed.
## LSLC-002T sample-record provenance

The endpoint provenance remains the pinned LSLC-002R/S official package and
native binary. A separately hashed private active probe used documented outlet
and sample APIs with one explicit timestamp/value input. No implementation
source was inspected. Raw handshake/sample bytes, XML, identifiers, endpoints,
diagnostics, binaries, environments, caches, and local paths are not committed.
## LSLC-002U clock framing provenance

Two private active probes used the pinned official endpoint and documented
time-correction surface: one synthetic service captured requests; one
independently authored request obtained an official response. Driver and raw
outputs are bound by SHA-256. No implementation source was inspected, and raw
bytes, numeric clock values, XML, identifiers, endpoints, diagnostics,
environments, caches, and local paths are not committed.
## LSLC-002X official Float32 interoperability observation

The observation pins the same official `pylsl 1.18.2` Windows AMD64 wheel,
library version 117, wheel digest, and native-library digest as LSLC-002S/T.
The public fixture binds independently authored private Rust/Python driver,
binary, and raw-result SHA-256 values. No official implementation source was
inspected. Raw bytes, XML, endpoints, diagnostics, environments, binaries, and
caches remain outside the repository.
## LSLC-002Y correction and rerun evidence

LSLC-002Y uses only accepted public-safe LSLC-002S/T/X role evidence and fresh
black-box reruns through the pinned official public APIs. The fixture binds
private driver, rebuilt Rust binary, and raw-result hashes. Official source was
not inspected; raw bytes, XML, endpoints, diagnostics, environments, binaries,
and caches remain private.
# LSLC-002Z private observation provenance

The public fixture binds SHA-256 values for the independently authored public-
API observation driver, Rust driver, raw private result, and private binary.
Raw packets, XML, endpoints, diagnostics, binaries, environments, and caches
remain outside the repository. No official implementation source was read.
# LSLC-003A provenance

The sanitized matrix binds the independently authored private driver,
configuration, and raw result by SHA-256. Exact packets, documents, endpoints,
diagnostics, binaries, environments, caches, and harness details remain private.
No official implementation source was inspected.
# LSLC-003B

Public evidence binds private drivers, raw outcome, and binary by SHA-256. Raw
packets, documents, endpoints, diagnostics, environments, and caches stay private.

## LSLC-003T

LSLC-003T is independently authored from the sanitized LSLC-003Q black-box
observation and LSLC-003S activation contract. Official liblsl and rLSL source
were not implementation inputs; private drivers, raw records, endpoints, and
machine identities remain outside the repository.

## LSLC-003U

The public fixture contains only independently authored value classifications,
dimensions, exact framing outcomes, and SHA-256 bindings. Drivers, raw results,
endpoints, diagnostics, machine identities, and four successful but unpinned
pylsl 1.18.1 drift runs remain private and are excluded from acceptance.
