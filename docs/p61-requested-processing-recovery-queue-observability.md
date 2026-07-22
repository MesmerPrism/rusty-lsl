# P61 requested-processing recovery/queue observability

P61 publicly composes the completed P60 Float32 requested-post-processing
lifecycle with exact facts from the finite-recovery and bounded-queue owners.
`CompleteTypedUdpDiscoveryFloat32RequestedPostProcessingRecoveryQueueLifecycle`
is an observability transaction: it does not discover, connect, recover,
process timestamps, admit queue items, infer loss, or select policy.

A recovered observation may be bound to a borrowed completed P60 lifecycle and
one exact queue outcome. The completed lifecycle retains discovery, selection,
record allocation, requested mode, processing facts, and committed sequence
health. The P61 projection retains only a copy of that immutable health plus
the two candidate modules' fixed-size snapshots. Exhaustion and cancellation
use `observe_terminal_recovery` and cannot claim processing or queue admission.

Each update stages both candidate owners, delegates their validation, and
commits only after every fact is accepted. Invalid attempt bounds, loss facts,
queue lengths, backpressure claims, observation bounds, or contradictory
terminal claims therefore leave all composed state unchanged. Queue rejection
does not erase an existing accepted processing projection.

The crate root and `runtime` facade expose the two candidate contracts and the
composition. Activation remains explicit and default-disabled. Discovery,
connection, recovery, cancellation, requested processing, queue/backpressure,
drop policy, and loss classification remain with their existing owners; P61
adds no retry, inference, background work, device, Makepad, or Manifold
authority.
