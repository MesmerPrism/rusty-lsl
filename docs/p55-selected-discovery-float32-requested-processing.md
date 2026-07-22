# P55R selected-discovery Float32 requested post-processing

P55R composes one caller-selected discovery index and the existing selected
Float32 session result with the compiled caller-requested report
post-processing owner. Only a canonically completed inlet-session report enters
the existing admission and all-or-nothing timestamp-processing batch. A session
failure bypasses processing unchanged.

The result continues to borrow the completed discovery run and retains the
caller-selected response index while owning the canonical processing outcome.
Its exact committed health is projected by borrowing that outcome. Failures
retain the same discovery/index context and exactly one existing typed session
or processing error; processing admission and record failures therefore keep
their original sequence, report, completed-prefix, current-record, and
untouched-suffix evidence without duplicating record allocations.

The caller still supplies the response index, requested timestamp mode, and
every exact sequence explicitly. The pre-existing processing owner remains the
sole transactional state owner and commits processing and health only after
the entire report succeeds. Failure leaves its prior state available for a
later transaction. Session lifecycle, codec, clock, queue, recovery, buffering,
backpressure, and policy owners are unchanged.

Activation remains explicit and default-disabled. This composition performs no
discovery, selection, retry, sequence inference, automatic processing,
background work, allocation cloning, device work, Makepad integration, or
Manifold action or authority.
