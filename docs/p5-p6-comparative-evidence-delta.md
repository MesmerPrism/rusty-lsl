# P5–P6 Comparative Evidence Delta

This frozen P46 lane adds one substantive crate-private proposal owner over two actual P45 `CallerRequestedFloat32ComparativeAdvisoryEvidence` values. The module remains deliberately undeclared, private, default-inert, advisory, and non-applying.

The owner consumes and retains both complete P45 inputs unchanged. It derives exactly four deterministic facts, in fixed order: the existing total fact count, then exact counts of existing equal, increase, and decrease relations. Each fact retains the earlier and later counts and exactly one checked relation: equal, increase, or decrease. Ties remain explicit equal relations. No values, relation amounts, totals, or evidence are estimated, normalized, weighted, or inferred.

The caller supplies an explicit nonzero fact bound. Construction checks the bound before allocation, converts the fact count fallibly, uses checked arithmetic while counting existing evidence, derives differences with checked subtraction, and reserves the complete output allocation fallibly. Zero, exact, and one-past bounds are closed. Allocation, conversion, arithmetic, difference, and bound failures return both complete inputs unchanged. Success and consuming extraction preserve both P45 objects and every nested package, proposal, evidence, fact, and sample allocation by move.

The proposal performs no inference about loss, continuity, causality, quality, health, policy, compatibility, or liblsl behavior. It has no public or runtime export, no activation or application path, and no session, transport, stream, routing, admission, authorization, device, oracle, Manifold, or other runtime authority.

The `usize`-to-`u64` configuration boundary is explicit: platforms whose `usize` range exceeds `u64` reject an unrepresentable maximum, while platforms whose complete `usize` range fits in `u64` have no such runtime value. Tests exercise the conversion failure through the same fallible construction seam without making a platform-width assumption.
