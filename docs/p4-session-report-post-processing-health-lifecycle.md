# Float32 session-report post-processing health lifecycle

## Decision and scope

`Float32SessionReportPostProcessingBatch` is the sole crate-private bounded owner that composes a completed Float32 inlet report with caller-supplied sequence numbers, P33 requested timestamp post-processing, and P33 exact sequence/loss health. The caller supplies exactly one explicit `u64` sequence for each retained report record. The owner preserves report order and returns each retained processed record with its zero-based index, sequence, exact sequence classification, and immutable health snapshot.

Construction rejects a zero maximum. A call rejects an empty report, unequal sequence/record extents, or an extent above the configured maximum before either candidate owner runs. Allocation and every affected counter are checked. Successful post-processing maps deterministically to exactly `RetainedUnchanged` or `RetainedChanged`; there is no `Discarded` state.

## Authority and failure lifecycle

The existing requested-timestamp processor owns Monotonic, DeJitter, and pass-through numerical state. The existing exact-health owner owns caller-sequence classification and exact checked counters. The batch owner clones both bounded states, processes records sequentially, and replaces both live owners only after total success. A post-processing error produces no health observation. Any processing, health, allocation, bound, or counter refusal leaves both live owners unchanged.

Failure evidence is owner-preserving and ordered. Validation errors return every original sequence and record. A record error returns the exact completed prefix, the current index and sequence, the typed P33 error that owns the current record (or current processed record for a health refusal), and untouched sequence/record suffixes. The completed prefix is retained outcome evidence from the failed candidate transaction; it is not committed owner state. No gap, duplicate, or out-of-order classification is converted into estimated packet loss, and no retry, threshold, monitoring, discarding, or automatic policy is introduced.

## Non-scope and equivalence boundary

This lifecycle performs no discovery, socket, queue, recovery, clock acquisition, background work, activation, device, oracle, or Manifold action. Project-owned Monotonic/DeJitter processing and exact-health mapping do not prove liblsl behavioral, numerical, or protocol equivalence. They establish only the bounded project-local semantics described here.

## Focused validation

Because the candidate module is intentionally not declared by `lib.rs`, validation uses an isolated temporary copy of the `rusty-lsl` crate with only a temporary module declaration. Focused unit tests cover ordered allocation retention, exact unchanged/changed mapping, explicit gap and out-of-order classification, zero/empty/equal/upper bounds, middle-record processing failure, health refusal, exact completed/current/remaining evidence, and all-or-nothing owner state. The temporary crate is removed after validation.
