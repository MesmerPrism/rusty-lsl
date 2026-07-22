# Architecture

## Public requested-processing boundary

P60 projects caller-selected timestamp modes, bounded configuration, exact
health, record facts, and stage-specific refusal evidence without exporting
the private processor or transactional batch owners. The public surface is a
read-only evidence boundary over the existing rollback-safe composition.

## Complete named Float32 production pipelines

P59 carries exact caller-named discovery and canonical Float32 session
completion into the public recovery/clock/queue batch and the existing
crate-owned transactional caller-requested post-processing owner. These are
composition edges only. Discovery, selection, session, recovery, clock, queue, processing,
health, terminal close, cleanup, allocation, cancellation, and activation keep
their existing sole owners and typed evidence.

## Complete named discovery to multiformat session

P58 completes the Float32 form of the same bounded lifecycle and projects all
seven concrete formats through one caller-explicit facade. The facade is a
closed dispatch edge, not a new discovery, selection, codec, session, close,
cleanup, retry, background-work, or activation owner.

P57 composes the existing bounded UDP discovery run and exact stream-name
suggestion with the selected-response and phased session owners for Double64,
Int64, Int32, Int16, Int8, and String. The order is fixed: explicitly
activated discovery, first receive-order exact-name suggestion, strict endpoint
and format/channel/identity validation, socket-free preflight, connection,
successful-only record advancement, canonical completion, and cleanup.
Integer outcomes retain the completed discovery and selected index with their
canonical reports; Double64 and String preserve the same evidence in typed
failure variants and return their existing reports on success. No generic
strategy, hidden retry, ranking, background task, activation default, codec,
device, Makepad, or Manifold stream authority is introduced.

## Selected discovery to Float32 session batch

`typed_udp_discovery_float32_session_batch_pipeline` is a concrete composition:
caller-selected response/index -> strict endpoint -> sole phased Float32
session -> canonical actual-extent report batch -> existing recovery/clock/queue
owner -> borrowed exact health. Its outcome and error wrappers preserve the
discovery reference, selected index, and existing batch/session evidence; they
do not become lifecycle, policy, allocation, activation, device, oracle,
Makepad, or Manifold owners.

## Concrete all-format bounded chunks

P31 projects caller-owned timestamped chunks through one concrete facade per
declared format. Root and `runtime` exports delegate to the sole crate-private
chunk-to-session projection; they do not expose the format-neutral lifecycle or
sealed codecs. Each projection inherits its concrete session's shape and count
bounds: Float32 keeps its accepted bounded shapes and legacy exact two-record
adapter; Double64, Int64, Int32, Int16, and Int8 keep only 1x1 and 2x3; String
keeps only 1x1 and the exact 0..=129 UTF-8-byte envelope. Consuming reports
retain caller order and original allocations, and existing indexed failure,
cancellation, deadline, terminal close, cleanup, and immediate-reuse semantics
remain owned below the facade.

## Caller-selected discovery to Int64 session

The concrete Int64 selected-discovery facade projects one caller-selected
typed response, validates its format, channel count, and handshake identity,
then delegates to the existing socket-free Int64 preflight and sole bounded
session lifecycle. Its connect entrypoint returns the existing phased
connected inlet; its run entrypoint returns the canonical report. The facade
does not own discovery, response selection, identity derivation, retry,
recovery, codec, allocation, cleanup, activation, or Manifold stream authority.

## Native Int64 bounded session

Native Int64 is one sealed format strategy beneath the existing sole
crate-private format-neutral session lifecycle. The codec maps each Rust `i64`
to exactly eight little-endian bytes and reconstructs the identical signed
64-bit value; channel and record order are unchanged. It owns framing only.
The lifecycle remains the sole owner of socket-free shape preflight,
accept/connect, handshake and initialization sequencing, the canonical
successful-only cursor, exact completion, cancellation/deadline classification,
terminal close, and cleanup.

Admission is closed to exactly one channel with one caller record and two
channels with three caller records. Concrete Int64 accepted-outlet and
connected-inlet facades may advance records, consume exact completion, or close
without manufacturing a report, but expose neither the lifecycle engine nor
the codec. Indexed truncation and trailing bytes remain typed damage; failed
record transfer does not advance progress, and cancellation, deadline, drop,
and explicit close all release run-owned resources.

This seam does not generalize integer widths, channel or record counts, publish
a generic strategy, or add discovery, selection, retries, recovery, clocks,
queues, background work, device behavior, oracle or official compatibility,
commands, or Manifold authority. It creates no activation owner or capability;
runtime activation remains caller-explicit and default-disabled.

Batch health is a borrowed observation over the existing concrete Float32
report-batch result, not another lifecycle or evidence owner. A successful
outcome yields `complete`, with total and completed equal to the existing
completed-outcome length and remaining equal to zero. `EmptyReport` yields
`empty-report` with all three counts zero. For indexed errors, completed is the
existing completed-prefix length and the current index is the error's existing
index: `NotAcquired` maps its termination to `cancelled`, `deadline`,
`terminal`, or `exhausted`, and its remaining count includes the untouched
current record and suffix; `Recovery` maps to `recovery-error` with the same
remaining convention; `Pipeline` maps to `pipeline-error` and derives its
remaining count from the current record retained by its existing error plus
the untouched suffix. `Invariant` maps to `invariant` and likewise counts the
indexed current position plus the exact remaining evidence retained by that
existing error. For every indexed result, total equals completed plus
remaining; no loss estimate fills a gap.

The projection resolves the outer batch result variant before any nested
termination variant and derives counts only after that classification. Thus
terminal and exhausted failure evidence remains in the recovery owner's
existing variant, while clock/queue evidence remains in the existing pipeline
error. Borrowing preserves allocation identity and record order; the snapshot
owns no records, states, failures, policy, thresholds, or background work.

Caller-selected discovery resolution has one crate-private, allocation-free
contract validator. Concrete adapters enforce endpoint, then format/channel
shape and UID/hostname/source-ID/session-ID identity, then existing session
preflight, and only then TCP. The six public format facades remain concrete;
the validator, strategies, codecs, sockets, cursors, allocations, and lifecycle
owners do not leak or multiply.

The concrete String discovery adapter borrows a caller-owned completed typed
discovery run and receive-order index, strictly projects its endpoint, performs
the existing exact 1x1 socket-free String preflight, and returns the existing
connected String inlet owner. The private lifecycle and sealed String codec
remain the sole socket, cursor, allocation, damage/trailing classification,
completion, and cleanup owners; the 0..=129 UTF-8-byte envelope is unchanged.

The integer discovery-to-session seam is a thin concrete adapter family. It
borrows the caller-owned completed discovery run and selected receive-order
index, strictly projects the endpoint, performs the existing format-specific
socket-free preflight, and returns the existing Int32, Int16, or Int8 connected
inlet owner. The private format-neutral lifecycle remains the sole stream,
cursor, record-allocation, completion, terminal-close, and cleanup owner; no
generic public discovery/session strategy is introduced.

Concrete phased Int32, Int16, and Int8 sessions contain the same private
format-neutral accepted/connected lifecycle owners used by the other formats.
The outlet owner borrows the original typed caller slice; the inlet lifecycle
allocates and retains typed records directly. No facade owns a parallel cursor,
recomputes progress, or retains a second projected record collection. Sealed
crate-private typed integer strategies preserve width and framing, while exact
completion produces the established reports and close/drop produces none.
Only the accepted 1x1 and 2x3 shapes remain admitted.

