# P56 Double64 and String selected-discovery lifecycle

## Decision

P56 lane B adds concrete selection-retaining phased owners for caller-selected
Double64 and String discovery connections. The connected states borrow the
completed discovery run and retain its receive-order response index throughout
caller-driven transfer. Canonical completion consumes the connected state and
retains those same selection borrows beside the unchanged concrete session
report.

## Authority

Strict endpoint projection and the existing selected-response contract still
precede socket-free preflight and TCP work. The existing Double64 and String
connected inlet sessions remain the sole owners of handshake, initialization,
cursor, allocation, transfer, cancellation, deadlines, terminal close, and
cleanup. Double64 keeps its sealed numeric codec and accepted shapes. String
keeps its exact one-channel, one-record shape and 0..=129 UTF-8-byte envelope.

Typed transfer failure leaves the selection-retaining connected owner
available with its successful prefix and report-free close path. Report-free
close consumes the connection and manufactures no completion report. Existing
one-shot functions remain delegates and return the canonical concrete reports.

## Non-scope

This slice adds no generic public strategy, discovery execution, automatic
selection, retry, recovery, policy, new format or shape, activation, device,
Makepad, Manifold, or background-work authority.

## Validation

Focused unit qualification covers retained discovery identity and response
index, phased success, canonical allocation-preserving completion, typed
transfer-failure evidence, report-free close, cleanup, and immediate TCP port
reuse for both concrete formats.
