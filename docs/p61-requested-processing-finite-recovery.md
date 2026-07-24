# P61 requested-processing finite-recovery observation

P61 adds a public, stable, bounded CPU/data-only contract for a canonical
integrator to observe finite recovery before composing a recovered sample with
P60 caller-requested timestamp post-processing. The source module is not wired
through the crate root in this milestone.

`RequestedPostProcessingRecoveryObserver` stores only fixed-size exact counters
and the last accepted facts. Its configuration supplies a nonzero observation
limit, a nonzero per-observation attempt limit, and a cumulative explicit
missing-sequence limit. A zero missing-sequence limit intentionally forbids
positive gap evidence. Observation performs no allocation.

Each input names exactly one existing recovery disposition: recovered on a
one-based successful attempt, exhausted after exactly the configured attempt budget,
or cancelled after an exact completed-attempt count. Zero completed attempts is
valid only for cancellation observed before the first attempt. Optional exact
sequence evidence may accompany only a recovered sample. It uses the same
first, contiguous, positive gap, duplicate, and positive out-of-order-distance
vocabulary as the existing exact sequence-loss owner; P61 neither derives nor
reclassifies those facts.

Validation is transactional. Zero required attempts, premature exhaustion,
attempt or observation limit violations, sequence evidence without a recovered sample, zero-sized gap
or out-of-order claims, cumulative missing-sequence limit violations, and
counter representation failures return typed errors without changing any
counter or last-observation fact.

This contract does not own or initiate recovery, cancellation, transport,
sessions, discovery, clocks, timestamp processing, queues, activation, retry
policy, threads, devices, or Manifold behavior. In particular, it does not
claim that a sequence gap is packet loss, invent sequence evidence for an
exhausted/cancelled acquisition, apply P60 processing, or advance P60 state.
The canonical integrator remains responsible for passing only facts produced by
their existing owners and for invoking requested post-processing only after a
recovered sample exists.
