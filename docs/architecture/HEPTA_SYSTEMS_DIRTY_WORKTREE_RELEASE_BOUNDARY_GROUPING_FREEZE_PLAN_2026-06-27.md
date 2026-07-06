# Dirty Worktree Release Boundary Grouping Freeze Plan

Date: 2026-06-27

Phase 12 adds a read-only grouping freeze plan for the Phase 11 dirty-worktree release-boundary inventory. It does not freeze the worktree in git, stage files, create commits, clean files, delete files, or record release evidence.

## Source

The source of truth is the Phase 11 report:

- `scripts/hepta-systems-dirty-worktree-release-boundary-inventory-report.sh`
- collection mode: `read_only_inventory_no_git_mutation`
- source command: `git status --porcelain`

Phase 12 consumes the Phase 11 `top_level_buckets` and `scope_buckets` and projects them into operator-visible grouping entries.

## Grouping Mode

The grouping mode is `top_level_and_scope_bucket`.

It produces two views:

- top-level repository buckets such as `scripts`, `codex-rs`, and `docs`
- scope buckets: `hepta_systems_owned` and `cross_lane_or_unowned`

Each group has a stable group key, readback route, owner hint, review lane, source count, tracked count, untracked count, and release evidence bucket placeholder.

## Freeze Mode

The freeze mode is `plan_only_not_applied`.

Every group is ready for operator review, but every group keeps:

- `freeze_state=planned_not_applied`
- `evidence_state=not_recorded`
- `freeze_applied=false`
- `git_mutation_allowed=false`
- `cleanup_allowed=false`
- `evidence_recording_allowed=false`
- `release_cutover_allowed=false`

This means the release boundary is easier to read and review, but nothing is accepted, waived, frozen, cleaned, staged, committed, or released.

## Closed Boundary

Phase 12 performs no git add, commit, push, reset, checkout, revert, cleanup, delete, evidence recording, evidence persistence, approval acceptance, blocker waiver, package, release, Public GA, canary activation, live activation, or live execution.

The gate requires every side-effect flag to stay false and requires the next migration step to remain a readback step:

`phase13_dirty_worktree_release_boundary_grouping_freeze_operator_readback_without_git_mutation`.

## Gate

Run:

```bash
scripts/hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan-gate.sh
```

The gate validates:

- Phase 11 inventory is ready and still open
- the Phase 12 Rust surface is exported by `hepta-runtime`
- every top-level and scope group is queryable and operator-visible
- every group remains `planned_not_applied`
- release cutover, canary activation, live activation, git mutation, cleanup, delete, evidence recording, and evidence persistence remain blocked
