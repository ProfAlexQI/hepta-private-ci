# Hepta UI v4 — Tranche 8: framework window visuals and Native component diagnostics

Date: 2026-08-28 (Asia/Singapore)

Branch: `codex/ui-v4-window-visuals-diagnostics-20260828`

Stacked parent:

- PR #9: `feat(ui): migrate room filter and wire root material lifecycle`
- parent commit: `d42202c61ef063113146087af846396a772458bd`
- parent tree: `75bdc5fc3f975ca6b5ef93ddcdc3dae846d4b9df`

Status: `SOURCE_IMPLEMENTED_COMPILE_RUNTIME_AND_READBACK_REQUIRED`

This tranche advances U3, U4, and U8. It does not grant network, mutation,
effect, live-adapter, production, operator-acceptance, promotion, or release
authority.

## 1. Public Makepad root-window material path

The pinned Makepad revision exposes a governed public window API:

- `WindowId` identifies the exact framework window;
- `WindowVisuals` describes transparent/backdrop/intensity intent;
- `WindowBackdrop` includes `Mica` and `Vibrancy`;
- `CxOsOp::SetWindowVisuals` carries the request to the platform backend.

`hepta_makepad_window_material.rs` uses only these public framework objects. It
does not read or search for HWND, NSWindow, UIView, Activity, JNI, or browser
host objects.

The controller obtains the exact root `WindowId` from its rendered Makepad area
or a matching focus event, rejects popup windows, updates Makepad's stored
window visuals, and queues at most the framework operation required by the
changed state.

Current requests are deliberately limited to persistent root-window chrome:

| Platform | Transparent preference | Requested root backdrop |
|---|---|---|
| Windows | allowed, non-high-contrast | `Mica` |
| macOS | host preference must allow it | `Vibrancy` |
| iOS | any | solid / unsupported by this tranche |
| Android | any | solid / unsupported by this tranche |
| Linux/unknown | any | solid |

Focus changes use a lower inactive intensity. Pause, Background, disabled
transparency, high contrast, and Shutdown restore `WindowBackdrop::None`.

## 2. Partial request is not a complete material receipt

A framework request proves only that the candidate asked Makepad to apply a
persistent root-window visual for an exact framework window. It does not prove:

- that the OS compositor accepted it;
- that the requested material was read back;
- that a transient Acrylic/popover/sheet host exists;
- that stable content became transparent;
- that the complete semantic platform profile is bound.

The source therefore keeps all of these false:

```text
transient_system_material_bound=false
complete_profile_bound=false
system_material_bound=false
runtime_readback=false
production_authority=false
effect_authority=false
live_adapter_authority=false
operator_acceptance=false
promotion=false
release=false
```

The existing full-profile material host remains attached to
`HeptaUnboundSystemMaterialAdapter`. Its verified-receipt rules and Windows DWM
adapter source are retained for a later governed transient host and runtime
readback tranche.

## 3. Root lifecycle integration

The zero-size lifecycle node now drives both layers:

1. the unbound full-profile material host; and
2. the partial Makepad root-window visuals controller.

Lifecycle behavior:

| Event | Full-profile host | Framework root-window controller |
|---|---|---|
| Startup | re-read preferences, remain unbound | waits for an exact window area |
| first completed draw | unchanged | binds exact non-popup `WindowId` and requests visuals |
| WindowGotFocus | unchanged | active intensity |
| WindowLostFocus | unchanged | inactive intensity |
| Resume / Foreground | re-read preferences | recompute request |
| Pause / Background | suspend and unbind | restore solid |
| Shutdown | terminal shutdown | restore solid and become terminal |

The controller does not infer a window from index zero. It accepts only a valid
Makepad `WindowId` associated with its own rendered area or matching focus
event.

## 4. Native room-filter runtime metrics producer

`apps/hepta-native/src/bin/hepta-ui-v4-filter-probe.rs` is an isolated component
producer. It starts no Matrix runtime, does not activate the Hepta bridge, and
has no network or mutation path.

It renders the real `RoomFilterInputBar` in a 390 × 180 Native window, enters a
sample value, reveals the clear affordance, then measures actual Makepad areas
for:

- the complete filter surface;
- the text-input area;
- the clear-button area;
- the window viewport and effective DPI.

Required checks:

```text
filter height >= 48
input height >= 48
clear width >= 48
clear height >= 48
filter contained by viewport
clear affordance visible
```

When `HEPTA_NATIVE_CAPTURE_FRAME_PATH` is supplied, the producer requests one
rendered-frame capture, records byte length and SHA-256, and cross-binds it with
the component metrics receipt. The receipt schema is:

`docs/ui/schemas/hepta.ui.v4.native-component-metrics.v1.schema.json`

A component receipt may prove only the isolated component metrics and optional
rendered frame. It explicitly keeps product runtime, system material binding,
device matrix, and every authority field false.

## 5. Qualification and CI

The source gate may emit only:

`PASS_WINDOW_VISUALS_DIAGNOSTICS_SOURCE_ONLY`

The compile matrix schedules Ubuntu, Windows, and macOS formatting, library
compile, probe-binary compile, and focused tests.

The component runtime producer is manual and isolated. It must emit a receipt
bound to the exact candidate commit/tree and a non-empty screenshot digest
before component runtime evidence can be accepted.

Source presence does not establish:

- Native compile success;
- Native product runtime success;
- Windows/macOS compositor acceptance;
- platform readback;
- iOS/Android device behavior;
- power or frame-time targets;
- production readiness.

## 6. Next tranche

1. obtain executable hosted compile and component-probe receipts for one frozen candidate;
2. add backend acknowledgement/readback for `SetWindowVisuals` rather than treating a queued operation as success;
3. introduce a governed transient host and prove Windows Acrylic separately;
4. implement macOS system preference probing and AppKit readback;
5. extend component probes to 320/360/412/600, landscape, keyboard, and 200% type;
6. implement iOS controller and Android Activity bridges;
7. bind screenshots, accessibility, frame time, and device identity to the same candidate tree.
