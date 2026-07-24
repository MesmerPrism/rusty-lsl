# P63 requested-processing execution supervision

## Decision

P63 defines an immutable, bounded CPU/data-only supervisor over an ordered
series of already validated P62 execution reports. It aggregates exact report,
termination, recovery-attempt, queue-observation, and derived-health facts. It
does not execute, retry, schedule, process, admit, monitor, or mutate anything.

## Snapshot contract

All reports in one supervision value must describe the same execution extent.
Their fully completed prefix may stay unchanged or advance, but cannot regress.
A complete report is final and cannot be followed by another snapshot. The
supervisor retains the first and last completed counts, the last remaining
extent/current index/health, exact termination totals, summed explicitly
reported recovery attempts, exact queue observation classifications, and the
last queue length that a P62 termination actually supplied.

Construction is transactional. Empty input, a zero or exceeded report bound,
changed total extent, regressed prefix, a report after completion,
contradictory report/health projections, or counter overflow returns a typed
error and no supervision value. P63 does not reinterpret one termination as
another or claim that it caused progress between snapshots.

## Loss and authority boundary

P62 contains no loss count or loss classification. Consequently
`loss_facts()` returns only `NotReportedByP62`; absence is not converted to
zero loss. P63 does not inspect P60 sequence evidence and accepts no ambient
loss estimate. Queue lengths remain exact caller-observed P62 facts. Recovery
attempts remain exact finite-recovery facts already represented by P62.

P60 remains the discovery, selection, requested-processing, sequence, and
processing-health owner. Finite recovery retains attempt, retry, cancellation,
deadline, and classification authority. The bounded queue retains storage,
wait, backpressure, closure, and admission authority. The caller retains
activation and policy. P63 adds no loss, queue, recovery, scheduling, alerting,
threshold, background-work, clock, storage, device, Makepad, Morphospace, or
Manifold authority, and it remains unwired and default-disabled.

## Focused validation

Unit coverage in the source file exercises exact multi-snapshot aggregation,
termination/recovery/queue/health projection, explicit unavailable loss facts,
changed-extent rejection, prefix-regression rejection, post-completion
rejection, and empty/bounded-series refusal. The module is intentionally not
wired through `lib.rs` or `runtime.rs` in this lane; a temporary external test
harness can compile the P62 report module and P63 supervisor together without
changing facade paths.
