# Context Debug Contracts

This file records the low-level context debug contracts for the Hepta context lane.
The debug surfaces must stay payload-light: they may expose counts, hashes, sources,
budgets, and audit codes, but must not export prompt bodies, recalled memory text,
tool arguments, tool outputs, response item payloads, compaction bodies, or raw
replacement history.

## Turn Context Manifest

`TurnContextManifestItem` is a versioned rollout item for model-visible context
metadata. Most fields are shadow/debug metadata that can be audited and replayed
without attaching memory context to a model turn. A selected-snippet envelope is
the one guarded exception: core/session may render a validated, bounded,
redacted snippet set into live prompt context.

Core/session may persist the same payload-light manifest as
`TurnContextItem.context_manifest`. That field is a shadow handoff boundary for
replay/debug consumers: it records what context metadata was attached or carried
by the session baseline, but it must not inject recall source metadata, ranked
payloads, source ids, score reasons, per-source lists, or query payloads into the
live prompt. When a steady-state turn emits no context diffs, the session may
carry forward the previous shadow manifest so the latest reference baseline
remains auditable.

Core/session may also attach a `ContextRecallProvider` rollup to that shadow
manifest by setting `recall_selection`. This rollup is count-only provider
metadata. It may refresh the manifest ledger even when no context diff was
emitted, but it must not append prompt history or carry provider source ids,
selected snippets, ranked item payloads, score reasons, or per-source lists.

Memory recall manifest payload-light gate: the recall snapshot -> recall query
-> context manifest chain must stay behavior-neutral and payload-light across
the implementation boundary. `hepta-memory` may expose the original recall
request and aggregate counts in `ContextRecallReport`, but that report must not
contain raw memory, transcript, control-record, selected snippet source
metadata, or prior prompt text. The core manifest side may carry only the
count-only `recall_selection` rollup or a guarded bounded selected-snippet
envelope, and it must not persist prior prompt text. The context debug gate and
preflight must run
`scripts/hepta-context-memory-recall-manifest-payload-light-gate.sh` after the
recall helper boundary gate and before memory taxonomy/formation gates. The gate
must cover
`store_snapshot_recall_context_report_is_payload_light_across_query_boundaries`,
`turn_context_manifest_resolves_recall_provider_rollup_without_payload_text`,
and `turn_context_manifest_resolves_selected_snippets_as_guarded_payload`; it
must remain `runtime-activation=disabled`.

Memory recall ranking: `hepta-memory` may populate
`ContextRecallBundle.ranked_items` as a deterministic, payload-light shadow
ranking for local eval and inspection. Ranked summaries may include source
class, byte counts, booleans, stable source references, and explainable score
components, but must not carry raw memory text, transcript text, query payloads,
prompt text, selected snippet text, tool payloads, or provider output. Ranked
items must not be promoted into `TurnContextManifestItem`, model-visible prompt
context, production memory writes, or runtime activation paths without a
separate approval-gated contract.

Allowed manifest fields include:

- `version`
- `estimated_tokens`
- `ledger_hash`
- `budget_tokens`
- `omitted_entries`
- `omitted_sources`
- `truncated`
- `decision_ledger_hash`
- `decision_ledger`
- `recall_selection`
- `recall_selected_snippets`
- `compression_candidates`
- `adaptive_budget_allocations`
- `compression_stages`
- `entries`

Manifest entries must contain only payload-light replay identity:

- `role`
- `tier`
- `source`
- `replay_key`
- `text_hash`
- `estimated_tokens`

`tier` is a payload-light semantic layer for the entry, not a prompt payload. It
must be drawn from the versioned context-tier vocabulary (`system`,
`developer`, `user`, `tool`, `runtime`, `session_state`,
`cross_session_memory`, `retrieved_snippets`, `summary`, or `unknown`), and it
must not encode source ids, memory ids, ranked payloads, query text, or prompt
bodies. Older manifests may omit `tier`; consumers must read that as `unknown`
for replay compatibility.

The manifest decision ledger is the payload-light contribution ledger for
model-visible turn-context fragments. It may record one included decision per
manifest contribution, but must not carry prompt text or rendered fragment
content. Decision entries may contain only:

- `source`
- `decision`
- `reason_hash`

`reason_hash` may summarize the internal policy class and inclusion reason, but
the unhashed reason text must remain local to the producer.

Stable manifest replay hash: manifest `text_hash`, `reason_hash`,
`ledger_hash`, and `decision_ledger_hash` values use the protocol
`stable_turn_context_manifest_replay_hash` boundary. The legacy
`stable_turn_context_manifest_text_hash` function is kept only as a
backwards-compatible name for text hash callers. These values are deterministic
16-hex replay identities for payload-light debug/replay comparison; they are
not canonical trust digests and must not be used to approve release artifacts,
prove operator approval freshness, validate tamper evidence, authorize
activation, or compare security-sensitive payloads. Trust and approval exports
must use explicit canonical SHA-256 digest reports such as the Context Plane
operator approval packet canonical export digest chain. The context debug gate
and preflight must run
`scripts/hepta-context-manifest-replay-hash-boundary-gate.sh` before the source
registry catalog gate so replay-hash/trust-digest drift is visible before
later cargo/runtime stages.

The decision ledger wire format remains the existing payload-light string for
replay compatibility, but producers must construct known decisions through the
versioned `TurnContextDecisionKind` schema rather than ad hoc string formatting.
Schema version 1 covers `included`, `policy`, `candidate_omit`,
`candidate_truncate`, `omitted`, and `truncated` decisions. Unknown or malformed
decision strings fail manifest decision-ledger integrity; response-debug may
surface only the decision schema version and typed decision kind counts, not raw
decision strings.

The manifest compression-candidate list is the payload-light dry-run plan for
future first-class compression. It is optional and empty when no budget pressure
exists. Under budget pressure, producers may record candidate `summary`,
`rewrite`, `defragment`, or `prune` actions without changing prompt content or
claiming that compression already ran. A candidate may contain only controlled
taxonomy and counters: `kind`, `tier`, `source_id`, `input_tokens`,
`estimated_output_tokens`, `affected_entries`, and `not_executed_reason`.
`source_id` must be a bounded source taxonomy value such as
`selected_context_recall` or `available_plugins`; it must not be a manifest
`source`, replay key, text hash, prompt/snippet body, query, memory id, topic
id, neuron id, or per-source payload list. Candidate integrity requires known
kind/tier/reason values, payload-light source taxonomy, `estimated_output_tokens
<= input_tokens`, and a non-zero `affected_entries` count whenever
`input_tokens` is non-zero. Invalid candidates fail manifest replay integrity.
Candidate presence alone must not set `truncated`, `omitted_entries`,
`omitted_sources`, or non-empty `compression_stages`.

The manifest adaptive budget-allocation list is the payload-light dry-run report
for future registry-backed budget allocation. It is optional and empty unless a
turn is over budget. Under budget pressure, producers may record one row per
registered source id that contributed context, comparing the current source-aware
heuristic action with a proposed per-source budget allocation. A row may contain
only controlled taxonomy and counters: `tier`, `source_id`, `budget_class`,
`input_tokens`, `reserve_tokens`, `proposed_budget_tokens`, `overflow_tokens`,
`omit_priority`, `compression_kind`, `estimated_compressed_tokens`,
`current_heuristic_action`, `proposed_action`, `would_drop`, and
`would_compress`. `source_id` and `budget_class` must come from the context
source registry and must not encode manifest source strings, replay keys, text
hashes, prompt bodies, snippet text, query text, ranked payloads, memory ids,
topic ids, or per-source payload lists. Allocation integrity requires known
tier/action/compression values, `reserve_tokens <= input_tokens`,
`proposed_budget_tokens <= input_tokens`, `overflow_tokens ==
input_tokens - proposed_budget_tokens`, and boolean `would_drop`/`would_compress`
values that match the proposed action. Allocation presence is behavior-neutral:
it must not mutate model-visible prompt items, must not set `truncated`,
`omitted_entries`, `omitted_sources`, or non-empty `compression_stages`, and
must not claim that compression already ran.

The manifest compression-stage list is the payload-light scaffold for future
first-class compression. It is optional and empty by default, so legacy manifests
and default non-compressing assembly keep their previous replay hash shape.
Non-empty stages must use only the versioned compression vocabulary:
`summary`, `rewrite`, `defragment`, or `prune`. A stage may record only
payload-light counters: `kind`, `input_tokens`, `output_tokens`, and
`affected_entries`. It must not carry source ids, replay keys, text hashes,
prompt text, snippet text, query text, ranked payloads, memory ids, topic ids, or
per-source lists. Stage integrity requires known kind values, `output_tokens <=
input_tokens`, and a non-zero `affected_entries` count whenever `input_tokens`
is non-zero. Invalid stages fail manifest replay integrity.

The manifest assembly policy must also be represented in the decision ledger as
a payload-light policy entry. The current rollout policy is
`policy:non_omitting_replay_baseline:{within_budget|budget_exceeded}`. It records
the effective assembly budget and budget pressure without changing prompt
content. Under budget pressure, the decision ledger must add source-aware dry-run
actions with `candidate_omit:*` for eligible low-priority tier/source pairs and,
if a remaining eligible low-priority fragment could be shortened, a
`candidate_truncate:*`. Candidate truncation must not point at protected
system/developer/user/runtime/session-state context merely because the budget is
still exceeded after low-priority omission candidates are exhausted. These
candidate decisions are payload-light evidence for the next real assembly
policy; they must not claim that prompt content was already changed. Therefore
`omitted_entries` remains zero, `omitted_sources` remains empty, and `truncated`
remains false unless a future policy actually omits or truncates model-visible
context.

The opt-in source-aware assembly policy is
`policy:source_aware_omission:{within_budget|budget_exceeded}`. It may omit only
low-priority context sources from the manifest entries, using the same
deterministic tier-guarded priority ladder as dry-run candidates:
tool-tier `extension_developer_capabilities`, `available_plugins`, `apps`, and
`available_skills`, followed by retrieved-snippet-tier
`selected_context_recall`. Omitted fragments must be recorded with `omitted:*`
decision entries, `omitted_entries`, and
`omitted_sources`; protected sources such as permissions, system/developer
instructions, environment, collaboration mode, realtime, personality, extension
policy, and user/developer instruction clears must remain included. Actual
text-level truncation is not enabled by this omission-only policy; if omission
alone is insufficient, the ledger may still emit `candidate_truncate:*` for a
remaining eligible low-priority fragment, but it must not nominate protected
context and `truncated` must remain false unless a paired assembly result
rewrites both the model-visible fragment text and the manifest entry `text_hash`
for the changed fragment. Session turns with a known effective model context
window must use that window as `budget_tokens`; test/default builders may fall
back to the included ledger estimate so the budget field is never absent for a
manifest with entries.

The narrower opt-in policy
`policy:source_aware_omission_and_truncation:{within_budget|budget_exceeded}`
may truncate only remaining low-priority text fragments after source-aware
omission cannot fit the budget. It must run through paired prompt/manifest
assembly, preserve the original source id, replace the prompt text before model
dispatch, replace the manifest entry hash with the hash of that truncated text,
set `truncated=true`, and record `truncated:*` decision evidence. Protected
sources remain non-omittable and non-truncatable under this policy. A caller must
not set `truncated=true` from manifest-only accounting, and must not expose an
untruncated prompt item with a manifest that claims truncation.

The opt-in summary policy
`policy:source_aware_summary:{within_budget|budget_exceeded}` may execute only a
retrieved-snippet-tier `selected_context_recall` summary transform. It must run
through paired prompt/manifest assembly, preserve the original source id, replace
the model-visible selected-recall text before dispatch, replace the manifest
entry hash and token estimate with the summary text identity, and record a
payload-light `summary` compression stage. The corresponding
`selected_context_recall` summary compression candidate must be removed from the
remaining dry-run candidate list once the stage is executed; tool-tier
`defragment`/`prune` candidates remain dry-run until a later explicit policy
executes them. The stage itself must not carry source ids, replay keys, text
hashes, prompt/snippet text, query text, ranked payloads, memory ids, topic ids,
or per-source lists. Default non-omitting assembly and manifest-only builders
must not claim real summary execution.

The opt-in tool defragment policy
`policy:source_aware_tool_defragment:{within_budget|budget_exceeded}` may execute
only tool-tier `available_plugins`, `apps`, or `available_skills` defragment
transforms. It must run through paired prompt/manifest assembly, preserve the
original source id, replace the model-visible tool inventory text before
dispatch, replace the manifest entry hash and token estimate with the
defragmented text identity, and record a payload-light `defragment` compression
stage. The corresponding executed tool inventory compression candidate must be
removed from the remaining dry-run candidate list. This policy must not execute
`prune` for `extension_developer_capabilities`, and the stage itself must not
carry source ids, replay keys, text hashes, prompt/tool text, query text, memory
ids, topic ids, or per-source lists.

The opt-in tool prune policy
`policy:source_aware_tool_prune:{within_budget|budget_exceeded}` may execute
only tool-tier `extension_developer_capabilities` prune transforms. It must run
through paired prompt/manifest assembly, preserve the original source id,
replace the model-visible extension capabilities text before dispatch, replace
the manifest entry hash and token estimate with the pruned text identity, and
record a payload-light `prune` compression stage. The corresponding executed
extension capabilities compression candidate must be removed from the remaining
dry-run candidate list. This policy must not execute `defragment` for
`available_plugins`, `apps`, or `available_skills`, and the stage itself must
not carry source ids, replay keys, text hashes, prompt/tool text, query text,
memory ids, topic ids, or per-source lists. Default non-omitting assembly and
manifest-only builders must not claim real prune execution.

The opt-in multi-compression policy
`policy:source_aware_compression:{within_budget|budget_exceeded}` may execute
the same bounded real transforms together in one paired assembly pass:
`summary` for retrieved-snippet-tier `selected_context_recall`, `defragment` for
tool-tier `available_plugins`/`apps`/`available_skills`, and `prune` for
tool-tier `extension_developer_capabilities`. Every executed transform must
rewrite the model-visible prompt before dispatch, preserve the original source
id in the manifest entry, update that entry's text hash and token estimate from
the rewritten text, record one payload-light compression stage, and remove its
executed dry-run candidate. This policy is explicit opt-in only and must not
change default non-omitting runtime assembly. Even under this explicit combined
policy and an exceeded budget, protected system/developer/user/runtime/
session-state context must remain included, must not be nominated as a
compression candidate, and must not be rewritten into summary, defragment, or
prune prompt markers.

