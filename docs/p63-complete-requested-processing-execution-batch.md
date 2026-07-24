# P63 complete requested-processing execution batch

## Decision

P63 publicly exposes the finite execution-batch and immutable supervision
contracts and composes them without widening either owner. A completed batch is
accepted only with exactly one caller-supplied P62 report series for every
committed cycle. Each series is independently validated by the supervision
owner under the same explicit nonzero report bound.

## Interface reconciliation

The batch candidate describes separate execution cycles. The supervision
candidate describes ordered snapshots of one execution extent. Consequently the
composition never passes reports from different cycles to one supervision
value. The ordered outer series corresponds one-for-one with the batch's
committed cycle order; association of observed reports with those cycles remains
an exact caller fact.

Cardinality refusal retains the unchanged completed batch and exact expected and
actual counts. Allocation refusal also retains the batch. A supervision refusal
retains the batch, the zero-based cycle, and the unchanged typed supervision
error. No partially composed public value is returned. Success can be consumed
back into the unchanged batch and ordered supervision values.

## Authority boundary

The composition is CPU/data-only. It performs no execution, report collection,
retry, recovery, queue admission or observation, clock work, processing,
scheduling, cancellation, storage, background work, or activation. It fabricates
no loss or queue facts. Existing P60, P61, and P62 owners and caller choices
remain unchanged, and activation remains explicit and default-disabled. It adds
no device, Makepad, Morphospace, or Manifold authority.

## Focused validation

Focused tests cover separate per-cycle supervision, exact ordered projection,
and zero-based retention of the first typed supervision refusal. Candidate tests
continue to cover finite budget completion, stop-prefix ownership, allocation
precedence, snapshot aggregation, explicit unavailable loss facts, changed
extent, prefix regression, and post-completion refusal.
