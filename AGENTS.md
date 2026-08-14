# Rusty LSL Agent Notes

## Current status

Rusty LSL source is integrated on public `main`. The crate remains `0.0.0`,
`publish = false`, default-inert, and neither stable nor released. Source
integration is not activation, compatibility, device, or acceptance evidence.

The production sequence is integrated through the public source surfaces
described by the architecture, compatibility, validation, roadmap, and history
documents. Remaining stability, host-to-Quest qualification, version, tag,
release, and registry decisions stay separately reviewed and unauthorized by
ordinary source work.

## Purpose and authority

Rusty LSL is an independently authored pure-Rust implementation of bounded Lab
Streaming Layer compatibility surfaces. It owns backend-neutral Rust contracts
and explicitly activated candidate runtime behavior for metadata, discovery,
connection and record transfer, timestamps, clocks, buffering, cancellation,
recovery, provider health, and typed observations or proposals.

Rusty LSL does not own Manifold admission, registry revisions, routes, leases,
authorization, provider epochs, or audit; Morphospace-native sample transport;
Quest packaging, permissions, networking policy, or lifecycle; Hostess
orchestration; application or recording policy; or commands derived directly
from inbound samples. Morphospace hooks stop at typed observations and
proposals, and the accepting adapter retains authority.

Project-owned source is licensed `AGPL-3.0-or-later`.

## Read order

1. `README.md`
2. `docs/LSL-PRODUCTION-ROADMAP.md`
3. `docs/ARCHITECTURE.md`
4. `docs/COMPATIBILITY.md`
5. `docs/PROVENANCE.md`
6. `docs/VALIDATION.md`
7. `tools/validation-policy.json`
8. the current unit in the dedicated private planning workspace, when one is
   named by that workspace

The public embedded `morphospace/` tree is immutable historical evidence, not
mutable workflow authority. Do not hand-edit it. Current workflow transitions
belong only to the dedicated private planning repository.

## Implementation rules

- Do not copy or translate liblsl or rLSL source. Official liblsl is an
  MIT-licensed compatibility oracle and reference endpoint, never an
  implementation template. Any diagnostic source inspection must be explicit
  and revision-bound. rLSL source is not an implementation input.
- Keep specification, planned observation, measured result, candidate result,
  and accepted behavior distinct. Do not claim broad compatibility without the
  named process and evidence.
- Start with one `std`-only facade crate. Split protocol, runtime, testkit,
  oracle, or C-ABI crates only when a reviewed ownership boundary requires it.
- Keep `unsafe_code = "forbid"` until a separately reviewed FFI or platform
  adapter demonstrates a need.
- Keep metadata, frames, channel counts, chunks, queues, timeouts, retries, and
  retained ranges explicitly bounded. Constructors validate complete inputs
  before committing accepted state and preserve owned error evidence.
- Keep the bounded Float32 sender buffer and socket-timeout cache session-owned.
  Allocate encoding state before caller-record transfer, preserve a fresh
  per-record deadline and cancellation checks, and keep this finite path
  distinct from the public persistent outlet.
- Keep `PersistentFloat32Outlet` caller-owned and explicitly activated. Allocate
  its fixed channel-shape buffer and bounded consumer registry at construction;
  `push_chunk` must encode once, allocate nothing, and issue one contiguous
  bounded write per retained consumer. Do not introduce an ambient worker,
  timer, discovery/selection policy, implicit retry, or unbounded registry.
- Keep `PersistentFloat32OutletService` caller-polled on a concrete IPv4
  interface: one query/consumer per poll; no worker, enumeration, fallback, retry.
- Runtime selection and activation remain separate. No default Cargo feature
  activates runtime behavior; accepted lock identity and explicit caller input
  remain required, and activation stays default-disabled.
- Discovery is observation, never identity, authorization, selection, or
  activation. Provider fallback is explicit and preserves failed evidence.
- No inbound sample may apply a command directly. High-rate media does not
  belong in the generic LSL sample path.

## Workflow and worktrees

Use one writer per branch and worktree. Account-specific or delegated work uses
a dedicated linked worktree and a `codex/*` branch. The main checkout is the
integration and review surface; do not implement there.

Derive write scope from the current owner-authorized unit. Preserve dirty,
detached, divergent, or unique work until an exact disposition is durably
recorded. Worktree cleanup, local-branch removal, remote-ref retirement,
publication, and release are separate actions; never infer one from another or
sweep unrelated worktrees.

A handoff records the baseline commit, branch, allowed paths, non-scope,
commands and results, unresolved risks, and rollback point. Passing validation
is evidence, not workflow acceptance or publication authority.

## Validation

The sole current validation policy authority is `tools/validation-policy.json`.
Use the repository facade:

```text
python ./tools/dispatch_validation.py --profile quick
python ./tools/dispatch_validation.py --profile standard
python ./tools/dispatch_validation.py --profile deep
```

