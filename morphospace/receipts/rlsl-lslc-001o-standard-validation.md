# LSLC-001O standard validation evidence

- Unit: `rlsl-lslc-001o-volatile-stream-info-data-contract`
- Project: `rusty-lsl`
- Branch: `codex/rlsl-autonomous`
- Claimed-base revision: `4cae984e7b06f99f4a9dad7e93dfc4488cf62fbe`
- Validated revision: `5013ba9fcf53b4f085b788a683d173fef9f5aff3`
- Validation tier: `standard`
- Device requirement: `forbidden`

## Accepted implementation

The validated source adds one dependency-free bounded accepted-data contract
for the eleven volatile stream-info roles observed by LSLC-001H. Fixed role
order is `version`, `created_at`, `uid`, `session_id`, `hostname`,
`v4address`, `v4data_port`, `v4service_port`, `v6address`, `v6data_port`, and
`v6service_port`.

The contract separates implementation-assigned version data, runtime-assigned
creation/identity/session/host data, and transport-owned address/port data.
Three nonzero maxima count Unicode scalar values by class. Limits reject in
class order and field values reject in fixed role order. Accepted state retains
only the limits and original eleven caller-owned `String` allocations.

## Validation results

The focused command `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001o.ps1` passed. It hash-bound the accepted LSLC-001A
corpus, LSLC-001H case/observation/provenance artifacts, and LSLC-001N receipt;
verified the exact role order and class mapping; validated the independently
authored overlay and protected closure; and executed all five focused Rust
tests.

The full owner command `powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_all.ps1` passed at the validated revision:

- 158 Rust tests passed, 0 failed;
- STRM-000, LSLC-001A through G, K through O, and CORE-001 through CORE-008
  passed;
- public-boundary, project lifecycle, inert lock, dependency, feature,
  publication, formatting, and whitespace checks passed.

The portable workflow command `powershell -NoProfile -ExecutionPolicy Bypass
-File <work-environment-root>/scripts/Test-WorkflowContracts.ps1
-WorkspaceRoot <project-root>/morphospace` passed.

## Scope, provenance, and portability audit

- Twenty implementation, validation, instruction, unit, receipt, and lifecycle paths changed from
  the claimed revision; all are inside the declared unit envelope.
- Cargo manifests and lock, project and feature locks, accepted A/H/N evidence,
  oracle drivers, dependencies, features, and native surfaces remain unchanged.
- Implementation inputs are empty; no external implementation source was
  inspected or used.
- Reachable unit commits contain no workstation path, account context, device
  identity, raw capture, credential, or private autonomy context.
- PowerShell files use complete CRLF; Rust, Python, JSON, and Markdown use LF.
- The worktree was clean at the validated revision.

## Instruction impact

`AGENTS.md`, README, architecture, compatibility, corpus, provenance,
validation, fixture routing, and project-workflow routing were updated and
validated. The stale README statement that no official observation existed and
the obsolete active-LSLC-001H workflow label were reconciled inside this unit.
The `system-engineering` and `rusty-morphospace-context` skills were reviewed
and left unchanged because their existing data/provider/representation,
public-source, activation, and Manifold authority rules cover this slice.

## Preserved failure history

The first full owner-gate attempt passed all 158 Rust tests, then STRM-000
rejected a line-wrapped compatibility disclaimer because its immutable
validator binds the exact accepted sentence. The README retained the corrected
LSLC-001H through O status while restoring that disclaimer verbatim. The
focused STRM-000 and LSLC-001O gates then passed, followed by a clean full owner
gate. The full gate and workflow contracts were rerun at the exact
validation-pass revision after its lifecycle transition was committed.

During focused-validator development, the first field-order lookup used a
nonexistent generic key instead of LSLC-001H's accepted
`direct_info_child_order`; the checker failed closed before validation. The
lookup was corrected to the exact accepted artifact shape before the validated
revision.

## Boundary decision and limitations

All eleven values remain opaque. The contract does not claim they were acquired
from a provider or are current, generated, unique, numeric, parsed, reachable,
or operational. It adds no XML legality or representation, document
composition, clock or host inspection, identity generation, address or port
semantics, network or transport behavior, discovery, protocol, wire, runtime,
adapter, provider, device, feature, external integration, or Manifold
authority.

## Verdict

Standard source-only validation passed. LSLC-001O is eligible for explicit
validation-pass and acceptance transitions.
