# P61 Requested-Processing Queue Health

P61 defines a stable public CPU/data-only health contract for a caller-owned
bounded queue after requested timestamp post-processing. It is intentionally
not wired through the crate root in this lane.

`RequestedPostProcessingQueueHealthConfig` binds the exact nonzero queue
capacity and a nonzero observation limit. The health owner accepts only exact
caller facts: accepted, explicitly dropped, full-queue backpressure, closed,
and cancelled. Every fact includes the observed queue length. A snapshot
contains the fixed capacity, per-outcome counts, total admitted observations,
the maximum observed queue length, and the last admitted fact.

The contract does not inspect or own queue storage, retain samples, perform
timestamp processing, read clocks, run recovery, choose drop policy, infer
packet loss, or perform transport/session work. In particular, “dropped” means
the caller explicitly reports its own completed drop decision; the existing
bounded queue continues to return unchanged rejected sample ownership and does
not acquire an automatic drop policy.

Validation is fail-closed and atomic. Zero capacity or zero observation limits
are rejected. A queue length above capacity, an accepted result with resulting
length zero, backpressure below full capacity, any fact after closure, an
exhausted observation bound, or counter overflow leaves prior health unchanged.
The observation bound takes precedence once exhausted. Closure is terminal for
this operation-outcome stream; draining remains wholly owned by the existing
queue and can be represented by a separate health owner if a caller needs a
distinct observation interval.

This milestone adds no activation, queue, backpressure policy, retry,
background work, monitoring threshold, alerting, clock, recovery, discovery,
session, device, Makepad, Morphospace, or Manifold authority. A canonical
integrator may feed successful P60 requested-processing results into its
existing queue and then submit the exact queue outcome here.
