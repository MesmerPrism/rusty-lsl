# Rusty LSL

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
