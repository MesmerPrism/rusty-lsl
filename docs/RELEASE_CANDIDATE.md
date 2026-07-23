# Release-candidate readiness

RLSL-P8 defines a deterministic feature-branch readiness gate, not a release
operation. From a clean candidate commit, run:

```powershell
pwsh -NoProfile -File ./tools/Test-ReleaseCandidateReadiness.ps1
```

PowerShell 7.6 or newer is required. The script resolves the repository from
its own tracked location and uses only tracked repository state plus Git's
identity and cleanliness reports. It performs no network, device, build,
package, registry, branch-changing, or publication operation.

## Evidence checked

The gate fails closed unless `HEAD` is a one-parent commit on an isolated
`codex/*` feature branch with a clean worktree. It requires tracked current
workspace and crate manifests, lockfile, validation policy and dispatcher,
capability overview, compatibility status, architecture, production roadmap,
P6 host qualification, validation guide, supply-chain boundary, this document,
and the gate itself.

The manifest checks preserve the `rusty-lsl` package and repository identity,
workspace-owned `0.0.0` version, `publish = false`, feature-free crate shape,
and absence of a default feature. The policy checks preserve its declared
schema and sole-authority identity, a positive revision, current unique gates,
the quick/standard/deep/CI profiles, closed profile membership, and tracked
repository-local command paths. A small in-memory self-test proves positive
manifest handling and rejection of damaged version, publication, default
activation, and missing-document-evidence cases.

Current documentation must continue to state the coherent public lifecycle
architecture, bounded compatibility claims, explicit default-disabled
activation, P6 qualification boundary, policy-owned validation facade, and P8
roadmap boundary. The successful output binds the result to exact branch,
commit, tree, and validation-policy identity.

## Limitations

This is a static readiness prerequisite. It does not run Standard, Quick, Rust
tests, host qualification, Quest qualification, private or official oracles,
or interoperability tests. It does not prove that a future integration merge
is conflict-free, that unpublished APIs are stable, that a release version is
correct, or that registry credentials and release infrastructure are ready.
Existing validation and qualification evidence retains its own scope and
limitations.

## Remaining specifically authorized boundary

A passing feature-branch candidate is reviewable evidence only. It does not
integrate public main and does not version, tag, release, or publish anything.
Those actions remain a separate, specifically authorized public-main
integration and release boundary. The authorized integrator must separately
review the candidate, run the validation profile appropriate to that boundary,
choose and approve a version, and explicitly authorize any merge, tag, release,
or registry publication.
