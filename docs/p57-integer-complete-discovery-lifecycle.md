# P57 integer complete discovery lifecycle

## Decision

P57 adds one concrete production composition per integer width: Int8, Int16,
Int32, and Int64. Each entry point starts with the existing bounded
`run_typed_udp_discovery`, asks the existing selection owner for the first
receive-order response whose stream name exactly equals the caller's non-empty
name, and passes that index through the existing concrete selected-response
integer inlet to canonical completion.

The successful result owns the completed discovery run, the selected
receive-order index, and the unchanged canonical integer inlet-session report.
Borrowed accessors keep all three addressable, while `into_parts` recovers them
without copying their retained allocations.

## Error and evidence ownership

The concrete per-width errors distinguish typed-discovery failure, invalid
empty selection input, no exact match, and the existing selected-response or
session error. Empty-name and no-match errors retain the completed discovery
run. Consequently a pre-I/O discovery cancellation remains visible as the
existing `Cancelled` discovery termination with an empty response collection;
it is not converted into retry, fallback, or an implicit selection.

Selected-response errors continue to preserve endpoint, format, channel-count,
four-field identity, preflight, handshake, transfer, terminal-close, deadline,
cancellation, and cleanup classifications through the existing concrete error
owners.

## Authority

The composition introduces no discovery, selection, codec, connection,
lifecycle, allocation, or cleanup engine. The existing owners remain ordered:

1. caller-supplied explicit discovery activation, configuration, query,
   cancellation, envelope limits, and admission limits;
2. allocation-free exact non-empty stream-name suggestion in receive order;
3. strict selected endpoint and format/channel/identity projection;
4. existing socket-free concrete integer session preflight;
5. existing connect, phased transfer, terminal close, and canonical completion.

Session activation, expected identity, handshake and I/O limits, accepted
channel/record shape, and session cancellation remain explicit caller inputs.
Activation remains default-disabled.

## Scope

The lifecycle remains limited to the already accepted one-channel/one-record
and two-channel/three-record shapes for Int8, Int16, Int32, and Int64. It adds
no ambiguous or automatic selection, retry, recovery, background work, codec or
wire behavior, device/platform work, Makepad action, public-main action, or
Manifold authority.

## Qualification

Module-local loopback qualification covers complete discovery through exact
selection and canonical session reports for all four integer widths using the
two-channel/three-record shape. It verifies exact ordered values, retained
discovery and response index, consuming result decomposition, outlet completion,
cleanup, and immediate TCP port reuse. Additional cases cover typed empty-name,
no-match, and pre-I/O discovery cancellation evidence. Existing focused module
tests continue to cover both accepted shapes, contract and endpoint damage,
phased transfer cancellation, report-free close, cleanup, and immediate reuse.
