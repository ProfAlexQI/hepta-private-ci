# Dirty Worktree Release Boundary Inventory

This note records the Phase 11 local-only release-boundary inventory for the
Hepta systems lane.

## Scope

Phase 11 consumes the Phase 10 controlled canary readiness plan and adds a
read-only inventory of the current dirty worktree boundary. It uses:

- `git status --porcelain`
- `read_only_inventory_no_git_mutation`
- `readback://release-boundary/dirty-worktree/inventory/v1`

The inventory classifies the current checkout into:

- tracked changes
- untracked changes
- staged/index changes
- unstaged/worktree changes
- modified, deleted, added, renamed, and unmerged status counts
- Hepta systems-owned paths
- cross-lane or unowned paths
- top-level path buckets
- a bounded sample of entries for operator readback

The inventory is `ready_blocked`: it is ready as a local release-boundary
read-model, but the release boundary remains open while dirty worktree entries
exist.

## Boundary

This phase does not clean the worktree and does not decide ownership. It only
makes the boundary visible and queryable before any controlled canary or live
cutover.

Closed boundary: no git add, commit, push, reset, checkout, revert, cleanup,
delete, evidence recording, evidence persistence, package, release, Public GA,
canary activation, live activation, or live execution.

Closed git/release boundary: no git add, commit, push, reset, checkout, revert, cleanup, delete, evidence recording, evidence persistence, package, release, Public GA, canary activation, live activation, or live execution.

It also does not:

- stage files
- commit files
- revert files
- delete unrelated work
- accept or record approval
- waive blockers
- persist release evidence
- mutate Gateway/Auth, Native POST, Telegram transport, or channel routing

## Verification

The gate validates:

- Phase 10 controlled canary readiness remains ready-blocked
- dirty worktree entry count is non-zero
- tracked plus untracked counts equal total inventory count
- Hepta systems-owned plus cross-lane/unowned counts equal total inventory count
- top-level and scope buckets cover the whole inventory
- sample entries are bounded and queryable
- every git, cleanup, evidence, release, canary, and live mutation flag remains
  false
- targeted hepta-runtime Rust tests pass

Gate phrase: read_only_inventory_no_git_mutation.

## Next Move

Phase 12 should add
`phase12_dirty_worktree_release_boundary_grouping_freeze_plan_without_git_mutation`.
It should turn this inventory into a grouping/freeze plan for release evidence
without staging, committing, reverting, deleting unrelated work, persisting
evidence, accepting approval, activating canary/live, or mutating transport and
runtime boundaries.
