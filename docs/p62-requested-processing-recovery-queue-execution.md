# P62 requested-processing recovery/queue execution

## Decision

P62 is an unwired, synchronous CPU/data-only adapter. It borrows the exact completed P60
Float32 requested-processing lifecycle, delegates caller-classified finite attempts to
`run_finite_sample_recovery`, and delegates bounded admission to `BoundedSampleQueue::push`.
Requested timestamp processing has already completed, so P62 does not invoke or own clock
correction.

## Authority and ownership

- P60 remains the owner of discovery, selection, requested processing, ordered record evidence,
  sequence facts, and committed health.
- The caller remains the owner of activation, recovery policy, failure classification, both
  cancellation signals, queue wait bounds, and the queue.
- The finite-recovery runtime remains the retry, deadline, cancellation, and state-trace owner.
- The bounded queue remains the storage, waiting, backpressure, closure, and admission owner.
- P62 holds no sample collection or worker. It borrows completed evidence and clones a processed
  sample only after a caller attempt succeeds, at the ownership-transfer boundary required by
  queue admission.

## Exact outcomes

Success reports the exact completed lifecycle borrow plus ordered record index, sequence, and
finite-recovery states. Recovery cancellation, deadline, terminal failure, exhaustion, and typed
setup failure retain that same immutable completed evidence and the already-queued prefix. Queue
backpressure, cancellation, deadline, closure, or poison additionally returns the existing queue
error, which owns the unchanged rejected sample. No error is reclassified and no terminal recovery
path invokes queue admission.

## Validation

Because the module is intentionally not wired into the crate facade, qualification uses a
standalone `rustc --test` harness that includes the crate root and adds only this module for the
test build. Focused deterministic tests cover ordered success, retry then success, terminal and
exhausted recovery, pre-attempt cancellation, full-queue backpressure, closed queue, and queue
cancellation while checking exact Float32/timestamp bits and retained evidence identity.

## Non-scope

No discovery, connection, transport, requested-processing, clock, inferred retry, background
work, queue implementation, activation default, facade, Cargo, device, Makepad, Manifold,
integration, workflow, cleanup, or publication authority is added.
