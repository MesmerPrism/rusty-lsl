# Validation

LSLC-006C focused UDP discovery response-ownership and exact closure conformance:

```text
cargo test -p rusty-lsl udp_discovery::tests -- --test-threads=1
python ./tools/check_lslc_004u.py
python ./tools/check_lslc_004v.py
```

It proves receive-order preservation, exact source/query identity, consuming
response allocation ownership, pre-cancellation precedence, immediate reuse
of the caller-selected port, and exact LSLC-004U/004V closure over the updated
test-bearing module. It is host-only test evidence and changes no production
discovery behavior, compatibility claim, device behavior, or Manifold
authority. Serialized Standard remains the aggregate owner gate.

LSLC-006A focused runtime-activation receipt-authority conformance:

```text
cargo test -p rusty-lsl runtime_activation::tests -- --test-threads=1
```

It proves canonical receipt identity across caller selection order, exact lock
and consumer binding, capability-marker agreement, and typed rejection without
partial authority. It is host-only test evidence and changes no production
activation behavior, compatibility claim, device behavior, or Manifold
authority. Serialized Standard remains the aggregate owner gate.

LSLC-005Z runs the unchanged LSLC-005Y assertion under the permitted
integration-test target:

```text
cargo test -p rusty-lsl --test public_api lslc_005z_runtime_acquisition_parts_preserve_borrowed_witness_and_all_four_value_allocations -- --exact --test-threads=1
```

The standalone LSLC-005Y target is removed. The assertion still proves only
that an accepted runtime acquisition exposes its exact witness and four values
to a borrower and returns the same witness and original value allocations when
consumed. Standard remains the aggregate owner gate and this test proves no
production behavior, provider policy, I/O, compatibility, device, or Manifold
behavior.

LSLC-005X focused coverage is:

```text
cargo test -p rusty-lsl stream_info_transport_provider::tests::provider_output_accessors_preserve_witness_and_value_allocations -- --exact --test-threads=1
```

It proves borrowed and consuming access preserves the exact provider witness,
six value roles, and allocations. Standard remains the aggregate owner gate
and proves no new provider, acquisition policy, I/O, compatibility, device, or
Manifold behavior.

LSLC-005W runs the unchanged LSLC-005V external conformance under the
repository-permitted integration-test target:

```text
cargo test -p rusty-lsl --test public_api lslc_005w_ -- --test-threads=1
```

The four assertions prove only the accepted evidence-limit constructor, Unicode-scalar
provider-identity bound, exact typed error payloads, and identity-mismatch
precedence. Standard remains the aggregate owner gate and proves no new
production behavior, provider policy, compatibility breadth, device behavior,
or Manifold authority.

LSLC-005U runs the unchanged LSLC-005T stateful-acquisition assertions through
the repository-permitted public API integration-test target:

```text
cargo test -p rusty-lsl --test public_api sequential_stateful_acquisitions_are_call_isolated_and_recover_after_typed_failures -- --exact --test-threads=1
```

The standalone LSLC-005T target is removed. This is test-target-only corrective
evidence; Standard remains the aggregate owner gate.

LSLC-005T focused stateful-acquisition conformance:

```text
cargo test -p rusty-lsl --test transport_provider_stateful_acquisition -- --test-threads=1
```

It covers sequential accepted, provider-error, value-error, recovery, and
exhaustion outcomes with exact call counts and unchanged prior accepted
ownership. It is host-only test evidence and changes no provider policy,
runtime behavior, compatibility breadth, device behavior, or authority.
Standard remains the aggregate owner gate.

LSLC-005S device validation is a separate serial-scoped gate. The public Quest
harness builds exact clean Rusty LSL and Rusty Quest revisions for
`aarch64-linux-android`, then requires the Rust-owned
`rusty.lsl.rust_on_quest_float32_two_record_chunk.v1` effective marker, two
ordered exact timestamp/value-bit pairs, immediate TCP port reuse, zero bounded
fatals, and target-only package/process/forward/reverse/property/staging cleanup.
Standard remains the source owner gate; neither route replays the official
oracle or proves arbitrary chunks, non-loopback behavior, or broader runtime
compatibility.

LSLC-005R focused transport-provider conformance:

```text
cargo test -p rusty-lsl stream_info_transport_provider::tests -- --test-threads=1
```