The concrete one-channel/one-record String session exposes accepted and
connected phases without exposing the neutral lifecycle, strategy, codec, or
socket. Both phases delegate initialization-once, the exact cursor,
successful-only advancement, allocation retention, terminal close, and cleanup
to the sole crate-private owner. Exact completion produces the existing
canonical reports; close/drop produces none, and legacy finish traverses the
same owners. The sealed String codec and 0 through 129 UTF-8-byte envelope are
unchanged.

Bounded homogeneous Float32 accepted/connected sessions may advance one record
at a time, but do not own a parallel lifecycle. Their concrete methods delegate
to the sole crate-private format-neutral phased owner, which initializes once,
checks overrun before I/O, advances its canonical cursor only after success,
retains inlet allocations, enforces exact-count consuming completion, and owns
terminal close/drop. Existing one-shot and exact two-record chunk facades use
the same path and expose neither sockets nor format strategies.

The format-neutral session engine is the sole transfer owner. After handshake
it initializes the selected sealed strategy exactly once, retains one canonical
zero-based cursor, advances only after a successful record operation, rejects
overrun before socket access, and permits completion facts only at the declared
count. Float32 exposes the first bounded projection through the exact 1x2 chunk
accepted-outlet and connected-inlet states; no strategy or socket is public.

LSLC-007P phases the bounded Float32 outlet into preflight, accepted-stream,
and completed-report states over one crate-private format-neutral lifecycle.
The accepted state exclusively retains the guarded socket and may be consumed
by canonical finish or report-free close; the legacy finish delegates through
the same owner. No socket or generic strategy crosses the public boundary.

LSLC-007O keeps Float32 inlet preflight, connection, and completion as phased
states over one crate-private format-neutral lifecycle. A connected inlet owns
exactly one stream; consuming finish yields the existing report, while close or
drop performs terminal cleanup without manufacturing completion evidence.

LSLC-007N keeps the exact Float32 chunk facade as an adapter only. It contains
the established Float32 session owner, and its report wrappers contain and
delegate to the canonical session reports rather than restating lifecycle
facts. The private format-neutral lifecycle and sealed Float32 codec remain the
only lifecycle and framing owners.

LSLC-007M makes the shared private session lifecycle consume a validated
`SessionShape` and return crate-private completion facts. Concrete Float32,
Double64, Int32/16/8, and String facades retain their public reports and errors;
sealed strategies retain codec ownership. No public neutral engine or new
shape is introduced.

The concrete String session facade admits only the accepted one-channel,
one-caller-record shape and already validated 0 through 129 UTF-8-byte values.
Preflight occurs before socket I/O; consuming reports preserve the received
String allocation. The sealed String strategy remains subordinate to the sole
format-neutral lifecycle, and legacy entrypoints explicitly retain their
historical error projection.

Typed integer sessions expose three concrete public facade/report families.
They do not expose the generic engine or sealed width strategy. Preflight owns
only exact bounded shape and typed-value projection; the private
format-neutral engine owns accept/connect, handshake, initialization, ordered
records, terminal close, and cleanup. Rich facade errors retain caller-record
indices and trailing-byte classification, while legacy adapters explicitly
project them back to their historical errors.

`float32_session_report_recovery_clock_queue` exposes a concrete batch boundary
between a completed typed session report and the existing P4 pipeline. The
report's actual `record_count()` is the exact retained batch extent; there is no
separate universal three-record maximum. The adapter delegates each record
sequentially to the sole recovery/clock/queue owner. Successful allocations
move into the caller queue, while an indexed failure retains completed-prefix,
current-record, and untouched-suffix evidence through the concrete batch
outcome and errors. It owns no lifecycle, retry classification, automatic
policy, clock, queue, other-format, or cancellation authority. The legacy
exactly-one-record boundary remains a separate facade over the same owners.

The bounded Float32 pipeline coordinates existing owners without absorbing
them: caller acquisition feeds finite recovery, one recovered record feeds the
clock owner once, and the corrected record feeds the caller-owned bounded
queue once. It owns no discovery selection, socket/session lifecycle, codec,
policy, clock domain, queue capacity, or cancellation source.

`format_neutral_session_runtime` is the sole crate-private bounded session
lifecycle engine. It owns accept/connect, handshake sequencing,
initialization, record transfer, peer-close enforcement, terminal close, and
cleanup. Float32, Double64, integer, and String encoding and validation remain
sealed subordinate strategies; they do not own the lifecycle, cancellation,
activation, or public policy.

## Caller-selected discovery to session

`typed_udp_discovery_double64_session_connection` is the corresponding
format-specific composition for the already phased Double64 inlet. It borrows
one completed typed discovery run and accepts the caller's receive-order index,
activation, expected identity, limits, shape, and cancellation reference. The
sole strict endpoint projector runs first; the existing Double64 preflight then
admits only 1x1 or 2x3; its existing connect owner returns the concrete
`TimestampedDouble64ConnectedInletSession`. One concrete three-variant error
preserves the strict endpoint, Double64 preflight, and Double64 session errors.
The whole-session function is a thin delegate through the same connected owner
and returns its canonical report. The adapter adds no discovery,
ranking, retry, codec, cursor, lifecycle, socket, report, error, or activation
owner.

`typed_udp_discovery_float32_session_connection` is a thin adapter over three
existing owners: a caller-owned completed typed discovery run, strict IPv4
service endpoint projection, and the sole bounded Float32 inlet session. Its
order is endpoint projection, bounded session preflight, then connection and
handshake. It borrows discovery, accepts an explicit response index, and
returns the existing `TimestampedFloat32ConnectedInletSession`; phased
transfer, allocation retention, exact completion, and report-free close stay
with that concrete owner and its sole private lifecycle. The whole-session
function delegates through this seam and returns the canonical report. The
adapter performs no discovery, automatic selection, framing, socket cleanup,
retry, or fallback.

## Bounded Float32 session facade

`timestamped_float32_session_runtime` retains the public Float32 outlet/inlet
owners, bounded shape preflight, reports, and sealed format strategies. Finish
delegates the connection lifecycle to `format_neutral_session_runtime`.
`Drop` performs only nonblocking best-effort cleanup through that engine. The
older one-record and two-record functions retain their exact legacy error
mappings; they are not parallel lifecycle or codec authorities. A zero
remaining terminal deadline is classified before invoking platform socket
timeout APIs.

## Bounded Double64 session seam

Double64 is a sealed format strategy beneath the same session lifecycle, not a
second socket owner. It owns its value-size-eight handshake, two exact
initialization records, record encoding, and bit-preserving decoding. The
lifecycle retains preflight, accept/connect, terminal deadline, close, reports,
and cleanup. Admission is deliberately closed to the two observed shapes:
one channel with one caller record, or two channels with three caller records.
The legacy fixed-width Double64 functions convert at the facade boundary and
map errors explicitly.

Concrete accepted-outlet and connected-inlet Double64 states expose the
caller-owned production boundary hidden by whole-session `finish`: callers may
advance one record at a time, observe canonical progress and retained inlet
allocations, request exact consuming completion, or close without a report.
These states contain the existing private generic owners and add no cursor,
codec, socket, allocation, shape, policy, or completion authority. Legacy
`finish` delegates through `accept`/`connect` and these same concrete states.

