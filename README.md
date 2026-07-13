# Rusty LSL

Rusty LSL is an independently authored Rust implementation of Lab Streaming
Layer compatibility. It is designed for the existing LSL ecosystem and for
explicit, typed integration with Rusty Morphospace.

Status: source-only crate with dependency-free bounded metadata and sample-shape
contracts, plus the accepted specification-level STRM-000 baseline. No LSL
protocol, wire, runtime, operational, or ecosystem compatibility is implemented
or claimed. Every historical STRM-000 catalog and damaged-case result remains
`not-implemented`, and no official-liblsl observation has been measured.

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

CORE-001 implements only local Rust contract semantics: validated nonzero
limits, atomic construction, deterministic bound and channel-mismatch errors,
and preservation of accepted caller-provided values. It adds no XML, protocol,
transport, runtime, or authority behavior. The separate STRM-000 baseline
continues to distinguish independently authored specifications, planned
black-box observations, and measured results; only the first exists today. See
[`docs/COMPATIBILITY.md`](docs/COMPATIBILITY.md),
[`docs/ORACLE.md`](docs/ORACLE.md), and the deterministic public fixtures under
[`fixtures/compatibility/`](fixtures/compatibility/README.md).

Development is bounded by the public project-local control surface under
[`morphospace/`](morphospace/README.md). Its lock selects no feature or module
and permits no runtime effect. Workflow state records CORE-001 as active for
this bounded implementation and validation slice. Activity and local unit-test
results are not LSL interoperability or runtime evidence.
