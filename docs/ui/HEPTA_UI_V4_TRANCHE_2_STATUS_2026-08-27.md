# Hepta UI v4 — Tranche 2 execution status

Date: 2026-08-27  
Branch: `codex/ui-light-glass-v4-20260827`  
Parent before this tranche: `ddefbbd2d7a9b35c164f285ff963e12eb66b6533`  
UI base: `647e294522a3b3341b4169e3f5a85f8f0df42cbe`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

This tranche implements the next U1–U4/U8 source slice. It remains
pre-release and fail-closed: no production caller, effect dispatch, operator
acceptance, promotion, or release authority is granted.

## Delivered

### Rust-served Control UI material binding

The previous tranche created `styles.v4.css`, but the Rust renderer constructs
`/styles.css` by concatenating named source files and did not consume that
file. Tranche 2 fixes the real runtime path without editing the large renderer:

1. `styles.v4.css` remains the canonical visual source.
2. `styles.v4.runtime.css` adds mobile sheet scroll locking and read-state CSS.
3. `styles.accessibility.base.css` preserves final system preference rules.
4. `scripts/hepta-ui-v4-runtime-css-sync` generates
   `styles.accessibility.css` from those three sources.
5. The existing Rust `include_str!(...styles.accessibility.css)` therefore
   receives v4, runtime-state rules, and final accessibility overrides in the
   correct order.
6. `styles.css` imports only the generated tail, preventing duplicate v4 rules.

This closes the “source exists but Rust runtime does not load it” defect.

### Mobile transient runtime source

`control-ui-v4-runtime.js` now implements a network-free mobile top-layer
controller with:

- Popover/sheet focus cycling and focus restoration.
- Escape dismissal.
- browser-back dismissal using a bounded history marker.
- scroll-lock state markers.
- safe media-query transition cleanup.
- a bounded `HeptaUiV4ReadState` API for idle/loading/fresh/stale/partial/
  offline/error states.

The file is syntax-gated but is **not yet concatenated into the digest-bound
Rust `/control-ui.js` asset**. The read-state contract records
`boundIntoRustServedControlUiJs=false`; no browser runtime claim is made.

### Native source migration

The Native lane now contains:

- `hepta_platform_material.rs`: per-platform semantic renderer profiles for
  Windows Mica/Acrylic, macOS system material, iOS navigation glass, Android
  dynamic tonal surfaces, Web backdrop, and solid fallback.
- `hepta_v4_controls.rs`: 48 logical-pixel text-input and icon-control
  compatibility overrides.
- `hepta_v4_layout.rs`: a 56 logical-pixel mobile stack header and a 48
  logical-pixel back target loaded after the legacy HomeScreen.
- Unit contracts that keep renderer binding, runtime verification, effect,
  production, operator acceptance, and promotion false.

These are source-ready contracts. Native compile/window/device evidence is not
claimed until a runner or local checkout executes the repository toolchain.

### Qualification infrastructure

Added:

- Runtime CSS composition generator/check.
- v4 delta/style linter with an explicit legacy-debt inventory.
- Visual receipt JSON Schema, fail-closed template, and gate.
- Read-state machine contract.
- Source-bound CI runner-block evidence.
- Expanded GitHub Actions workflow for Ruby/Node checks, generated CSS,
  static/style gates, visual template validation, Native check, and focused
  Native tests.

## Current state

| Lane | State |
|---|---|
| v4 CSS canonical source | implemented |
| Rust-served v4 stylesheet composition | source-bound |
| mobile bottom-sheet layout/scroll lock CSS | source-bound |
| mobile JS focus/back controller | source-ready, renderer-unbound |
| read-state state machine and CSS | source-ready, runtime hook unbound |
| Native platform profile | source-ready, system adapter unbound |
| Native 48/56 target migration | source-ready, compile unverified |
| Ruby syntax / Node syntax / JSON parse / CSS composition tooling | locally checked |
| Full repository static gate | pending executable checkout/runner |
| Native compile and tests | pending executable checkout/runner |
| browser screenshots and accessibility capture | `REQUIRED_NOT_RUN` |
| Android/iOS/macOS/Windows device capture | `REQUIRED_NOT_RUN` |
| production/effect/promotion | false |

## Next executable tranche

1. Bind `control-ui-v4-runtime.js` into the digest-bound Rust JS asset and
   regenerate the exact SHA-256 receipt.
2. Run the expanded static/style gates on a full repository checkout.
3. Compile Native and resolve any Makepad DSL compatibility issue from the
   late-bound control/layout overrides.
4. Add real Windows/macOS/iOS/Android platform adapter implementations behind
   the source-ready profile.
5. Capture the first Chrome 390px, Edge 125%, macOS reduced-transparency, and
   Android 360px receipts.
6. Freeze a candidate commit/tree, then update only the corresponding visual
   matrix rows with digest-bound evidence.
