## LSLC-003K Pinned Rust 1.80 Clippy Baseline Policy

LSLC-003K records the exact normalized pinned Rust 1.80 Clippy diagnostic
baseline: 319 library warnings and 350 all-target test warnings, represented by
669 coded diagnostic occurrences. The live checker verifies the exact Clippy
and rustc identity and rejects missing, changed, duplicated, or new diagnostics.
This is a debt baseline, not warning acceptance or cleanup authority. Run
`./tools/check_lslc_003k.ps1`, `python ./tools/test_lslc_003k.py`, and
`./tools/check_all.ps1`. Do not regenerate the baseline outside a separately
claimed policy/cleanup unit.

## LSLC-003J Historical/Current Gate Role Closure Recovery

LSLC-003J preserves `tools/current-gates.json` and all eighteen v1 historical
checker pairs byte-for-byte. `tools/current-gates-v2.json` binds each historical
gate to an ancestral commit, receipt, and exact hashes, replays it only in an
isolated clean worktree, and verifies cleanup. LSLC-003J is the sole live-tree
forward checker. It also carries the blocked LSLC-003I three-literal Rust 1.80
correction through the resolver-owned revision-13 descriptor, lock, activation,
fixture, and workspace closure without changing bytes or behavior. Run
`./tools/check_lslc_003j.ps1`, `python ./tools/test_dispatch_current_gates_v2.py`,
and `./tools/check_all.ps1`. Never hand-edit the feature lock or historical
checker bytes.

## LSLC-003I Pinned Rust 1.80 Const-Bit Compatibility

LSLC-003I replaces only three pre-const-stabilization floating `to_bits` calls
with their exact IEEE integer literals so pinned Rust 1.80 can compile the
unchanged Float32 initialization path. Focused evidence proves the literals,
encoded bytes, and runtime behavior are identical. Clippy policy and warning
cleanup, feature breadth, locks, dependencies, devices, authority, public API,
and historical checker changes remain outside this unit. Run
`./tools/check_lslc_003i.ps1`; the accepted current-gates dispatcher remains
unchanged and is run separately through `./tools/check_all.ps1`.

## LSLC-003H Manifest-Driven Current Gates

LSLC-003H makes `tools/current-gates.json` the explicit ordered inventory for
current accepted focused checks. `tools/dispatch_current_gates.py` validates the
complete inventory and every checker path before executing, then stops at the
first failure. `check_all.ps1` and CI route through this mechanism; each
historical `check_lslc_*` entrypoint remains directly runnable and unchanged.
Run `./tools/check_lslc_003h.ps1` for dispatcher negative tests and
`./tools/check_all.ps1` for the complete owner gate.

## LSLC-003G Public Role/Plane Facades

LSLC-003G adds only `contract` and `runtime` consumer namespaces over existing
crate-root exports. Every root name remains the compatibility facade; the new
modules define no types, effects, defaults, or authority and expose no private
implementation module. An external integration test uses only supported public
paths. Feature lock, activation, runtime behavior, protocol bytes, errors,
limits, cancellation, and cleanup remain unchanged. Run
`./tools/check_lslc_003g.ps1`.

## LSLC-003F Dependency-Closed Bounded Record Transport Correction

LSLC-003F retains the blocked LSLC-003E crate-private exact-record TCP helper
and closes its omitted descriptor, lock, and exact activation bindings. Only
the accepted Float32 and fixed-width numeric sample runtimes delegate byte
transfer; their public facades, initialization, encoding, typed errors, bounds,
activation capabilities, and cleanup remain unchanged. LSLC-003E remains
blocked history. No format, chunk, adapter, device, dependency, official-source
intake, or Manifold authority is added. Run `./tools/check_lslc_003f.ps1`.

## LSLC-003D Dependency-Closed Runtime Activation Composition

LSLC-003D makes the LSLC-003C opaque module capability mandatory at every
existing runtime activation facade. Sample activation consumes handshake
activation; queue and integrated clock consume Float32 sample activation; and
finite recovery consumes queue activation, exactly matching the resolved lock.
Runtime I/O, limits, cancellation, cleanup, and public facade names remain
unchanged. No breadth, adapter, device, dependency, or Manifold authority is
added. Run `.\tools\check_lslc_003d.ps1`.

## LSLC-003C Lock-Bound Runtime Activation Capability

LSLC-003C admits an explicit caller selection only against the complete
accepted feature-lock fingerprint and revision. It rejects stale locks,
unknown or duplicate modules, wrong effective markers, and missing declared
dependencies before returning opaque module-nominal capabilities and a
separate consumer-issued receipt. An absent selection remains inert.

Existing runtime activation constructors and effects are unchanged; consuming
these capabilities in queue, recovery, clock, or transport composition is a
later unit. This unit adds no format, chunk, adapter, device, dependency,
official-source intake, ambient activation, or Manifold authority. Run
`.\tools\check_lslc_003c.ps1`.

## LSLC-003B Fixed-Width Numeric Runtime Family

LSLC-003B adds one explicitly activated, bounded, single-channel one-record
family for `double64`, `int32`, `int16`, and `int8`, with exact observed widths
and format-specific initialization. Both pinned-official IPv4-loopback
directions preserve timestamp and value for all four formats. It adds no
`int64`, string, chunks, multicast, devices, or Manifold authority. Run
`.\tools\check_lslc_003b.ps1`.

## LSLC-003A Fixed-Width Sample Format Observation

LSLC-003A records a finite pinned-official IPv4-loopback black-box matrix for
`double64`, `int32`, `int16`, `int8`, and `int64`. The first four official-
outlet directions preserve record width, marker, initialization timestamp,
caller sample timestamp, and caller value; their initialization values are
format-specific. Reverse synthetic outlets using the Float32 initialization
pattern fail the official test-pattern check. The pinned public binding reports
`int64` outlet construction unavailable on this platform.

This is sanitized evidence, not production implementation. Raw artifacts stay
private; no string, chunk, multicast, device, or Manifold claim is added. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_003a.ps1`.

## LSLC-002Z Bounded Short-Info Discovery Responder Interoperability

LSLC-002Z adds one explicitly activated, synchronous, caller-configured IPv4
UDP responder call. Finite datagram/request/deadline/receive-slice bounds,
cancellation, exact LSLC-002D query admission, LSLC-002F response encoding,
typed failures, and scope-owned cleanup bound every call. A pinned official
public client resolved the independently authored response on loopback; only
sanitized outcomes and hashes are public.

Raw packets, XML, endpoints, diagnostics, binaries, environments, and caches
remain private. This adds no multicast membership, interface selection,
background service, non-loopback reachability, sample behavior, device work,
or Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass
-File .\tools\check_lslc_002z.ps1`.

## LSLC-002Y Official Float32 Initialization Compatibility Correction

LSLC-002Y corrects both LSLC-002X failure stages without inspecting official
source. Request admission keeps every fixed and identity role exact while
admitting the observed `Endian-Performance` role only as a finite positive
bounded `f64`. The Float32 data path exchanges and validates exactly two
observed initialization records after the handshake and before the caller
sample. Both pinned-official IPv4-loopback directions then preserve the
explicit caller timestamp and value bits.

Raw evidence remains private. This adds no other format, chunk, multicast,
non-loopback, performance, device, or Manifold claim. Run `powershell
-NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_002y.ps1`.

