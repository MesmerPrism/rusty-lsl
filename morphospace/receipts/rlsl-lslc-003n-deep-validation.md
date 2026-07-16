# LSLC-003N Deep Validation

Result: pass at source head `7e3faea6625164bc8e0a550a96bf3177c3469f6e`.

- Recovered execution: the Codex restart interrupted the first Deep run; it is excluded from acceptance evidence. No validator, Cargo, Clippy, or Rust compiler process and no owned replay worktree survived. Two untracked generated Python cache files were removed before the complete rerun.
- Complete Deep policy profile: pass in 450.2 seconds; 264 library tests and 2 public API tests pass, with current gates, public-boundary checks, and every pinned historical replay passing.
- Workflow contracts: pass against work-environment release 0.4.0.
- Instruction synchronization: pass; every declared surface is complete or explicitly reviewed with no change.
- Device validation: forbidden and not run.

The incomplete pre-restart run is not reused as evidence. The complete rerun began from the exact clean source commit above. Validation generated only disposable untracked `tools/__pycache__/` files, which were removed after execution.
