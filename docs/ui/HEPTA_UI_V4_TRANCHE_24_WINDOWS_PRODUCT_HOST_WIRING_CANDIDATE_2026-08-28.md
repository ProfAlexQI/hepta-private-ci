# Hepta UI v4 Tranche 24 — Governed Windows Product-host Wiring Candidate

## Purpose

This tranche closes the remaining repository-source seam between the isolated Windows product-host transaction candidate and the Native product composition. It does not apply the product patch on the branch. The patch may be applied only after the exact implementation qualification receipt passes.

## Source graph

```text
runtime review envelope
+ implementation qualification
+ operator acceptance digest
+ physical-device qualification digest
+ exact reviewed WindowIds/HWNDs and request sequence 1/2/3
        ↓
immutable evidence seal
        ↓ explicit caller only
transactional product host
        ↓
DWM root Mica + transient Acrylic exact readback
        ↓
high-contrast/transparency/suspend/shutdown rollback to root/transient None
```

The implementation reuses the existing audited Windows DWM set/readback interfaces and the existing transactional host. It does not create a second material authority kernel.

## Product patch

The governed patch declares `hepta_ui_windows_system_material_v4` in the product Cargo manifest but leaves it out of `default`. It registers the review, implementation, wiring, and zero-size lifecycle node only under that feature. The node never activates on Startup, Resume, Draw, or Focus. Activation requires an explicit sealed call.

Current branch state remains:

```text
productPatchApplied=false
productCargoFeatureDeclared=false
productModuleRegistered=false
productLifecycleWired=false
automaticBindingAllowed=false
productCallerRegistered=false
runtimeValidated=false
productBound=false
systemMaterialBound=false
```

## Evidence hardening

The evidence seal binds:

- implementation build commit/tree;
- review-envelope candidate and fixed Makepad revision;
- review binding digest and source run;
- implementation approval candidate;
- physical-device candidate and review digest;
- operator acceptance digest;
- physical-device qualification digest;
- exact root/transient identities;
- request sequence 1/2/3;
- Destroyed acknowledgement;
- high-contrast and transparency-disabled fallbacks;
- suspend and shutdown rollbacks;
- final device state `Unbound`.

A caller cannot replace the reviewed window identity during activation.

## Qualification

Source validation, isolated default-off/feature builds, and temporary product-patch builds are separate. Applying the product patch requires `PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION` for the exact implementation candidate.

## External blockers

Repository source cannot manufacture physical Windows/macOS/iOS/Android compositor evidence, GitHub runner assignment, cross-browser screenshots, or human operator acceptance. These are recorded in `HEPTA_UI_REMAINING_EXTERNAL_BLOCKERS_V1.json` rather than being misreported as code failures or PASS.

## Permanent boundary

```text
productCallerRegistered=false
runtimeValidated=false
productBound=false
systemMaterialBound=false
network=false
mutation=false
effect=false
liveAdapter=false
production=false
operatorAcceptance=false
promotion=false
release=false
```
