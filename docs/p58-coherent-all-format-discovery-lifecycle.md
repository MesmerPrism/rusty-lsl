# P58 coherent all-format discovery lifecycle facade

P58 lane B adds one product-facing, concrete dispatch surface over the existing
complete caller-named typed UDP discovery lifecycles for Int8, Int16, Int32,
Int64, Double64, and String. The request keeps discovery limits, exact stream
name, expected handshake identity, shape, cancellation, activation, and each
format's bounded I/O/session limits explicit. Empty names and zero dimensions
are rejected before any I/O.

The result and error enums do not erase concrete ownership. Integer successes
retain their existing completed discovery run, receive-order response index,
and canonical report. Double64 and String retain their existing canonical
reports, while the facade result retains the exact caller name. Every error is
the unchanged format-specific complete-lifecycle error, preserving its existing
discovery, selection, connection, transfer, incomplete, completion, close, and
cleanup classification and evidence where that owner supplies it.

`CompleteTypedUdpDiscoveryFormat`, the session request, output, and error are
concrete additive enums. The internal match dispatches directly to one existing
complete lifecycle. A canonical integration can add Float32 as a new parallel
variant after lane A without renaming or reshaping any existing variant.

This facade owns no discovery engine, response ranking, automatic selection,
codec, session lifecycle, allocation, socket, terminal close, cleanup, retry,
recovery, clock, queue, policy, background work, device behavior, Makepad, or
Manifold authority. Activation remains explicit and default-disabled. It adds
no dependency and is intentionally not wired into `lib.rs` or `runtime.rs` in
this path-disjoint lane.

Lane validation uses formatting and workspace checks on the unchanged exported
crate, plus an isolated copied-crate seam that declares this file as a module
and runs its focused `p58_` tests. Canonical facade export and aggregate
qualification belong to the integrator after lane composition.
