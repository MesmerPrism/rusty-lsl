# P63 requested-processing execution batch

## Decision

P63 defines a finite synchronous batch contract over exact P62 requested-processing,
finite-recovery, and bounded-queue execution outcomes. The caller supplies a nonzero exact cycle
budget and a cycle executor. P63 invokes that executor once for each increasing zero-based cycle
until the budget is consumed or the first P62 refusal is returned.

## Ownership and evidence

A completed batch retains the ordered P62 outcomes for exactly the caller-budgeted number of
cycles. A stopped batch retains the original budget, exact zero-based stopped cycle, every fully
committed earlier P62 outcome, and the unchanged P62 execution error as its stop cause. Consuming
access returns the retained outcomes and error without reclassification or projection. No later
cycle is invoked after a refusal.

The caller-provided executor remains the sole owner of each cycle's completed P60 evidence,
activation, recovery policy and classification, cancellation inputs, queue and wait bounds, and
queue-length observation. P62 remains the owner of per-record recovery state, admitted-prefix
evidence, rejected sample ownership, and terminal classification. P63 does not merge partial
current-cycle evidence into the committed cycle prefix; that evidence remains inside the exact
P62 error.

## Bounds and non-scope

Zero cycles are rejected before execution. The batch performs only a bounded synchronous loop and
retains at most the exact caller-permitted number of successful P62 outcomes. Budget completion is
not a failure and does not authorize another cycle. Exact committed-prefix capacity is reserved
before the first cycle; allocation refusal retains the original budget and invokes no cycle.

P63 creates no implicit retry, scheduling, recovery, queue, backpressure, clock, requested
processing, discovery, connection, storage, cancellation, activation, background work, device,
Makepad, Morphospace, or Manifold authority. It defines no Standard profile, facade, workflow,
publication, tag, release, or public-main integration.

## Focused validation

Unit structure in the source file checks exact increasing cycle invocation, exact finite budget
completion, immediate stop at the first refusal, retained committed-prefix order, unchanged stop
cause, allocation refusal before execution, and zero-budget rejection. The candidate intentionally
is not wired through `lib.rs` or `runtime.rs`; therefore Cargo cannot discover these new unit tests
in P63 without an unauthorized facade edit. Existing crate tests remain the available regression
surface, and no temporary compile route is introduced.
