# Hepta UI v4 Tranche 24 — Product-host safety and physical-device closure

This tranche closes the remaining source-level safety gaps between the disabled
Windows product-host candidate and a governed physical-device qualification.
It remains stacked on PR #52 and does not wire the product.

## Closed safety gaps

The earlier candidate required `physical_device_validated=true` and
`rollback_drill_validated=true` before it could bind the windows that must be
used to produce those facts. The qualification host separates the device drill
from future product activation: device and rollback validation are outputs.

The earlier candidate also discarded the active window identity after rollback
failure. A later suspend or shutdown could therefore publish an unbound receipt
without proving that DWM had returned both windows to `None`. The qualification
host retains the exact identity and `rollback_required=true` until a verified
retry succeeds. `RejectedUnsafe` cannot transition to a safe lifecycle state.

The external Review Envelope is represented by a seal containing its evidence
candidate and binding digest. The implementation approval must bind the exact
implementation candidate and the same review digest before the backend is
called.

## Governed device producer

The qualification-only Windows binary creates a real root top-level window and
an independent popup window, applies and reads back Mica and Acrylic through
DWM, rolls both back to `DWMSBT_NONE`, and executes high-contrast,
transparency-disabled, suspend, and shutdown rollback drills. It writes the
existing physical-device receipt schema only after the final state is safely
unbound.

The binary starts no Matrix runtime, network client, provider, mutation path,
or product lifecycle. The workflow is manual-only and requires a valid review
envelope artifact plus explicit implementation-review and operator inputs on a
runner labelled `self-hosted`, `Windows`, `X64`, and `hepta-ui-dwm`.

## Permanent boundary

Even a physical-device PASS keeps product Cargo declaration, module
registration, lifecycle wiring, automatic binding, product binding, system
material binding, Native product runtime, effect, production, promotion, and
release false. It only supplies the missing device artifact to PR #52's
qualification producer.
