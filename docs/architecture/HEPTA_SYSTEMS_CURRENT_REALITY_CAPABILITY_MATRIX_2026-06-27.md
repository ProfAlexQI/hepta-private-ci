# Hepta Systems Current Reality Capability Matrix - 2026-06-27

This note records the local-only Current Reality Capability Matrix for the
Hepta systems lane. It is a Phase 0 guard after the plugins/tools/workflow
audit. It reconciles current checkout facts with memory/filesystem drift and
does not open live execution.

## Current Facts

The matrix reads existing local reports instead of replaying historical patches:

- `scripts/hepta-systems-plugin-contribution-point-abi-report.sh`
- `scripts/hepta-systems-plugin-contribution-point-loader-binding-report.sh`
- `scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh`
- `scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh`
- `scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh`
- `scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh`
- `scripts/hepta-systems-work-graph-durable-identity-preview-report.sh`
- `scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-receipt-preview-report.sh`
- `scripts/hepta-systems-workflow-durable-store-adapter-report.sh`
- `scripts/hepta-systems-workflow-durable-store-test-only-append-fixture-report.sh`
- `scripts/hepta-systems-hepta-system-status-read-only-e2e-report.sh`
- `scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh`
- `scripts/hepta-systems-hepta-system-status-operator-approval-protocol-report.sh`
- `scripts/hepta-systems-controlled-canary-readiness-plan-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-inventory-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback-report.sh`
- `scripts/hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot-report.sh`
- `scripts/hepta-systems-controlled-live-readiness-audit-report.sh`
- `scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh`
- `scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh`
- `scripts/hepta-systems-controlled-live-operator-packet-non-send-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback-report.sh`
- `scripts/hepta-systems-current-compact-capability-summary-report.sh`

The `hepta-system` fixture is present in the current checkout. Its manifest
uses path fields for skills, MCP servers, and app connectors, so the matrix
resolves those paths before counting declarations:

- 1 skill declaration under `./skills`
- 1 MCP server declaration in `./.mcp.json`
- 1 app connector declaration in `./.app.json`
- 2 tool schemas
- 2 permission declarations
- 2 activation event declarations
- 2 tool policy declarations

The matrix is ready only when all forty-four local capability rows are ready and no
row enables live execution, public GA, tool invocation, ToolRegistry mutation,
ledger writes, approval requests, WorkGraph execution, replay, rollback,
provider/model calls, gateway/auth mutation, Native POST mutation, package or
release writes, or channel sends.

The workflow durable-store test-only append fixture row is ready: it covers all
nine durable workflow event contracts with append-only sequence, idempotency,
checkpoint, replay validation, rollback metadata, and duplicate append denial
fixture entries. Runtime feature gate enablement, event-log writes, SQLite
writes, fixture persistence, workflow execution, replay, rollback, and live
execution remain disabled.

The hepta-system status internal read-only invocation row is ready: it
materializes one internal status payload from the selected MCP candidate
`preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp` while keeping the
app connector candidate preflight-only. External network access, credential
reads, external tool invocation, ToolRegistry live switch, ledger writes,
approval requests, approval acceptance, receipt persistence, workflow event-log
writes, SQLite writes, Native POST mutation, channel send, and live execution
remain disabled.

The hepta-system status operator approval protocol row is `ready_blocked`: it
binds the Phase 8 internal status payload to an approval subject, nonce,
operator session binding, approval packet preview, and non-acceptance receipt
projection. Approval request, approval acceptance, approval recording,
auto-approval, approval broker writes, evidence recording, credential reads,
external network access, ledger writes, receipt persistence, workflow event-log
writes, SQLite writes, transport mutation, Native POST mutation, channel send,
and live execution remain disabled.

The controlled canary readiness plan row is `ready_blocked`: it consumes the
Phase 9 approval protocol plus the Phase 5n kill-switch rehearsal boundary
readback and turns the seven unchanged-missing blockers into a canary readiness
plan. Canary activation, approval acceptance, evidence recording, credential
reads, Gateway/Auth mutation, Native POST mutation, Telegram/channel transport,
persistence, package/release writes, Public GA, and live execution remain
disabled.

The dirty worktree release-boundary inventory row is `ready_blocked`: it reads
`git status --porcelain` and classifies the dirty checkout into tracked,
untracked, staged/index, unstaged/worktree, top-level, Hepta systems-owned, and
cross-lane/unowned buckets. It keeps the release boundary open and blocks git
mutation, cleanup, evidence persistence, package/release writes, Public GA,
canary activation, and live execution.