## Bounded integer session convergence

Int32, Int16, and Int8 are sealed format strategies beneath the same session
lifecycle. Their width-specific handshake, initialization values, and record
bytes remain subordinate codec mechanics; preflight, connection, terminal
deadline, close, and cleanup retain one owner. Only one channel with one record
and two channels with three records are admitted before I/O. Existing
fixed-width entrypoints adapt at the facade boundary with their legacy typed
errors and do not expose a public generic codec or a parallel socket lifecycle.

## Production convergence

The target architecture is a single coherent public outlet/inlet session
lifecycle over reusable bounded record and chunk engines. Existing narrow
facades and accepted activation receipts remain compatibility surfaces while
the shared engine is introduced vertically; capability-specific orchestration
must not become the long-term public architecture.

The lifecycle separates descriptor/stream-info ownership, explicit activation,
discovery or caller-selected endpoints, connection ownership, bounded record or
chunk exchange, cancellation/deadline classification, and terminal cleanup.
Later slices compose clock correction, post-processing, queue/backpressure,
finite recovery, and health without merging their policy owners.

Rusty LSL owns LSL-compatible protocol/runtime behavior and typed advisory
Morphospace observations or proposals. Manifold continues to own admission,
routes, leases, authorization, revisions, and audit. No roadmap or lock grants
ambient activation.

LSLC-006E changes only tests around the existing recovery-to-clock-correction-
to-queue composition. Recovery cancellation, failure classification, recovered
record/state ownership, clock cancellation, and queue admission retain their
existing production order, owners, and bytes; no policy, activation,
compatibility, device, or Manifold authority is added.

LSLC-006D changes only tests around the existing bounded two-record Float32
chunk runtime. Record order, timestamp/value ownership, cancellation lanes,
terminal outcomes, and socket cleanup retain their existing production owners
and bytes; no chunk, activation, compatibility, device, or Manifold authority
is added.

LSLC-006C changes only tests around the existing UDP discovery owner and exact
validation-closure hashes. Response order, source address, query identity,
owned bytes, cancellation ordering, and socket lifetime retain their existing
production ownership and bytes; no discovery, selection, activation,
compatibility, device, or Manifold authority is added.

LSLC-006A changes only tests around the existing runtime-activation owner. The
accepted lock, consumer identity, canonical receipt, effective markers, and
capability projections retain their existing production ownership and bytes;
no activation, compatibility, device, or Manifold authority is added.

LSLC-005Z changes only where the LSLC-005Y external assertion is compiled. The
runtime acquisition types, ownership, allocation behavior, provider boundary,
and all production architecture remain byte-unchanged.

LSLC-005Y exercises the accepted runtime acquisition boundary solely as an
external consumer. Borrowed access observes the already accepted witness and
values; consuming access moves the same witness and four original `String`
allocations without cloning, reallocating, revalidating, or granting shared
provider, transaction, epoch, runtime, or Manifold authority.

LSLC-005X exposes the accepted transport provider output through borrowed
`witness` and `values` access plus consuming `into_parts`. These accessors move
or borrow existing owned state without cloning, reallocating, validating, or
granting provider, transaction, epoch, runtime, or Manifold authority.

LSLC-005D is the bounded combined minimum-runtime composition: finite recovery
owns only repeated caller-classified inlet attempts; integrated clock correction
runs once only after recovery; queue admission follows correction. The caller
retains every policy, activation, cancellation, provider/domain, wait bound,
raw timestamp, and rejected-record owner.

## LSLC-005C selected discovery Float32 clock-correction queue composition

LSLC-005C places the existing integrated clock-correction owner after one
successful selected Float32 inlet and before admission to an existing queue.
It retains the sample and raw source timestamp and adds only the owner's
`ClockCorrected` derived timestamp. Inlet, clock, and queue cancellation and
all activation, clock-domain, and backpressure owners remain distinct.

## LSLC-005B selected discovery Float32 recovery queue composition

LSLC-005B invokes the existing finite recovery owner around repeated calls to the
same caller-selected LSLC-004Z inlet. The caller classifies each typed inlet error,
and no endpoint reselection or rediscovery occurs. Only a recovered record reaches
the existing queue. Recovery, inlet, and queue cancellation remain distinct; queue
rejection retains the sample and recovery states. No policy or authority moves.

## LSLC-005A selected discovery Float32 queue composition

LSLC-005A adds one thin data-plane edge after LSLC-004Z. The caller passes an
already separately activated queue, explicit queue wait bounds, and cancellation
inputs distinct from inlet cancellation. The existing inlet owns endpoint and TCP
work; the existing queue owns capacity, backpressure, blocking, and rejected sample
ownership. Raw timestamp evidence moves unchanged. No worker, queue construction,
recovery, selection, compatibility evidence, or Manifold authority moves here.

## LSLC-004Z selected discovery Float32 inlet composition

LSLC-004Z is a thin minimum-runtime-spine edge. The caller-selected typed
discovery response is projected by LSLC-004X, then the existing LSLC-002T
runtime independently owns handshake, initialization, one timestamped Float32
record, deadlines, cancellation, and socket cleanup. Projection failure occurs
before TCP I/O. No selection, persistent connection, chunking, recovery,
format generalization, routing, admission, or Manifold authority moves here.

## LSLC-004Y selected discovery handshake composition

LSLC-004Y is a thin owner-preserving edge: caller-selected typed discovery
state is projected by LSLC-004X, then passed to the existing independently
activated finite inlet handshake. Projection failure precedes TCP I/O. The
composition owns no selection, identity, retry/fallback, persistent socket,
sample transport, routing, admission, or Manifold authority.

Standard host validation transfers ownership of an already bound loopback UDP
socket into the same private responder runner used by production entry points.
This closes only the test readiness window. A test-only shared mutex recovers
poisoned ownership to prevent cascading failures; runtime paths never use it.

LSLC-004X sits after caller-explicit LSLC-004W suggestion and accepted typed UDP
discovery state. It parses only the selected response's existing `v4address` and
`v4service_port` text into a canonical, concrete-unicast `SocketAddrV4` proposal.
Connection setup remains separately activated and caller-owned; no endpoint authority,
fallback, ranking, I/O, routing, admission, or Manifold authority moves here.

LSLC-004W sits strictly after typed UDP discovery admission. A nonempty caller name
may be compared exactly against retained typed response names, producing at most one
first receive-order index. The suggestion is local advisory evidence; socket ownership,
activation, endpoint choice, connection policy, and Manifold authority are unchanged.

## LSLC-003S StringSample activation descriptor

`StringSample` has its own opaque `RuntimeModuleCapability`, effective marker,
and declared `StreamHandshake` dependency. Admission rejects stale locks,
unknown or duplicate modules, mismatched markers, missing dependencies, and
absent selections. Lock revision 14 selects the descriptor while retaining
`run_activation_default: disabled`; the descriptor declares no executable
effect or permission. Capability construction stays private and all eight
prior module identities and relationships remain unchanged.

## LSLC-003P bounded record sequence

