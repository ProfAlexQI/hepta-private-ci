# Hepta UI v4 Tranche 16 — exact-candidate and materialization hardening

## Purpose

GitHub pull-request workflows default to the synthetic merge ref. That ref is
useful for mergeability testing, but it is not the PR head commit. UI
qualification receipts must therefore select
`github.event.pull_request.head.sha || github.sha`, check out that exact object,
and verify `git rev-parse HEAD` before producing evidence.

This tranche also makes vendored Makepad materialization fail closed with a
machine-readable failure receipt. A failed patch command may no longer disappear
behind `tee`, and artifact upload must preserve the bounded failure reason.

## Source changes

- harden `hepta-ui-v4-materialize-makepad-windows-hook` with PASS/FAIL receipts;
- add self-tests to the exact-blob repair wrapper;
- add a Draft 2020-12 receipt Schema;
- add one canonical exact-candidate workflow;
- add a source gate checking exact checkout, pipefail, bounded failures, and
  authority containment.

## Evidence boundary

Source success may claim only
`PASS_EXACT_CANDIDATE_MATERIALIZATION_HARDENING_SOURCE_ONLY`.

It does not prove patched Makepad compilation, DWM runtime, complete Windows
material, product host binding, device qualification, or any production/effect
authority.
