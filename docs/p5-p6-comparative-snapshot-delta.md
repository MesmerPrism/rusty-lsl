# P5–P6 Comparative Snapshot Delta

This P48 candidate adds one substantive crate-private bounded pairwise advisory delta proposal over two actual published P47 `CallerRequestedFloat32ComparativeAdvisoryEvidenceSnapshot` values. The owner consumes and retains both complete snapshots unchanged and returns both unchanged on every failure.

The proposal derives six fixed-order facts solely from exact existing snapshot observations: total observations, history-evidence observations, delta-fact observations, and delta facts whose retained relation is equal, increase, or decrease. Each fact records the exact earlier and later counts and a checked equal, increase, or decrease relation. Equality is an explicit tie; no tie is broken or reinterpreted. Nothing estimates or infers loss, continuity, causality, quality, health, or policy.

The caller supplies an explicit nonzero fact bound. Construction checks and fallibly converts the fixed fact count, uses checked arithmetic for every derived count, checked subtraction for directional amounts, and fallibly reserves the complete result before emitting facts. Zero, tight, and one-short bounds are explicit. Bound, conversion, arithmetic, subtraction, and allocation failures return both complete P47 snapshots with every nested owner and allocation identity unchanged.

The module is deliberately crate-private, undeclared after focused qualification, default-inert, advisory, and non-applying. It has no root or runtime export, activation or application path, liblsl-equivalence claim, or Manifold, session, stream, transport, control, routing, admission, authorization, device, or oracle authority.
