# Validation

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