It covers one-call acquisition, mismatch precedence, typed value ownership,
allocation preservation, fixed role order, and repeated determinism. It is
host-only test evidence and does not change provider policy, runtime behavior,
compatibility breadth, device behavior, or authority. Standard remains the
aggregate owner gate.

LSLC-005Q focused three-owner snapshot conformance:

```text
cargo test -p rusty-lsl lslc_005q_ -- --test-threads=1
```

It covers caller-selected acquisition order, separate typed provider errors,
allocation preservation, delegated limits, and repeated deterministic
no-cross-owner composition. It is host-only test evidence and does not change
provider policy, runtime behavior, compatibility breadth, device behavior, or
authority. Standard remains the aggregate owner gate.

LSLC-005O focused exact-closure validation reuses the existing LSLC-004V
checker:

```text
python ./tools/check_lslc_004v.py
```

Only the accepted typed projection blob is rebound; the UDP blob, semantics,
damaged mutations, and Standard policy inventory remain unchanged.

LSLC-005N focused typed discovery-response conformance:

```text
cargo test -p rusty-lsl lslc_005n_ -- --test-threads=1
```

It covers exact UTF-8 position and envelope-error ownership plus repeated
exact-boundary acceptance with complete IPv6 source preservation. Standard
remains the aggregate owner gate.

LSLC-005M focused clock-correction damage and soak conformance:

```text
cargo test -p rusty-lsl lslc_005m_ -- --test-threads=1
```

LSLC-005L device validation is a separate serial-scoped gate. The public Quest
harness builds exact clean Rusty LSL and Rusty Quest revisions for
`aarch64-linux-android`, then requires the Rust-owned
`rusty.lsl.rust_on_quest_float32_loopback.v1` effective marker, exact
timestamp/value-bit retention, immediate TCP port reuse, zero bounded fatals,
and target-only package/process/forward/reverse/property/staging cleanup.
Standard remains the source owner gate; neither route replays the official
oracle or proves non-loopback or broader runtime compatibility.

LSLC-005K focused conformance runs with
`cargo test -p rusty-lsl lslc_005k_ -- --test-threads=1`; it repeats twelve
deterministic recovery/correction/queue cycles with pressure, cancellation,
terminal bypass, teardown, and immediate TCP/UDP port reuse. It is host-only
test evidence and does not replace Standard validation or prove new runtime,
compatibility, device, or authority breadth.

LSLC-005J focused conformance runs with
`cargo test -p rusty-lsl lslc_003p_ -- --test-threads=1`; it covers the
accepted fixed-width sequence owner’s damaged paths and cleanup only and does
not replace Standard validation or prove broader runtime compatibility.

LSLC-005H device validation is a separate serial-scoped gate. It requires the
Rust-owned `rusty.lsl.rust_on_quest_core_contract.v1` effective marker from an
exact-source-locked `aarch64-linux-android` build, zero bounded target/system
fatals, and complete removal of the run-owned package/process without changing
forwards, reverses, properties, or staging. It proves one local core contract
execution on Quest, not Java LSL behavior, transport, or broad compatibility.

LSLC-005G adds serialized synthetic terminal-path tests for unchanged
LSLC-005D. They prove that terminal, exhausted, recovery-cancelled, and
recovery-deadline outcomes do not read the clock or admit a queue record. They
do not prove official interoperability, devices, automatic policy, provider
selection, merged cancellation/backpressure, or Manifold authority.

LSLC-005F adds two serialized synthetic damaged-path tests for unchanged
LSLC-005D. They prove record/state ownership across post-recovery clock
cancellation and post-correction queue cancellation. They do not prove
official interoperability, devices, automatic policy, provider selection,
merged cancellation/backpressure, or Manifold authority.

LSLC-005E adds one serialized synthetic loopback test for the unchanged
LSLC-005D recovery-to-correction-to-queue path. It proves caller-classified
retry order, exactly-once clock correction after recovery, queue admission,
and bit-preserving raw/value evidence beside a derived timestamp. It does not
prove official interoperability, device behavior, automatic policy, provider
selection, merged cancellation/backpressure ownership, or Manifold authority.

LSLC-005D focused tests use the lslc_005d filter; Standard remains the owner
gate for the complete bounded recovery, correction, queue, public API, and
public-boundary closure.

LSLC-005C focused validation is `cargo test -p rusty-lsl lslc_005c`. It proves
only unchanged raw/value reconstruction beside the accepted clock owner's
derived value and public composition shape. Standard remains the owner gate and
proves no automatic, official, device, broad clock, or Manifold behavior.