LSLC-003P composes the existing fixed-width numeric handshake and bounded
transport into three ordered timestamped records, each with exactly two
homogeneous channels. Constructors reject non-finite timestamps and the first
format mismatch before I/O. Inlet acceptance is atomic after initialization
and all three complete records arrive. Activation and authority are unchanged.

## Consumer role and plane facades

The crate root remains the compatibility facade and sole public-name source.
`contract` projects existing data and protocol contracts for data-plane
consumers; `runtime` projects existing explicit effects and activation
contracts for control/runtime consumers. Both modules contain re-exports only.
Private sibling implementation modules remain private and keep their existing
dependency direction, so the projections cannot become a second authority or
change activation and runtime behavior.

## Crate-private bounded fixed-record transport

The Float32 and fixed-width numeric sample families share one crate-private
exact-length TCP transfer helper. It owns only read/write looping, I/O-slice
timeouts, total-deadline observation, cancellation polling, peer truncation,
and socket error classification. Each runtime still owns record encoding,
format-specific initialization, activation capability consumption, its public
limits and error facade, and scope-owned socket cleanup.

LSLC-003E exposed the extraction seam but could not close changed feature
provenance inside its claimed paths, so it remains blocked. LSLC-003F retains
that history and refreshes the two feature descriptors, resolver-owned lock,
and LSLC-003C exact fingerprint binding as separate artifacts. The helper is
not public API and adds no transport selection or Manifold authority.

## Dependency-closed runtime facade composition

Runtime activation direction now follows the resolved lock: handshake feeds
both sample families; Float32 sample activation feeds queue and integrated
clock; queue feeds finite recovery. Discovery client and responder remain
dependency-free but still require their own nominal lock capability. The
facades consume opaque evidence and cannot construct or expand the lock.

## Lock-bound runtime activation

The resolved `feature.lock` remains composition evidence, not live runtime
state. `runtime_activation` independently admits only its exact accepted
fingerprint and revision plus caller-observed effective markers. The result
contains private-inner, module-nominal capabilities and a distinct
consumer-issued receipt that owns the bounded consumer identity and ordered
selected-module set. A missing module yields no capability; a requested module
without its declared dependency fails closed.

This closes lock admission only. Existing runtime facades do not yet consume
these capabilities, so LSLC-003C neither activates an effect nor claims full
runtime dependency composition. That reconciliation is the next architecture
slice. No adapter or Manifold authority participates.

## Official-compatible Float32 stream initialization

The outlet request parser retains exact role order, fixed values, and identity
fields; only `Endian-Performance` is structurally admitted as a finite positive
number inside the existing header bound. After handshake response, outlet and
inlet exchange exactly two evidence-pinned initialization records before one
caller sample. The inlet validates marker, timestamp bits, value bits, and
order before exposing caller data.

## Finite sample recovery

LSLC-002W synchronously invokes a caller operation under explicit attempt,
state, delay-slice, and deadline bounds. The caller classifies opaque failures
as retryable or terminal. The returned ordered trace makes every attempt and
termination observable; no endpoint or worker authority is absorbed.

## LSLC-002P bounded discovery runtime boundary

`udp_discovery` is a synchronous edge adapter owned by Rusty LSL. The caller
supplies the bind address, destination, accepted query bytes, response-envelope
limits, datagram/count limits, receive slice, total deadline, and cancellation
flag. The adapter owns one `UdpSocket` only for the call, observes its assigned
local address and response sources without claiming endpoint authority, and
moves admitted datagram allocations into the result before dropping the
socket. A selected lock remains inert until this explicit runtime input/call.

There is no interface enumeration, multicast membership, address selection,
retry loop, background worker, provider/currentness inference, or Manifold
state transition. Loopback behavior and cleanup evidence are separate from the
next official-endpoint interoperability unit.

LSLC-002E separates the observed response transport envelope from the existing
document contracts: an uninterpreted canonical decimal query-identifier prefix,
one CRLF delimiter, then an unchanged body accepted by LSLC-002A and LSLC-002B.
The official endpoint is evidence producer only. A later unit may implement
bounded local envelope parsing; correlation and networking remain separate.

## LSLC-002D corrective boundary

The wire-shape leaf now owns CRLF delimiters only. LSLC-002C's LF-only
candidate remains rejected historical evidence. No runtime or authority role
changes: the artifact is still inert bounded data, not endpoint selection,
network execution, discovery state, interoperability, or Manifold authority.

## LSLC-002C query byte-shape boundary

The short-info query module is a data-contract leaf. It owns exact local bytes,
field and payload maxima, canonical unsigned-decimal spelling, and borrowed
shape admission. The query text is nonempty printable ASCII and remains opaque;
the nonzero `u16` return port and `u64` query identifier remain uninterpreted.
It neither chooses nor opens an endpoint. Response documents and every network,
resolver, timing, provider, activation, and Manifold authority stay beyond this
boundary.

## LSLC-002B typed observation boundary

LSLC-002B is the consuming boundary from fixed LSLC-002A parsed state to the
existing static-definition and volatile-field contracts. Representation is
decoded without manufacturing acquisition evidence or authority.

## LSLC-002A observed document shape boundary

The private `stream_info_observed_document_parser` module scans one borrowed
`str` under an explicit nonzero byte maximum. It recognizes only the exact
empty-description LSLC-001R surface: fixed declaration and layout, seventeen
ordered leaves, represented character data, fixed end tags, and the final
empty `desc`/root suffix. Fixed borrowed end-tag spellings and a fixed
seventeen-range array eliminate transient and retained structural allocation.
Accepted values remain byte ranges into the unchanged source.

The scanner neither decodes represented data nor constructs semantic stream
state. It is not general XML, endpoint, protocol, wire, discovery, transport,
runtime, network, feature, device, or Manifold authority.

## LSLC-001Z local document facade

`stream_info_three_owner_observed_document` closes the accepted N/X-to-R local
dependency chain through P and Q. Its accepted state keeps the implementation,
runtime, and transport witnesses separate from the owned document; intermediate
accepted snapshot and element-tree state is not exposed as a new authority.

## LSLC-001X three-owner composition boundary

`stream_info_three_owner_snapshot` is a consuming composition edge above the
accepted T/U/V acquisition contracts and S admission. Owner evidence remains
three nominal witness objects; value allocations alone move into S's fixed
implementation, runtime, and transport lanes. The module performs no provider
call and establishes no relation among owner epochs or revisions. Acquisition,
activation, platform/network behavior, and Manifold authority remain outside.

## LSLC-001V transport-owner boundary

`stream_info_transport_provider` keeps six same-owner endpoint strings atomic
at the evidence boundary: one call, one shared identity/epoch/revision witness,
and six opaque allocations. It applies only the accepted O transport bound and
can project only S's transport lane. Complete S admission, platform endpoint
acquisition, interface inspection, address/port semantics, sockets, networking,
reachability, authorization, activation, and Manifold authority remain separate.

## LSLC-001U runtime-owner boundary

`stream_info_runtime_provider` keeps four same-owner values atomic at the
provider evidence boundary: one call, one witness, four opaque allocations.
This prevents mixed provider epochs or revisions inside the runtime lane.
Complete LSLC-001S admission and every platform acquisition mechanism remain
separate.

## LSLC-001T implementation-version provider boundary