Controlled callers that opt into `source_aware_omission`, `source_aware_summary`,
`source_aware_tool_defragment`, `source_aware_tool_prune`,
`source_aware_compression`, or
`source_aware_omission_and_truncation` must assemble prompt items and manifest
evidence as one result. They must filter the actual prompt fragments named in
`omitted_sources` before model dispatch, apply any paired rewrites before the
manifest is persisted, and persist or expose the paired manifest from the same
assembly pass. A caller must not combine a manifest that claims omitted,
summarized, defragmented, pruned, or truncated fragments with
unfiltered/unrewritten prompt items, and must not rebuild source ids after
filtering in a way that loses the original evidence. The crate-local session
handoff explicit-policy entry point is the controlled non-default caller for
this behavior: it may use these policies only when explicitly passed that policy,
while the normal runtime handoff continues to pass the non-omitting replay
baseline. For `source_aware_summary`, that handoff must persist both the
summarized selected-recall prompt item and the paired manifest with the summary
text hash and `compression_stages=[summary]` from the same assembly pass.
For `source_aware_tool_defragment`, that handoff must persist both the
defragmented tool-inventory prompt item and the paired manifest with the
defragmented text hash and `compression_stages=[defragment]` from the same
assembly pass.
For `source_aware_tool_prune`, that handoff must persist both the pruned
extension capabilities prompt item and the paired manifest with the pruned text
hash and `compression_stages=[prune]` from the same assembly pass.
For `source_aware_compression`, that handoff must persist the summarized
selected-recall prompt item, the defragmented tool-inventory prompt item, the
pruned extension capabilities prompt item, and the paired manifest with matching
rewritten text hashes plus one payload-light stage for each executed transform
from the same assembly pass. The normal session handoff may opt into
`source_aware_compression` only when the `source_aware_compression_canary`
feature is enabled and the explicit turn-scoped assembly-policy marker is
present in extension data. Without both controls, even an exceeded model context
budget must stay on the non-omitting replay baseline and emit only dry-run
compression candidates. Enabling the canary feature through app-server
experimental feature control is not sufficient by itself; the explicit
turn-scoped marker remains the second gate for real prompt rewriting. Rollout
persistence/readback must contain only the resulting prompt items and paired
manifest; it must not serialize the canary feature key or the opt-in marker
type/value.
The turn-scoped marker injection path is intentionally named and local:
`insert_source_aware_compression_policy_opt_in_marker` is the only crate-visible
way to add the `SourceAwareCompression` assembly-policy marker to extension
data. The marker payload type itself must remain private to the context manifest
module, so future runtime/app-server/config/thread/history/debug surfaces cannot
synthesize the marker by writing a raw type into extension data. A canary feature
gate without that helper-injected marker must resolve to the non-omitting replay
baseline, and a helper-injected marker without the canary feature gate must also
resolve to the baseline.
Until a deliberately named runtime activation route is added, helper call sites
must remain restricted to the context manifest module and session tests. The
source-aware compression activation-surface audit must fail if app-server
protocol/config/read/write/turn-start/thread-history surfaces, native gateway,
runtime selected-snippet bridges, TUI, exec, or response-debug/export code call
the marker helper, write the raw marker type, or resolve assembly policy outside
the normal core session handoff path.
The reserved future production seam name is
`apply_source_aware_compression_operator_approved_runtime_activation_marker`,
and the reserved activation key is
`source_aware_compression_operator_approved_runtime_activation`. Both are only a
contract for a future explicitly approved runtime route, not an implemented
activation path. Until that route is deliberately added with operator approval,
the canary feature gate, and the existing turn-scoped marker semantics, those
names must not appear in production code and ad-hoc
`source_aware_compression` runtime activation code must be rejected by preflight.
Runtime activation readiness checklist: before the reserved seam can be
implemented, the route must prove operator approval evidence, the `source_aware_compression_canary`
feature enabled, an explicit helper-injected turn-scoped source-aware compression
marker, no rollout/debug/export marker or canary leakage, and a negative matrix
where default runtime must remain on the non-omitting replay baseline whenever
any one of those controls is missing. The source-aware compression readiness gate
must run before runtime cargo stages in the context preflight, so sibling runtime
generated-code failures cannot hide checklist drift.
Operator approval evidence contract: the future reserved route must require a
structured evidence record named
`source_aware_compression_operator_approval_evidence` with type
`SourceAwareCompressionOperatorApprovalEvidence`. That evidence shape must bind
`source_aware_compression_operator_approval_id`,
`source_aware_compression_operator_identity_hash`,
`source_aware_compression_activation_request_id`, the reserved activation key,
the reserved activation entrypoint, the `source_aware_compression_canary`
feature key, the helper-injected marker binding,
`source_aware_compression_operator_approval_scope_hash`,
`source_aware_compression_operator_approval_nonce`, and
`source_aware_compression_operator_approval_expires_at`. Until the future route
is deliberately implemented, those names remain contract-only: app-server,
runtime, config, thread-history, TUI, exec, native-gateway, and debug/export
surfaces must not synthesize, persist, or consume that evidence. Default runtime
must remain on the non-omitting replay baseline unless the reserved route +
canary + helper-injected marker + approval evidence are all present.
Source-aware compression readiness export surface: the contract-only readiness,
operator-approval evidence, and positive-route checks must also be represented
as a payload-light fixed status block from
`scripts/hepta-context-source-aware-compression-readiness-export-report.sh`.
That report must emit `source-aware-readiness-export=pass`,
`source-aware-readiness-export.runtime-activation=disabled`,
`source-aware-readiness-export.operator-approval-evidence=contract-only`,
`source-aware-readiness-export.positive-route=unimplemented`,
`source-aware-readiness-export.no-production-consumption=pass`, and
`source-aware-readiness-export.no-debug-export-leak=pass` without source ids,
prompt text, transcript text, memory text, answer text, operator identity, raw
payload, or activation evidence. The guard
`scripts/hepta-context-source-aware-compression-readiness-export-gate.sh` must
run after the operator approval evidence gate and before the activation
negative matrix gate in debug, preflight, and the source-aware front-door so a
stale or duplicated readiness summary cannot pass as implicit approval.
Source-aware compression activation negative matrix: every missing-control
combination across the four future activation controls must resolve to
baseline/non-rewriting behavior with no summary/defragment/prune prompt marker,
no activation evidence persistence, and no rollout/debug/export routing leakage.
The four controls are the reserved runtime activation route, the
`source_aware_compression_canary` feature enabled state, the helper-injected
turn-scoped source-aware compression marker, and the
`source_aware_compression_operator_approval_evidence` record. The matrix must
retain all 15 negative case labels: `missing-route`, `missing-canary`,
`missing-helper-marker`, `missing-approval-evidence`, `missing-route+canary`,
`missing-route+helper-marker`, `missing-route+approval-evidence`,
`missing-canary+helper-marker`, `missing-canary+approval-evidence`,
`missing-helper-marker+approval-evidence`,
`missing-route+canary+helper-marker`,
`missing-route+canary+approval-evidence`,
`missing-route+helper-marker+approval-evidence`,
`missing-canary+helper-marker+approval-evidence`, and
`missing-route+canary+helper-marker+approval-evidence`. Only the complete
positive set of reserved route + canary + helper-injected marker + approval
evidence may leave the baseline, and that positive route remains unimplemented
until an explicitly reviewed runtime activation change lands.
The response-debug export gate and app-server thread-history contract must keep
negative-matrix labels and operator-approval evidence names as leak bait. Those
bait values may appear in fixture input, selected-snippet handoff payloads, or
test assertions, but must not appear in response-debug exported JSON,
thread/read, thread/turns/list, rollout summaries, or any runtime routing
metadata surface.
Source-aware compression positive-route readiness review: the complete positive
set of reserved route + canary + helper-injected marker + approval evidence
remains unimplemented. Before production code may consume
`source_aware_compression_operator_approved_runtime_activation`, call
`apply_source_aware_compression_operator_approved_runtime_activation_marker`, or
consume `source_aware_compression_operator_approval_evidence`, the change must
update the source-aware compression readiness, operator-approval evidence,
activation negative-matrix, activation-surface, and leak-bait gates:
`scripts/hepta-context-source-aware-compression-readiness-gate.sh`,
`scripts/hepta-context-source-aware-compression-operator-approval-evidence-gate.sh`,
`scripts/hepta-context-source-aware-compression-activation-negative-matrix-gate.sh`,
`scripts/hepta-context-source-aware-compression-activation-surface-audit.sh`,
and `scripts/hepta-context-source-aware-compression-leak-bait-gate.sh`. It must
also add rollout/debug/export no-leak tests covering the response-debug export
gate and app-server thread-history contract, binding
`source_aware_compression_canary`,
`insert_source_aware_compression_policy_opt_in_marker`, the reserved activation
seam, and the operator-approval evidence record together. Until those controls
and tests land together, the default runtime remains non-rewriting and the
positive route remains unimplemented.
Source-aware compression positive-route implementation-change detector: the
reserved positive route must stay unimplemented unless its activation code and
no-leak tests land in one reviewed change. If production code starts referencing
`source_aware_compression_operator_approved_runtime_activation`,
`apply_source_aware_compression_operator_approved_runtime_activation_marker`, or
`source_aware_compression_operator_approval_evidence`, the positive-route
readiness gate must also be updated and these explicit fixture names must exist:
`source_aware_compression_positive_route_response_debug_export_no_leak`,
`source_aware_compression_positive_route_app_server_thread_history_no_leak`, and
`source_aware_compression_positive_route_rollout_readback_no_leak`. Without
those names in the response-debug export gate, app-server thread-history tests,
and rollout/readback tests respectively, preflight must fail. While the positive
route remains contract-only, production code must not reference the reserved
activation key, reserved entrypoint, or operator-approval evidence.
Source-aware compression compile-independent front-door gate: the context
preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report.sh` before
runtime cargo stages. That mini-runner must execute
`scripts/hepta-context-source-aware-compression-front-door-gate.sh`, preserve the
front-door output, and then emit a compact machine-readable status block
including `source-aware-contracts=pass`,
`source-aware-contracts.runtime-dirty-classifier=none|non-blocking`, and
`source-aware-contracts.runtime-activation=disabled`. This front-door gate is
compile-independent and aggregates the source-aware compression readiness,
operator-approval evidence, readiness-export, negative-matrix,
activation-surface, leak-bait, positive-route readiness, and
implementation-change detector gates before any generated runtime code can
obscure their result. The front-door may classify
sibling runtime generated preview dirty state as a non-blocking classifier
notice, but it must not fail the context source-aware contract gates for that
sibling state, must not edit runtime code, and does not enable runtime activation.
Its dirty-preview truncation must avoid early-closing head pipelines under
`pipefail`, so a long sibling generated-preview dirty list cannot fail the
front-door before the readiness gates run.
Source-aware compression front-door report status assertion: the context
preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh`
immediately after the front-door report and before the fine-grained source-aware
gates. The status assertion gate must parse the machine-readable report output
and require exactly one `source-aware-contracts=pass`, exactly one
`source-aware-contracts.front-door=pass`, exactly one
`source-aware-contracts.runtime-dirty-classifier=none|non-blocking`, exactly one
`source-aware-contracts.runtime-activation=disabled`, and exactly one gate-list
line equal to
`readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector`.
It must reject any unknown extra source-aware-contracts.* key and preserve a
stable machine-readable status order:
`source-aware-contracts`, `source-aware-contracts.front-door`,
`source-aware-contracts.runtime-dirty-classifier`,
`source-aware-contracts.runtime-activation`, then
`source-aware-contracts.gates`. If the status block is missing, duplicated,
malformed, reordered, includes unknown machine-readable status keys, or claims
runtime activation is anything other than disabled, the preflight must fail
before any runtime cargo stage.
Source-aware compression front-door report status negative harness: the context
preflight must also run
`scripts/hepta-context-source-aware-compression-front-door-report-status-negative-gate.sh`
after the live status assertion and before fine-grained source-aware gates. This
negative harness must use synthetic report input, not the full front-door, to
prove the parser rejects duplicate source-aware-contracts status, a missing gate list,
a malformed classifier, and `runtime-activation=enabled`. It must keep a
good synthetic report passing so failures identify parser drift rather than the
synthetic input mechanism.
Source-aware compression front-door report status fixture matrix: the context
preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-fixture-matrix-gate.sh`
after the status negative harness and before fine-grained source-aware gates.
This matrix protects the synthetic input seam as a contract: a good synthetic
report must pass, while an unknown extra source-aware-contracts.* key, a
reordered machine-readable block, and a duplicated machine-readable block must
fail. The matrix must keep runtime activation disabled and must not run the full
front-door, so parser-contract drift is isolated from compile or runtime dirty
state.
Source-aware compression front-door report status artifact consumer: the
context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh`
after the status fixture matrix and before fine-grained source-aware gates. This
artifact consumer must run the real front-door report, extract exactly the five allowlisted source-aware-contracts status lines
into a temporary artifact, prove the artifact is in stable order, prove it has
no front-door diagnostic noise, and then feed that artifact back through the
report status assertion gate. The
artifact contract is only for downstream CI/operator log ingestion; runtime
activation disabled remains mandatory and this artifact must not become a
runtime activation route.
Source-aware compression persisted status artifact export: the context preflight
must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-gate.sh`
after the status artifact consumer and before fine-grained source-aware gates.
When `HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` is set, the
artifact consumer may write the already-validated five-line status artifact to a
caller-provided path. The persisted artifact must preserve the allowlisted
source-aware-contracts key order, contain no front-door diagnostic noise, keep
runtime activation disabled, and must not become a runtime activation route.
Source-aware compression persisted status artifact export negative matrix: the
context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-negative-gate.sh`
after the persisted status artifact export and before fine-grained source-aware
gates. The negative matrix must prove that
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` rejects bad
caller-provided paths, including a directory target and a missing parent
directory. This missing parent directory case and the directory target case must
not write a persisted artifact, must keep runtime
activation disabled, and must not become a runtime activation route.
Source-aware compression persisted status artifact export precheck: the context
preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-precheck-gate.sh`
after the persisted artifact export negative matrix and before fine-grained
source-aware gates. Invalid
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` paths, including a
directory target and a missing parent directory, must fail before running the real front-door report, before emitting `source-aware-contracts` status, and
without enabling runtime activation. This precheck remains a contract-only guard
and must not become a runtime activation route.
Source-aware compression persisted status artifact export overwrite/idempotence:
the context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-idempotence-gate.sh`
after the persisted artifact export precheck and before fine-grained
source-aware gates. When
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` points at a
preexisting caller-provided file, repeated successful runs must leave the
persisted artifact overwritten, not appended. The resulting artifact must still
contain exactly the five allowlisted source-aware-contracts lines in stable
order, have no front-door diagnostic noise, keep runtime activation disabled,
and must not become a runtime activation route.
Source-aware compression persisted status artifact export atomic replace: the
context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-atomic-gate.sh`
after the persisted artifact export overwrite/idempotence gate and before
fine-grained source-aware gates. When
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` points at a
preexisting caller-provided file, the artifact consumer must write the validated
five-line status artifact to a same-directory temporary file and use `mv` to
replace the final output path. It must have no direct final-path copy, leave
no temporary artifact residue on successful export, keep runtime activation
disabled, and must not become a runtime activation route.
Source-aware compression persisted status artifact export writability precheck:
the context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-writability-precheck-gate.sh`
after the persisted artifact export atomic replace gate and before fine-grained
source-aware gates. When
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` points inside an
unwritable parent directory, the artifact consumer must fail before running the
real front-door report, before emitting source-aware-contracts status, and
without creating a final artifact. Runtime activation disabled remains mandatory
and this precheck must not become a runtime activation route.
Source-aware compression persisted status artifact export symlink replacement:
the context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate.sh`
after the persisted artifact export writability precheck and before
fine-grained source-aware gates. When
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` is a symlink to a
victim file, the atomic replace path must replace the symlink itself with the
validated five-line status artifact and must not follow the symlink or mutate
the victim file. Runtime activation disabled remains mandatory and this
behavior must not become a runtime activation route.
Source-aware compression persisted status artifact export hardlink replacement:
the context preflight must run
`scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-hardlink-gate.sh`
after the persisted artifact export symlink replacement and before fine-grained
source-aware gates. When
`HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT` is a hardlink to a
victim file, the atomic replace path must unlink/replace only that output pathname
with the validated five-line status artifact and must not mutate the other hardlink target.
Runtime activation disabled remains mandatory and this behavior must not become a
runtime activation route.
Source-aware compression front-door gate-list parity: the context preflight must
run
`scripts/hepta-context-source-aware-compression-front-door-gate-list-parity-gate.sh`
after the persisted artifact export hardlink replacement and before
fine-grained source-aware gates. The parity gate must derive the actual front-door run_contract_gate order from
`scripts/hepta-context-source-aware-compression-front-door-gate.sh`, map only the
known source-aware contract labels to machine-readable tokens, and require the
derived list to equal
`readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector`.
It must also prove that the front-door report, status parser, synthetic
fixtures, and persisted artifact consumers use the same list and no stale
seven-gate summary; the same eight gate tokens must be derivable from the
explicit preflight stages, even when prerequisite-sensitive stages such as the
activation surface audit run later in preflight than they do in the front-door
script. On success it emits
`source-aware-front-door-gate-list-parity=pass`,
`source-aware-front-door-gate-list-parity.gate-count=8`, the exact front-door
gate list, `source-aware-front-door-gate-list-parity.preflight-gates=...` with
the exact preflight gate set, and
`source-aware-front-door-gate-list-parity.runtime-activation=disabled`. It must
not inspect or consume raw payloads, enable runtime activation, or become a
production routing surface.
The normal app-server `turn/start` bridge must preserve that boundary: enabling
`source_aware_compression_canary` in config and sending selected-snippet context
without a turn-scoped source-aware compression marker must leave the model
request with the bounded selected-recall payload and no summary/defragment/prune
markers or routing metadata.
App-server thread history/debug surfaces must preserve the same separation:
after such a canary-enabled selected-snippet turn, `thread/read` and
`thread/turns/list` may expose only ordinary user/assistant turn items. They
must not serialize the canary key, the turn-scoped opt-in marker, selected-recall
shadow payloads, source identifiers, or compression prompt markers.

Contribution sources must be semantic enough to audit deterministic context
coverage across both initial context injection and steady-state settings diffs.
Known source ids include `permissions`, `environment`, `model_switch`,
`collaboration_mode`, `realtime`, `personality`, `apps`, `available_skills`,
`available_plugins`, `extension_developer_policy`,
`extension_developer_capabilities`, `extension_separate_developer`,
`extension_contextual_user`, `user_instructions`, `developer_instructions`,
`multi_agent_usage_hint`, and `selected_context_recall`. Pure positional entries
are not sufficient for new context producers because they cannot explain which
policy surface changed.
Context lane release manifest: `codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv` is
the required file-set manifest for the Hepta context lane. It must list every
context-lane contract, registry, Rust context/memory/protocol handoff module,
response-debug bridge, and `scripts/hepta-context*.sh` gate/report script that
is required for a release bundle. Each row must contain a relative `path`,
controlled `category`, `owner_lane=hepta-context`, and
`release_class=required`. The machine-readable gate is
`context-lane-release-manifest=pass`; it must verify the manifest schema,
sortedness, duplicate-free path set, file existence, dynamic coverage of all
`scripts/hepta-context*.sh` files, and git-tracked release state for every
required path. Normal debug runs may report `release-ready=blocked-untracked`
when local lane artifacts have not yet been promoted into git, but
`HEPTA_CONTEXT_RELEASE_STRICT_GIT=1` is the strict git tracking mode. The strict
git tracking mode must fail if any required path is still untracked, and the
context preflight must invoke the release manifest gate in strict mode. The
manifest and gate are release-readiness control-plane metadata only: they must
not change prompt assembly, must not write turn-scoped opt-in markers, must not
stage or mutate git state, and must not become a runtime activation route. The
context debug gate and preflight must run
`scripts/hepta-context-release-manifest-gate.sh` before the source registry
catalog gate so publishability drift is visible before later cargo/runtime
stages.
Context source registry catalog: `codex-rs/CONTEXT_SOURCE_REGISTRY.tsv` is the
read-only catalog for current turn-context source taxonomy. The catalog must
contain one stable row per source id with exactly these columns: `source_id`,
`tier`, `owner_lane`, `privacy_class`, `budget_class`, `ttl`, `volatility`,
`trust_class`, `redaction_policy`, `quality_metric`, `activation_guard`,
`rollback_policy`, `omit_priority`, and `allowed_compression_actions`. It must
include the manifest classifier sources
`apps`, `available_plugins`, `available_skills`, `collaboration_mode`,
`context`, `contextual_user`, `developer_instructions`, `environment`,
`extension_contextual_user`, `extension_developer_capabilities`,
`extension_developer_policy`, `extension_separate_developer`, `model_switch`,
`multi_agent_usage_hint`, `non_text_content`, `permissions`, `personality`, `realtime`,
`selected_context_recall`, and `user_instructions` in stable sorted order. The
catalog is for audit/debug/export and future budget-planner input only; it must
not change prompt assembly, must not write turn-scoped opt-in markers, and must
not become a runtime activation route. The context preflight must run
`scripts/hepta-context-source-registry-catalog-gate.sh` before the source-aware
compression front-door report so source taxonomy drift is visible before later
cargo/runtime stages.
Context source registry Rust resolver: `codex-rs/core/src/context_manager/source_registry.rs`
is the read-only Rust-side resolver facade for the same catalog. Its leaf files
`source_registry/catalog.rs`, `source_registry/entry.rs`,
`source_registry/health.rs`, and `source_registry/tests.rs` must expose the
registry entries, typed entry metadata, the payload-light registry health
report, and a test that compares the Rust rows with
`CONTEXT_SOURCE_REGISTRY.tsv`. The facade must provide the single registry lookup
used by the manifest classifier, source-aware omit priority, and allowed
compression action selection. `manifest.rs` must not carry a second hard-coded omit-priority
or compression-action ladder for the same sources. This resolver is still
control-plane metadata only: it must not change prompt assembly by itself, must
not inject source-aware compression markers, and must keep
`runtime-activation=disabled` until the explicit operator-approved activation
path is implemented. The context debug gate and preflight must run
`scripts/hepta-context-source-registry-rust-gate.sh` after the catalog gate and
`scripts/hepta-context-source-registry-health-gate.sh` after the Rust resolver
gate and before the source-aware compression front-door report. The health gate
must report descriptor coverage for all 20 sources, must keep
`live-activation-routes=0`, and must keep `runtime-activation=disabled`.
Context health/meta report: `scripts/hepta-context-health-report.sh` is the
payload-light rollup for the context lane's gate surface. It may aggregate only
control-plane counts and fixed status strings from the release manifest, source
registry, source-aware compression readiness contracts, selected-snippet canary
contract, and preflight stage list. It must not export prompt bodies, recalled
memory text, tool arguments, tool outputs, response item payloads, compaction
bodies, raw replay keys, or rollback hash values. The paired
`scripts/hepta-context-health-gate.sh` must verify source-registry descriptor
coverage, zero live activation routes, the `selected_context_recall` operator
approval guard, explicit-canary-only prompt mutation, and
`runtime-activation=disabled`. This meta-gate must not rerun cargo gates or
become a runtime activation route; it exists to catch drift in the growing gate
surface before heavier checks run. The context debug gate and preflight must run
`scripts/hepta-context-health-gate.sh` after the source-registry health gate and
before the adaptive budget allocation report gate. The emitted status lines are
the source-registry descriptor coverage checkpoint for this lane and must keep
the selected_context_recall operator approval guard visible as a fixed
control-plane status.
Context adaptive budget allocation dry-run report: the non-omitting baseline
manifest may use the Rust source registry as allocator input when estimated
context exceeds the effective budget. It must emit a payload-light
`adaptive_budget_allocations` plan containing proposed per-source
budget/reserve/overflow decisions, `would_drop`/`would_compress` booleans, and a
comparison between `current_heuristic_action` and `proposed_action`. The core
implementation lives behind `codex-rs/core/src/context_manager/budget_planner.rs`
so `manifest.rs` remains the assembly owner rather than the budget-planning
owner. The report is an observation surface only: it must not change actual prompt assembly,
must not remove context items, must not execute
summary/defragment/prune transforms, must not inject source-aware compression
opt-in markers, and must keep
`runtime-activation=disabled`. Response-debug export may summarize allocation
schema version, source ids, budget classes, action names, token totals, and
drop/compress counts, but must not export prompt payloads, manifest source
strings, replay keys, text hashes, queries, memory ids, or per-source payload
lists. The context debug gate and preflight must run
`scripts/hepta-context-adaptive-budget-allocation-report-gate.sh` after the Rust
registry resolver gate and before the source-aware compression front-door report.
Memory taxonomy report: recall diagnostics may expose a payload-light
`memory_taxonomy` report that maps durable memory hits to `semantic`, session
summary hits to `episodic`, filtered tombstone/conflict records to `control`,
and recent/query transcript evidence to `transcript`. The report is an
observation surface only: it must not write production memory, form memory
candidates, alter prompt assembly, or enable runtime activation. Buckets may
contain only controlled taxonomy and counters: `class`, `source_count`,
`returned_count`, `available_count`, `omitted_count`, and
`provenance_span_count`. They must not contain prompt text, transcript text,
memory text, source ids, replay keys, text hashes, memory ids, topic ids, neuron
ids, query payloads, ranked payloads, or per-source lists. Bucket integrity
requires a known class, `returned_count <= available_count`, and
`omitted_count == available_count - returned_count`. Response-debug export may
summarize only schema version, classes, aggregate source/returned/available/
omitted/provenance counts, and invalid state. The context debug gate and
preflight must run `scripts/hepta-context-memory-taxonomy-report-gate.sh` after
the adaptive budget allocation gate and before the source-aware compression
front-door report. It must keep `runtime-activation=disabled`.
Background memory formation receipt report: recall diagnostics may expose a
payload-light `memory_formation_receipts` report that represents hot-path
enqueue intent for future background memory formation. Receipts are metadata
only and may contain controlled candidate types (`fact`, `task`, `preference`,
`decision`, `summary`), transcript/provenance span counts,
`confidence_basis_points`, a stable `idempotency_key_hash`, `privacy_class`,
`queued_for_background`, and `production_write=false`. They must not contain
candidate text, prompt text, transcript text, memory text, source ids, replay
keys, text hashes, raw idempotency keys, memory ids, topic ids, neuron ids,
query payloads, ranked payloads, tool arguments, or per-source lists. Receipt
integrity requires a known candidate type, positive bounded span counts,
`confidence_basis_points <= 10000`, a stable hashed idempotency key, a
payload-light privacy class, `queued_for_background=true`, and
`production_write=false`. This report must not write production memory, must not create durable memory candidates, must not alter prompt assembly, and must
not enable runtime activation. Response-debug export may summarize only schema
version, candidate types, privacy classes, aggregate span/confidence counts,
queued count, production-write count, and invalid state; it must not export
idempotency hashes or any raw payload. The context debug gate and preflight must
run `scripts/hepta-context-memory-formation-receipt-gate.sh` after
`scripts/hepta-context-memory-taxonomy-report-gate.sh` and before
`scripts/hepta-context-memory-formation-queue-gate.sh`. It must keep
`runtime-activation=disabled`.
Memory formation queue dry-run report: recall diagnostics may expose a
payload-light `memory_formation_queue` report that converts formation receipts
into background-queue metadata before any durable write path exists. Queue items
may contain controlled candidate types, transcript/provenance span counts,
`confidence_basis_points`, stable `idempotency_key_hash`,
`source_receipt_hash`, `revocation_key_hash`, `privacy_class`,
`operator_review_required`, `retention_ttl_turns`, `queued_for_background=true`,
`dry_run_only=true`, `idempotency_enforced=true`,
`can_revoke_before_commit=true`, `production_write=false`,
`graph_write=false`, and `hot_path_write=false`. They must not contain
candidate text, prompt text, transcript text, memory text, source ids, replay
keys, text hashes, raw idempotency keys, memory ids, topic ids, neuron ids,
query payloads, ranked payloads, tool arguments, per-source lists,
email-shaped strings, phone-shaped strings, user identifiers, or any queue
replay payload. Queue integrity requires known candidate and operator policy
enums, positive bounded span counts, `confidence_basis_points <= 10000`, stable
source receipt and revocation hashes, a payload-light privacy class, non-zero
retention TTL, background queue intent, enforced idempotency, revocation before
commit, and no production writes, graph writes, or hot-path writes. This report
must not write production memory, must not write graph facts, must not promote
dry-run queue items into durable memory, must not alter prompt assembly, and
must not enable runtime activation. The context debug gate and preflight must
run `scripts/hepta-context-memory-formation-queue-gate.sh` after
`scripts/hepta-context-memory-formation-receipt-gate.sh` and before
`scripts/hepta-context-memory-formation-candidate-no-leak-export-gate.sh`. It
must keep `runtime-activation=disabled`.
Memory formation candidate no-leak/export guard: until an eval harness and
explicit operator-approved write path exist, any future memory formation
candidate preview surface must remain payload-dark in response-debug/export.
Inputs may contain bait keys such as `memory_formation_candidates`,
`memory_formation_candidate_previews`, `candidate_text`, `transcript_text`,
`memory_text`, `tool_args`, `raw_idempotency_key`, `idempotency_key`, source
ids, memory ids, per-source candidate lists, email-shaped strings, phone-shaped
strings, or user identifiers, but exports must ignore those raw fields and may
surface only the already-approved receipt metadata counters/classes described
above. This guard must not write production memory, must not promote preview
candidate text into durable memory, must not alter prompt assembly, and must not
enable runtime activation. The context debug gate and preflight must run
`scripts/hepta-context-memory-formation-candidate-no-leak-export-gate.sh` after
`scripts/hepta-context-memory-formation-queue-gate.sh` and before the
source-aware compression front-door report. It must keep
`runtime-activation=disabled`.
Memory temporal fact schema dry-run: recall diagnostics may expose
payload-light `memory_temporal_facts` metadata for future temporal memory
facts. Facts are dry-run metadata only and may contain controlled fact types
(`attribute`, `preference`, `task_state`, `decision`, `summary`), stable
`entity_hash`, `provenance_span_count`, `valid_from_sequence`,
`invalid_at_sequence`, `confidence_basis_points`, stable
`supersedes_fact_hash`, `privacy_class`, `dry_run_only=true`, and
`production_write=false`. They must not contain entity text, fact text,
transcript text, memory text, prompt text, source ids, replay keys, text hashes,
raw idempotency keys, memory ids, topic ids, neuron ids, query payloads, ranked
payloads, tool arguments, per-source lists, email-shaped strings, phone-shaped
strings, or user identifiers. Temporal fact integrity requires a known fact
type, positive provenance span count, stable entity hash, positive
valid-from sequence, optional invalid-at sequence strictly after valid-from,
`confidence_basis_points <= 10000`, stable optional supersedes hash, a
payload-light privacy class, `dry_run_only=true`, and
`production_write=false`. This report must not write graph facts, must not write
production memory, must not promote dry-run facts into durable memory, must not
alter prompt assembly, and must not enable runtime activation. Response-debug
export may summarize only schema version, fact types, privacy classes,
aggregate span/confidence counts, open/invalidated/supersedes counts, dry-run
count, production-write count, and invalid state; it must not export entity
hashes, supersedes hashes, or any raw payload. The context debug gate and
preflight must run `scripts/hepta-context-memory-temporal-fact-schema-gate.sh`
after `scripts/hepta-context-memory-formation-candidate-no-leak-export-gate.sh`
and before `scripts/hepta-context-memory-temporal-fact-graph-gate.sh`. It must keep
`runtime-activation=disabled`.
Memory temporal fact graph dry-run: recall diagnostics may expose a
payload-light `memory_temporal_fact_graph` topology derived from temporal fact
metadata for future temporal memory graph evaluation. Graph nodes may contain
only stable `fact_hash`, controlled fact type, provenance span count,
`valid_from_sequence`, optional `invalid_at_sequence`,
`confidence_basis_points`, `has_supersedes`, privacy class, and explicit
dry-run/side-effect booleans. Graph edges may contain only stable `edge_hash`,
controlled edge kind (`provenance`, `validity_window`, or `supersedes`), stable
from/to fact hashes, provenance span count, validity window fields, and
explicit dry-run/side-effect booleans. The graph must not contain entity hashes,
entity text, fact text, transcript text, memory text, prompt text, answer text,
source ids, replay keys, raw text hashes, raw idempotency keys, memory ids,
topic ids, neuron ids, query payloads, ranked payloads, tool arguments,
per-source lists, email-shaped strings, phone-shaped strings, or user
identifiers. Graph integrity requires schema version 1, stable fact and edge
hashes, known edge kinds, positive provenance span counts, positive
valid-from sequences, optional invalid-at sequences strictly after valid-from,
`confidence_basis_points <= 10000`, payload-light privacy classes,
`dry_run_only=true`, `production_write=false`, `graph_write=false`, no prompt
assembly change, and no runtime activation. The graph report must not write
graph facts, must not write production memory, must not promote dry-run facts
into durable memory, must not alter prompt assembly, and must not enable
runtime activation. The context debug gate and preflight must run
`scripts/hepta-context-memory-temporal-fact-graph-gate.sh` after
`scripts/hepta-context-memory-temporal-fact-schema-gate.sh` and before
`scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh`. It must keep
`runtime-activation=disabled`.
Temporal graph shadow eval: recall diagnostics may expose an offline,
behavior-neutral deterministic-shadow scoreboard for temporal fact graph
topology. The report may contain only fixed metric names (`node_coverage`,
`edge_coverage`, `validity_window_coverage`, `supersedes_coverage`, `latency`,
and `regret`), controlled fixture kinds (`topology_coverage`,
`validity_window_replay`, `supersedes_replay`, and `regression_guard`), stable
fixture hashes, aggregate temporal fact counts, graph node and edge counts,
expected/observed validity-window and supersedes edge counts, coverage basis
points, latency milliseconds and latency budget, regret basis points, a blocked
regression fixture, fixed threshold labels
(`node-coverage-floor-basis-points`, `edge-coverage-floor-basis-points`,
`validity-window-floor-basis-points`, `supersedes-floor-basis-points`,
`latency-max-ms`, and `regret-max-basis-points`), and explicit side-effect
booleans. It must not contain entity text, fact text, transcript text, memory
text, prompt text, answer text, query payloads, raw graph payloads, entity
hashes, fact hashes, edge hashes, source ids, session ids, memory ids, trace
ids, tool arguments, tool outputs, raw fact/entity values, email-shaped
strings, phone-shaped strings, operator identity, or user identifiers. Shadow
integrity requires schema version 1, `deterministic-shadow` mode, exactly four
fixtures, three positive fixtures, one negative regression fixture, minimum
positive node/edge/validity-window/supersedes coverage of 10000 basis points,
maximum positive latency 47 ms, zero positive regret, and the regression
fixture blocked. It must not write production memory, must not write graph
facts, must not alter prompt assembly, must not enable runtime activation, must
not enable a production route, and must not allow operator activation. The
Rust-backed fixture is `ContextMemoryTemporalGraphShadowEvalReport` in
`codex-rs/hepta-core/src/memory/eval_harness/temporal_graph_shadow.rs`,
exposed through `context_memory_temporal_graph_shadow_eval_report` on both
`StoreSnapshot` and `InMemoryStore`.

`scripts/hepta-context-memory-temporal-graph-shadow-eval-report.sh` emits the
payload-light scoreboard, and
`scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh` verifies the
report, Rust-backed fixture boundary, hepta-core/hepta-memory helper tests,
debug/preflight wiring, source-aware front-door static check, release manifest
entries, and no-leak constraints. The context debug gate and preflight must run
`scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh` after
`scripts/hepta-context-memory-temporal-fact-graph-gate.sh` and before
`scripts/hepta-context-memory-eval-harness-seed-gate.sh`. The gate output must
include `temporal-graph-shadow-eval=pass`,
`temporal-graph-shadow-eval.payload-light=pass`,
`temporal-graph-shadow-eval.fixtures=4`,
`temporal-graph-shadow-eval.regression-fixture=blocked`,
`temporal-graph-shadow-eval.graph-write=disabled`, and
`temporal-graph-shadow-eval.runtime-activation=disabled`.
Context memory eval harness seed: recall diagnostics may expose an offline,
behavior-neutral eval harness seed for future quality gates. The seed may
contain only fixed metric names (`recall_coverage`, `missing_critical_fact`,
`precision`, `latency`, `token_cost`, `token_saved`, `safety_leak`, and
`answer_quality_regression`), controlled fixture kinds
(`synthetic_long_session` and `redacted_trace`), stable fixture hashes, scenario
counts, critical-fact counts, recalled/missing critical-fact counts,
predicted-relevant and false-positive counts, basis-point recall/precision,
observed/budget latency milliseconds, token cost, token saved, leak/regression
counts, and explicit side-effect booleans. It must not contain prompt text, must
not contain transcript text, must not contain memory text, must not contain
answer text, and must not contain source ids, session ids, memory ids, trace ids,
query payloads, ranked payloads, tool arguments, raw fact/entity values,
email-shaped strings, phone-shaped strings, or user identifiers. Eval
Forbidden payload clauses: must not contain prompt text; must not contain transcript text; must not contain memory text; must not contain answer text.
seed integrity requires both fixture kinds, the full fixed metric set,
internally consistent recall/precision/missing-fact counts, zero safety leaks,
zero answer-quality regressions, no production memory writes, no graph writes,
no runtime activation, and no operator activation allowance. This seed must not
activate adaptive allocation, must not activate source-aware compression, must
not write graph facts, must not write production memory, must not alter prompt
assembly, and must not enable runtime activation. The context debug gate and
preflight must run `scripts/hepta-context-memory-eval-harness-seed-gate.sh`
after `scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh` and before
the source-aware compression front-door report. It must keep
`runtime-activation=disabled`. Its core implementation boundary is
`codex-rs/hepta-core/src/memory/eval_harness/eval_seed.rs`, re-exported
through `codex-rs/hepta-core/src/memory/eval_harness.rs` and
`codex-rs/hepta-core/src/memory.rs` without changing public API paths. Its
hepta-memory snapshot/store helper boundary is
`codex-rs/hepta-memory/src/context_plane_helpers.rs`, keeping the
`StoreSnapshot` and `InMemoryStore` helper method names unchanged.
Adaptive allocator eval shadow: recall diagnostics may expose an offline,
behavior-neutral shadow comparison between `current_heuristic` and
`proposed_adaptive` allocator arms using the seeded eval metrics. The shadow
report may contain only fixed metric names (`recall_coverage`,
`missing_critical_fact`, `precision`, `latency`, `token_cost`, `token_saved`,
`safety_leak`, and `answer_quality_regression`), controlled fixture kinds
(`synthetic_long_session` and `redacted_trace`), stable fixture hashes, arm
names, scenario counts, critical-fact counts, recalled/missing critical-fact
counts, predicted-relevant and false-positive counts, basis-point recall/
precision, observed/budget latency milliseconds, token cost, token saved,
leak/regression counts, a `comparison_verdict` aggregate with the
`shadow_threshold_pass` verdict plus current/proposed totals and per-metric
regression counts, and explicit side-effect booleans. It must not contain
prompt text, must not contain transcript text, must not contain memory text,
must not contain answer text, and must not contain source ids, session ids,
memory ids, trace ids, query payloads, ranked payloads, tool arguments, raw
fact/entity values, email-shaped strings, phone-shaped strings, or user
identifiers. Shadow integrity requires both allocator arms for both fixture
kinds, a `shadow_threshold_pass` comparison verdict, zero
`missing_critical_fact_regression_count`, no recall regression, no precision
regression, no latency regression, no token-cost regression, zero
`token_saved_regression_count`, no token-saved regression, zero safety leaks, zero answer-quality
regressions, no production memory writes, no graph writes, no runtime
activation, no adaptive allocator runtime activation, no source-aware runtime
activation, no prompt assembly changes, and no operator activation allowance. This shadow must not
activate adaptive allocation, must not activate source-aware compression, must
write no graph facts, must write no production memory, must not alter prompt
assembly, and must not enable runtime activation. The context debug gate and
preflight must run
`scripts/hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh` after
`scripts/hepta-context-memory-eval-harness-seed-gate.sh` and before the
source-aware compression front-door report. It must keep
`runtime-activation=disabled`. Its core implementation boundary is
`codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow.rs`, with
allocator result, comparison, and report contracts split into
`codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/result.rs`,
`codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/comparison.rs`,
and
`codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/report.rs`,
re-exported through `codex-rs/hepta-core/src/memory/eval_harness.rs` and
`codex-rs/hepta-core/src/memory.rs` without changing public API paths. Its
hepta-memory snapshot/store helper boundary is
`codex-rs/hepta-memory/src/context_plane_helpers.rs`, keeping the
`StoreSnapshot` and `InMemoryStore` helper method names unchanged.
Threshold phrases that gates must keep visible:
- no recall regression
- no precision regression
- no latency regression
- no token-cost regression
- no token-saved regression
- zero safety leaks
- zero answer-quality regressions
Context memory recall quality gate: recall diagnostics may expose an offline,
behavior-neutral quality verdict derived from the adaptive allocator shadow
results. The quality gate may contain only aggregate metric counts, fixture
counts, a payload-light `fixture_matrix`, per-fixture controlled fixture kinds,
per-fixture scenario counts, per-fixture current/proposed missing-critical-fact
counts, per-fixture proposed critical-fact/recalled/predicted-relevant and
false-positive counts, per-fixture current/proposed recall and precision basis
points, per-fixture `gate_pass`/`blocked` verdicts, per-fixture
missing-critical-fact/recall/precision regression booleans, a payload-light
per-fixture `blocking_reasons` vector with only controlled reason values
(`missing_critical_fact_regression`, `recall_coverage_regression`,
`precision_regression`, `safety_leak`, `answer_quality_regression`, and
`side_effect_flag_enabled`), aggregate
critical-fact totals, recalled/missing critical-fact totals,
predicted-relevant and false-positive totals, observed recall/precision basis
points, the explicit recall and precision floors, the explicit missing critical
fact limit, fixture pass/blocked counts, aggregate missing-critical-fact,
recall, and precision regression counts, `blocking_reason_count`, safety leak count, answer-quality
regression count, the controlled `gate_pass` verdict, and explicit side-effect
booleans. It must not contain fixture hashes, prompt text, transcript text,
memory text, answer text, source ids, session ids, memory ids, trace ids, query
payloads, ranked payloads, tool arguments, raw fact/entity values,
email-shaped strings, phone-shaped strings, or user identifiers. Gate integrity
requires schema version 2, the fixed metric count, both seeded fixture kinds represented exactly once
in the `fixture_matrix`, with minimum recall coverage 7000 basis points,
minimum precision 7000 basis points, and missing critical fact limit 2.
Gate integrity also requires internally consistent per-fixture and
aggregate missing-fact and precision counts, fixture pass count equal to fixture count,
zero fixture blocked count, zero blocking reason count, zero missing-critical-fact regressions,
zero recall regressions, zero precision regressions, zero safety leaks, zero
answer-quality regressions, no production memory writes, no graph writes, no
runtime activation, no adaptive allocator runtime activation, no source-aware
runtime activation, no prompt assembly changes, and no operator activation
allowance. This gate must not activate
adaptive allocation, must not activate source-aware compression, must not write
graph facts, must not write production memory, must not alter prompt assembly,
and must not enable runtime activation. Its core implementation boundary is
`codex-rs/hepta-core/src/memory/recall_quality_gate.rs`, with per-fixture
blocking reason and row logic in
`codex-rs/hepta-core/src/memory/recall_quality_gate/fixture.rs` and aggregate
gate report logic in
`codex-rs/hepta-core/src/memory/recall_quality_gate/report.rs`, re-exported from
`codex-rs/hepta-core/src/memory.rs` without changing public API paths. Its
hepta-memory snapshot/store helper boundary is
`codex-rs/hepta-memory/src/context_plane_helpers.rs`, keeping the
`StoreSnapshot` and `InMemoryStore` helper method names unchanged. The context debug gate and preflight
must run `scripts/hepta-context-memory-recall-quality-gate.sh` after
`scripts/hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh` and
before `scripts/hepta-context-plane-status-report-gate.sh`. It must keep
`runtime-activation=disabled`.
Ranked recall shadow eval: recall diagnostics may expose an offline,
behavior-neutral deterministic-shadow scoreboard for ranked recall output. The
report may contain only fixed metric names (`recall`, `precision`,
`token_saved`, `latency`, and `regret`), controlled fixture kinds
(`query_match`, `recency_tie_break`, `budget_pressure`, and
`regression_guard`), stable fixture hashes, ranked item counts, expected/
recalled/predicted relevant counts, false-positive counts, recall and precision
basis points, baseline/ranked token counts, token-saved counts and basis
points, latency milliseconds and latency budget, regret basis points, a blocked
regression fixture, fixed threshold labels (`recall-floor-basis-points`,
`precision-floor-basis-points`, `token-saved-min-basis-points`,
`latency-max-ms`, and `regret-max-basis-points`), and explicit side-effect
booleans. It must not contain prompt text, transcript text, memory text, answer
text, query payloads, ranked payloads, raw ranked payloads, rank explanations,
score reasons, source ids, session ids, memory ids, trace ids, tool arguments,
tool outputs, raw fact/entity values, email-shaped strings, phone-shaped
strings, or user identifiers. Shadow integrity requires schema version 1,
`deterministic-shadow` mode, exactly four fixtures, three positive fixtures,
one negative regression fixture, ranked item counts on every fixture, minimum
positive recall and precision of 8000 basis points, total positive
token-saved count 2140, maximum positive latency 55 ms, zero positive regret,
and the regression fixture blocked. It must not write production memory, must
not write graph facts, must not alter prompt assembly, must not enable runtime
activation, must not enable a production route, and must not allow operator
activation. The Rust-backed fixture is
`ContextMemoryRankedRecallShadowEvalReport` in
`codex-rs/hepta-core/src/memory/eval_harness/ranked_recall_shadow.rs`, exposed
through `context_memory_ranked_recall_shadow_eval_report` on both
`StoreSnapshot` and `InMemoryStore`.

`scripts/hepta-context-memory-ranked-recall-shadow-eval-report.sh` emits the
payload-light scoreboard, and
`scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh` verifies the
report, Rust-backed fixture boundary, hepta-core/hepta-memory helper tests,
debug/preflight wiring, source-aware front-door static check, release manifest
entries, and no-leak constraints. The context debug gate and preflight must run
`scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh` after
`scripts/hepta-context-memory-recall-quality-gate.sh` and before
`scripts/hepta-context-plane-status-report-gate.sh`. The gate output must
include `ranked-recall-shadow-eval=pass`,
`ranked-recall-shadow-eval.payload-light=pass`,
`ranked-recall-shadow-eval.fixtures=4`,
`ranked-recall-shadow-eval.regression-fixture=blocked`, and
`ranked-recall-shadow-eval.runtime-activation=disabled`.

MemoryProvider boundary: runtime-facing recall providers must own query,
`update_context`, `report`, and `clear` attempts through a single typed
provider contract instead of letting callers scatter recall injection logic.
The Rust contract is `MemoryProvider` plus
`MemoryProviderContextUpdateEnvelope`, `MemoryProviderReport`, and
`MemoryProviderClearReport` in `codex-rs/hepta-core/src/memory/provider_plane.rs`,
re-exported from `hepta_core::memory`. The initial provider implementation is
shadow-only: `update_context` may summarize returned source counts, limit
pressure, ranked item counts, selected item counts, and estimated token budget,
but it must not export prompt text, query text, transcript payloads, memory
payloads, ranked item payloads, source ids, session ids, memory ids, trace ids,
tool arguments, tool outputs, or user identifiers. The envelope must keep
`payload_light=true`, `operator_approval_required=true`,
`prompt_payload_exported=false`, `query_payload_exported=false`,
`ranked_payload_exported=false`, `write_performed=false`, and
`runtime_activation=false`.

Provider `clear` is likewise constrained until an explicit activation design
exists. The reference `hepta-memory` provider must return either a dry-run clear
report or a blocked clear report, with `clear_performed=false`,
`affected_record_count=0`, `prompt_payload_exported=false`,
`write_performed=false`, and `runtime_activation=false`. The provider report
must pair the builtin descriptor with the compact update envelope and preserve
context fencing plus provenance requirements. The boundary is covered by
focused `hepta-core` provider trait tests and `hepta-memory` reference-provider
payload-light/no-mutation tests; it does not enable a production route or alter
prompt assembly.
`scripts/hepta-context-memory-provider-boundary-report.sh` must emit a
payload-light fixed report with `memory-provider-boundary=pass`,
`memory-provider-boundary.payload-light=pass`,
`memory-provider-boundary.update-context=guarded-envelope`,
`memory-provider-boundary.clear=dry-run-or-blocked`, and
`memory-provider-boundary.runtime-activation=disabled`.
`scripts/hepta-context-memory-provider-boundary-gate.sh` must run the focused
provider tests and must be wired into debug/preflight after
`scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh` and before
`scripts/hepta-context-plane-status-report-gate.sh`.

Memory shadow regression dashboard: recall diagnostics may expose a
payload-light shadow-only dashboard that aggregates the ranked recall shadow,
temporal graph shadow, recall quality, and provider boundary reports before any
status-plane promotion. The Rust report is
`ContextMemoryShadowRegressionDashboardReport` in
`codex-rs/hepta-core/src/memory/eval_harness/shadow_regression_dashboard.rs`,
re-exported through `codex-rs/hepta-core/src/memory/eval_harness.rs` and
`codex-rs/hepta-core/src/memory.rs`, and exposed through
`context_memory_shadow_regression_dashboard_report` on both `StoreSnapshot` and
`InMemoryStore`. The report may contain only schema/mode, `input_report_count`,
`input_report_pass_count`, `regression_blocking_count`, aggregate ranked recall
fixture counts and thresholds, aggregate temporal graph fixture counts and
thresholds, recall-quality blocker/regression counts, provider-boundary
payload-light booleans such as `provider_payload_light`, compact provider item
and token counts, and explicit side-effect booleans. It must not contain prompt
text, query text, transcript text, memory text, answer text, ranked payloads,
raw ranked payloads, graph payloads, raw graph payloads, source ids, session
ids, memory ids, trace ids, tool arguments, tool outputs, operator identity,
email-shaped strings, phone-shaped strings, or user identifiers. Dashboard
integrity requires schema version 1, shadow-only mode, exactly four input
reports, four passing input reports, zero regression blockers, ranked recall
and temporal graph regression fixtures blocked, zero recall-quality blocking
reasons, `provider_payload_light=true`, operator approval required, no
production route, no production memory write, no graph write, no prompt
assembly change, no runtime activation, and no operator activation allowance.
`scripts/hepta-context-memory-shadow-regression-dashboard-report.sh` must emit
`memory-shadow-regression-dashboard=pass`,
`memory-shadow-regression-dashboard.payload-light=pass`,
`memory-shadow-regression-dashboard.input-report-pass-count=4`,
`memory-shadow-regression-dashboard.regression-blocking-count=0`, and
`memory-shadow-regression-dashboard.runtime-activation=disabled`.
`scripts/hepta-context-memory-shadow-regression-dashboard-gate.sh` must verify
the Rust-backed report, helper tests, release manifest, front-door static
contract, debug/preflight wiring, and no-leak constraints. The context debug
gate and preflight must run it after
`scripts/hepta-context-memory-provider-boundary-gate.sh` and before
`scripts/hepta-context-plane-status-report-gate.sh`.

Context Plane status/export report: recall diagnostics may expose a unified,
payload-light operator status surface that stitches together the source
registry, adaptive budget allocation dry-run, memory taxonomy, memory formation
receipts, memory formation queue, temporal facts, temporal fact graph, eval
harness seed, adaptive allocator eval shadow, recall quality gate, memory
provider boundary, and source-aware front-door readiness. The
machine-readable report is
`context-plane-status=pass` plus fixed allowlisted `context-plane-status.*`
keys only. The Rust status sections are `source_registry`,
`adaptive_budget_allocation`, `memory_taxonomy`,
`memory_formation_receipts`, `memory_formation_queue`, `memory_temporal_facts`,
`memory_temporal_fact_graph`, `memory_temporal_graph_shadow_eval`,
`eval_harness_seed`, `adaptive_allocator_eval_shadow`, `recall_quality_gate`,
`memory_provider_boundary`, and `source_aware_front_door`. Section states may be
`ready`, `shadow`, `disabled`, or `blocked`; they may carry only counts and
side-effect booleans. The `memory_temporal_graph_shadow_eval` row is
shadow-only until a separately approved graph route is promoted. The
`memory_provider_boundary` row is shadow-only until a separately approved
provider route is promoted. The
`recall_quality_gate` status row may additionally carry
`recall_quality_blocking_reason_count` and
`recall_quality_blocking_reasons`, but those reasons must be controlled
recall-quality blocker enums only:
`missing_critical_fact_regression`, `recall_coverage_regression`,
`precision_regression`, `safety_leak`, `answer_quality_regression`, and
`side_effect_flag_enabled`. Gate-pass status must export
`context-plane-status.memory-temporal-graph-shadow-eval=shadow`,
`context-plane-status.memory-provider-boundary=shadow`,
`context-plane-status.recall-quality-blocking-reason-count=0` and
`context-plane-status.recall-quality-blocking-reasons=none`. The report
must not
contain transcript text, must not contain memory text, and must not contain
answer text. It must not expose source ids, session ids, memory ids, trace ids,
query payloads, ranked payloads, tool arguments, raw fact/entity values,
email-shaped strings, phone-shaped strings, user identifiers, transcript spans,
candidate text, entity hashes, fact hashes, edge hashes, supersedes hashes,
fixture hashes, or idempotency hashes. Status integrity requires all thirteen
sections, no production
memory writes, no graph writes, no runtime activation, no adaptive allocator
runtime activation, no source-aware runtime activation, no prompt assembly
changes, and no operator activation allowance. This status surface is
observational only: no production memory writes, no graph writes, no runtime
activation, no adaptive allocator runtime activation, no source-aware runtime
activation, no prompt assembly changes, and no operator activation allowance.
The context debug gate and preflight must run
`scripts/hepta-context-plane-status-report-gate.sh` after
`scripts/hepta-context-memory-recall-quality-gate.sh` and before
the source-aware compression front-door report. The allowlisted shell export is
`scripts/hepta-context-plane-status-report.sh`, and it must keep
`runtime-activation=disabled`. Its hepta-memory snapshot/store helper boundary
is `codex-rs/hepta-memory/src/context_plane_helpers.rs`, keeping the
`StoreSnapshot` and `InMemoryStore` helper method names unchanged.
Context Plane activation blocker matrix: recall diagnostics may also expose an
observational activation-readiness threshold matrix derived from the Context
Plane status report. The machine-readable report is
`context-plane-activation-blockers=pass` plus fixed allowlisted
`context-plane-activation-blockers.*` keys only. Matrix targets are
`source_registry`, `adaptive_budget_allocation`, `memory_taxonomy`,
`memory_formation_receipts`, `memory_formation_queue`,
`memory_temporal_facts`, `memory_temporal_fact_graph`,
`memory_temporal_graph_shadow_eval`, `eval_harness_seed`,
`adaptive_allocator_eval_shadow`, `recall_quality_gate`,
`memory_provider_boundary`, `source_aware_front_door`, and
`operator_approval`. The current blocker reasons
are controlled enum values:
`adaptive_budget_allocation_shadow_only`,
`temporal_graph_shadow_eval_shadow_only`,
`memory_provider_boundary_shadow_only`, `source_aware_front_door_disabled`,
`operator_approval_missing`, and `side_effect_flag_enabled`; future reasons must
be added to the enum and gate before export. Matrix rows may carry only
target/status/required-status taxonomy, threshold booleans, blocker reason
enums, counts, and explicit side-effect booleans. A status input carrying any
production-write, graph-write, runtime-activation, adaptive-allocator runtime,
source-aware runtime, prompt-assembly, or operator-activation side-effect flag
must produce a `side_effect_flag_enabled` blocker without propagating enabled
side-effect booleans into the exported matrix. The `recall_quality_gate` row may
also carry `recall_quality_blocking_reason_count` and
`recall_quality_blocking_reasons` copied from the status report, but only as the
controlled recall-quality blocker enums `missing_critical_fact_regression`,
`recall_coverage_regression`, `precision_regression`, `safety_leak`,
`answer_quality_regression`, and `side_effect_flag_enabled`. Gate-pass
activation matrix export must include
`context-plane-activation-blockers.memory-temporal-graph-shadow-eval=blocked:temporal_graph_shadow_eval_shadow_only`,
`context-plane-activation-blockers.memory-provider-boundary=blocked:memory_provider_boundary_shadow_only`,
`context-plane-activation-blockers.recall-quality-blocking-reason-count=0` and
`context-plane-activation-blockers.recall-quality-blocking-reasons=none`. The
matrix must not contain
prompt text, must not contain transcript text, must not contain memory text, and
must not contain answer text.
It must not expose source ids, session ids, memory ids, trace ids, query
payloads, ranked payloads, tool arguments, raw fact/entity values, email-shaped
strings, phone-shaped strings, user identifiers, transcript spans, candidate
text, entity hashes, fact hashes, edge hashes, supersedes hashes, fixture hashes,
or idempotency hashes. Matrix integrity requires all fourteen targets, exact blocker counts, no production
memory writes, no graph writes, no runtime activation, no adaptive allocator
runtime activation, no source-aware runtime activation, no prompt assembly
changes, no operator activation allowance, and `activation_allowed=false`. This
matrix is explanatory only and must not promote shadow/disabled sections into
runtime behavior. The context debug gate and preflight must run
`scripts/hepta-context-plane-activation-blocker-matrix-gate.sh` after
`scripts/hepta-context-plane-status-report-gate.sh` and before the source-aware
compression front-door report. The allowlisted shell export is
`scripts/hepta-context-plane-activation-blocker-matrix-report.sh`, and it must
keep `runtime-activation=disabled`. Its hepta-memory snapshot/store helper
boundary is `codex-rs/hepta-memory/src/context_plane_helpers.rs`, keeping the
`StoreSnapshot` and `InMemoryStore` helper method names unchanged.
Context Plane operator approval packet dry-run: recall diagnostics may expose a
payload-light approval readiness receipt derived from the activation blocker
matrix. The machine-readable report is
`context-plane-operator-approval-packet=pass` plus fixed allowlisted
`context-plane-operator-approval-packet.*` keys only. The packet may carry
`approval_required`, `dry_run_only`, `activation_command_present=false`, matrix
row counts, blocker reason counts, threshold snapshot, required approval scopes,
and explicit side-effect booleans. The approval packet matrix row counts are
payload-light counters only. Required approval scopes are controlled enum values:
`adaptive_budget_allocation_runtime`,
`source_aware_runtime_activation`, `production_memory_write`, `graph_write`,
`prompt_assembly_change`, and `operator_activation`. Current blocker reason
counts may include only the controlled activation blocker enum values
`adaptive_budget_allocation_shadow_only`,
`temporal_graph_shadow_eval_shadow_only`,
`memory_provider_boundary_shadow_only`, `source_aware_front_door_disabled`,
`operator_approval_missing`, and `side_effect_flag_enabled` until new reasons are
added to the Rust enum and gate. The packet may also carry
`recall_quality_blocking_reason_count` and
`recall_quality_blocking_reason_counts`, aggregated only from
`recall_quality_gate` matrix rows and only as no-payload counts of the
controlled recall-quality blocker enums `missing_critical_fact_regression`,
`recall_coverage_regression`, `precision_regression`, `safety_leak`,
`answer_quality_regression`, and `side_effect_flag_enabled`. Gate-pass operator
approval export must include
`context-plane-operator-approval-packet.blocker.temporal-graph-shadow-eval-shadow-only=1`,
`context-plane-operator-approval-packet.blocker.memory-provider-boundary-shadow-only=1`,
`context-plane-operator-approval-packet.recall-quality-blocking-reason-count=0`
and
`context-plane-operator-approval-packet.recall-quality-blocking-reasons=none`.
The packet must not contain prompt text, must not contain transcript text,
must not contain memory text, and must not contain answer text. It must not
expose source ids, session ids, memory ids, trace ids, query payloads, ranked
payloads, tool arguments, raw fact/entity values, email-shaped strings,
phone-shaped strings, user identifiers, transcript spans, candidate text, entity
hashes, supersedes hashes, fixture hashes, or idempotency hashes. The packet
must not include activation commands or any command-shaped field that could be
executed by an operator path. Packet integrity requires all fourteen matrix rows,
exact threshold counts, exact blocker reason counts, all required approval
scopes, no production memory writes, no graph writes, no runtime activation, no
adaptive allocator runtime activation, no source-aware runtime activation, no
prompt assembly changes, no operator activation allowance, and dry-run-only
export.
This approval packet is explanatory only and must not promote shadow/disabled
sections into runtime behavior. The context debug gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-gate.sh` after
`scripts/hepta-context-plane-activation-blocker-matrix-gate.sh` and before the
source-aware compression front-door report. The allowlisted shell export is
`scripts/hepta-context-plane-operator-approval-packet-report.sh`, and it must
keep `runtime-activation=disabled`. Its hepta-memory snapshot/store helper
boundary is `codex-rs/hepta-memory/src/context_plane_helpers.rs`, keeping the
`StoreSnapshot` and `InMemoryStore` helper method names unchanged.
Context Plane operator approval packet no-activation-command negative export
guard: malformed or activation-shaped operator approval packet inputs must be
rejected or kept out of every exported report. The approval packet serde
contract must reject unknown fields such as `activation_command`, `tool_args`,
`raw_payload`, raw operator identifiers, source ids, session ids, memory ids,
trace ids, raw fact/entity values, email-shaped strings, phone-shaped strings,
and other PII-shaped values. Known side-effect booleans such as
`activation_command_present=true`, `production_write=true`, `graph_write=true`,
`runtime_activation=true`, `adaptive_allocator_runtime_activation=true`,
`source_aware_runtime_activation=true`, `prompt_assembly_change=true`, or
`operator_activation_allowed=true` may parse only as invalid packets and must
fail packet integrity. The machine-readable negative guard is
`context-plane-operator-approval-packet-negative-export=pass`; it must prove the
allowlisted approval packet report still exports only counted readiness metadata
with `activation-command=absent` and all write/runtime/operator activation keys
disabled. It must not contain activation commands, command-shaped fields, raw
payloads, prompt text, transcript text, memory text, answer text, source ids,
session ids, memory ids, trace ids, query payloads, ranked payloads, tool
arguments, raw fact/entity values, transcript spans, candidate text, entity
hashes, supersedes hashes, fixture hashes, idempotency hashes, email-shaped
strings, phone-shaped strings, user identifiers, or PII-shaped values. The
context debug gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-negative-export-gate.sh`
after `scripts/hepta-context-plane-operator-approval-packet-gate.sh` and before
the source-aware compression front-door report. This guard is observational
only; it must not activate adaptive allocation, must not activate source-aware
compression, must not write graph facts, must not write production memory, must
not alter prompt assembly, and must not enable operator activation.
The allowlisted shell export for the negative guard is
`scripts/hepta-context-plane-operator-approval-packet-negative-export-report.sh`.
Context Plane operator approval packet canonical export digest: recall
diagnostics may expose a deterministic digest over the allowlisted operator
approval packet report and the allowlisted negative export report. The
machine-readable digest report is
`context-plane-operator-approval-packet-canonical-export-digest=pass` plus fixed
allowlisted `context-plane-operator-approval-packet-canonical-export-digest.*`
keys only. It may carry only schema version, canonical line counts, SHA-256
digests for the approval report, negative export report, and combined report,
plus explicit disabled runtime/operator activation booleans. Current canonical
line counts are approval report 31 lines, negative export report 4 lines, and
combined report 35 lines. The digest report must be deterministic and idempotent:
two consecutive runs over unchanged inputs must be byte-for-byte equal. It must not contain activation commands, command-shaped fields, raw
payloads, prompt text, transcript text, memory text, answer text, source ids,
session ids, memory ids, trace ids, query payloads, ranked payloads, tool
arguments, raw fact/entity values, transcript spans, candidate text, entity
hashes, supersedes hashes, fixture hashes, idempotency hashes, email-shaped
strings, phone-shaped strings, user identifiers, or PII-shaped values. The
context debug gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh`
after `scripts/hepta-context-plane-operator-approval-packet-negative-export-gate.sh`
and before the source-aware compression front-door report. The allowlisted shell
export is
`scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-report.sh`.
This digest is observational only; it must not activate adaptive allocation,
must not activate source-aware compression, must not write graph facts, must not
write production memory, must not alter prompt assembly, and must not enable runtime or operator activation.
Context Plane operator approval packet digest tamper fixture negative matrix:
recall diagnostics must prove the canonical digest/no-payload guard rejects
malformed canonical export variants. The machine-readable matrix is
`context-plane-operator-approval-packet-digest-tamper-matrix=pass` and must
cover line-order tamper, line-count tamper, digest-value tamper,
activation-command injection, raw-payload injection, PII-shaped value injection,
and write/activation flag injection. Each fixture must fail the same
line-count, SHA-256, and no-payload guard used by the canonical digest report;
no fixture may be accepted as a valid approval packet, negative export, digest
report, activation command, write route, or operator approval route. The matrix
must not contain prompt text, transcript text, memory text, answer text, source
ids, session ids, memory ids, trace ids, query payloads, ranked payloads, tool
arguments, raw payloads, raw fact/entity values, transcript spans, candidate
text, entity hashes, supersedes hashes, fixture hashes, idempotency hashes,
email-shaped strings, phone-shaped strings, user identifiers, or PII-shaped
values in its allowlisted output. The context debug gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh`
after
`scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh`
and before the source-aware compression front-door report. This matrix is
observational only; it must not activate adaptive allocation, must not activate
source-aware compression, must not write graph facts, must not write production
memory, must not alter prompt assembly, and must not enable runtime or operator
activation.
Context Plane operator approval packet freshness/staleness replay-protection dry-run:
recall diagnostics must bind the current operator approval packet
readiness export to a payload-light freshness record so stale, expired, future,
or replayed readiness packets cannot be mistaken for current readiness. The
machine-readable report is
`context-plane-operator-approval-packet-freshness=pass` with fixed allowlisted
`context-plane-operator-approval-packet-freshness.*` keys only. It may carry
only schema version, source canonical digest report line count, source canonical
digest report SHA-256, `approval-readiness-sequence=273`,
`current-readiness-sequence=273`, `expires-after-sequence=274`,
`max-replay-age-sequences=0`, and explicit negative decisions
`stale-sequence=reject`, `expired-sequence=reject`, `future-sequence=reject`,
and `digest-replay=reject`, plus disabled runtime/operator activation
booleans. The freshness/staleness/replay guard must reject stale sequence,
expired sequence, future sequence, replayed digest, line-count mutation,
activation-command injection, and write/activation flag injection fixtures. The
report and fixture output must not contain prompt text, transcript text, memory
text, answer text, source ids, session ids, memory ids, trace ids, query
payloads, ranked payloads, tool arguments, raw payloads, raw fact/entity values,
transcript spans, candidate text, entity hashes, supersedes hashes, fixture
hashes, idempotency hashes, email-shaped strings, phone-shaped strings, user
identifiers, or PII-shaped values in its allowlisted output. The context debug
gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-freshness-gate.sh` after
`scripts/hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh`
and before the source-aware compression front-door report. The allowlisted
shell export is
`scripts/hepta-context-plane-operator-approval-packet-freshness-report.sh`.
This dry-run is observational only; it must not activate adaptive allocation,
must not activate source-aware compression, must not write graph facts, must not
write production memory, must not alter prompt assembly, and must not enable
runtime or operator activation.
Context Plane operator approval packet freshness dependency-chain stale-source negative matrix:
recall diagnostics must bind the freshness/staleness replay-protection dry-run
to its upstream approval packet report dependency, negative export report
dependency, canonical digest report dependency, tamper matrix report dependency,
and freshness report dependency so mixed-generation or stale-source reports
cannot be recombined into current readiness. The machine-readable report is
The dependency-chain record includes approval report dependency, negative export report dependency, canonical digest report dependency, tamper matrix report dependency, and freshness report dependency.
`context-plane-operator-approval-packet-freshness-dependency-chain=pass` with
fixed allowlisted `context-plane-operator-approval-packet-freshness-dependency-chain.*`
keys only. It may carry only schema version, upstream line counts and SHA-256
digests, `readiness-chain-generation=274`, `freshness-source-sequence=273`, and
explicit negative decisions `stale-source=reject`, `mixed-generation=reject`,
`source-digest-mismatch=reject`, and `tamper-matrix-replay=reject`, plus
disabled runtime/operator activation booleans. The dependency-chain guard must
reject stale approval packet source, stale negative export source, stale
canonical digest source, tamper matrix replay source, freshness source digest
mutation, mixed generation, mixed freshness sequence, line-count mutation,
activation-command injection, and write/activation flag injection fixtures. The
report and fixture output must not contain prompt text, transcript text, memory
text, answer text, source ids, session ids, memory ids, trace ids, query
payloads, ranked payloads, tool arguments, raw payloads, raw fact/entity values,
transcript spans, candidate text, entity hashes, supersedes hashes, fixture
hashes, idempotency hashes, email-shaped strings, phone-shaped strings, user
identifiers, or PII-shaped values in its allowlisted output. The context debug
gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh`
after
`scripts/hepta-context-plane-operator-approval-packet-freshness-gate.sh` and
before the source-aware compression front-door report. The allowlisted shell
export is
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-report.sh`.
This dependency-chain dry-run is observational only; it must not activate
adaptive allocation, must not activate source-aware compression, must not write
graph facts, must not write production memory, must not alter prompt assembly,
and must not enable runtime or operator activation.
Context Plane operator approval packet freshness dependency-chain canonical digest mixed-source tamper matrix:
recall diagnostics must seal the P0-274 freshness dependency-chain export with a
deterministic canonical digest before any later readiness/front-door stage can
consume it. The machine-readable report is
`context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass`
with fixed allowlisted
`context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.*`
keys only. It may carry only schema version, dependency-chain report 20 lines,
dependency-chain report SHA-256, `readiness-chain-generation=275`,
`source-readiness-chain-generation=274`, `source-freshness-sequence=273`, and
explicit negative decisions for reordered dependency rows, mismatched upstream
digests, mixed generation/sequence replay windows, injected activation/write/payload fields,
plus disabled runtime/operator activation
booleans. The dependency-chain canonical digest guard must reject reordered
dependency rows, mismatched upstream digests, mixed generation replay, mixed
sequence replay, dependency-chain digest mutation, payload field injection,
activation-command injection, and write/activation flag injection fixtures.
The report and fixture output must not contain prompt text, transcript text,
memory text, answer text, source ids, session ids, memory ids, trace ids, query
payloads, ranked payloads, tool arguments, raw payloads, raw fact/entity values,
transcript spans, candidate text, entity hashes, supersedes hashes, fixture
hashes, idempotency hashes, email-shaped strings, phone-shaped strings, user
identifiers, or PII-shaped values in its allowlisted output. The context debug
gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh`
after
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh`
and before the source-aware compression front-door report. The allowlisted shell
export is
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh`.
This dependency-chain canonical digest dry-run is observational only; it must
not activate adaptive allocation, must not activate source-aware compression,
must not write graph facts, must not write production memory, must not alter
prompt assembly, and must not enable runtime or operator activation.
Context Plane operator approval packet freshness dependency-chain expiry/readiness-window drift guard:
recall diagnostics must bind the dependency-chain canonical digest to a
payload-light readiness-window record so expired windows, sequence drift, and
digest replay cannot be mistaken for current readiness. The machine-readable
report is
`context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift=pass`
with fixed allowlisted
`context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.*`
keys only. It may carry only schema version, source canonical digest report 15 lines,
source canonical digest report SHA-256, `readiness-chain-generation=276`,
`source-readiness-chain-generation=275`, `source-dependency-chain-generation=274`,
`source-freshness-sequence=273`, `readiness-window-start-sequence=273`,
`readiness-window-current-sequence=276`,
`readiness-window-expires-after-sequence=277`,
`readiness-window-max-drift-sequences=0`, and explicit negative decisions
`expired-window=reject`, `window-start-drift=reject`,
`window-current-drift=reject`, `window-expiry-drift=reject`, and
`source-digest-replay=reject`, plus payload/write injection rejection and
disabled runtime/operator activation booleans. The expiry/readiness-window
drift guard must reject source digest replay, source generation drift, source
dependency generation drift, source freshness sequence drift, readiness window
start/current/expiry drift, expired-window decision mutation, payload field
injection, activation-command injection, and write/activation flag injection
fixtures. The report and fixture output must not contain prompt text,
transcript text, memory text, answer text, source ids, session ids, memory ids,
trace ids, query payloads, ranked payloads, tool arguments, raw payloads, raw
fact/entity values, transcript spans, candidate text, entity hashes, supersedes
hashes, fixture hashes, idempotency hashes, email-shaped strings, phone-shaped
strings, user identifiers, or PII-shaped values in its allowlisted output. The
context debug gate and preflight must run
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh`
after
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh`
and before the source-aware compression front-door report. The allowlisted shell
export is
`scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-report.sh`.
This expiry/readiness-window drift dry-run is observational only; it must not
activate adaptive allocation, must not activate source-aware compression, must
not write graph facts, must not write production memory, must not alter prompt
assembly, and must not enable runtime or operator activation.
Activation/write clauses: must not activate adaptive allocation; must not activate source-aware compression; must not write graph facts; must not write production memory; must not alter prompt assembly; must not enable runtime activation.
Steady-state settings diffs must also represent disappearance, not only
appearance. In particular, when previously visible collaboration-mode developer
instructions are cleared, the diff must emit a `collaboration_mode` source entry
that tells the model not to keep applying the prior collaboration-mode guidance.
Likewise, when persisted user or developer instructions disappear, the diff must
emit `user_instructions` or `developer_instructions` source entries that
explicitly clear the prior model-visible guidance.
When persisted `model_switch` guidance disappears because the current model has
no model-specific switch instructions, the diff must emit a `model_switch`
source clear and must stay quiet after that clear hash is already persisted.
Capability inventory sources (`apps`, `available_skills`, and
`available_plugins`) must compare the previous manifest hash against the next
rendered inventory. When an inventory disappears, the diff must emit a matching
source clear entry; when that clear entry is already the persisted manifest hash,
later no-inventory turns must stay quiet instead of repeating the clear.
Extension prompt fragment sources (`extension_developer_policy`,
`extension_developer_capabilities`, `extension_separate_developer`, and
`extension_contextual_user`) must be marker-wrapped before entering model
history so manifest classification does not collapse them into generic
developer or contextual-user text. Settings diffs must compare ordered
per-source manifest hash lists against the next rendered extension fragment
list, emit source-specific replacements or clears when they change or disappear,
and stay quiet after a persisted clear hash has already recorded disappearance.
Multi-agent usage hints must be marker-wrapped as `multi_agent_usage_hint`
developer sources before entering model history. Settings diffs must compare the
previous manifest hash against the current rendered root/subagent usage hint,
emit replacements when the guidance changes, emit a source-specific clear when
the hint disappears, and stay quiet after that clear hash is already persisted.

`text_hash` and manifest ledger hashes use the stable 16-lower-hex shape. A
manifest passes replay integrity only when its version is supported, entries have
replay identity, optional ledger hashes match, decision entries are hash-shaped,
and recall-selection counters are internally consistent.

## Recall Selection

`TurnContextRecallSelectionSummary` is the payload-light recall rollup. It may
carry source counts and quality pressure, but must not carry source ids,
summaries, memory snippets, transcript text, or ranking payloads.

The recall rollup integrity rules are:

- `selected_source_count <= returned_source_count`
- `ranked_source_count <= selected_source_count`
- `ranked_source_count <= ranked_item_count`
- nonzero `ranked_item_count` requires at least one ranked source
- `returned_unselected_source_count == returned_source_count - selected_source_count`
- `max_per_source` is the selector source-quota target when available; it is a
  payload-light policy counter and does not require exporting per-source item
  counts
- `omitted_by_budget_count` counts ranked items omitted by item budget or source
  quota selection pressure when available
- `memory_control_omitted_count` counts memory candidates omitted because they
  are explicit recall-control records when available
- low-trust and low-recency ranked item counts cannot exceed `ranked_item_count`
- when `source_diversity_target > 0`, `source_diversity_met` must match whether
  selected sources satisfy that target

Source diversity is judged by selected sources, not ranked sources/items. A valid
unranked selection may have selected sources, zero ranked sources, zero ranked
items, and still satisfy source diversity.

## Runtime Recall Quality

Runtime ranked recall debug surfaces must keep low-quality pressure visible
without exporting ranked payloads. The current runtime quality thresholds are:

- low-trust ranked item: `score.confidence < 0.50`
- low-recency ranked item: `score.recency < 0.50`

`RuntimeContextRecallSlice`, provenance overview, and phase2 overview may expose
only payload-light counts for these states. They must not export ranked item
summaries, recalled memory text, transcript excerpts, source ids, score reasons,
or per-source item lists solely to explain low-trust / low-recency pressure.

Runtime recall summaries may also expose
`recall_memory_control_omitted_items`: the count of memory query candidates that
matched the query but were filtered because they were explicit recall-control
records. This count must not carry the control record body, marker-bearing
summary, source id, or per-source item list.

`RuntimeContextRecallProviderRollup` is the runtime/hepta-memory adapter for the
core/session shadow manifest handoff. It is built from a real
`RuntimeContextRecallSlice`, but may expose only the protocol-compatible
`recall_selection` counters: returned/selected/ranked source counts, selector
budget pressure, memory-control omission pressure, and low-quality ranked-item
pressure. It must not carry ranked summaries, recalled text, source ids, score
reasons, transcript spans, memory ids, topic ids, neuron ids, or per-source item
lists.

`RuntimeContextRecallSelectedSnippetEnvelope` is the runtime selected-snippet
readiness envelope for the core/session handoff gate. It may carry bounded,
redacted snippet text plus snippet hashes, token estimates, truncation/redaction
counters, and safety booleans. It must not carry source ids, source lanes, raw
ranked payloads, score reasons, marker-bearing control records, per-source
lists, memory ids, topic ids, neuron ids, or query payload. The envelope may feed
the core/session guarded live handoff only through the runtime-to-core protocol
conversion helper, which drops non-shadow-integrity envelopes before request
submission. Core/session must still repeat count, bounds, safety, and prompt-safe
text checks after receiving the request.

`RuntimeContextRecallTurnHandoff` is the runtime-owner in-memory package for
turn creation. It is built from one real `RuntimeContextRecallSlice` so the
payload-light provider rollup and optional selected-snippet core envelope cannot
drift across separate provider queries. The selected-snippet side must remain
`None` unless the caller is on an experimental API path and the runtime envelope
passes the same runtime-to-core shadow-integrity helper. This package is not a
response-debug, manifest export, session-log, or public API serialization
surface; its debug shape must not print snippet text, source ids, query payload,
score reasons, or raw ranked payloads.

The native runtime request assembly adapter may consume this handoff only through
an explicit experimental path. The default native/demo turn path must continue to
assemble model messages without selected snippets unless the owner asks for
`native_turn_messages_with_context_recall_handoff` or the explicit
`run_demo_turn_in_session_with_context_recall_handoff` run adapter. When opted
in, native request assembly may add one `<selected_context_recall>` section to
the system context using only bounded snippet text, stable snippet hashes, token
estimates, and redaction/truncation flags. It must drop the section if snippet
text contains prompt-unsafe metadata labels such as source ids, source lanes,
raw ranked payloads, score reasons, memory/topic/neuron ids, control markers,
query payload labels, or nested selected-context tags. The opt-in run adapter's
debug shape must expose at most provider rollup, selected-snippet presence/count,
and run metadata; it must not print model messages or snippet text.

Runtime provider rollup manifest handoff gate: the runtime/core boundary must
prove that `RuntimeContextRecallTurnHandoff` packages the payload-light provider
rollup and optional selected-snippet core envelope from the same recall slice.
The provider rollup and optional selected-snippet core envelope cannot drift
across separate provider queries. Runtime tests
`context_recall_provider_rollup_maps_runtime_recall_to_payload_light_counts`,
`context_recall_turn_handoff_packages_rollup_and_opted_in_core_snippets`, and
`native_turn_messages_with_context_recall_handoff_consumes_opted_in_runtime_handoff_without_leak`
must remain covered by
`scripts/hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh`.
Core/session must keep the combined handoff regression
`record_context_updates_and_set_reference_context_item_consumes_turn_scoped_recall_rollup_and_selected_snippets_without_drift`:
the rollup may refresh the shadow manifest ledger, the selected-snippet envelope
may add at most one guarded `<selected_context_recall>` live context section,
and subsequent rollup refreshes must not duplicate that prompt section or leak
source ids, memory ids, control markers, raw query terms, or selected-snippet
source metadata.

Worker-task callers are the first real runtime owner surface for that adapter.
Default `run_worker_task` execution must continue to use disabled context recall
handoff behavior. A worker task may invoke selected-snippet recall only through
the explicit `run_worker_task_with_context_recall_handoff` caller and
`WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved`. The opted-in
worker report may expose provider rollup and selected-snippet presence/count, but
must not serialize or debug-print snippet text, source ids, source lanes, raw
ranked payload, score reasons, memory/topic/neuron ids, control markers, query
payload labels, or the native redacted-query marker.

Worker scheduler/batch callers must preserve the same default. The default
`run_due_worker_tasks` and `run_ready_worker_tasks` APIs must continue to call
the disabled worker path and return the legacy reports. Operators may opt in only
through the explicit context-recall scheduler variants, which return context-aware
reports with aggregate selected-snippet presence/count and per-run provider
rollups. Those scheduler reports have the same no-snippet-text/no-source-id
serialization and debug constraints as the single worker task report.

The runtime operator scheduler adapter
`run_worker_scheduler_with_context_recall_operator_handoff` is the first
operator-approved caller that may execute those context-recall scheduler variants.
It must require a non-empty operator id, non-empty idempotency key, explicit
operator confirmation, and a policy-allowed decision before running ready/due
schedulers with `ExperimentalOperatorApproved`. If approval is missing, it must
return a blocked report and leave worker state untouched. Its operator-facing
report may expose scheduler kind, policy, run counts, provider-rollup presence
count, and selected-snippet presence/count only; it must not serialize or
debug-print task prompts, final text, snippet text, source ids, source lanes,
query payloads, redacted-query markers, memory labels, or native selected-context
XML tags.

The runtime operator invocation facade
`run_worker_scheduler_with_context_recall_operator_invocation` is the first
operator/native-shaped execution surface for that adapter. It must evaluate the
channel-scoped operator policy from the supplied channel id, sender id,
owner/sender state, and context-recall worker scheduler command before it can
call the lower-level scheduler adapter. A policy result that still requires
approval or is denied must be translated into `policy_allowed=false`, returning a
blocked scheduler report and leaving worker state untouched. The invocation
report may expose only redacted identity booleans, idempotency-key presence,
operator confirmation state, policy decision/label/counts, and the lower-level
summary scheduler report. It must not serialize or debug-print raw operator id,
raw sender/channel id, idempotency key, task prompts, final text, snippet text,
source ids, source lanes, query payloads, redacted-query markers, memory labels,
or native selected-context XML tags.

Multi-agent runtime callers are an independent runtime owner surface for the same
selected-snippet boundary. Default `run_ready_agents` and
`run_ready_agents_with_reducer` must continue to use disabled context-recall
handoff behavior and return the legacy multi-agent report shape. A multi-agent
run may opt in only through `run_ready_agents_with_context_recall_handoff` and
`AgentRuntimeContextRecallHandoffPolicy::ExperimentalOperatorApproved`. The
context-aware wrapper report may expose provider-rollup presence count and
selected-snippet presence/count only; it must not serialize or debug-print
snippet text, source ids, source lanes, raw ranked payloads, query payloads,
redacted-query markers, memory labels, or native selected-context XML tags.
`run_ready_agents_with_context_recall_operator_invocation` is the policy-aware
multi-agent invocation facade for that entrypoint. It must require a non-empty
operator id, non-empty idempotency key, explicit operator confirmation, and an
operator-policy `allow` decision before calling the selected-snippet handoff. A
policy result that still requires approval, a denied policy, or missing operator
confirmation must return a blocked aggregate report, must not run ready agents,
and must keep the context-recall handoff policy at `Disabled`. Its report may
expose only redacted identity booleans, idempotency-key presence, policy
decision/label/counts, reducer mode, aggregate agent/message counts, and
selected-snippet/provider presence counts. It must not serialize or debug-print
raw operator id, raw sender/channel id, idempotency key, agent prompts, final
text, snippet text, source ids, source lanes, query payloads, redacted-query
markers, memory labels, or native selected-context XML tags.

The unified runtime dispatcher `run_context_recall_operator_invocation` is the
only shared runtime-native selected-snippet operator entrypoint. It may route to
the worker ready/due scheduler invocation facade or to the multi-agent ready
invocation facade, but it must not evaluate its own looser policy or bypass the
target facade's existing operator policy, confirmation, idempotency, and
selected-snippet gates. The unified report is aggregate-only: target, status,
redacted identity booleans, idempotency-key presence, policy decision/counts,
bounded scheduler/agent/message counts, provider-rollup presence count, and
selected-snippet presence/count. It must not embed target-specific full reports
that could contain prompts or final text, and must not serialize or debug-print
raw operator id, raw sender/channel id, idempotency key, task or agent prompt,
final text, snippet text, source ids, source lanes, query payloads,
redacted-query markers, memory labels, or native selected-context XML tags.

`run_context_recall_operator_invocation_command` is a thin external/native-shaped
command facade over the unified dispatcher, not a separate permission surface. It
may parse only the bounded target names `worker-ready`, `worker-due`, and
`multi-agent-ready`; unsupported targets, missing operator ids, or missing
idempotency keys must return a blocked command report without calling the
dispatcher. A supported command may call only
`run_context_recall_operator_invocation`, so the target facade still owns
operator policy, confirmation, idempotency, disabled defaults, and selected
snippet integrity. The command report is aggregate-only and must not serialize
or debug-print raw target text, raw operator id, raw sender/channel id,
idempotency key, prompts, final text, snippet text, source ids, source lanes,
query payloads, redacted-query markers, memory labels, or native
selected-context XML tags.

The selected-snippet default surface/schema audit is a rollout blocker for this
experimental path. It must fail if the app-server source field stops being
explicitly experimental, if stable `TurnStartParams` properties expose
`contextRecallSelectedSnippets`, `context_recall_selected_snippets`, helper
type names, or generic `selectedSnippet` / `selected_snippet` field names in
any generated JSON schema bundle, or if generated stable TypeScript
`TurnStartParams` imports or references the selected-snippet handoff. The
audit must also fail if any non-`TurnStartParams` v2 request params expose the
handoff in their individual JSON schemas, their aggregate JSON schema
definitions, or generated v2 TypeScript, and if any root/non-v2 generated JSON
`*Params` schema exposes the handoff in an individual schema or aggregate root
definition. That audit is derived from the generated `*Params` schema/type
inventory rather than a hand-maintained short list, so newly generated request
params are covered by default. The audit must also fail if selected-snippet JSON
schema markers, including generic `selectedSnippet` / `selected_snippet` payload
or counter names, appear outside the known `ClientRequest`, aggregate protocol,
v2 aggregate protocol, and v2 `TurnStartParams` schema files, or if any
definition other than the selected-snippet helper definitions references the
handoff in those files. Selected snippets are a `turn/start`-only experimental
handoff. The audit must also fail if any root generated TypeScript file outside
the v2 helper namespace, including `ClientRequest.ts`, root `*Params.ts`,
response/notification/helper types, or the root TypeScript index, directly
imports, exports, or exposes selected-snippet helper/payload/counter fields. The
experimental helper schema/type may exist for opted-in clients under the v2
helper surface, but the only v2 generated TypeScript files that may reference it
are the `ContextRecallSelectedSnippet*` helper files and the v2 TypeScript index
that exports those helpers. Stable request params, root stable client
entrypoints, and v2 response/notification/non-helper files must not reference
it.
App-server protocol source markers for the experimental handoff must remain
limited to `protocol/v2/turn.rs` and its v2 protocol tests, so new request
modules cannot promote selected snippets without updating the rollout blocker.
Core/protocol source markers for the live prompt handoff must remain limited to
the core manifest builder, event mapping, the prompt-input no-leak regression in
`prompt_debug.rs`, session handoff sites/tests, context history rollback tests,
and `protocol.rs` typed envelope/manifest contracts, so new core modules cannot
add selected-snippet live prompt handling without updating the rollout blocker.
Hepta-runtime source markers for this handoff must remain limited to the query
types/conversion path, native prompt formatting and aggregate reports, worker
task scheduler/reporting, multi-agent opt-in/invocation, and operator scheduler
/ invocation command modules, so new runtime modules cannot add selected-snippet
handling without updating the rollout blocker.
Any app-server API docs that mention the selected-snippet handoff, including
`contextRecallSelectedSnippets`, helper type names, generic
`selectedSnippet` / `selected_snippet` field names, or the selected-snippet
wording, prose aliases such as `selected snippet(s)`, `selected recall`, or
`recall snippet(s)`, must state the `capabilities.experimentalApi = true` gate
and the `turn/start` / new-turn-only scope on the same line. The audit must
also fail if selected-snippet/prose markers spread into new app-server, native
gateway, TUI, or exec source files outside the known docs, helper, conversion,
test, and dry-run route files. The audit must also fail if `native_gateway`,
app-server, TUI, or exec surfaces directly invoke the runtime command facade
before an intentional live dependency boundary exists, if those surfaces grow
live calls to the selected-snippet runtime handoff/operator entrypoints, or if
their crates add a direct `hepta-runtime` dependency for this path. Runtime may
keep the command facade available for a future external caller, and dry-run
surfaces may expose entrypoint names as strings, but default external surfaces
must remain disabled or `None` until that caller is explicitly approved and
gated.

The native gateway operator route
`/api/hepta-context-recall-worker-scheduler-handoff` is a read-only, dry-run
contract surface for that scheduler policy. It may expose operator gate status,
the disabled/default policy, the experimental operator-approved policy name, and
the allowed runtime entrypoint names. It must not execute ready/due schedulers,
run worker tasks, invoke providers or models, inject selected snippets, mutate
task/session/gateway state, write files, or promote the experimental app-server
field into stable schema. The route response must not include snippet text,
source ids, source lanes, query payloads, redacted-query markers, memory labels,
or native selected-context XML tags. The selected-snippet surface audit must
also lock the native route binding and report shape as dry-run-only: the
ControlUi route spec remains an exact `GET` source block with source command
`--dry-run --json`, capability
`hepta-context-recall-worker-scheduler-handoff`, and a
no-worker/no-model/no-injection/no-write/no-stable-promotion side-effect
boundary, so extra fields or source-shape drift cannot imply execution; the
route endpoint constant remains exactly
`/api/hepta-context-recall-worker-scheduler-handoff`, the raw endpoint path
literal remains limited to that const value only, and its non-test source usage
remains limited to the const declaration, ControlUi pattern, endpoint handler
arm, and report endpoint field; the dry-run source command literal
`/hepta-context-recall-worker-scheduler-handoff --dry-run --json` remains
limited to the ControlUi source command and the dry-run report field only; the
ControlUi capability literal remains a single non-test source line in that
exact route spec so duplicate capability advertising cannot be added silently;
the ControlUi side-effect boundary literal also remains a single non-test source
line in that exact route spec so the no-worker/no-model/no-injection/no-write
claim cannot be copied to another production route or spec silently; the
approval env constant
remains exactly `HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED`, the
approval env const name remains limited to the const declaration, report env
read, and report response field exposure only, the
dry-run report function signature remains parameter-free, non-`pub`, non-`async`,
and returns `HeptaContextRecallWorkerSchedulerHandoffResponse`, the endpoint
handler source block remains exact and returns only
`hepta_context_recall_worker_scheduler_handoff_report()` as JSON and does not
call runtime scheduler/operator entrypoints, read approval env, or add hidden
preconditions; the
`HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED` env var remains a
report-only visibility flag read by the dry-run report, not by the endpoint
handler or ControlUi route spec, and it must not become an execution switch; the
`operator_approval_enabled` source marker remains allowlisted to the report's
env read, disabled-blocker branch, status branch, and response field exposure
only, and must not drive side effects or runtime execution; the
native dry-run response and nested side-effect struct field sets remain
allowlisted so new payload-shaped keys cannot be added silently; the
response and side-effect structs must keep only `#[derive(Debug, Serialize)]`
as their item attribute so serde rename/skip/flatten attributes cannot change
the JSON shape without updating the rollout blocker; the
`allowed_runtime_entrypoints` value list remains limited to the ready/due worker
handoff entrypoint names, and its source array remains allowlisted to those two
quoted entries only so dynamic or unquoted entrypoints cannot be introduced
without updating the rollout blocker; it must not expose operator invocation or
command facade entrypoints; the blocker list remains limited to the
approval-env-disabled and plan-only/no-worker-execution blockers, its source
construction remains allowlisted to the vector initialization, those two pushes,
and response field exposure only, and the two blocker literals remain limited to
the report blocker pushes only, and status remains `blocked` or
`operator_gate_visible`, never a ready/approved execution state; the status
source block remains exactly the `operator_approval_enabled` branch that maps
true to `operator_gate_visible` and false to `blocked`; the report
source must not contain filesystem, process, network, telegram/send, or runtime
scheduler/operator invocation calls while claiming false side effects; the
report source command remains `--dry-run --json`,
route execution/model/injection/schema-promotion booleans remain false,
TUI/exec/app-server defaults remain `None`, snippet text/source id/query payload
exposure booleans remain false, the report metadata/policy/next-step strings
remain allowlisted, and all scalar side-effect booleans remain allowlisted false.

