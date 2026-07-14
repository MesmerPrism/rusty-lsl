# Rusty LSL Agent Notes

## LSLC-002P Bounded UDP Discovery Activation

LSLC-002P is the first explicit runtime effect: one synchronous, caller-bound
UDP socket sends one already accepted short-info query to one caller-selected
destination and admits only bounded accepted response envelopes. Nonzero
datagram/count/receive-slice/total-deadline limits, an explicit cancellation
flag, checked/fallible allocation, typed failures, and scope-owned socket drop
bound every call. Loopback tests cover valid, malformed, oversized, timeout,
cancellation, response-limit, and immediate port-rebind cleanup paths.

The selected feature lock plus the explicit call/configuration opens only this
effect. It adds no interface enumeration, multicast join, endpoint selection,
retry/background runtime, official-endpoint interoperability, currentness,
outlet/inlet/sample behavior, device behavior, or Manifold authority. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002p.ps1`.

## LSLC-002O Explicit Clock-Offset Application Contract

LSLC-002O owns one finite bit-preserving offset value and one explicit
caller-invoked addition to an accepted raw timestamp. A finite result is
labelled through the existing `ClockCorrected` derived timestamp; non-finite
offsets and sums fail typed without partial state.

It adds no offset acquisition, selection, history, automatic mapping,
clock/timer read, packet, I/O, UDP, retry, scheduling, smoothing, dejitter,
drift fitting, currentness, runtime, dependency, device, unsafe/FFI, or
Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002o.ps1`.

## LSLC-002N Bounded Minimum-RTT Selection Contract

LSLC-002N requires an explicit nonzero result-count maximum, rejects empty and
oversized batches, retains the caller's original `Vec`, and selects the first
input with the numerically minimum already finite LSLC-002M RTT. First-on-tie
is local candidate policy, not observed endpoint behavior.

The unit adds no exchange acquisition, packet, clock/timer read, I/O, UDP,
retry, scheduling, periodic execution, implicit default count, broader
statistical filtering, offset history, correction/mapping, smoothing, dejitter,
currentness, runtime, dependency, device, unsafe/FFI, or Manifold authority.
Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002n.ps1`.

## LSLC-002M Raw Clock-Exchange Formula Contract

LSLC-002M owns only four finite opaque `f64` values in `t0` through `t3` role
order and a separate fallible evaluation of the LSLC-002L-documented RTT and
OFS formulas. Inputs retain exact bits; non-finite inputs reject in role order,
and non-finite arithmetic rejects at the first typed intermediate stage.

It imposes no timestamp ordering or clock-domain meaning and adds no packet,
clock/timer read, I/O, UDP, retry, scheduling, eight-exchange collection,
filter/minimum selection, history, correction/mapping, smoothing, dejitter,
currentness, runtime, dependency, device, unsafe/FFI, or Manifold authority.
Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002m.ps1`.

## LSLC-002L Clock-Offset Public Documentation Evidence

LSLC-002L records only pinned official documentation facts: eight exchanges by
default, the displayed RTT and OFS formulas, minimum-associated-RTT selection,
periodic measurement wording, symmetric-delay limitation, half-asymmetry bias,
and recipient-owned timestamp mapping. No implementation source was inspected.

This unit adds no timestamp or numeric contract, clock/timer read, packet
shape, socket, UDP exchange, retry, scheduling, filter, offset history,
correction, smoothing, dejitter, currentness, runtime, dependency, device,
unsafe/FFI, or Manifold authority. Run `powershell -NoProfile -ExecutionPolicy
Bypass -File .\tools\check_lslc_002l.ps1`.

## LSLC-002K Documented Discovery Query Proposal Composition

LSLC-002K infallibly moves one caller-selected LSLC-002J destination label
beside one accepted LSLC-002D query wire value. Borrowed and consuming access
preserve the exact destination data and query allocation; port 16571 remains a
documented numeric fact, not endpoint authority.

The proposal performs no address interpretation, destination expansion or
selection policy, socket operation, send, interface enumeration, multicast
join, broadcast configuration, response collection, discovery runtime,
reachability, interoperability, activation, device behavior, dependency,
unsafe/FFI, or Manifold authority. Run `powershell -NoProfile -ExecutionPolicy
Bypass -File .\tools\check_lslc_002k.ps1`.

## LSLC-002J Documented Discovery Destination Data Contract

