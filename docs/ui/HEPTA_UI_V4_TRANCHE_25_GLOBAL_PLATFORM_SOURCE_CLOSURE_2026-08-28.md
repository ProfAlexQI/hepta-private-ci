# Hepta UI v4 Tranche 25 — Global Platform Source Closure

This tranche closes every source-controllable UI gap recorded by the Tranche 24 blocker ledger while preserving all runtime, product, device, effect, and release boundaries.

## Closed source gaps

- macOS AppKit chrome/transient adapter contract, scoped observation, accessibility bridge, and transactional rollback;
- iOS UIKit navigation/sheet adapter contract, Dynamic Type, safe-area, keyboard, accessibility, and rollback boundaries;
- Android Material 3 Activity/JNI contract, verified dynamic color, tonal chrome/sheet, animator scale, IME/navigation inset, and solid fallback;
- operator-governed explicit activation caller with exact candidate binding, evidence and acceptance digests, bounded lifetime, and nonce replay prevention.

The modules are compiled by an isolated `#![forbid(unsafe_code)]` harness. They are not registered in `shared/mod.rs`, not referenced by the product material lifecycle, and not enabled by a product Cargo feature.

## Runtime boundary

`globalSourceControllableGapsClosed=true` means only that no known repository source implementation remains missing. It does not establish AppKit, UIKit, Android, Windows, browser, or physical-device evidence.

The only remaining gaps are external evidence:

- executable GitHub runner capacity;
- physical Windows DWM qualification;
- candidate-bound human operator acceptance;
- macOS, iOS, and Android physical-device evidence;
- Chrome, Edge, Safari, and Firefox screenshot/accessibility acceptance.

## Permanent authority boundary

```text
productWired=false
systemMaterialBound=false
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
