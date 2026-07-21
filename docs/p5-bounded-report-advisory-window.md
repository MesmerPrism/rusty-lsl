# P37 bounded Float32 report advisory window candidate

## Decision and frozen sibling interface

This disposable, crate-private, unwired candidate reads an owned ordered window
through a faithful borrowed interface over exact P36 observations. Each report
exposes its report index, ordered records, checked record extent, and the exact
P36 explicit-missing, duplicate, out-of-order, and retained-changed aggregates.
Each record exposes its record index, caller sequence, and signed adjustment
bits. Report index followed by record index disambiguates every item, including
duplicate sequences.

The caller explicitly bounds report count, records per report, total records,
the four exact counters, and finite nonnegative absolute adjustment. Zero is a
valid strict threshold for every value threshold; `u64::MAX` is valid for all
integer thresholds. A nonzero report capacity is required. Exact equality
retains; only a strictly greater observation reviews.

## Deterministic result and failures

The result is exactly `Retain` or `Review`, always carrying the original owned
window and checked aggregate evidence. Review reasons have fixed order: total
records, explicit missing sequences, duplicates, out of order, retained
changed, then absolute adjustment. The largest absolute adjustment wins;
equal magnitudes use ascending report index and then ascending record index,
independent of the window's iteration order. Its report index, record index,
sequence, and signed/absolute bits remain addressable.

Empty windows, report and per-report record bounds, unrepresentable counts,
declared/actual record mismatch, checked counter overflow, nonfinite adjustment,
and reason-allocation failure are typed. Evaluation borrows before returning;
every failure returns the complete owned window without moving, cloning, or
reallocating its reports, records, or retained sample allocations.

## Exact facts and authority boundary

The candidate sums only counters already exposed by P36. “Missing” means only
P36's explicit missing-sequence count. It never estimates packet loss, infers
absence, reclassifies records, or invents terminal facts. It defines no
terminal model and does not derive facts from time, transport, queues, report
spacing, or omitted calls.

The output is inert advice. There is no apply, accept, route, lease, revision,
authorization, application, activation, or audit mechanism. It grants no
runtime, device, ADB, Makepad, Manifold, stream, policy, or public authority and
makes no behavioral, numerical, wire, protocol, health, loss, or liblsl
equivalence claim.

## Qualification and integration boundary

The source is intentionally absent from `lib.rs` and every public/runtime
surface. Its isolated faithful sibling fixtures exercise zero and extreme
thresholds, repeated identical windows, exact aggregate and reason order,
report/record index tie resolution, record-bound and counter-overflow failures,
injected allocation refusal, retained allocation identity, extent/nonfinite
failures, and explicit authority denials. A future reviewed integration would
need to implement the borrowed interface for the frozen actual P36 observation
without copying or reinterpreting facts; this candidate is not integration,
activation, acceptance, or runtime evidence.
