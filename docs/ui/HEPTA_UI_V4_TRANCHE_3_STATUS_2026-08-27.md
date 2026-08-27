# Hepta UI v4 — Tranche 3 status

Date: 2026-08-27  
Branch: `codex/ui-light-glass-v4-20260827`

This tranche advances U2, U4 and U8 without granting production, effect,
live-adapter, operator-acceptance, promotion or release authority.

## Delivered

### Mobile runtime controller source

`apps/hepta-control-ui/control-ui-v4-runtime.js` now provides a bounded,
network-free controller for mobile transient layers:

- focus cycling and focus restoration;
- Escape dismissal;
- bounded browser-back dismissal;
- body/document scroll lock;
- media-query transition cleanup;
- dynamic attachment for newly inserted popovers/dialogs;
- typed idle/loading/fresh/stale/partial/offline/error read states;
- bounded source, freshness and evidence-digest metadata.

The controller is intentionally a separate source asset in this tranche. The
Rust-served `/control-ui.js` continues to be byte-for-byte bound to
`apps/hepta-control-ui/control-ui.js`; therefore the new controller is not yet
claimed as production-runtime bound.

### Runtime CSS

`styles.v4.runtime.css` defines scroll locking, focus-obscuration margins,
read-state presentation, reduced-motion behavior and forced-colors behavior
without adding another backdrop-blur layer.

### Native platform runtime seam

`hepta_platform_material_runtime.rs` adds:

- compilation-target platform detection;
- transparency/high-contrast/reduced-motion preference input;
- solid fallback resolution;
- an explicit system material adapter trait;
- a fail-closed unbound adapter;
- receipt validation that rejects mismatched profiles or authority claims.

This is the runtime binding seam, not a claim that Windows, macOS, iOS or
Android system APIs have already been invoked.

### Fixture browser qualification

`scripts/hepta-ui-v4-browser-qualification.cjs` creates a local, GET-only,
same-origin fixture server and captures eight Chromium scenarios:

- 320 phone thread;
- 390 reduced-height IME simulation;
- 412 composer tools sheet;
- 600 chats navigation;
- 768 DPR 2;
- 980 compact desktop;
- 1280 desktop;
- 1440 reduced-transparency emulation.

The receipt binds candidate commit/tree, source digests, browser version,
viewport, DPR and screenshot SHA-256. It explicitly reports:

- `fixture=true`;
- `runtimeAssetInjectedForQualification=true`;
- `rustServedRuntimeAssetBound=false`;
- `rustRuntimeValidation=false`;
- `deviceValidation=false`;
- all production/effect/promotion fields false.

A fixture PASS cannot promote a visual-matrix device row or replace the Rust
server, Native compile or real-device qualification.

## Remaining next actions

1. Merge the runtime controller into the digest-bound canonical
   `apps/hepta-control-ui/control-ui.js` asset and update the existing served
   source/ETag gate.
2. Execute the fixture browser receipt on an assigned runner or local checkout.
3. Run the Native focused tests and resolve Makepad/platform integration issues.
4. Implement real system adapter calls behind the new trait.
5. Capture Rust-served Chrome/Edge/Safari and Android/iOS/macOS/Windows evidence.
6. Freeze a candidate and update only evidence-backed visual matrix rows.
