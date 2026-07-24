# Validation Authority Inventory

Inventory baseline: `68db7ceabac67a72040c99272742c292bc420c`.

| Surface | Prior role | Classification | Migration |
|---|---|---|---|
| `tools/check_all.ps1` formatting, metadata shape, all tests, v2 dispatch, public boundary, diff hygiene | local aggregate and CI child | executed; partly duplicated by CI | preserved as stable policy gates in `standard` and `ci` |
| `.github/workflows/ci.yml` Rust 1.80 setup, LSLC-003K, aggregate | CI inventory | executed; conflicting ownership, mechanically safe union | CI now selects only policy profile `ci` |
| `tools/current-gates-v2.json` and dispatcher | 18 pinned gates plus LSLC-003J live gate | executed by aggregate; declared current/historical | retained as immutable replay index and current-closure implementation |
| `tools/current-gates.json` | original 18-entry inventory | historical-only | immutable ordered prefix |
| LSLC-003J | generic live descriptor/lock/activation/workspace closure | executed current | stable `current-closure` gate |
| LSLC-003K | pinned Rust 1.80 Clippy baseline | executed only in CI; declared focused | stable `pinned-rust-180-clippy` gate in `ci`/`deep` |
| LSLC-003L and LSLC-003M | exact AGENTS/README archives and compact routers | accepted claims; focused gates declared but not aggregate-executed | durable archive/router invariants lifted to `documentation-router`; receipts stay historical |
| `docs/VALIDATION.md` focused LSLC/CORE/STRM recipes | live prose consumer | declared; predominantly historical-only; duplicated across AGENTS and receipts | exact file archived; current file is a compact policy router |
| AGENTS focused commands | first-hop prose | declared; historical and current mixed | retained during initial migration; future unit impact routes to policy |
| accepted iteration units and receipts | acceptance criteria and execution evidence | declared and historical evidence | immutable; never policy authority |
| project-workflow validator | external workflow authority invoked by receipts | declared per unit, not owner aggregate | remains acceptance evidence; policy does not claim ownership of external workflow code |

No criterion or limitation was removed. The only prior execution split was CI's
extra LSLC-003K invocation; the stronger union is now explicit in `ci` and
`deep`. Historical focused commands remain replayable at their pins. No
unresolved semantic conflict was found during inventory. Migration execution
then exposed one latent accepted-history conflict: an immutable LSLC-003M
receipt lacks a terminal newline. The policy records the exact resolution as a
semantic delta: immutable receipts retain complete content scanning and exact
bytes, while only their mutable-text newline rule is suppressed when Git proves
the working bytes equal `HEAD`.

Future units must classify validation impact as `none` with a specific
justification, `implementation-only`, or `policy`. Accepted unit-specific gates
become pinned evidence; any durable invariant is promoted under an owner-named
stable gate ID. Changes to profile membership, claims, limitations, or state
require a machine-readable `semantic_deltas` entry and an ADR review. The
private pre-claim recovery preserved an earlier invalid top-level
`validation/` proposal; the workflow-authorized implementation uses the
existing `tools/` envelope and reintroduced those authored bytes with an exact
recorded path mapping.