`stream_info_implementation_version_provider` is an explicitly invoked edge
adapter for one implementation-owned value. Output contains separate opaque
version data and owner-issued provider identity/epoch/revision evidence. The
adapter calls the caller-selected provider once, requires exact evidence match,
applies the LSLC-001O implementation text bound, and can move only the version
into an LSLC-001S lane value. LSLC-001S still owns complete three-lane
admission. No clock, registry, runtime lane, transport lane, ambient inspection,
socket, activation, or Manifold authority enters.

The `stream_info_volatile_snapshot` module is a one-shot candidate-to-accepted
composition layer above LSLC-001O. Its three lanes preserve implementation,
runtime, and transport ownership without implementing any provider or claiming
freshness; actual acquisition remains a later owner-adapter concern.

## LSLC-001R observation-bound document representation

`stream_info_observed_document` is a specialized borrowed representation layer
above accepted LSLC-001Q. It owns only the observed declaration, LF/tab layout,
empty-desc spelling, and final LF. A bounded iterative frame table supplies
depth-first traversal; checked exact length precedes limit rejection and exact
output allocation. Childless non-desc containers reject as outside the
observed structural domain.

LSLC-001G remains the generic compact explicit-tag serializer. Neither layer
owns parsing, endpoints, providers, transport, runtime, or authority.

## LSLC-001Q ordered element composition layer

`stream_info_ordered_xml` is a consuming structural layer above the accepted
LSLC-001N and LSLC-001P element trees. It validates their fixed `info` shapes,
shares the static component root, inserts the eleven volatile leaves after the
six static leaves, and moves `desc` plus its descendants after them. Only
description-internal parents are offset by eleven before final delegation to
`XmlElementTree`; values are neither cloned nor re-represented.

This layer owns no complete-document spelling and no observation, semantic
data acquisition, provider, protocol, transport, runtime, or authority role.

## LSLC-001P volatile XML representation layer

`stream_info_volatile_xml` is a one-way borrowed projection from accepted
LSLC-001O data into an owned `info` element tree. It applies explicit existing
name, text, represented-byte, and tree limits, checks the twelve-node target
bound before allocation, and retains fixed node-indexed delegated errors.
Provider acquisition and complete-document representation remain separate.

## LSLC-001O volatile accepted-data layer

`stream_info_volatile_fields` is a dependency-free data layer below any XML
representation or provider. It retains eleven opaque caller-owned strings in
the LSLC-001H observed order. `StreamInfoVolatileFieldRole` supplies the fixed
inventory, while `StreamInfoVolatileFieldClass` separates implementation-
assigned version data, runtime-assigned creation/identity/session/host data,
and transport-owned address/port data.

`StreamInfoVolatileFieldLimits` validates three nonzero Unicode scalar maxima
in class order. `StreamInfoVolatileFields` then validates the complete input in
role order and owns only those limits plus the unchanged input. It performs no
allocation, clone, normalization, parsing, inference, provider acquisition,
clock or host read, identity generation, address or port interpretation, XML
operation, transport, runtime activation, or authority action.

## LSLC-001N description composition

`stream_info_description_xml` is a consuming arena merge above LSLC-001M and a
separate LSLC-001F projection. Admission requires an already accepted
container root named exactly `desc`; this keeps generic metadata semantics with
the caller and prevents ambient reinterpretation. The merged arena preserves
values and allocations and delegates final structural bounds to
`XmlElementTree`. No runtime or complete-document owner is introduced.

## LSLC-001M static XML composition

`stream_info_static_xml` is a leaf composition layer above the accepted static
semantic and lexical projections and below any description, volatile-field, or
complete-document policy. `StreamInfoStaticXml::compose` first closes the
LSLC-001L numeric domain, then reserves exactly seven nodes and delegates fixed
names, copied logical text, represented character data, and hierarchy checks
to the accepted XML contracts. Accepted state owns only its explicit limits
and `XmlElementTree`; it retains no runtime or authority handle.

## LSLC-001H black-box observation boundary

The oracle is a repository tool, never a production dependency or provider.
Its PowerShell layer owns the explicit external root, exact wheel acquisition,
hash and architecture checks, dependency-free isolated installation, bounded
process capture, and append-only failure history. Its Python layer imports the
pinned distribution only at capture time, calls documented public
`StreamInfo` and metadata-element APIs, takes two bounded `as_xml()`
snapshots of each unchanged object, and verifies exact repeat identity.

Raw XML, stderr, wheel, DLL, environment, and cache files remain external.
Only public-safe XML enters the append-only overlay. Ten runtime or
machine-specific text ranges are replaced by byte position; markup, whitespace,
core character data, numeric/format spelling, and description structure are
otherwise unchanged. This observation plane neither feeds the LSLC-001G local
serializer nor opens a StreamDefinition mapping, endpoint, runtime, provider,
adapter, discovery, networking, inlet, outlet, or Manifold authority plane.

## LSLC-001G element-tree serialization

The private `xml_element_serialization` module borrows one accepted
`XmlElementTree` and derives one owned UTF-8 `String` under a separate explicit
nonzero output-byte maximum. A checked length pass accounts for two copies of
each accepted name, five tag-punctuation bytes per node, and leaf character
data. Limit rejection occurs before allocation, followed by one exact fallible
reserve for a node-count-bounded traversal-frame stack and one exact fallible reserve
for the output string.

One forward pass over the accepted parent indexes records each direct-child and
next-sibling link in that stack. The serializer then performs depth-first linear
traversal without recursion, emitting siblings by ascending original arena
index. Start and end tags are always explicit, no whitespace is inserted, and
accepted character data is copied verbatim. The module neither consumes nor
mutates the source and owns no parsing, decoding, document, stream-info, LSL
mapping, endpoint, protocol, wire, I/O, runtime, adapter, provider, or authority
behavior.

## Current slice

The repository contains one `std`-only facade crate. Its public surface reports
`BoundedLocalContracts`, declares the repository ownership boundary, and
implements local bounded metadata, sample shape, timestamp value, timestamped
sample, chunk, core stream-descriptor, and flat metadata-tree families.
The separate descriptor/sample binding family accepts exactly seven
homogeneous `Sample<T>` representations and binds each one to the matching
data-only descriptor format and exact descriptor channel count.
The separate timestamped descriptor/sample composition family accepts the
same seven `TimestampedSample<T>` representations, moves each apart once, and
delegates the unchanged sample to `BoundDescriptorSample::new`. Its unforgeable
accepted state owns only that compact binding plus the unchanged raw source and
optional derived timestamp evidence.
The separate timestamped descriptor/chunk composition family accepts the same
seven `TimestampedChunk<T>` representations. It rejects an empty existing chunk
before delegation, retains the original chunk limits, and moves every ordered
sample exactly once through `BoundTimestampedDescriptorSample::new`. Its
unforgeable accepted state owns only those limits and the ordered CORE-006
bindings; indexed failures retain the first rejected sample location and
unchanged delegated error.
The focused stream-definition composition moves one already validated
`StreamDescriptor` and one already validated generic `MetadataTree` directly
into an unforgeable `StreamDefinition`. Its private state contains exactly
those two owned components. Borrowed access exposes each unchanged component;
consuming access returns both without cross-component validation,
interpretation, normalization, inference, cloning, or allocation by the
composition layer.
`RawSourceTimestamp` and
`DerivedTimestamp` accept
only finite `f64` values and preserve their bits. Every `DerivedTimestamp`
stores an explicit non-exhaustive `DerivedTimestampKind`: currently
`ClockCorrected` or `Smoothed`. These are caller-supplied classifications, not
algorithm implementations. `TimestampedSample<T>` always retains its raw value
and can additionally retain a distinct optional derived timestamp.
`TimestampedChunk<T>` retains explicit `ChunkLimits` for maximum sample and
channel counts; a valid nonzero limit configuration accepts an empty bounded
collection. It does not parse or serialize XML, open sockets,
discover streams, create threads, read clocks, allocate queues, load native
libraries, or alter process or platform state. There are no Cargo features or
dependencies.

