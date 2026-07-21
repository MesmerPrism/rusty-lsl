# P5/P6 Float32 report advisory snapshot

The P40 snapshot owner is a crate-private, default-inert, caller-requested
composition of the actual P38 observation-history owner and the actual P39
delta-history and stability-proposal owners. It consumes all three concrete
inputs and returns them intact on success or every typed failure. The snapshot
therefore retains the complete source evidence and every original Float32
sample allocation; it does not clone, normalize, or reconstruct a report.

The caller supplies nonzero bounds for observation windows, deltas, evidence
per delta, stability evidence, and the complete ordered evidence index. Every
size-to-index conversion and evidence-cardinality addition is checked. The
single index allocation uses `try_reserve_exact`; refusal returns all three
inputs unchanged. Ordering is deterministic: observation windows in P38 order,
then each P39 delta and its evidence in retained order, then stability evidence
in its proposal order. Equal, repeated, and changing evidence is never deduped.

The index contains copied, exact P39 evidence values only as addresses into the
snapshot's ordering. The owning P38/P39 values remain canonical and are
available by borrow or consuming extraction. No absence is interpreted as
loss or continuity, and the snapshot makes no behavioral, numerical, wire, or
other liblsl-equivalence claim.

This owner observes only. It cannot apply, accept, route, authorize, activate,
or audit advice and adds no public/root/runtime export, feature activation,
socket, plugin, device behavior, Makepad behavior, or Manifold stream
authority. Focused qualification temporarily declares the private module,
exercises actual-type success and rollback paths, and removes that declaration
after the test run.
