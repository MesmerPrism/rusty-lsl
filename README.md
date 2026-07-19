# Rusty LSL

The bounded Float32 inlet session also exposes a caller-explicit connected
state between preflight and its canonical consuming completion report. This is
the same private lifecycle owner used by the legacy one-shot finish path.

The exact one-channel, two-record Float32 chunk path now has consuming public
session facades that preserve canonical lifecycle reports and allocation-owned
records while keeping the established compatibility entrypoints unchanged.

Validated format-neutral session shape and successful completion facts remain
crate-private beneath the existing concrete typed facades. LSLC-007M does not
widen accepted String or numeric shapes, public APIs, or runtime activation.

The concrete String outlet/inlet session facade admits only one channel and one
caller record containing 0 through 129 UTF-8 bytes. It preflights before I/O,
returns consuming completed reports, and uses the same private lifecycle and
sealed String strategy as the legacy entrypoints; activation remains explicit
and disabled by default.

Concrete Int32, Int16, and Int8 session facades cover the evidenced one-record
and two-channel/three-record shapes. They preflight before I/O and complete
through the same private lifecycle engine as the existing Float32, Double64,
and String paths; activation remains explicit and disabled by default.

The production runtime now exposes a bounded discovery-independent Float32
recovery → clock-correction → queue pipeline. Callers retain acquisition,
policy, activation, clock, queue, and distinct cancellation ownership; runtime
activation remains explicit and default-disabled.

One bounded adapter can consume an exactly-one-record completed Float32 inlet
report into that pipeline. It retains the whole report until recovery actually
acquires the record, so pre-acquisition terminal paths return caller evidence
unchanged; no automatic policy or activation is implied.

The accepted Float32, Double64, integer, and String paths now share one
format-neutral crate-private bounded session lifecycle engine. Their sealed
format strategies preserve existing framing, errors, and public adapters;
activation remains explicit and default-disabled.

Rusty LSL is being developed into a production-quality, independently authored,
pure-Rust Lab Streaming Layer implementation for Rusty Morphospace. The current
priority is one coherent native outlet/inlet lifecycle and shared bounded
record/chunk engine, followed by general stream shapes, discovery and
connection lifecycle, clocks/buffering/recovery, typed advisory Morphospace
integration, host/Quest qualification, and stable promotion.

Every runtime remains default-disabled and requires the accepted lock plus an
explicit caller runtime input. Rusty LSL emits typed observations and proposals;
Manifold retains admission, routing, lease, authorization, revision, and audit
authority. Compatibility work is scheduled when it validates the production
runtime; speculative ecosystem breadth is secondary.

The first production vertical's explicit Float32 outlet/inlet roles and
caller-bounded homogeneous channel and record shapes remain public facades
over the format-neutral lifecycle. Its codec remains sealed and subordinate.
Legacy one-record and two-record functions remain thin adapters.

The same sole lifecycle now has a sealed Double64 strategy for exactly the
evidenced one-channel/one-record and two-channel/three-record shapes. Public
Double64 sessions own consuming completion reports, and the older fixed-width
Double64 functions adapt into them. Int32, Int16, and Int8 now use sealed
strategies beneath that same lifecycle for only the accepted one-by-one and
two-by-three shapes; their existing fixed-width functions remain thin facade
adapters. This does not claim arbitrary shapes or official interoperability.

A caller may now borrow one completed typed UDP discovery run, explicitly
select a response, project its strict IPv4 service endpoint, and enter the sole
bounded Float32 inlet session. Discovery execution and selection policy remain
caller-owned; the adapter returns the existing session report directly.

See [Production Roadmap](docs/LSL-PRODUCTION-ROADMAP.md) for the completion
definition and next production slice. Detailed accepted unit history remains in
[LSLC Work-Unit History](docs/history/LSLC-WORK-UNIT-HISTORY.md); the notes below
are preserved historical routing, not the current schedule.

LSLC-006E adds test-only deterministic conformance for the accepted
recovery-to-clock-correction-to-queue composition: recovery cancellation
precedes classification, clock work, and queue admission, while clock
cancellation retains the exact recovered record and recovery states before
queue admission. It changes no production behavior, recovery policy, clock
provider/domain, queue policy, activation, compatibility claim, device
behavior, or Manifold authority.

