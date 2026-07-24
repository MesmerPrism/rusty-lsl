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

