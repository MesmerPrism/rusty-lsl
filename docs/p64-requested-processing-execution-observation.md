# P64 requested-processing execution observation

## Decision

P64 adds one typed, immutable, bounded Morphospace-facing observation projection over exact P63
requested-processing execution-batch and per-cycle supervision facts. The caller supplies opaque
`u128` source and execution identities; P64 preserves them exactly and does not interpret,
validate, correlate, discover, or authorize either value.

Each committed cycle produces exactly one ordered cycle observation from exactly one P63
supervision value. Distinct batch cycles remain distinct executions and are never combined as
snapshots. The observation preserves the caller budget, zero-based cycle association, report and
execution extents, first and last committed prefixes, remaining extent, current index,
termination totals, recovery-attempt and stage-health facts, queue facts, and final P62 health.

## Completion, refusal, and unavailable facts

A completed batch is labelled `BudgetCompleted`; this is exact finite-budget exhaustion, not a
recommendation to run again. A stopped batch retains the exact stopped cycle and borrows the
unchanged P62 refusal as its cause. Because P63 does not commit the refused cycle, its supervision
state is explicitly `NotReportedForUncommittedCycle`; P64 does not merge partial refusal evidence
into the committed prefix.

An exact last queue length is reported only when P62 supplied one. Otherwise queue length is
`NotReportedByP62`. Loss remains the existing `NotReportedByP62` fact. P64 never converts absent
queue or loss evidence to zero, and never derives, grades, or forecasts health beyond retaining the
exact P62 health projection.

## Transactional bounded admission

The caller supplies nonzero cycle and aggregate-report limits. Before returning an observation,
P64 rejects an exceeded cycle or report bound, report-count overflow, mismatched committed-cycle
and supervision cardinality, completed-prefix/budget disagreement, invalid stopped-cycle
association, or disagreement between a committed P62 queued extent and its P63 supervised
execution extent. Cycle storage is reserved exactly; allocation refusal returns no partial
observation. Existing P62 and P63 constructors remain the owners of within-report and
within-supervision contradiction checks.

## Authority boundary

This module is CPU/data-only and default-inert. It performs no execution, collection, retry,
recovery, requested processing, queue admission or measurement, scheduling, clock work, storage,
monitoring, inference, recommendation, policy, route selection, authorization, activation,
background work, device work, Makepad work, or Manifold action. It defines no Standard profile,
workflow, public facade, public main, tag, release, or compatibility claim.

The candidate is intentionally not wired through `lib.rs` or `runtime.rs`; facade ownership stays
with the integrator. Focused unit tests remain in the source file and can be run through a
standalone include harness without changing a facade.
