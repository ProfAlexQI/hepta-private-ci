# Hepta UI v4 Tranche 23 — Windows Product Host Implementation Qualification Closure

## Status

```text
SOURCE_READY / EXECUTABLE_MATRIX_REQUIRED / REVIEW_ENVELOPE_REQUIRED /
PHYSICAL_DEVICE_REQUIRED / PRODUCT_WIRING_FORBIDDEN
```

This tranche is stacked on the exact Tranche 22 implementation-candidate head:

```text
base branch = codex/ui-v4-windows-product-host-implementation-candidate-20260828
base commit = aba38df64c1e104a2962157dc0f944a8f9045cf1
base tree   = 9b4b34f870dcb35ff78466896c2c7a4408ebadc2
```

The candidate branch for this corrected qualification contract is:

```text
codex/ui-v4-windows-product-host-qualification-closure-v2-20260828
```

It does not modify the product Cargo manifest, `shared/mod.rs`, the material
application lifecycle, or any Windows system-call implementation.

## Purpose

Tranche 22 added a default-disabled, explicit-only implementation candidate.
Source presence is not enough to approve product wiring. This tranche creates
one fail-closed qualification boundary over four independent evidence classes:

1. candidate source contract;
2. default-off and explicit-feature compile/test on three operating systems;
3. the exact runtime product-host review envelope;
4. a physical Windows device and rollback drill receipt.

A single green compile job, a review-only receipt, or an isolated device run
cannot independently authorize product wiring.

## Canonical states

### Review envelope required

```text
BLOCKED_WINDOWS_PRODUCT_HOST_REVIEW_ENVELOPE_REQUIRED
```

This state requires source plus all six compile/test lanes to pass, while no
qualified runtime review envelope is present. It is an expected governance
blocker, not a product failure and not an implementation PASS.

### Physical device qualification required

```text
BLOCKED_WINDOWS_PRODUCT_HOST_DEVICE_QUALIFICATION_REQUIRED
```

This state additionally requires:

```text
PASS_WINDOWS_PRODUCT_HOST_INTEGRATION_REVIEW_ENVELOPE
```

but no valid physical-device rollback receipt is present.

### Implementation qualification PASS

```text
PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION
```

This requires all source and compile evidence, the qualified runtime review
envelope, and a physical Windows receipt proving:

```text
explicit feature enabled
root Mica exact
transient Acrylic exact
explicit root/transient None rollback
high-contrast solid fallback
transparency-disabled solid fallback
suspend rollback
shutdown rollback
final state = Unbound
```

Even this PASS only permits a later product-wiring review.

### Fail closed

```text
FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION
```

Candidate identity drift, missing compile lanes, feature-mode drift, invalid
review evidence, device identity reuse, rollback failure, or authority escape
all produce this state. A FAIL receipt contains an explicit six-cell compile
matrix with `qualified=false` placeholders, so a missing lane cannot cause a
second Schema failure that hides the original error.

## Compile matrix

The workflow freezes six exact lanes:

| Platform | Mode | Feature |
|---|---|---|
| Ubuntu | default-off | disabled |
| Ubuntu | explicit-feature | enabled |
| Windows | default-off | disabled |
| Windows | explicit-feature | enabled |
| macOS | default-off | disabled |
| macOS | explicit-feature | enabled |

Every lane runs isolated-harness rustfmt, all-target check, and tests. Every
receipt binds the same candidate commit and tree.

The explicit feature name is:

```text
hepta_ui_windows_system_material_v4
```

It remains absent from the product Cargo feature list.

## Review evidence

A runtime review envelope must have status:

```text
PASS_WINDOWS_PRODUCT_HOST_INTEGRATION_REVIEW_ENVELOPE
```

and must remain review-only:

```text
implementationApproved=false
productHostMayBind=false
productBound=false
systemMaterialBound=false
authority=false
```

The qualification receipt records both the review file SHA-256 and its sealed
binding digest. A valid pre-runtime review blocker remains a structured BLOCKED
state and cannot be promoted.

## Device drill evidence

The device receipt Schema requires a physical runner carrying all labels:

```text
self-hosted
Windows
X64
hepta-ui-dwm
```

It binds the implementation-candidate commit/tree and the review binding digest.
The Producer, rather than JSON Schema cross-field equality, proves root and
transient WindowIds and HWNDs are distinct. The drill must finish in `Unbound`,
with every product and authority field still false.

Synthetic fixtures, Hosted Windows Server, a fake backend, or a device receipt
bound to another implementation candidate cannot satisfy this gate.

## Artifact handling

Optional review and device artifacts are supplied only through explicit GitHub
artifact IDs. The workflow verifies metadata, expiration, artifact identity,
and safe ZIP entries before extraction. It rejects absolute paths, drive paths,
UNC paths, and `..` traversal.

Same-run source or compile artifact loss does not suppress the final receipt.
The final job passes a deliberate missing-file path into the Producer, which
then emits a bounded fail-closed qualification receipt.

## Source gate

The source gate emits only:

```text
PASS_WINDOWS_PRODUCT_HOST_QUALIFICATION_SOURCE_ONLY
```

It checks:

- the four-state Producer and five self-tests;
- six compile lanes;
- FAIL-receipt partial evidence support;
- review binding and device rollback contracts;
- product Cargo feature remains undeclared;
- implementation module remains unregistered;
- product lifecycle remains unwired;
- all qualification and authority fields remain false;
- read-only workflow permissions;
- no deploy, publish, promotion, or release job.

Source PASS does not imply Hosted compilation or device qualification.

## Permanent claim boundary for this tranche

Every output, including final implementation qualification PASS, retains:

```text
productCargoFeatureDeclared=false
productModuleRegistered=false
productLifecycleWired=false
automaticBindingAllowed=false
implementationApproved=false
productHostMayBind=false
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

The next tranche may only propose product wiring after the exact implementation
candidate obtains `PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION`.
