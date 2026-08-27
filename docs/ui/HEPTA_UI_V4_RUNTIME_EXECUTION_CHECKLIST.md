# Hepta UI v4 runtime execution checklist

This checklist starts only after the source candidate is frozen. Every result
must bind candidate commit/tree and evidence digest.

## Control UI

- [ ] Run `ruby scripts/hepta-ui-v4-static-gate` and store JSON receipt.
- [ ] Run generated token check.
- [ ] Run existing Control UI smoke and no-JS fallback.
- [ ] Capture Chrome at 320, 390, 412, 600, 768, 980, 1280 and 1440.
- [ ] Capture Firefox, Safari and Edge reference rows.
- [ ] Verify 200% zoom, keyboard-only focus, forced colors, increased contrast,
      reduced transparency and reduced motion.
- [ ] Verify one visible chrome layer plus at most one active transient layer.
- [ ] Verify no stable scroll surface uses backdrop blur.
- [ ] Verify mobile sheets trap focus, dismiss on Escape/back, lock background
      scroll and remain above the IME/safe area.

## Native

- [ ] Compile the Native library and run the v4 fail-closed unit test.
- [ ] Migrate remaining legacy controls whose concrete height is below 48.
- [ ] Capture login, rooms list, thread, composer, settings and modal states.
- [ ] Verify desktop/mobile adaptive transition preserves state.
- [ ] Verify Android keyboard handling and system bars.
- [ ] Verify iOS safe areas and Dynamic Type 200%.
- [ ] Verify Windows transparency-off/high-contrast fallback.
- [ ] Verify macOS Reduce Transparency/Increase Contrast fallback.

## Platform renderer adapters

- [ ] Windows Mica-like environment and transient Acrylic-like adapter.
- [ ] macOS system sidebar/titlebar/popover material adapter.
- [ ] iOS system navigation/control glass and sheet adapter.
- [ ] Android Material 3 dynamic tonal surface adapter.
- [ ] Solid fallback for unsupported or disabled transparency.

## Capability and authority

- [ ] Hydrate capability state from typed data, not copy strings.
- [ ] Render disabled/gated/unverified reasons consistently.
- [ ] Keep send, task publish, approval mutation, provider effect and promotion
      fail-closed until independent authority receipts exist.
- [ ] Verify all effect attempts are denied at UI, client and server boundaries.

## Qualification

- [ ] Replace `TO_BE_BOUND_AFTER_COMMIT` only after candidate freeze.
- [ ] Populate visual rows individually; do not bulk-convert NOT_RUN to PASS.
- [ ] Record OS, device, renderer, viewport, scale, preferences and screenshot
      SHA-256 for every PASS.
- [ ] Record blockers with reproducible reasons.
- [ ] Generate implementation receipt and independent verification result.
- [ ] Keep release and promotion false until operator acceptance exists.
