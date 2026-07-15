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

## Durable Promotion Rule

The immutable v1 inventory is the exact ordered prefix of historical roles.
Later accepted gates may be appended after that prefix only with a unique ID,
contiguous order, ancestral pin, receipt, launcher, companion, and exact hashes.
A successor current role must remain nonempty and disjoint from all historical
IDs. The live checker validates relationships rather than LSLC-003J values: the
current source, descriptor and ancestral source revision, resolver-owned lock,
runtime activation constants, and workspace module projection must agree. It
does not require an active unit, so acceptance may set `current_unit` to null.

Worktree cleanup is part of the replay result. Removal must succeed, the exact
owned path must disappear from `git worktree list --porcelain`, and its directory
must be absent. A failure leaves the owned path for explicit recovery and never
invokes broad pruning.
