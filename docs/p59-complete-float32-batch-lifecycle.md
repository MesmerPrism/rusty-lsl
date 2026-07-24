# P59 complete Float32 batch lifecycle

This slice adds one caller-explicit typed composition from bounded UDP discovery through exact
caller-named receive-order selection, canonical bounded Float32 inlet completion, and the existing
actual-report-extent recovery/clock/queue batch owner. Success owns the unchanged completed
discovery run, exact selected response index, canonical batch outcome, and its borrowed exact health.

## Evidence and errors

Discovery, selection, selected-response validation, connection, transfer, terminal close, and
cleanup delegate to the existing complete Float32 lifecycle. A failure at any of those stages is
returned as its existing typed complete-lifecycle error, including completed discovery and selected
index evidence wherever that owner has it. Only a canonically completed inlet report reaches the
batch owner.

A batch failure owns the unchanged completed discovery run, exact selected index, and existing
owner-preserving batch error. Its health accessor borrows the batch error and therefore neither
moves nor duplicates completed-prefix, current-record, untouched-suffix, recovery-state, sample,
or allocation evidence. Success health likewise borrows the canonical batch outcome.

## Authority

The caller supplies the discovery destination and bounds, query, exact stream name, discovery and
session cancellation, response/admission limits, expected identity, handshake/sample/session
limits, bounded shape, recovery activation and policy, clock activation/configuration/provider,
queue and wait policy, and the three batch cancellation inputs. Every activation remains explicit
and default-disabled.

This facade is orchestration only. Existing discovery, exact-name selection, endpoint/contract,
session lifecycle, codec, allocation, terminal-close, cleanup, recovery, clock, queue, batch, and
health owners remain sole authorities. The facade adds no retry, fallback, rediscovery, automatic
or ambiguous selection, background work, dependency, new format or shape, device/ADB, Makepad, or
Manifold authority.

## Qualification

Focused P59 loopback tests cover complete multi-stage success with retained discovery/selection,
exact queued Float32 evidence, and complete health. A typed refusal case covers exact-name no-match,
retained completed discovery evidence, no TCP/session or batch work, and immediate UDP cleanup and
port reuse. The tests use only finite caller-bounded IPv4 loopback seams.