The focused `xml_value` module owns only bounded XML 1.0 Fifth Edition value
validation. `XmlTextLimit` and `XmlNameLimit` are separate explicit nonzero
Unicode scalar-value maxima. `XmlText` accepts empty input and exactly the
`Char` production. `XmlElementName` requires a `NameStartChar` followed by
zero or more `NameChar` scalars. Both accepted types privately retain the
validated limit and original `String`, expose borrowed text, and return the
same allocation through consuming access.

Text validation checks length before the first illegal scalar. Name validation
checks empty, length, start, then continuation, with scalar indexes and code
points retained in typed errors. Colon has syntax-only meaning. Ampersand,
less-than, greater-than, and `]]>` remain caller values; no representation
policy is selected. The module owns no parser, serializer, escaping, entity,
CDATA, document, byte-output, attribute, namespace, schema, query, LSL mapping,
protocol, wire, transport, runtime, or I/O API.

The focused private `xml_character_data` module composes only over borrowed
accepted `XmlText`. `XmlCharacterDataLimit` owns a nonzero encoded UTF-8 byte
maximum. `XmlCharacterData::encode` performs an exact checked-length pass,
rejects an exceeded maximum before allocation, uses `String::try_reserve_exact`,
and then writes the exact precomputed length. Its deterministic error order is
length overflow, exceeded limit with exact expected/required byte counts, then
allocation failure with the requested count.

The candidate-owned representation maps every ampersand, less-than, and
greater-than to `&amp;`, `&lt;`, and `&gt;`. All other legal input scalars, including
quotes, apostrophes, whitespace, non-ASCII scalars, and legal noncharacters,
remain unchanged. Accepted output and its limit are private; borrowed and
consuming access preserves the output allocation. This local representation is
not an element, attribute, document, parser, decoder, generic entity engine,
CDATA-section API, LSL field mapping, or endpoint serialization claim.

The focused private `xml_leaf_element` module adds only a leaf-only composition
over one accepted `XmlElementName` and one accepted `XmlCharacterData`.
`XmlLeafElement` owns exactly those two private components. Its infallible
constructor moves both directly, borrowed access returns each unchanged, and
`into_parts` returns both with their existing limits, contents, and string
allocations preserved. Validation remains owned by LSLC-001B and representation
remains owned by LSLC-001C.

This composition assigns no tag spelling, namespace meaning, tree position,
document role, metadata-tree meaning, or stream-info mapping. It adds no raw
string or byte entrypoint, allocation, error or limit family, attributes,
children, mixed content, roots, parser, serializer, protocol, wire, transport,
runtime, or compatibility behavior.

`MetadataTree` owns a parent-before-child flat arena. Unvalidated
`MetadataNodeInput` values use `Option<usize>` parent indices: exactly one root
at index zero has no parent, and each later node must name a strictly earlier
parent. Accepted nodes do not own children. One forward pass computes depths
and direct-child counts in vectors, so construction uses no recursive public
ownership and no recursive validation or traversal. Root depth is one.
`MetadataTreeLimits` requires nonzero maxima for nodes, depth, direct children,
name Unicode scalar values, and optional value Unicode scalar values. Required
names are nonempty; optional values preserve absence versus an empty string.
Accepted flat order, parent indices, text, whitespace, and optional-value form
are unchanged and available only through read-only or consuming accessors.

`StreamDescriptor` requires a nonempty stream name, a positive bounded channel
count, and explicit nonzero maxima for name, content-type, source-id Unicode
scalar counts and channel count. Content type and source correlation are
optional bounded opaque text. Accepted text is preserved without trimming,
case folding, normalization, inference, or reordering. Source correlation has
no identity, discovery, recovery, authorization, routing, permission,
admission, or Morphospace/Manifold authority effect. The descriptor exposes no
runtime-assigned version, creation time, UID, session, host, address, or port.

`NominalSampleRate` distinguishes `Irregular` from a validated finite positive
`RegularHz` value and preserves accepted regular-rate bits. It performs no
clock read, rate measurement, scheduling, enforcement, interpolation, or rate
derivation. `ChannelFormat` has exactly seven independently named data-only
variants and assigns no protocol or wire numeric discriminants; it performs no
byte sizing, encoding, decoding, or value conversion.

`DescriptorSampleInput` is public unvalidated binding input. It owns one
already validated `Sample<T>` for exactly one of `f32`, `f64`, `String`, `i32`,
`i16`, `i8`, or `i64`. `BoundDescriptorSample` cannot be publicly forged: it
stores private accepted fields containing the unchanged sample and only a
compact descriptor-shape snapshot of channel count and `ChannelFormat`.
Construction borrows the validated descriptor and does not clone it.
`DescriptorSampleLimits` requires a nonzero maximum Unicode scalar-value count
for each String channel. Validation reports format mismatch, channel-count
mismatch, or the first oversized String channel deterministically. It performs
no conversion, casting, parsing, formatting, normalization, inference, byte
sizing, encoding, decoding, endianness, wire mapping, or runtime action.

Construction validates the complete caller-provided value before returning an
accepted value. Invalid limit configurations, exceeded metadata or chunk
bounds, invalid declared channel counts, value-count mismatches, non-finite
timestamps, and inconsistent chunk shapes return typed deterministic errors
with stable fields. Accepted strings, sample values, floating-point timestamp
bits, sample/time pairing, and order are not normalized or reordered. Protocol,
runtime, testkit, oracle, and C ABI crates remain deferred until a concrete
ownership or dependency boundary justifies a split.

The `morphospace/` directory is an inert planning and composition control
surface. Its presence does not activate code, packaging, permissions, network
access, native libraries, runtime profiles, or compatibility behavior.

The accepted STRM-000 baseline adds only specification-level compatibility cases, damaged-input
expectations, an isolated black-box oracle procedure, and deterministic
validation. These feedback-plane artifacts are neither data-plane behavior nor
runtime receipts. CORE-001, CORE-002, CORE-003, CORE-004, CORE-005, CORE-006,
CORE-007, and CORE-008
record local Rust contract tests in separate overlays rather than rewriting
that historical baseline as a measurement.

## Ownership

Rusty LSL owns independently authored, backend-neutral APIs and behavior for:

- LSL-compatible metadata and bounded metadata parsing;
- discovery observations;
- typed sample frames and chunks;
- raw source timestamps and derived clock views;
- bounded buffering, cancellation, and recovery;
- provider selection, health, and explicit fallback evidence.

