# Dirty Worktree Release Boundary Clean Worktree Strategy Operator Packet Git-Mutation Boundary Readback

Date: 2026-06-27

Phase 17 consumes the Phase 16 clean-worktree strategy operator packet non-send readback and makes the git-mutation boundary explicit. The packet is still visible, unsent, and unpersisted; this phase adds operator-readable proof that git add, commit, push, reset, checkout, revert, cleanup, and delete remain blocked.

## Inputs

- `scripts/hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback-report.sh`
- `codex-rs/hepta-runtime/src/dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback.rs`

The source non-send readback remains `ready_blocked`: it is suitable for local operator review, but it is not a release action, cleanup action, or git operation request.

## Readback Scope

The readback scope is:

- `readback_id`: `dirty-worktree.release-boundary.clean-worktree-strategy.operator-packet.git-mutation-boundary-readback.v1`
- `readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/git-mutation-boundary/v1`
- `source_non_send_readback_route`: `readback://release-boundary/dirty-worktree/clean-worktree-strategy/operator-packet/non-send/v1`
- `readback_mode`: `git_mutation_boundary_readback_only`
- `git_mutation_boundary`: `closed`
- `git_index_boundary`: `blocked`
- `cleanup_boundary`: `blocked`
- `deletion_boundary`: `blocked`

## Readback Entries

Each Phase 16 non-send readback entry becomes one git-mutation boundary readback entry with:

- source non-send readback key and route
- git-boundary readback key and route
- group type and source bucket
- source counts
- owner/review lane
- recommended strategy and operator action
- required evidence
- unchanged blocked git-mutation state

All entries remain operator-visible, queryable, and diffable. Their git-mutation state is `blocked` before and after this phase, with `git_mutation_state_delta=unchanged_blocked`.

## Closed Boundary

Phase 17 keeps the release boundary closed:

no git add, commit, push, reset, checkout, revert, cleanup, delete, strategy application, packet send, packet persistence, readback persistence, evidence recording, evidence persistence, approval request, approval acceptance, blocker waiver, package, release, Public GA, canary activation, live activation, or live execution.

The readback is `ready_blocked`: ready for local operator inspection, blocked for any real cleanup, git mutation, release, canary, or live movement.

## Next

The recommended next gate is:

`phase18_dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_without_git_mutation`

Phase 18 should collapse the clean-worktree strategy packet and git-boundary readbacks into an operator decision checklist. It should remain a checklist/readback surface only, not an approval acceptance, evidence recording, git mutation, cleanup, release, canary activation, or live execution path.