LSLC-006D adds test-only deterministic conformance for the accepted
one-channel, two-record Float32 chunk runtime: nontrivial exact timestamp/value
bits, ordered ownership, terminal/deadline/cancellation separation, cleanup,
and immediate port reuse. It changes no production behavior, chunk breadth,
activation, compatibility claim, device behavior, or Manifold authority.

LSLC-006C adds test-only deterministic conformance for the accepted UDP
discovery runtime and truthfully rebinds the exact LSLC-004U/004V validation
closure: receive-order preservation, exact source/query identity, consuming
response allocation ownership, pre-cancellation precedence, and caller-selected
port cleanup. It changes no production behavior, discovery policy, parsing,
selection, activation, compatibility claim, device behavior, or Manifold
authority.

LSLC-006A adds test-only deterministic conformance for the accepted runtime
activation receipt authority: canonical receipt identity across caller order,
exact lock and consumer binding, capability-marker agreement, and rejection
without partial authority. It changes no production behavior, accepted lock,
activation breadth, compatibility claim, device behavior, or Manifold
authority.

LSLC-005Z corrects only the LSLC-005Y Cargo target shape by moving its
unchanged assertion into the permitted `public_api` target and removing the
standalone target. It changes no production behavior, provider policy,
compatibility breadth, device behavior, or Manifold authority.

LSLC-005Y adds test-only external conformance for accepted runtime acquisition
parts: borrowed witness/value access and consuming preservation of the exact
witness and all four original value allocations. It changes no production
behavior, provider policy, compatibility breadth, device behavior, or
Manifold authority.

LSLC-005X adds `witness`, `values`, and allocation-preserving `into_parts`
access to the accepted transport provider output. It adds no provider,
acquisition policy, runtime behavior, compatibility breadth, device behavior,
or Manifold authority.

LSLC-005W corrects only the LSLC-005V Cargo target shape by moving its
unchanged evidence-limit assertions into the permitted `public_api` target and
removing the standalone target. It changes no production behavior, provider
policy, compatibility breadth, device behavior, or Manifold authority.

LSLC-005V adds test-only external conformance for the accepted transport
provider evidence-limit contract: exact nonzero bound retention,
Unicode-scalar identity counting, typed rejection payloads, and
identity-mismatch precedence. It changes no production behavior, provider
policy, compatibility breadth, device behavior, or Manifold authority.

LSLC-005U corrects only the LSLC-005T Cargo target shape by moving its
stateful-acquisition assertions under the repository-permitted `public_api`
integration-test target and removing the standalone target. The assertions and
test-only claim boundary are unchanged; production behavior, provider policy,
transport, activation, compatibility breadth, devices, and authority remain
unchanged.

LSLC-005T adds test-only sequential stateful-acquisition conformance for the
existing transport provider. It verifies isolation across accepted, typed
provider-error, typed value-error, recovery, and exhaustion outcomes while
preserving earlier accepted values, without changing production behavior,
provider policy, transport, activation, compatibility breadth, devices, or
authority.

LSLC-005S provides bounded Rust-on-Quest conformance for the accepted
one-channel, two-record Float32 chunk runtime. The exact clean Rusty LSL source
executes the ordered exchange inside Rust over `127.0.0.1`, preserves both
timestamp/value bit pairs, and releases its TCP port immediately. Android Java
owns lifecycle only. This adds no arbitrary chunking, production behavior,
activation, official/non-loopback compatibility, device breadth, or authority.

LSLC-005R adds test-only deterministic conformance for the existing transport
provider. It verifies one-call acquisition, mismatch precedence, typed value
ownership, original allocation preservation, fixed role order, and repeated
determinism without changing production behavior, provider policy, transport,
activation, compatibility breadth, devices, or authority.

LSLC-005Q adds test-only deterministic conformance for the existing
three-owner stream-info snapshot. It verifies caller-selected acquisition
order, separate provider-error ownership, allocation preservation, delegated
limits, and repeated no-cross-owner composition without changing production
behavior, provider policy, transport, activation, compatibility breadth,
devices, or authority.

