# Rusty LSL

Rusty LSL is an independently authored Rust implementation of Lab Streaming
Layer compatibility. It is designed for the existing LSL ecosystem and for
explicit, typed integration with Rusty Morphospace.

Status: source-only scaffold with a specification-level STRM-000 compatibility
baseline. No protocol, runtime, wire, or ecosystem compatibility is implemented
or claimed. Every catalog and damaged-case result remains `not-implemented`,
and no official-liblsl observation has been measured.

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

The current milestone defines a reproducible compatibility and provenance
baseline before runtime work. It separates independently authored behavior
specifications, planned black-box observations, and measured results; only the
first exists today. See [`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md),
[`docs/ORACLE.md`](docs/ORACLE.md), and the deterministic public fixtures under
[`fixtures/compatibility/`](fixtures/compatibility/README.md).

Development is bounded by the public project-local control surface under
[`morphospace/`](morphospace/README.md). Its lock selects no feature or module
and permits no runtime effect. Workflow state records STRM-000 as active for
this documentation-and-validation slice; activity is not compatibility or
runtime evidence.