LSLC-002J owns only a closed, allocation-free data inventory for the pinned
LSLC-002I public-documentation facts: UDP port 16571, seven exact displayed
destination spellings in source order, and whether each spelling was shown in
parentheses. The unusual `FF08113D` token remains unchanged and uninterpreted.

This unit performs no address parsing, validation, normalization, correction,
classification, binary conversion, selection, socket operation, interface
enumeration, multicast join, broadcast send, discovery runtime, reachability,
interoperability, activation, device behavior, dependency, unsafe/FFI, or
Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002j.ps1`.

## LSLC-002I Default Discovery Destination Evidence

LSLC-002I records only pinned official public-documentation facts for default
settings: UDP broadcasts and/or multicast on decimal port 16571 and seven exact
displayed destination spellings. It preserves the unusual `FF08113D...`
spelling and presentation parentheses without parsing, normalization, or
correction. This is specification evidence, not observation, candidate
contract, socket behavior, discovery runtime, or interoperability. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_002i.ps1`.

## LSLC-002H Typed Short-Info Response Observation

LSLC-002H consumes one accepted response envelope into its unchanged
uninterpreted identifier and existing LSLC-002B typed fields. Admission errors
delegate unchanged. The composition adds no matching, lookup, correlation,
currentness, endpoint, networking, runtime, activation, or Manifold authority.
Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002h.ps1`.

## LSLC-002G Response-Envelope Validation Correction

LSLC-002G additively corrects two primary-review gaps in accepted LSLC-002F.
A named regression now inserts a second CRLF before a valid LSLC-002A body and
binds the complete-envelope body-start offset plus unchanged delegated parser
error. The crate status and ownership declaration now name the already existing
bounded local source-only response-envelope contract while continuing to deny
endpoint response behavior and networking. LSLC-002F behavior and history are
unchanged. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002g.ps1`.

## LSLC-002F Short-Info Response Envelope

LSLC-002F owns only a dependency-free, explicitly bounded canonical response
envelope: one uninterpreted canonical `u64` decimal identifier, exact CRLF, and
one unchanged LSLC-002A-shaped document. Parsing borrows the complete source
and delegates body shape admission to LSLC-002A; LSLC-002B typed admission is a
separate explicit caller action. Encoding uses checked sizing and one exact
fallible reserve. It adds no correlation semantics, socket, discovery runtime,
endpoint ownership, retry, clock, currentness, interoperability, dependency,
feature, device, activation, or Manifold authority. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_002f.ps1`.

## LSLC-002E Short-Info Response Black-Box Observation

LSLC-002E records an exact official protocol-110 endpoint observation without
adding response implementation. Two bounded loopback-only cases returned the
canonical decimal query identifier, CRLF, then one LSLC-002A-shaped document.
The document begins immediately after CRLF, retains its internal LF layout and
final LF, and both extracted bodies passed existing LSLC-002A parsing and
LSLC-002B typed admission unchanged.

Raw datagrams, XML, diagnostics, environments, caches, local paths, and
runtime/host/endpoint values remain private. This unit adds no encoder, parser,
correlation semantics, socket, discovery runtime, endpoint ownership, retry,
clock, currentness, interoperability, dependency, feature, device, activation,
or Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass
-File .\tools\check_lslc_002e.ps1`.

## LSLC-002D Short-Info Query CRLF Correction

LSLC-002D additively corrects rejected LSLC-002C. The pinned public document
logs a 65-byte packet; its three displayed content lines contain 59 bytes, so
three CRLF delimiters account for the remaining six bytes. RST indentation is
presentation only. Encoding now emits CRLF after all three lines and parsing
admits only that form with exact first failure offsets.

LSLC-002C history remains unchanged evidence of the rejected 62-byte LF-only
candidate. This correction adds no query semantics, response, socket,
multicast, endpoint, discovery runtime, retry, clock, currentness, provider,
interoperability, activation, device, dependency, feature, or Manifold
authority. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002d.ps1`.

## LSLC-002C Protocol-110 Short-Info Query Wire Shape

LSLC-002C owns only one dependency-free, explicitly byte-bounded canonical
three-line query payload: `LSL:shortinfo`, one nonempty printable-ASCII query
line, and canonical unsigned decimal nonzero return-port plus query identifier,
with LF after every line. Encoding uses checked sizing before one exact
fallible reserve; parsing borrows the unchanged source and retains typed first
failure offsets. Public-safe fixtures are independently authored from the
official public network-connectivity documentation example.

The numeric fields remain uninterpreted. This unit adds no query evaluation,
response shape, endpoint semantics, sockets, datagrams, multicast, interface or
endpoint selection, discovery runtime, retries, clocks, currentness, provider
evidence, interoperability, runtime activation, device behavior, dependency,
feature, or Manifold authority. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_002c.ps1
```

