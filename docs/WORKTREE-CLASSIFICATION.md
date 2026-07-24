# Registered worktree classification

This source-only snapshot records the non-mutating classification observed
after public-main integration. It is preservation and handoff evidence, not
cleanup authority. Worktrees, branches, indexes, and files were not reset,
cleaned, pruned, removed, overwritten, or otherwise changed.

The inventory used `git worktree list --porcelain` followed by
`git status --porcelain=v1 --untracked-files=all` in every registered
worktree. Names below are worktree basenames; all changed paths are
repository-relative. No local root or absolute path is retained.

## Summary

- registered and present: 326
- missing or prunable: 0
- locked: 0
- branch-attached: 315
- detached: 11
- clean: 319
- dirty: 7
- tracked-only dirt: 1
- untracked-only dirt: 6
- mixed tracked and untracked dirt: 0
- status failures: 0

## Material dirty overlays

### `rusty-lsl-parallel-int64-discovery-p30-20260720`

- branch: `codex/rlsl-parallel-int64-discovery-p30-20260720`
- commit: `97cd008f91cd81b95003b9e4d79d24b9f07e0ec1`
- category: four unstaged tracked Rust edits
- paths:
  - `crates/rusty-lsl/src/lib.rs`
  - `crates/rusty-lsl/src/runtime.rs`
  - `crates/rusty-lsl/src/typed_udp_discovery_integer_session_connection.rs`
  - `crates/rusty-lsl/tests/public_api.rs`

### `rusty-lsl-candidate-review-v7-on-2c06c764`

- state: detached
- commit: `2c06c764520eaac5eb38e5813dffd3971124b15b`
- category: three untracked Rust integration tests
- paths:
  - `crates/rusty-lsl/tests/composition_roundtrip_matrix.rs`
  - `crates/rusty-lsl/tests/cross_family_error_precedence.rs`
  - `crates/rusty-lsl/tests/unicode_timestamp_interactions.rs`

## Incidental generated overlays

Five other dirty worktrees contain only untracked Python bytecode under
`tools/__pycache__/`:

- `rusty-lsl-autonomous` — two files
- `rusty-lsl-p55r-integration-20260722` — two files
- `rusty-lsl-p56-integration-20260722` — two files
- `rusty-lsl-parallel-clock-buffer-recovery-p7-20260719` — one file
- `rusty-lsl-parallel-session-report-p4-p9-20260719` — one file

These generated overlays remain in place. Their classification does not make
them disposable and grants no deletion authority.

## Planning handoff

`morphospace/workspace.state.json` remains unchanged because source cannot
self-certify post-publication workflow closure. External planning must
independently bind public `main`
`400003539878be3787b9c2b8787c50c0e7004f3c`, merged feature head
`1919b0139db1b51ab26052e16dda0d00cd16fcf0`, their ancestry and merge facts,
and the delta from the source state's recorded
`codex/rlsl-autonomous` head
`1a6d533ff826cc779fec17eeb277d1adf25f29b8`. The source state still carries
the ready `rlsl-lslc-004v-typed-udp-discovery-run-push-bundle`; its eight
historical blockers must remain historical rather than being silently cleared.
No executed-push or publication receipt is tracked in this source tree. The
external workflow must therefore validate its already-published recovery path,
using `unplanned_publication_closure.v1` and `ReconcilePublication` when their
preconditions hold, rather than manufacturing retrospective source
acceptance.