LSLC-005O updates only the exact LSLC-004V validation hash for the accepted
LSLC-005N typed response projection. It changes no production behavior, gate
inventory, fixture meaning, compatibility breadth, or activation.

LSLC-005N adds test-only deterministic typed UDP discovery-response projection
conformance for exact UTF-8 positions, envelope-error ownership, repeated
exact-boundary acceptance, and complete IPv6 source preservation. Production
behavior, parsing, I/O, selection, activation, and compatibility breadth are
unchanged.

LSLC-005M adds test-only deterministic damaged-response and lifecycle
conformance for the existing integrated clock-correction owner; it changes no
production behavior, policy, activation, or compatibility claim.

LSLC-005L provides a second bounded Rust-on-Quest proof: the exact accepted
one-channel, one-record Float32 handshake/sample runtime executes inside Rust
on Quest over `127.0.0.1`, preserving exact timestamp/value bits and releasing
its TCP port for immediate reuse. Android Java owns lifecycle only. This does
not widen production behavior, activation, non-loopback or official
compatibility, supported shapes, device breadth, or authority.

LSLC-005K strengthens only test coverage for the existing finite
recovery-to-clock-correction-to-bounded-queue composition. Its deterministic
twelve-cycle host soak exercises retry, queue pressure and cancellation,
terminal/cancel bypass, exact timestamp/value ownership, repeated teardown,
and immediate TCP/UDP port reuse without changing production behavior,
activation, compatibility breadth, devices, or authority.

LSLC-005J strengthens only test coverage for the existing exact two-channel,
three-record Double64, Int32, Int16, and Int8 loopback runtime. It exercises
per-width truncation, width-shift damage, cancellation versus deadline,
repeated teardown, and immediate port reuse without changing production bytes,
runtime breadth, activation, device behavior, or authority.

LSLC-005I adds one bounded, capability-gated IPv4-loopback runtime for exactly
one channel and two Float32 caller records. It reuses the accepted one-record
codec and initialization owner and preserves typed marker, truncation,
extra-record, deadline, cancellation, cleanup, and port-reuse behavior. The
sanitized two-direction evidence does not generalize record counts, formats,
channels, network scope, devices, activation, or authority.

The LSLC-005H device harness provides the first bounded Rust-on-Quest proof:
an exact clean Rusty LSL revision is compiled for `aarch64-linux-android`, and
its core one-channel Float32 descriptor/sample binding executes inside a
distinct public Quest test package. This is separate from Android Java and
host-Rust evidence and does not claim wire or ecosystem compatibility.

LSLC-005G adds test-only conformance for the byte-unchanged LSLC-005D
pre-correction terminal paths. Terminal, exhausted, recovery-cancelled, and
recovery-deadline outcomes bypass both clock correction and queue admission
while retaining their existing failures and states. It changes no production
behavior, policy, ownership, compatibility, device, or Manifold authority.

LSLC-005F adds test-only damaged-path conformance for unchanged LSLC-005D:
clock cancellation preserves the recovered record and recovery states before
queue admission, and queue cancellation retains the corrected record and
states. It changes no production behavior, policy, ownership, compatibility,
device, or Manifold authority.

LSLC-005E adds test-only synthetic conformance for the unchanged LSLC-005D
minimum runtime spine: one caller-classified retry, one recovered Float32
record, one separately activated clock correction, and one bounded-queue
admission. It changes no production behavior or API and does not widen policy,
provider/domain ownership, cancellation, backpressure, compatibility, device,
or Manifold authority claims.

LSLC-005D composes caller-classified finite inlet recovery with the separately
activated integrated clock-correction owner and an already activated bounded
queue. Correction runs only for a recovered record; recovery policy, clock
provider/domain, raw timestamp, four cancellation lanes, activation, and
backpressure remain separate. It adds no automatic policy, rediscovery,
broader compatibility, or Manifold authority.

