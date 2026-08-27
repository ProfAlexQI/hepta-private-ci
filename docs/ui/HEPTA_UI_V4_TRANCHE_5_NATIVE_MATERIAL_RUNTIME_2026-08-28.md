# Hepta UI v4 — Tranche 5 Native material runtime

Date: 2026-08-28 (Asia/Singapore)  
Branch: `codex/ui-v4-native-material-runtime-20260828`  
Stacked base: `0b2a8262455cb85aa8403f61e602e1ead5ad6c74`  
UI implementation base: `codex/ui-light-glass-v4-20260827`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

Status: `SOURCE_IMPLEMENTED_COMPILE_RUNTIME_AND_DEVICE_EVIDENCE_REQUIRED`

This tranche advances U3, U4 and U8. It adds no network, mutation, effect,
live-adapter, production, operator-acceptance, promotion or release authority.

## Delivered

### Native style graph closure

`hepta_v4_controls.rs` and `hepta_v4_layout.rs` already contained the 48 logical-
pixel control primitives and the 56 logical-pixel mobile header contract, but
were not declared or loaded by `shared/mod.rs`. They are now registered after
`hepta_v4`, so their definitions can participate in the real Native style graph.

### Capability-aware semantic resolver

The platform material resolver now accepts explicit capabilities. Android tonal
chrome and sheet intents are selected only when dynamic color has been verified
as available. Unknown Android capability, disabled transparency, high contrast,
Linux and unknown platforms resolve to a fully solid profile.

Stable content remains `Solid`, stable-content backdrop layers remain zero and
the visible backdrop budget remains at most two.

### System preference probes

Added `hepta_system_preferences.rs` with a fail-closed shared contract:

- unavailable probes disable transparency and enable reduced motion;
- host-provided values are represented explicitly rather than inferred;
- Windows reads high-contrast and client-animation preferences through
  `SystemParametersInfoW` and reads `EnableTransparency` through `RegGetValueW`;
- iOS reads Reduce Transparency, Reduce Motion and Darker System Colors through
  UIKit accessibility functions;
- macOS, Android, Web, Linux and unknown Native targets remain fail-closed until
  their required host objects or browser media-query bridge are supplied.

No preference probe owns a network or effect path.

### Windows DWM adapter

Added `hepta_windows_material_adapter.rs`:

- explicit non-zero chrome and transient HWND values are required;
- persistent chrome receives `DWMSBT_MAINWINDOW` Mica;
- the separate transient host receives `DWMSBT_TRANSIENTWINDOW` Acrylic;
- partial failure rolls the already-bound chrome window back to no backdrop;
- unbind clears transient first and chrome second;
- the returned receipt must report a real system binding and keeps all authority
  fields false.

The adapter does not discover Makepad window handles and therefore cannot claim
runtime binding until the host integration supplies exact HWND values.

### Runtime receipt hardening

The runtime resolver now distinguishes `Unbound` and `Bound` explicitly. A
material receipt that reports `system_material_bound=false`, a mismatched
platform/profile, or any production/effect authority is rejected and unbound.

The current-platform resolver consumes the new system-preference snapshot, but
no platform is marked runtime-verified by source presence alone.

### Qualification chain

Added:

- `scripts/hepta-ui-v4-native-material-runtime-gate`;
- `.github/workflows/hepta-ui-v4-native-material-runtime.yml`;
- an updated platform material adapter matrix.

The gate checks module registration, 48/56 logical-pixel floors, Android dynamic
color fallback, system preference probes, DWM source calls, rollback behavior,
receipt boundaries and authority values. Its only successful source status is:

`PASS_NATIVE_MATERIAL_RUNTIME_SOURCE_ONLY`

The workflow contains portable Linux compile/tests and a Windows compile/test
lane. A job that receives no runner or executes no steps is infrastructure
blockage, not a source PASS or failure.

## Current qualification state

| Lane | State |
|---|---|
| Native controls/layout registration | source implemented |
| Stable-content and layer-budget resolver | source implemented |
| Android dynamic-color fallback | source implemented |
| Windows preference probe | source implemented |
| iOS accessibility preference probe | source implemented |
| Windows DWM Mica/Acrylic adapter | source implemented; host HWND binding required |
| macOS material adapter | required |
| Android Activity/dynamic-color host adapter | required |
| iOS view/controller material adapter | required |
| Native portable compile | `REQUIRED_NOT_RUN` |
| Windows compile and adapter tests | `REQUIRED_NOT_RUN` |
| Runtime window binding | `REQUIRED_NOT_RUN` |
| Desktop/mobile device evidence | `REQUIRED_NOT_RUN` |
| Production/effect/promotion/release | false |

## Next executable tranche

1. Run the source, portable Native and Windows DWM jobs against one frozen
   candidate and correct every compile or Makepad DSL failure without weakening
   the 48/56 logical-pixel and solid-fallback contracts.
2. Identify the exact Makepad Windows host-window API at the pinned Makepad
   revision and bind its HWND values into `HeptaWindowsMaterialAdapter`.
3. Add macOS `NSVisualEffectView` or equivalent system-material integration with
   Reduce Transparency and Increase Contrast observation.
4. Add iOS navigation/sheet host integration while preserving UIKit
   accessibility fallbacks.
5. Add an Android Activity/JNI preference bridge for API level, animation scale
   and dynamic-color availability; use tonal surfaces only after verification.
6. Capture Windows 125% and High Contrast, macOS Reduce Transparency, Android
   360/412 and iOS Dynamic Type evidence with commit/tree and screenshot digests.
7. Update individual platform-matrix rows only from executed receipts; keep all
   authority and promotion values false until independent acceptance exists.
