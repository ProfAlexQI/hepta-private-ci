# Hepta UI v4 Tranche 10 — Windows DWM acknowledgement producer

## Identity

- Base PR: #17
- Base branch: `codex/ui-v4-window-ack-matrix-20260828`
- Base commit: `947f9e82af935dc6b015b577fe6e5347330e448e`
- Base tree: `a48531929fcd929702cc2724df62745497d30190`
- Candidate branch: `codex/ui-v4-windows-dwm-ack-producer-20260828`
- Plan lane: U4 platform renderer and U8 source-bound qualification

This tranche is source-only until an assigned runner compiles it and an explicit
Makepad-root HWND binding invokes the producer on a real Windows 11 window.

## Problem corrected

The previous Windows adapter mapped the semantic `None` backdrop to
`DWMSBT_AUTO` (`0`). AUTO permits DWM to choose a material and therefore does
not provide a fail-closed solid fallback. This tranche maps `None` to
`DWMSBT_NONE` (`1`) and retains AUTO as a distinct observable value.

## Scoped acknowledgement readback

`DwmGetWindowAttribute(DWMWA_SYSTEMBACKDROP_TYPE)` can retrieve the system
backdrop enum. It cannot prove Makepad's `transparent` flag or
`backdrop_intensity`. The acknowledgement contract therefore distinguishes:

- no readback;
- backdrop-only readback;
- full-visual readback.

A Windows DWM producer emits only `BackdropOnly`. It must never construct a
`FullVisuals` readback from the DWM backdrop enum.

## Producer boundary

The producer requires an explicit binding containing:

- non-zero HWND;
- exact Makepad WindowId index;
- exact Makepad WindowId generation.

It performs no HWND discovery and does not inspect Makepad global window lists.
It validates that the request is Windows and that the exact WindowId identity
matches the host binding before reading DWM state.

The producer does not call `DwmSetWindowAttribute`. Makepad remains the owner of
the queued root-window request. The producer is intended to run after that exact
request has been processed and reads only `DWMWA_SYSTEMBACKDROP_TYPE`.

Supported root requests are bounded to:

- persistent chrome + `WindowBackdrop::Mica`;
- solid fallback + `WindowBackdrop::None`.

AUTO or Acrylic readback against a Mica request is delivered to the verifier and
rejected as a mismatch. Mica Alt is rejected because the current Makepad
`WindowBackdrop` contract has no distinct Mica Alt representation.

## Full-profile separation

This source does not bind transient Acrylic. The existing full-profile Windows
adapter still requires separate persistent and transient host handles and still
must satisfy the complete material-host receipt. A successful root-window
backdrop acknowledgement cannot set:

- `transient_system_material_bound`;
- `complete_profile_bound`;
- `system_material_bound`;
- Native product runtime or device qualification;
- production, effect, live-adapter, operator, promotion or release authority.

## Evidence states

Source presence may establish only:

`PASS_WINDOWS_DWM_ACK_PRODUCER_SOURCE_ONLY`

The following remain false until exact-candidate hosted evidence exists:

- Native compile validation;
- Windows compile validation;
- explicit HWND host binding;
- backend acknowledgement runtime validation;
- backdrop readback validation;
- full-visual readback validation;
- transient material;
- complete profile;
- system material binding;
- device validation;
- all authority and release fields.

## Next tranche

1. add a governed Makepad backend hook that supplies the exact HWND and request
   sequence without index-based discovery;
2. execute the DWM producer after the corresponding backend operation;
3. serialize a source-bound Windows acknowledgement receipt;
4. create a separate transient-window host and validate Desktop Acrylic;
5. implement the analogous AppKit acknowledgement producer;
6. keep the PR stack Draft until the same commit/tree has compile, runtime,
   readback and visual evidence.
