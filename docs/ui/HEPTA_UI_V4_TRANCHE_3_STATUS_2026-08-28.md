# Hepta UI v4 — Tranche 3 execution status

Date: 2026-08-28 (Asia/Singapore)  
Branch: `codex/ui-light-glass-v4-20260827`  
Starting parent: `dee119fd3c1b9bcbc09bb039ea1c9c65cd841834`  
UI base: `647e294522a3b3341b4169e3f5a85f8f0df42cbe`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

This tranche implements the next executable U1–U4/U8 slice. It does not grant
production caller, mutation, effect, live-adapter, operator-acceptance,
promotion, or release authority.

## Delivered

### Control UI runtime binding

- Added `control-ui-v4-runtime.js` as the canonical, network-free runtime
  source for mobile top-layer behavior and read states.
- Bound that source into the existing digest-controlled `control-ui.js` asset
  between deterministic start/end markers.
- Added `scripts/hepta-ui-v4-runtime-js-sync` so the binding is reproducible and
  duplicate or stale blocks fail closed.
- The runtime implements:
  - one active mobile transient layer;
  - focus entry, focus cycling, and focus restoration;
  - Escape dismissal;
  - browser-back dismissal through a bounded history marker;
  - background scroll locking and restoration;
  - media-query transition cleanup;
  - `idle/loading/fresh/stale/partial/offline/error` read states;
  - explicit `productionAuthority=false`, `effectAuthority=false`,
    `liveAdapterAuthority=false`, and `promotion=false`.

### Control UI stylesheet binding

- Added `styles.v4.runtime.css` for executable sheet and read-state markers.
- Preserved the pre-v4 accessibility source as
  `styles.accessibility.base.css`.
- Added `scripts/hepta-ui-v4-runtime-css-sync` to generate the final served tail
  in this order:
  1. semantic v4 material overlay;
  2. executable runtime state layer;
  3. reduced-transparency, forced-colors, contrast, and reduced-motion rules.
- Kept `styles.css` as a single import path to the generated accessibility tail,
  avoiding duplicate v4 overlays.

### Native platform and adaptive primitives

- Added `hepta_platform_material.rs`:
  - Windows Mica/Acrylic intent;
  - macOS window/sidebar/popover intent;
  - iOS system background/navigation/sheet intent;
  - Android dynamic tonal/elevation intent;
  - Web backdrop intent;
  - solid fallback for content, disabled transparency, high contrast, Linux,
    unknown platforms, and unavailable dynamic color.
- Added `hepta_v4_controls.rs` with 48 logical-pixel text, icon, and primary
  control primitives.
- Added `hepta_v4_layout.rs` with a 56 logical-pixel mobile header, 48 logical-
  pixel back target, 280 logical-pixel desktop rail floor, and stable-content
  container.
- Registered the new widget modules after the legacy style graph.
- All platform profiles remain `system_adapter_bound=false` and
  `runtime_verified=false` until platform-specific adapters and evidence exist.

### Executable qualification chain

- Added a source/style lint covering backdrop blur, fixed top coordinates,
  reduced-transparency/motion/forced-colors fallbacks, network-free runtime,
  authority flags, and Native metric floors.
- Added a dependency-free CDP browser matrix for:
  - 320×800;
  - 390×844;
  - 412×915;
  - 600×960;
  - 768×1024 at DPR 2;
  - 980×800;
  - 1280×800;
  - 1440×900.
- The matrix verifies horizontal reflow, 15/15/12 typography floors, 44/48
  control targets, stable-content no-blur, one-transient budget, mobile sheet
  focus/Escape/scroll behavior, read-state transitions, console errors,
  cross-origin requests, and non-GET requests.
- Added a source-static browser runner, JSON Schema, receipt gate, screenshot
  SHA-256 binding, candidate commit/tree binding, and bounded claims.
- Expanded `.github/workflows/hepta-ui-v4.yml` to run source bindings, style
  lint, browser matrix, Native compile, and focused Native contracts.

## Claim boundaries

| Claim | State |
|---|---|
| Runtime JS source binding | implemented; CI verification required |
| Runtime CSS source binding | implemented; CI verification required |
| Source-static browser matrix | executable; result pending CI |
| Rust-served browser matrix | not yet claimed |
| Native compile | executable in CI; result pending |
| Windows/macOS/iOS/Android system material adapter | not bound |
| Device screenshots | `REQUIRED_NOT_RUN` |
| Production/effect/promotion | false |

## Next tranche after CI feedback

1. Fix every source-binding, Node syntax, Makepad DSL, Native compile, or browser
   matrix failure against the exact candidate.
2. Run the same browser assertions through the Rust-served Control UI route and
   issue a separate `RUST_SERVED_BROWSER_ONLY` receipt.
3. Implement operating-system adapter calls behind the fail-closed material
   resolver without changing semantic roles.
4. Capture macOS desktop, Windows 125%/high contrast, Android 360/412, and iOS
   Dynamic Type/reduced-transparency rows.
5. Update individual visual-matrix rows only when commit/tree/environment and
   screenshot digests exist.
6. Keep the PR draft and keep all production/effect flags false until independent
   operator acceptance and promotion receipts are present.