The dirty worktree release-boundary grouping freeze-plan row is `ready_blocked`:
it consumes the inventory top-level and scope buckets and turns them into
operator-visible grouping entries with owner hints, review lanes, and readback
routes. Every entry stays `planned_not_applied`, with freeze application, git
mutation, cleanup, evidence recording, evidence persistence, approval
acceptance, blocker waiver, package/release writes, Public GA, canary
activation, and live execution disabled.

The dirty worktree release-boundary grouping freeze operator readback row is
`ready_blocked`: it consumes the grouping freeze plan and gives each group a
stable readback key, readback route, diff key, and comparison anchor. Freeze
state and evidence state remain unchanged, while freeze application, git
mutation, cleanup, evidence recording, evidence persistence, approval
acceptance, blocker waiver, package/release writes, Public GA, canary
activation, and live execution remain disabled.

The dirty worktree release-boundary actionable clean-worktree strategy row is
`ready_blocked`: it converts the seven operator readback groups into an
operator-visible strategy while strategy application, git mutation, cleanup,
evidence recording, evidence persistence, approval acceptance, blocker waiver,
package/release writes, Public GA, canary activation, and live execution remain
disabled.

The dirty worktree release-boundary clean-worktree strategy operator packet row
is `ready_blocked`: it packages the strategy for local operator review while
packet send, packet persistence, strategy application, git mutation, cleanup,
evidence recording, evidence persistence, release, canary activation, and live
execution remain disabled.

The dirty worktree release-boundary clean-worktree strategy operator packet
non-send readback row is `ready_blocked`: it proves the packet is visible,
unsent, and unpersisted while readback persistence, strategy application, git
mutation, cleanup, evidence recording, evidence persistence, release, canary
activation, and live execution remain disabled.

The dirty worktree release-boundary clean-worktree strategy operator packet
git-mutation boundary readback row is `ready_blocked`: it makes git add, index
mutation, commit, push, reset, checkout, revert, cleanup, and delete boundaries
explicit while packet send, packet persistence, readback persistence, strategy
application, evidence recording, evidence persistence, release, canary
activation, and live execution remain disabled.

The dirty worktree release-boundary clean-worktree strategy operator decision
checklist row is `ready_blocked`: it collapses the clean-worktree strategy
packet and git-boundary readbacks into seven pending operator decision
checklist entries while decision recording, approval acceptance, evidence
recording, git mutation, cleanup, delete, strategy application, release, canary
activation, and live execution remain disabled.

The dirty worktree release-boundary clean-worktree strategy operator decision
checklist packet readback row is `ready_blocked`: it renders the seven pending
operator decision checklist entries as a visible local packet/readback while
packet send, packet persistence, readback persistence, decision recording,
approval acceptance, evidence recording, git mutation, cleanup, release, canary
activation, and live execution remain disabled.

The dirty worktree release-boundary clean-worktree strategy operator decision
recording boundary readback row is `ready_blocked`: it makes decision recording,
decision persistence, decision receipt persistence, and approval acceptance
boundaries explicit for the same seven dirty-worktree strategy groups while
packet send, packet persistence, readback persistence, evidence recording, git
mutation, cleanup, release, canary activation, and live execution remain
disabled.

The dirty worktree release-boundary clean-worktree strategy operator approval
acceptance boundary readback row is `ready_blocked`: it makes approval request,
approval acceptance, approval recording, approval receipt persistence, decision
recording, and evidence recording boundaries explicit for the same seven
dirty-worktree strategy groups while packet send, packet persistence, readback
persistence, git mutation, cleanup, release, canary activation, and live
execution remain disabled.

The dirty worktree release-boundary clean-worktree strategy operator evidence
recording boundary readback row is `ready_blocked`: it makes evidence recording,
evidence persistence, and evidence receipt persistence boundaries explicit for
the same seven dirty-worktree strategy groups while approval request, approval
acceptance, approval recording, approval receipt persistence, decision
recording, packet send, packet persistence, readback persistence, git mutation,
cleanup, release, canary activation, and live execution remain disabled.

The dirty worktree release-boundary release risk snapshot row is
`ready_blocked`: it collapses the same seven dirty-worktree strategy groups into
one critical, four high, and two medium release-risk entries while snapshot
persistence, evidence recording, approval acceptance, decision recording, git
mutation, cleanup, release, canary activation, and live execution remain
disabled.

The controlled-live readiness row is deliberately `ready_blocked`: it is ready
as an audit surface and blocked as a live cutover. It requires the operator
approval, soak/readback, credential boundary, Gateway/Native/Telegram POST
boundary, rollback rehearsal, and kill-switch blockers to stay visible.

