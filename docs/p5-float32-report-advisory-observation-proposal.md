# P36 Float32 report advisory proposal candidate

## Decision and exact interface

This integrated candidate adds one crate-private deterministic advisory proposal
owner directly over the crate-private `MorphospaceFloat32ReportObservation`
owner. It defines no parallel
observation trait, sequence classification, disposition, effective timestamp,
or terminal-health model. It reads the sibling's ordered record index and
sequence, existing `ExactSequenceClassification`,
`RequestedTimestampPostProcessingDisposition`, adjustment and effective
timestamp facts, and terminal `ExactSequenceLossHealthSnapshot`.

The caller supplies a nonzero record bound and thresholds for the snapshot's
explicit missing-sequence, duplicate, out-of-order, and retained-changed
counts, plus a finite nonnegative maximum absolute adjustment. Zero is a valid
strict threshold and `u64::MAX` is a valid count threshold. Success is exactly
`RecommendRetain` or `RecommendReview`, carrying the sole unchanged observation
and exact evidence.

The largest-adjustment evidence contains the observation record index,
sequence, signed and absolute adjustment bits, and the existing effective
timestamp value/source. The ordered record index disambiguates duplicate
sequences. Equal absolute magnitudes retain the earliest ordered record: later
ties never replace it. Review reasons are deterministic: explicit missing,
duplicates, out of order, retained changed, then absolute adjustment.

## Evidence, ownership, and failures

Sequence health comes from the sibling's terminal
`ExactSequenceLossHealthSnapshot`; the proposal does not recreate or reinterpret
`First`, `Contiguous`, `Gap`, `Duplicate`, or `OutOfOrder`. “Missing” therefore
means only the exact caller-sequence count already supplied by the observation
owner. It is not estimated packet loss and is never inferred from absent calls,
record count, timestamps, disposition, transport, elapsed time, or queues.

Configuration, record-bound, representation, snapshot-extent invariant,
nonfinite-adjustment, and reason-allocation failures are typed. Both proposal
variants and every typed error return the exact owned observation. No record or
sample is cloned, moved out, replaced, or reallocated; the real retained sample
allocations remain owned by that observation.

## Authority and non-scope

The result is inert advice only. The owner cannot accept, route, lease, revise,
authorize, apply, activate, recover, retry, mutate, log, or audit anything, and
exposes no mechanism for those actions. It grants no device, ADB, Makepad,
Manifold, stream, runtime, or policy authority.

This independently authored candidate does not claim behavioral, numerical,
wire, protocol, post-processing, health, or loss equivalence with liblsl.

## Actual-interface validation and integration boundary

The crate root declares both modules privately. Neither module is re-exported,
registered, activated, or reachable through a public runtime facade. The
observation owner consumes the canonical successful P35 outcome, and the
proposal owner consumes that observation without adding a second evidence
model or an applying action.

Actual-interface tests cover all five exact sequence classifications, signed
and exact adjustment boundaries, effective timestamp source/value, every
relevant snapshot count, duplicate-sequence index disambiguation, deterministic
equal-magnitude selection and reason order, zero/extreme configuration,
record-bound and injected allocation failures, real sample-allocation identity,
and explicit authority denials.

Crate-level focused qualification exercises the real P35 outcome through
observation and retain/review proposal results. The private, default-inert
wiring adds no public facade, application surface, activation, or authority.