LSLC-005C composes one accepted selected-response Float32 inlet record through
the separately activated integrated clock-correction owner and into an already
activated bounded queue. Clock provider/domain, raw timestamp, cancellation,
backpressure, and activation ownership remain separate. It adds no automatic
correction, recovery, broader compatibility, or Manifold authority.

LSLC-005B composes caller-classified finite recovery around the accepted
selected-response Float32 inlet, then queues only a recovered record in an already
activated bounded queue. Recovery policy, failure classification, three cancellation
lanes, queue backpressure, and raw timestamps remain separately owned. It adds no
rediscovery, endpoint reselection, automatic recovery policy, or Manifold authority.

LSLC-005A composes the accepted caller-selected one-record Float32 inlet result
into an already separately activated caller-owned bounded queue. Inlet and queue
cancellation remain distinct, queue backpressure retains rejected record ownership,
and raw timestamps remain unchanged. It adds no queue construction, worker,
recovery, compatibility breadth, or Manifold authority.

LSLC-004Z adds the first bounded discovery-to-data composition: one
caller-selected typed discovery response is strictly projected to its concrete
IPv4 service endpoint and passed to the existing separately capability-gated
one-record Float32 inlet. It adds no automatic selection, retained connection,
chunking, other formats, retry/recovery, or authority.

LSLC-004Y adds an explicit bounded composition from one caller-selected typed
discovery response to the existing separately activated inlet handshake. It
preserves strict endpoint and handshake errors and adds no automatic selection,
fallback, identity derivation, retained socket, sample transport, or authority.

Host validation uses pre-bound unicast responder sockets and non-cascading
test-only multicast serialization. These mechanisms change neither public APIs
nor production socket, timeout, activation, cleanup, or authority behavior.

LSLC-004X strictly projects one caller-selected accepted typed response's canonical
concrete-unicast IPv4 address and nonzero service port into a `SocketAddrV4` proposal.
It performs no I/O or connection and grants no routing, admission, activation, or
authority.

LSLC-004W adds only an allocation-free caller-explicit exact-name suggestion over an
already accepted bounded typed UDP discovery run. It returns the first receive-order
index or no suggestion and adds no I/O, ranking, endpoint selection, connection,
admission, routing, device behavior, or Manifold authority.

LSLC-003S registers a distinct `StringSample` capability in the closed feature
lock. It is selected-but-run-disabled and capability-only: no String transport
or runtime effect exists until a later reviewed runtime consumes exact nominal
admission plus the existing handshake dependency.

LSLC-003Q records observation-only protocol-110 String framing for one bounded
13-byte value, one channel, and one caller record in two repeated loopback runs.
It is evidence for a later bounded candidate, not a String implementation,
activation, arbitrary String support, or broad compatibility.

LSLC-003P implements a selected-but-run-disabled bounded sequence runtime for
exactly two homogeneous channels and three ordered records across double64,
int32, int16, and int8. It is finite IPv4-loopback candidate behavior bound to
LSLC-003O evidence, not activation, arbitrary-count support, non-loopback
support, broad ecosystem compatibility, or authority.

Rusty LSL is an independently authored Rust implementation of bounded Lab
Streaming Layer compatibility surfaces. Project-owned source is licensed
AGPL-3.0-or-later. Official liblsl is used only as a pinned black-box
compatibility oracle, never as an implementation template.

## Current capability surface

The `rusty-lsl` crate provides dependency-light bounded contracts for stream
descriptors, metadata trees, XML values and observed stream-info documents,
timestamps, homogeneous samples and chunks, and descriptor/sample binding.
These local contracts do not by themselves perform discovery, networking,
clock correction, buffering, recovery, protocol exchange, or runtime
activation.

The closed project lock selects eight candidate capability families:

- bounded sample queue;
- finite sample recovery;
- fixed-width numeric sample transport;
- integrated clock correction;
- short-info discovery responder;
- stream handshake;
- timestamped Float32 sample transport;
- UDP discovery.

Every selected capability remains disabled by default. Runtime effects require
the accepted lock plus an explicit descriptor-approved caller input and an
effective activation receipt. Selection is not activation, compatibility,
endpoint authority, discovery identity, authorization, or Manifold stream
authority. Supported claims and their evidence limits are defined by the
documents below, not by this summary.

