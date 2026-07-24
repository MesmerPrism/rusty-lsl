# P5 bounded report-window delta proposal

The crate-private `MorphospaceFloat32ReportWindowDeltaProposalOwner` consumes
and retains two complete, actual P37 `MorphospaceFloat32ReportObservationWindow`
values. It verifies their checked totals from the retained observations and
emits deterministic evidence in a fixed order: the eleven proven P37 counters,
then the largest absolute timestamp-adjustment observation. Counter evidence is
exactly unchanged, increased, or decreased and carries a checked absolute
`u64` delta. Adjustment ties retain the earliest report/record in each window;
equal magnitudes are unchanged regardless of sequence or signed bits.

Caller bounds cover reports per window, records per report, and total evidence.
They are explicit, nonzero, and checked for `usize` to `u64` representation.
Evidence allocation is fallible and exact. Every error consumes no partial
result and returns both complete windows in their original earlier/later order,
including their original record allocations.

“Missing” means only the P37 counter for explicitly observed missing sequence
identities. The proposal does not estimate or infer packet loss, continuity, or
transport behavior.

This data owner is default-inert. It cannot apply, accept, activate, admit,
route, lease, revise, authorize, or audit anything. It is not a claim of liblsl
equivalence and grants no Manifold stream authority, device authority, or
application policy authority.