## LSLC-002B Observed Document Typed Admission

LSLC-002B consumes accepted LSLC-002A state, decodes only its three admitted
entity spellings, and delegates into existing `StreamDefinition` and
`StreamInfoVolatileFields` contracts. It adds no owner witness, currentness,
endpoint semantics, wire interoperability, networking, runtime activation, or
Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002b.ps1`.

## LSLC-002A Bounded Observed Document Shape Parser

LSLC-002A borrows one UTF-8 string and admits only the exact empty-description
LSLC-001R document shape under one caller-selected nonzero byte maximum. One
forward scan checks the observed declaration, LF/tab layout, fixed seventeen
field order, represented character data, exact closing tags, `<desc />`, root
close, and final LF. Accepted state borrows the unchanged source and owns only
a fixed seventeen-range index array; parsing performs no structural heap
allocation. Typed failures retain the first failing byte offset.

This is not a general XML parser, semantic field decoder, raw endpoint or wire
claim. It adds no attributes, namespaces, DTD, CDATA, generic entities,
provider/acquisition behavior, endpoint semantics, discovery, clocks, sockets,
networking, runtime activation, device, dependency, feature, or Manifold
authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_002a.ps1
```

## LSLC-001Z Three-Owner Observed Document Composition

LSLC-001Z consumes one accepted LSLC-001N static-description composition and
one accepted LSLC-001X snapshot. One explicit facade call projects the accepted
volatile fields through P, consumes N and P through Q, and projects Q through R.
The result owns the bounded observed document beside the unchanged
implementation, runtime, and transport witnesses as three separately
inspectable owner artifacts.

The facade adds no acquisition, parsing, raw endpoint or wire claim, common
owner epoch or revision, freshness, authorization, endpoint semantics,
discovery, clocks, sockets, network inspection, runtime activation, device,
dependency, feature, or Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001z.ps1
```

## LSLC-001X Three-Owner Acquisition Snapshot Composition

LSLC-001X consumes one already accepted T, U, and V acquisition. Their
implementation, runtime, and transport witnesses move into three separately
inspectable owner artifacts while all eleven original opaque value allocations
move through the three fixed LSLC-001S lanes into one complete admitted
snapshot. The composer compares or combines no owner identity, epoch, or
revision and infers no cross-owner atomicity, freshness, currentness,
authorization, or activation. It performs no acquisition, ambient inspection,
socket/network operation, runtime effect, device behavior, dependency/feature
activation, or Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001x.ps1
```

## LSLC-001V Transport-Owned Acquisition Evidence

LSLC-001V calls one explicit caller-selected provider once to acquire opaque
`v4address`, `v4data_port`, `v4service_port`, `v6address`, `v6data_port`, and
`v6service_port` values together. All six share one separately inspectable
bounded transport-owner identity/epoch/revision witness; currentness requires
exact expected-witness match. Values validate in fixed LSLC-001O order and may
move only into the six-value LSLC-001S transport lane. The unit inspects no
interfaces, parses no endpoint semantics, opens no sockets, reads no network,
and claims no reachability, authorization, activation, device, runtime effect,
or Manifold route/session/topology authority. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001v.ps1
```

## LSLC-001U Runtime-Assigned Acquisition Evidence

LSLC-001U calls one explicit caller-selected provider once to acquire opaque
`created_at`, `uid`, `session_id`, and `hostname` values together. All four
share one separately inspectable bounded owner identity/epoch/revision witness;
currentness requires exact expected-witness match. Values then validate in
fixed LSLC-001O order and may move only into the four-value LSLC-001S runtime
lane. The unit reads no clock, environment, or host state, generates no
identity, and performs no selection, transport, networking, activation,
device, or Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001u.ps1
```

## LSLC-001T Implementation-Version Acquisition Evidence

