# P6 local host qualification

`tools/Invoke-P6HostQualification.ps1` runs the frozen P66 host lifecycle qualification as one focused Rust integration test. It is default-inert: execution requires an explicit script invocation and the Rust transaction explicitly admits every runtime capability it uses.

The transaction uses only local IPv4 loopback and existing public Rusty LSL owners. It performs one bounded UDP discovery and caller-selected receive-order response, an exact two-record Float32 outlet/inlet session, exact timestamp and value-bit checks, bounded recovery, integrated clock correction, bounded queue admission, terminal close, and immediate TCP address reuse.

Run from any directory with PowerShell 7 or newer:

```powershell
pwsh -NoProfile -File ./tools/Invoke-P6HostQualification.ps1 -OutputDirectory ./artifacts/p6-host
```

The route fails closed unless the worktree is clean. It binds the result to the unchanged Git commit and tree before and after the focused test, sets `PYTHONDONTWRITEBYTECODE=1`, and writes the sole versioned JSON receipt to `p6-host-qualification-v1.json` beneath the caller-supplied output directory. The output directory should be outside the worktree (or already ignored), because creating an untracked output inside the worktree correctly violates the cleanliness requirement.

This is reusable local-host lifecycle evidence. It does not establish official runtime or oracle equivalence, device behavior, cross-network behavior, automatic selection/retry/policy, or Manifold authority.