## LSLC-002X Official Float32 Interoperability Failure Observation

LSLC-002X observes the accepted Rust one-record Float32 runtime against the
pinned official protocol-110 endpoint in both IPv4-loopback directions. The
official-outlet direction completed connection setup but Rust returned
post-handshake initialization data as a sample with neither expected timestamp
nor value bits. The reverse direction resolved one Rust candidate and reached
its streamfeed listener, where Rust rejected the official request as a typed
identity mismatch before responding; the official inlet reported stream loss.

Only sanitized stages and pinned hashes are public. Raw packets, XML,
endpoints, diagnostics, binaries, environments, and caches remain private.
This failure evidence adds no correction, successful interoperability, other
format, device, or Manifold claim. Run `powershell -NoProfile
-ExecutionPolicy Bypass -File .\tools\check_lslc_002x.ps1`.

## LSLC-002W Finite Sample Recovery Runtime

LSLC-002W adds one explicitly activated synchronous recovery coordinator over
the accepted Float32 queue path. The caller supplies nonzero attempt/state
bounds, retry delay slices, a total deadline, cancellation, the attempt
operation, and retryable-versus-terminal classification. Ordered states remain
observable. No worker, endpoint selection, or hidden reconnect policy exists.

This unit adds no replay, persistence, other formats, devices, unsafe/FFI, or
Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002w.ps1`.

## LSLC-002V Bounded Sample Queue Backpressure and Cancellation

LSLC-002V adds one selected, explicitly constructed, caller-owned bounded FIFO
for accepted timestamped single-channel Float32 samples. Nonblocking calls
expose `Full`/`Empty`; blocking calls require finite wait-slice and total-
deadline bounds and observe explicit cancellation. Close wakes all waiters and
allows already buffered samples to drain. No worker is owned by the queue.

This unit adds no retry, reconnection, recovery, replay, other sample formats,
async runtime, device behavior, unsafe/FFI, or Manifold authority. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_lslc_002v.ps1`.

## LSLC-002U Integrated Clock Correction Runtime

LSLC-002U acquires one explicit nonzero bounded UDP timedata batch from a
caller-selected peer and caller-owned clock provider. Each four-timestamp
exchange passes unchanged through LSLC-002M evaluation, LSLC-002N first
minimum-RTT selection, and LSLC-002O finite offset application. The accepted
raw sample timestamp remains separately visible beside its `ClockCorrected`
mapping. Datagram/count/time bounds, cancellation, and scope-owned cleanup
bound the call; raw observation artifacts remain private.

It adds no implicit default count, periodic scheduling, history, drift,
smoothing, dejitter, queues, recovery, endpoint selection, devices,
unsafe/FFI, or Manifold authority. Run `powershell -NoProfile
-ExecutionPolicy Bypass -File .\tools\check_lslc_002u.ps1`.

## LSLC-002T Bounded Timestamped Float32 Sample Runtime

LSLC-002T composes the selected handshake with exactly one single-channel
`float32` record carrying one already finite raw source timestamp. The fixed
record preserves timestamp and value bits under separate finite I/O bounds,
explicit cancellation, typed malformed/truncated/non-finite rejection, and
scope-owned socket cleanup. Sanitized black-box evidence binds the independent
record shape while raw bytes and runtime fields remain private.

It adds no chunks, multiple samples/channels, other formats, clock acquisition
or correction, queues, recovery, background runtime, device behavior,
unsafe/FFI, or Manifold authority. Run `powershell -NoProfile -ExecutionPolicy
Bypass -File .\tools\check_lslc_002t.ps1`.

## LSLC-002S Bounded Stream Handshake Runtime

LSLC-002S is one explicitly selected, caller-configured synchronous TCP
connection-setup slice. A bounded inlet request and outlet response retain one
caller-owned opaque identity under finite header/field, I/O-slice, and total
deadline limits, explicit cancellation, typed malformed-peer outcomes, and
scope-owned socket cleanup. Its independently authored framing is bound to
sanitized black-box evidence; raw bytes, XML, endpoints, diagnostics,
environments, and caches remain private.

This adds no sample or timestamp transport, clock exchange/correction, queue,
retry, recovery, background runtime, endpoint selection, non-loopback claim,
device behavior, unsafe/FFI, or Manifold authority. Run `powershell -NoProfile
-ExecutionPolicy Bypass -File .\tools\check_lslc_002s.ps1`.

## LSLC-002R Official Loopback Stream Handshake Observation

LSLC-002R observes two finite official outlet-to-inlet connection setups
through documented public APIs under a private IPv4-loopback-only
configuration. Each case resolved one synthetic homogeneous stream, opened the
inlet with an explicit timeout, admitted matching full stream information,
confirmed only loopback established connections, closed explicitly, sent no
sample, and exited its bounded child process.

Only sanitized roles, typed outcomes, bounds, and pinned provenance are
public. Raw packets, XML, connection rows, endpoints, identifiers, diagnostics,
binaries, environments, caches, and failed-attempt payloads remain private.
This observation adds no Rust outlet/inlet or wire implementation, sample path,
clock behavior, buffering, recovery, non-loopback reachability, broad
interoperability, device behavior, or Manifold authority. Run
`powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002r.ps1`.

## LSLC-002Q Official Loopback Discovery Client Observation

LSLC-002Q observes the accepted LSLC-002P Rust client against the pinned
official protocol-110 responder in two finite IPv4 loopback-only cases. Both
calls sent an accepted query, received one bounded response, admitted the
unchanged response envelope/document, passed separate LSLC-002B typed
admission, terminated at the response-count bound, and exited with scope-owned
socket cleanup.

Only sanitized typed outcomes and pinned provenance are public. Raw packets,
XML, endpoint values, diagnostics, binaries, environments, and caches remain
private. This one-direction observation adds no Rust responder, multicast,
interface selection, non-loopback reachability, ecosystem-wide compatibility,
correlation/currentness, outlet/inlet/sample behavior, device behavior, or
Manifold authority. Run `powershell -NoProfile -ExecutionPolicy Bypass -File
.\tools\check_lslc_002q.ps1`.

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

## DOC-024 First-Hop Pre-Compaction Snapshot

The following is the complete paragraph-level content of `AGENTS.md` at
public base `8c6a01a80c5b41c59548b186c7063b937a94abf0`, before DOC-024
compacted the live instruction surface. It is retained here as immutable
chronology and semantic-coverage evidence; the live `AGENTS.md` remains the
current router authority.

# Rusty LSL Agent Notes

P68 source is integrated on public `main`. The crate remains version `0.0.0`,
`publish = false`, default-inert, and a candidate rather than a stable, tagged,
or registry-published release. Integration changes no activation,
official-oracle, non-loopback, device, Manifold, Makepad, version, tag,
release, or registry authority. Release review must use PowerShell 7.6+
through `pwsh` and the executing gate documented in
`docs/RELEASE_CANDIDATE.md`; static readiness remains non-executing.

The pre-compaction README chronology is preserved at
`docs/history/README-AT-P68-INTEGRATION.md`. Keep this first-hop instruction
surface focused on durable authority, non-scope, and validation rules.

P65 transactionally composes caller-supplied whole-session lifecycle facts with
one caller-authored default-inert advisory. Exact caller/native identity,
proposal provenance, P64 source/execution/budget/extent, receive-order
selection, lifecycle ordering, processing, terminal, close, cleanup, recovery,
loss, and health facts must agree before composition commits. Explicit unknown
or not-observed states remain unchanged. The result is data-only and grants no
execution, connection, close, retry, recovery, scheduling, queue, route,
admission, authorization, lease, audit, activation, or Manifold authority.

