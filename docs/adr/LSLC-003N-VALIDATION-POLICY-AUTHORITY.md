# LSLC-003N: Validation Policy Authority

Status: accepted architecture, implementation pending unit acceptance

Owner: Rusty LSL validation policy

## Decision

`tools/validation-policy.json` is the sole current policy authority. One validating
dispatcher owns profile selection and execution. PowerShell and CI are thin
adapters. Pinned historical gates and receipts are immutable evidence, never
current policy. Human documentation is a compact router plus exact archives.

## Drivers and alternatives

The prior behavior was safe but split among CI, a PowerShell aggregate, a v2
role manifest, and literal prose. Keeping that split would preserve ambiguity;
making receipts authoritative would conflate evidence with policy. A single
versioned policy with separate pinned evidence is the smallest extension of
LSLC-003J's accepted role architecture.

## Consequences

Every gate has a stable ID, owner, command, dependencies, profiles, affected
paths/categories, positive and negative claim bounds, environment, timeout,
and state. Semantic removals require explicit deltas. Unit gates retire to the
historical index after acceptance. CI and local execution cannot maintain
independent inventories.

## Confirmation

`python tools/test_validation_policy.py` exercises malformed, orphaned,
duplicated, stale, bypassed, overlapping, documentation-drift, and archive
damage cases. `python tools/dispatch_validation.py --profile deep` confirms the
full current and pinned behavior.

Review on schema revision or authority-owner change. Supersedes no historical
ADR; it extends LSLC-003J and preserves LSLC-003K/L/M evidence.
