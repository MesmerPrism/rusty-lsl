# P35 caller-requested Float32 report post-processing entrypoint

`CallerRequestedFloat32ReportPostProcessing` is a crate-private, default-inert production entrypoint. It owns exactly one `Float32SessionReportPostProcessingBatch` and consumes exactly one owned `CallerRequestedFloat32ReportPostProcessingPlan`. The plan already owns its sole completed `TimestampedFloat32InletSessionReport`; no second report can enter this boundary. The entrypoint adds no public export or activation route.

## Admission and transaction boundary

Construction fixes the caller-requested post-processing configuration and maximum record extent in the sole P34 batch owner. For every call, the entrypoint compares the plan's admitted request and maximum with that fixed identity. Either mismatch rejects before the batch is invoked or mutated and returns the complete plan intact, including its request, maximum, exact sequence allocation, report metadata, and retained report-record allocations.

After those checks, the entrypoint consumes the plan in its frozen `(maximum_records, request, sequences, report)` tuple order and delegates the owned sequences and sole owned report exactly once to `Float32SessionReportPostProcessingBatch::process_report`. Success returns the canonical P34 outcome directly. Failure wraps the canonical owner-preserving P34 error without replacing, duplicating, or weakening its completed/current/remaining record and sequence evidence. P34 remains the sole candidate-copy, per-record processing, exact-health, commit, and rollback owner, so a whole report commits once only after total success.

The admission owner remains responsible for validating its request, nonzero maximum, retained extent, exact sequence count, and binding of the one report before a plan exists. This entrypoint adds only the persistent-owner identity check. It performs no unrelated-report comparison, inferred loss calculation, retry, threshold, monitoring, automatic policy, discovery, socket, clock acquisition, queue action, recovery, background work, or activation.

## Authority and equivalence boundary

This project-local composition does not establish behavioral, numerical, wire, or protocol equivalence with liblsl. It grants no device authority, Makepad authority, or Manifold stream authority. The caller continues to own the explicit request and admitted evidence; the existing report and P34 owners retain lifecycle, allocation, processing, health, and transaction authority.

## Focused isolated validation

The candidate is validated from an isolated temporary copy because its admission owner and crate module declarations arrive from a sibling lane. Validation copies the actual sibling admission candidate, adds only temporary module declarations and a test-only report constructor, and exercises the real `CallerRequestedFloat32ReportPostProcessingPlan` interface. Focused tests cover intact plan return on request and maximum mismatch, successful ordered single-report delegation, and failing P34 delegation with exact evidence and unchanged live state. Temporary resources are removed after the run.