P64 publicly composes the exact bounded P63 execution observation with a
default-inert advisory proposal. The caller binds opaque source/execution
identity, budget, committed-cycle extent, and exact per-cycle facts;
identity, extent, or fact drift refuses transactionally. The proposal is
data-only and grants no Manifold schema, command, admission, route, lease,
revision, authorization, scheduling, or audit authority, and adds no
execution, retry, recovery policy, queue, clock, discovery, socket,
background work, or default activation.

P63 adds a finite caller-budgeted P62 execution batch, immutable bounded
supervision of exact same-execution report snapshots, and one transactional
composition requiring exactly one supervision series per committed cycle.
Distinct cycles are never treated as snapshots of one execution. The caller
retains report-to-cycle association, execution, recovery, queue, clock,
scheduling, cancellation, and activation authority; absent loss facts remain
explicitly unavailable, and activation remains explicit and default-disabled.

P62 wires the completed P60 requested-processing evidence through the existing
finite-recovery and bounded-queue execution owners, transactionally constructs
the stable bounded execution report, and commits only exact representable P61
observations. Queue length remains a caller-observed fact; report or P61 refusal
leaves P61 health unchanged. The composition adds no discovery, processing,
recovery, queue, clock, storage, cancellation, scheduling, device, Makepad, or
Manifold authority, and activation remains explicit and default-disabled.

P61 composes a borrowed completed P60 Float32 requested-processing lifecycle
with exact finite-recovery and bounded-queue outcome observations. The new
owner transactionally projects immutable processing, recovery, cancellation,
backpressure, and loss health without owning or repeating discovery,
connection, processing, recovery, queue admission, or loss classification.
Terminal recovery bypasses processing and queue evidence; every refusal leaves
P61 state unchanged. Activation remains explicit and default-disabled, with no
inference, retry, background work, device, Makepad, or Manifold authority.

P60 exposes caller-requested timestamp modes/configuration and safe public
Float32 transactional lifecycle evidence without leaking private processors,
batch owners, or mutable health state. Preserve explicit caller choice, exact
sequence facts, rollback-safe commits, discovery/selection evidence, and
default-disabled activation; add no inference, retry, or parallel authority.

P59 composes exact caller-named Float32 discovery and canonical session
completion into the existing recovery/clock/queue batch and transactional
caller-requested post-processing owners. Preserve discovery, selection,
stage-specific errors, retained allocations, committed health, close, cleanup,
and explicit activation unchanged. Do not add automatic selection, inference,
retry policy, background work, default activation, device, Makepad, or
Manifold authority.

P58 adds the final Float32 discovery-to-cleanup composition and the
`complete_typed_udp_discovery_lifecycle` facade for caller-explicit dispatch
across every concrete format. Preserve the facade as delegation only: concrete
format owners retain validation, codec, session, close, and cleanup authority,
and all discovery/session activation and cancellation remain caller-owned.

P57 adds concrete root/runtime qualification for complete bounded discovery,
exact caller-named receive-order selection, selected-response validation,
connection, phased transfer, canonical completion, and cleanup across
Double64, Int64, Int32, Int16, Int8, and String. Existing discovery,
selection, format-specific session, codec, allocation, cancellation, deadline,
terminal-close, and cleanup owners remain sole authorities. Selection is never
automatic or ambiguous; activation remains explicit/default-disabled, and no
retry, background work, device, Makepad, or Manifold authority is added.

The frozen selected-discovery Float32 session-batch facade adds only concrete
root/runtime exports and public qualification for one caller-selected response
and index. Strict endpoint projection precedes the sole phased Float32 session;
its canonical completed report enters the existing actual-extent recovery,
clock, and queue batch owner, after which health is borrowed exactly. Existing
types retain lifecycle, policy, allocation, activation, device, oracle, and
Manifold authority; no Makepad authority is added.

The frozen all-format bounded-chunk facade adds only concrete root/runtime
exports and public qualification for Float32, Double64, Int64, Int32, Int16,
Int8, and String. Each adapter remains restricted to shapes and record bounds
already accepted by its concrete session; String remains exact 1x1 with the
0 through 129 UTF-8-byte envelope. The sole crate-private projection and
format-neutral lifecycle retain transfer, allocation, cancellation, deadline,
terminal-close, and cleanup ownership. No generic public strategy or private
engine is exposed. Activation revision 34 remains explicit and
default-disabled, with no discovery, recovery, clock, queue, device, oracle,
automatic policy, or Manifold authority.

The frozen Int64 selected-discovery facade adds only concrete root/runtime
exports and public qualification for the caller-selected connect, run, and
typed error surface. Endpoint projection and the existing selected-response
contract precede socket-free Int64 preflight and the sole bounded session
lifecycle. Discovery execution, response selection, identity, cancellation,
deadlines, codec, allocation, cleanup, and activation ownership remain
unchanged; activation stays explicit and default-disabled, with no retry,
recovery, policy, device, oracle, automatic selection, or Manifold authority.

The frozen native Int64 bounded-session milestone adds only signed 64-bit
little-endian record behavior for the already accepted one-channel/one-record
and two-channel/three-record shapes. The sole crate-private format-neutral
lifecycle owns preflight, accept/connect, handshake, initialization, the
successful-only cursor, terminal close, and cleanup; its sealed crate-private
Int64 codec alone owns exact eight-byte signed value framing without exposing a
generic strategy. Qualification must cover exact values/bits/order, indexed and
trailing damage, cancellation, deadline, cleanup and immediate port reuse,
concrete public facades, existing-format regression, and unchanged explicit
default-disabled activation. It adds no other integer format, shape, discovery,
selection, retry, recovery, clock, queue, device, oracle, official compatibility,
automatic policy, background work, command, or Manifold authority.

The Float32 report-batch health surface is an immutable borrowed projection of
the existing successful outcome or owner-preserving batch error. It reports
exact total/completed/remaining counts, an existing zero-based current index
where present, and exactly one classification: complete, empty-report,
cancelled, deadline, terminal, exhausted, recovery-error, pipeline-error, or
invariant. Precedence is the existing outer batch result variant, then the
existing termination variant, then exact retained lengths. It neither moves
nor duplicates retained allocations or failure evidence and adds no estimated
packet loss, thresholds, monitoring, policy, background work, format, device,
oracle, activation, or Manifold authority; revision 33 remains explicit and
default-disabled.

The completed Float32 inlet-report batch facade uses the report's actual
`record_count()` as its exact retained extent and delegates records
sequentially through the sole recovery/clock/queue owner. Successful record
allocations transfer to the caller queue; concrete outcomes and errors retain
indexed completed-prefix, current-record, and untouched-suffix evidence. Three
records is only a representative fixture, not a universal maximum. The facade
adds no automatic policy, other-format, lifecycle, queue, activation, device,
oracle, or Manifold authority; activation remains explicit and
default-disabled.

The selected-discovery resolution milestone adds one crate-private,
allocation-free contract owner for concrete format, channel count, UID,
hostname, source ID, and session ID. Endpoint projection precedes that contract,
which precedes existing preflight and TCP across the six concrete format
adapters. Caller selection and every lifecycle, codec, allocation, completion,
cleanup, and activation owner remain unchanged; activation is explicit and
default-disabled.

