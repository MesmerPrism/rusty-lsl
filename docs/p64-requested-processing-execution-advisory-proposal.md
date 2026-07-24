# P64 requested-processing execution advisory proposal

## Decision

P64 defines one immutable, caller-owned, data-only advisory proposal over an ordered slice of
already validated P63 per-cycle supervision values. The caller supplies an opaque producer and
execution-batch identity, an exact declared cycle count, and explicit nonzero cycle, report, and
execution-extent bounds. The proposal retains the borrowed source unchanged and copies only
fixed-size, addressable P63 facts for each increasing zero-based cycle.

The descriptive result is `AllCyclesComplete` only when every final P63 snapshot has zero
remaining executions and no current index. Otherwise it is `IncompleteEvidencePresent`. Neither
classification requests or causes behavior.

## Exact binding and refusal

The owner is constructed for one expected source identity. It refuses identity substitution,
empty input, declared/actual cycle drift, cycle or per-cycle report expansion, execution-extent
expansion, contradictory completed/remaining/current-index facts, and allocation failure. Every
refusal returns the unchanged caller source, including the identical borrowed supervision slice.
Distinct cycles remain distinct; P64 neither combines them into one execution nor reconstructs
the P62 report series that P63 intentionally does not retain.

Accepted evidence preserves the exact P63 report count, total extent, first and last completed
prefix, final remaining extent and current index, final P62 health, termination totals,
recovery/queue owner facts, and P63's explicit `NotReportedByP62` loss fact. Elapsed time and the
cause of observed progress are explicitly `NotSuppliedByP63`; absence is never converted to zero,
success, continuity, or causality.

## Authority boundary

This crate-private candidate is default-inert, non-applying, and unwired. It defines no Manifold
command, admission, routing, lease, revision, authorization, scheduling, audit, execution, retry,
recovery policy, queue, clock, discovery, socket, background work, activation, public facade, or
public main. It performs no I/O and grants no device, Makepad, Morphospace runtime, or Manifold
authority. Existing P60 through P63 owners and all caller associations remain unchanged.

## Focused validation

Source-local tests cover exact identity and borrowed-slice retention, ordered exact evidence,
complete/incomplete classification, explicit unknown facts, identity substitution, cycle drift,
and extent expansion. A standalone include harness compiles these tests with the P62 report and
P63 supervision modules without editing a facade.
