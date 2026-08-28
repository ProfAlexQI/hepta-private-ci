# Hepta UI v4 Tranche 21 — Windows Product Host Integration Review Envelope

## Purpose

This tranche introduces a review-only admission boundary between qualified Windows material evidence and any future product-host implementation. It does **not** bind `HeptaPlatformMaterialHost`, attach a Windows adapter to the product lifecycle, enable a feature flag, or grant effect, production, operator-acceptance, promotion, or release authority.

The boundary exists because a passing isolated Mica/Acrylic fixture and a passing execution-provenance audit are necessary but not sufficient to mutate the product host. A separately reviewable envelope must bind all input evidence to one exact candidate and preserve the remaining rollout requirements.

## Exact source baseline

- base branch: `codex/ui-v4-execution-provenance-audit-20260828`
- base commit: `cfdc9742cd8a59c55fdb959e1760b4e1f6fa5048`
- base tree: `a6ebc3bda2ff955c14910782c50532be80afc625`
- intended branch: `codex/ui-v4-product-host-integration-review-20260828`

## Admission inputs

A passing review envelope requires all three receipts:

1. `PASS_EXACT_HEAD_WINDOWS_RUNTIME_PROVENANCE`;
2. `PASS_EXACT_HEAD_WINDOWS_RUNTIME_QUALIFICATION`;
3. `PASS_WINDOWS_MATERIAL_PROFILE_AGGREGATE`.

The producer requires exact equality for the candidate commit and tree across all receipts. It also requires the fixed Makepad revision:

`c4335cee10b22aca768510c9d072b0ca1bba15c8`

The runtime chain is fixed to:

```text
1 = persistent root Mica
2 = dedicated popup Acrylic
3 = explicit WindowBackdrop::None rollback
then exact Destroyed acknowledgement
```

Root and transient WindowIds must differ, root and transient HWNDs must be nonzero and distinct, and the destroyed identity must equal the transient identity.

## Canonical states

### Review envelope PASS

`PASS_WINDOWS_PRODUCT_HOST_INTEGRATION_REVIEW_ENVELOPE`

This means only that implementation review may begin. It does not authorize code to bind the product host.

### Expected pre-runtime block

`BLOCKED_WINDOWS_RUNTIME_PROVENANCE_REQUIRED`

This is emitted when the supplied provenance is a valid pre-runtime PASS but no governed Windows runtime provenance exists yet. It is not treated as product eligibility.

### Fail-closed

`FAIL_WINDOWS_PRODUCT_HOST_INTEGRATION_REVIEW_ENVELOPE`

Candidate drift, receipt mismatch, Makepad drift, reused window identities, sequence drift, missing rollback/destroy evidence, binding escape, or authority escape all produce this state.

## Evidence sealing

The PASS envelope records SHA-256 digests for:

- the execution-provenance receipt;
- the exact-head qualification index;
- the runtime aggregate receipt.

It then computes a deterministic binding digest over the reviewer identity, candidate identity, source run, fixed Makepad revision, input digests, and root/transient runtime identities.

## Dormant Rust admission controller

`hepta_windows_product_host_integration_review.rs` provides a pure Rust admission controller and tests. The module is intentionally not registered in the product script graph and is not referenced by `hepta_material_app_lifecycle.rs`.

Even its accepted receipt fixes these to false:

```text
implementationApproved
productHostMayBind
productBound
transientSystemMaterialBound
completeProfileBound
systemMaterialBound
nativeProductRuntime
deviceValidated
```

The generated plan requires:

- a disabled-by-default feature flag;
- separate operator acceptance;
- physical-device validation;
- a rollback drill;
- solid fallback when transparency is disabled;
- high-contrast fallback;
- suspend/shutdown unbind behavior.

## Workflow

The read-only workflow supports source validation on pull requests and replay after a completed execution-provenance audit. A pre-runtime provenance receipt produces a bounded BLOCKED envelope. A Windows-runtime provenance receipt additionally downloads the exact qualification-index and runtime-aggregate artifacts from the audited source run.

The workflow never modifies product code or repository state and has no deploy, publish, promotion, or release job.

## Promotion boundary

A PASS review envelope allows only a future implementation-review tranche. That future tranche must still:

1. add a disabled product feature flag;
2. bind a concrete Windows adapter transactionally;
3. re-check current transparency and high-contrast preferences;
4. unbind on pause, background, shutdown, handle invalidation, or receipt mismatch;
5. execute device and rollback qualification;
6. obtain explicit operator acceptance;
7. leave production/effect/release authority false until a separate promotion receipt exists.

Machine authority fields remain `operatorAcceptance=false`, `promotion=false`, and `release=false` throughout this tranche.

The machine plan stage is exactly `IMPLEMENTATION_REVIEW_ONLY`.