LSLC-005B focused validation is `cargo test -p rusty-lsl lslc_005b`. It proves
only caller-classified retry over one fixed selected endpoint, exact recovered
record queueing, and retained sample/state evidence on queue cancellation. Standard
proves no automatic, official, device, broad recovery, or Manifold behavior.

LSLC-005A focused validation is `cargo test -p rusty-lsl lslc_005a`. It proves
only the bounded selected-response Float32 inlet-to-existing-queue composition,
exact raw timestamp/value preservation, and separate queue cancellation ownership.
Standard remains the owner gate and proves no official, device, recovery, broader
compatibility, or Manifold behavior.

LSLC-004Z focused validation is `cargo test -p rusty-lsl lslc_004z`. It proves
only the bounded selected-response-to-one-Float32-record composition and
projection-before-I/O rejection. Standard remains the owner gate and proves no
device, non-loopback, broad compatibility, or Manifold behavior.

LSLC-004Y focused coverage checks projection-before-I/O, one finite synthetic
loopback handshake, delegated typed failures, cancellation, and cleanup. Device
validation is forbidden.

The focused owner-determinism check is
`cargo test -p rusty-lsl lslc_002z -- --test-threads=1`. The tests retain their
original assertions and cleanup checks while beginning with exact socket ownership;
shared multicast serialization does not propagate poison between tests.

The aggregate `rust-tests` owner gate preserves the complete workspace/all-targets suite
and runs its test binaries with `--test-threads=1` to isolate unrelated timing-sensitive
loopback cases. Historical workflow vocabulary must be checked through
`tools/check_lslc_004q.ps1`, which binds the accepted LSLC-004Q adoption receipt and exact
owner commit; the raw owner validator is not a substitute for that project-owned route.

LSLC-004X focused coverage checks canonical concrete-unicast IPv4 service endpoint
projection and malformed, noncanonical, zero-port, unspecified, multicast, and
broadcast rejection without I/O.

LSLC-004W focused coverage checks exact first-match suggestion, no-match, empty-input
rejection, and unchanged typed-discovery behavior. It is source-only and forbids device
or official-oracle claims.

LSLC-003S focused activation validation runs
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003s.ps1`.
It validates the exact descriptor source binding, resolver-owned relative
paths, canonical lock fingerprint/revision, workspace registry, nominal
capability, dependency closure, absence-is-inert behavior, damaged fixture
inventory, all LSLC-003C preservation checks, Rust tests, and public boundary.
It proves no String transport, I/O, framing, runtime execution, device behavior,
ambient activation, or authority.

LSLC-003Q focused validation runs
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003q.ps1`.
The policy-owned `lslc-003q-observation` gate checks exact sanitized bounds,
framing, hashes, nonclaims, damaged fixture mutations, required routes, and the
public boundary. It does not rerun the private oracle or prove implementation,
activation, damaged-peer behavior, arbitrary Strings, devices, or authority.

LSLC-003P focused validation runs
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003p.ps1`.
The policy-owned `lslc-003p-runtime` gate proves only the closed two-channel,
three-record local runtime contract, not activation, arbitrary counts,
non-loopback behavior, devices, or broad compatibility.

The sole current validation-policy authority is
[`tools/validation-policy.json`](../tools/validation-policy.json). Run its portable facade:

```text
python ./tools/dispatch_validation.py --profile quick
python ./tools/dispatch_validation.py --profile standard
python ./tools/dispatch_validation.py --profile deep
```

`tools/check_all.ps1` is the compatibility wrapper for `standard`; CI invokes
the policy-owned `ci` profile directly. Gates declare stable IDs, owners,
dependencies, change scope, claims, limitations, environment, and timeouts.
Receipts record executions but never select policy.

LSLC-003O observation evidence is checked through the policy-owned
`lslc-003o-observation` gate. Its focused direct route is:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003o.ps1
```

This validates the sanitized bounded matrix, damaged evidence mutations,
provenance hashes, public/private boundary, and documentation routes. It does
not rerun the private oracle or prove a production runtime.

Pinned immutable evidence is routed by `validation/historical-gates.json` and
the bound v2 manifest. The complete prior validation guide is preserved
byte-for-byte at
[`docs/history/VALIDATION-THROUGH-LSLC-003M.md`](history/VALIDATION-THROUGH-LSLC-003M.md).
The migration inventory and decision are in
[`docs/validation/VALIDATION-AUTHORITY-INVENTORY.md`](validation/VALIDATION-AUTHORITY-INVENTORY.md)
and [`docs/adr/LSLC-003N-VALIDATION-POLICY-AUTHORITY.md`](adr/LSLC-003N-VALIDATION-POLICY-AUTHORITY.md).

