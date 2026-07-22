# P54 caller-requested Float32 report post-processing pipeline

P54 composes one caller-owned completed Float32 inlet-session report, one
caller-requested timestamp post-processing mode, and one exact caller sequence
per retained record through the existing admission and transactional batch
owners. The production entrypoint admits the canonical report against the
processing owner's existing nonzero record bound and then delegates the
validation-complete plan to the existing all-or-nothing batch lifecycle.

Admission failures retain the unchanged request, sequence allocation, and
canonical report. Owner-identity mismatch retains the admitted plan. Record
processing failures retain the existing completed prefix, current record
failure, and untouched suffix evidence, while the processing and exact-health
owners commit only after total batch success. Successful processing preserves
record order and transfers each report record allocation into its existing
processed outcome.

The caller continues to select the post-processing request and provide every
sequence number explicitly. The composition neither estimates packet loss nor
infers sequences, thresholds, retry, buffering, or recovery policy. It adds no
session, discovery, clock-provider, activation, background-work, device,
oracle, compatibility, command, or Manifold authority.