The controlled-live denial readback row is also `ready_blocked`: seven blockers
are queryable and operator-facing through stable readback routes, but waiver,
acceptance, approval request, readback persistence, and live execution remain
disabled.

The controlled-live required evidence gap operator packet attachment transport
boundary readback row is `ready_blocked`: it makes the Gateway/Auth, Native
POST, Telegram transport, and channel send closed boundary operator-visible for
those seven attached readbacks, but approval request, approval acceptance,
evidence recording, credential reads, blocker waiver, packet send, attachment
send, packet persistence, attachment persistence, readback persistence, transport
mutation, and live execution remain disabled.

The controlled-live required evidence gap operator packet attachment credential
boundary readback row is `ready_blocked`: it makes credential reads, credential
material loads, credential value exposure, and credential handle resolution
closed and operator-visible for those seven attached readbacks, but approval
request, approval acceptance, evidence recording, blocker waiver, packet send,
attachment send, packet persistence, attachment persistence, readback
persistence, transport mutation, and live execution remain disabled.

The controlled-live required evidence gap operator packet attachment rollback
rehearsal boundary readback row is `ready_blocked`: it makes rollback rehearsal
execution, rollback execution, rehearsal recording, and rehearsal receipt
persistence closed and operator-visible for those seven attached readbacks, but
approval request, approval acceptance, evidence recording, blocker waiver,
credential reads, packet send, attachment send, packet persistence, attachment
persistence, readback persistence, transport mutation, and live execution remain
disabled.

The controlled-live required evidence gap operator packet attachment kill-switch
rehearsal boundary readback row is `ready_blocked`: it makes kill-switch
rehearsal execution, kill-switch mutation, rehearsal recording, and rehearsal
receipt persistence closed and operator-visible for those seven attached
readbacks, but rollback rehearsal execution, rollback execution, approval
request, approval acceptance, evidence recording, blocker waiver, credential
reads, packet send, attachment send, packet persistence, attachment persistence,
readback persistence, transport mutation, and live execution remain disabled.

The Phase 6 controlled-live operator readiness dashboard is intentionally not a
new current-reality matrix capability row. It consumes this matrix and the
Phase 5n kill-switch rehearsal boundary readback, then collapses the current
ready rows plus seven unchanged-missing blockers into one operator-facing
dashboard.

The controlled-live operator packet preview row is `ready_blocked`: the packet
assembles scope, payload hash, rollback owner, all seven blocker readbacks, and
required evidence, but approval request, approval recording, packet persistence,
readback persistence, denial acceptance, blocker waiver, and live execution
remain disabled.

The controlled-live operator packet non-send readback row is `ready_blocked`: it
proves the packet is visible, unsent, unpersisted, and still not an approval
request, while transport, persistence, channel send, and live execution remain
disabled.

The controlled-live required evidence collection plan row is `ready_blocked`: it
lists the evidence required for all seven blockers, but evidence recording,
approval acceptance, credential reads, blocker waiver, persistence, and live
execution remain disabled.

The controlled-live required evidence readback index row is `ready_blocked`: it
makes those seven evidence requirements queryable and diffable, but evidence
recording, approval acceptance, credential reads, blocker waiver, readback
persistence, transport mutation, and live execution remain disabled.

The controlled-live required evidence gap summary row is `ready_blocked`: it
groups the seven missing evidence requirements by owner and cutover risk, but
approval acceptance, evidence recording, credential reads, blocker waiver,
readback persistence, transport mutation, and live execution remain disabled.

The controlled-live required evidence gap diff view row is `ready_blocked`: it
keeps those seven missing evidence gaps comparable across readbacks with stable
diff view keys and comparison anchors, but approval acceptance, evidence
recording, credential reads, blocker waiver, readback persistence, transport
mutation, and live execution remain disabled.

The controlled-live required evidence gap operator readback row is
`ready_blocked`: it presents those seven unchanged missing gaps as stable
operator-facing readbacks with readback keys, routes, display order, and blocked
status, but approval acceptance, evidence recording, credential reads, blocker
waiver, readback persistence, transport mutation, and live execution remain
disabled.

The controlled-live required evidence gap operator packet attachment row is
`ready_blocked`: it attaches those seven unchanged missing operator readbacks to
the local operator packet preview with attachment keys and routes, but approval
request, approval acceptance, evidence recording, credential reads, blocker
waiver, packet persistence, attachment persistence, readback persistence,
transport mutation, and live execution remain disabled.

