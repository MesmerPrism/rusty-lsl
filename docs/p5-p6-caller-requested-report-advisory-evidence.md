# Caller-requested report/advisory evidence composition

This candidate adds one crate-private, default-inert composition between the actual P34 transactional Float32 report lifecycle outcome and an actual P40 `MorphospaceFloat32ReportAdvisorySnapshot`. Construction occurs only through an explicit caller request. It consumes and retains both complete inputs and builds a deterministic report-first index over their already-existing evidence.

The caller supplies nonzero bounds for advisory evidence and total ordered evidence. Bound conversion, advisory count, total-count arithmetic, index conversion, and allocation are fallible. Every typed failure returns the complete P34 outcome and P40 snapshot unchanged; construction performs no mutation before all validation succeeds. Success preserves the original sample and evidence allocations because the composition moves the owners and copies only compact facts into its index.

The first ordered item records the P34 sequence, exact classification, requested-processing disposition, raw timestamp bits, and effective timestamp bits. Remaining items retain P40 evidence in its existing order with explicit zero-based indices. These are observations only: no continuity or packet-loss value is inferred, and no advice is accepted or applied.

The module has no root or runtime export and adds no activation, socket, plugin, device, Quest, Android, Makepad, session, stream, transport, control, application, Manifold, or liblsl-equivalence authority. Focused unit qualification temporarily declares the private module; that declaration is removed after the focused tests.
