# LSLC-001M standard validation evidence

- Unit: `rlsl-lslc-001m-static-stream-info-xml-composition`
- Project: `rusty-lsl`
- Branch: `codex/rlsl-autonomous`
- Claimed-base revision: `0cf2f3bc40d515c9e440b79be6f3d32c72e96189`
- Validated revision: `cbd234a9955637f286614b00ebd76f65780c4bfc`
- Validation tier: `standard`
- Device requirement: `forbidden`

## Accepted implementation

The validated tree adds one dependency-free bounded projection from a borrowed
accepted `StreamInfoStaticFields` into one owned `XmlElementTree`. The tree has
exactly seven nodes: root `info`, followed by direct leaves `name`, `type`,
`channel_count`, `channel_format`, `source_id`, and `nominal_srate` in that
order. The projection delegates numeric spelling to LSLC-001L and XML values,
character data, leaf composition, and tree validation to LSLC-001B through E.
The accepted LSLC-001G serializer provides the tested compact explicit-tag
bytes without adding a second serialization policy.

Numeric-domain rejection precedes the exact seven-node arena reserve. Each
fixed name and borrowed static value receives a separate exact fallible copy
before existing XML validation. Typed errors preserve the failing node index
and unchanged delegated error. The source definition, original optional forms,
and generic metadata remain borrowed, unchanged, and reusable.

## Validation results

### Formatting and focused gate

`cargo fmt --all --check` passed through the full owner gate.

`powershell -NoProfile -ExecutionPolicy Bypass -File
./tools/check_lslc_001m.ps1` passed. It bound the immutable H case and
observation artifacts plus accepted G, K, and L overlays; checked source shape,
allocation order, static child order, candidate/document separation,
documentation, protected surfaces, and dependency closure; and executed all
five focused Rust tests.

### Full owner-repository gate

`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1`
passed at the validated revision.

- Rust tests: 148 passed, 0 failed.
- STRM-000: pass.
- LSLC-001A through G, K, L, and M: pass.
- CORE-001 through CORE-008: pass.
- Public-boundary check: pass.
- Project-workspace lifecycle and inert-lock checks: pass.
- Source-only dependency, feature, and publication closure: pass.

### Portable workflow contracts

`powershell -NoProfile -ExecutionPolicy Bypass -File
<work-environment-root>/scripts/Test-WorkflowContracts.ps1 -WorkspaceRoot
<project-root>/morphospace` passed.

## Independent mutation checks

Three temporary, out-of-repository mutations were rejected without changing
repository bytes:

- the fixed node count changed from seven to eight;
- the `type` and `channel_count` compact children were swapped;
- the accepted LSLC-001L overlay digest was replaced.

The first mutation-harness attempt assigned a temporary path into the mapping
returned by `runpy.run_path`; Python function globals were not changed, so the
damaged node bound was incorrectly reported as accepted by the harness. The
harness was corrected to update the validator function's actual `__globals__`
mapping. During this review, the focused checker was also strengthened to bind
compact child order directly. All three corrected mutations then rejected.

## Scope, provenance, and portability audit

- Exactly 19 validation-receipt paths changed from the claimed revision, all
  inside the unit's declared repository envelope. The portable workflow's
  separately validated transaction-ledger files are transition outputs rather
  than unit changed-path claims.
- Cargo manifests and lock, project and feature locks, accepted H/G/K/L
  fixtures, oracle drivers, dependencies, features, and native surfaces are
  unchanged.
- Implementation inputs remain empty; no liblsl, pylsl, rLSL, wrapper,
  application, vendored, generated, or external implementation source was
  inspected or used.
- Reachable unit commits were scanned for workstation paths, account context,
  and the private autonomy document name; none were present.
- Modified PowerShell files contain complete CRLF with no lone LF or carriage
  return; Rust, Python, JSON, and Markdown follow the repository LF policy.
- The worktree was clean at the validated revision.

## Instruction impact

- `AGENTS.md`: updated and validated.
- `README.md` and detailed architecture, compatibility, corpus, provenance,
  and validation routes: updated and validated.
- `system-engineering` skill: reviewed; unchanged because its existing
  artifact-role, closed-activation, and authority guidance covers this slice.
- `rusty-morphospace-context` skill: reviewed; unchanged because its existing
  public-source and Manifold-boundary routing covers this slice.

## Boundary decision and limitations

This unit stops at a compact static `info` element tree. It does not add or
claim the observed XML declaration, inter-element whitespace, self-closing
`desc` form, generic metadata-to-`desc` interpretation, volatile fields,
complete endpoint document, parser, round trip, protocol, wire, networking,
runtime, adapter, provider, device, feature, or external integration behavior.

No Manifold admission, identity, registry, subscription, route, lease,
revision, replay, authorization, audit, command, session, topology, transport,
or product-policy authority was added.

## Verdict

Standard source-only validation passed. LSLC-001M is eligible for explicit
validating, validation-pass, and acceptance transitions.
