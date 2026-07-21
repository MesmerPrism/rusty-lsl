# P5 bounded report-window stability proposal

The P39 owner is a crate-private, default-inert, consuming proposal over the
actual P38 `MorphospaceFloat32ReportObservationHistory`. It compares each pair
of adjacent, exact P37 windows in retained order and returns the complete owned
history on success and on every error. Original sample allocations are neither
copied nor replaced.

The companion crate-private P39 delta-history owner accepts the actual P38
window-delta proposal type. Focused composition qualification retains those
proposals in insertion order, consumes their concrete window pairs back into
the actual P38 observation-history type, and passes that history directly to
the stability owner. The qualification covers exact sample-allocation identity,
complete typed-failure inputs, deterministic equality/increase/decrease,
reason and tie order, explicit missing-sequence evidence only, bounds,
overflow, and absence of partial mutation.

The caller supplies explicit maximum windows, reports per window, records per
report, evidence entries, counter change, and absolute-adjustment change. All
size conversions, totals, evidence cardinality, and allocations are checked
before a proposal is returned. Evidence order is stable: adjacent window pair,
then the eleven counters in their declared order, then largest absolute
timestamp adjustment. Equal adjustment magnitudes retain the first record in
report and record order.

Every evidence entry states equal, increase, or decrease and whether the exact
absolute change is within or exceeds the caller threshold. The missing-sequence
entry is solely the already recorded `explicit_missing_sequence_count`.
Absence of explicit evidence never becomes inferred loss, inferred continuity,
packet-loss estimation, or a compatibility claim.

The result is advice only. It cannot apply or accept itself and grants no
admission, routing, lease, revision, authorization, audit, activation, device,
Makepad, or Manifold authority. It is not behavioral, numerical, wire, or other
equivalence with liblsl. Callers remain the sole owners of any later decision.