The controlled-live required evidence gap operator packet attachment non-send
readback row is `ready_blocked`: it proves those seven attached readbacks are
visible, unsent, unpersisted, and still not an approval request, but approval
request, approval acceptance, evidence recording, credential reads, blocker
waiver, packet send, attachment send, packet persistence, attachment
persistence, readback persistence, transport mutation, and live execution remain
disabled.

## Memory/Filesystem Drift

Phase 1 restored the plugin lifecycle memory surfaces in the current checkout:

- `codex-rs/core-plugins/src/lifecycle_state_machine.rs`
- `codex-rs/core-plugins/src/lifecycle_phase_summary.rs`

Phase 3 restored the remaining workflow durable-store surfaces in the current
checkout:

- `codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs`
- `codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs`
- `codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs`

This gate treats all five memory references as explicit drift entries and now
requires all five to be resolved in the current checkout. The workflow adapter
is still feature-gated and does not enable event-log writes or live execution.

## Boundaries

This matrix is report-only. It does not:

- invoke tools
- register plugin tools
- install plugins
- mutate plugin cache
- write the tool invocation ledger
- request or resolve approvals
- mutate workflow event logs or SQLite state
- execute WorkGraph, replay, or rollback paths
- persist readback receipts
- mutate gateway/auth or Native POST routing
- call providers or models
- send channels
- package, release, or promote Public GA

## Next Move

Phase 14 adds
`phase14_dirty_worktree_release_boundary_actionable_clean_worktree_strategy_without_git_mutation`.
It turns the dirty-worktree grouping readbacks into an operator-visible
clean-worktree strategy, without staging, committing, reverting, deleting
unrelated work, persisting evidence, accepting approval, activating canary/live,
or mutating transport and runtime boundaries.

Phase 15 adds
`phase15_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_without_git_mutation`.
It packages the clean-worktree strategy for operator review while keeping git
mutation, cleanup, evidence persistence, approval acceptance, release, canary
activation, and live execution disabled.

Phase 16 adds
`phase16_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_without_git_mutation`.
It proves the operator packet remains visible but unsent and unpersisted
while keeping git mutation, cleanup, evidence persistence, approval acceptance,
release, canary activation, and live execution disabled.

Phase 17 adds
`phase17_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_without_git_mutation`.
It makes git add, commit, push, reset, checkout, revert, cleanup, and delete
boundaries explicit for the clean-worktree strategy packet without mutating the
index or applying the strategy.

Phase 18 adds
`phase18_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_without_git_mutation`.
It collapses the packet and git-boundary readbacks into an operator decision
checklist without accepting approvals, recording decisions, recording evidence,
mutating git state, cleaning up files, releasing, activating canary, or enabling
live.

Phase 19 adds
`phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation`.
It stabilizes a packet/readback view of the checklist without sending or
persisting packets, accepting approvals, recording decisions, recording
evidence, mutating git, cleaning up files, releasing, activating canary, or
enabling live.

Phase 20 added
`phase20_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback_without_recording`.
It makes the decision-recording, decision persistence, and decision receipt
persistence boundaries explicit without recording decisions, accepting
approvals, persisting evidence, mutating git, cleaning up files, releasing,
activating canary, or enabling live.

Phase 21 added
`phase21_dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_without_acceptance`.
It makes the approval request, approval acceptance, approval recording, and
approval receipt persistence boundaries explicit without accepting approvals,
recording decisions, persisting evidence, mutating git, cleaning up files,
releasing, activating canary, or enabling live.

Phase 22 added
`phase22_dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_without_recording`.
It makes the evidence recording, evidence persistence, and evidence receipt
persistence boundaries explicit without recording evidence, accepting
approvals, recording decisions, mutating git, cleaning up files, releasing,
activating canary, or enabling live.

Phase 23 added
`phase23_dirty_worktree_release_boundary_release_risk_snapshot_without_git_mutation`.
It stops suffix expansion and collapses dirty-worktree release risk into a fast
local snapshot without staging, committing, reverting, deleting unrelated files,
recording evidence, accepting approvals, releasing, activating canary, or
enabling live.

Phase 24 added
`phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation`.
It makes the clean-worktree strategy rehearsal visible in test-only mode without
staging, committing, reverting, deleting unrelated files, recording evidence,
accepting approvals, releasing, activating canary, or enabling live.

Phase 25 should add
`phase25_dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_without_git_mutation`.
It should summarize the test-only rehearsal outcome boundary without executing
probes, mutating git, cleaning up files, recording evidence, accepting
approvals, releasing, activating canary, or enabling live.