Rusty LSL does not decide stream admission, identity, authorization, routes,
leases, registry revisions, product policy, platform permissions, packaging,
or application defaults. Discovery produces observations, not authority.
Inbound samples are data and cannot directly apply commands. High-rate media
does not belong in the generic sample path.

Deeper Rusty Morphospace integration stops at typed observations and
proposals. Rusty Manifold remains the authority for accepted streams,
subscriptions, routes, leases, revisions, and audit. Morphospace-native sample
transport, topology, identity, permissions, platform lifecycle, Quest and
Hostess adapters, and application policy remain in their owning repositories.

## Contract invariants

CORE-001 makes metadata collection/text limits and sample channel limits part
of validated construction. CORE-002 adds validated maximum sample and channel
counts for chunks. CORE-003 adds validated maximum Unicode scalar counts for
the required name and optional opaque text plus a validated maximum descriptor
channel count. Future public contracts must likewise make bounds part of
their types or construction:
metadata size and depth, channel count, frame and chunk size, queue capacity,
timeout, retry count, and retained timestamp range. Invalid or oversized input
must return a typed error rather than trigger unbounded work.

CORE-004 makes flat metadata-tree node count, root/parent structure, depth,
direct child fanout, required names, and optional-value text bounds part of
validated construction. It does not define XML names or documents, parsing,
serialization, escaping, namespaces, attributes, entities, schemas, queries,
mutation, discovery, protocol, wire, transport, runtime, or authority behavior.

CORE-005 makes exact descriptor format, exact descriptor/sample channel count,
and per-String-channel Unicode scalar bounds part of accepted construction.
Accepted strings, integer values, and floating-point bits including signed zero
and NaN payloads remain unchanged and ordered.

CORE-006 composes, but does not replace, the CORE-002 and CORE-005 contracts.
It preserves mandatory raw source timestamp bits, optional derived absence or
presence, derived kind and bits, homogeneous values and order, and their exact
pairing. Format, then channel count, then per-String Unicode scalar validation
remains owned by CORE-005 and returns its unchanged typed errors. CORE-006 does
not clone values, read clocks, derive or rewrite timestamps, sort, schedule,
buffer, convert, encode, transport, or perform runtime work.

CORE-007 composes, but does not replace, the CORE-002, CORE-005, and CORE-006
contracts. It adds only local non-empty acceptance, original `ChunkLimits`
retention, caller-order iteration, and zero-based first-failure indexing.
Format, channel count, String bounds, values, raw and derived timestamp
evidence, and their pairings remain owned by the delegated contracts. CORE-007
does not clone values, read clocks, calculate timestamps, sort, rewrite, split,
merge, rechunk, buffer, queue, schedule, convert, encode, transport, or perform
runtime work. Its empty rejection is not LSL compatibility evidence.

CORE-008 composes, but does not replace or reinterpret, the CORE-003 and
CORE-004 contracts. `StreamDefinition` retains the complete validated
descriptor and complete validated tree rather than copying snapshots or
creating parallel limits. Its infallible constructor moves both components
directly and adds no error family or validation order. The generic
metadata-tree root is not an LSL `desc` element. CORE-008 adds no XML document
shape, channel metadata convention, runtime identity, version, creation time,
UID, session, host, address, port, fingerprint, recovery, discovery, clock,
buffer, provider, adapter, authority, protocol, wire, or runtime behavior.

LSLC-001B implements only XML legal-text and element-name value invariants.
Text and name maxima count Unicode scalar values rather than bytes or grapheme
clusters. Accepted allocation and content are unchanged. The local contract
does not interpret the generic metadata tree, create XML nodes or documents,
or choose how accepted caller values are represented.

LSLC-001C composes over, but does not replace, the LSLC-001B `XmlText`
contract. It neither revalidates nor mutates the source and owns only the fixed
local three-character replacement policy plus exact bounded output allocation.
Its global greater-than replacement is candidate policy, not oracle evidence.

CORE-002 implements finite raw source timestamp retention and a separately
typed optional derived timestamp value classified as `ClockCorrected` or
`Smoothed`. A derived value cannot replace, hide, or mutate the raw value. The
kind and value are both caller-provided: the crate does not read clocks or
calculate correction, dejittering, smoothing, interpolation, or
sample-rate-derived timestamps. Provider fallback must name the selected
candidate and retain the rejected candidate's failure.

Only the metadata, sample-shape, timestamp-value, bounded-chunk, core
stream-descriptor, flat metadata-tree, descriptor/sample binding, and
timestamped descriptor/sample, non-empty descriptor/chunk, and
stream-definition composition
construction invariants are
implemented. The remaining
invariants constrain future design; none is an LSL runtime claim.

The private `xml_element_tree` module is a bounded parent-before-child arena
over accepted component values. Accepted state contains exactly its limits and
the original caller node `Vec`; it allocates no replacement arena. One private
scratch `Vec` is fallibly reserved before per-node validation and iteratively
tracks root-one depth and child counts. Checked retained bytes cover only owned
container names, leaf names, and represented character data.

This arena is not recursive public ownership, a general DOM, mixed content, a
document, serialization order, raw-byte output, `MetadataTree` conversion, or
an LSL `info`, `desc`, or stream-info projection.

## Dependency direction

LSLC-002O is an explicit pure numeric adapter from existing raw timestamp plus
finite offset values to the existing derived timestamp label. It owns no
provider, policy, history, automatic post-processing, or activation edge.


LSLC-002N consumes only already evaluated M results. Its single bounded scan
retains the input vector and records an index; acquisition, scheduling,
history, correction, and runtime layers remain absent.


LSLC-002M is an allocation-free numeric data/formula leaf. Raw finite values
are admitted separately from fallible arithmetic results. No provider,
packet, clock, scheduling, filter, mapping, or activation edge is introduced.


LSLC-002K is a data-plane proposal composition from the accepted query wire
and one caller-selected documented destination label. It has no control-plane
selection policy and no runtime/effect edge; any socket adoption remains a
separate closed activation unit.


LSLC-002J adds a closed data-only leaf beside the existing protocol candidates.
It returns static source spellings and a numeric documented port; it has no
dependency on address, socket, interface, discovery, clock, or runtime layers.
Those later layers may consume this evidence-backed candidate only through
separately reviewed contracts and activation units.


The default production closure is currently:

```text
rusty-lsl -> Rust standard library
```

Official native libraries, wrappers, oracle executables, captures, and test
endpoints must remain outside that closure. Any future dependency requires an
explicit purpose, license and provenance review, enabled-feature review, and a
validation update.

## Promotion gate

Runtime work may begin only after the compatibility behavior under test is
written independently, its fixtures have recorded provenance, oracle use is
isolated and reproducible, negative cases are named, and the resulting claim
is limited to the evidence actually collected. See `COMPATIBILITY.md`,
`PROVENANCE.md`, and `VALIDATION.md`.

## LSLC-001F metadata XML projection

The private `metadata_xml_projection` module is a one-way adapter between the
accepted CORE-004 arena and LSLC-001B through LSLC-001E values. Its sole public
entry point consumes `MetadataTree` plus four explicit accepted limit values.
It checks target node count, scans for the first child of a value-bearing
parent, reserves one distinct output arena exactly, projects names and present
values in caller order, and delegates the completed arena unchanged to
`XmlElementTree`.

