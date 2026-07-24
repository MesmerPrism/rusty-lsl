# LSLC-003K: Pinned Rust 1.80 Clippy Baseline

Status: accepted policy candidate

## Decision

Rusty LSL runs Clippy through the exact Rust 1.80 toolchain and compares every
coded warning against a canonical machine-independent baseline. The baseline
retains duplicate occurrences because library diagnostics are emitted again
for the all-target test build. Compiler summary diagnostics separately bind the
319 library and 350 all-target test counts.

The policy rejects a wrong Clippy/rustc identity, failed invocation, changed
summary counts, missing or changed diagnostics, and any newly introduced
diagnostic. Canonical sorting makes compiler emission order irrelevant while
requiring the checked-in artifact itself to have one deterministic order.
CI invokes this focused policy before the complete v2 dispatch. LSLC-003J
remains the sole generic current-manifest checker; this unit does not widen the
claimed envelope to rewrite its dispatcher regression suite.

## Consequences and authority

The baseline exposes existing warning debt without changing source, lint
levels, behavior, API, protocol, activation, dependencies, devices, or
authority. It is not permission to suppress or bulk-fix warnings. A future
cleanup unit may remove warnings and update the baseline only through an exact
claimed envelope, focused review, and complete current-gate validation.

Host identity and rendered diagnostic prose are excluded. Exact diagnostic
codes, messages, child messages, source spans, labels, and suggestions are
retained; paths are project-relative or pinned-rustc virtual paths.

## Confirmation

Run `python ./tools/test_lslc_003k.py` for canonical and damaged-artifact
coverage, `./tools/check_lslc_003k.ps1` for real pinned-toolchain comparison,
and `./tools/check_all.ps1` for complete current dispatch and owner gates.
