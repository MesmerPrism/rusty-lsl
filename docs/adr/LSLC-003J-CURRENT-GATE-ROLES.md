# LSLC-003J Historical And Current Gate Roles

Status: accepted for implementation in LSLC-003J.

## Decision

Preserve the LSLC-003H v1 manifest and every historical checker byte. Add a v2
manifest with two explicit roles: eighteen receipt/hash/pin-bound historical
replays and one live forward-current LSLC-003J checker. Historical gates run
only in detached clean worktrees at their accepted ancestral pins; the
dispatcher owns removal and rejects failed cleanup. The live checker owns the
current descriptor, resolver lock, activation binding, compatibility fixtures,
and workspace projection.

## Drivers And Alternatives

Historical unit checkers contain valid unit-specific source and scope
assumptions, so replaying them on an evolved live tree conflates history with
forward closure. Editing them would rewrite accepted evidence; dropping them
would lose executable history. Pinning preserves their original meaning while
a dedicated current checker supplies durable forward coverage.

## Consequences And Boundaries

Full dispatch is intentionally heavier because it creates and removes isolated
worktrees. All pins, receipts, launchers, companions, and the v1 manifest are
verified before execution. No dependency, device, runtime breadth, public API,
protocol behavior, or Manifold authority is added. LSLC-003I remains blocked
history while its exact representation-only implementation is dependency-
closed by LSLC-003J.

## Confirmation

- `python ./tools/test_dispatch_current_gates_v2.py`
- `powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003j.ps1`
- `powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1`

Review this decision if Git worktree cleanup semantics change or a future unit
adds another live forward checker.