The caller-selected typed-discovery response adapter may now enter the existing
one-channel/one-record phased String connected inlet. Strict endpoint projection
precedes socket-free String preflight and the sole existing session lifecycle;
the caller retains discovery, selection, identity, cancellation, deadlines, and
activation. The exact 0 through 129 UTF-8-byte envelope, typed damage/trailing
errors, allocation and completion owners, legacy APIs, and default-disabled
explicit activation remain unchanged.

LSLC-007Y adds only concrete caller-selected typed-discovery response/run-index
adapters into the existing phased Int32, Int16, and Int8 connected inlet
owners. Endpoint projection precedes socket-free preflight, which precedes the
sole existing session lifecycle; only the accepted 1x1 and 2x3 shapes remain
available. The caller retains discovery execution, selection, identity,
cancellation, deadlines, and activation. No generic public strategy, new
capability, automatic selection, retry, device behavior, or Manifold authority
is added, and activation remains explicit and default-disabled.

The accepted Int32, Int16, and Int8 shapes now expose concrete phased
accepted-outlet and connected-inlet owners for caller-driven transfer, exact
completion, or report-free close. The sole crate-private format-neutral
lifecycle retains the cursor, successful-only progress, allocation ownership,
terminal close, and cleanup; sealed typed strategies preserve exact integer
width, bits, endian, and order without retaining a duplicate projected record
collection. Legacy whole-session finish delegates through these owners.
Activation remains explicit and default-disabled, with no format, shape,
discovery, device, oracle, or Manifold authority widening.

The already accepted one-channel/one-record String shape now exposes concrete
accepted-outlet and connected-inlet owners for caller-driven transfer, exact
completion, or report-free close. The sole crate-private format-neutral
lifecycle retains initialization, the cursor, the sealed String codec,
allocation ownership, terminal close, and cleanup; legacy whole-session finish
delegates through these owners. The exact 0 through 129 UTF-8-byte envelope and
default-disabled explicit activation remain unchanged, with no authority or
shape widening.

LSLC-007V composes one caller-selected completed typed-discovery response into
the existing phased Double64 connected inlet for only the accepted one-channel
one-record and two-channel three-record shapes. A concrete typed error preserves
endpoint, preflight, and session failures, while the one-shot entrypoint remains
a thin delegate returning the canonical report. Discovery selection, lifecycle,
codec, socket, report, and activation ownership remain unchanged; activation is
explicit and default-disabled.

The already evidenced Double64 shapes now expose concrete phased accepted-outlet
and connected-inlet owners for caller-driven record transfer, exact completion,
or report-free close. The sole crate-private format-neutral lifecycle retains
the codec, initialization, cursor, allocation, completion, and cleanup; legacy
whole-session entrypoints delegate through these owners. Activation remains
explicit and default-disabled, with no shape, discovery, device, oracle, or
Manifold authority widening.

The caller-selected typed-discovery response adapter may now stop after the
sole bounded Float32 inlet owner connects and completes its handshake. The
returned existing concrete connected owner retains phased transfer,
completion, allocation, and report-free close; the whole-session adapter
delegates through it. The caller retains discovery, response index/identity,
cancellation, deadlines, and selection policy. Activation remains explicit
and default-disabled, with no automatic discovery, selection, retry, format,
device, oracle, or Manifold authority widening.

The bounded homogeneous Float32 session exposes phased record advancement only
through its existing concrete accepted/connected owners. The sole crate-private
format-neutral lifecycle remains the initialization, cursor, exact-count,
completion, terminal-close, and allocation owner. One-shot and exact-chunk
facades remain delegates; activation stays explicit and default-disabled, with
no generic strategy, socket, format, discovery, policy, device, or Manifold
authority widening.

LSLC-007R corrects only the LSLC-004L immutable-receipt checker to the
byte-exact LSLC-003M Git blob that has existed unchanged since its introduction.
The historical receipt remains untouched; the former expected hash identified
only a CRLF-transformed checkout. No validation result, runtime, activation,
compatibility, device, or authority claim is changed.

The sole crate-private format-neutral lifecycle owns phased bounded transfer:
one-time initialization, the canonical exact-count cursor, successful-only
record advancement, overrun rejection before extra I/O, and consuming
completion facts. The exact one-channel/two-record Float32 chunk lifecycle is
the first public proof; accepted/connected owners retain the stream and close
or drop without reports. Activation remains explicit and default-disabled.

The accepted Float32 outlet lifecycle is phased into caller-explicit preflight,
accepted stream ownership, and canonical completion beneath the sole private
format-neutral engine. The accepted owner retains the stream and supports
canonical finish or report-free close; legacy finish delegates through the
same path. Reports, errors, cleanup, and default-disabled activation remain
unchanged, with no discovery, device, oracle, or Manifold authority.

LSLC-007O phases the accepted Float32 inlet lifecycle into caller-explicit
preflight, connected, and completed-report states beneath the sole private
format-neutral session owner. The connected owner retains the one stream and
supports either canonical finish or report-free close; legacy finish delegates
through the same path. No discovery, format, shape, activation, device, or
Manifold authority is added.

LSLC-007N adds exact one-channel, two-record Float32 chunk inlet/outlet
facades as thin adapters over the existing concrete sessions and sole private
lifecycle. Their reports contain and delegate to the canonical session reports;
the inlet projection moves the retained record allocation into the accepted
chunk without copying. Legacy success and error projections remain unchanged.
Activation remains explicit/default-disabled, with no shape, format, discovery,
policy, device, compatibility, or Manifold authority widening.

LSLC-007M centralizes crate-private validated session shape, socket-free
preflight, and consuming successful-completion facts for every accepted
concrete session facade. The sole private lifecycle engine and sealed format
strategies remain the only lifecycle and codec owners. String stays exactly
one channel and one record, while numeric formats stay within already evidenced
shapes. Public facades, reports, errors, and legacy projections are unchanged;
activation remains explicit and default-disabled, with no capability widening,
device behavior, or Manifold authority.

LSLC-007L adds concrete bounded one-channel, one-record String inlet/outlet
session facades and consuming reports over the sole private format-neutral
lifecycle owner. The sealed String codec retains the exact 0 through 129
UTF-8-byte envelope, initialization, framing, typed indexed/trailing errors,
allocation ownership, and legacy projection. Activation remains explicit and
default-disabled; no broader String shape, discovery, policy, device,
compatibility, or Manifold authority is added.

LSLC-007K adds concrete typed Int32, Int16, and Int8 inlet/outlet session
facades and consuming reports for only the accepted one-channel/one-record and
two-channel/three-record shapes. The format-neutral engine remains the sole
lifecycle owner, integer framing remains sealed, and legacy functions preserve
their historical error mappings. Activation remains explicit and
default-disabled; no new capability, format, shape, device, or Manifold
authority is added.

LSLC-007J composes exactly one completed Float32 inlet-session report into the
existing recovery-to-clock-to-queue pipeline. Count rejection and every
pre-acquisition cancellation, deadline, or setup failure retain the unchanged
report; after acquisition the existing clock and queue errors retain their
samples and recovery states. It adds no session, discovery, policy, capability,
other-format, device, or Manifold authority.

