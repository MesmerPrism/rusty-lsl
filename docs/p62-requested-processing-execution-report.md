# P62 requested-processing execution report

P62 defines a stable public, bounded, CPU/data-only report contract for an
already attempted requested-processing/recovery/queue execution. The P62
composition constructs the report only from exact evidence already supplied by
the batch, finite-recovery, requested-processing, and bounded-queue owners.

`RequestedProcessingExecutionReportLimits` fixes nonzero execution, recovery
attempt, and queue-capacity bounds. One immutable
`RequestedProcessingRecoveryQueueExecutionReport` retains the exact total,
completed prefix, remaining extent, optional zero-based current index, one
terminal classification, and fixed-size derived health. Complete means the
entire extent completed requested processing and queue admission. Every other
classification identifies the first execution outside that completed prefix.

Cancellation and deadline facts name their exact owner stage. Recovery-stage
termination exposes no processing or queue fact. Processing-stage termination
requires a recovered sample but exposes no completed processing or queue fact.
Queue-stage termination requires successful recovery and processing plus an
exact queue length. Exhaustion consumes exactly the configured attempt budget;
terminal recovery retains its exact completed-attempt count. Backpressure is
valid only at exact full capacity, while closure retains any in-capacity length.

Construction validates the complete candidate before returning public state.
Zero or exceeded bounds, impossible completed/total extents, success with an
incomplete extent, failure after a complete extent, zero successful attempts,
attempt overflow, premature exhaustion, queue lengths beyond capacity,
non-full backpressure, and stage-inconsistent evidence return typed errors.
No partially populated report is observable.

The health projection states only whether the current execution was recovered,
processed, or observed by the queue and whether its queue result was
backpressure or closure, alongside the exact fully completed prefix. It does
not infer attempts for prior executions, estimated packet loss, drop counts, or
any fact absent from the supplied owner evidence.

The contract performs no I/O, allocation, clock acquisition, timestamp
processing, recovery, retry, cancellation, queue admission, sample retention,
loss classification, monitoring, background work, activation, device work, or
policy selection. It adds no Makepad, Morphospace, Manifold, or integration
authority.

Focused crate coverage checks complete extent, each
recovery terminal class, cancellation/deadline stage boundaries, queue
backpressure and closure, exact health projection, and fail-closed rejection of
contradictory or unbounded facts.
