# Hepta Systems Dirty Worktree Owner-Freeze Source Cache

## Purpose

This source-cache keeps dirty worktree owner-freeze as a short canonical fact
instead of making every downstream gate walk the full inventory, grouping,
owner-freeze, packet, git-mutation, and release-risk chain directly.

It groups the current dirty worktree by lane/owner/risk while keeping the
existing release boundary closed. The source-cache is readback-only: it does
not create a clean scoped worktree, does not assign owners, does not apply a
freeze, and does not mutate git.

## Contract

The report must prove:

- dirty inventory is present and tracked/untracked totals are internally
  consistent.
- grouping-freeze sources expose top-level buckets plus the
  `hepta_systems_owned` owner scope.
- owner-freeze outcome and operator packet readbacks remain visible-only.
- release-risk buckets remain dynamic and may include critical, high, and
  medium tiers without hard-coded dirty counts.
- the source-cache preserves the clean scoped worktree blocker before any
  status canary or release/live path.
- every bucket remains queryable and diffable by lane/owner/risk.
- `auditable_inventory` is rendered from the current `git status --short`
  readback into a compact lane/owner/risk inventory. Its dirty totals are
  dynamic and must be re-read by the gate; the contract intentionally does not
  freeze a hard-coded count such as 349.
- the auditable inventory remains readback-only and is not persisted as owner
  assignment, freeze application, classification evidence, or release evidence.

## Side-Effect Boundary

This source-cache is report-only. It performs no git add, commit, push, reset,
checkout, revert, cleanup, delete, owner assignment persistence, freeze
application, classification persistence, packet send, packet persistence,
evidence recording, approval acceptance, decision recording, package, release,
Public GA, canary activation, live activation, or live execution.

Closed boundary summary: no git add, commit, push, reset, checkout, revert, cleanup, delete, owner assignment persistence, freeze application, classification persistence, packet send, packet persistence, evidence recording, approval acceptance, decision recording, package, release, Public GA, canary activation, live activation, or live execution.
