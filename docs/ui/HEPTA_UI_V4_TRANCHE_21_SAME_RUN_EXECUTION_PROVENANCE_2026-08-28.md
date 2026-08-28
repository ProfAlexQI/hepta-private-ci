# Hepta UI v4 Tranche 21 — Same-run execution provenance

## Purpose

Tranche 20 introduced an independent completed-run execution provenance audit. This tranche retains that replay path and additionally places the same provenance verdict at the end of the canonical exact-head Windows material workflow.

The same Actions run now has two independent final boundaries:

1. `Canonical exact-head qualification index` proves source, exact Makepad materialization, three-platform compile/test evidence, and optional governed Windows runtime receipts.
2. `Canonical exact-head execution provenance audit` proves that the qualified jobs actually executed the platform-specific steps on the required runner classes and that all typed runtime identities and request sequences remain exact.

## Same-run execution model

The provenance job runs with `if: always()` after source, materialization, compile, optional runtime, and qualification-index jobs. It reads only the current `GITHUB_RUN_ID`, verifies the exact candidate commit/tree, downloads only candidate-suffixed artifacts from that run, rejects unsafe archive paths, and invokes a blob-bound wrapper around the canonical completed-run auditor.

The wrapper is pinned to canonical audit Git blob:

`0ad77b0f629509ed0e8c6f4affe54dedd2ddeabb`

It adds one bounded mode:

`--allow-in-progress-run`

In this mode the current workflow may be `in_progress` with a null conclusion because the provenance job itself is still running. All upstream jobs required by the audit must already be completed with their normal success/skip contracts. A completed run still requires conclusion `success`.

## Output states

A normal pull request or dispatch without the governed compositor producer may emit only:

`PASS_EXACT_HEAD_PRE_RUNTIME_PROVENANCE`

A dispatch with governed Windows runtime enabled may emit only:

`PASS_EXACT_HEAD_WINDOWS_RUNTIME_PROVENANCE`

Any missing, failed, skipped, stale, duplicated, label-drifting, step-drifting, identity-drifting, or artifact-drifting evidence emits:

`FAIL_EXACT_HEAD_EXECUTION_PROVENANCE`

## Evidence requirements

The audit still requires:

- exact source, materialization, Ubuntu, Windows, macOS, and qualification-index jobs;
- nonzero runner IDs, nonempty runner names, expected runner labels, and nonempty successful steps;
- Ubuntu/macOS non-Windows producer stub compilation;
- Windows full producer compilation against the patched API;
- candidate-bound materialization and pinned Makepad revision;
- qualification-index job and receipt bound to the same run;
- when requested, governed labels `self-hosted`, `Windows`, `X64`, `hepta-ui-dwm`;
- typed root/transient WindowIds and HWNDs;
- request sequences exactly `1`, `2`, `3`;
- Mica, Acrylic, explicit None rollback, and exact Destroyed evidence.

## Safety boundary

The completed-run audit workflow remains available as an independent replay/audit layer. Same-run PASS does not replace that later replay and does not bind the product material host.

Even `PASS_EXACT_HEAD_WINDOWS_RUNTIME_PROVENANCE` keeps:

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

A Windows-runtime provenance PASS permits only a separate product-host integration review tranche.