LSLC-003O adds observation-only evidence for two-channel, three-record
`double64`, `int32`, `int16`, and `int8` sequences in two bounded pinned-
official IPv4-loopback directions. It adds no production implementation,
activation, broad interoperability, or authority.

## Authoritative project documents

- [Agent instructions](AGENTS.md) — ownership, public-safety, architecture,
  worktree, and validation rules.
- [Architecture](docs/ARCHITECTURE.md) — component boundaries and authority.
- [Compatibility](docs/COMPATIBILITY.md) — supported and unresolved
  interoperability claims.
- [Provenance](docs/PROVENANCE.md) — independent authorship and evidence
  classification.
- [Validation](docs/VALIDATION.md) — current commands and what they prove.
- [Validation policy](tools/validation-policy.json) — the sole current gate,
  profile, claim, and limitation authority used by local and CI dispatch.
- [Corpus](docs/CORPUS.md) and [oracle policy](docs/ORACLE.md) — public
  documentation inputs and black-box observation discipline.
- [Licensing](docs/LICENSING.md) and [supply chain](docs/SUPPLY_CHAIN.md) —
  source/dependency review boundaries.
- [Project workflow](morphospace/README.md) — bounded work-unit lifecycle and
  inert-by-default composition state.
- [Compatibility fixtures](fixtures/compatibility/README.md) — public-safe
  deterministic fixture routing.

## Preserved history

- [LSLC work-unit history](docs/history/LSLC-WORK-UNIT-HISTORY.md) preserves
  the chronological unit notes formerly carried by `AGENTS.md`.
- [README through accepted LSLC-003L](docs/history/README-THROUGH-LSLC-003L.md)
  preserves the complete prior README byte-for-byte as historical evidence.

Historical descriptions and passing local tests are not current runtime or
interoperability claims. Consult the authoritative documents and accepted
receipts for the exact scope of each claim.

LSLC-003T provides the closed capability-gated one-channel, one-record bounded
String loopback runtime observed by LSLC-003Q. It remains run-disabled unless
the caller presents the selected LSLC-003S `StringSample` capability.

LSLC-003U records sanitized pinned black-box evidence for mixed-width UTF-8 and
the exact 127-byte boundary. Private drivers, raw records, endpoints,
diagnostics, and version-drift runs remain outside the repository.

LSLC-003V confirms the unchanged LSLC-003T runtime handles both LSLC-003U
value classes in synthetic finite loopback; it changes tests and evidence only.

LSLC-003W records sanitized pinned black-box evidence for exactly one empty
String caller record in both finite IPv4-loopback directions. It changes no
runtime or activation; private drivers, raw records, endpoints, diagnostics,
environment, and machine identity remain outside the repository.

LSLC-003X extends the existing capability-gated one-channel, one-record String
runtime only to the LSLC-003W-observed empty value. The 127-byte maximum,
one-byte length form, finite loopback behavior, and activation closure remain.

LSLC-003Y records sanitized pinned black-box evidence that one independently
authored exact-128-byte String uses length form one with length 128 in both
finite loopback directions. It changes no runtime or activation.

LSLC-004B extends the closed capability-gated String runtime only through that
observed exact 129-byte boundary. Existing empty and nonempty cases remain;
130 bytes reject, and activation, framing form, finite loopback, and authority
do not widen.

LSLC-004C records sanitized single-platform evidence for one documented IPv4
multicast discovery group on an explicit loopback interface, including finite
query/response directions and membership cleanup. It changes no runtime and
does not generalize interfaces, retry policy, platforms, or authority.

LSLC-003Z extends the closed capability-gated String runtime only through that
observed exact 128-byte boundary. Existing empty and nonempty cases remain;
129 bytes reject, and activation, framing form, finite loopback, and authority
do not widen.

LSLC-004A records sanitized pinned black-box evidence that one independently
authored exact-129-byte String uses length form one with length 129 in both
finite loopback directions. It changes no runtime or activation.

