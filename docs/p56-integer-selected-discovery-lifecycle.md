# P56 integer selected-discovery stream lifecycle

## Decision

P56 extends all four concrete caller-selected integer discovery adapters so the
caller-owned completed discovery run and caller-selected receive-order response
index remain addressable through socket-free resolution, connect,
caller-driven transfer, and canonical completion. Int64, Int32, Int16, and Int8
share one macro-generated ownership shape while retaining concrete public state
and error types. Each completed state contains its unchanged canonical integer
inlet-session report and can return that report without copying.

## Authority

Strict endpoint projection still precedes the selected-response format,
channel-count, and four-role identity contract. Existing socket-free integer
session preflight follows that contract. The existing concrete integer sessions
remain the sole owners of handshake, initialization, codec, cursor, allocation,
cancellation, deadlines, terminal close, socket lifecycle, and cleanup. The new
resolved, connected, and completed states retain only discovery and selection
borrows beside those existing owners.

Transfer failure leaves the connected selected owner available with its exact
successful prefix, discovery run, response index, and report-free close path.
Canonical completion consumes the connected state and retains the same
selection borrows beside the canonical report. Report-free close consumes the
connection and creates no report. The legacy one-shot entrypoints remain thin
delegates that return the unchanged canonical reports.

## Scope

The lifecycle remains restricted to the accepted one-channel/one-record and
two-channel/three-record integer shapes. It adds no generic public strategy,
new format or shape, discovery execution, automatic selection, retry, recovery,
identity derivation, activation, background work, device behavior, Makepad
authority, or Manifold authority. Runtime activation remains explicit and
default-disabled.

## Validation

Focused macro-parity qualification covers selection identity through
resolution, connect, every transfer, and completion for both accepted shapes in
all four formats. Representative Int16 failure and Int8 report-free-close paths
cover retained failure evidence, cleanup, and immediate TCP port reuse.
Formatting and exact two-path diff checks close the slice.
