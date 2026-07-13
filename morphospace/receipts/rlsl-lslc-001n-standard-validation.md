# LSLC-001N standard validation evidence

- Unit: `rlsl-lslc-001n-description-xml-composition`
- Project: `rusty-lsl`
- Branch: `codex/rlsl-autonomous`
- Claimed-base revision: `fb01ff92cc9f958c2b9b8afe11f17fb105975e82`
- Validated revision: `a5481b483388dfb6f789f80fda1d43fef5593416`
- Validation tier: `standard`
- Device requirement: `forbidden`

## Accepted implementation

The validated tree adds one dependency-free consuming merge of an accepted
LSLC-001M static tree with a separately accepted LSLC-001F element tree whose
root is exactly the container `desc`. The description root becomes direct
`info` child index seven after `nominal_srate`. Every later description parent
is remapped by the fixed seven-node offset. Root admission and checked total
node count precede the target node bound and one exact merged-arena reserve.

All XML component values and allocations move without cloning. Description
hierarchy and order remain exact; LSLC-001F `None` containers and every `Some`,
including empty, leaf remain distinct. Final structural limits delegate to the
existing `XmlElementTree` contract.

## Validation results

The focused command `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001n.ps1` passed. It bound immutable H, F, G, and M
artifacts; checked explicit root admission, node/offset arithmetic, allocation
order, documentation and closure; and executed all five focused Rust tests.

The full owner command `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_all.ps1` passed at the validated revision:

- 153 Rust tests passed, 0 failed;
- STRM-000, LSLC-001A through G, K through N, and CORE-001 through CORE-008 passed;
- public-boundary, project lifecycle, inert lock, dependency, feature, and
  publication checks passed.

The portable workflow command `powershell -NoProfile -ExecutionPolicy Bypass
-File <work-environment-root>/scripts/Test-WorkflowContracts.ps1
-WorkspaceRoot <project-root>/morphospace` passed.

## Independent mutation checks

Temporary out-of-repository mutations changed the fixed static offset from
seven to eight, swapped the nested `first` and `second` description structure,
and replaced the accepted LSLC-001M overlay digest. The focused validators
rejected all three. Review strengthened the checker to bind the exact compact
nested description structure, not only value membership.

## Scope, provenance, and portability audit

- Fifteen implementation/validation paths changed from the claimed revision;
  all are inside the unit envelope.
- Cargo manifests and lock, project and feature locks, accepted H/F/G/M
  fixtures, oracle drivers, dependencies, features, and native surfaces remain
  unchanged.
- Implementation inputs are empty; no external implementation source was
  inspected or used.
- Reachable unit commits contain no workstation path, account context, or
  private autonomy document name.
- Modified PowerShell files contain complete CRLF with no lone line endings;
  Rust, Python, JSON, and Markdown follow LF policy.
- The worktree was clean at the validated revision.

## Instruction impact

`AGENTS.md`, README, architecture, compatibility, corpus, provenance,
validation, and project-workflow routes were updated and validated. The
`system-engineering` and `rusty-morphospace-context` skills were reviewed and
left unchanged because their current fail-closed semantic admission,
public-source, activation, and Manifold authority guidance covers this slice.

## Preserved failure history

After readiness, one combined shell invocation accidentally ran its Git commit
step in the portable workflow repository rather than the Rusty LSL worktree.
That repository was clean and reported nothing to commit; no mutation occurred.
The following claim correctly rejected the still-dirty Rusty LSL readiness
paths. The readiness transition was then committed in the dedicated worktree,
and a fresh clean claim succeeded at the recorded base revision.

## Boundary decision and limitations

The unit admits only a separately accepted container named exactly `desc`; it
does not reinterpret arbitrary generic metadata. Compact empty descriptions use
`<desc></desc>` under unchanged LSLC-001G policy and do not claim the observed
self-closing spelling. No XML declaration, observed whitespace, volatile field,
complete endpoint document, protocol, wire, network, runtime, adapter,
provider, device, feature, external integration, or Manifold authority was
added.

## Verdict

Standard source-only validation passed. LSLC-001N is eligible for explicit
validating, validation-pass, and acceptance transitions.
