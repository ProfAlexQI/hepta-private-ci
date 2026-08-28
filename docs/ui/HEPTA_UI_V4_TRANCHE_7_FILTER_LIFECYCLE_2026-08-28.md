# Hepta UI v4 — Tranche 7: bounded room filter and root lifecycle bridge

Date: 2026-08-28 (Asia/Singapore)  
Branch: `codex/ui-v4-filter-lifecycle-20260828`  
Stacked parent: `codex/ui-v4-native-host-consolidation-20260828`  
Parent commit: `603984a30ae5477c55a51300cce1aaea092a83ac`

Status: `SOURCE_IMPLEMENTED_RUNTIME_AND_DEVICE_VALIDATION_REQUIRED`

This tranche advances U2, U3, U4 and U8. It does not grant network,
mutation, effect, live-adapter, production, operator-acceptance, promotion, or
release authority.

## 1. Why this tranche exists

The previous candidate intentionally did not globally rebind
`RobrixTextInput` because the home filter was still embedded in fixed 35, 39,
and 45 logical-pixel containers. Raising only the child input would create a
source-level false positive while risking clipping and overlapping the rooms
workspace.

The material host also existed without an application event source. It could be
tested directly, but startup, foreground, pause, background, and shutdown did
not drive it from the real widget tree.

## 2. Bounded room-filter migration

The filter is now migrated as one unit:

- filter control: 48 logical pixels;
- clear affordance: 48 × 48 logical pixels;
- text: 15 logical pixels;
- search/clear icon: 18 logical pixels;
- desktop and mobile containing row: 56 logical pixels;
- stable content material: opaque/near-opaque with no backdrop blur;
- nested input background: transparent inside the stable outer surface;
- focus, disabled, and selection semantics inherited from the explicit v4
  Robrix migration template.

The post-registration Home override replaces only named prototypes:

- the desktop `home_page` inside the existing PageFlip;
- the mobile `RoomsSideBar` variant;
- the mobile StackNavigation compatibility prototype.

It does not fork the full Rust HomeScreen implementation. Existing room-list,
selection, PageFlip, settings, add-room, and StackNavigation state remains
owned by the canonical widgets.

## 3. Root lifecycle source integration

`hepta_material_app_lifecycle.rs` injects a zero-size, non-interactive node into
the canonical Window prototype before the App root is evaluated. Makepad
forwards global lifecycle events through the Window event tree, so the node
receives:

- Startup;
- Resume;
- Foreground;
- Pause;
- Background;
- Shutdown.

Active events read the bounded system preference snapshot and drive the
consolidated material host. Pause and Background suspend and unbind. Shutdown
unbinds and moves the host to its terminal state.

The bridge deliberately owns `HeptaUnboundSystemMaterialAdapter` and passes
`HeptaMaterialHostCapabilities::default()`. Therefore this source integration
cannot claim `SystemMaterialBound`; it may emit only:

- `SemanticIntentOnly`;
- `SolidFallback`;
- `Suspended`;
- `Shutdown`.

The source constants remain:

```text
system_adapter_available=false
window_handle_bound=false
production_authority=false
effect_authority=false
live_adapter_authority=false
operator_acceptance=false
promotion=false
release=false
```

A later tranche must replace the unbound adapter only after obtaining exact
Makepad host objects and providing runtime readback evidence.

## 4. Source tests

Focused tests cover:

- 48px filter and clear targets;
- 15px filter text;
- 56px desktop/mobile row contract;
- retained 56px mobile stack header and 48px back target;
- unbound lifecycle never claiming system material;
- suspend and shutdown transitions;
- terminal shutdown rejection;
- all lifecycle and authority constants remaining false.

## 5. Qualification boundary

| Claim | State |
|---|---|
| filter/control source migration | implemented |
| desktop/mobile named prototype override | implemented |
| root Window lifecycle event source | implemented |
| system preference read path | implemented |
| OS system material adapter attached to bridge | false |
| Windows HWND extraction | `REQUIRED_NOT_RUN` |
| macOS AppKit adapter | `REQUIRED_NOT_RUN` |
| iOS view/controller adapter | `REQUIRED_NOT_RUN` |
| Android Activity/JNI bridge | `REQUIRED_NOT_RUN` |
| Native compile and focused tests | `REQUIRED_NOT_RUN` |
| 320/360/390/412/600 captures | `REQUIRED_NOT_RUN` |
| keyboard, landscape, 200% type captures | `REQUIRED_NOT_RUN` |
| production/effect/live adapter | false |
| operator acceptance/promotion/release | false |

A source gate may emit only
`PASS_FILTER_LIFECYCLE_SOURCE_ONLY`. A workflow that has no assigned runner,
no steps, or no logs remains `BLOCKED_RUNNER_NOT_ASSIGNED` rather than PASS or
source failure.

## 6. Next implementation order

1. Compile and run the exact candidate on Linux, Windows, and macOS runners.
2. Capture Native filter hit rectangles and 320–600 width layouts.
3. Attach the main Makepad host window to the Windows material adapter without
   guessing or scraping an HWND.
4. Implement macOS, iOS, and Android host adapters behind the same lifecycle.
5. Re-probe accessibility/material preferences on platform notifications.
6. Bind device screenshots and performance evidence to one frozen commit/tree.