LSLC-001T adds one dependency-free, explicitly invoked provider trait for only
the implementation-assigned `version` role. One caller-selected provider is
called once. Its opaque version allocation remains separate from a bounded
owner-issued witness naming provider identity, epoch, and revision; currentness
is accepted only by exact match against the caller's expected owner witness,
never from a clock, arrival time, or local inference.

An accepted acquisition can move only `version` into one LSLC-001S provider
value. It does not construct or admit the complete S snapshot and adds no
runtime- or transport-owned acquisition, provider selection, host inspection,
retry, background work, sockets, networking, activation, device, feature, or
Manifold authority. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001t.ps1
```

## LSLC-001S Volatile Provider Snapshot Admission

LSLC-001S adds only a dependency-free one-shot admission contract over three
explicit caller-supplied lanes: one implementation-assigned role, four
runtime-assigned roles, and six transport-owned roles. It rejects oversized
lanes, cross-lane roles, duplicates, and the first missing LSLC-001O role
before moving a complete snapshot into `StreamInfoVolatileFields`; O retains
all opaque-text bounds and address/port values remain uninterpreted.

Acceptance proves neither freshness nor currentness because this contract has
no clock, revision, epoch, or freshness witness. It performs no acquisition,
provider selection, environment or host inspection, identity generation,
socket or network operation, XML/document representation, transport, runtime
activation, device behavior, feature effect, or Manifold authority. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001s.ps1
```

## LSLC-001R Observed Stream-Info Document Envelope

LSLC-001R borrows one accepted `StreamInfoOrderedXml` and projects one owned,
explicitly byte-bounded UTF-8 string. Its specialized policy emits exactly the
LSLC-001H-observed XML 1.0 declaration followed by LF, one horizontal tab per
element depth below `info`, LF after every element line, `<desc />` only for an
empty fixed description root, and one final LF. Accepted element names and
represented character data are emitted unchanged. A childless description
container other than `desc` fails closed because its empty spelling was not
observed.

This projection is separate from and does not modify LSLC-001G compact
serialization. It is local observation-bound representation evidence, not a
parser, canonical XML engine, raw endpoint or wire claim. It adds no provider,
clock or host inspection, identity generation, address/port semantics,
networking, discovery, protocol, transport, runtime, adapter, device, feature,
or Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001r.ps1
```

## LSLC-001Q Ordered Stream-Info Element Composition

LSLC-001Q adds only a dependency-free consuming merge of one accepted
`StreamInfoDescriptionXml` and one accepted `StreamInfoVolatileXml`. The final
`info` tree retains six static leaves, eleven volatile leaves, then `desc` in
the accepted LSLC-001H order. It validates both fixed component shapes, checks
the exact root-sharing total and target node bound before one exact fallible
reserve, discards only the duplicate volatile `info` root, and adds eleven only
to parents inside the description subtree. Component names and represented
character data move unchanged without cloning.

The result is a compact local element tree, not an observed endpoint document.
It adds no XML declaration, observed whitespace or self-closing spelling,
parser, provider, clock or host inspection, identity generation, address or
port ownership semantics, protocol, transport, networking, runtime, adapter,
device, feature, or Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001q.ps1
```

## LSLC-001P Bounded Volatile Stream-Info XML Composition

LSLC-001P borrows one accepted LSLC-001O field set and projects its eleven
opaque values into a twelve-node `XmlElementTree`. The root is `info`; direct
leaves remain in accepted `version` through `v6service_port` order. The target
node bound precedes one exact arena reserve, and every fixed name and value is
copied separately through accepted LSLC-001B/C/E contracts.

The source data remains unchanged and reusable. This local element tree adds no
provider, acquisition, static or `desc` merge, XML declaration, observed
whitespace, self-closing spelling, complete document, clock/host/identity
generation, endpoint semantics, networking, runtime, feature, device, or
Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001p.ps1
```

## LSLC-001O Bounded Volatile Stream-Info Data

LSLC-001O adds only a dependency-free bounded accepted-data contract for the
eleven volatile roles observed by LSLC-001H. Fixed role order is `version`,
`created_at`, `uid`, `session_id`, `hostname`, `v4address`, `v4data_port`,
`v4service_port`, `v6address`, `v6data_port`, and `v6service_port`. `version`
is implementation-assigned; creation, identity, session, and host fields are
runtime-assigned; address and port fields are transport-owned.

Three separate nonzero maxima count Unicode scalar values for those classes.
Limits reject in implementation, runtime, then transport order; values reject
in fixed role order. Empty and arbitrary opaque text is accepted unchanged.
Accepted state owns only the limits and original eleven `String` allocations.

This data layer does not acquire values from a provider or assert that they are
current, generated, unique, numeric, parsed, reachable, or operational. It
adds no XML validation or representation, document composition, clock or host
inspection, identity generation, address or port semantics, networking,
protocol, wire, discovery, runtime activation, adapter, device, feature, or
Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001o.ps1
```

