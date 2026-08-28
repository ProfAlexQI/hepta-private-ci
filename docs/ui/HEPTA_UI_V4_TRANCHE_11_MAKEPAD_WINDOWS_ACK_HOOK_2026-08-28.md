# Hepta UI v4 Tranche 11 — Makepad Windows acknowledgement hook bridge

## Exact stack

- base PR: `#18`
- base branch: `codex/ui-v4-windows-dwm-ack-producer-20260828`
- base commit: `e6c46c6ed28ddab268c95d5c42b5056baac2d865`
- base tree: `66f6ae67fef6bf43229fe33113da9316888d71a5`
- candidate branch: `codex/ui-v4-makepad-windows-ack-hook-20260828`
- pinned Makepad revision: `c4335cee10b22aca768510c9d072b0ca1bba15c8`

## Scope

This tranche closes the Hepta-side state-machine gap between the existing
Windows DWM backdrop-only producer and a future exact backend callback. It does
not modify the external Makepad dependency or claim that the callback exists at
runtime.

## Implemented source

`hepta_windows_backend_ack_bridge.rs` adds:

- explicit non-zero HWND binding;
- exact Makepad WindowId index and generation binding;
- one pending request per exact window;
- monotonic request-sequence enforcement;
- exact processed-event visual equality;
- popup rejection;
- backend-set failure containment without readback;
- read-only DWM acknowledgement on a successful processed event;
- exact window-destruction invalidation;
- stale destroy protection across generation reuse;
- terminal shutdown;
- authority-free snapshots and focused tests.

## Dependency boundary

The pinned Makepad backend already owns both `WindowId` and HWND while handling
`CxOsOp::SetWindowVisuals`, but the operation has no correlation sequence and
no post-apply callback. Hepta cannot safely recover that relationship through
the public dependency surface.

The machine contract and patch specification freeze the required upstream API:

- a new correlated visual operation;
- an ordered processed callback carrying sequence, exact WindowId, HWND,
  visuals, DWM set result and popup state;
- an exact destroyed callback;
- no global HWND discovery.

## Evidence boundary

Source-only development checks may establish only:

```text
bridge_source=true
hook_contract_source=true
```

They may not establish:

```text
makepad_patch_applied
hook_bound
explicit_hwnd_from_framework
native_compile
windows_compile
runtime_acknowledgement
backdrop_readback
full_visual_readback
transient_system_material
complete_profile
system_material_binding
native_product_runtime
device_validation
production_authority
effect_authority
operator_acceptance
promotion
release
```

## Next executable step

A dedicated Makepad fork or accepted upstream patch must implement the exact
contract at the pinned source anchors. Only after that dependency is pinned in
`apps/hepta-native/Cargo.toml` may the correlated hook be enabled and exercised
on Windows 11. The first runtime receipt must bind one exact candidate commit,
tree, Makepad revision, request sequence, WindowId generation, HWND, requested
visuals and DWM backdrop readback.
