# LSLC-003K Standard Validation

Result: pass

Validated revision: `9e9098f911f2377c3af7e7b8ffe50c953ddbd65d`

- `python ./tools/test_lslc_003k.py`: 7 valid/damaged baseline tests passed.
- `powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003k.ps1`: exact pinned Rust 1.80 Clippy identity and 319 library / 350 all-target warning baseline passed.
- `rustup run 1.80.0 cargo test --workspace --all-targets --offline --locked --no-run`: passed.
- `powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1`: passed; 264 library tests, 2 public API tests, all 18 isolated historical replays, durable LSLC-003J live closure, formatting, metadata, public boundary, diff, and workspace checks passed.
- corrected workflow-owner `Test-WorkflowContracts.ps1`: 3 tests passed.
- Exact comparison from LSLC-003J publication through the validated revision shows no v1 manifest or historical checker byte changes.
- No device validation applied.

Preserved failed attempts:

- Initial baseline generation failed before producing evidence because Python inherited Windows CP-1252 for UTF-8 Cargo JSON. The subprocess boundary was corrected to strict UTF-8.
- The first focused integration run added LSLC-003K to the v2 current list and failed the existing LSLC-003J regression assertion. The implementation was narrowed without widening the claim: LSLC-003J remains the sole generic current-manifest checker, CI runs LSLC-003K before the complete unchanged dispatch, and the corrected focused run passed.

This evidence proves pinned lint-baseline stability and repository validation only. It does not accept the warnings as style, clean them up, or change runtime behavior, public API, protocol, activation, dependencies, devices, or authority.