## LSLC-001N Bounded Description XML Composition

LSLC-001N adds only a dependency-free consuming merge of one accepted
`StreamInfoStaticXml` and one separately accepted LSLC-001F projection whose
root is exactly the container `desc`. The description root becomes the seventh
direct `info` child, immediately after `nominal_srate`; every later description
parent index receives the fixed seven-node offset. One checked total precedes
the target node limit and one exact fallible merged-arena reserve.

Component values and allocations move without cloning. LSLC-001F `None`
containers and every `Some`, including `Some("")`, leaf remain distinct and in
source order. Arbitrary or leaf roots reject before allocation. Compact
serialization continues to use unchanged LSLC-001G explicit tags, so an empty
description spells `<desc></desc>` locally rather than claiming the observed
self-closing form.

This unit adds no implicit meaning for arbitrary generic metadata roots, XML
declaration, observed whitespace, volatile/runtime fields, complete document,
protocol, wire, I/O, adapter, provider, device, feature, or authority behavior.
Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001n.ps1
```

## LSLC-001M Bounded Static Stream-Info XML Composition

LSLC-001M adds only a dependency-free bounded projection from one borrowed
accepted `StreamInfoStaticFields` into one owned `XmlElementTree`. The root is
`info`; its exactly six direct leaves remain in `name`, `type`,
`channel_count`, `channel_format`, `source_id`, `nominal_srate` order. It
reuses LSLC-001L numeric spellings and LSLC-001B through E value/tree
contracts. The unchanged LSLC-001G serializer can project that tree into
compact explicit-tag text with no inserted whitespace.

Numeric-domain validation precedes the exact seven-node reserve. Every fixed
name and static value uses a separate exact fallible copy before existing XML
validation and character-data representation. Typed errors retain the failing
node and unchanged delegated error. The borrowed static fields, source
definition, original optional forms, and generic metadata remain unchanged and
reusable.

This local candidate surface is not the observed complete stream-info
document. It adds no XML declaration, observed whitespace, self-closing form,
`desc` mapping, volatile/runtime fields, endpoint bytes, parser, protocol,
wire, I/O, adapter, provider, device, feature, or authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001m.ps1
```

## LSLC-001L Bounded Static Numeric Spellings

LSLC-001L adds only a dependency-free bounded lexical projection that borrows
one accepted `StreamInfoStaticFields` and exposes owned `channel_count` and
`nominal_srate` text. Channel counts use at most 20 decimal bytes. Irregular
rates spell exactly `0.000000000000000`; regular rates are accepted only when
their `f64` bits equal the five observed values, spelling exactly
`100.0000000000000`, `59.94000000000000`, `1.000000000000000`,
`256.5000000000000`, or `1000000.250000000`. Any other regular rate returns a
typed error containing its unchanged bits.

The two exact output lengths precede separate fallible exact reserves. The
borrowed static fields and source definition remain unchanged and reusable.
This narrow policy makes no exponent, locale, shortest-round-trip, rounding,
or general floating-point compatibility claim. It adds no XML construction or
serialization, `desc` meaning, volatile/runtime fields, protocol, wire, I/O,
adapter, provider, device, dependency, feature, or authority behavior. The
rolling focused gate executes all seven accepted LSLC-001H/K cases directly in
Rust and reuses the immutable historical validators. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001l.ps1
```

## LSLC-001K Borrowed StreamInfo Static Fields

LSLC-001K adds only a borrowed, allocation-free semantic projection from one
accepted `StreamDefinition`. Its fixed descriptor-owned role order is `name`,
`type`, `channel_count`, `channel_format`, `source_id`, `nominal_srate`.
Original optional content-type and source-id forms remain separately visible;
effective access maps only absence to empty text. Original irregular or regular
nominal-rate form also remains visible; the separate effective numeric view maps
only irregular to positive `0.0` and preserves regular `f64` bits. The seven
format spellings are exactly `float32`, `double64`, `string`, `int32`, `int16`,
`int8`, and `int64`.

The source definition, all borrowed text, and its generic `MetadataTree` remain
unchanged and reusable. The metadata gains no `desc` meaning. This unit adds no
XML construction or representation, numeric formatting, volatile fields,
protocol, wire, I/O, runtime, adapter, provider, device, or authority behavior.
The rolling focused gate reuses the full immutable LSLC-001H corpus, case,
observation, provenance, and driver validators. It does not reapply LSLC-001J's
historical current-tree pin after this explicitly authorized source addition;
the accepted LSLC-001J receipt remains the evidence for that validation-only
unit.
Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001k.ps1
```

