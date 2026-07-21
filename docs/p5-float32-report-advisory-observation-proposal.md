# P36 Float32 report advisory observation proposal candidate

## Decision and interface

This candidate adds one crate-private, deterministic, effect-free advisory
proposal owner over the frozen completed Float32 report observation shape. The
borrowed interface exposes ordered records and terminal exact health. Each
record exposes its caller sequence, explicit sequence classification,
successful disposition, adjustment bits, and effective timestamp bits.

The caller supplies a nonzero record bound plus explicit thresholds for the
sum of classification-supplied missing sequences, duplicates, out-of-order
records, changed dispositions, and maximum absolute adjustment. Zero is a
valid strict threshold; `u64::MAX` is a valid counter threshold; the adjustment
threshold must be finite and nonnegative. Success returns exactly one typed
`RecommendRetain` or `RecommendReview` proposal, the original observation, and
checked exact evidence. Review reasons have stable order: terminal health,
explicit missing, duplicate, out-of-order, changed, then absolute adjustment.

## Authority and non-scope

“Missing” is counted only from an observation record explicitly classified as
a gap. The owner never estimates packet loss, interprets absent sequences,
reclassifies records, or derives loss from timestamps, disposition, terminal
health, elapsed time, transport, queues, or record count. A recommendation is
data only. It cannot accept, route, lease, revise, authorize, apply, audit,
recover, retry, activate, or mutate anything, and it exposes no mechanism for
doing so. All configuration, record-bound, checked-counter, nonfinite-value,
and reason-allocation failures are typed and return or precede ownership of the
unchanged observation.

This independently authored advice does not claim behavioral, numerical,
protocol, post-processing, health, or loss equivalence with liblsl. It grants
no device, ADB, Makepad, Manifold, stream, policy, or runtime authority.

## Validation and future integration

Because this exact clean base contains no wired sibling observation module and
this task forbids changes to `lib.rs` and `runtime.rs`, validation uses only the
candidate's faithful isolated sibling stub implementing the borrowed interface.
Tests cover zero and extreme configuration, exact threshold edges, gaps,
duplicates, out-of-order records, changed and signed adjustments, deterministic
multi-reason ordering, typed refusal, retained allocation identity, and
unchanged observation contents.

A future actual-code integration is required before this candidate can consume
the production sibling observation. That reviewed integration must implement
the borrowed interface for the frozen actual observation without copying or
reinterpreting its facts, wire the module explicitly, and rerun crate-level
qualification. It must not add any application or authority surface while
doing so. This candidate is not itself runtime integration evidence.