The bounded session lifecycle is now a format-neutral crate-private engine.
It alone owns accept/connect, handshake sequencing, initialization and record
transfer, terminal-close enforcement, and consuming cleanup. Float32,
Double64, fixed-width integer, and String behavior remains behind sealed
subordinate strategies and unchanged public adapters. This extraction adds no
shape, compatibility, activation, discovery, recovery, device, or Manifold
authority.

LSLC-007H adds one discovery-independent bounded Float32 production pipeline
that composes caller-owned finite acquisition/recovery, clock correction, and
queue admission. The existing discovery composition is a thin adapter; each
activation, policy, provider, queue, and cancellation owner remains separate.
It adds no socket/session lifecycle, automatic policy, device, or Manifold
authority.

LSLC-007G moves the accepted one-channel one-record String path beneath the
sole bounded session lifecycle owner. The String framing strategy remains
sealed and crate-private; legacy functions preserve exact initialization,
UTF-8 and 0–129-byte behavior, typed damage, cancellation, cleanup, and port
reuse. Numeric behavior remains compatible, activation stays default-disabled,
and no broader shape, official, device, or Manifold authority is added.

LSLC-007F moves the accepted Int32, Int16, and Int8 one-record and
two-channel three-record paths beneath the sole session lifecycle owner. The
integer strategies remain sealed and crate-private; legacy fixed-width
functions preserve exact bytes, errors, cancellation, cleanup, and port reuse.
Float32 and Double64 remain compatible, activation stays default-disabled, and
no broader shape, compatibility, device, or Manifold authority is added.

LSLC-007E adds a bounded Double64 outlet/inlet session seam beneath the sole
session lifecycle owner. The sealed codec admits only the evidenced one-channel
one-record and two-channel three-record shapes; existing Double64 fixed-width
functions are thin adapters, while Float32 and integer behavior remain
compatible. Activation stays default-disabled. No discovery, clocks, queues,
recovery, official compatibility, devices, or Manifold authority is added.

LSLC-007D adds one thin caller-selected typed-discovery response to bounded
Float32 inlet-session adapter. The caller retains the completed discovery run,
response selection, identity, cancellation, deadlines, and shape. Strict
endpoint projection precedes the sole session owner's preflight and finish;
success returns that owner's report directly. It adds no discovery execution,
automatic selection, parallel report/lifecycle/codec authority, compatibility
claim, device behavior, or Manifold authority.

LSLC-007C extends the sole bounded Float32 session owner to explicit bounded
homogeneous channel and record shapes. The session owns the sealed crate-private
codec, while the earlier one- and two-record public functions remain thin
adapters. Zero remaining terminal-close time is classified as a typed deadline
before platform socket timeout APIs are called. It adds no discovery, clocks,
queues, recovery, other formats, device behavior, compatibility claim, or
Manifold authority.

LSLC-007B introduces the sole bounded Float32 outlet/inlet session owner for
exactly one or two records. It owns preflight, accept/connect, handshake,
one-time initialization, terminal close, consuming completion reports, and a
crate-private codec; the earlier one-record and two-record public functions are
thin compatibility adapters. Feature-lock revision 16 binds the exact session
source and remains default-disabled. It adds no discovery, clocks, queues,
recovery, other formats, arbitrary shapes, device behavior, broader
compatibility, or Manifold authority.

Current canonical direction is the production roadmap in
`docs/LSL-PRODUCTION-ROADMAP.md`: build one coherent independently authored
pure-Rust LSL runtime, prioritize production lifecycle and reusable engines,
and use compatibility work only where it validates that runtime. Runtime
activation remains default-disabled and receipt-bound. Morphospace integration
remains typed observation/proposal only; Manifold retains stream authority.

The detailed LSLC notes below are preserved historical routing. Do not use them
as a source-path scheduler or as a reason to prefer test-only work over the
current production roadmap. The byte-preserved chronological record is in
`docs/history/LSLC-WORK-UNIT-HISTORY.md`.

LSLC-006E adds only test-only deterministic conformance for the accepted
recovery-to-clock-correction-to-queue composition. Recovery cancellation
precedes classification, clock work, and queue admission; clock cancellation
retains the exact recovered record and recovery states before queue admission.
It changes no production behavior, recovery policy, clock provider/domain,
queue policy, activation, compatibility claim, device behavior, or Manifold
authority.

LSLC-006D adds only test-only deterministic conformance for the accepted
one-channel, two-record Float32 chunk runtime. Nontrivial exact timestamp/value
bits, ordered ownership, terminal/deadline/cancellation separation, cleanup,
and immediate port reuse change no production behavior, chunk breadth,
activation, compatibility claim, device behavior, or Manifold authority.

LSLC-006C adds only test-only deterministic conformance for the accepted UDP
discovery runtime and rebinds the exact LSLC-004U/004V validation closure.
Receive-order preservation, exact source/query identity, consuming allocation
ownership, pre-cancellation precedence, and caller port cleanup change no
production behavior, discovery policy, parsing, selection, activation,
compatibility claim, device behavior, or Manifold authority.

LSLC-006A adds only test-only deterministic conformance for the accepted
runtime activation receipt authority. Identity-exact canonical receipts,
lock/consumer binding, capability-marker agreement, and rejection without
partial authority change no production behavior, accepted lock, activation
breadth, compatibility claim, device behavior, or Manifold authority.

LSLC-005Z corrects only the LSLC-005Y Cargo target shape. Its unchanged runtime
acquisition-parts assertion now runs under the repository-permitted
`public_api` integration-test target, and the standalone target is removed. It
changes no production behavior, provider policy, serialization, transport,
activation, compatibility claim, device behavior, or Manifold authority.

LSLC-005Y adds only test-only external conformance for accepted runtime
acquisition parts. Borrowed witness/value access and consuming exact witness
plus four-value allocation preservation change no production behavior,
provider authority, acquisition policy, validation order, serialization,
transport, activation, compatibility claim, device behavior, or Manifold
authority.

LSLC-005X adds only symmetric borrowed and consuming accessors to the accepted
transport provider output. Exact witness/value allocation preservation changes
no provider authority, acquisition policy, validation order, serialization,
transport, activation, compatibility claim, device behavior, or Manifold
authority.

LSLC-005W corrects only the LSLC-005V Cargo target shape. Its unchanged
transport evidence-limit assertions now run under the repository-permitted
`public_api` integration-test target, and the standalone target is removed. It
changes no production behavior, provider policy, serialization, transport,
activation, compatibility claim, device behavior, or Manifold authority.

LSLC-005V adds only test-only external conformance for the accepted transport
provider evidence-limit contract. Nonzero bound retention, Unicode-scalar
identity counting, exact typed rejection payloads, and identity-mismatch
precedence change no production behavior, provider policy, serialization,
transport, activation, compatibility claim, device behavior, or Manifold
authority.

LSLC-005U corrects only the LSLC-005T Cargo target shape. Its unchanged
stateful-acquisition assertions now run under the repository-permitted
`public_api` integration-test target, and the standalone target is removed.
It changes no production behavior, provider policy, serialization, transport,
activation, compatibility claim, device behavior, or Manifold authority.

LSLC-005T adds only test-only sequential stateful-acquisition conformance for
the accepted transport provider. Accepted, provider-error, value-error,
recovery, and exhaustion calls remain isolated while prior accepted ownership
stays unchanged. It changes no production behavior, provider policy,
serialization, transport, activation, compatibility claim, device behavior,
or Manifold authority.

