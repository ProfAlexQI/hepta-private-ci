# Hepta UI v4 Tranche 26 — Rust-served Cross-browser Source Closure

This tranche closes the remaining repository-source gap between the existing ten-scenario Chromium qualification and a complete multi-engine/browser execution surface.

## Formal runtime

Every lane starts the real Rust `hepta --serve-ui` loopback server through the existing identity-safe runner. The qualifier verifies the exact build-time bundle bytes, SHA-256 ETag, single `/control-ui.js` path, absent runtime side route, ready runtime marker, local-ui-only authority, same-origin GET-only requests, and screenshot digests.

No runtime JavaScript is injected for qualification.

## Required lanes

- Chromium on Ubuntu;
- Firefox on Ubuntu;
- Playwright WebKit on Ubuntu;
- Microsoft Edge through the `msedge` channel on Windows;
- Playwright WebKit on macOS.

Each lane executes the canonical ten-scenario responsive, transient, zoom, and reduced-motion matrix.

## Evidence boundary

Playwright WebKit is browser-engine evidence, not physical Safari product evidence. A successful matrix therefore keeps:

```text
physicalBrowserAcceptance=false
safariProductValidation=false
deviceValidation=false
productWired=false
productionAuthority=false
```

Physical Edge, Firefox, and Safari screenshot/accessibility acceptance remains an external human/device evidence requirement.

## Canonical blocker result

After this tranche:

```text
sourceImplementationGaps=[]
windowsSourceControllableGapsClosed=true
globalPlatformSourceControllableGapsClosed=true
crossBrowserSourceControllableGapsClosed=true
```

The only remaining gaps are executable runner capacity, physical-device receipts, and human governance or visual acceptance.
