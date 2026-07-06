# Dirty Worktree Release Boundary Actionable Clean Worktree Strategy

Date: 2026-06-27

Phase 14 converts the Phase 13 dirty-worktree grouping freeze operator readback into an actionable clean-worktree strategy read-model. It does not clean the worktree. It does not stage, commit, revert, delete, or apply the strategy.

## Inputs

- `scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback-report.sh`
- `codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_grouping_freeze_operator_readback.rs`

The report consumes the operator readback rows from Phase 13:

- top-level dirty buckets
- scope buckets
- stable readback keys
- diff keys
- owner/review lane hints
- unchanged `planned_not_applied` freeze state
- unchanged `not_recorded` evidence state

## Strategy Scope

The strategy scope is:

- `strategy_id`: `dirty-worktree.release-boundary.actionable-clean-worktree-strategy.v1`
- `strategy_route`: `readback://release-boundary/dirty-worktree/actionable-clean-worktree-strategy/v1`
- `strategy_mode`: `operator_strategy_only`
- `action_mode`: `no_git_mutation_no_cleanup_no_evidence_recording`
- `mutation_boundary`: `closed`

Each strategy entry maps one Phase 13 readback row to:

- a stable strategy key
- a stable strategy route
- source readback and diff keys
- source entry counts
- owner/review lane hints
- a recommended strategy
- an operator action
- a pending operator decision state
- the required clean-worktree decision record

## Current Strategy

The current report is expected to produce seven strategy rows from the live checkout:

- `artifacts`
- `codex-rs`
- `docs`
- `plugins`
- `scripts`
- `cross_lane_or_unowned`
- `hepta_systems_owned`

The strategies are intentionally conservative:

- `hepta_systems_owned_batch_review`
- `cross_lane_owner_review_required`
- `split_owned_and_cross_lane_review`
- `operator_classification_required`

The only action taken by this phase is to make those strategies queryable and operator-visible.

## Closed Boundary

Phase 14 has no git mutation boundary:

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, evidence recording, evidence persistence, approval acceptance, blocker waiver, package, release, Public GA, canary activation, live activation, or live execution.

The strategy remains `ready_blocked`. A clean worktree decision record is still required before any release or cutover boundary can close.

## Next

The recommended next gate is:

`phase15_dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_without_git_mutation`

Phase 15 should package the clean-worktree strategy for operator review, still without staging, committing, reverting, deleting, recording evidence, accepting approval, activating canary, or enabling live execution.
