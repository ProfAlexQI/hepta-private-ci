# Hepta UI v4 — Canonical runtime bundle guard

Date: 2026-08-28 (Asia/Singapore)  
Base branch: `codex/ui-light-glass-v4-20260827`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

## Problem closed

The UI lane already had a canonical build-time composition path:

1. `apps/hepta-control-ui/control-ui.js`;
2. `/* hepta-ui-v4-runtime-bundle-boundary */`;
3. `apps/hepta-control-ui/control-ui-v4-runtime.js`.

`codex-rs/hepta-core/build.rs` generates the one official
`control-ui.bundle.js`, its SHA-256 and ETag, and the Native Gateway exposes
only `/control-ui.js`.

A later helper, `scripts/hepta-ui-v4-runtime-js-sync`, could also write the
runtime directly into the base `control-ui.js`. Running that mutating path would
cause the build script to append the same runtime a second time. The base file
had not yet been mutated, so no duplicate runtime was present in the candidate.

## Resolution

- Build-time composition is the sole runtime binding authority.
- `control-ui.js` must remain the base controller only.
- `control-ui-v4-runtime.js` remains the canonical v4 runtime source.
- `hepta-ui-v4-runtime-js-sync` is now check-only.
- `--write` fails closed and performs no mutation.
- The gate fails if base JavaScript contains runtime markers, read-state API, or
  runtime-ready markers.
- The read-state contract records
  `bindingMode=HEPTA_CORE_BUILD_TIME_BUNDLE`,
  `boundIntoRustServedControlUiJs=true`, and
  `directSourceMutationAllowed=false`.
- The compiled bundle test remains the runtime-occurrence/order authority.

## Browser qualification

The existing Rust-served qualifier and identity-safe runner are retained. A
separate source gate and receipt Schema cover:

- exact served bytes and ETag;
- rejection of a runtime side route;
- loopback-only bounded reads;
- ten Chromium scenarios;
- stable-content blur, text floor, touch target and transparency fallbacks;
- read-state projection;
- Escape/back focus restoration;
- GET-only/same-origin traffic;
- screenshot SHA-256;
- exact candidate commit/tree;
- false device, effect, production, live-adapter, operator, promotion and
  release authority.

The manual workflow may claim Rust runtime and Chromium validation only after
real execution. It may not claim native-device or production readiness.

## Current claim boundary

| Claim | State |
|---|---|
| Canonical build-time bundle source | implemented |
| Direct base-script mutation | prohibited |
| Runtime occurrence/order compile test | implemented, execution pending |
| Rust-served browser source gate | implemented |
| Receipt Schema | implemented |
| Chromium execution | `REQUIRED_NOT_RUN` |
| Native platform/device execution | `REQUIRED_NOT_RUN` |
| Production/effect/live adapter/operator/promotion/release | false |
