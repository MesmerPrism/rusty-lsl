# P54 Selected-Discovery Connection Lifecycle

P54 completes the socket-free selected-discovery preflight stage across every concrete bounded inlet format. Float32 already exposed a resolved owner; Double64, Int64, Int32, Int16, Int8, and String previously combined selected-response validation, session preflight, and TCP connection in one facade.

Each existing concrete inlet owner now provides `preflight_selected_typed_udp_discovery`. The operation performs strict endpoint projection, then the shared format/channel/identity contract, then the format owner's existing bounded preflight. It performs no TCP I/O. The caller may retain the preflighted owner and choose when to call its existing `connect` operation.

The existing `connect_selected_*` and `run_selected_*` facades retain their signatures and delegate through this stage. Caller receive-order selection, expected identity, activation, limits, cancellation, and deadline inputs remain explicit. The existing session owners retain socket, transfer, allocation, completion, terminal close, and cleanup authority.

This slice adds no discovery execution, automatic selection, retry, recovery, identity derivation, new format or shape, generic public strategy, activation change, device behavior, compatibility claim, background work, or Manifold authority. Activation remains explicit and default-disabled.
