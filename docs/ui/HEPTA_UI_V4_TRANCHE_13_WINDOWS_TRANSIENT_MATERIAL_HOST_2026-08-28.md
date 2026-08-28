# Hepta UI v4 Tranche 13 — Windows transient Acrylic host

## Scope

This tranche adds a separate, fail-closed Windows transient-material lifecycle
on top of the vendored correlated Makepad hook in PR #22.

Persistent root-window Mica and transient Acrylic remain distinct identities,
requests, receipts, and cleanup paths. The transient host accepts only a
Makepad popup window with a different full `WindowId` and HWND from the root.

## Source implementation

The new host:

- observes a popup `WindowId` before HWND delivery;
- accepts the explicit popup HWND only from the correlated backend event;
- requires monotonic request sequences and one pending request;
- supports Acrylic and explicit `DWMSBT_NONE` cleanup requests;
- validates only `DWMWA_SYSTEMBACKDROP_TYPE` readback;
- requires rollback after failed or mismatched Acrylic application;
- requires solid cleanup on synthetic focus loss;
- requires a separate solid close cleanup before `CloseWindow`;
- accepts `Destroyed` only for the exact HWND/index/generation;
- ignores stale Destroyed events for old windows;
- refuses shutdown while Acrylic or cleanup remains pending.

The root bridge continues to reject popup events. The new transient host is a
separate module and does not reinterpret root acknowledgement states.

## Isolated Windows probe

The fixture-only probe executes:

1. correlated root Mica acknowledgement;
2. creation of a separate Makepad popup;
3. correlated Acrylic acknowledgement;
4. synthetic focus-loss cleanup to `DWMSBT_NONE`;
5. a second Acrylic acknowledgement;
6. explicit close cleanup to `DWMSBT_NONE`;
7. exact Destroyed acknowledgement.

The focus-loss step is explicitly synthetic and is not physical-device evidence.
The probe starts no Matrix runtime, network request, mutation, provider, shared
product material lifecycle, or production path.

## Qualification boundary

Source presence may yield only

`PASS_WINDOWS_TRANSIENT_MATERIAL_HOST_SOURCE_ONLY`

A future passing isolated runtime receipt may prove exact root/transient HWND
separation, Acrylic backdrop acknowledgement, solid cleanup, and exact
destruction for the fixture. It still cannot claim full `WindowVisuals`
readback, product-host binding, complete Windows profile, Native product
runtime, device qualification, or any product authority.

All network, mutation, effect, live-adapter, production, operator-acceptance,
promotion, and release authority remains false.
