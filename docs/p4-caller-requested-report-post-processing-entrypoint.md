# P35 caller-requested Float32 report post-processing entrypoint

`CallerRequestedFloat32ReportPostProcessing` is a crate-private, default-inert production entrypoint. It owns exactly one `Float32SessionReportPostProcessingBatch` and accepts only an owned `CallerRequestedFloat32ReportPostProcessingAdmission` plan plus one completed `TimestampedFloat32InletSessionReport`. It adds no public export or activation route.

## Admission and transaction boundary

Construction fixes the caller-requested post-processing configuration and maximum record extent in the sole P34 batch owner. For every call, the entrypoint first compares the admitted request and maximum with that fixed identity, then compares the admitted exact record count with the report's actual `record_count()`. Either mismatch rejects before the batch is invoked or mutated and returns the complete admission plan and report as typed evidence.

After those checks, the entrypoint consumes the admission into its explicit sequence allocation and delegates exactly once to `Float32SessionReportPostProcessingBatch::process_report`. Success returns the canonical P34 outcome directly. Failure wraps the canonical owner-preserving P34 error without replacing, duplicating, or weakening its completed/current/remaining record and sequence evidence. P34 remains the sole candidate-copy, per-record processing, exact-health, commit, and rollback owner, so a whole report commits once only after total success.

The admission owner remains responsible for validating its request, nonzero maximum, retained extent, and exact sequence count before a plan exists. This entrypoint adds only identity and report-binding checks. It performs no inferred loss calculation, retry, threshold, monitoring, automatic policy, discovery, socket, clock acquisition, queue action, recovery, background work, or activation.

## Authority and equivalence boundary

This project-local composition does not establish behavioral, numerical, wire, or protocol equivalence with liblsl. It grants no device authority, Makepad authority, or Manifold stream authority. The caller continues to own the explicit request and admitted evidence; the existing report and P34 owners retain lifecycle, allocation, processing, health, and transaction authority.

## Focused isolated validation

The candidate is validated from an isolated temporary copy because its admission owner and crate module declarations arrive from a sibling lane. Validation supplies only a temporary admission stub matching the frozen `request()`, `maximum_records()`, `record_count()`, `sequences()`, and `into_parts()` interface, temporary module declarations, and a test-only report constructor. Focused tests cover request/maximum mismatch without mutation, report-extent mismatch without mutation, successful ordered delegation, and failing delegation with exact P34 evidence and unchanged live state. Temporary resources are removed after the run.
