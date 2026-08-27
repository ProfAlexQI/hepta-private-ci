# Hepta UI v4 — Native registration and material-host tranche

Date: 2026-08-28 (Asia/Singapore)  
Branch: `codex/ui-v4-native-host-adapter-20260828`  
Stacked base: `codex/ui-v4-canonical-bundle-guard-20260828`  
Base commit: `0b2a8262455cb85aa8403f61e602e1ead5ad6c74`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

Status: `SOURCE_IMPLEMENTED_NATIVE_COMPILE_OS_ADAPTER_AND_DEVICE_EVIDENCE_REQUIRED`

This tranche advances U3 and U4 without granting production caller, mutation,
effect, live-adapter, operator-acceptance, promotion, or release authority.

## 1. Native widget registration repaired

The repository already contained `hepta_v4_controls.rs` and
`hepta_v4_layout.rs`, but the shared script graph did not register either
module. Their 48px controls and 56px mobile-layout primitives therefore were
not guaranteed to exist when downstream widgets were constructed.

`shared/mod.rs` now declares both modules and loads them after:

1. the legacy shared styles;
2. the semantic `hepta_v4` override;

and before the widgets that consume compatibility names.

## 2. Legacy input compatibility migration

`RobrixTextInput` is rebound after the legacy style graph to inherit
`HeptaV4TextInput`. Existing login, room, settings, and composer code can keep
its stable compatibility name while receiving:

- a 48 logical-pixel minimum height;
- 15px text;
- stable solid content material;
- v4 focus, border, disabled, cursor, and selection semantics;
- the existing bounded scroll-bar presentation.

This is a source migration. Actual hit rectangles and text layout still require
Native compilation and window/device measurement.

## 3. Mobile stack header migration

A post-HomeScreen override upgrades the compatibility stack prototype from its
legacy 45px contract to:

- 56px mobile header;
- 48px minimum back target;
- 20px icon inside the larger hit area;
- body offset derived from the same header token.

The override is registered immediately after `home_screen::script_mod`, so the
upstream screen remains rebaseable and the v4 migration is isolated.

## 4. Platform material host lifecycle

`hepta_platform_material_host.rs` adds a transaction coordinator around the
existing `HeptaSystemMaterialAdapter` seam. It implements:

- semantic-intent state while a real system adapter is unavailable;
- solid fallback for high contrast or disabled transparency;
- solid fallback for unsupported platforms;
- Android dynamic-color availability gating;
- exact receipt validation for platform, chrome, transient, bounded layers,
  stable solid content, and false authority flags;
- rollback-to-solid and `unbind()` on adapter error or rejected receipt;
- explicit suspend and shutdown cleanup;
- monotonically increasing generation snapshots;
- immutable false production/effect/live-adapter/operator/promotion/release
  fields in every host snapshot.

The host does not invoke Windows, AppKit, UIKit, or Android platform APIs. Those
remain behind adapters and remain `REQUIRED_NOT_RUN`.

## 5. Governance and CI

Updated machine records:

- `HEPTA_NATIVE_CONTROL_MIGRATION_V1.json`;
- `HEPTA_PLATFORM_MATERIAL_ADAPTER_MATRIX_V1.json`.

Added:

- `scripts/hepta-ui-v4-native-host-gate`;
- `.github/workflows/hepta-ui-v4-native-host.yml`.

The source gate can only emit:

`PASS_NATIVE_HOST_SOURCE_CONTRACT_ONLY`

It leaves Native compile, Native runtime, operating-system adapter, device,
production, effect, live-adapter, operator, promotion, and release validation
false.

The compile job separately checks Makepad/Rust registration and runs focused
control, layout, home override, and transactional host tests. A runner must
execute those steps before any compile claim changes.

## 6. Remaining next actions

1. Execute the source and Native compile jobs on an assigned runner.
2. Resolve any Makepad prototype-rebind or DSL compatibility failure without
   reducing 48/56px requirements.
3. Implement Windows Mica/Acrylic calls behind the adapter trait and verify
   transparency-off/high-contrast fallback.
4. Implement macOS system chrome/popover material calls and inactive-window
   fallback.
5. Implement iOS navigation/sheet material calls with Reduce Transparency and
   Dynamic Type coverage.
6. Implement Android dynamic tonal chrome/sheet mapping with system-bar and
   large-font coverage.
7. Capture login, rooms, thread, settings, modal, keyboard, safe-area, and 200%
   text evidence bound to one frozen commit/tree.
8. Update matrix rows individually; do not promote source presence to runtime or
   device PASS.
