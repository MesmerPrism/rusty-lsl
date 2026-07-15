# LSLC-003D Standard Validation

- Validating source head: `279b82df583c0a2ba7b0739c4761daba4083a853`
- Accepted base: `7469c30cf61847e05339ad7f6c6aa0146c9800ef`
- Feature lock: revision 11, fingerprint `a0f12ac8f64eabce3badbdb10d96fa7638d88766716733300629cc40494c3b17`
- Resolver owner correction: `5212bc5674eb8477aad7115ddf17aec857b0975f`

## Passing evidence

- `powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003d.ps1`
  passed the dependency-closed evidence checker, the focused LSLC-003D test,
  all 262 Rust tests, and the public-boundary check.
- `powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1`
  passed all 262 Rust tests plus every owner, source-only, public-boundary,
  instruction, and historical unit gate.
- The corrected resolver self-test passed location-independent fingerprinting
  and rejection of descriptors outside the project workspace.
- The portable workflow contract gate passed against this project workspace.
- All declared instruction surfaces are complete. Device validation is
  forbidden and was not run.

The regenerated lock contains only project-spec-relative forward-slash
`features/*.json` descriptor paths. The LSLC-003D checker rejects absolute,
drive-qualified, backslashed, dot-segmented, traversing, and out-of-project
descriptor references.

## Preserved failed attempts

- The pre-correction resolver emitted absolute descriptor paths; relative
  arguments and a changed working directory did not correct its output.
- An initial resolver invocation defaulted the lock revision to 1 before the
  explicit revision-11 regeneration.
- The first concurrent Standard run caused two Windows UDP loopback tests to
  fail (`WSAECONNRESET` and a damaged-datagram deadline). The exact same owner
  gate passed when rerun sequentially, so no product claim is based on the
  interfered run.

## Limits

This validates dependency-capability composition through the existing runtime
facades. It does not add or prove new runtime behavior, formats, chunks,
adapters, devices, dependencies, official-source intake, or Manifold authority.
