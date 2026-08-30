# Hepta Q0.30 — Canonical branch CI coverage and protection boundary

## Purpose

Q0.30 is a repository-governance repair stacked directly after the selected
Q0.29 Windows execution-context closure. It closes the source-controlled gap
where the repository default branch `integration/vnext-main-20260811` is not
included in either canonical `blocking-ci` push coverage or the specialized
Hepta vNext qualification push coverage.

This package changes no Rust runtime, schema, migration, model, provider,
product caller, external effect, operator acceptance, promotion, release, or
`CALLERS` authority.

## Exact source base

```text
repository = ProfHepta/hepta-private-ci
selected parent PR = #150
parent branch = codex/hepta-intelligence-q0-29-execution-context-20260830
parent commit = 925f2871754f89c31051080e20a9bd0448302245
parent tree = 1f821a14a2c89a170c7251c44bd0e64ba13ad8a3
```

## Source changes

Exactly these six paths may change:

```text
.github/workflows/blocking-ci.yml
.github/workflows/hepta-vnext-qualification.yml
.github/workflows/repo-checks.yml
scripts/verify-hepta-canonical-branch-ci.py
plans/hepta-convergence/HEPTA_CANONICAL_BRANCH_CI_PLAN_V1.md
plans/hepta-convergence/HEPTA_CANONICAL_BRANCH_POLICY_V1.json
```

The repair:

- retains `pull_request` as the repository merge-candidate entrypoint;
- adds the exact current default branch to `blocking-ci` push coverage;
- adds `main` and the exact current default branch to Hepta vNext qualification
  push coverage while retaining `vnext-main` and
  `integration/vnext-main-full-ci-*`;
- executes a fail-closed canonical-branch verifier from ordinary `repo-checks`;
- preserves the stable required context `CI required` and candidate-level
  concurrency cancellation;
- keeps workflow permissions read-only;
- separates source trigger coverage from live GitHub ruleset enforcement.

## Ruleset boundary

The intended repository ruleset targets the current default branch, rejects
branch deletion and non-fast-forward updates, requires pull requests, resolved
review threads and the stable `CI required` context, and does not bypass the
independent-review or operator-evidence requirements.

Ruleset application is an administrator-side GitHub mutation. The checked-in
policy must retain `ruleset.applied=false` until an API readback proves that the
live rule is active on the exact default branch. Source presence, PR text, or a
local script is not live enforcement evidence.

## Exit gates

Q0.30 source PASS requires:

```text
python3 scripts/verify-hepta-canonical-branch-ci.py
python3 -m py_compile scripts/verify-hepta-canonical-branch-ci.py
workflow YAML parse for all three modified workflows
exact six-path diff and single-parent topology
git diff --check
```

Repository qualification additionally requires assigned-runner exact-head and
synthetic-merge results. Live branch protection additionally requires a
separate successful administrator readback. Neither source validation nor
ruleset creation grants runtime, production, operator, promotion, release, or
`CALLERS` authority.
