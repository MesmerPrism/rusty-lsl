# P59 complete Float32 requested post-processing lifecycle

This candidate composes the exact caller-named bounded UDP discovery and canonical Float32 inlet
session from P58 with the existing transactional caller-requested report post-processing owner. A
successful value owns the unchanged completed discovery run, exact receive-order selected index,
and canonical committed processing outcome. Its health projection is borrowed directly from that
outcome.

The composition preserves stage boundaries. Discovery, exact-name selection, selected-response
validation, connection, phased transfer, terminal close, and cleanup failures remain the existing
typed lifecycle error and retain the caller's unconsumed request and exact sequence allocation. If
the session completes, admission or processing refusal retains the completed discovery and selected
index alongside the existing owner-preserving processing error. Thus sequence/report mismatch and
record-level refusal keep their original report allocation, completed prefix, current evidence, and
untouched suffix, while the processing owner's previously committed health remains unchanged.

The caller supplies the requested processing mode and every exact sequence; no sequence or loss fact
is inferred. The P58 lifecycle remains the sole discovery, selection, session, transfer, terminal
close, and cleanup owner. The existing post-processing owner remains the sole admission,
timestamp-processing, sequence-health, and transaction-commit owner. The new type only orders those
two consuming delegations.

Focused finite-loopback qualification covers successful discovery, exact selection, Float32 bit and
sequence preservation, committed health, terminal cleanup with immediate port reuse, and a typed
sequence-extent refusal that retains completed discovery/report evidence and rolls back processing
health.

All activation remains explicit and default-disabled. This candidate adds no automatic selection,
inference, retry, fallback, rediscovery, background work, dependency, device/ADB behavior, Makepad
integration, Manifold action, or authority.
