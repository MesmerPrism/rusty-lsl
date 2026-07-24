# LSLC-003H Standard Validation

- Accepted base: `40e164440c17fe96e916e49038662227bb474cd0`
- Validating source head: `4b1fab1e96d294059d9820c68427a5fe6bb24df4`
- Feature lock remains revision 12 with fingerprint
  `39063d95e3269048444ba6aa0fe961b5960429b6d3d0c0bccc00bbe455719319`.

## Passing evidence

- The audit found that `tools/check_all.ps1` manually selected only ten focused
  gates and silently omitted accepted current-safe gates LSLC-002V through
  LSLC-002Z and LSLC-003A through LSLC-003B.
- `tools/current-gates.json` now declares the exact ordered current set from
  LSLC-002Q through LSLC-003H. The dispatcher preflights the entire manifest,
  rejects malformed, missing, duplicated, absolute, or traversing paths, and
  stops at the first failing checker.
- Focused dispatcher tests proved exact ordering, missing/duplicate/traversal
  rejection, and first-failure termination without rewriting historical gates.
- The complete dispatcher ran all 18 declared direct checkers successfully.
- `tools/check_all.ps1` passed with the dispatcher as its sole focused-gate
  route, and CI continues to use that accepted aggregate entrypoint.
- The pinned owner workflow contracts and public-boundary gates passed.
- Instruction review confirmed a validation-only authority artifact: feature
  lock, runtime implementation, protocol behavior, dependencies, devices, and
  Manifold authority are unchanged.

## Preserved failed attempts

- The first `BeginValidation` invocation supplied the repository root instead
  of its `morphospace` project-workspace directory. Owner automation rejected
  it before changing lifecycle state because `project.spec.json` was absent at
  that root. The corrected invocation used the accepted workspace directory.
- One combined aggregate run produced more console output than the session
  could retain. Its result was not used as evidence; the aggregate was rerun
  with compact output and an explicit successful exit check.
- During correction recovery, one compact aggregate invocation returned a
  transient nonzero result with output suppressed after the focused gate had
  passed. The unchanged aggregate was immediately rerun with diagnostic output
  and completed successfully; the failed invocation is not acceptance evidence.

## Limits

This validates only deterministic, fail-closed dispatch of the explicitly
declared current accepted checker set. Every historical direct checker remains
unchanged. It adds no Clippy policy, documentation compaction, runtime breadth,
dependency, device, official-source, or Manifold behavior.
