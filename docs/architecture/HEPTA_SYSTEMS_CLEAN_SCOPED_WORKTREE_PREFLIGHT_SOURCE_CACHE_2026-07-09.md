# Hepta Systems Clean Scoped Worktree Preflight Source Cache

## Purpose

This source-cache makes the current clean scoped worktree blocker queryable
without forcing downstream gates to walk the full dirty inventory, strategy,
operator packet, git boundary, evidence, approval, decision, and test-only
rehearsal chain.

It does not create or clean a worktree. It is a readback-only preflight that
states why release, status canary, live execution, and Public GA remain blocked.

## Contract

The report must prove:

- dirty owner-freeze is already source-cached and the clean scoped worktree is
  absent
- clean-worktree strategies exist for every current owner-freeze bucket but are
  not applied
- operator decision checklist and packet readbacks remain visible-only,
  unsent, and unpersisted
- git mutation, cleanup, deletion, evidence recording, approval acceptance,
  decision recording, and test probe execution are all blocked for every bucket
- release cutover, status canary, live activation, live execution, and Public GA
  remain blocked

## Side-Effect Boundary

This source-cache is report-only: no git add, commit, push, reset, checkout,
revert, cleanup, delete, strategy application, operator packet send, packet
persistence, readback persistence, evidence recording, evidence persistence,
approval request, approval acceptance, approval recording, approval receipt
persistence, decision recording, decision persistence, test probe execution,
blocker waiver, package, release, canary start, live execution, or Public GA.

Closed boundary summary: no git add, commit, push, reset, checkout, revert, cleanup, delete, and no evidence recording, approval request, approval acceptance, decision recording, test probe execution, canary start, live execution, or Public GA.
