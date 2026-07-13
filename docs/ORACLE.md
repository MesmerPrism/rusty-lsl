# Official liblsl black-box oracle procedure

## LSLC-001H pinned execution

The first measured procedure uses only the official PyPI Windows AMD64
`pylsl 1.18.2` binary wheel, installed with `--no-deps` under an explicit
external temporary root. The harness verifies wheel SHA-256, CPython AMD64,
the pylsl version, public liblsl library version `117`, and the loaded native
DLL SHA-256 before accepting evidence. It invokes only public `StreamInfo`,
metadata-element, `as_xml`, and library-version APIs before any outlet or
inlet exists; it calls no resolution, discovery, or networking API.

Each case is captured twice from the same bounded object and must be byte
identical. Nonzero exit, over-bound output, missing/wrong artifacts,
version/architecture drift, repeat mismatch, malformed evidence, or prohibited
public content fails at a typed stage. Failure history is append-only. Machine-
specific raw XML and native stderr remain external; public normalization is
limited to recorded byte ranges of runtime/session/host/address/port text.

## Role and limit

Official liblsl may serve only as an MIT-licensed black-box reference endpoint.
Its source is not an implementation input or a test template. This procedure
does not run an oracle in STRM-000 and does not add a library, wrapper, native
binary, header, capture, or generated source to the repository.

An oracle observation can characterize only the named case, endpoint version,
environment, and bounded input. Agreement does not prove wire compatibility,
ecosystem compatibility, runtime support, or correctness beyond that case.

## Reproducible run procedure

1. Start from a clean checkout of the Rusty LSL revision under test. Record its
   commit, the case and negative-case identifiers, and the fixture digests.
2. Obtain an official released liblsl endpoint from its canonical public
   distribution outside this repository. Record release identity, retrieval
   URL, SHA-256, MIT license decision, and any platform signature information.
   Do not inspect or use liblsl source, generated protocol files, headers as
   design input, or rLSL source.
3. Use a purpose-built black-box driver outside the production dependency
   closure. Record driver identity, digest, command template, compiler or
   interpreter version, operating-system family and version, architecture,
   locale, time zone, and network topology class. Replace local paths, host
   names, endpoint addresses, and process identifiers with neutral labels.
4. Select exactly one catalog case and its declared bounds. Generate only the
   independently authored synthetic values named by that case. Apply a bounded
   wall-clock timeout and bounded retry count. Do not retain packet captures or
   implementation-derived protocol bytes in public fixtures.
5. Run each direction separately when direction matters: future Rusty LSL as
   producer with the official endpoint as observer, then the official endpoint
   as producer with future Rusty LSL as observer. A setup failure is classified
   separately and cannot become a product result.
6. Normalize the observation into the vocabulary below. Preserve the raw log
   outside the repository until public-boundary and license review; commit only
   a minimal normalized observation whose manifest records whether
   normalization discarded relevant detail.
7. Recompute SHA-256 for every committed observation and update its manifest.
   Run `tools/check_strm_000.ps1` and `tools/check_all.ps1` before review.

The driver command is recorded as an argument vector, not a shell transcript:

```text
<driver> --case <case-id> --role <producer-or-observer> --timeout-seconds <bound> --output <normalized-output>
```

Placeholders are replaced in the private run record. Public manifests retain
only neutral values and content digests.

## Normalized classifications

- `accepted`: the endpoint accepted the complete bounded input;
- `reject-input`: the endpoint rejected the input before a complete result;
- `incomplete-observation`: the bounded window ended with only a partial result;
- `timeout`: the explicit deadline elapsed;
- `unsupported`: the selected public endpoint reports the case unsupported;
- `provider-failure`: the endpoint or driver failed after setup;
- `setup-failure`: acquisition, launch, environment, or driver setup failed.

Repository-owned semantic checks may additionally use
`reject-authority-escalation`, `reject-semantic-loss`, and
`reject-evidence-loss`. Those are Rusty LSL contract expectations, not claims
about official liblsl behavior.

## Observation record

A measured record must contain the provenance fields defined in
`PROVENANCE.md`, a non-empty normalized observation, start and finish times,
exit classification, bounded stdout/stderr digests, and a `does_not_prove`
list. Until such a reviewed record exists, both `current_result` and the
measured-result status remain `not-implemented`.
