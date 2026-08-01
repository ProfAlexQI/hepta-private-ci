# Hepta UI 2026-08-01 evidence receipt

This directory freezes the representative evidence for the desktop/mobile
light-glass productization pass. The source run was
`/Users/qianqi/.openclaw/tmp/hepta-ui-product-final-r3.20260801T1904`.

`current-source-readiness.json` is anchored to implementation commit
`39a5f6abee0c78e77155d1a6e292a27e7cbebe11` with
`source.worktree_clean=true`. It reports `ui_lane_ready=true` while preserving
`full_product_ready=false`, `backend_live_adapter_ready=false` and
`public_ga_ready=false`.

## Durable subset

- `control-desktop-1365x900.png` and `control-phone-320x844.png`: default
  Control browser surfaces at the largest and smallest accepted viewports.
- `control-phone-attachment-popover.png`: native-Popover state reached by a
  real browser click at 320 pixels.
- `control-phone-room-pane.png`: real-click mobile Room route.
- `native-desktop-base.png` and `native-mobile-base.png`: Makepad base windows.
- `native-desktop-route-actions.png` and
  `native-mobile-route-approvals.png`: Makepad desktop/mobile route variants.
- `native-desktop-secondary-search.png` and
  `native-mobile-secondary-modal.png`: Makepad secondary surfaces.
- `native-fixture-mobile-keyboard-open.png`: explicitly simulated fixture
  evidence for keyboard avoidance; it is not a real-device keyboard claim.
- `screenshot-manifest.json`: full 65-screenshot census from the source run.

The 11 PNG files are a reviewable repository subset. The readiness report and
manifest preserve hashes and metadata for the complete run. Verify this frozen
subset with `shasum -a 256 -c SHA256SUMS` from this directory.