## LSLC-001J Shallow-Checkout Protected-Surface Gate

LSLC-001J is a validation-only correction for the LSLC-001H protected-source
guard. GitHub Actions runs `29276386135` and `29278122366` remain distinct
failed pre-fix integration attempts. Run `29278122366` passed all 134 Rust
tests and LSLC-001A through LSLC-001G, then failed because revision `9650de4`
was absent from the depth-1 checkout.

The focused checker now binds the exact 21-entry binary `git ls-tree` output
for `HEAD` across `crates/rusty-lsl`, both Cargo files, the feature lock, and
the project specification. Its accepted SHA-256 is
`ee776163e904ea3c6eb336dd1855d12f0def3e257634272e0c33e7b6e784d8e1`.
It separately rejects staged or unstaged protected-path drift and every
untracked protected path. Disposable local one-commit shallow clones prove the
history-independent pass and damaged manifest, worktree, index, and untracked
rejections without fetching or changing this source worktree.

This gate does not inspect protected implementation contents, rerun or change
the oracle, or prove candidate XML, protocol, wire, runtime, dependency,
platform, compatibility, publication, or authority behavior. The LSLC-001I
working-tree driver LF/CRLF provenance behavior remains unchanged. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001h.ps1
```

## LSLC-001I Portable Driver Provenance

LSLC-001I is a bounded validation-only correction for the two LSLC-001H text
driver bindings. The checker reads their current working-tree bytes, accepts
either complete LF or byte-equivalent complete CRLF materialization, converts
only CRLF pairs to LF for canonical SHA-256 comparison, and rejects mixed
LF/CRLF, lone carriage returns, and all non-line-ending source mutations.
Both recorded driver SHA-256 values remain unchanged and their explicit digest
basis is `canonical-lf-source-bytes`.

GitHub Actions run `29276386135` remains the failed pre-fix integration
attempt: its Windows checkout materialized the committed LF source as complete
CRLF. This correction changes no oracle driver, observation, capture fact,
candidate behavior, runtime surface, dependency, or authority boundary. The
focused gate includes deterministic in-memory LF/CRLF equivalence and damaged
newline/content checks and does not rerun the external oracle. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001h.ps1
```

## LSLC-001H StreamInfo XML Black-Box Observation

LSLC-001H is an append-only, bounded black-box observation through the exact
official PyPI Windows AMD64 `pylsl 1.18.2` wheel. Its external-only harness
verifies the wheel, public liblsl version, and loaded DLL digest; invokes only
documented `StreamInfo`, metadata-element, and XML-return APIs; creates no
outlet or inlet; makes no discovery or networking call; and retains wheels,
DLLs, environments, caches, native diagnostics, and raw XML outside the
repository.

