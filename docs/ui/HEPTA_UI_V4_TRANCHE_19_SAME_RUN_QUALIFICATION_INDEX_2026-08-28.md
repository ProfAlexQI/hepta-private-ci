# Hepta UI v4 Tranche 19 — Same-run exact-head qualification index

Date: 2026-08-28

## Purpose

Tranche 18 introduced a deterministic qualification-index producer and a replay
workflow for completed runs. That replay path is necessary for canonical-branch
automation and historical re-evaluation, but it cannot provide the final
pull-request check inside the source exact workflow itself.

This tranche adds one final job to
`hepta-ui-v4-windows-material-profile-exact` so every pull request and explicit
runtime dispatch receives a machine-readable aggregate verdict from the same
workflow run.

## Same-run ordering

```text
exact source contract
→ candidate-bound Makepad materialization
→ Ubuntu / Windows / macOS compile and focused tests
→ optional governed Windows compositor producer
→ Canonical exact-head qualification index
```

The final job uses `if: always()` so upstream failure still produces a bounded
`FAIL_EXACT_HEAD_QUALIFICATION_INDEX` receipt rather than only an absent check.

## Evidence identity

The final job verifies:

- the checked-out commit equals `HEPTA_CANDIDATE_SHA`;
- the current Actions run ID equals `GITHUB_RUN_ID`;
- the run workflow name is exact;
- the run `head_sha` equals the candidate;
- the repository identity is unchanged;
- source, materialization, compile, and optional runtime artifacts all end in
  the same candidate SHA;
- artifact workflow metadata, when present, has the same head SHA;
- all receipts bind the same candidate commit and tree.

The producer is invoked with `--allow-in-progress-run` only from this final job.
All required upstream jobs must already be completed with nonzero runners and
nonempty successful steps.

## Artifact safety

Artifact discovery is bounded to twelve attempts with five-second intervals.
Only exact artifact names composed of `A-Za-z0-9_.-` are accepted. ZIP entries
are rejected if they are absolute, drive-qualified, empty, or contain a `..`
path component.

Missing artifacts are not converted into infrastructure success. The producer
receives the available evidence and emits a fail-closed receipt.

## Output states

A normal pull request or dispatch without compositor runtime must end in:

```text
PASS_EXACT_HEAD_PRE_RUNTIME_QUALIFICATION
```

A dispatch with `run_windows_profile_probe=true` must end in:

```text
PASS_EXACT_HEAD_WINDOWS_RUNTIME_QUALIFICATION
```

Any missing, stale, duplicated, skipped, failed, or identity-drifting evidence
must end in:

```text
FAIL_EXACT_HEAD_QUALIFICATION_INDEX
```

## Authority boundary

Even a Windows-runtime qualification keeps:

```text
productBound=false
transientSystemMaterialBound=false
completeProfileBound=false
systemMaterialBound=false
nativeProductRuntime=false
deviceValidation=false
network=false
mutation=false
effect=false
liveAdapter=false
production=false
operatorAcceptance=false
promotion=false
release=false
```

A runtime PASS makes the isolated evidence eligible only for a separate
product-host integration review.
