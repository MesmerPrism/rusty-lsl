# P5–P6 Advisory Report-Package Delta Proposal

This lane adds one crate-private, caller-requested, default-inert proposal owner over exactly two actual `CallerRequestedFloat32AdvisoryReportPackage` P43 values. The module remains intentionally undeclared, so it has no production or activation surface.

The caller supplies an earlier package, a later package, and an explicit nonzero relation bound. Successful construction consumes and retains both complete packages unchanged and returns four facts in fixed order: history-value count, history-evidence count, summary-fact count, and package-fact count. Each fact records the two existing exact `u64` totals and exactly one deterministic relation: equal, increase by an exact checked difference, or decrease by an exact checked difference. Ties are explicit `Equal` facts and retain their fixed position.

Construction validates the bound, uses checked `usize` arithmetic and checked `usize`/`u64` conversion, derives checked `u64` differences, and reserves the complete fact allocation fallibly before committing any fact. Zero, one-past, arithmetic, conversion, difference, and allocation failures return both complete packages unchanged, including their nested sample allocations. Construction is all-or-nothing and has no observation, application, activation, I/O, or background side effect.

The proposal is advisory and non-applying. Count relations do not infer packet loss, continuity, cause, health policy, or compatibility equivalence. In particular, this work claims no liblsl equivalence and acquires no Manifold, session, stream, transport, control, routing, admission, authorization, or application authority.