Absent values become containers; present values, including empty strings,
become leaves. Name allocations move into accepted XML names, while represented
character data owns the separate LSLC-001C allocation. The module owns no
reverse conversion, decoding, defaults, mutable XML state, document or
serialization behavior, LSL mapping, protocol, wire, or runtime authority.

## LSLC-001L static numeric lexical projection

The private `stream_info_static_numeric_spellings` module retains one borrowed
`StreamInfoStaticFields` reference and two private owned strings. Construction
first selects nominal-rate text from the closed observed policy, before either
allocation, then converts the channel count through a fixed 20-byte stack
buffer. The irregular form and each of the five bit-exact accepted regular
forms select one 17-byte spelling. Any other regular bits reject before either
allocation. Each accepted output performs one exact fallible reserve; accepted
state exposes only borrowed channel-count and nominal-rate text.

The source remains unchanged. There is no generic float formatter,
exponent/locale/rounding policy, XML node or document ownership, `desc`
mapping, volatile-field surface, protocol, wire, runtime, adapter, provider,
device, feature, effect, or authority behavior.

## LSLC-001K static semantic projection

The private `stream_info_static_fields` module is an allocation-free borrowed
view over one accepted `StreamDefinition`. It owns no copied descriptor or
metadata state. Original option and nominal-rate forms remain observable beside
their explicit effective views, and channel-format spelling is a total mapping
over the existing seven variants. Borrowed extended metadata remains generic;
the adapter assigns no XML, `desc`, document, runtime, transport, or authority
role.
## LSLC-002S bounded TCP connection setup

The stream-handshake feature owns one call-scoped listener or connector. The
caller owns endpoint selection and identity input; the selected lock plus exact
runtime marker opens the effect. Header allocation, each blocking slice, the
total call, cancellation observation, and socket lifetime are finite. Accepted
request and response headers do not open sample, clock, queue, recovery, or
authority planes.
## LSLC-002T one-record data plane

An internal continuation retains the already admitted handshake socket only
inside the composed call. One fixed-size record carries a marker, little-endian
finite raw timestamp, and one `float32` value. The public handshake-only calls
still close immediately; the composed sample calls also close after exactly one
record. Sample I/O has its own finite slice/deadline and cancellation checks.
## LSLC-002U explicit clock data plane

One synchronous call owns one UDP socket and an explicit exchange count. The
caller owns bind/peer selection and the clock provider/domain. Each admitted
response echoes the outstanding identifier and `t0`; the caller supplies `t3`
after receipt. Existing M/N/O contracts retain formula, selection, and mapping
authority. No periodic worker, offset history, drift, or smoothing is opened.
## Bounded sample queue

The LSLC-002V queue is a caller-owned data-plane buffer. A nonzero capacity is
reserved before exposure. Immediate operations report full or empty; blocking
operations poll cancellation within an explicit finite slice and total
deadline. Close wakes all callers and permits buffered FIFO drain. Thread
creation and recovery policy remain caller concerns and outside this unit.
# LSLC-002Z responder ownership

The caller owns activation, bind address, finite limits, accepted document,
and cancellation. The responder owns one socket for one synchronous call,
admits canonical queries, derives the response identifier from that query, and
drops the socket on every return path. It acquires no endpoint-selection or
ambient authority.
# LSLC-003B

One closed value enum owns the four observed widths. Explicit activation,
caller identity/listener or peer, finite deadlines, cancellation, and
scope-owned sockets bound the data-plane call.

## LSLC-003T

LSLC-003T composes the module-nominal `StringSample` capability with the
existing handshake capability before finite loopback I/O. Source, enum, lock,
or descriptor presence alone has no runtime effect. The runtime owns exactly
two observed initialization records and one caller String record; it does not
generalize activation or framing authority.

## LSLC-003V

Only the private Rust test module gains boundary conformance cases. The
production prefix is checked byte-for-byte against accepted revision
`f1e68657253b572c6f1bfae58747b11637ec07ee`; module composition is unchanged.
## LSLC-003X empty String runtime

The String runtime accepts zero through 127 UTF-8 payload bytes only after the
caller supplies both the distinct `StringSample` capability and stream-
handshake activation. Empty data uses the separately observed one-byte length
form with length zero; it does not change channel/record shape, activation,
socket ownership, deadlines, cancellation, cleanup, or authority.

## LSLC-003Z exact 128-byte String runtime

The same closed composition now accepts zero through 128 UTF-8 payload bytes,
using only the LSLC-003Y-observed one-byte length form. A 129-byte value rejects
before I/O. Capability admission, channel/record shape, deadlines,
cancellation, socket cleanup, dependencies, and authority are unchanged.

## LSLC-004D multicast requester conformance

The production prefix remains byte-identical to accepted LSLC-004C. A
test-only joined-loopback peer composes with the existing explicit-bind,
explicit-destination requester for exactly one query and one response at
239.255.172.215:16571, then drops membership and releases its socket. No
responder runtime or interface-selection policy is added.

## LSLC-004E exact multicast responder

An exact-group entry point binds port 16571, rejects non-loopback interface
addresses before I/O, joins 239.255.172.215 on the caller's loopback interface,
and delegates to the unchanged LSLC-002Z bounded responder loop. Socket scope
owns membership cleanup; there is no enumerator, default selector, background
worker, or generic membership service.

## LSLC-004F exact multicast composition

A test-only owner case composes the accepted requester and responder under the
shared multicast test lock. Both production prefixes remain byte-identical to
LSLC-004E; the test adds no endpoint policy or runtime surface.

## LSLC-004B exact 129-byte String runtime

The same closed composition now accepts zero through 129 UTF-8 payload bytes,
using only the LSLC-004A-observed one-byte length form. A 130-byte value rejects
before I/O. Capability admission, channel/record shape, deadlines,
cancellation, socket cleanup, dependencies, and authority are unchanged.

## LSLC-004J explicit concrete IPv4 responder interface

The new entry point validates one caller-owned IPv4 interface value and then
uses the same exact-group bind, membership, and LSLC-002Z responder loop as the
accepted loopback composition. Validation rejects the unspecified address,
all multicast addresses, and the broadcast address before socket I/O. The
existing loopback function retains its earlier typed non-loopback rejection and
delegates only after that check. There is no interface inventory, default
selection, fallback, retry worker, or endpoint policy.

## LSLC-004U local typed UDP projection

`TypedUdpDiscoveryResponse` sits after bounded UDP admission and delegates
existing parsing/admission. It owns typed state plus the copied observed source,
not sockets, discovery policy, activation, devices, or authority.

## LSLC-004V typed UDP discovery run

`run_typed_udp_discovery` is a thin composition after the existing nominal
UDP activation gate. The UDP owner still opens, bounds, cancels, and cleans up
the socket. The new layer consumes its completed run, preserves local address
and termination, and projects each admitted response in receive order through
LSLC-004U using caller limits. It adds only fallible bounded output allocation
and a zero-based response index around unchanged typed projection failures.
# P61 requested-processing recovery/queue observability

The P61 facade binds an already completed P60 Float32 requested timestamp
processing lifecycle to caller-supplied exact observations from the existing
finite-recovery and bounded-queue owners. It stages both bounded data-only
observers and commits their immutable health together, so a projection refusal
cannot partially advance composed state. See
`p61-requested-processing-recovery-queue-observability.md`.