`TurnContextManifestItem.recall_selected_snippets` is the core/session-owned
shadow copy of that envelope and the source of the guarded live prompt handoff.
The manifest may persist the bounded, redacted snippet text in this field so
resume/compaction can prove the same selected-snippet candidate set. The manifest
ledger hash covers the selected-snippet envelope. Invalid count, bounds, marker,
safety state, or prompt-unsafe text must be rejected before the field is
persisted from turn-scoped extension data or explicit manifest options.

Core/session may append one live developer context item for validated selected
snippets. That live item may contain only the bounded redacted snippet text,
stable snippet hash, and redaction/truncation markers. It must not contain source
ids, source lanes, raw ranked payloads, score reasons, marker-bearing control
records, per-source lists, memory/topic/neuron ids, or query payload. If any
selected snippet contains prompt-unsafe metadata such as source identifiers,
ranked payload labels, control markers, per-origin labels, or query-payload
labels, core/session must drop both the live prompt item and the manifest
selected-snippet field for that turn.

Core/session must not repeat the same selected-snippet live developer item when
that exact bounded item is already present in model history. Duplicate
no-context-diff request retries may refresh or carry the manifest baseline, but
must not append another `<selected_context_recall>` block. The live
`<selected_context_recall>` block is rollback-trimmable contextual developer
content and must be removed when rolling back the user turn it prepared.

