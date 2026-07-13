# Rusty LSL Agent Notes

Rusty LSL is a public Rusty Morphospace repository for an independently
authored Rust implementation of Lab Streaming Layer compatibility. Keep every
committed file portable, public-safe, and free of private paths, product names,
device identities, raw captures, credentials, signing material, or local
planning history.

Project-owned source is licensed `AGPL-3.0-or-later`.

## Purpose

Rusty LSL owns:

- backend-neutral Rust APIs for LSL-compatible metadata, discovery, samples,
  clocks, buffering, cancellation, recovery, and provider health;
- independently authored LSL protocol and runtime behavior;
- compatibility fixtures and differential evidence against official liblsl;
- observation and proposal hooks that deeper Rusty Morphospace adapters can
  consume.

Rusty LSL does not own:

- Manifold stream admission, registry revisions, subscriptions, routes, leases,
  provider epochs, authorization, or audit;
- Morphospace-native sample transport or generic stream authority;
- Quest networking, permissions, packaging, Android lifecycle, or device
  resources;
- Hostess orchestration, application policy, recording policy, or runtime
  defaults;
- commands derived directly from inbound LSL samples.

Morphospace hooks stop at typed observations and proposals. The accepting
adapter and authority remain in their owning repositories.

## Read Order

1. `README.md`
2. `docs/ARCHITECTURE.md`
3. `docs/COMPATIBILITY.md`
4. `docs/PROVENANCE.md`
5. `docs/VALIDATION.md`

## Provenance And Compatibility

- Do not copy or translate liblsl or rLSL source.
- Do not use rLSL source as an implementation input.
- Official liblsl is an MIT-licensed compatibility oracle and reference
  endpoint, not a source template.
- Record every fixture or observation as independently authored, generated,
  black-box observed, adapted, or copied. Copied material requires an explicit
  license and notice review.
- Do not claim clean-room implementation, wire compatibility, ecosystem
  compatibility, or runtime support without the named process and evidence.
- Keep official native libraries and wrappers outside the default production
  dependency closure.
- The initial repository is inert and source-only. Repository presence must not
  change any runtime, package, permission, network, or feature default.

## Architecture Rules

- Start with one `std`-only facade crate. Split protocol, runtime, testkit,
  oracle, or C-ABI crates only when a reviewed ownership boundary requires it.
- Keep `unsafe_code = "forbid"` until a separately reviewed FFI or platform
  adapter demonstrates a need.
- Keep metadata, frames, channel counts, chunks, queues, timeouts, retries, and
  retained ranges explicitly bounded.
- Preserve raw source timestamps. Corrected and smoothed timestamps are derived
  views.
- Discovery is observation, never identity, authorization, or activation.
- No inbound sample may apply a command directly.
- No high-rate media belongs in the generic LSL sample path.
- Provider fallback is explicit and preserves the failed candidate evidence.

## Worktree And Agent Policy

Use one writer per branch and worktree. Account-specific or delegated work must
use a dedicated linked worktree and a `codex/*` branch. The main checkout is
the integration and review surface; delegated agents must not write there.

A handoff records the baseline commit, branch, allowed paths, non-scope,
commands run, results, unresolved risks, and rollback point.

## Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

Until that entrypoint exists on the scaffold branch, run `git diff --check`
and inspect every committed path for public-boundary leakage.
