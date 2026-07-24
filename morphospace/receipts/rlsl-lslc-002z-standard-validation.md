# LSLC-002Z Standard validation

Result: pass

- 254 Rust tests passed, including focused valid response correlation,
  malformed and oversized admission, cancellation, deadline, request bounds,
  and immediate port-reuse coverage.
- The pinned official protocol-110 public client resolved the independently
  authored Rust response in a finite IPv4-loopback-only observation.
- Focused provenance/privacy, owner, public-boundary, source-only, and portable
  workflow 0.4.0 gates passed. Device validation was forbidden and not run.
- Raw packets, XML, endpoints, diagnostics, binaries, environments, and caches
  remain private. No official implementation source or Manifold authority was
  used.

Preserved failed attempts:

- Initial focused test compilation passed parser arguments in reverse order;
  the call sites were corrected without changing runtime behavior.
- The first official observation began resolution while the private Rust
  driver was still compiling and resolved nothing; the already-built bounded
  rerun passed unchanged.
- The first public fixture transcribed two private SHA-256 values incorrectly;
  exact local file hashes replaced them before publication.
- The first feature-lock resolution materialized absolute descriptor paths;
  the established portable path normalization and fingerprint readback were
  applied before validation.
