# P58 Float32 complete discovery lifecycle

This slice adds one caller-explicit composition that runs bounded typed UDP discovery, asks the
existing exact-name selector for the first receive-order match, validates that response as Float32,
connects through the existing selected-response phased inlet, advances the requested bounded record
extent, and consumes canonical completion. Success owns the unchanged discovery run, exact selected
index, and canonical session report.

## Authority

`run_typed_udp_discovery_float32_session_inlet` is orchestration only. Existing owners remain sole
authorities for discovery bounds and termination, exact-name selection, response parsing and
validation, endpoint projection, format/channel/identity contract checks, activation, preflight,
handshake, Float32 framing, allocation, the successful-only cursor, terminal close, and socket
cleanup. The composition explicitly closes the connected owner after a phased transfer failure;
other consuming completion paths retain their existing cleanup behavior.

Typed stage failures preserve the completed discovery run whenever discovery completed. Selection,
no-match, connection, transfer, incomplete-completion, and session failures remain distinct; every
post-selection failure also preserves the exact receive-order index. Discovery failure retains the
existing discovery error because no completed run exists at that stage.

## Qualification

Focused loopback tests cover canonical success with exact Float32 bits and discovery/selection
identity, exact-name no-match with retained discovery evidence, session cancellation with retained
selection identity, transfer failure, explicit close, cleanup, and immediate TCP port reuse. The
tests use finite IPv4 loopback sockets only.

## Limitations

Selection is exact, caller-named, receive-order based, and never automatic or ambiguous. The caller
supplies all discovery/session limits, identities, cancellations, shapes, and explicit capability
activations. This slice adds no retry, fallback, rediscovery, recovery, policy, background work,
unbounded networking, other format, new shape, codec, session engine, device behavior, Makepad,
Manifold, or default activation authority. It does not claim official LSL compatibility.
