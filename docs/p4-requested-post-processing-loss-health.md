# P4 requested post-processing loss-health candidate

This candidate is a crate-private-ready, effect-free observation owner. A caller
supplies each sequence number and one explicit post-processing fact: retained
unchanged, retained changed, or discarded. The owner returns the exact sequence
relationship and maintains an immutable-copy snapshot of checked bounded counts.

## Exact interpretation

The first number establishes a high-water mark. A number one greater than that
mark is contiguous; a greater number beyond it is a gap whose intervening count
is computed exactly; equality is a duplicate; and a lower number is out of
order by an exact distance. Only a number above the mark advances it. This means
an old repeated number is conservatively classified out of order, not duplicate:
the bounded owner does not retain an unbounded set of all earlier numbers.

The explicit missing-sequence count is evidence about the caller-provided
sequence domain only. It is not estimated packet loss and does not assert that
transport emitted, dropped, or should recover any packet. Likewise, discarded
is counted only when the caller states that post-processing discarded that
observation. No behavior is inferred from samples, timestamps, sockets, queue
state, elapsed time, or absence of calls.

Every affected counter is checked in a candidate snapshot before commit. The
caller-configured observation limit and arithmetic overflow are typed errors,
and either refusal leaves all prior state unchanged. Snapshots are deterministic
value copies containing counts, the high-water sequence, and the last explicit
classification/fact; they borrow and duplicate no sample allocation or failure
evidence.

## Authority boundary and future integration

This file is deliberately not first-hop documentation and the module is not yet
wired through `lib.rs` or `runtime.rs`. A future reviewed P4 integration may
place the owner after a caller-selected sequence source and requested
post-processing stage, forwarding the stage's explicit disposition. That
adapter must preserve the sequence domain, decide an observation bound, and
choose when to take or publish snapshots; none of those decisions belongs here.

The candidate performs no I/O, transport-loss estimation, implicit missing-
packet inference, thresholding, policy selection, recovery, queue action,
background monitoring, logging, activation, or device work. It grants no
Makepad or Manifold authority. Stream admission, routing, leases,
authorization, revision, and audit remain outside Rusty LSL.
