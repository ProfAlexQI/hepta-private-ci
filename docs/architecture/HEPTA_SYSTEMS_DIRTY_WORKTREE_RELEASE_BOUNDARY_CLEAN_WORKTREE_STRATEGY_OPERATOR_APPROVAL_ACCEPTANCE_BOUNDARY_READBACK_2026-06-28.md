# Dirty Worktree Release Boundary Clean Worktree Strategy Operator Approval Acceptance Boundary Readback

Date: 2026-06-28

Phase 21 consumes the Phase 20 clean-worktree strategy operator decision
recording boundary readback and renders an explicit approval-acceptance
boundary readback. This is still a local read-model: it does not request an
approval, accept an approval, record an approval, persist an approval receipt,
record a decision, persist decision evidence, send or persist a packet, persist
a readback, mutate git, clean up files, or open any release/canary/live path.

## Inputs

- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback-report.sh`
- `codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback.rs`

The source decision-recording boundary is `ready_blocked`: all 7 dirty-worktree
strategy groups are visible, unpersisted, queryable, diffable, and still
`pending_operator_decision`, while decision recording and approval acceptance
remain blocked.

## Boundary Scope

The approval-acceptance boundary readback scope is:

- `boundary_readback_id`: `dirty-worktree.release-boundary.clean-worktree-strategy.operator-approval-acceptance-boundary-readback.v1`
- `boundary_readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-approval-acceptance-boundary/v1`
- `source_decision_recording_boundary_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision-recording-boundary/v1`
- `readback_mode`: `operator_approval_acceptance_boundary_readback_only`
- `approval_request_boundary`: `blocked`
- `approval_acceptance_boundary`: `blocked`
- `approval_recording_boundary`: `blocked`
- `approval_receipt_boundary`: `blocked`
- `decision_recording_boundary`: `blocked`
- `evidence_recording_boundary`: `blocked`

## Boundary Entries

Each Phase 20 decision-recording boundary entry becomes one
approval-acceptance boundary entry with:

- source decision-recording boundary key and route
- source packet and source packet readback key/route
- approval-acceptance boundary key and route
- approval checkpoint and decision checkpoint
- group type and source bucket
- source counts
- owner/review lane
- recommended strategy and operator action
- required evidence
- `decision_state=pending_operator_decision`
- `approval_request_state=approval_request_blocked`
- `approval_acceptance_state=approval_acceptance_blocked`
- `approval_recording_state=approval_recording_blocked`
- `approval_receipt_state=approval_receipt_blocked`
- `decision_recording_state=decision_recording_blocked`
- `decision_persistence_state=decision_persistence_blocked`

All entries remain operator-visible, queryable, and diffable. approval request,
approval acceptance, approval recording, approval receipt persistence, decision
recording, evidence recording, packet send, packet persistence, readback
persistence, git mutation, cleanup, delete, strategy application, release,
canary activation, and live execution remain blocked.

approval request, approval acceptance, approval recording, approval receipt persistence, decision recording, evidence recording, packet send, packet persistence, readback persistence, git mutation, cleanup, delete, strategy application, release, canary activation, and live execution remain blocked.

## Closed Boundary

Phase 21 keeps the release boundary closed:

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy
application, approval request, approval acceptance, approval recording,
approval receipt persistence, decision recording, decision persistence,
decision receipt persistence, evidence recording, evidence persistence, packet
send, packet persistence, readback persistence, package, release, Public GA,
canary activation, live activation, or live execution.

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, approval request, approval acceptance, approval recording, approval receipt persistence, decision recording, decision persistence, decision receipt persistence, evidence recording, evidence persistence, packet send, packet persistence, readback persistence, package, release, Public GA, canary activation, live activation, or live execution.

The approval-acceptance boundary readback is `ready_blocked`: ready for local
operator inspection, blocked for approval request/acceptance/recording,
approval receipt persistence, decision recording, evidence persistence, git
mutation, cleanup, release, canary, or live movement.

## Next

The recommended next gate is:

`phase22_dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_without_recording`

Phase 22 should make the evidence recording boundary explicit without
recording evidence, accepting approvals, recording decisions, mutating git,
applying cleanup, persisting receipts, activating canary, or executing live
paths.
