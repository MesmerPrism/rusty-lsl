# Validation

LSLC-003S focused activation validation runs
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003s.ps1`.
It validates the exact descriptor source binding, resolver-owned relative
paths, canonical lock fingerprint/revision, workspace registry, nominal
capability, dependency closure, absence-is-inert behavior, damaged fixture
inventory, all LSLC-003C preservation checks, Rust tests, and public boundary.
It proves no String transport, I/O, framing, runtime execution, device behavior,
ambient activation, or authority.

LSLC-003Q focused validation runs
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003q.ps1`.
The policy-owned `lslc-003q-observation` gate checks exact sanitized bounds,
framing, hashes, nonclaims, damaged fixture mutations, required routes, and the
public boundary. It does not rerun the private oracle or prove implementation,
activation, damaged-peer behavior, arbitrary Strings, devices, or authority.

LSLC-003P focused validation runs
`powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003p.ps1`.
The policy-owned `lslc-003p-runtime` gate proves only the closed two-channel,
three-record local runtime contract, not activation, arbitrary counts,
non-loopback behavior, devices, or broad compatibility.

The sole current validation-policy authority is
[`tools/validation-policy.json`](../tools/validation-policy.json). Run its portable facade:

```text
python ./tools/dispatch_validation.py --profile quick
python ./tools/dispatch_validation.py --profile standard
python ./tools/dispatch_validation.py --profile deep
```

`tools/check_all.ps1` is the compatibility wrapper for `standard`; CI invokes
the policy-owned `ci` profile directly. Gates declare stable IDs, owners,
dependencies, change scope, claims, limitations, environment, and timeouts.
Receipts record executions but never select policy.

LSLC-003O observation evidence is checked through the policy-owned
`lslc-003o-observation` gate. Its focused direct route is:

```text
powershell -NoProfile -ExecutionPolicy Bypass -File ./tools/check_lslc_003o.ps1
```

This validates the sanitized bounded matrix, damaged evidence mutations,
provenance hashes, public/private boundary, and documentation routes. It does
not rerun the private oracle or prove a production runtime.

Pinned immutable evidence is routed by `validation/historical-gates.json` and
the bound v2 manifest. The complete prior validation guide is preserved
byte-for-byte at
[`docs/history/VALIDATION-THROUGH-LSLC-003M.md`](history/VALIDATION-THROUGH-LSLC-003M.md).
The migration inventory and decision are in
[`docs/validation/VALIDATION-AUTHORITY-INVENTORY.md`](validation/VALIDATION-AUTHORITY-INVENTORY.md)
and [`docs/adr/LSLC-003N-VALIDATION-POLICY-AUTHORITY.md`](adr/LSLC-003N-VALIDATION-POLICY-AUTHORITY.md).

LSLC-003T is checked by `tools/check_lslc_003t.ps1` and policy gate
`lslc-003t-runtime`, covering exact capability composition, valid loopback,
damaged framing, UTF-8 and byte bounds, cancellation, deadline, cleanup,
provenance, documentation, and public boundary.

LSLC-003U is checked by `tools/check_lslc_003u.ps1` and policy gate
`lslc-003u-observation`. The gate validates the two exact cases, four pinned
attempt hashes, six damaged mutations, documentation routes, limitations, and
the current public boundary without executing an oracle in portable validation.

LSLC-003V is checked by `tools/check_lslc_003v.ps1` and policy gate
`lslc-003v-runtime-conformance`, including the accepted production-prefix byte
comparison, two focused loopback cases, damaged fixture mutations, routes, and
public boundary.

LSLC-003W is checked by `tools/check_lslc_003w.ps1` and policy gate
`lslc-003w-observation`. The gate validates the exact empty record, two pinned
attempt hashes, six damaged mutations, documentation routes, limitations, and
the public boundary without executing an oracle in portable validation.

LSLC-003X is checked by `tools/check_lslc_003x.ps1` and policy gate
`lslc-003x-runtime`. It validates empty-value loopback, prior bounds and
capability markers, six damaged fixture mutations, cleanup, documentation
routes, and the public boundary.

LSLC-003Y is checked by `tools/check_lslc_003y.ps1` and policy gate
`lslc-003y-observation`. The gate validates the exact 128-byte record, two
pinned attempt hashes, six damaged mutations, documentation routes,
limitations, and public boundary without executing an oracle.

LSLC-003Z is checked by `tools/check_lslc_003z.ps1` and policy gate
`lslc-003z-runtime`. It validates exact-128 loopback and cleanup, typed
129-byte rejection, preservation markers, six damaged fixture mutations,
documentation routes, and public boundary.

LSLC-004A is checked by `tools/check_lslc_004a.ps1` and policy gate
`lslc-004a-observation`. The gate validates the exact 129-byte record, two
pinned attempt hashes, six damaged mutations, documentation routes,
limitations, and public boundary without executing an oracle.
