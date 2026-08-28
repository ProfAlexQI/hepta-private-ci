# Hepta UI v4 Tranche 14 — Windows transient host and profile aggregate

## Scope

This tranche implements both the fail-closed transient evidence producer and
the dual-receipt aggregator needed before any Windows material product-host
integration can be reviewed.

The evidence inputs are:

1. the persistent root-window Mica acknowledgement;
2. a dedicated popup lifecycle containing Acrylic acknowledgement, explicit
   `DWMSBT_NONE` rollback, and exact destroyed acknowledgement.

A successful aggregate means only `ReadyForProductIntegrationReview`.

## Transient host

`HeptaWindowsTransientMaterialHost` is a pure Rust state machine with no Script
registration and no product-lifecycle attachment. It requires a parent identity
first, then one monotonic Acrylic request, one later solid-rollback request, and
an exact destroyed event.

The popup HWND must be non-zero and different from the root HWND. Backend apply
failure is consumed before DWM readback. Acrylic must read back Acrylic; rollback
must read back `None`. Closing is forbidden until rollback is acknowledged.
Only the `Closed` phase can export `profile_evidence()`.

## Required identity binding

The root receipt must bind the same non-zero HWND and full Makepad `WindowId`
index/generation supplied by the governed backend bridge. The transient parent
must match that root identity exactly. The transient destroyed identity must
match the same popup HWND, index, and generation.

## Required receipt ordering

The root receipt must be an exact Windows DWM Mica `BackdropOnly` receipt. The
transient Acrylic request sequence must be lower than the solid rollback
sequence. Missing destroy, identity drift, receipt drift, root-HWND reuse, or
any authority flag rejects the aggregate.

## Authority boundary

Even a successful aggregate keeps product binding, transient system-material
binding, complete-profile binding, Native product runtime, device validation,
production authority, operator acceptance, promotion, and release false.

A later, separate tranche may consume a runtime-qualified aggregate as an input
to `HeptaPlatformMaterialHost`; this source tranche does not do so.
