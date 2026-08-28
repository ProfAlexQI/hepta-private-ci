# Hepta UI v4 Tranche 20 — exact-head execution provenance audit

## Purpose

This tranche adds an independent, completed-run audit above the canonical same-run qualification index. The audit does not replace the same-run index. It verifies that the index was produced from executable jobs with the platform-specific steps and runner labels required by the Windows material contract.

The audit distinguishes:

- `PASS_EXACT_HEAD_PRE_RUNTIME_PROVENANCE`;
- `PASS_EXACT_HEAD_WINDOWS_RUNTIME_PROVENANCE`;
- `FAIL_EXACT_HEAD_EXECUTION_PROVENANCE`.

A PASS is evidence integrity only. It does not bind the product material host or grant effect, production, promotion, or release authority.

## Findings closed

The previous index required successful source, materialization, and compile jobs, but its generic compile-step contract did not independently require the Windows-only full producer compilation or the non-Windows stub compilation. It also did not verify the actual governed runner labels, and runtime sequence ordering was skipped when a sequence value was not an integer.

This tranche closes those gaps by requiring:

1. exact platform-specific compile steps;
2. actual `ubuntu-latest`, `windows-latest`, and `macos-latest` labels;
3. actual governed runtime labels `self-hosted`, `Windows`, `X64`, and `hepta-ui-dwm`;
4. nonzero runner IDs, nonempty runner names, nonempty successful steps;
5. strict runtime identity types;
6. distinct root and transient WindowIds and HWNDs;
7. exact request sequences `1`, `2`, and `3`;
8. exact Destroyed identity;
9. a successful same-run qualification-index job and candidate-bound index receipt;
10. the pinned Makepad revision and candidate-bound materialization receipt.

## Required jobs

The auditor requires exactly one of each:

- `Exact-head full source chain and patcher self-test`;
- `Candidate-bound Makepad materialization`;
- `Exact-head compile and focused tests (ubuntu-latest)`;
- `Exact-head compile and focused tests (windows-latest)`;
- `Exact-head compile and focused tests (macos-latest)`;
- `Governed exact-head Windows Mica/Acrylic producer`;
- `Canonical exact-head qualification index`.

Pre-runtime qualification requires the governed runtime job to be skipped with no executed steps and no runtime artifact. Windows-runtime qualification requires the job to succeed on the governed label set and to have a matching runtime receipt.

## Platform-specific compile closure

Ubuntu and macOS jobs must contain:

- `Compile non-Windows producer stub`.

The Windows job must contain:

- `Compile full Windows producer against patched API`.

All three must also contain exact checkout verification, Makepad materialization, and the full format/check/focused-test step.

## Runtime receipt closure

Windows-runtime provenance requires:

```text
sequence 1: root Mica
sequence 2: dedicated popup Acrylic
sequence 3: explicit None rollback
then exact popup Destroyed
```

All sequences must be positive integers and exactly equal to `1`, `2`, and `3`. Window indexes and generations must be nonnegative integers. HWNDs must be positive decimal strings. Root and transient WindowIds and HWNDs must differ.

## Workflow model

`hepta-ui-v4-execution-provenance-audit.yml` supports:

- pull-request and push source validation;
- automatic audit after a completed `hepta-ui-v4-windows-material-profile-exact` run;
- manual replay of a supplied run ID;
- strict manual mode requiring Windows-runtime provenance.

The workflow uses read-only contents and actions permissions, validates the source run, safely downloads exact candidate-suffixed artifacts, and emits a bounded atomic audit receipt.

## Authority boundary

Even Windows-runtime provenance keeps the following false:

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

A Windows-runtime provenance PASS permits only a later, separate product-host integration review.