The separate observation overlay binds the frozen LSLC-001A corpus by SHA-256
without changing its `not-observed` oracle or candidate roles. Public XML is
committed only after exact byte-positioned replacement of runtime/session/host/
address/port character data and a fail-closed boundary scan. Those operations
preserve observed core order, whitespace, tag form, numeric and format
spelling, caller character data, and `desc` placement. This adds no candidate
mapping or serialization, parser, protocol, wire, runtime, adapter, provider,
device, or Manifold authority behavior. Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001h.ps1
```

## LSLC-001G Bounded Element-Tree Serialization

LSLC-001G adds only a dependency-free borrowed, explicitly byte-bounded,
fallible, non-recursive projection from one accepted `XmlElementTree` to one
owned UTF-8 `String`. Fixed local policy emits explicit start and end tags,
inserts no whitespace, visits children depth-first with direct siblings in
ascending arena index, and emits accepted `XmlCharacterData` verbatim.

Exact checked output length and limit rejection precede one exact fallible
traversal-frame-stack reserve and one exact fallible `String` reserve. The
frames index direct-child and next-sibling links once before linear traversal.
Errors retain
the failing node or exact expected, required, and requested counts. Accepted
state owns exactly the limit and output string; the source remains borrowed and
unchanged. This adds no complete-document, stream-info, field-mapping,
endpoint, oracle, parser, decoder, protocol, wire, I/O, runtime, adapter,
provider, device, or authority meaning. Run the focused gate with:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001g.ps1
```

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
results remain `not-implemented`. LSLC-001A adds only an independently authored,
provenance-locked public-documentation corpus for documented stream-info
document roles and XML 1.0 character constraints. Every LSLC-001A oracle
observation and candidate result remains `not-observed` with null evidence;
exact serialization remains unresolved for a separately approved black-box
unit. LSLC-001B adds only dependency-free bounded XML 1.0 Fifth Edition legal
text and element-name value contracts. It preserves caller strings unchanged,
including representation-sensitive delimiters, and adds no escaping, parsing,
serialization, document, LSL field-mapping, protocol, wire, or runtime behavior.
LSLC-001C adds only a dependency-free bounded character-data representation
over borrowed accepted `XmlText`. Its fixed local candidate policy emits `&`,
`<`, and `>` as `&amp;`, `&lt;`, and `&gt;`, respectively, while preserving every
other legal scalar unchanged. This policy is not observed liblsl behavior and
adds no element, attribute, document, parser, LSL mapping, protocol, wire, or
runtime behavior.
LSLC-001D adds only an infallible dependency-free leaf-only composition that
moves one accepted `XmlElementName` and one accepted `XmlCharacterData` into
private two-component state. Borrowed and consuming access preserves both
components and their owned string allocations unchanged. It adds no tag
spelling, tree, document, raw-byte, parser, serializer, stream-info mapping,
protocol, wire, compatibility, or runtime behavior.
LSLC-001F adds only a dependency-free consuming one-way projection from one
accepted generic `MetadataTree` into one accepted `XmlElementTree`. `None`
classifies as a container and every `Some`, including `Some("")`, classifies
as a leaf under explicit caller-selected limits. This is local candidate
policy, not decoding, round-trip, document, stream-info, LSL field-mapping,
endpoint, compatibility, protocol, wire, or runtime behavior.
CORE-001 opens only dependency-free local
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
- LSLC-001A public-documentation cases keep specification, oracle observation,
  and candidate result separate. Its bounds are Rusty LSL test policy, not
  liblsl limits, and it implements no XML behavior.
- Keep official native libraries and wrappers outside the default production
  dependency closure.
- The repository is source-only. Its local constructors have no runtime,
  package, permission, network, authority, or feature-activation effect.

## Architecture Rules

- Start with one `std`-only facade crate. Split protocol, runtime, testkit,
  oracle, or C-ABI crates only when a reviewed ownership boundary requires it.
- Keep `unsafe_code = "forbid"` until a separately reviewed FFI or platform
  adapter demonstrates a need.
- LSLC-001O keeps the eleven volatile values as opaque caller-owned text under
  three explicit class bounds. The role inventory and class mapping are data
  contracts only; they do not confer provider, representation, endpoint,
  runtime, identity, transport, security, recovery, or authority meaning.
- LSLC-001Q consumes only accepted N and P trees. It validates their fixed
  shapes, shares the `info` root, retains six static and eleven volatile leaves
  before `desc`, offsets only description-internal parents by eleven, and
  delegates all final hierarchy bounds to `XmlElementTree`.
- LSLC-001Q is local element composition only. It does not own a declaration,
  observed whitespace or self-closing policy, complete-document bytes,
  provider acquisition, runtime values, transport, activation, or authority.
- LSLC-001R borrows accepted Q state and owns only the H-observed declaration,
  LF/tab layout, empty fixed `desc` spelling, and final LF. Other childless
  containers reject as unobserved rather than inheriting that spelling.
- LSLC-001R does not modify or generalize LSLC-001G. Its owned string is local
  observation-bound candidate evidence, not endpoint, wire, provider, runtime,
  transport, device, feature, or authority proof.
