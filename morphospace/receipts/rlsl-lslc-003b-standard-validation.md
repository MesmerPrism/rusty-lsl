# LSLC-003B Standard validation

Result: pass

- 257 Rust tests pass, including exact four-format value/timestamp preservation,
  format-specific initialization, cross-format rejection, bounds, and cleanup.
- Both pinned-official IPv4-loopback directions pass for `double64`, `int32`,
  `int16`, and `int8` with sanitized outcomes and private raw provenance.
- Focused, owner, public-boundary, source-only, and portable workflow gates pass.
- No `int64`, string, chunks, multicast, devices, official source inspection,
  or Manifold authority was added.

Preserved failures: the first cross-format test expected rejection at record 0
but the shared low byte delayed typed rejection to record 1; the first reverse
integer reruns exposed the observed `Supports-Subnormals: 0` request role,
which was then admitted and emitted explicitly for integer formats.