LSLC-005S adds only device conformance for the accepted one-channel,
two-record Float32 chunk runtime. An exact clean Rusty LSL revision built for
`aarch64-linux-android` completed one Rust-owned finite IPv4-loopback chunk
exchange on Quest with ordered exact timestamp/value bits, immediate TCP port
reuse, zero bounded fatals, and complete run-owned cleanup. It changes no
production behavior, chunk breadth, activation, compatibility breadth, Java
LSL behavior, network scope, or Manifold authority.

LSLC-005R adds only test-only deterministic conformance for the accepted
transport provider. One-call acquisition, mismatch precedence, typed value
ownership, original allocation preservation, fixed role order, and repeated
determinism change no production behavior, provider policy, serialization,
transport, activation, compatibility claim, device behavior, or Manifold
authority.

LSLC-005Q adds only test-only deterministic conformance for the accepted
three-owner stream-info snapshot. Caller-selected acquisition order, separate
typed provider errors, allocation preservation, delegated limits, and repeated
no-cross-owner composition change no production behavior, provider policy,
serialization, transport, activation, compatibility claim, device behavior,
or Manifold authority.

LSLC-005O rebinds only the LSLC-004V validation closure to the exact accepted
LSLC-005N typed response projection blob. The UDP blob, fixture semantics,
damaged mutations, production behavior, validation inventory, compatibility
claims, device behavior, and Manifold authority remain unchanged.

LSLC-005N adds only test-only deterministic conformance for the accepted typed
UDP discovery-response projection. Exact UTF-8 positions, envelope-error
ownership, exact-boundary repetition, and complete IPv6 source preservation
change no production behavior, parsing policy, I/O, selection, activation,
compatibility claim, device behavior, or Manifold authority.

LSLC-005M adds only test-only deterministic damaged-response and lifecycle
conformance for the accepted integrated clock-correction runtime. Malformed,
mismatched, duplicate, and foreign-peer responses, active cancellation,
deadline, deterministic selection, repeated teardown, and immediate UDP port
reuse change no production behavior, clock algorithm, provider/domain,
activation, compatibility claim, device behavior, or Manifold authority.

LSLC-005L adds only device conformance for the accepted one-channel,
one-record Float32 runtime. An exact clean Rusty LSL revision built for
`aarch64-linux-android` completed one Rust-owned finite IPv4-loopback
outlet/inlet exchange on Quest with exact timestamp/value bits, immediate TCP
port reuse, zero bounded fatals, and complete run-owned cleanup. It changes no
production behavior, activation, compatibility breadth, Java LSL behavior,
network scope, or Manifold authority.

LSLC-005K adds only test-only deterministic host soak conformance for the
accepted LSLC-005D through LSLC-005G recovery-to-clock-correction-to-queue
composition. Twelve cycles cover retry/recovery, queue pressure and
cancellation, terminal/cancel pre-correction bypass, exact record ownership,
repeated teardown, and immediate TCP/UDP port reuse. It changes no production
behavior, policy, activation, compatibility claim, device behavior, or
Manifold authority.

LSLC-005J adds only test-only damaged-path and cleanup conformance for the
accepted LSLC-003P two-channel, three-record Double64, Int32, Int16, and Int8
runtime. Per-width truncation, width-shift damage, cancellation, deadline,
repeated teardown, and immediate port reuse change no production behavior,
format or count breadth, activation, compatibility claim, device behavior, or
Manifold authority.

LSLC-005I adds only the exact one-channel, two-record Float32 chunk runtime
supported by its sanitized two-direction loopback evidence. It reuses the
accepted one-record codec and initialization owner and adds bounded damaged,
cancellation, deadline, cleanup, and port-reuse coverage. It adds no arbitrary
chunking, other formats or channel counts, retained connection, activation,
device behavior, broader compatibility, or Manifold authority.

LSLC-005H adds only a distinct public test harness proving independently
authored Rusty LSL Rust core descriptor/sample binding code can be built for
`aarch64-linux-android` and executed on Quest. Android Java owns lifecycle only;
the Rust-owned effective marker, exact source locks, bounded fatal evidence,
and target-only cleanup remain separate. It adds no production behavior, wire
compatibility, ambient activation, device breadth, or Manifold authority.

LSLC-005G adds only test-only pre-correction terminal-path conformance for the
byte-unchanged LSLC-005D composition. Terminal, exhausted,
recovery-cancelled, and recovery-deadline outcomes retain their existing
failure/state ownership and bypass clock correction and queue admission. It
changes no production behavior, policy, activation, cancellation ownership,
compatibility, device, or Manifold authority.

LSLC-005F adds only test-only damaged-path conformance for the byte-unchanged
LSLC-005D composition: clock cancellation returns the unchanged recovered
record and recovery states before queue admission, while queue cancellation
retains the corrected record and states. It changes no production behavior,
policy, ownership, compatibility, device, or Manifold authority.

LSLC-005E adds only test-only synthetic end-to-end conformance for the
byte-unchanged LSLC-005D recovery-to-correction-to-queue composition. One
caller-classified retry precedes one recovered Float32 record, exactly one
separately activated clock correction, and admission to an already activated
bounded queue. It changes no production API or behavior and adds no policy,
provider/domain ownership, merged cancellation, backpressure, compatibility,
device, or Manifold authority claim.

LSLC-005D composes only caller-classified finite recovery around the LSLC-004Z
selected-response Float32 inlet, then passes a recovered record once through
the separately activated LSLC-002U integrated clock-correction owner and into
an already activated bounded queue. Recovery policy, clock provider/domain,
raw timestamp, all four cancellation inputs, activation, backpressure, and
rejected record ownership remain separate. It adds no automatic policy,
rediscovery, broader compatibility, or Manifold authority.

LSLC-005C composes only one accepted LSLC-004Z Float32 inlet record through the
separately activated LSLC-002U integrated clock-correction owner and into an
already activated bounded queue. The caller retains the clock provider/domain,
configuration, activation, all three cancellation inputs, queue wait bounds,
and raw timestamp; correction adds only the separate derived value. It adds no
automatic correction, recovery, policy, backpressure, compatibility breadth,
or Manifold authority.

LSLC-005B composes only caller-classified finite recovery around the LSLC-004Z
selected-response Float32 inlet and queues only a recovered record in an already
activated bounded queue. Recovery activation/policy, typed failure classification,
fixed endpoint, inlet/recovery/queue cancellation, backpressure, rejected sample,
state trace, and raw timestamps retain their existing owners. It adds no rediscovery,
automatic recovery policy, background work, compatibility breadth, or authority.

LSLC-005A composes only the LSLC-004Z caller-selected one-record Float32 result
into an already separately capability-gated caller-owned bounded queue. Queue
activation, capacity, backpressure, wait bounds, cancellation, close/drain policy,
and rejected sample ownership remain with the queue; inlet cancellation and raw
timestamps remain separate and unchanged. It adds no worker, recovery, breadth,
admission, routing, or Manifold authority.

LSLC-004Z composes only one caller-selected accepted typed discovery response
through the LSLC-004X strict IPv4 service projection into the separately
capability-gated finite one-record Float32 inlet. The caller still owns
selection, identity, limits, cancellation, and activation. It adds no
discovery, fallback, retry, retained connection, chunks, other formats,
recovery, admission, or authority.

