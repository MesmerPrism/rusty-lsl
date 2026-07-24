# P42 retained report/advisory summary

This candidate adds one crate-private, default-inert summary owner over the actual P41 caller-requested report/advisory evidence composition and its actual bounded advisory-snapshot history. The caller supplies both complete owning inputs and explicit nonzero bounds. Successful construction retains those canonical owners unchanged and creates a separate deterministic index containing only exact `Copy` facts already exposed by them.

The summary records each P41 retained-evidence item in its existing order, then each history snapshot and its advisory evidence in existing snapshot/evidence order. Indices and counts are exact. It does not estimate missing packets, infer loss or continuity, reinterpret evidence, accept advice, or apply advice.

Configuration checks nonzero bounds and `usize`-to-`u64` representability. Construction checks retained-evidence and history-snapshot bounds, every count and index conversion, all count additions, the total summary bound, and exact allocation. No input is mutated while these checks run. Every typed failure returns the complete P41 evidence composition and history unchanged, including their retained allocations; success moves those owners without reconstructing or duplicating them.

The module has no root or runtime export and adds no activation, public API, liblsl-equivalence claim, socket, plugin, session, stream, transport, control, application, Manifold, device, Quest, ADB, Android, or Makepad authority. Focused actual-code tests temporarily declare the private module; the declaration is removed after testing.
