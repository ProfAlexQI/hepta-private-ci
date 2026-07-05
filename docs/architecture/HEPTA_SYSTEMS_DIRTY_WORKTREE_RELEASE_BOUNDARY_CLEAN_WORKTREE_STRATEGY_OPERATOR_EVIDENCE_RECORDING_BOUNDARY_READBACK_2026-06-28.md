# Dirty Worktree Release Boundary Clean Worktree Strategy Operator Evidence Recording Boundary Readback

Date: 2026-06-28

Phase 22 consumes the Phase 21 clean-worktree strategy operator approval
acceptance boundary readback and renders an explicit evidence-recording
boundary readback. This is still a local read-model: it does not record
evidence, persist evidence, persist an evidence receipt, request an approval,
accept an approval, record an approval, persist an approval receipt, record a
decision, send or persist a packet, persist a readback, mutate git, clean up
files, or open any release/canary/live path.

## Inputs

- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback-report.sh`
- `codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback.rs`

The source approval-acceptance boundary is `ready_blocked`: all 7
dirty-worktree strategy groups are visible, unpersisted, queryable, diffable,
and still `pending_operator_decision`, while approval request/acceptance,
approval recording, approval receipt persistence, decision recording, and
evidence recording remain blocked.

## Boundary Scope

The evidence-recording boundary readback scope is:

- `boundary_readback_id`: `dirty-worktree.release-boundary.clean-worktree-strategy.operator-evidence-recording-boundary-readback.v1`
- `boundary_readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-evidence-recording-boundary/v1`
- `source_approval_acceptance_boundary_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-approval-acceptance-boundary/v1`
- `readback_mode`: `operator_evidence_recording_boundary_readback_only`
- `evidence_recording_boundary`: `blocked`
- `evidence_persistence_boundary`: `blocked`
- `evidence_receipt_boundary`: `blocked`
- `approval_acceptance_boundary`: `blocked`
- `decision_recording_boundary`: `blocked`
- `git_mutation_boundary`: `blocked`

## Boundary Entries

Each Phase 21 approval-acceptance boundary entry becomes one
evidence-recording boundary entry with:

- source approval-acceptance boundary key and route
- source packet and source packet readback key/route
- evidence-recording boundary key and route
- evidence checkpoint, approval checkpoint, and decision checkpoint
- group type and source bucket
- source counts
- owner/review lane
- recommended strategy and operator action
- required evidence
- `decision_state=pending_operator_decision`
- `evidence_recording_state=evidence_recording_blocked`
- `evidence_persistence_state=evidence_persistence_blocked`
- `evidence_receipt_state=evidence_receipt_blocked`
- `approval_request_state=approval_request_blocked`
- `approval_acceptance_state=approval_acceptance_blocked`
- `approval_recording_state=approval_recording_blocked`
- `approval_receipt_state=approval_receipt_blocked`
- `decision_recording_state=decision_recording_blocked`

All entries remain operator-visible, queryable, and diffable. evidence
recording, evidence persistence, evidence receipt persistence, approval
request, approval acceptance, approval recording, approval receipt persistence,
decision recording, packet send, packet persistence, readback persistence, git
mutation, cleanup, delete, strategy application, release, canary activation,
and live execution remain blocked.

evidence recording, evidence persistence, evidence receipt persistence, approval request, approval acceptance, approval recording, approval receipt persistence, decision recording, packet send, packet persistence, readback persistence, git mutation, cleanup, delete, strategy application, release, canary activation, and live execution remain blocked.

## Closed Boundary

Phase 22 keeps the release boundary closed:

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy
application, evidence recording, evidence persistence, evidence receipt
persistence, approval request, approval acceptance, approval recording,
approval receipt persistence, decision recording, decision persistence,
decision receipt persistence, packet send, packet persistence, readback
persistence, package, release, Public GA, canary activation, live activation,
or live execution.

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, evidence recording, evidence persistence, evidence receipt persistence, approval request, approval acceptance, approval recording, approval receipt persistence, decision recording, decision persistence, decision receipt persistence, packet send, packet persistence, readback persistence, package, release, Public GA, canary activation, live activation, or live execution.

The evidence-recording boundary readback is `ready_blocked`: ready for local
operator inspection, blocked for evidence recording/persistence, approval
request/acceptance/recording, approval receipt persistence, decision recording,
git mutation, cleanup, release, canary, or live movement.

## Next

The recommended next gate is:

`phase23_dirty_worktree_release_boundary_release_risk_snapshot_without_git_mutation`

Phase 23 should stop extending the decision-boundary suffix ladder and collapse
the dirty-worktree release risk into a fast local snapshot without mutating git,
cleaning up files, recording evidence, accepting approvals, releasing,
activating canary, or executing live paths.