Selected-snippet live prompt compression gate: when the
`source_aware_compression_canary` feature and the helper-injected
source-aware-compression marker are both present, core/session must still reject
prompt-unsafe selected snippets before live prompt or manifest persistence. The
source-aware compression chain may continue compressing other eligible context
diffs, but the rejected selected snippet must not create a
`<selected_context_recall>` live item, must not create
`TurnContextManifestItem.recall_selected_snippets`, must not trigger selected
recall summary compression, and must not leak the unsafe snippet payload or
source-aware routing metadata into history. The gate is
`scripts/hepta-context-selected-snippet-live-prompt-compression-gate.sh`, covering
`record_context_updates_and_set_reference_context_item_rejects_prompt_unsafe_selected_snippets_under_source_aware_compression_opt_in`,
`record_context_updates_and_set_reference_context_item_honors_turn_scoped_source_aware_compression_opt_in`,
and `user_input_with_turn_context_selected_snippets_reject_prompt_unsafe_payload`.

`Op::UserInputWithTurnContext.context_recall_selected_snippets` is the typed
request-path handoff for real provider snippets. It may carry only a
`TurnContextRecallSelectedSnippetEnvelope`; callers that own runtime recall data
must convert runtime envelopes into this protocol shape before submission.
Core/session must treat this field as untrusted input. The request handler may
attach it to turn-scoped extension data, but manifest construction and live
prompt history must re-run the same count, bounds, safety, and prompt-safe text
validation before consuming it. Legacy `Op::UserInput` has no selected-snippet
request field.

