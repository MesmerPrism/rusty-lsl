# Rusty LSL

Rusty LSL is an independently authored Rust implementation of Lab Streaming
Layer compatibility. It is designed for the existing LSL ecosystem and for
explicit, typed integration with Rusty Morphospace.

Status: repository scaffold only. No protocol, runtime, wire, or ecosystem
compatibility is implemented or claimed yet.

The architecture keeps LSL interoperability at a data-plane edge:

- Rusty LSL owns LSL-compatible metadata, discovery, samples, clocks, recovery,
  and backend-neutral Rust APIs.
- Rusty Manifold remains the authority for accepted Morphospace streams,
  subscriptions, routes, leases, revisions, and audit.
- Platform and operator adapters remain outside this repository.
- Deeper Morphospace hooks emit observations and proposals; they do not bypass
  the owning authority.

Project-owned public source is licensed `AGPL-3.0-or-later`. Official liblsl
is an MIT-licensed interoperability oracle, not a source template. rLSL source
is not an implementation input.

The first accepted milestone is a reproducible compatibility and provenance
baseline. Runtime implementation begins only after that gate is reviewed.

Development is bounded by the public project-local control surface under
[`morphospace/`](morphospace/README.md). Its initial lock selects no feature or
module and permits no runtime effect. The first planned compatibility-baseline
unit is proposed only; it must pass the portable readiness transition before
implementation begins.
