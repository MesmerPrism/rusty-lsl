# LSLC-003A Standard validation

Result: pass (bounded compatibility observation)

- Four official-outlet directions (`double64`, `int32`, `int16`, `int8`)
  preserve marker, record width, initialization timestamp, caller sample
  timestamp, and caller value. Initialization values are format-specific.
- All five reverse directions reject the Float32-derived initialization pattern
  at the official test-pattern check. The pinned public binding reports its
  `int64` outlet surface unavailable on this platform.
- 254 Rust tests plus focused provenance/privacy, owner, public-boundary,
  source-only, and portable workflow gates passed. No device work was run.
- Raw artifacts and exact harness details remain private; no official
  implementation source or Manifold authority was used.

Preserved failed attempts:

- The first private classifier compared initialization timestamps to the
  caller sample timestamp and initialization values to Float32's `4/2` pair;
  the classifier was corrected to distinguish the observed fixed
  initialization timestamp and format-specific values before rerunning.
