# Provenance

## Policy

Project-owned source and documentation are licensed `AGPL-3.0-or-later`.
Contributors must author implementation code independently. Do not copy or
translate liblsl or rLSL source, and do not use rLSL source as an
implementation input.

Official liblsl is an MIT-licensed black-box compatibility oracle and reference
endpoint. Its source is not a template. Native libraries, headers, wrappers,
and executables are not production dependencies and are not stored in this
repository by default.

## Artifact classifications

Every fixture, observation, or imported artifact must use one classification:

- `independently-authored`: written from the repository's public behavior
  requirements without copying implementation source;
- `generated`: produced by a named generator from recorded inputs;
- `black-box-observed`: captured from a named public endpoint using recorded
  commands and versions, without inspecting implementation source;
- `adapted`: transformed from a named artifact under a compatible license;
- `copied`: retained verbatim from a named artifact under a compatible license.

`adapted` and `copied` require license and notice review before commit.
Generated and observed material must record whether normalization can discard
semantically relevant details.

## Current inventory

| Paths | Classification | Origin | License |
| --- | --- | --- | --- |
| `crates/**` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `docs/**`, `README.md`, `AGENTS.md` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `tools/**`, Cargo manifests | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `Cargo.lock`, `.gitignore`, `.gitattributes` | generated or independently-authored | Cargo or Rusty LSL contributors | AGPL-3.0-or-later |
| `.github/workflows/**`, `Justfile` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `morphospace/**` | generated then independently reviewed and adapted | Rusty Morphospace workflow and Rusty LSL contributors | AGPL-3.0-or-later |
| `THIRD_PARTY_NOTICES.md` | independently-authored | Rusty LSL contributors | AGPL-3.0-or-later |
| `LICENSE` | copied | Free Software Foundation license text | AGPL-3.0 |

There are currently no compatibility fixtures, captures, oracle outputs,
third-party source files, vendored dependencies, or generated protocol
artifacts.

## Required fixture record

Each future fixture family must have an adjacent machine-readable manifest or
README recording:

- stable fixture identifier and classification;
- author or upstream project and canonical public source;
- creation or retrieval date and exact version;
- generator or observation command and normalized inputs;
- license expression, notice requirement, and reviewer decision;
- content digest for external or generated inputs;
- intended compatibility case and what the artifact does not prove.

Do not commit local filesystem paths, host or device identities, endpoint
addresses from private runs, raw captures, credentials, signing material, or
private planning history.

## Review gate

A dependency or reference artifact may enter only after its task-specific
purpose and license are recorded. A Cargo lockfile proves resolution, not
dependency acceptance. Any material whose origin or redistribution rights are
unclear remains outside the repository until resolved.
