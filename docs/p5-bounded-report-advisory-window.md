# P37 bounded Float32 report trend proposal candidate

## Decision and concrete interface

This repaired disposable candidate consumes the concrete crate-private
`MorphospaceFloat32ReportObservationWindow` frozen at `bebc5741`. It directly
borrows the window's ordered `MorphospaceFloat32ReportObservation` values,
their ordered `MorphospaceFloat32ReportRecordObservation` values, and the
window's checked `MorphospaceFloat32ReportObservationWindowTotals`. It defines
no compatibility trait, mirror record, mirror observation, or synthetic window
interface.

No sibling change is required for the candidate: the actual window already
provides borrowed observations and checked totals, while actual P36 records
already provide crate-private record index, caller sequence, and processed
facts. A future integrator only needs private module composition; no new public
or runtime accessor is requested.

## Thresholds, evidence, and deterministic order

The caller explicitly supplies nonzero report and per-report record bounds,
plus thresholds for total records, explicit missing sequences, duplicates,
out-of-order records, retained-changed records, and finite nonnegative absolute
adjustment. Count thresholds accept zero through `u64::MAX`. Equality retains;
only strict exceedance reviews.

The result is exactly `Retain` or `Review` and returns the complete concrete
window with its original observation, record, and sample allocations. Evidence
contains the exact checked window totals and the largest actual P36 adjustment.
Report index is the zero-based ordered window position. Record index is the
actual P36 record index, so duplicate caller sequences remain unambiguous.
Largest-magnitude ties retain the earliest report index, then earliest record
index. Review reasons are fixed as total records, explicit missing sequences,
duplicates, out of order, retained changed, then absolute adjustment.

## Exact aggregation and failures

Trend collection checked-sums the actual observations only to verify the
frozen window totals. The returned counters are the window owner's totals, not
a replacement model. The proposal neither reconstructs classifications nor
creates terminal state. “Missing” is only the exact explicit-missing counter
already supplied by P36 and checked by the window; it is not estimated loss.

Empty window, trend report bound, per-report record bound, checked aggregation
overflow, window-total mismatch, and review-reason allocation refusal are
typed. Every failure returns the complete concrete window. Collection and
reason allocation occur while borrowing it; reports, observations, records,
and retained sample allocations are never copied, extracted, or replaced.

## Authority and integration boundary

This is inert proposal data only. The source defines no operation that applies
or accepts a result and no admission, routing, lease, revision, authorization,
application, activation, or audit mechanism. It has no root/runtime export,
feature-lock entry, device behavior, ADB, Makepad, Manifold authority, or
liblsl-equivalence claim.

Focused qualification is performed in a disposable source copy composed from
the exact `c607605` candidate history, the exact `bebc5741` window source, and
the exact P36 observation/runtime sources. The copy adds only private module
declarations needed to compile the two frozen sibling candidates together.
Tests construct real P36 observations, append the actual window, and exercise
trend retain, review, strict thresholds, reason and tie order, repeated windows,
all trend failures, checked-overflow injection, and real allocation identity.
The actual window's own focused tests additionally cover allocation rollback
with retry and atomic refusal for every checked aggregate counter. Structural
tests inspect the disposable crate root, runtime, and activation source to
prove private-only reachability without comment-word assertions.

The candidate remains absent from this branch's `lib.rs`; canonical integration
must compose the exact window before the repaired trend source. This repair is
not integration, public activation, acceptance, or runtime evidence.
