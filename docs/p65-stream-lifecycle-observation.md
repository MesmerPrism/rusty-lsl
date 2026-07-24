# P65 whole-session lifecycle observation

## Decision

P65 defines one immutable, typed, finite whole-session observation spanning caller-selected typed
discovery, connection establishment, caller-requested Float32 post-processing, P64 execution
observation extents, terminal state, close, cleanup, and observable health. The caller supplies one
opaque provenance value plus the accepted source ID, session ID, and stream UID. Identity repeated
by an owner must match exactly; owners that do not expose identity are represented explicitly as
`NotExposedByOwner`.

The observation borrows identity text and copies only fixed-size facts. It does not retain a socket,
discovery run, sample allocation, processing owner, queue, or refusal. Integration should project
facts from the existing P54 selected-discovery preflight/connection owner, the completed P60
requested-processing lifecycle, P61 health, and the P64 immutable execution observation. P62 and
P63 remain the authorities for report and supervision validity.

## Transactional association

Construction validates the complete candidate before returning it. Nonempty identity components
have a caller-supplied byte bound. Selection must name an existing receive-order response, its
concrete datagram source, and its separately projected concrete service endpoint. An established
peer must equal that service endpoint; the discovery datagram source port is not substituted for
the advertised service port. Repeated caller,
source, session, and stream identities must be byte-exact across stages.

Requested processing cannot precede connection. Execution cannot precede requested processing.
Completed and stopped P64 extents must agree with their finite budget: completion commits the whole
budget; refusal names exactly the next uncommitted cycle. Cycle, aggregate report, and record extents
are bounded. Executed records cannot exceed the records retained by requested processing. Exact
health must agree with processing and execution extents. Zero-attempt recovery, impossible loss,
terminal-before-connection, close-before-terminal, cleanup-without-close, and successful cleanup
after close refusal are rejected. A refusal returns no partial observation.

## Explicit absence

`NotAttempted`, `NotRequested`, `NotExecuted`, `NotReached`, `NotObserved`,
`NotExposedByOwner`, and `NotReportedByAcceptedOwners` are evidence states, not defaults inferred as
success. In particular, accepted P62/P63/P64 evidence supplies no packet-loss count. Integration
must therefore use `NotReportedByAcceptedOwners` unless another accepted native owner supplies an
exact count. Cleanup is `NotObserved` unless the lifecycle owner supplies exact release evidence;
port reuse or resource release must not be guessed from canonical completion alone.

## Integration contract

The facade integrator must register the module without widening visibility, and must map only
already accepted facts:

- P54/P55 supplies receive-order selection, response source, accepted identity, connection peer,
  transfer terminal state, terminal close, and any exact cleanup evidence it actually exposes.
- P60 supplies the same completed discovery run and response index, retained processing record
  count, and requested-processing health. Those facts must agree with the selected-discovery
  binding.
- P61 may supply exact recovery/queue health; unavailable facts remain `NotObserved`.
- P64 supplies caller source/execution identity, budget, committed-cycle association, report
  extent, stopped-cycle state, and completed-prefix health. The integrator must preserve its exact
  per-cycle association and must not flatten distinct cycles into snapshots.
- The integrator must reject before construction if its source token cannot be associated exactly
  with the P65 source identity. It must not turn canonical close into cleanup evidence or absent
  P62 loss into zero loss.

The source intentionally remains unwired in this lane. A standalone `rustc --test` include harness
can qualify its focused unit tests without editing `lib.rs` or `runtime.rs`.

## Authority boundary

P65 is immutable CPU/data evidence only. It makes no recommendation and performs no discovery,
selection, connection, transfer, processing, recovery, retry, queue admission, scheduling,
routing, authorization, activation, storage, monitoring, background work, lease, revision, audit,
device, Makepad, or Manifold action. It defines no schema or command authority. Runtime activation
remains explicit and default-disabled.
