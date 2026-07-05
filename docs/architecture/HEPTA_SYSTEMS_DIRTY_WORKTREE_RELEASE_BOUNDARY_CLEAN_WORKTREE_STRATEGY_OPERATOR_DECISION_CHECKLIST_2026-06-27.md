# Dirty Worktree Release Boundary Clean Worktree Strategy Operator Decision Checklist

Date: 2026-06-27

Phase 18 consumes the Phase 17 clean-worktree strategy operator packet
git-mutation boundary readback and collapses it into an operator decision
checklist. This is still a local read-model: it does not record a decision,
accept an approval, apply a strategy, mutate git, clean up files, or open any
release/canary/live path.

## Inputs

- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback-report.sh`
- `codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback.rs`

The source git-boundary readback is `ready_blocked`: the clean-worktree strategy
packet is visible, unsent, unpersisted, and explicitly blocked from git add,
commit, push, reset, checkout, revert, cleanup, delete, strategy application,
evidence recording, release, canary activation, and live execution.

## Checklist Scope

The checklist scope is:

- `checklist_id`: `dirty-worktree.release-boundary.clean-worktree-strategy.operator-decision-checklist.v1`
- `checklist_route`: `checklist://release-boundary/dirty-worktree/clean-worktree-strategy/operator-decision/v1`
- `source_git_boundary_readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/v1`
- `checklist_mode`: `operator_decision_checklist_only`
- `decision_recording_boundary`: `blocked`
- `git_mutation_boundary`: `closed`
- `cleanup_boundary`: `blocked`
- `evidence_boundary`: `blocked`

## Checklist Entries

Each Phase 17 git-boundary readback entry becomes one checklist entry with:

- source git-boundary readback key and route
- checklist key and route
- decision checkpoint
- group type and source bucket
- source counts
- owner/review lane
- recommended strategy and operator action
- required evidence
- `decision_state=pending_operator_decision`
- `checklist_state=ready_blocked_pending_operator_decision`

All entries remain operator-visible, queryable, and diffable. Decision
recording, approval acceptance, evidence recording, git mutation, cleanup,
delete, strategy application, release, canary activation, and live execution
remain blocked.

decision recording, approval acceptance, evidence recording, git mutation, cleanup, delete, strategy application, release, canary activation, and live execution remain blocked.

## Closed Boundary

Phase 18 keeps the release boundary closed:

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy
application, decision recording, approval acceptance, evidence recording,
evidence persistence, packet send, packet persistence, readback persistence,
package, release, Public GA, canary activation, live activation, or live
execution.

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, decision recording, approval acceptance, evidence recording, evidence persistence, packet send, packet persistence, readback persistence, package, release, Public GA, canary activation, live activation, or live execution.

The checklist is `ready_blocked`: ready for local operator inspection, blocked
for any cleanup, git mutation, approval acceptance, evidence persistence,
release, canary, or live movement.

## Next

The recommended next gate is:

`phase19_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_without_git_mutation`

Phase 19 should make the operator decision checklist packet/readback shape
stable without recording decisions, accepting approvals, applying cleanup,
mutating git, persisting evidence, activating canary, or executing live paths.