- LSLC-001B uses separate nonzero Unicode scalar-value maxima for XML text and
  element names. Text accepts exactly the XML 1.0 Fifth Edition `Char`
  production; names accept the complete `NameStartChar` and `NameChar`
  productions. Accepted strings and allocations remain unchanged behind
  private fields with borrowed and consuming access.
- XML text length rejects before its first indexed illegal scalar. Element-name
  rejection order is empty, length, invalid start, then first invalid
  continuation. Colon is syntax only and grants no namespace interpretation.
- LSLC-001B accepts ampersand, less-than, greater-than, and `]]>` as caller
  values. It owns no representation policy, escaping, entity selection, CDATA
  handling, parsing, serialization, byte output, document assembly, attributes,
  namespaces, schemas, queries, or canonicalization.
- LSLC-001C borrows an already validated `XmlText` without consuming,
  mutating, reinterpreting, or revalidating it. A separate nonzero maximum
  counts encoded UTF-8 bytes. Exact checked length precedes limit rejection,
  which precedes a non-panicking fallible reserve; typed errors retain
  `LengthOverflow`, exact expected/required bounds, or the requested allocation.
- Character-data accepted state is private and exposes only its limit,
  borrowed encoded text, and consuming allocation-preserving `String` access.
  Quotes and apostrophes remain literal. No generic entity engine, CDATA
  section, decoder, document assembly, or exact endpoint representation is
  implied.
- LSLC-001D accepts only the existing `XmlElementName` and `XmlCharacterData`
  types. Its infallible constructor moves them directly without cloning,
  allocation, validation, re-encoding, normalization, or interpretation.
- `XmlLeafElement` has exactly two private fields and exposes only borrowed
  `name` and `character_data` access plus allocation-preserving `into_parts`.
  Colon remains syntax only, and existing greater-than escaping remains
  LSLC-001C local candidate policy rather than observed liblsl behavior.
- LSLC-001D adds no raw-string entrypoint, limits, errors, tag spelling,
  attributes, children, mixed content, trees, roots, documents, namespaces,
  raw bytes, parsing, serialization, or LSL field mapping.
- LSLC-001E accepts one root at index zero and requires every later node to
  name a strictly earlier container parent. Leaves cannot parent children.
  Four nonzero maxima bound nodes, root-one depth, direct children per
  container, and retained UTF-8 bytes across owned container names, leaf names,
  and represented character data. Retained bytes are an arena resource bound,
  not serialized or wire size.
- Hierarchy rejection order is empty arena, node bound, root-parent shape, one
  fallible scratch reservation, then each later node in caller order for
  parent identity, parent kind, depth, and child bound, followed by checked
  retained-byte calculation and its bound. Failures are typed and non-panicking.
- Accepted `XmlElementTree` state owns only its limits and the original
  candidate-node `Vec`. Owning candidate node, value, and tree types are not
  `Clone` and expose no mutable access. The hierarchy grants no mixed-content,
  complete-document, tag-spelling, serialization, `MetadataTree`, stream-info,
  `info`, `desc`, protocol, wire, compatibility, or runtime meaning.
- LSLC-001F rejects the target node bound first, then the first child in caller
  order whose parent has a value, then fallibly reserves one exact distinct
  output arena. It projects nodes in order through XML name validation,
  optional text validation, character-data representation, and unchanged
  `XmlElementTree` delegation.
- The projection consumes the source without cloning. It preserves name
  allocations and parent/order identity, while accepted character data owns
  the separate LSLC-001C represented-string allocation. It exposes no borrowed
  or reverse projection, `From`/`TryFrom`, default limits, decoder, mutable XML
  ownership, source recovery, or round-trip claim.
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

For LSLC-001A corpus or corpus-documentation edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001a.ps1
```

For LSLC-001B XML name/text value-contract edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001b.ps1
```

For LSLC-001C XML character-data representation edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001c.ps1
```

For LSLC-001D XML leaf-element composition edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001d.ps1
```

For LSLC-001E XML container/leaf hierarchy edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001e.ps1
```

For LSLC-001F metadata-to-XML-element-tree projection edits, also run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_001f.ps1
```

The gates prove only the source-level baseline, local Rust contract semantics,
and inert dependency/activation closure. They do not prove protocol behavior,
interoperability, clock behavior, transport, or runtime support.
