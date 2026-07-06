# Hepta Systems Dirty Worktree Release Boundary Release Risk Snapshot - 2026-06-28

This note records Phase 23:
`phase23_dirty_worktree_release_boundary_release_risk_snapshot_without_git_mutation`.
It consumes the Phase 22 evidence-recording boundary readback and collapses the
same seven dirty-worktree strategy groups into a fast local release-risk
snapshot.

## Scope

The snapshot is local and read-only. It does not extend the approval/evidence
suffix ladder. Its mode is `fast_local_release_risk_snapshot_only`.

The snapshot keeps seven operator-visible entries:

- `artifacts`
- `codex-rs`
- `docs`
- `plugins`
- `scripts`
- `cross_lane_or_unowned`
- `hepta_systems_owned`

Each entry keeps its source evidence boundary key and route, adds a release-risk
snapshot key and route, and carries:

- owner hint
- review lane
- recommended clean-worktree strategy
- critical, high, and medium release-risk tiers
- release blocker
- release blocker state
- test-only rehearsal action

## Risk Tiers

The snapshot classifies one critical bucket, four high-risk buckets, and two
medium-risk buckets:

- critical: `cross_lane_or_unowned`
- high: `codex-rs`, `plugins`, `scripts`, `hepta_systems_owned`
- medium: `artifacts`, `docs`

Every entry remains a test-only clean-worktree rehearsal candidate, but no
rehearsal is executed in this phase.

## Boundary

Release cutover remains blocked by the dirty worktree. Git mutation, cleanup,
delete, evidence recording, approval acceptance, decision recording, package
creation, release writes, Public GA, canary activation, live activation, and
live execution remain blocked.

Closed Phase 23 boundary: no git add, commit, push, reset, checkout, revert,
cleanup, delete, evidence recording, approval acceptance, decision recording,
package, release, Public GA, canary activation, live activation, or live
execution.

Gate phrase: no git add, commit, push, reset, checkout, revert, cleanup, delete, evidence recording, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution.

## Verification

The gate validates:

- Phase 22 evidence-recording boundary readback is ready and unpersisted
- the snapshot has seven stable snapshot keys and routes
- one critical, four high, and two medium release-risk entries are present
- every entry is visible, queryable, diffable, and a test-only rehearsal
  candidate
- release cutover, git mutation, cleanup/delete, evidence recording, approval
  acceptance, decision recording, canary activation, and live execution remain
  blocked
- targeted hepta-runtime Rust tests pass

## Next Move

Phase 24 should add
`phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation`.
It should rehearse the clean-worktree strategy in test-only mode without staging,
committing, reverting, deleting unrelated files, recording evidence, accepting
approvals, releasing, activating canary, or enabling live.
