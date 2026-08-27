# Hepta UI v4 — Tranche 4 qualification status

Date: 2026-08-27  
Branch: `codex/ui-light-glass-v4-20260827`  
UI base: `647e294522a3b3341b4169e3f5a85f8f0df42cbe`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

Status: `RUST_SERVED_QUALIFICATION_SOURCE_READY_EXECUTION_REQUIRED`

This record supersedes the earlier Tranche 4 note that listed Rust-served
browser qualification as future work. The formal bundle, canonical smoke
binding, ten-scenario browser qualifier, and identity-safe runner are now
present in source. No compile, browser, desktop-OS, mobile-device, effect,
production, operator-acceptance, promotion, or release PASS is inferred until
an executable receipt binds the same commit and tree.

## Delivered

- A single generated `/control-ui.js` containing the base controller, an exact
  boundary, and the network-free v4 runtime.
- Generated base/runtime/bundle SHA-256 values and a bundle-bound ETag.
- Native Gateway exposure through one official script route only.
- A compiled `hepta-core` integration test for bundle order and markers.
- Existing canonical browser smoke comparison against the exact served bundle.
- A loopback-only Rust-served browser qualifier with bounded asset reads.
- Ten Chromium scenarios covering 320, 390, 412, 600, 768, 980, 1280 and
  1440 widths, simulated IME, 200% font scale, reduced transparency, forced
  colors, and reduced motion.
- GET-only/same-origin request auditing, stable-content blur checks, 12px text
  floor, 48px mobile target checks, focus visibility, read-state projection,
  Escape/back focus restoration, and screenshot SHA-256.
- An identity-safe runner using bounded loopback ports, isolated runtime stores,
  safe output resolution, and exact candidate commit/tree binding.
- A machine-readable receipt Schema and source gate.
- A manual GitHub Actions job that keeps runtime/browser evidence separate from
  source and device claims.

## Claim boundary

| Claim | State |
|---|---|
| Formal runtime bundle source-bound | true |
| Exact bundle SHA-256/ETag source-bound | true |
| Qualifier and safe runner source-ready | true |
| Source gate execution | pending assigned runner |
| Rust compile/runtime validation | `REQUIRED_NOT_RUN` |
| Chromium screenshot qualification | `REQUIRED_NOT_RUN` |
| Windows/macOS material execution | `REQUIRED_NOT_RUN` |
| Android/iOS device qualification | `REQUIRED_NOT_RUN` |
| Production/effect/live adapter/operator acceptance/promotion/release | false |

## Next executable tranche

1. Execute source gates and Rust/Native compile tests on an assigned runner.
2. Manually dispatch the Rust-served browser job for the same candidate.
3. Validate the receipt against the committed Schema and inspect all ten
   screenshots before changing any visual-matrix row.
4. Fix actual runtime failures rather than weakening typography, touch-target,
   backdrop, network, or authority assertions.
5. Bind real platform accessibility preference readers and system material
   adapters, then capture Windows, macOS, Android, and iOS evidence.
