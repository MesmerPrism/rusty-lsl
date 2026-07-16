# LSLC-004D standard validation

- Focused wrapper: pass; exact production-prefix hash matched, six damaged
  fixtures rejected, one synthetic joined-loopback query/response passed, and
  the public boundary passed.
- Standard profile: pass; 275 library tests and two public API tests passed,
  with current policy, documentation, instruction, public-boundary, and diff
  gates complete.
- Fixed portable workflow examples: pass.
- Project-workspace workflow replay: preserved failure with 27 pre-existing
  vocabulary/schema errors (feature-lock revision drift plus historical
  `activation`, `compatibility`, `documentation`, and `compatibility` profile
  vocabulary). The added LSLC-004D unit contributes the same already-known
  `compatibility` and `documentation` vocabulary mismatch; this is not a
  runtime, public-boundary, or unit-scope defect.
- Devices: forbidden and not used.
- Production prefix through the first `#[cfg(test)]` marker remained identical
  to accepted LSLC-004C revision `b8407facecbc1e24e7759dafed52c9af90d8e4bc`
  with SHA-256 `41c2393adb561669aa824e3635ae70e85b3b412892a38100fc835d3bfc8b4263`.