LSLC-003T is checked by `tools/check_lslc_003t.ps1` and policy gate
`lslc-003t-runtime`, covering exact capability composition, valid loopback,
damaged framing, UTF-8 and byte bounds, cancellation, deadline, cleanup,
provenance, documentation, and public boundary.

LSLC-003U is checked by `tools/check_lslc_003u.ps1` and policy gate
`lslc-003u-observation`. The gate validates the two exact cases, four pinned
attempt hashes, six damaged mutations, documentation routes, limitations, and
the current public boundary without executing an oracle in portable validation.

LSLC-003V is checked by `tools/check_lslc_003v.ps1` and policy gate
`lslc-003v-runtime-conformance`, including the accepted production-prefix byte
comparison, two focused loopback cases, damaged fixture mutations, routes, and
public boundary.

LSLC-003W is checked by `tools/check_lslc_003w.ps1` and policy gate
`lslc-003w-observation`. The gate validates the exact empty record, two pinned
attempt hashes, six damaged mutations, documentation routes, limitations, and
the public boundary without executing an oracle in portable validation.

LSLC-003X is checked by `tools/check_lslc_003x.ps1` and policy gate
`lslc-003x-runtime`. It validates empty-value loopback, prior bounds and
capability markers, six damaged fixture mutations, cleanup, documentation
routes, and the public boundary.

LSLC-003Y is checked by `tools/check_lslc_003y.ps1` and policy gate
`lslc-003y-observation`. The gate validates the exact 128-byte record, two
pinned attempt hashes, six damaged mutations, documentation routes,
limitations, and public boundary without executing an oracle.

LSLC-004H is checked by `tools/check_lslc_004h.ps1` and policy gate
`lslc-004h-active-interface-observation`. Portable validation checks the
sanitized exact-source fixture, two repeat dimensions, private artifact hashes,
six damaged mutations, documentation routes, limitations, and public boundary.
It performs no network operation and does not replay private artifacts.

LSLC-004I is checked by `tools/check_lslc_004i.ps1`. It independently
recomputes the revision-14 fingerprint, checks the runtime and current fixtures,
runs the direct LSLC-003S and LSLC-003J owners, and executes closed activation
tests proving current admission, stale rejection, and inert empty selection.

LSLC-004D is checked by `tools/check_lslc_004d.ps1` and policy gate
`lslc-004d-runtime-conformance`. It byte-compares the production prefix with
accepted LSLC-004C, rejects six damaged fixture mutations, runs the focused
synthetic multicast test, retains the deadline/cancellation owner markers, and
checks the public boundary.

LSLC-004E is checked by `tools/check_lslc_004e.ps1` and policy gate
`lslc-004e-runtime`. It runs the exact joined-loopback owner test, rejects six
damaged fixture mutations, retains the LSLC-002Z cancellation/deadline owner
route, checks documentation and public boundaries, and forbids broader claims.

LSLC-004F is checked by `tools/check_lslc_004f.ps1` and policy gate
`lslc-004f-runtime-conformance`. It byte-checks both accepted production
prefixes, rejects six damaged fixtures, runs one exact composition test, and
checks documentation and public boundaries.

LSLC-004G is checked by `tools/check_lslc_004g.ps1` and policy gate
`lslc-004g-quest-device-conformance`. Portable validation checks only the
sanitized exact-head fixture, seven private-artifact hashes, six damaged
mutations, cleanup/fatal outcomes, limitations, and public boundary. It does
not contact a headset or replay private artifacts.

LSLC-004B is checked by `tools/check_lslc_004b.ps1` and policy gate
`lslc-004b-runtime`. It validates exact-129 loopback and cleanup, typed
130-byte rejection, preservation markers, six damaged fixture mutations,
documentation routes, and public boundary.

LSLC-004C is checked by `tools/check_lslc_004c.ps1` and policy gate
`lslc-004c-observation`. It validates the exact group, explicit-loopback and
single-platform scope, two repeats, membership cleanup, both finite directions,
hash bindings, six damaged mutations, documentation routes, and public boundary.