App-server v2 exposes the same handoff only on `turn/start` as
`contextRecallSelectedSnippets`. The app-server API shape is camelCase and maps
into the core protocol envelope before submission. The field is experimental:
clients must opt into `initialize.params.capabilities.experimentalApi = true`
before sending it, and the stable schema must not expose it. App-server must
reject envelopes that fail the protocol shadow-integrity gate before accepting
the turn, but core/session still treats accepted envelopes as untrusted and
re-runs the guarded manifest/live prompt checks. `turn/steer` does not accept
this field, because it is not a new turn-context boundary.

Caller-side request assembly must also apply the opt-in boundary before
populating app-server `TurnStartParams`: if a caller has no runtime/provider
selected-snippet envelope, or the client did not initialize with experimental
API enabled, the field stays `None`. Current TUI/exec callers route through a
typed core-envelope-to-v2-envelope helper but do not synthesize provider data.
The TUI `AppCommand::UserTurn` queue can carry an optional core selected-snippet
envelope for a future provider, but that field must not serialize into the TUI
outbound session log. The exec initial user-turn operation likewise carries only
an optional core envelope and defaults to `None` until a real provider supplies
one. Real provider population must enter through that helper chain and must drop
invalid shadow-integrity envelopes before request submission.

Selected-snippet API/caller surface gate: the standalone gate
`scripts/hepta-context-selected-snippet-api-surface-gate.sh` is the cross-layer
blocker for app-server/TUI/exec caller surfaces. It runs the default schema/type
surface audit, the typed protocol request handoff tests, the app-server v2
`turn/start` camelCase round-trip and core-conversion tests, the TUI helper and
outbound no-log tests, the exec helper tests, the app-server turn/start handoff
tests, the app-server experimental API capability rejection, and the app-server
thread-history no-routing-metadata regression. The gate must emit
`selected-snippet-api-surface=pass`,
`selected-snippet-api-surface.default-surface=audit-pass`,
`selected-snippet-api-surface.app-server=experimental-turn-start-only`,
`selected-snippet-api-surface.tui=opt-in-no-log`,
`selected-snippet-api-surface.exec=opt-in`,
`selected-snippet-api-surface.history=no-routing-metadata`, and
`selected-snippet-api-surface.runtime-activation=disabled`.

