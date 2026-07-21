# P36 Float32 report advisory proposal candidate

## Decision and exact interface

This repaired candidate adds one crate-private deterministic advisory proposal
owner directly over `MorphospaceFloat32ReportObservation` from exact sibling
commit `32dbc7b2869fbf2f165cadc1d344c17fcff98dbf`. It defines no parallel
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

## Isolated actual-interface validation and integration requirement

The candidate base intentionally does not contain or wire the sibling module,
and this task forbids `lib.rs` changes. Validation therefore uses a disposable
copy of the repository containing the exact sibling observation source from
commit `32dbc7b2869fbf2f165cadc1d344c17fcff98dbf`. Only that disposable copy may
add temporary module declarations or accessor adaptation; the candidate
checkout remains limited to its two owned paths.

Actual-interface tests cover all five exact sequence classifications, signed
and exact adjustment boundaries, effective timestamp source/value, every
relevant snapshot count, duplicate-sequence index disambiguation, deterministic
equal-magnitude selection and reason order, zero/extreme configuration,
record-bound and injected allocation failures, real sample-allocation identity,
and explicit authority denials.

A future reviewed integration must land or otherwise provide the exact sibling
observation, wire both private modules, and rerun crate-level qualification.
That wiring must not add a public facade or any application or authority
surface. This candidate is not itself actual runtime integration evidence.
