# P60 Public Complete Requested-Processing Evidence

Lane B replaces the P59 crate-scoped complete Float32 requested-processing
boundary with public projections suitable for an external caller. The
transactional post-processing owner, its candidate state, exact-health owner,
and allocation-bearing internal outcome and refusal types remain private.

The public completion value retains the completed bounded discovery run and
the exact receive-order selected response index. It exposes ordered record
evidence by index: the caller sequence, retained public `TimestampedSample`,
exact sequence classification, and the health state committed at that record.
Its terminal health projection exposes exact observation, relationship,
explicit-missing, retained-changed/unchanged, and high-water counters without
exposing the private health snapshot type.

The public error value exposes a stable lifecycle-versus-processing stage and
a public processing classification. Lifecycle failure access returns the
existing public typed lifecycle error and retains the unchanged requested mode
and caller sequence allocation. Processing failure access retains and exposes
the completed discovery run, exact selected index, and exact retained sequence
and report extents while the allocation-owning refusal stays private.

Construction maps the private owner configuration errors to a public bounded
configuration error. Requested mode/config types are expected from P60 lane A.
No discovery selection, retry, lifecycle, processing policy, allocation,
activation, device, Makepad, or Manifold authority moves into this facade.

Focused validation from exact base
`5c2e897d3f962bc4f2753ed96a4f84f42d422236`:

```text
cargo test -p rusty-lsl complete_typed_udp_discovery_float32_requested_post_processing_lifecycle --lib
2 passed; 0 failed
```

The success case proves public discovery/selection retention, exact Float32
bits, caller sequence order, record classification, and all terminal health
facts. The refusal case proves the public processing stage/classification,
completed discovery and selected-index retention, exact sequence/report
extents, unchanged record allocation evidence, and unchanged prior committed
health. The loopback fixture also confirms terminal cleanup by immediate TCP
port reuse.
