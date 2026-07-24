# Float32 session-report post-processing health lifecycle

## Decision and scope

`Float32SessionReportRequestedPostProcessing` is the sole reusable crate-private per-record owner of P33 requested timestamp post-processing and P33 exact sequence/loss health. `Float32SessionReportPostProcessingBatch` contains exactly one such record owner and adds only a bounded, whole-report transaction. The caller supplies exactly one explicit `u64` sequence for each retained report record. The batch preserves report order and returns each retained processed record with its zero-based index, sequence, exact sequence classification, and immutable health snapshot.

Construction rejects a zero maximum. A call rejects an empty report, unequal sequence/record extents, or an extent above the configured maximum before either candidate owner runs. Allocation and every affected counter are checked. Successful post-processing maps deterministically to exactly `RetainedUnchanged` or `RetainedChanged`; there is no `Discarded` state.

## Authority and failure lifecycle

The requested-timestamp processor owns Monotonic, DeJitter, and pass-through numerical state. Its ordinary `Clone` path is intentionally absent. Candidate creation starts with an empty history vector, fallibly reserves the full configured history bound, and then copies retained `f64` values, so later candidate pushes cannot allocate. Allocation refusal is typed and retains the current sample. The exact-health owner owns caller-sequence classification and exact checked counters.

The record owner provides the sole processing/health composition. The batch fallibly copies that record owner once, processes records sequentially through it, and replaces the one live record owner only after total success. A post-processing error produces no health observation. Any processing, health, allocation, or bound refusal leaves live state unchanged. Loop position is derived from the already bounded completed prefix; there is no impossible terminal counter-overflow branch or debug-only completion claim.

Failure evidence is owner-preserving and ordered. Validation errors return every original sequence and record. A record error returns the exact completed prefix, the current index and sequence, the typed P33 error that owns the current record (or current processed record for a health refusal), and untouched sequence/record suffixes. The completed prefix is retained outcome evidence from the failed candidate transaction; it is not committed owner state. No gap, duplicate, or out-of-order classification is converted into estimated packet loss, and no retry, threshold, monitoring, discarding, or automatic policy is introduced.

## Non-scope and equivalence boundary

This lifecycle performs no discovery, socket, queue, recovery, clock acquisition, background work, activation, device, oracle, or Manifold action. Project-owned Monotonic/DeJitter processing and exact-health mapping do not prove liblsl behavioral, numerical, or protocol equivalence. They establish only the bounded project-local semantics described here.

## Focused validation

Both private owners are declared by the committed crate `lib.rs`. Canonical focused validation covers typed fallible-copy refusal, reserved-capacity behavior, ordered allocation retention, exact unchanged/changed mapping, `u64::MAX`, maximum gap, duplicate and out-of-order classification, validation bounds, processing and health refusals, exact completed/current/remaining evidence, repeated use, and all-or-nothing single-commit state. The `public_api` target confirms that no public export was added.
