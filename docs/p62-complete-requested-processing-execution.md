# P62 complete requested-processing execution

## Decision

`run_complete_requested_processing_recovery_queue_execution` is the public
bounded production path joining the two P62 candidates with P61 observability.
It accepts an already completed P60 Float32 lifecycle and explicit caller-owned
activation, recovery policy, cancellation signals, queue wait bounds, report
limits, queue, P61 lifecycle, failure classifier, and queue-length observer.

## Authority and interfaces

P60 remains the discovery, selection, requested-processing, sequence, and
processing-health owner. Finite recovery owns attempts, retry timing, deadline,
cancellation, and state evidence. The bounded queue owns storage, waiting,
closure, and admission. The caller remains the exact queue-length observation
authority because the queue exposes no public storage snapshot. P62 invokes
that observation only immediately after admission or refusal and never infers a
length.

The composed outcome returns the existing execution outcome, the stable report,
and immutable P61 health. Execution failures retain the existing failure and
completed prefix. Report validation occurs before P61 mutation. P61 observations
are applied to a clone and committed together, so any P61 refusal leaves prior
health unchanged. Recovery setup/allocation and poisoned-queue failures have no
stable report classification and are returned without fabricated report facts.

## Observability

Successful records project their exact recovery attempt, P60 sequence-loss
classification, and caller-observed accepted queue length. Exhaustion and
cancellation project through P61's existing terminal-recovery contract.
Terminal failure and recovery deadline remain exact in the stable report but do
not acquire a new P61 classification. Queue deadline and cancellation retain
their exact stage and queue length in the report.

## Validation and non-scope

Focused P62 tests cover success, retry, cancellation, deadline, exhaustion,
terminal recovery, backpressure, closed queue, exact sample/state retention,
bounded report rejection, and transactional P61 refusal. Relevant P61 and P60
tests remain the regression surface. No Standard profile is part of this unit.

P62 adds no discovery, connection, timestamp processing, recovery policy,
queue policy, clock, storage, cancellation, scheduling, background work,
activation default, device, Makepad, Morphospace, or Manifold authority.