The selected-snippet API/caller surface gate specifically covers the protocol
request filter `user_input_with_turn_context_`, app-server protocol tests
`turn_start_params_round_trip_context_recall_selected_snippets` and
`context_recall_selected_snippets_from_core`, TUI tests
`context_recall_selected_snippets_for_turn_start` and
`user_turn_selected_snippets_are_not_serialized`, exec test filter
`context_recall_selected_snippets_for_turn_start`, app-server handoff filter
`context_recall_selected_snippets_v2`, app-server experimental gate
`turn_start_context_recall_selected_snippets_requires_experimental_api_capability`,
and app-server history regression
`turn_start_source_aware_compression_canary_thread_history_hides_routing_metadata`.
The app-server handoff filter must run with `--test-threads=1` in the
context-lane gates so the two selected-snippet handoff fixtures do not start two
app-server processes concurrently and create a false initialization deadline
under full-preflight load.

Response-debug export may flatten only selected-snippet envelope counters and
safety booleans: presence, invalid state, selected/omitted/redacted/truncated
counts, max snippet limits, ready state, and bounded state. It must not export
snippet text, snippet source ids, raw ranked payloads, source lanes, memory/topic
/neuron ids, score reasons, control markers, per-source lists, or query payload.
For real assembly truncation, response-debug may also flatten payload-light
manifest truncation evidence: whether `truncated=true`, the count of
`truncated:*` decision entries, the manifest entry sources referenced by those
decisions, and whether the evidence is missing or malformed. It must not export
raw `decision_ledger` strings, replay keys, text hashes, or prompt text. Strict
mode must fail if a manifest claims `truncated=true` without matching
`truncated:*` evidence, if truncated evidence appears while `truncated=false`,
or if a truncated decision is malformed or source-disconnected from the manifest
entries.
Response-debug may additionally export only payload-light decision schema
summary fields: `latest_manifest_decision_schema_version`,
`latest_manifest_decision_known_count`,
`latest_manifest_decision_unknown_count`, and per-kind counts for `included`,
`policy`, `candidate_omit`, `candidate_truncate`, `omitted`, and `truncated`
decisions.
Response-debug may additionally export only payload-light compression-stage
summary fields: stage schema version, stage count, unique stage kind rollup,
unique loss-check status rollup, rollback source text hash count, protected-tier
invariant rollup, aggregate input/output token counts, aggregate saved tokens,
aggregate affected entry count, and whether stage integrity failed. It must not
export raw `compression_stages`, source ids, replay keys, text hashes, rollback
hash values, prompt text, snippet text, query text, or memory/topic/neuron ids.
Strict mode must fail with
`manifest_compression_stages_invalid` when non-empty stages violate the
payload-light stage contract, including invalid rollback source text hashes or
unknown loss-check/protected-tier invariant values. Session-rollout-shaped debug
fixtures must cover executed `summary`, `defragment`, and `prune` stages
together and prove ordinary response items plus stage-adjacent source ids,
replay keys, text hashes, rollback hash values, prompt markers, and query text
do not reach the exported JSON.
They must also treat source-aware compression canary feature keys and opt-in
marker type/value strings as routing-only metadata that never reach
response-debug export. App-server experimental feature list/read/write surfaces
may expose the canary feature key as feature-control metadata, but that metadata
must not be copied into rollout/debug manifests or prompt-history exports.
Response-debug may additionally export only payload-light compression-candidate
summary fields: candidate schema version, candidate count, unique stage kind
rollup, tier rollup, source taxonomy rollup, not-executed reason rollup,
aggregate input/output token counts, aggregate estimated saved tokens, aggregate
affected entry count, and whether candidate integrity failed. It must not export
raw `compression_candidates`, manifest sources, replay keys, text hashes, prompt
text, snippet text, query text, or memory/topic/neuron ids. Strict mode must
fail with `manifest_compression_candidates_invalid` when non-empty candidates
violate the payload-light candidate contract.
Response-debug selected-snippet source markers must remain limited to the
aggregate-only export implementation and its inline tests in
`response-debug-context/src/lib.rs`, so new response-debug binaries or source
modules cannot start handling snippet payload fields without updating the
rollout blocker. The response-debug export gate must also reject exported JSON
paths that introduce payload-shaped keys such as snippet arrays, snippet hashes,
raw text, source ids, replay keys, raw ranked payloads, rank explanations,
control markers, query payloads, per-origin lists, or memory/topic/neuron ids.
That gate must keep the full exported JSON path set allowlisted to the existing
version, summary counters/booleans, manifest source rollup, and audit findings
shape; adding any new response-debug export field for this area must update the
rollout blocker explicitly.
The response-debug export gate must also keep the combined surface regression
`rollout_context_debug_summary_combines_payload_light_surfaces_without_cross_surface_leaks`.
That regression places recall selection, selected-snippet envelope, memory
taxonomy, memory formation receipts, temporal facts, compression candidates,
adaptive budget allocations, executed compression stages, and truncation
evidence into one manifest and proves the flattened export remains payload-light
across the combined surface. The gate must emit
`response-debug-export=pass`, `response-debug-export.payload-light=pass`,
`response-debug-export.combined-surfaces=no-leak`,
`response-debug-export.strict-invalid=reject`, and
`response-debug-export.runtime-activation=disabled`.

