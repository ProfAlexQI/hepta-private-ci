# Current Reality Matrix Compact Cache Boundary Readback

This note closes the local readback step after the Temporal-Lite checkpoint and rollback anchor projection.

## Scope

- Surface: `current_reality_matrix_compact_cache_boundary_readback`.
- Source: the single-render current reality matrix boundary.
- Projection: capability counts, live-blocker state, dirty-worktree counts, and dashboard matrix-rerun status are projected into a compact in-memory readback.
- Dashboard change: the controlled-live dashboard gate should validate the matrix facts exposed by the dashboard report instead of invoking the full matrix report a second time.

## Boundary

This is a readback-only compact boundary. It consumes the single-render matrix summary, but it does not write a cache or persist compact state.

The closed boundary is explicit: no cache write, compact cache persistence, evidence recording, approval acceptance, decision recording, event-log write, SQLite write, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the dirty-worktree owner/freeze/classification operator packet git-mutation
boundary is represented as the 63rd matrix row, the next local gate is
`dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation`.

That gate should keep release-risk progress local and reversible while reducing
the clean-worktree blocker: no git index mutation, cleanup/delete, evidence
recording, approval acceptance, decision recording, operator packet send,
release, canary activation, or live execution.