LSLC-003Z is checked by `tools/check_lslc_003z.ps1` and policy gate
`lslc-003z-runtime`. It validates exact-128 loopback and cleanup, typed
129-byte rejection, preservation markers, six damaged fixture mutations,
documentation routes, and public boundary.

LSLC-004A is checked by `tools/check_lslc_004a.ps1` and policy gate
`lslc-004a-observation`. The gate validates the exact 129-byte record, two
pinned attempt hashes, six damaged mutations, documentation routes,
limitations, and public boundary without executing an oracle.

LSLC-004J is checked by `tools/check_lslc_004j.ps1` and policy gate
`lslc-004j-runtime`. It validates the exact destination, caller-explicit
concrete-interface contract, two sanitized private active-interface repeats,
loopback-wrapper preservation, typed pre-I/O nonconcrete rejection, six damaged
fixture mutations, documentation routes, owner tests, and public boundary.

LSLC-004K is checked by `tools/check_lslc_004k.ps1` and policy gate
`lslc-004k-observation`. Portable validation checks exact pinned versions,
group/interface scope, two sanitized query/response and cleanup results,
private-artifact hashes, limitations, six damaged mutations, documentation
routes, and public boundary without executing the private oracle.

LSLC-004L is checked by `tools/check_lslc_004l.ps1` and policy gate
`lslc-004l-immutable-receipt-head-identity`. The focused self-test positively
binds exact immutable LSLC-003M and LSLC-004J bytes and rejects modified or new
receipts, non-receipts, private paths/content, credentials, build artifacts,
and trailing whitespace. It also hash-checks that the direct raw checker and
both accepted receipts remain byte-identical.

LSLC-004N addressable official-query evidence is checked with
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004n.ps1`.
It verifies exact hashes/lengths, sanitized grammar, pin and two-repeat limits,
ten damaged mutations, documentation routes, and the current public boundary;
it does not replay the private oracle.

LSLC-004M is checked with
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004m.ps1`.
The route runs the focused owner test, ten damaged fixture mutations, exact
production-prefix binding, public boundary, and the current Standard profile.

LSLC-004O is checked with
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004o.ps1`.
It verifies two sanitized official-to-production results, exact production
prefix, ten damaged mutations, public boundary, instructions, and Standard;
the private oracle and active interface are not replayed by portable gates.

LSLC-004P is checked with
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004p.ps1`.
It verifies two sanitized official-outlet-to-production-requester results,
exact accepted requester bytes, ten damaged mutations, public boundary,
instructions, and Standard; the private outlet and interface are not replayed.

LSLC-004Q is checked with
`pwsh -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004q.ps1`
while `RLSL_WORK_ENVIRONMENT_ROOT` names an exact clean materialization of
owner commit `50f8e8a67641f535347c3061d531e6d4df46e535`. The gate validates all
fourteen immutable legacy units through the project receipt and keeps every
current unit closed against current registries.

LSLC-004R is checked with
`pwsh -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004r.ps1`.
It verifies two addressable response and document hashes/lengths, minimum
sanitized structure, honest dynamic-byte limits, ten damaged mutations,
documentation routes, and the current public boundary without replaying the
private oracle or response values.

LSLC-004S is checked with
`pwsh -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004s.ps1`.
It verifies exact production-prefix identity, runs the two focused owner tests,
rejects ten damaged fixture mutations, checks documentation and the public
boundary, and never replays an official response or private value.

LSLC-004T is checked with
`pwsh -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004t.ps1`.
It verifies the exact unchanged production prefix, runs the focused typed
composition and damaged delegation, rejects ten evidence mutations, and checks
documentation plus the public boundary.

LSLC-004U is checked with
`pwsh -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004u.ps1`.
It pins the unchanged UDP blob, tests positive and delegated-error paths,
checks public API visibility, rejects ten mutations, and runs public boundary.

LSLC-004V is checked with
`pwsh -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_004v.ps1`.
It pins the unchanged UDP and typed-projection blobs, runs focused positive,
indexed-damage, cancellation, cleanup, and public-API cases, rejects ten
evidence mutations, and checks the public boundary without oracle or device work.
# Float32 two-record chunk integration candidate

Focused validation is `cargo test -p rusty-lsl candidate_two_record_chunk`.
It proves only exact two-record ordering, raw timestamp/value preservation,
pre-I/O count rejection, and loopback cleanup for the independently authored
candidate. Standard remains the aggregate owner gate. Neither route replays
the private oracle or proves canonical acceptance, arbitrary chunks, devices,
broad interoperability, or Manifold authority.
