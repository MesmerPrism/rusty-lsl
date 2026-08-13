# Registered worktree classification

This source-only record closes the two remaining dirty Rusty LSL historical
overlays at exact public base
`8c6a01a80c5b41c59548b186c7063b937a94abf0`. It records portable identities
and decisions, not local machine paths. Private planning retains the physical
worktree and Git common-directory identities until cleanup is complete.

The pre-cleanup inventory used `git worktree list --porcelain` followed by
`git status --porcelain=v1 --untracked-files=all` in every registered
worktree. It found 297 registered and present worktrees, no missing or prunable
registrations, no status failures, 295 clean worktrees, and exactly the two
dirty overlays below. The six-worktree obsolete cluster is named explicitly;
the other 291 worktrees were clean and are retained outside this closure.

## Final disposition: Int64 discovery overlay

- worktree: `rusty-lsl-parallel-int64-discovery-p30-20260720`
- local branch: `codex/rlsl-parallel-int64-discovery-p30-20260720`
- HEAD: `97cd008f91cd81b95003b9e4d79d24b9f07e0ec1`
- dirty shape: four unstaged tracked Rust edits and no other changed path
- canonical binary/full-index patch: 7,187 bytes, SHA-256
  `d98d4cba1f28c5be7c2d4c7d2c361fa7eb95700d6e983d86d7a1128f6fd4602b`

Working-file SHA-256 values:

- `crates/rusty-lsl/src/lib.rs` —
  `42f299011879b79fc82f73683f3a5c50111edd222d1ebdeef5c574c3c08fa3eb`
- `crates/rusty-lsl/src/runtime.rs` —
  `5528a7b708bc3fd842fb1a126d2cc88e5867396b119e49bfecedb666503debdf`
- `crates/rusty-lsl/src/typed_udp_discovery_integer_session_connection.rs`
  — `dbc1a7771237daa7a2125b3c370b01017b50c5ce079f6754458f69f4de258916`
- `crates/rusty-lsl/tests/public_api.rs` —
  `9b4256cc4c208546dde732e2b7381d2b463e780c8bf4a7689e90f1a9e7563013`

Disposition: rejected as superseded. Adopt no bytes. Accepted repair commit
`99562173362bba103ea7a52faa733a85e4c65d8b` is an ancestor of current public
`main` and supplies the same Int64 selected-discovery behavior with stronger
validation. No runtime bytes adopted from this overlay.

## Final disposition: detached candidate-review overlay

- worktree: `rusty-lsl-candidate-review-v7-on-2c06c764`
- state: detached at `2c06c764520eaac5eb38e5813dffd3971124b15b`
- dirty shape: exactly three untracked Rust integration-test files

Exact file and committed-counterpart bindings:

- `composition_roundtrip_matrix.rs` — SHA-256
  `6a5f71f8795a7e4ddeb04d82fd4da72e1353772c4a5c3de2da04eb9ef9a74308`;
  byte-identical counterpart commit
  `0c79d3ee86a692bc71196ac4b88c05023f759d60` on local branch
  `codex/rlsl-parallel-composition-roundtrip-matrix-v7`
- `cross_family_error_precedence.rs` — SHA-256
  `1116b1ccdbce642f96204ab7501d33b300466bc50dd44b019bf84df550960b5f`;
  byte-identical counterpart commit
  `2bb28938a4d15e30a1a3adebec372aa82c52aacb` on local branch
  `codex/rlsl-parallel-cross-family-error-precedence-v7`
- `unicode_timestamp_interactions.rs` — SHA-256
  `56ed0c71776d7e74e74f420ea74fa9571514ab113b67a48c8d3c69feaa26bf68`;
  byte-identical counterpart commit
  `bdf0464796d6bee8eca75b59ec83f117a362ce91` on local branch
  `codex/rlsl-parallel-unicode-timestamp-interactions-v7`

The prior disposable exact-main diagnostic compiled and ran all eight tests.
DOC-024 independently reproduced that result at exact base `8c6a01a` with the
three admitted SHA-256 identities and temporary explicit test targets: 8/8
passed. The diagnostic worktree was then removed. Current Cargo shape
deliberately permits only the `public_api` integration target, and current
owner-local tests provide stronger coverage. Disposition: reject the three
files as redundant review experiments. Do not add standalone targets and do
not move them into `public_api`. No runtime or test bytes adopted from this
overlay.

## Exact post-merge local cleanup cluster

Only after the public closure merge is independently read back and private
planning has durably committed the evidence seal may local cleanup remove:

1. the dirty Int64 worktree and its attached local branch;
2. the dirty detached candidate-review worktree;
3. the three clean counterpart worktrees and their attached local branches;
4. the stale clean `successor-doc024-lsl-20260807` worktree and attached local
   branch `codex/doc-024-first-hop-chronology-20260807`.

Each target must be resolved as an exact absolute path and verified against
its recorded HEAD, branch or detached state, status, and shared Git
common-directory identity before removal. Use targeted `git worktree remove`
and `git worktree prune`; never sweep all Rusty LSL worktrees. Remove only the
five named local branches after their worktrees are absent. Remote refs are a separate
authority surface: this closure neither deletes nor retires one.

## Remaining worktrees

The 291 non-cluster worktrees were clean at the pre-cleanup observation and
are retained without mutation. Their exact branch or detached tips remain
ordinary historical or operational consumers outside DOC-024. Because the two
dirty overlays and the three byte-identical counterpart experiments now have
final hash-bound dispositions, none of the retained worktrees needs semantic
reconsideration for these artifacts. Private cleanup readback owns the final
registry count, zero-dirty proof, and target-absence evidence.
