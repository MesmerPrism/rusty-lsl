# Rusty LSL LSLC-001K standard validation

- Unit: `rlsl-lslc-001k-stream-info-static-fields`
- Base revision: `a0db6b9d95f3e1dd621179aa28fbc67cc45640cd`
- Implementation HEAD: `d555b25c21c50ff2e2120ab999e0a954aac1acaa`
- Validation authority: main Codex account
- Secondary MCP role: unrestricted bounded implementation worker, not lifecycle,
  Git integration, acceptance, publication, device, or Manifold authority
- Device validation: forbidden by the unit and not run

## Implemented boundary

- PASS: `StreamInfoStaticFields` contains exactly one borrowed
  `&StreamDefinition`, allocates no owned view state, exposes no mutable access,
  and leaves the source reusable.
- PASS: The exact descriptor-owned role order is `name`, `type`,
  `channel_count`, `channel_format`, `source_id`, `nominal_srate`.
- PASS: Original absent, present-empty, and present-nonempty content-type and
  source-id forms remain observable. Separate effective accessors map only
  absence to empty text without allocation, trimming, normalization, or case
  folding.
- PASS: Original irregular and regular nominal-rate forms remain observable.
  The separate effective numeric view maps only irregular to positive `0.0`
  and otherwise preserves the regular `f64` bits.
- PASS: The seven `ChannelFormat` variants map totally and exactly to
  `float32`, `double64`, `string`, `int32`, `int16`, `int8`, and `int64`.
- PASS: Borrowed name, type, source identifier, definition, and generic
  `MetadataTree` identities are preserved. Metadata gains no `desc` meaning.

## Observation and provenance separation

- PASS: The local-results overlay binds the accepted CORE-008 contract and the
  exact LSLC-001H case and observation artifacts.
- PASS: A direct Rust test executes all seven accepted black-box semantic input
  cases and verifies the same field values recorded by the overlay.
- PASS: The accepted LSLC-001H corpus, case, observation, provenance, raw-capture
  inventory, candidate status, and canonical-LF driver bindings remain
  unchanged. The rolling LSLC-001K checker reuses the full immutable historical
  semantic and provenance validators, not only aggregate file hashes.
- PASS: LSLC-001J's exact current-tree pin remains historical evidence for that
  validation-only unit. It is not incorrectly reapplied after LSLC-001K's
  explicitly authorized Rust source addition.
- PASS: No external implementation source, package, network, oracle, native
  library, or device supplied implementation input.

## Independent main-account review and damaged checks

- PASS: Main review confirmed the public view has exactly one private reference,
  all format match arms are total, effective irregular zero has bits `0`, and
  regular rate bits are unchanged.
- PASS: In-memory damaged checks rejected a changed historical-validator digest,
  a reversed static-role order, and a changed CORE-008 binding at their intended
  boundaries without modifying repository files.
- PASS: Exact scope review found only the 15 declared implementation,
  documentation, overlay, checker, aggregate-route, and unit paths. Cargo files,
  the feature lock, project specification, oracle drivers, LSLC-001H artifacts,
  existing accepted fixtures, and all unrelated repositories remained unchanged.
- PASS: Both PowerShell checker files use complete CRLF materialization as
  required by `.gitattributes`; Rust, Python, Markdown, and JSON surfaces remain
  LF-normalized.
- PASS: Both installed shared skills were reviewed in full. Their current
  adapter, authority, public-firewall, evidence, and validation rules already
  cover this unit and required no edit.

## Authoritative validation gates

- PASS: `powershell -NoProfile -ExecutionPolicy Bypass -File
  ./tools/check_lslc_001k.ps1`
- PASS: `cargo test --workspace --all-targets --offline --locked` — 140 passed,
  0 failed.
- PASS: `powershell -NoProfile -ExecutionPolicy Bypass -File
  ./tools/check_all.ps1` — formatting, all historical focused gates, the rolling
  LSLC-001K gate, public boundary, workspace, dependency/feature, and inert-lock
  checks passed.
- PASS: `pwsh -NoProfile -File
  <work-environment-root>/scripts/Test-WorkflowContracts.ps1 -WorkspaceRoot
  <project-root>/morphospace`
- PASS: `git diff --check`, JSON parsing, exact changed-path scope, no staged
  files, and no unexpected untracked files.

## Preserved implementation and validation setup failures

- PRESERVED: The secondary worker's first aggregate run reached LSLC-001J's
  historical protected-tree gate and rejected LSLC-001K's legitimate source
  addition. The rolling aggregate route now validates all immutable LSLC-001H
  evidence through LSLC-001K while keeping LSLC-001J's acceptance receipt as
  historical proof. No product result was credited from the failed setup.
- PRESERVED: The worker's first replacement gate read a CRLF-materialized
  PowerShell oracle driver. It was corrected to reuse LSLC-001I canonical-LF
  binding without changing the driver or recorded digest.
- PRESERVED: The worker's first scope audit found an edit to
  `fixtures/compatibility/README.md`; that out-of-envelope edit was removed and
  the exact scope audit passed.
- PRESERVED: Main review found that the first overlay version inferred the
  seven-case matrix from component tests rather than directly executing those
  seven Rust cases. A direct matrix test was added before acceptance.
- PRESERVED: Main review found that the first rolling checker bound three
  LSLC-001H hashes but did not re-execute the complete immutable semantic and
  provenance validators. The checker now binds and reuses those validators;
  damaged checks and the full gate passed.
- PRESERVED: One main-review command guessed a nonexistent LSLC-001G local-result
  filename and made no repository change. The correct contract-result file was
  inspected afterward.
- PRESERVED: Patch application left one new LF line in an existing CRLF
  PowerShell file and created the new wrapper with LF lines. A mechanical
  `.gitattributes`-conforming CRLF normalization was applied before validation;
  no script content changed during normalization.

## Evidence limits

- PASS: This evidence proves only the borrowed static semantic-field adapter and
  exact seven-case semantic mapping described above.
- PASS: It does not prove XML validation, representation, tree construction,
  `desc` mapping, serialization, parsing, decoding, numeric lexical formatting,
  complete documents, endpoint bytes, volatile runtime fields, discovery,
  clocks, samples, chunks, buffering, recovery, protocol, wire, I/O, transport,
  FFI, adapters, providers, devices, runtime support, or broad ecosystem
  compatibility.
- PASS: It adds no Manifold admission, registry, subscription, route, lease,
  revision, replay, identity, authorization, audit, command, session, topology,
  transport, or product-policy authority.
- PASS: Public CI is a later integration readback after acceptance and push; it
  is not represented as completed by this validation receipt.