Core/session consumes recall provider rollup only as a turn-scoped shadow
attachment containing `TurnContextRecallSelectionSummary`. Invalid count
integrity is not persisted into `TurnContextItem.context_manifest`. Consuming the
attachment may refresh the shadow manifest ledger, but must not append prompt
history or turn the rollup into live prompt injection.

## Reference Memory Recall Controls

Until the memory contract grows structured tombstone/conflict metadata, the
reference `hepta-memory` store recognizes explicit control records with these
markers:

- `[hepta-memory:tombstone]`
- `[hepta-memory:conflict]`

Records carrying those markers are metadata/control records, not recall
payloads. Snapshot search, async memory search, memory query reports, and
`recall_context` must filter them before matched/availability counts are
computed. Returned hits, recall bundles, and query reports must not expose the
control-marker records or require per-source payload details to explain the
filtering. `MemoryQueryReport::omitted_control_count` is the portable
payload-light pressure signal for these filtered candidates.

Hepta-memory snapshot helper boundary: the reference store implementation for
`StoreSnapshot` stats, manifests, inventories, integrity reports, audit reports,
inspection bundles, inspection drift/health summaries, restore previews,
restore readiness/safety summaries, and `InspectedStoreSnapshot` normalization
and restore delegates is rooted at the thin wrapper
`codex-rs/hepta-memory/src/snapshot_helpers.rs`, with snapshot calculations in
`codex-rs/hepta-memory/src/snapshot_helpers/snapshot.rs`, inspected snapshot
serde and delegates in
`codex-rs/hepta-memory/src/snapshot_helpers/inspected_snapshot.rs`, and store
snapshot/inspection/restore facade methods in
`codex-rs/hepta-memory/src/snapshot_helpers/store.rs`. The snapshot helper
method names include `inspection_bundle`, `inspection_drift_report`,
`inspection_health`, `restore_preview_against`, and
`restore_readiness_against`. The store helper submodule owns the
`InMemoryStore` snapshot, inspected snapshot, inspection, and restore preview
wrapper methods such as `snapshot_inspection_bundle` and `preview_restore`.
`codex-rs/hepta-memory/src/lib.rs` must keep the `mod snapshot_helpers;`
declaration and the public `StoreSnapshot`, `InspectedStoreSnapshot`, and
`InMemoryStore` helper method names unchanged. The context debug gate and
preflight must run
`scripts/hepta-context-memory-snapshot-helper-boundary-gate.sh` after
`scripts/hepta-context-adaptive-budget-allocation-report-gate.sh` and before
`scripts/hepta-context-memory-recall-helper-boundary-gate.sh`.

Hepta-memory test module boundary: the reference store test wrapper must live in
`codex-rs/hepta-memory/src/tests/mod.rs` instead of an inline `#[cfg(test)]`
body in `codex-rs/hepta-memory/src/lib.rs`. `lib.rs` may keep only the external
`mod tests;` declaration for hepta-memory tests. `tests/mod.rs` may keep shared
imports, shared fixtures, and submodule declarations. Root test modules may keep
nested submodule declarations when a test family needs another boundary; concrete
wrapper paths include `codex-rs/hepta-memory/src/tests/context_plane.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality.rs`,
`codex-rs/hepta-memory/src/tests/recall_memory.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inspection.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inventory.rs`, and
`codex-rs/hepta-memory/src/tests/snapshot_restore.rs`. Concrete test bodies
must live in `codex-rs/hepta-memory/src/tests/context_memory.rs`,
`codex-rs/hepta-memory/src/tests/context_plane/activation_matrix.rs`,
`codex-rs/hepta-memory/src/tests/context_plane/operator_packet.rs`,
`codex-rs/hepta-memory/src/tests/context_plane/status.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_core.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers/availability.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers/bundle.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers/coverage.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers/limit_pressure.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers/omission.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_helpers/provenance.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality/availability.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality/coverage.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality/inspection.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality/limit_pressure.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality/omission.rs`,
`codex-rs/hepta-memory/src/tests/recall_context_quality/provenance.rs`,
`codex-rs/hepta-memory/src/tests/recall_memory/formation.rs`,
`codex-rs/hepta-memory/src/tests/recall_memory/taxonomy.rs`,
`codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs`,
`codex-rs/hepta-memory/src/tests/restore_preview.rs`,
`codex-rs/hepta-memory/src/tests/search.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_core.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inspection/audit.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inspection/drift.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inspection/health.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inspection/inspected.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_integrity.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inventory/manifest.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inventory/session_inventory.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_inventory/stats.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_restore/impact.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_restore/inspected.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_restore/preview.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_restore/readiness.rs`,
`codex-rs/hepta-memory/src/tests/snapshot_restore/roundtrip.rs`, and
`codex-rs/hepta-memory/src/tests/store.rs`. The external test modules must
retain coverage for `store_snapshot`, `recall_context`, and `context_plane`
fixtures without changing public API paths, wire schema, payload-light behavior,
or runtime activation state. The context debug gate and preflight must run
`scripts/hepta-context-memory-test-module-boundary-gate.sh` after
`scripts/hepta-context-memory-snapshot-helper-boundary-gate.sh` and before
`scripts/hepta-context-memory-recall-helper-boundary-gate.sh`.

