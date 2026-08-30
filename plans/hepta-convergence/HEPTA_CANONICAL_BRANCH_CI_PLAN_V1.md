# Hepta Q0.31 — Canonical branch CI coverage and live-protection boundary

## Purpose

Q0.31 is a repository-governance repair stacked directly after the selected
Q0.30 direct-Bazel execution closure. It closes the source-controlled gap where
the repository's actual default branch, `integration/vnext-main-20260811`, was
not included in either canonical `blocking-ci` push coverage or the specialized
Hepta vNext qualification push coverage.

This package changes no Rust runtime, schema, migration, model, provider,
product caller, external effect, operator acceptance, promotion, release, or
`CALLERS` authority.

## Exact source base

```text
repository = ProfHepta/hepta-private-ci
selected parent PR = #155
parent branch = codex/hepta-intelligence-q0-30-direct-bazel-20260830
parent commit = 58f7df731a8c0febd8118be9a34cd69663089253
parent tree = 528884f1d8d5bc8b32326873107adc3b49334ce0
```

## Exact source surface

Exactly these six paths may change:

```text
.github/workflows/blocking-ci.yml
.github/workflows/hepta-vnext-qualification.yml
.github/workflows/repo-checks.yml
plans/hepta-convergence/HEPTA_CANONICAL_BRANCH_CI_PLAN_V1.md
plans/hepta-convergence/HEPTA_CANONICAL_BRANCH_POLICY_V1.json
scripts/verify-hepta-canonical-branch-ci.py
```

The repair:

- retains `pull_request` as the ordinary merge-candidate entrypoint;
- retains `main` push coverage and adds the exact current default branch to
  `blocking-ci`;
- adds both `main` and the exact current default branch to Hepta vNext
  qualification while retaining `vnext-main` and
  `integration/vnext-main-full-ci-*`;
- executes one strict canonical-branch verifier from ordinary `repo-checks`;
- preserves stable required context `CI required`, the Windows gnullvm boundary,
  candidate-level stale-run cancellation, and read-only specialized workflow
  permissions;
- parses the machine policy with duplicate-key rejection and exact recursive
  field sets;
- keeps source trigger coverage separate from live GitHub ruleset enforcement.

## Live ruleset boundary

The intended live rule targets the exact current default branch, rejects branch
deletion and non-fast-forward updates, requires pull requests, resolved review
threads and stable context `CI required`, and preserves independent-review and
operator-evidence gates.

The GitHub ruleset collection was empty at Q0.31 construction time. The checked
policy therefore retains `ruleset.applied=false` and a null readback receipt.
Only an administrator API mutation followed by an exact live readback may
advance those fields. Source, PR text, CI success or repository-owner intent is
not enforcement evidence.

## Exit gates

Source closure requires the strict verifier, Python compilation, workflow YAML
validation, exact six-path topology and `git diff --check`. Repository
qualification additionally requires assigned-runner exact-head and synthetic-
merge results. Live protection additionally requires an administrator readback.
None of these facts grant runtime, production, operator, promotion, release or
`CALLERS` authority.
