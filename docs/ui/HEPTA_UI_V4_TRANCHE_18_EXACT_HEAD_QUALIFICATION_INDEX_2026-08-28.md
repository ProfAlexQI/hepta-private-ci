# Hepta UI v4 Tranche 18 — Exact-head qualification index

Date: 2026-08-28

## Purpose

This tranche adds one canonical qualification index for the exact-head Windows
material evidence chain. It prevents an isolated green source, materialization,
compile, or runtime job from being interpreted as a complete UI qualification.

The index consumes one completed run of:

`hepta-ui-v4-windows-material-profile-exact`

and binds all evidence to one Hepta commit, one Hepta tree, and the pinned
Makepad revision:

`c4335cee10b22aca768510c9d072b0ca1bba15c8`

## Deterministic producer layout

The executable entry point loads four ordered, repository-tracked Ruby source
parts. The source gate concatenates the same main/part set before checking all
status, job, identity, runtime-chain, and authority invariants. This split is
only a transport/layout choice; the self-test executes the concatenated source
and the receipt contract is unchanged.

## Accepted states

### `PASS_EXACT_HEAD_PRE_RUNTIME_QUALIFICATION`

Requires all of the following from one source run:

- four inherited source-gate PASS receipts;
- exact candidate commit and tree files;
- one executable source job with a nonzero runner and nonempty successful steps;
- one executable candidate-bound materialization job;
- a PASS materialization receipt bound to the same commit/tree;
- successful Ubuntu, Windows, and macOS compile/test jobs;
- candidate-bound materialization receipts from every compile artifact;
- no successful governed Windows runtime job or runtime receipt.

This state proves pre-runtime source, patch, compile, and focused-test closure.
It does not make the candidate eligible for product material integration.

### `PASS_EXACT_HEAD_WINDOWS_RUNTIME_QUALIFICATION`

Requires the complete pre-runtime state plus a successful governed Windows job
and exact runtime aggregate receipt proving:

- root Mica backdrop readback;
- a distinct popup HWND;
- popup Acrylic backdrop readback;
- explicit `WindowBackdrop::None` rollback;
- exact popup Destroyed acknowledgement;
- candidate commit/tree and Makepad revision identity;
- no authority escape.

This state makes the isolated evidence set eligible for a separate
product-integration review only. It does not bind the product host.

### `FAIL_EXACT_HEAD_QUALIFICATION_INDEX`

The index fails closed for missing, duplicate, ambiguous, stale, skipped, or
identity-drifting evidence. Examples include:

- `runner_id=0` or `steps=[]` on a required job;
- a missing compile platform;
- duplicate required job names;
- materialization or runtime receipt from another commit/tree;
- a runtime receipt without a successful governed runtime job;
- a successful runtime job without its aggregate receipt;
- product/system-material/authority fields becoming true.

## Artifact contract

The producer expects artifact directories named by the exact candidate:

- `hepta-ui-v4-exact-source-<commit>`;
- `hepta-ui-v4-exact-materialization-<commit>`;
- `hepta-ui-v4-exact-compile-ubuntu-latest-<commit>`;
- `hepta-ui-v4-exact-compile-windows-latest-<commit>`;
- `hepta-ui-v4-exact-compile-macos-latest-<commit>`;
- optional `hepta-ui-v4-exact-windows-profile-<commit>`.

A runtime artifact is rejected when the governed runtime job did not complete
successfully.

## Workflow behavior

`.github/workflows/hepta-ui-v4-exact-head-qualification-index.yml` provides:

- source-only validation on push and pull request;
- automatic replay after the canonical exact workflow completes once the
  workflow is present on a canonical branch;
- manual replay for a supplied source run ID;
- an optional strict mode that requires Windows runtime qualification;
- bounded FAIL receipts and retained index evidence.

Repository permissions are read-only (`contents: read`, `actions: read`).
There is no deploy, publish, promotion, or release job.

## Authority boundary

For every index state, including Windows runtime PASS:

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

The next tranche may begin product-host integration only after a
`PASS_EXACT_HEAD_WINDOWS_RUNTIME_QUALIFICATION` receipt has been independently
reviewed and bound to the same frozen commit/tree and Makepad revision.