`tools/check_all.ps1` remains the Standard compatibility wrapper. The Cargo
composition invariant is checked by `python ./tools/check_cargo_shape.py` and
permits only the `public_api` integration-test target. DOC-024 historical
coverage is checked by `python ./tools/check_doc_024_coverage.py`.

Run the non-gating Float32 sender microbenchmark through
`python ./tools/run_float32_sender_benchmark.py`; bind its emitted revision,
dirty state, host, mode, dimensions, warmup, and iterations without a universal claim.

The persistent counterpart, `tools/run_persistent_float32_outlet_benchmark.py`,
measures one `push_chunk` to one established consumer, not discovery,
connection, BLE-to-recorder, or ecosystem latency.

Official-consumer and Polar comparison routes are
`tools/run_persistent_float32_outlet_official_consumer.py` and
`tools/run_polar_stream_sender_ab_benchmark.py`; bind emitted subjects, host,
dimensions, and units, without broad, device, release, or universal claims.

DEVICE-001 physical-H10 evidence is checked by
`python ./tools/check_device_001.py`. Keep device identity, participant data,
raw samples, endpoints, paths, and logs private. Public claims stop at one
Windows H10, the observed 73x1 ECG and 36x3 accelerometer notification shapes,
one exact ECG-frame official-consumer replay, and descriptive sender occupancy.
Do not compose this into a production-replacement, broad liblsl, medical,
cross-platform, LabRecorder, recovery, or licensing claim.

The compact router retains the route keys consumed by focused owner gates:
`LSLC-001A`, `LSLC-001B`, `LSLC-001C`, `LSLC-001D`, `LSLC-001H`,
`LSLC-003O`, `LSLC-003P`, `LSLC-003Q`, `LSLC-003S`, `LSLC-003T`,
`LSLC-003U`, `LSLC-003V`, `LSLC-003W`, `LSLC-003X`, `LSLC-003Y`,
`LSLC-003Z`, `LSLC-004A`, `LSLC-004B`, `LSLC-004C`, `LSLC-004D`,
`LSLC-004E`, `LSLC-004F`, `LSLC-004H`, `LSLC-004J`, `LSLC-004K`,
`LSLC-004M`, `LSLC-004N`, `LSLC-004O`, `LSLC-004P`, `LSLC-004R`,
`LSLC-004S`, `LSLC-004T`, `LSLC-004U`, and `LSLC-004V`; plus
`check_lslc_001e.ps1`, `check_lslc_001f.ps1`, `check_lslc_001g.ps1`,
`check_lslc_001k.ps1`, `check_lslc_001l.ps1`, `check_lslc_001m.ps1`,
`check_lslc_001n.ps1`, `check_lslc_001o.ps1`, `check_lslc_001p.ps1`,
`check_lslc_001q.ps1`, `check_lslc_001r.ps1`, `check_lslc_001s.ps1`,
`check_lslc_001t.ps1`, `check_lslc_001u.ps1`, `check_lslc_001v.ps1`,
`check_lslc_001x.ps1`, `check_lslc_001z.ps1`, and
`check_lslc_002a.ps1`. These are compatibility route keys, not current
chronology or expanded claims; their scopes remain in the canonical documents.

Run focused checks while iterating, then the risk-selected owner aggregate on
the frozen candidate. Docs do not justify device execution. PR CI validates
feature candidates; post-merge `main` validation is independent readback.

Do not change validation policy, workflows, action pins, or the approval trust
root inside a unit that would thereby approve itself. Route such work through
the separately authorized validation-authority process.

## Public and private boundary

Every committed public file must remain portable and free of local paths,
private repository identities, device identifiers or endpoints, credentials,
signing material, raw logs, captures, traces, screenshots, generated
applications, and unsanitized planning evidence. Public evidence may contain
portable schemas, synthetic fixtures, upstream references, exact public Git
identities, and sanitized hash-bound summaries.

Preserve historical or binary bytes unless policy admits conversion; canonical
public text is UTF-8 without BOM, LF-only, and terminal-newline complete.

## Release boundary

The current source is not a release. Release review requires PowerShell 7.6+
and the executing gate in `docs/RELEASE_CANDIDATE.md`; static readiness does
not execute. Do not
change the version, create a tag or release, publish a registry package, or
claim stable API without separate owner authorization and the named evidence.

## Preserved history

Chronological unit notes and the complete pre-DOC-024 first-hop paragraph set
are preserved in [LSLC Work-Unit History](docs/history/LSLC-WORK-UNIT-HISTORY.md).
The [README at P68 integration](docs/history/README-AT-P68-INTEGRATION.md),
[README through accepted LSLC-003L](docs/history/README-THROUGH-LSLC-003L.md),
and [historical validation guide](docs/history/VALIDATION-THROUGH-LSLC-003M.md)
remain immutable scope evidence, not current authority.
