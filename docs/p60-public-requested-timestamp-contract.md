# P60 public caller-requested timestamp contract

P60 makes the existing caller-requested timestamp mode and its immutable
configuration and result facts usable as public value types. It does not expose
the processor: construction, retained history, candidate copies, processing,
and persistence remain owned by the crate-private
`RequestedTimestampPostProcessor`.

## Explicit request

Callers choose exactly one mode with
`RequestedTimestampPostProcessing::pass_through()`, `::monotonic(config)`, or
`::de_jitter(config)`. `kind()` reports that choice and `configuration()`
returns the exact supplied configuration for a stateful mode. Pass-through has
no configuration. No mode is inferred from timestamps, stream properties, or
prior results, and P60 adds no default activation.

`RequestedTimestampPostProcessingConfig::new(history_samples, minimum_step,
maximum_adjustment)` preserves the existing validation contract:

- `history_samples` is in the inclusive range 2 through 4096;
- `minimum_step` is finite and greater than zero; and
- `maximum_adjustment` is finite and greater than or equal to zero.

The configuration accessors return those exact values. Invalid values retain
their existing typed `RequestedTimestampPostProcessingConfigError`, including
the exact accepted bounds or rejected floating-point bits. Allocation failure
remains typed with the exact requested capacity. P60 does not change when or
how the bounded history allocation is acquired.

## Immutable facts

`RequestedTimestampPostProcessingFacts` exposes the existing mode kind, input
timestamp, effective timestamp and source, adjustment, non-discarding
disposition, retained history length, and state-advanced flag through immutable
accessors. `RequestedEffectiveTimestamp` exposes its value and exact source.
The only successful dispositions remain `RetainedUnchanged` and
`RetainedChanged`; neither means that a sample was discarded or reallocated.

Pass-through preserves the existing derived timestamp classification when one
exists and otherwise reports the raw source. Stateful modes report
`ProjectPostProcessed` on success. These facts neither select a mode nor grant
processing authority.

## Preserved boundary

P60 changes no monotonic or de-jitter arithmetic, history bound, adjustment
limit, ordering rule, validation precedence, allocation, sample ownership, or
failure behavior. The algorithms remain independently authored project
candidates and are not evidence of behavioral, numerical, or protocol
equivalence with liblsl. They acquire no clock, infer no loss or policy, start
no background work, and add no device, Makepad, or Manifold authority.
