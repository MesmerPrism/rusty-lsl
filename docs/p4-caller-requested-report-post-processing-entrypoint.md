# P35 caller-requested Float32 report post-processing entrypoint

`CallerRequestedFloat32ReportPostProcessing` is a crate-private, default-inert production entrypoint. It owns exactly one `Float32SessionReportPostProcessingBatch` and consumes exactly one owned `CallerRequestedFloat32ReportPostProcessingPlan`. The plan already owns its sole completed `TimestampedFloat32InletSessionReport`; no second report can enter this boundary. The entrypoint adds no public export or activation route.

## Admission and transaction boundary

Construction fixes the caller-requested post-processing configuration and maximum record extent in the sole P34 batch owner. For every call, the entrypoint compares the plan's admitted request and maximum with that fixed identity. Either mismatch rejects before the batch is invoked or mutated and returns the complete plan intact, including its request, maximum, exact sequence allocation, report metadata, and retained report-record allocations.

After those checks, the entrypoint consumes the plan in its frozen `(maximum_records, request, sequences, report)` tuple order and delegates the owned sequences and sole owned report exactly once to `Float32SessionReportPostProcessingBatch::process_report`. Success returns the canonical P34 outcome directly. Failure wraps the canonical owner-preserving P34 error without replacing, duplicating, or weakening its completed/current/remaining record and sequence evidence. P34 remains the sole candidate-copy, per-record processing, exact-health, commit, and rollback owner, so a whole report commits once only after total success.

The admission owner remains responsible for validating its request, nonzero maximum, retained extent, exact sequence count, and binding of the one report before a plan exists. This entrypoint adds only the persistent-owner identity check. It performs no unrelated-report comparison, inferred loss calculation, retry, threshold, monitoring, automatic policy, discovery, socket, clock acquisition, queue action, recovery, background work, or activation.

## Authority and equivalence boundary

This project-local composition does not establish behavioral, numerical, wire, or protocol equivalence with liblsl. It grants no device authority, Makepad authority, or Manifold stream authority. The caller continues to own the explicit request and admitted evidence; the existing report and P34 owners retain lifecycle, allocation, processing, health, and transaction authority.

## Focused isolated validation

The canonical integrated tree is validated directly with only these four focused commands:

```text
cargo test -p rusty-lsl caller_requested_float32_report_post_processing_admission::tests --lib
cargo test -p rusty-lsl caller_requested_float32_report_post_processing::tests --lib
cargo test -p rusty-lsl float32_session_report_post_processing_batch::tests --lib
cargo test -p rusty-lsl --test public_api
```

The focused private tests exercise real admitted plans and real completed loopback reports across the admission-to-entrypoint boundary. They cover exact caller sequence-allocation retention, intact plan/report return on request or maximum mismatch, success, first/middle/final P34 failure partitions, retained record allocations, actual fallible outcome-storage and candidate-copy refusals, rollback followed by success, repeated reports, `u64` extremes, and the crate-private default-inert boundary. P34 failure suffixes retain the already-owned sequence and record vector allocations with their exact cursors; constructing a record failure performs no allocating collection and makes no claim about an allocation failure that cannot be induced at a genuine fallible seam.