LSLC-004Y composes only one caller-selected accepted typed discovery response
through the LSLC-004X strict IPv4 service projection into the separately
capability-gated finite inlet handshake. The caller still owns selection,
identity, limits, cancellation, and both runtime inputs. It adds no discovery,
fallback, retry, retained connection, sample transport, admission, or authority.

The Standard owner gate's unicast responder tests transfer an already bound
test socket into the existing internal runner, so readiness has no released-port
window. Shared multicast test serialization recovers test-only mutex poison so
one failed assertion cannot cascade; production behavior and tests are unchanged.

The Standard aggregate Rust owner gate runs test binaries serially so unrelated
timing-sensitive loopback tests do not contend with each other. Historical workflow
vocabulary is validated only through the accepted LSLC-004Q hash-bound adoption route;
do not weaken tests or rewrite immutable historical descriptors.

LSLC-004X adds only a strict caller-explicit projection from one in-range accepted
typed UDP discovery response to its canonical concrete-unicast IPv4 service
`SocketAddrV4`. It performs no I/O, selection, connection, activation, fallback,
routing, admission, device behavior, or Manifold authority.

LSLC-004W adds only an allocation-free exact-name suggestion over an accepted typed
UDP discovery run. It returns the first receive-order index or none and grants no I/O,
endpoint, connection, admission, routing, device, runtime activation, or authority.

LSLC-003S adds a distinct capability-only `StringSample` module identity to
the closed activation set. It is selected in lock revision 14 but remains
run-disabled and requires both exact-lock admission, the stream-handshake
dependency, and an explicit caller capability request. Enum, descriptor, lock,
or source presence alone causes no String transport, I/O, framing, runtime
effect, device behavior, ambient activation, or authority.

LSLC-003Q adds only two repeated sanitized black-box observations for one
nonempty 13-byte ASCII/UTF-8 String, exactly one channel, two initialization
records, and one caller record in both finite IPv4-loopback directions. It adds
no String runtime, activation, empty/multiple/oversized values, non-loopback
claim, device behavior, dependency, copied source, or authority.

LSLC-003P adds only a selected, run-disabled, finite IPv4-loopback composition
for exactly two homogeneous channels and exactly three ordered caller records
in double64, int32, int16, or int8. It preserves the LSLC-003O-observed
initialization, channel/record order, timestamps, and values. It does not
generalize counts, add formats, activate a feature, claim non-loopback or broad
compatibility, use devices, or grant runtime or Manifold authority.

## Work-Unit History

Chronological LSLC unit notes are preserved byte-for-byte in
[LSLC Work-Unit History](docs/history/LSLC-WORK-UNIT-HISTORY.md). They are
historical evidence and focused-check routing; the durable instructions below
govern current work.

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
- LSLC-003O records only sanitized black-box evidence for two channels and
  three ordered caller records across four already observed numeric formats.
  Its private raw runs, requests, endpoints, diagnostics, environment, and
  binary stay outside the repository; the evidence adds no runtime code,
  activation, String/int64 behavior, damaged-peer policy, or broad compatibility.
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

The sole current policy authority is `tools/validation-policy.json`; CI and
local wrappers must not carry independent gate inventories. Use the facade:

```text
python ./tools/dispatch_validation.py --profile quick
python ./tools/dispatch_validation.py --profile standard
python ./tools/dispatch_validation.py --profile deep
```

`tools/check_all.ps1` remains the compatibility wrapper for `standard`.
`docs/VALIDATION.md` routes the policy, exact historical guide, inventory, and
ADR. Receipts are execution evidence, never policy authority.

Every future unit declares validation impact as `none` with a specific
justification, `implementation-only`, or `policy`. Unit-specific gates become
pinned historical evidence after acceptance; durable invariants move to stable
owner-named current gate IDs. Profile, claim, limitation, or state changes
require an explicit machine-readable semantic delta. The policy's
`does_not_prove` fields remain binding evidence limits.

LSLC-003T adds only one finite IPv4-loopback String record runtime requiring
the distinct LSLC-003S `StringSample` capability and handshake dependency. It
accepts exactly one channel, one caller record, and a nonempty value of at most
127 UTF-8 bytes using only the LSLC-003Q-observed initialization and one-byte
length form. It adds no ambient activation, broader framing, devices,
dependencies, unsafe/FFI, copied source, or Manifold authority.

LSLC-003V adds test-only synthetic loopback conformance for the two LSLC-003U
value classes. The production portion of `string_sample_runtime.rs` remains
byte-identical to the accepted LSLC-003U baseline. It adds no API, framing,
activation, bounds, dependency, device, runtime effect, or authority.

LSLC-003U adds only sanitized black-box evidence that the existing one-channel,
one-record String envelope preserves one independently authored mixed-width
9-byte UTF-8 value and one independently authored exact-127-byte value in both
finite loopback directions. Two pinned repeats per case passed. It changes no
implementation, activation, framing authority, device, dependency, or runtime.

LSLC-003W adds only sanitized black-box evidence that exactly one empty String
caller record uses the already observed one-byte length form with zero payload
bytes in both finite IPv4-loopback directions. Two pinned repeats passed. It
adds no runtime, activation, nonempty/oversized breadth, arbitrary length form,
device, dependency, copied source, command, or Manifold authority.

LSLC-003X adds only the LSLC-003W-observed empty value to the existing
capability-gated one-channel, one-record String runtime. It preserves all prior
nonempty values through 127 UTF-8 bytes, one-byte length form, finite loopback,
deadline, cancellation, cleanup, StringSample plus handshake admission, and
default-disabled activation. It adds no other breadth, dependency, device,
unsafe/FFI, copied source, command, or Manifold authority.

LSLC-003Y adds only sanitized black-box evidence that exactly one independently
authored 128-byte ASCII/UTF-8 String caller record uses length form one with
length 128 in both finite IPv4-loopback directions. Two pinned repeats passed.
It adds no runtime, activation, other value/count breadth, device, dependency,
copied source, command, or Manifold authority.

LSLC-003Z extends only the existing capability-gated one-channel, one-record
String runtime maximum from 127 to the LSLC-003Y-observed exact 128 UTF-8
bytes. Zero through 127 remain accepted and 129 rejects. It preserves the
one-byte length form, StringSample plus handshake activation, finite loopback,
deadline, cancellation, cleanup, dependencies, devices, unsafe/FFI posture,
commands, and authority.

LSLC-004I corrects only the stale runtime projection of the accepted
revision-14 feature lock. Exact-lock admission now binds the deterministic
current fingerprint while the prior fingerprint rejects. Default activation,
the nine selected modules, dependencies, explicit caller capability requests,
runtime behavior, devices, and authority remain unchanged.

LSLC-004H adds only sanitized two-repeat observation evidence for the accepted
Rust requester and a private independently authored joined responder on one
caller-explicit active private IPv4 host interface at exact
239.255.172.215:16571. It changes no production bytes, responder guard,
activation, dependency, device behavior, interface policy, routing, admission,
command, or Manifold authority. Private driver, raw attempts, interface and
endpoint details, diagnostics, environment, and machine identity stay private.

LSLC-004D adds only synthetic test conformance between the byte-unchanged
explicit-destination UDP requester and one joined-loopback peer at exactly
239.255.172.215:16571. One query and one response compose under the existing
finite deadline, cancellation, and cleanup contracts. It adds no responder
runtime, membership or interface policy, other network scope, device,
dependency, unsafe/FFI, routing, admission, or Manifold authority.

