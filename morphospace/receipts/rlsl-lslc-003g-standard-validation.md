# LSLC-003G Standard Validation

- Accepted base: `67818eb02b79209e5c0b7a472ff2b454b8ca2958`
- Validating source head: `2bfb8a2a0c8e819090d2a5914cf9f4ffff9578c5`
- Feature lock remains revision 12, fingerprint
  `39063d95e3269048444ba6aa0fe961b5960429b6d3d0c0bccc00bbe455719319`.

## Passing evidence

- `./tools/check_lslc_003g.ps1` proved the crate-root diff contains only the
  two additive public modules, both modules are re-export-only, runtime source
  and feature lock are unchanged, the external consumer passes, and the public
  boundary remains clean.
- `./tools/check_all.ps1` passed all 264 library tests, both external consumer
  tests, every historical unit gate, source-only checks, and public boundaries.
- The pinned workflow contract gate passed against the validating workspace.
- Rusty Morphospace and system-engineering instruction reviews confirmed the
  facade-only role/plane split preserves single authority and closed activation.
- Device validation is forbidden and was not run.

## Preserved failed attempts

- The first consumer compile assumed a two-argument `SampleLimits` constructor,
  a two-argument `Sample::new`, and a `channels` accessor. The accepted public
  API instead requires `SampleLimits::new(1)`, `Sample::new(limits, 1, values)`,
  and `values()`; the external consumer was corrected to those exact calls.
- The first focused wrapper named a nonexistent singular public-boundary
  script after all Rust tests passed. It was corrected to the owner-provided
  `check_public_boundaries.py` entrypoint.
- The first workflow run required the relevant `system-engineering` review;
  that review was completed and recorded without changing the skill.
- A full owner run rejected the mechanically rewritten unit JSON for a missing
  terminal newline. The newline was restored and the complete owner suite was
  rerun successfully.
- An earlier combined wrapper exceeded 60 seconds after the owner suite while
  reaching the workflow failure above; no acceptance claim uses that run.

## Limits

This proves additive public consumer projections only. It does not change or
prove new implementation behavior, activation, protocol bytes, errors, limits,
cancellation, cleanup, dependencies, devices, adapters, or Manifold authority.
