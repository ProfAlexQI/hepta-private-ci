# Dirty Worktree Release Boundary Clean Worktree Strategy Operator Decision Recording Boundary Readback

Date: 2026-06-28

Phase 20 consumes the Phase 19 clean-worktree strategy operator decision
checklist packet/readback and renders an explicit decision-recording boundary
readback. This is still a local read-model: it does not record a decision,
persist a decision, persist a decision receipt, accept an approval, send or
persist a packet, persist a readback, mutate git, clean up files, or open any
release/canary/live path.

## Inputs

- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback-report.sh`
- `codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback.rs`

The source packet/readback is `ready_blocked`: all 7 dirty-worktree strategy
groups are packet-visible, unsent, unpersisted, readback-visible,
readback-unpersisted, queryable, diffable, and still
`pending_operator_decision`.

## Boundary Scope

The decision-recording boundary readback scope is:

- `boundary_readback_id`: `dirty-worktree.release-boundary.clean-worktree-strategy.operator-decision-recording-boundary-readback.v1`
- `boundary_readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision-recording-boundary/v1`
- `source_packet_readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision-checklist-packet/v1`
- `readback_mode`: `operator_decision_recording_boundary_readback_only`
- `decision_recording_boundary`: `blocked`
- `decision_persistence_boundary`: `blocked`
- `decision_receipt_boundary`: `blocked`
- `approval_acceptance_boundary`: `blocked`

## Boundary Entries

Each Phase 19 packet/readback entry becomes one decision-recording boundary
entry with:

- source packet key and route
- source packet readback key and route
- decision-recording boundary key and route
- decision checkpoint
- group type and source bucket
- source counts
- owner/review lane
- recommended strategy and operator action
- required evidence
- `decision_state=pending_operator_decision`
- `recording_state=decision_recording_blocked`
- `persistence_state=decision_persistence_blocked`
- `receipt_state=decision_receipt_blocked`

All entries remain operator-visible, queryable, and diffable. decision
recording, decision persistence, decision receipt persistence, approval
acceptance, evidence recording, packet send, packet persistence, readback
persistence, git mutation, cleanup, delete, strategy application, release,
canary activation, and live execution remain blocked.

decision recording, decision persistence, decision receipt persistence, approval acceptance, evidence recording, packet send, packet persistence, readback persistence, git mutation, cleanup, delete, strategy application, release, canary activation, and live execution remain blocked.

## Closed Boundary

Phase 20 keeps the release boundary closed:

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy
application, decision recording, decision persistence, decision receipt
persistence, approval request, approval acceptance, evidence recording,
evidence persistence, packet send, packet persistence, readback persistence,
package, release, Public GA, canary activation, live activation, or live
execution.

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, decision recording, decision persistence, decision receipt persistence, approval request, approval acceptance, evidence recording, evidence persistence, packet send, packet persistence, readback persistence, package, release, Public GA, canary activation, live activation, or live execution.

The decision-recording boundary readback is `ready_blocked`: ready for local
operator inspection, blocked for any decision recording, approval acceptance,
evidence persistence, git mutation, cleanup, release, canary, or live movement.

## Next

The recommended next gate is:

`phase21_dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_without_acceptance`

Phase 21 should make the approval acceptance boundary explicit without
accepting approvals, recording decisions, mutating git, applying cleanup,
persisting evidence, activating canary, or executing live paths.
