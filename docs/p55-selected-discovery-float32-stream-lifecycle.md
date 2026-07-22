# P55R selected-discovery Float32 stream lifecycle

## Decision

P55R P3 extends the compiled concrete selected-discovery Float32 connection
owner so the caller-owned completed discovery run and caller-selected
receive-order response index remain addressable after connect, during phased
transfer, and after canonical completion. The completed state contains the
unchanged canonical inlet-session report and can return it without copying.

## Authority

Strict endpoint projection still precedes the selected-response format,
channel-count, and four-role identity contract. Existing socket-free session
preflight follows that contract. The existing Float32 connected session remains
the sole owner of handshake, initialization, codec, cursor, allocation,
cancellation, deadlines, terminal close, and cleanup. The new concrete states
only retain discovery and selection borrows and delegate lifecycle operations.

Transfer failure leaves the connected selected owner available, including its
discovery run, response index, successful prefix, and report-free close path.
Completion consumes the connected state and retains the same selection borrows
beside the canonical report. Report-free close consumes the connection and
creates no report.

## Non-scope

This slice adds no generic public strategy, discovery execution, automatic
selection, retry or recovery policy, identity derivation, format or shape,
activation, background work, device behavior, Makepad authority, or Manifold
authority. Runtime activation remains explicit and default-disabled.

## Validation

Focused unit qualification covers selection identity across connect, each
caller-driven transfer, canonical completion, transfer-failure evidence,
report-free close, cleanup, and immediate TCP port reuse. Formatting and an
exact two-path diff check close the slice.
