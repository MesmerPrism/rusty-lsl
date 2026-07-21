# P43 caller-requested advisory report package

This candidate adds one substantive crate-private, default-inert package over the actual P42 `CallerRequestedFloat32ReportAdvisoryEvidenceHistory` and the actual `MorphospaceFloat32RetainedAdvisorySummary`. Construction happens only when a caller explicitly invokes the owner. The package consumes and retains both canonical inputs unchanged, including every nested report, advisory snapshot, history, and sample allocation.

The separate package index contains only deterministic `Copy` facts already exposed by those owners. In order, it records each P42 history value and its exact ordered-evidence count, each existing ordered evidence item, and each existing retained-summary fact. It does not derive packet loss, continuity, advice, policy, or application behavior.

Four nonzero bounds separately constrain history values, history evidence, retained-summary facts, and total package facts. Bound conversion, evidence accumulation, total arithmetic, every index/count conversion, and exact index allocation are fallible. Checked conversions and checked arithmetic precede allocation. Every typed failure returns the complete history and summary unchanged; successful indexing copies no sample, report, snapshot, history, or evidence allocation.

The module has no root or runtime export and is private, default-inert, and non-applying. It adds no liblsl-equivalence claim, socket, plugin, session, stream, transport, control, application, Manifold, device, Quest, ADB, Android, or Makepad authority. Focused qualification temporarily declares the module in `lib.rs`; that declaration is removed after the focused test run.

Focused qualification covers actual P42 history and actual retained-summary composition, exact deterministic order and totals, nested allocation identity across success and consuming extraction, all four explicit bounds, checked addition and conversion refusals, allocation refusal, complete owner return on every typed failure, and the private/default-inert/non-applying boundary.
