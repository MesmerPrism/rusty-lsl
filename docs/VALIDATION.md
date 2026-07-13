# Validation

## Entry point

Run the full source-only gate from the repository root:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_all.ps1
```

The script runs formatting, locked offline metadata inspection, locked offline
tests, the public-boundary and text-hygiene checker, the dependency-free local
project-workspace checker, and Git whitespace checks.

## Acceptance criteria

The scaffold passes when:

- the crate builds and tests without third-party dependencies;
- unsafe Rust is forbidden;
- the public status remains `ScaffoldOnly`;
- the only package remains unpublished at `crates/rusty-lsl`, exposes no Cargo
  feature, and has exactly one library target;
- repository content passes the tested public-boundary rules;
- the project-local workspace remains well-formed, source-only, and inert;
- every visible source file passes whole-tree trailing-whitespace and terminal
  newline checks, including untracked files before commit;
- `git diff --check` reports no additional Git whitespace errors.

The script rejects any dependency in `cargo metadata`. Build and development
dependencies require a future review and a corresponding gate change rather
than silent addition.

## Evidence limits

A passing source-only gate proves that this revision satisfies the scaffold
checks in the local Rust and PowerShell environment. It does not prove protocol
behavior, wire interoperability, ecosystem compatibility, network behavior,
performance, native-library safety, platform support, or publication readiness.

Future compatibility claims require focused positive and damaged fixtures,
oracle versioning, normalized differential results, and platform details. Live
or external evidence must remain separate from source validation and must name
its cleanup and reproducibility limits.

When the portable Rusty Morphospace work-environment repository is available,
also run its `Test-WorkflowContracts.ps1` against `morphospace/`. The local
checker is a repository gate; it does not replace portable lifecycle or
transition validation.
