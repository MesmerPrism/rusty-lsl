# P57 Double64 and String complete discovery lifecycle

## Decision

P57 lane B adds one concrete production composition per format. Each starts one
existing bounded typed UDP discovery call, asks the existing selection owner for
the first receive-order response whose stream name exactly equals the caller's
nonempty name, and passes that index through the existing selected-response
preflight and connection. The adapter explicitly advances the existing phased
transfer owner to its declared extent and consumes canonical completion.

## Preserved authority

Discovery activation, configuration, query, cancellation, envelope limits, and
typed admission limits remain caller inputs. Empty names and no matches stop
with distinct typed errors; there is no ambiguous fallback or automatic policy.

Session activation, cancellation, handshake identity and limits, I/O limits,
and bounded shape remain caller inputs. Double64 retains exact bit framing and
only its accepted 1x1 and 2x3 shapes. String retains its exact 1x1 shape and
0 through 129 UTF-8-byte envelope. Existing owners retain endpoint projection,
contract validation, codec, sockets, cursor, successful-only progress,
allocation, terminal close, cleanup, and canonical reports.

## Non-scope

This composition adds no retry, recovery, background work, codec or wire change,
new format or shape, activation default, device or platform work, Makepad,
public-main action, or Manifold authority.

## Qualification

Owned-module tests cover bounded discovery through exact-name selection and
canonical session completion for Double64 and String, exact Double64 value bits,
the String UTF-8 envelope, typed selection and lifecycle failures, cleanup, and
immediate TCP port reuse. Deterministic socket ownership coordinates loopback
peers without sleeps.