LSLC-004E adds one capability-gated responder composition for exactly
239.255.172.215:16571 on one caller-explicit IPv4 loopback interface. It reuses
the LSLC-002Z parser, envelope, request bound, deadline, cancellation, and
cleanup loop for one query/response. It adds no generic membership or interface
policy, other network scope, ambient activation, device, dependency, unsafe/FFI,
routing, admission, or Manifold authority.

LSLC-004F adds only synthetic test conformance composing the byte-unchanged
accepted requester and responder for one query and one response at exact
239.255.172.215:16571 on explicit loopback. It adds no production, API,
activation, network-scope, device, dependency, routing, admission, or authority
change.

LSLC-004G adds only sanitized, hash-bound two-Quest device conformance for the
existing exact IPv4 multicast surface. A public native Android test harness
used one caller-explicit Wi-Fi interface on each Quest 3S, join/drop/rejoin,
one peer query/response, one explicit cancellation probe, target-package fatal
scanning, and complete run-owned cleanup. It changes no Rusty LSL runtime,
API, activation, dependencies, unsafe/FFI posture, group/interface policy, or
Manifold admission, routing, identity, command, or authority.

LSLC-004C adds only sanitized pinned black-box evidence for the documented
IPv4 group `239.255.172.215:16571` on one explicit loopback interface. Two
repeats passed membership join/receive/drop/rejoin and both finite query/
response directions. Observed datagram counts are not a portable retry policy.
It adds no runtime, activation, other group/family/interface behavior, device,
dependency, command, stream admission, routing, or Manifold authority.

LSLC-004A adds only sanitized black-box evidence that exactly one independently
authored 129-byte ASCII/UTF-8 String caller record uses length form one with
length 129 in both finite IPv4-loopback directions. Two pinned repeats passed.
It adds no runtime, activation, other value/count breadth, device, dependency,
copied source, command, or authority.

LSLC-004B extends only the existing capability-gated one-channel, one-record
String runtime maximum from 128 to the LSLC-004A-observed exact 129 UTF-8
bytes. Zero through 128 remain accepted and 130 rejects. It preserves the
one-byte length form, StringSample plus handshake activation, finite loopback,
deadline, cancellation, cleanup, dependencies, devices, unsafe/FFI posture,
commands, and authority.

LSLC-004J generalizes only the exact-group capability-gated responder from one
caller-explicit loopback IPv4 interface to one caller-explicit concrete IPv4
interface. Unspecified, multicast, and broadcast interface values reject
before I/O; the existing loopback entry point and bounded responder loop remain
available. It adds no enumeration, default selection, ambient fallback, IPv6,
other group/port, retries, background work, device behavior, dependency,
routing, admission, command, or Manifold authority.

LSLC-004K adds only sanitized two-repeat observation evidence that pinned pylsl
1.18.2/liblsl 1.17/protocol 110 emitted one exact-group query per repeat on one
caller-explicit active private IPv4 host interface, accepted one independently
authored bounded response, and returned the matching source. It changes no
runtime, API, activation, dependency, device behavior, interface policy,
routing, admission, command, or Manifold authority. Private drivers, raw
records, configuration, interfaces, endpoints, diagnostics, environment, and
machine identity stay private.

LSLC-004L changes only the current validation dispatcher so the already
declared immutable-receipt newline exception requires exact working-byte
equality to the tracked `HEAD` blob instead of existence at one fixed baseline.
It preserves the stricter direct raw checker and every path, binary, UTF-8,
private-content, credential, build-artifact, trailing-whitespace, mutable-file,
and non-receipt newline rule. Accepted LSLC-003M and LSLC-004J receipt bytes and
all historical events remain unchanged.

LSLC-004N records only two serialized pre-response black-box captures of the
pinned official IPv4 short-info query at exact 239.255.172.215:16571 on one
caller-explicit active private interface. It publishes per-attempt SHA-256 and
length plus minimal public-safe grammar; reply-port/query-id values, raw bytes,
interface/endpoints, diagnostics, environment, and machine identity remain
private. Equality proves only these two repeats and adds no runtime or authority.

LSLC-004M adds only test-only structural conformance between the LSLC-004N
published official query grammar and the unchanged caller-explicit exact-group
production responder. It independently selects private-field-shaped values,
replays no observed reply port or query id, preserves production bytes,
deadline/cancellation/cleanup owners and inert activation, and adds no broader
network, device, compatibility, routing, admission, or Manifold authority.

LSLC-004O records only two serialized pinned-official resolver compositions
against the unchanged production responder on one caller-explicit active IPv4
interface at exact 239.255.172.215:16571. One query, response, matching source,
finite completion, membership cleanup, and socket cleanup passed per repeat.
The ignored harness adds no production effect; all private network and raw
evidence remains excluded, and no broader compatibility or authority follows.

LSLC-004P records only two serialized pinned-official outlet compositions
against the unchanged explicit-destination requester at exact
239.255.172.215:16571 through one bounded active IPv4 host-interface path.
It adds no requester-interface policy, production change, portable multicast
claim, device scope, retry policy, routing/admission behavior, or authority.

LSLC-004Q adds only a project-owned hash-bound workflow adoption receipt for
fourteen immutable accepted or blocked units using legacy vocabulary. It pins
the corrective owner commit, exact unit bytes and terminal evidence, maps only
legacy routing semantics, keeps `network-interface` historical-only, and does
not alter runtime, activation, current registries, devices, or authority.

LSLC-004R records only two serialized pre-parser response-datagram captures
from pinned official outlets on one caller-explicit active IPv4 interface at
exact 239.255.172.215:16571. Exact hashes and lengths plus minimum sanitized
envelope/XML structure are public; raw bytes, values, endpoints, identifiers,
interface details, and diagnostics remain private. The response bytes differed,
so only length and structure stability across these two repeats is claimed. It
changes no production, activation, device, routing, admission, or authority.

LSLC-004S adds only test-side conformance from independently authored values
matching the LSLC-004R public 19-digit/CRLF envelope, 711-byte canonical
document, and ordered role structure through the unchanged UDP requester and
document parser. It proves bounded admission/retention, correlation,
deadline/cancellation, damaged rejection, and cleanup on loopback. It replays
no private value, changes no production byte, and adds no interface, retry,
device, compatibility, routing, admission, or Manifold authority.

LSLC-004T adds only test-side composition from one independently authored
LSLC-004S-shaped response admitted by the unchanged UDP requester into the
existing parsed-envelope and typed observation/admission contracts. Query ID,
typed name/count, response source, bounds, termination, and cleanup are
preserved; noncanonical channel count delegates the existing typed error. It
adds no production, parser, admission, activation, network/device, or authority.

LSLC-004U adds only a dependency-free, non-I/O local projection borrowing one
already accepted UDP response and caller-selected envelope/admission limits.
It returns existing typed observation state plus the unchanged source and
delegates existing errors. The UDP module stays byte-identical; no policy or authority follows.

LSLC-004V composes only one explicitly activated existing bounded UDP call
with the LSLC-004U projection. It preserves local address, termination,
receive order, response source, caller limits, and existing UDP or indexed
typed failures. It adds no filtering, retry/interface policy, oracle/device
claim, ambient activation, routing, admission, command, or Manifold authority.
