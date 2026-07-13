# LSLC-001Q standard validation evidence

- Unit: `rlsl-lslc-001q-ordered-stream-info-element-composition`
- Project: `rusty-lsl`
- Branch: `codex/rlsl-autonomous`
- Claimed-base revision: `ab58c3bb99e805ce6ca1f1a5f86ddce1a0e3b0ea`
- Validated revision: `bc74fb8c5e9ac94fca48178532d0290e60b9974a`
- Validation tier: `standard`
- Device requirement: `forbidden`

## Accepted implementation

The validated source consumes one accepted LSLC-001N static-plus-description
tree and one accepted LSLC-001P volatile tree. It validates both fixed shapes,
checks the root-sharing total and target node bound before one exact merged
reserve, retains the static `info` root and six leaves, moves all eleven
volatile leaves, then moves `desc` and its descendants. Only description-
internal parent indexes receive the fixed eleven-node offset. Names and
represented character data move without cloning.

## Validation results

`./tools/check_lslc_001q.ps1` passed, binding accepted H/N/P artifacts and
receipts and executing four focused Rust tests, including all seven accepted
case shapes. The final full `./tools/check_all.ps1` owner gate passed:

- 166 Rust tests passed, 0 failed;
- STRM-000, LSLC-001A through G, K through Q, and CORE-001 through CORE-008
  passed, including the historical Rust 1.80 gates;
- public-boundary, lifecycle, inert-lock, dependency, feature, publication,
  formatting, and whitespace checks passed.

The portable workflow contract command also passed at the validated revision.
An earlier full attempt passed the default 166-test suite but the Rust 1.80
historical gate rejected the newer `Option::is_none_or` convenience method.
Commit `bc74fb8c5e9ac94fca48178532d0290e60b9974a` replaced it with equivalent
Rust-1.80-compatible matching; the pinned focused test and complete owner gate
then passed.

## Scope, provenance, instruction, and portability audit

- Twenty implementation, validation, instruction, unit, receipt, and lifecycle
  paths comprise the validation set; transaction ledgers are separately
  preserved and excluded by workflow policy.
- Cargo/project/feature locks, accepted evidence and oracle drivers,
  dependencies, features, native surfaces, and implementation inputs remain
  unchanged or empty.
- Public status prose now routes accepted Q behavior. Both shared skills were
  reviewed and required no change because their current artifact-role and
  repo-family boundaries already cover the composition.
- Committed content is public-safe and portable; no device operation occurred.

## Boundary decision

The result is only a compact local ordered `info` element tree. It adds no XML
declaration, observed whitespace or self-closing policy, endpoint document,
parser, provider, clock or host inspection, identity generation, address/port
ownership semantics, networking, protocol, transport, runtime, adapter,
device, feature, external integration, or Manifold authority behavior.

## Verdict

Standard source-only validation passed. LSLC-001Q is eligible for explicit
validation-pass and acceptance transitions.
