# Rusty LSL

LSLC-003S registers a distinct `StringSample` capability in the closed feature
lock. It is selected-but-run-disabled and capability-only: no String transport
or runtime effect exists until a later reviewed runtime consumes exact nominal
admission plus the existing handshake dependency.

LSLC-003Q records observation-only protocol-110 String framing for one bounded
13-byte value, one channel, and one caller record in two repeated loopback runs.
It is evidence for a later bounded candidate, not a String implementation,
activation, arbitrary String support, or broad compatibility.

LSLC-003P implements a selected-but-run-disabled bounded sequence runtime for
exactly two homogeneous channels and three ordered records across double64,
int32, int16, and int8. It is finite IPv4-loopback candidate behavior bound to
LSLC-003O evidence, not activation, arbitrary-count support, non-loopback
support, broad ecosystem compatibility, or authority.

Rusty LSL is an independently authored Rust implementation of bounded Lab
Streaming Layer compatibility surfaces. Project-owned source is licensed
AGPL-3.0-or-later. Official liblsl is used only as a pinned black-box
compatibility oracle, never as an implementation template.

## Current capability surface

The `rusty-lsl` crate provides dependency-light bounded contracts for stream
descriptors, metadata trees, XML values and observed stream-info documents,
timestamps, homogeneous samples and chunks, and descriptor/sample binding.
These local contracts do not by themselves perform discovery, networking,
clock correction, buffering, recovery, protocol exchange, or runtime
activation.

The closed project lock selects eight candidate capability families:

- bounded sample queue;
- finite sample recovery;
- fixed-width numeric sample transport;
- integrated clock correction;
- short-info discovery responder;
- stream handshake;
- timestamped Float32 sample transport;
- UDP discovery.

Every selected capability remains disabled by default. Runtime effects require
the accepted lock plus an explicit descriptor-approved caller input and an
effective activation receipt. Selection is not activation, compatibility,
endpoint authority, discovery identity, authorization, or Manifold stream
authority. Supported claims and their evidence limits are defined by the
documents below, not by this summary.

LSLC-003O adds observation-only evidence for two-channel, three-record
`double64`, `int32`, `int16`, and `int8` sequences in two bounded pinned-
official IPv4-loopback directions. It adds no production implementation,
activation, broad interoperability, or authority.

## Authoritative project documents

- [Agent instructions](AGENTS.md) — ownership, public-safety, architecture,
  worktree, and validation rules.
- [Architecture](docs/ARCHITECTURE.md) — component boundaries and authority.
- [Compatibility](docs/COMPATIBILITY.md) — supported and unresolved
  interoperability claims.
- [Provenance](docs/PROVENANCE.md) — independent authorship and evidence
  classification.
- [Validation](docs/VALIDATION.md) — current commands and what they prove.
- [Validation policy](tools/validation-policy.json) — the sole current gate,
  profile, claim, and limitation authority used by local and CI dispatch.
- [Corpus](docs/CORPUS.md) and [oracle policy](docs/ORACLE.md) — public
  documentation inputs and black-box observation discipline.
- [Licensing](docs/LICENSING.md) and [supply chain](docs/SUPPLY_CHAIN.md) —
  source/dependency review boundaries.
- [Project workflow](morphospace/README.md) — bounded work-unit lifecycle and
  inert-by-default composition state.
- [Compatibility fixtures](fixtures/compatibility/README.md) — public-safe
  deterministic fixture routing.

## Preserved history

- [LSLC work-unit history](docs/history/LSLC-WORK-UNIT-HISTORY.md) preserves
  the chronological unit notes formerly carried by `AGENTS.md`.
- [README through accepted LSLC-003L](docs/history/README-THROUGH-LSLC-003L.md)
  preserves the complete prior README byte-for-byte as historical evidence.

Historical descriptions and passing local tests are not current runtime or
interoperability claims. Consult the authoritative documents and accepted
receipts for the exact scope of each claim.

LSLC-003T provides the closed capability-gated one-channel, one-record bounded
String loopback runtime observed by LSLC-003Q. It remains run-disabled unless
the caller presents the selected LSLC-003S `StringSample` capability.

LSLC-003U records sanitized pinned black-box evidence for mixed-width UTF-8 and
the exact 127-byte boundary. Private drivers, raw records, endpoints,
diagnostics, and version-drift runs remain outside the repository.

LSLC-003V confirms the unchanged LSLC-003T runtime handles both LSLC-003U
value classes in synthetic finite loopback; it changes tests and evidence only.

LSLC-003W records sanitized pinned black-box evidence for exactly one empty
String caller record in both finite IPv4-loopback directions. It changes no
runtime or activation; private drivers, raw records, endpoints, diagnostics,
environment, and machine identity remain outside the repository.

LSLC-003X extends the existing capability-gated one-channel, one-record String
runtime only to the LSLC-003W-observed empty value. The 127-byte maximum,
one-byte length form, finite loopback behavior, and activation closure remain.

LSLC-003Y records sanitized pinned black-box evidence that one independently
authored exact-128-byte String uses length form one with length 128 in both
finite loopback directions. It changes no runtime or activation.

LSLC-004B extends the closed capability-gated String runtime only through that
observed exact 129-byte boundary. Existing empty and nonempty cases remain;
130 bytes reject, and activation, framing form, finite loopback, and authority
do not widen.

LSLC-004C records sanitized single-platform evidence for one documented IPv4
multicast discovery group on an explicit loopback interface, including finite
query/response directions and membership cleanup. It changes no runtime and
does not generalize interfaces, retry policy, platforms, or authority.

LSLC-003Z extends the closed capability-gated String runtime only through that
observed exact 128-byte boundary. Existing empty and nonempty cases remain;
129 bytes reject, and activation, framing form, finite loopback, and authority
do not widen.

LSLC-004A records sanitized pinned black-box evidence that one independently
authored exact-129-byte String uses length form one with length 129 in both
finite loopback directions. It changes no runtime or activation.
