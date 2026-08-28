# Hepta UI v4 Tranche 22 — Windows Product Host Implementation Candidate

## Status

```text
SOURCE_READY / REVIEW_ENVELOPE_REQUIRED / PRODUCT_UNWIRED / NO_AUTHORITY
```

This tranche implements the explicit product-host transaction that may be used
only after the Windows review envelope, operator acceptance, physical-device
validation, and rollback drill are all bound to one candidate. It does not add
a product lifecycle caller or enable a system-material feature in the product
Cargo manifest.

## Default-disabled boundary

The exact feature contract is:

```text
hepta_ui_windows_system_material_v4
```

The isolated harness declares it with `default = []`. Current product state is:

```text
productCargoFeatureDeclared=false
productModuleRegistered=false
productLifecycleWired=false
automaticBindingAllowed=false
```

The implementation source is not referenced by `shared/mod.rs` and is not
called by `hepta_material_app_lifecycle.rs`.

## Explicit activation prerequisites

Activation requires all of:

- a review receipt in `EligibleForImplementationReview`;
- a sealed 40-character candidate commit and tree;
- a 64-character review binding digest;
- an explicit implementation approval;
- explicit operator acceptance;
- physical-device validation;
- a completed rollback drill;
- the compile-time feature and runtime feature request;
- transparency allowed at activation time;
- high contrast disabled at activation time;
- distinct non-zero root and transient HWNDs;
- distinct root and transient WindowIds.

No constructor, `Drop` implementation, startup callback, or Script registration
performs automatic binding.

## Transactional material contract

The backend must return a verified binding receipt proving:

```text
root Mica exact
transient Acrylic exact
complete profile bound
system material bound
no authority
```

A backend error or invalid receipt immediately requests rollback and never
publishes `Bound` state.

Rollback must prove:

```text
root None exact
transient None exact
rollback verified
no authority
```

Suspend and shutdown require the same rollback when the host is bound.

## Runtime receipt boundary

Future runtime receipts may use:

```text
PASS_WINDOWS_PRODUCT_HOST_ACTIVATION
PASS_WINDOWS_PRODUCT_HOST_ROLLBACK
FAIL_WINDOWS_PRODUCT_HOST_ACTIVATION
```

Even a runtime activation receipt grants no network, mutation, model/provider,
effect, production, promotion, or release authority. Visual product binding is
not business-operation authority.

## Qualification

The dedicated read-only workflow runs:

- source gate and machine-contract parsing;
- default-disabled compilation and tests;
- explicit-feature compilation and tests;
- Ubuntu, Windows, and macOS isolated harness jobs.

It does not compile or run the full product, create a window, call DWM, bind the
product lifecycle, deploy, publish, promote, or release.

Current claims remain:

```text
sourceImplemented=true
hostedSource=false
defaultFeatureCompile=false
enabledFeatureCompile=false
enabledFeatureTests=false
reviewEnvelopePassed=false
productBound=false
systemMaterialBound=false
production=false
release=false
```

## Machine boundary summary

```text
default disabled
productCargoFeatureDeclared=false
productLifecycleWired=false
production=false
release=false
```
