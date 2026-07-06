# Hepta Systems Dirty Worktree Release Boundary Test-Only Clean Worktree Strategy Rehearsal - 2026-06-28

This note records Phase 24:
`phase24_dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_without_git_mutation`.
It consumes the Phase 23 release-risk snapshot and maps each dirty-worktree
bucket into a local, visible-only test rehearsal boundary.

## Scope

The rehearsal mode is
`test_only_no_git_mutation_no_cleanup_no_evidence_recording`.

The report keeps the same seven release-risk buckets:

- `artifacts`
- `codex-rs`
- `docs`
- `plugins`
- `scripts`
- `cross_lane_or_unowned`
- `hepta_systems_owned`

Each entry keeps its source release-risk snapshot key and route, adds a
test-only rehearsal key and route, and identifies the local gate that would need
to pass before any clean-worktree convergence could be considered.

## Local Gates

Phase 24 does not run these probes. It makes the owner attribution, targeted
Rust, plugin, script, owned-lane, artifact, and doc gates visible and queryable:

Required local gates phrase: owner attribution, targeted Rust, plugin, script, owned-lane, artifact, and doc gates.

- `cross_lane_or_unowned`: owner attribution and freeze gate
- `codex-rs`: targeted Rust gate
- `plugins`: plugin surface gate
- `scripts`: script syntax gate
- `hepta_systems_owned`: owned-lane freeze gate
- `artifacts`: artifact classification gate
- `docs`: doc evidence consistency gate

The convergence states remain blocked or candidate-only. Cross-lane or unowned
changes remain blocked until owner attribution. Runtime, plugin, script, owned
lane, artifact, and doc buckets remain candidates only after their local gate.

## Boundary

The rehearsal is visible-only. It does not execute test probes, mutate git,
clean up files, delete files, persist readback, record evidence, accept
approval, record a decision, package, release, start canary, activate live, or
execute live paths.

Closed Phase 24 boundary: no git add, commit, push, reset, checkout, revert,
cleanup, delete, test probe execution, evidence recording, approval acceptance,
decision recording, package, release, Public GA, canary activation, live
activation, or live execution.

Gate phrase: no git add, commit, push, reset, checkout, revert, cleanup, delete, test probe execution, evidence recording, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution.

## Verification

The gate validates:

- Phase 23 release-risk snapshot is ready, visible, and unpersisted
- seven stable rehearsal keys and routes are present
- the owner attribution, targeted Rust, plugin, script, owned-lane, artifact,
  and doc gates are all represented exactly once
- every entry is visible, queryable, diffable, mutation-free, and test-only
- test probe execution, git mutation, cleanup/delete, evidence recording,
  approval acceptance, decision recording, release, canary, and live execution
  remain blocked
- targeted hepta-runtime Rust tests pass

## Next Move

Phase 25 should add
`phase25_dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_without_git_mutation`.
It should turn the Phase 24 test-only rehearsal boundary into an outcome
readback without executing probes, staging, committing, reverting, deleting
files, recording evidence, accepting approvals, releasing, activating canary, or
enabling live.
