# Hepta UI v4 — pinned Makepad Windows acknowledgement hook contract

## Identity

- Makepad repository: `kevinaboos/makepad`
- pinned revision: `c4335cee10b22aca768510c9d072b0ca1bba15c8`
- Hepta dependency: `makepad-widgets` at the exact revision above
- hook patch applied to the dependency: **false**
- runtime hook bound: **false**

This document freezes the smallest upstream change required to connect the
Hepta-side bridge without scanning global HWND lists or guessing a native
window. It is an implementation contract, not evidence that the patch has been
applied or compiled.

## Verified upstream insertion points

| File | Pinned blob | Required change |
|---|---|---|
| `platform/src/cx_api.rs` | `87d871cd6414d67ed3da14f3a1a48f98940d0782` | add a correlated visual operation carrying `request_sequence` |
| `platform/src/window.rs` | `8e6e9a9d41c34d654de6721f4956558a3eb649fe` | expose a correlated setter without changing the existing setter |
| `platform/src/os/windows/win32_app.rs` | `42c98828d0f7d5f11fd45083e9102065465c560b` | store an optional ordered hook callback and typed events |
| `platform/src/os/windows/win32_window.rs` | `67899eceaf83e46dc8690f0e8be98ad4941abb3a` | return the DWM system-backdrop set result from `apply_window_visuals` |
| `platform/src/os/windows/windows.rs` | `58df661d9d84660fcca2ac960f5bcf5b36c52f3a` | emit processed and destroyed events at the backend ownership boundary |

The pinned Windows backend already owns the exact pair needed by the hook:
`Win32Window.window_id` and `Win32Window.hwnd`. The current public application
surface does not expose a safe `WindowId -> HWND` lookup, and `Win32App` only
retains a global HWND list. Hepta therefore must not reconstruct the mapping by
scanning `all_windows`, calling `FindWindow`, or selecting a foreground window.

## Required upstream API shape

### 1. Correlated operation

Add a new operation rather than changing the existing public variant:

```rust
CxOsOp::SetWindowVisualsCorrelated {
    window_id: WindowId,
    visuals: WindowVisuals,
    request_sequence: u64,
}
```

The existing `CxOsOp::SetWindowVisuals(WindowId, WindowVisuals)` remains
available for callers that do not request acknowledgement evidence.

### 2. Correlated setter

Add a `WindowHandle::set_window_visuals_correlated(...)` path that:

1. normalizes `WindowVisuals`;
2. updates the same `CxWindow` state as the existing setter;
3. queues the correlated operation only when the state changes and the window
   is already created;
4. never manufactures an operation for a deduplicated/no-op request.

### 3. Ordered Windows hook events

Add an optional callback to the Windows platform owner with these event shapes:

```rust
Win32WindowVisualsHookEvent::Processed {
    request_sequence: u64,
    window_id: WindowId,
    hwnd: isize,
    visuals: WindowVisuals,
    backend_apply_succeeded: bool,
    is_popup: bool,
}

Win32WindowVisualsHookEvent::Destroyed {
    window_id: WindowId,
    hwnd: isize,
}
```

The callback must run on the Windows UI thread and in the same ordering domain
as `Cx::handle_platform_ops`.

### 4. Processing point

For `SetWindowVisualsCorrelated`, the backend must:

1. resolve the exact `D3d11Window` by the full `WindowId`;
2. call `Win32Window::apply_window_visuals(visuals)`;
3. capture only the DWM system-backdrop set result;
4. emit `Processed` immediately after the call with the same sequence,
   `WindowId`, HWND, normalized visuals, result, and popup state.

The result means only that the `DWMWA_SYSTEMBACKDROP_TYPE` set call succeeded.
It is not full `WindowVisuals` readback and does not prove transparency,
intensity, transient Acrylic, or a complete platform profile.

### 5. Destruction invalidation

Emit `Destroyed` before removing the exact `D3d11Window` from backend storage.
The event must preserve the full `WindowId` generation. A delayed destroy event
for an older generation must not invalidate a newer replacement window.

## Hepta consumption boundary

`hepta_windows_backend_ack_bridge.rs` consumes the event contract through these
steps:

1. bind an explicit exact window identity;
2. register one monotonic queued request;
3. require sequence, index, generation, HWND and normalized visual equality;
4. reject popup events;
5. skip DWM readback when the backend set failed;
6. on success, invoke the existing read-only DWM producer;
7. verify backdrop-only acknowledgement;
8. consume the pending request exactly once;
9. invalidate state on an exact destroyed event;
10. keep all product and authority claims false.

## Forbidden shortcuts

The integration must not use:

- `FindWindow`, `EnumWindows` or foreground-window lookup;
- iteration over `Win32App::all_windows` to discover a matching HWND;
- `GWLP_USERDATA` scanning from application code;
- window index without generation;
- backdrop equality without request-sequence correlation;
- a root HWND as a transient Acrylic host;
- DWM backdrop readback as proof of complete `WindowVisuals`;
- source presence as runtime, device, operator or release evidence.

## Current status

```text
hepta_bridge_source=true
upstream_hook_contract=true
makepad_patch_applied=false
hook_bound=false
native_compile=false
windows_compile=false
runtime_acknowledgement=false
backdrop_readback=false
transient_system_material=false
complete_profile=false
system_material_binding=false
production_authority=false
effect_authority=false
operator_acceptance=false
promotion=false
release=false
```
