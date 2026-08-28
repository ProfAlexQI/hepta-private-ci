# Hepta UI v4 Tranche 14 — Windows material-profile evidence aggregate

## Scope

This tranche adds a fail-closed aggregator for two independently produced
Windows evidence sets:

1. the persistent root-window Mica acknowledgement;
2. the dedicated transient-window lifecycle: Acrylic acknowledgement, explicit
   `DWMSBT_NONE` rollback, and exact destroyed acknowledgement.

The aggregator is intentionally not a product material adapter. A successful
result means only `ReadyForProductIntegrationReview`.

## Required identity binding

The root receipt must bind the same non-zero HWND and full Makepad `WindowId`
index/generation supplied by the governed backend bridge. The transient parent
must match that root identity exactly. The transient HWND must be non-zero and
different from the root HWND, and its destroyed identity must match the same
transient generation.

## Required receipt ordering

The root receipt must be an exact Windows DWM Mica `BackdropOnly` receipt. The
transient evidence must contain exact Acrylic and `None` readbacks, with the
solid rollback sequence strictly after the Acrylic sequence. Missing destroy,
identity drift, receipt drift, or any authority flag rejects the aggregate.

## Authority boundary

Even a successful aggregate keeps product binding, transient system-material
binding, complete-profile binding, Native product runtime, device validation,
production authority, operator acceptance, promotion, and release false.

A later, separate tranche may consume a runtime-qualified aggregate as an input
to `HeptaPlatformMaterialHost`; this source tranche does not do so.
