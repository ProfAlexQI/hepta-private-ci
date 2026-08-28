# Hepta UI v4 Tranche 12 — vendored Makepad Windows hook

## Scope

This tranche turns the Tranche 11 hook design into an executable, reviewable
vendored patch path for the exact Makepad revision
`c4335cee10b22aca768510c9d072b0ca1bba15c8`.

The default `hepta-native` manifest and lockfile remain unchanged. The patched
Makepad checkout is materialized only by an explicit build script and injected
through an ephemeral Cargo patch configuration. This prevents an unreviewed
source switch from entering normal product builds.

## Patch contents

The patch makes five bounded changes to Makepad:

1. adds Windows-only `SetWindowVisualsCorrelated` without deleting the existing
   uncorrelated operation;
2. adds a bounded Windows UI-thread hook queue carrying exact request sequence,
   complete `WindowId`, explicit HWND, normalized visuals, DWM set success, and
   popup state;
3. changes `Win32Window::apply_window_visuals` to return only the DWM
   system-backdrop set result;
4. emits a processed hook immediately after correlated backend application;
5. emits an exact destroyed hook before backend window removal.

No global HWND scan, window-title lookup, foreground-window lookup, or
application-level reconstruction is introduced.

## Isolated runtime producer

`hepta-ui-v4-windows-window-ack-probe` is compiled only with Windows plus the
explicit `hepta_makepad_windows_ack_hook` cfg. It deliberately does not load the
Hepta shared product window lifecycle. It submits two correlated requests:

1. Mica, sequence 1;
2. explicit solid fallback, sequence 2.

For each request it consumes the exact backend event, binds the existing Hepta
bridge to the explicit HWND and full `WindowId`, performs backdrop-only
`DWMWA_SYSTEMBACKDROP_TYPE` readback, runs the existing acknowledgement
verifier, and records a bounded receipt.

A passing probe proves only the isolated root-window hook and backdrop
readback. It does not prove full `WindowVisuals` readback, transient Acrylic,
complete profile binding, Native product runtime, physical-device validation,
or any product authority.

## Qualification boundary

Source presence may yield only
`PASS_VENDORED_MAKEPAD_WINDOWS_HOOK_SOURCE_ONLY`.

Exact patch application, patched compilation, and the Windows runtime probe
must each execute separately and bind the same Hepta commit/tree and pinned
Makepad revision. All network, mutation, effect, live-adapter, production,
operator-acceptance, promotion, and release authority remains false.