LSLC-004D adds only a synthetic test of the unchanged explicit-destination UDP
requester against one joined loopback peer at exactly 239.255.172.215:16571.
It preserves the existing finite deadline, cancellation, cleanup, activation,
API, dependencies, and authority boundaries.

LSLC-004E adds only an explicitly activated responder for the same exact group
on a caller-selected loopback interface. It serves one bounded existing
short-info query/response and then releases membership and the socket; broader
interface, group, family, retry, routing, and authority policy remain absent.

LSLC-004F composes the unchanged accepted requester and responder only in a
synthetic test for one exact-group loopback query/response. It adds no runtime
surface or device claim.

LSLC-004G records sanitized, hash-bound device conformance from two Quest 3S
peers on one explicit IPv4 Wi-Fi interface. Both peers joined, dropped, and
rejoined the exact group; one peer received one multicast query and the other
received its one response. Explicit cancellation, socket and multicast-lock
cleanup, zero target-package fatals, and complete run-owned cleanup passed.
This changes no Rusty LSL runtime or activation and grants no Manifold
admission, routing, identity, or authority.

LSLC-004H records sanitized two-repeat evidence that the accepted Rust UDP
requester composed with an independently authored joined responder on one
caller-explicit active private IPv4 host interface at the exact group. One
bounded query/response, pre-I/O cancellation, no-response deadline, membership
cleanup, and socket cleanup passed per repeat. This single Windows host
observation does not widen the loopback-only production responder or establish
portable interface, retry, cross-host, device, official, or Manifold behavior.

LSLC-004I rebinds the inert-by-default runtime activation admission to the
accepted deterministic revision-14 lock fingerprint after the project
composition changed in LSLC-004G. The selected module set, dependencies,
explicit effective-marker requirements, runtime behavior, and authority do not
change; the superseded fingerprint now rejects as stale.

LSLC-004J adds one separately named responder entry point for the exact
239.255.172.215:16571 destination on a caller-supplied concrete IPv4 interface.
It neither discovers nor chooses that interface. Unspecified, multicast, and
broadcast values reject before socket I/O, while the earlier loopback-only API
and all activation, deadline, cancellation, and cleanup behavior remain.

LSLC-004K records sanitized pinned-official evidence on one caller-explicit
active IPv4 interface. In two private repeats the resolver emitted the exact
group query, accepted one independently authored bounded response, returned
the matching source, and cleanup passed. This observation changes no Rusty LSL
production surface and is not portable interface, cross-host, or device proof.

LSLC-004L aligns the policy-owned public-boundary dispatcher with its existing
immutable-evidence decision: only a tracked receipt whose current bytes exactly
equal its `HEAD` blob may retain historical missing-newline bytes. New,
untracked, modified, non-receipt, or otherwise damaged content still rejects;
the direct raw boundary checker remains unchanged.

LSLC-004N adds addressable observation-only evidence for two identical 65-byte
pinned-official short-info query datagrams. It publishes exact hashes and a
sanitized three-line grammar without publishing reply-routing/correlation
values or claiming stability beyond the two bounded repeats.

LSLC-004M test-only conformance shows that the unchanged exact-group responder
accepts the LSLC-004N-observed public query grammar with independently selected
five- and twenty-digit values. It does not replay private observed values or
change production runtime behavior.

LSLC-004O adds sanitized observation-only evidence that two pinned-official
resolver calls reached the unchanged exact-group production responder on one
explicit active host interface and returned one matching source with cleanup.
It adds no production widening or portable interface policy.

LSLC-004P adds the opposite observation direction: two pinned-official outlet
responses were admitted by the unchanged explicit-destination requester on one
bounded active IPv4 host-interface path. It adds no requester-interface policy,
production widening, multicast portability, or authority.

LSLC-004U exposes a bounded local `TypedUdpDiscoveryResponse` projection from
one accepted UDP response with explicit caller limits and no I/O or activation.

LSLC-004V exposes one caller-explicit `run_typed_udp_discovery` composition.
It uses the existing capability-gated bounded UDP call and typed projection,
preserving run termination, local/source addresses, receive order, and indexed
typed errors without adding discovery policy, compatibility breadth, or authority.