Hepta-memory recall helper boundary: the reference store implementation for
`search_report`, `transcript_search_report`, `recall_context_parts`,
`recall_context_report`, `recall_context_inspection`,
`recall_context_coverage`, `recall_context_source_availability`, and
`memory_records_matching_recall_query` is rooted at the thin wrapper
`codex-rs/hepta-memory/src/recall_helpers.rs`, with query/control filtering in
`codex-rs/hepta-memory/src/recall_helpers/query.rs`, snapshot recall helpers in
`codex-rs/hepta-memory/src/recall_helpers/snapshot.rs`, and store recall
helpers in `codex-rs/hepta-memory/src/recall_helpers/store.rs`.
`codex-rs/hepta-memory/src/lib.rs` must keep the `mod recall_helpers;`
declaration, the public `StoreSnapshot` and `InMemoryStore` helper method names
unchanged, and the async memory search path must continue to use the same
control-record filtering helper. The context debug gate and preflight must run
`scripts/hepta-context-memory-recall-helper-boundary-gate.sh` after
`scripts/hepta-context-adaptive-budget-allocation-report-gate.sh` and before
`scripts/hepta-context-memory-recall-manifest-payload-light-gate.sh`.

## Response Debug Export

`response_debug_context --strict` reads rollout JSONL from stdin and emits a
payload-light JSON audit. The export may include flattened counters such as:

- `latest_manifest_recall_returned_source_count`
- `latest_manifest_recall_selected_source_count`
- `latest_manifest_recall_ranked_source_count`
- `latest_manifest_recall_returned_unselected_source_count`
- `latest_manifest_recall_source_diversity_met`
- `latest_manifest_recall_source_diversity_target`
- `latest_manifest_recall_max_per_source`
- `latest_manifest_recall_ranked_item_count`
- `latest_manifest_recall_omitted_by_budget_count`
- `latest_manifest_recall_memory_control_omitted_count`
- `latest_manifest_recall_low_trust_ranked_item_count`
- `latest_manifest_recall_low_recency_ranked_item_count`
- `latest_manifest_recall_selection_invalid`
- `latest_manifest_memory_taxonomy_schema_version`
- `latest_manifest_memory_taxonomy_count`
- `latest_manifest_memory_taxonomy_classes`
- `latest_manifest_memory_taxonomy_source_count`
- `latest_manifest_memory_taxonomy_returned_count`
- `latest_manifest_memory_taxonomy_available_count`
- `latest_manifest_memory_taxonomy_omitted_count`
- `latest_manifest_memory_taxonomy_provenance_span_count`
- `latest_manifest_memory_taxonomy_invalid`
- `latest_manifest_memory_formation_receipt_schema_version`
- `latest_manifest_memory_formation_receipt_count`
- `latest_manifest_memory_formation_receipt_candidate_types`
- `latest_manifest_memory_formation_receipt_privacy_classes`
- `latest_manifest_memory_formation_receipt_transcript_span_count`
- `latest_manifest_memory_formation_receipt_provenance_span_count`
- `latest_manifest_memory_formation_receipt_confidence_basis_points`
- `latest_manifest_memory_formation_receipt_queued_count`
- `latest_manifest_memory_formation_receipt_production_write_count`
- `latest_manifest_memory_formation_receipt_invalid`
- `latest_manifest_memory_temporal_fact_schema_version`
- `latest_manifest_memory_temporal_fact_count`
- `latest_manifest_memory_temporal_fact_types`
- `latest_manifest_memory_temporal_fact_privacy_classes`
- `latest_manifest_memory_temporal_fact_provenance_span_count`
- `latest_manifest_memory_temporal_fact_confidence_basis_points`
- `latest_manifest_memory_temporal_fact_open_count`
- `latest_manifest_memory_temporal_fact_invalidated_count`
- `latest_manifest_memory_temporal_fact_supersedes_count`
- `latest_manifest_memory_temporal_fact_dry_run_count`
- `latest_manifest_memory_temporal_fact_production_write_count`
- `latest_manifest_memory_temporal_fact_invalid`
- `latest_manifest_decision_schema_version`
- `latest_manifest_decision_known_count`
- `latest_manifest_decision_unknown_count`
- `latest_manifest_decision_included_count`
- `latest_manifest_decision_policy_count`
- `latest_manifest_decision_candidate_omit_count`
- `latest_manifest_decision_candidate_truncate_count`
- `latest_manifest_decision_omitted_count`
- `latest_manifest_decision_truncated_count`
- `latest_manifest_compression_candidate_schema_version`
- `latest_manifest_compression_candidate_count`
- `latest_manifest_compression_candidate_stages`
- `latest_manifest_compression_candidate_tiers`
- `latest_manifest_compression_candidate_sources`
- `latest_manifest_compression_candidate_reasons`
- `latest_manifest_compression_candidate_input_tokens`
- `latest_manifest_compression_candidate_output_tokens`
- `latest_manifest_compression_candidate_tokens_saved`
- `latest_manifest_compression_candidate_affected_entries`
- `latest_manifest_compression_candidate_invalid`
- `latest_manifest_compression_stage_schema_version`
- `latest_manifest_compression_stage_count`
- `latest_manifest_compression_stages`
- `latest_manifest_compression_loss_check_statuses`
- `latest_manifest_compression_rollback_source_text_hash_count`
- `latest_manifest_compression_protected_tier_invariants`
- `latest_manifest_compression_input_tokens`
- `latest_manifest_compression_output_tokens`
- `latest_manifest_compression_tokens_saved`
- `latest_manifest_compression_affected_entries`
- `latest_manifest_compression_invalid`
- `latest_manifest_truncated`
- `latest_manifest_truncated_decision_count`
- `latest_manifest_truncated_sources`
- `latest_manifest_truncation_evidence_present`
- `latest_manifest_truncation_evidence_invalid`
- `latest_manifest_tiers`

Strict mode must fail with `manifest_recall_selection_invalid` when malformed
recall-selection counters are present. It must accept valid unranked rollups and
must not leak recall source ids or payload text into the export. Exported tier
rollups may contain only context-tier vocabulary values, compression candidate
stage rollups may contain only compression-stage vocabulary values, compression
candidate tier rollups may contain only context-tier vocabulary values, and
compression candidate source rollups may contain only bounded source taxonomy
values. For invalid compression candidates, strict mode must fail with
`manifest_compression_candidates_invalid`. For invalid compression stages,
strict mode must fail with `manifest_compression_stages_invalid`. For invalid
memory taxonomy buckets, strict mode must fail with
`manifest_memory_taxonomy_invalid`. For invalid background memory formation
receipts, strict mode must fail with
`manifest_memory_formation_receipts_invalid`. For invalid memory temporal facts,
strict mode must fail with `manifest_memory_temporal_facts_invalid`. For true
manifest truncation, strict mode
must also fail with
`manifest_truncation_evidence_missing`,
`manifest_truncation_evidence_unexpected`, or
`manifest_truncation_evidence_invalid` when the manifest flag and payload-light
`truncated:*` decision evidence do not match.

Primary local gate:

```bash
scripts/hepta-context-response-debug-export-gate.sh
scripts/hepta-context-selected-snippet-live-prompt-compression-gate.sh
scripts/hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh
```

The response-debug export gate must keep
`rollout_context_debug_summary_combines_payload_light_surfaces_without_cross_surface_leaks`
in the `codex-response-debug-context` rollout test set. The gate output must
include `response-debug-export=pass`,
`response-debug-export.combined-surfaces=no-leak`, and
`response-debug-export.strict-invalid=reject` so preflight logs identify the
combined payload-light/no-leak coverage explicitly.

## Prompt Input Gate

The current main branch does not yet expose the old payload-light
`debug prompt-input --summary` surface. Until that surface is restored, the
context lane prompt-input gate keeps the existing prompt-input construction smoke
covered and adds a direct `Session` prompt-input regression for turn-scoped
context manifest options.

`build_prompt_input_from_session_consumes_context_manifest_without_shadow_leak`
must prove that prompt-input construction can consume a payload-light recall
provider rollup and a guarded selected-snippet envelope without serializing the
shadow manifest ledger, recall-selection counters as field names, selected
snippet envelope fields, source ids, raw memory markers, or recall bait payloads
into the model-visible prompt input. The live prompt may contain only the
guarded `<selected_context_recall>` item with bounded snippet text/hash and the
ordinary user message. Summary propagation remains a follow-up for the restored
debug summary surface.

Primary local gate:

```bash
scripts/hepta-context-prompt-input-summary-gate.sh
```

The gate output must include `prompt-input=pass`,
`prompt-input.context-manifest=no-leak`,
`prompt-input.live-selected-snippet=guarded`, and
`prompt-input.runtime-activation=disabled` so preflight logs identify this edge
surface explicitly.

## Selected-Recall Summary Canary Readiness Gate

`scripts/hepta-context-selected-recall-summary-canary-report.sh` is the
payload-light scoreboard for moving the `selected_context_recall` summary
transform from scattered shadow proofs toward a measured canary. It must only
emit fixed control-plane readiness lines and source-registry descriptor values:
redaction policy, quality metric, activation guard, rollback policy, compression
action, required shadow-vs-live comparison, token-saved metric,
latency-delta metric, quality-delta metric, rollback-readback requirement,
prompt-input-proof requirement, response-debug-proof requirement, operator
approval requirement, prompt-input/readback proof labels, response-debug
readback proof labels, rollback fixture/readback proof labels, production route
disabled, and runtime activation disabled. It must not export prompt text,
recalled memory text, snippet text,
snippet hashes, query payloads, source ids beyond the fixed
`selected_context_recall` control-plane label, replay keys, text hashes,
rollback hash values, tool arguments, tool outputs, session ids, trace ids, or
operator identities.

`scripts/hepta-context-selected-recall-summary-canary-gate.sh` verifies the
report, the registry descriptor row, the debug/preflight wiring, and the release
manifest entries for both scripts. It must also verify the narrow Rust
selected-recall controller boundary in
`codex-rs/core/src/context_manager/manifest/selected_recall.rs`, where
`SelectedRecallControllerDecision` owns extension-data intake for
`recall_selection` and `recall_selected_snippets` plus the manifest application
step. The same controller must expose
`SelectedRecallControllerCanaryReadiness` as a payload-light readiness view for
shadow-vs-live, token-saved, latency-delta, quality-delta,
rollback-readback, prompt-input proof, response-debug proof, operator approval,
production route disabled, and runtime activation disabled. It must also expose
`SelectedRecallControllerCanaryMetrics` for the fixed token-saved, latency,
quality, rollback-readback fixture, prompt-input proof, and response-debug proof
thresholds used by the canary scoreboard, while keeping production route and
runtime activation disabled. The controller must also expose
`SelectedRecallControllerReadbackProofs`,
`SelectedRecallControllerReadbackProof`, and
`SelectedRecallControllerReadbackSurface` so prompt-input manifest consumption,
prompt-input shadow metadata omission, guarded live selected-snippet prompt
handoff, response-debug manifest summary coverage, response-debug payload-light
summary coverage, rollback-readback fixture coverage, and rollback hash omission
are fixed controller-readback proof objects rather than an aggregate Boolean
score. These controller-readback proofs must be independently asserted without
adding manifest wire fields. The gate is observational only: it must not enable
a production route, must not mutate prompt assembly by itself, must not write
memory or graph state, must not accept an operator activation, and must keep
`runtime-activation=disabled`. In `scripts/hepta-context-preflight.sh`, it
must run after the context response-debug export gate and context prompt-input
gate so the final scoreboard can rely on those proof surfaces. The debug gate
must run it after `hepta-context-response-debug-export-gate.sh` and
`hepta-context-prompt-input-summary-gate.sh`.

The gate output must include `selected-recall-summary-canary=pass`,
`selected-recall-summary-canary.payload-light=pass`,
`selected-recall-summary-canary.metrics=shadow-live-token-latency-quality`,
`selected-recall-summary-canary.readback.prompt-input=manifest-no-leak`,
`selected-recall-summary-canary.readback.response-debug=payload-light-summary`,
`selected-recall-summary-canary.readback.rollback=fixture-covered`,
`selected-recall-summary-canary.controller-readback.prompt-input.manifest-consumed=covered`,
`selected-recall-summary-canary.controller-readback.prompt-input.shadow-metadata=omitted`,
`selected-recall-summary-canary.controller-readback.prompt-input.live-selected-snippet=guarded`,
`selected-recall-summary-canary.controller-readback.response-debug.manifest-summary=covered`,
`selected-recall-summary-canary.controller-readback.response-debug.payload-light-summary=covered`,
`selected-recall-summary-canary.controller-readback.rollback.fixture=covered`,
`selected-recall-summary-canary.controller-readback.rollback.hash=omitted`,
`selected-recall-summary-canary.operator-approval=required`, and
`selected-recall-summary-canary.runtime-activation=disabled`.

## Selected-Recall Summary Canary Eval Replay Gate

`scripts/hepta-context-selected-recall-summary-canary-eval-report.sh` is the
payload-light golden-replay-shadow scoreboard for the selected-recall summary
canary. It depends on the selected-recall summary canary readiness gate and the
existing memory eval harness, adaptive shadow, and recall quality gate contracts,
but it must not run a live runtime route or emit any payload material. The report
may contain only fixed fixture counts, positive/negative fixture counts,
shadow-vs-live pair counts, rollback-readback fixture counts, prompt-input and
response-debug proof coverage, fixed threshold labels and values
(`token-saved-min-basis-points`, `latency-delta-max-ms`, and
`quality-delta-min-basis-points`), regression fixture status, operator approval
requirement, production route disabled, and runtime activation disabled.

The scoreboard is backed by a Rust fixture in
`codex-rs/hepta-core/src/memory/eval_harness/selected_recall_canary.rs` exposed
as `ContextMemorySelectedRecallSummaryCanaryEvalReport`. `hepta-memory`
surfaces it through
`context_memory_selected_recall_summary_canary_eval_report` on both
`StoreSnapshot` and `InMemoryStore`, keeping the same fixed four-fixture
golden-replay-shadow counts without reading payloads from the store.

`scripts/hepta-context-selected-recall-summary-canary-eval-gate.sh` verifies the
report, Rust-backed fixture boundary, hepta-core/hepta-memory helper tests, the
upstream readiness gate, debug/preflight wiring, release manifest entries, and
payload-light no-leak constraints. The regression fixture must be blocked,
fixture-blocked-count must remain zero for the positive replay set, and the
report must keep a rollback-readback fixture visible without exporting raw
rollback hash values. It must not export prompt text, recalled memory text,
snippet text, snippet hashes, query payloads, source ids, replay keys, text
hashes, rollback hash values, tool arguments, tool outputs, session ids, trace
ids, or operator identities. The gate is observational only: it must not mutate
prompt assembly, must not write memory or graph state, must not enable a
production route, must not accept operator activation, and must keep
`runtime-activation=disabled`.

The debug gate and preflight must run
`scripts/hepta-context-selected-recall-summary-canary-eval-gate.sh` immediately
after `scripts/hepta-context-selected-recall-summary-canary-gate.sh`. The gate
output must include `selected-recall-summary-canary-eval=pass`,
`selected-recall-summary-canary-eval.payload-light=pass`,
`selected-recall-summary-canary-eval.fixtures=4`,
`selected-recall-summary-canary-eval.regression-fixture=blocked`, and
`selected-recall-summary-canary-eval.runtime-activation=disabled`.

## Context Gate Target Directory

Context lane cargo gates must use a stable lane target directory so integration
tests do not spawn stale app-server binaries from a sibling cache. The default
lane `hepta-context` must resolve to
`$HOME/.openclaw/tmp/cargo-targets/hepta-context`, not
`$HOME/.openclaw/tmp/cargo-targets/hepta-hepta-context`; lanes without a
`hepta-` prefix may still be normalized by adding that prefix. An explicit
`HEPTA_CARGO_TARGET_DIR` override remains authoritative.

The selected-snippet surface audit statically checks the cargo gate scripts for
that target-leaf normalization and rejects the older `$target_root/hepta-$lane`
default. This keeps the context preflight/debug gates from silently splitting
their app-server binary cache away from the active `hepta-context` lane.

When sibling runtime generated-preview code is compile-broken, the context lane
may run the explicit non-runtime preflight scope via
`scripts/hepta-context-non-runtime-preflight.sh`. That wrapper sets
`HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME=1`, causing
`scripts/hepta-context-preflight.sh` to skip only the hepta-runtime and native
gateway runtime stages. The scoped run must emit
`hepta-context-preflight.scope=non-runtime`,
`hepta-context-preflight.runtime-stages=skipped`,
`hepta-context-preflight.runtime-activation=disabled`,
`hepta-context-non-runtime-preflight=pass`, and
`hepta-context-non-runtime-preflight.runtime-stages=skipped`. Default preflight
must keep `skip-runtime-stages=0` and must not skip runtime stages silently. The
non-runtime scope is a temporary verification surface, not a replacement for a
full preflight after the sibling runtime compile blocker is cleared.

Primary local gates:

```bash
scripts/hepta-context-non-runtime-preflight.sh
scripts/hepta-context-preflight.sh
scripts/hepta-context-response-debug-export-gate.sh
scripts/hepta-context-prompt-input-summary-gate.sh
scripts/hepta-context-selected-recall-summary-canary-gate.sh
scripts/hepta-context-selected-recall-summary-canary-eval-gate.sh
scripts/hepta-context-selected-snippet-api-surface-gate.sh
scripts/hepta-context-selected-snippet-surface-audit.sh
```
