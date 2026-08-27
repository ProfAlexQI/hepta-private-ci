# Hepta UI v4 implementation lane

This directory is the source-bound planning and qualification index for the
2026-08-27 light-glass UI implementation lane.

## Authority

The lane may change local UI source, static contracts, read-only previews,
tests, and evidence preparation. It does not grant production caller, writer,
effect, live-adapter, operator-acceptance, promotion, or release authority.

## Files

- `HEPTA_UI_DEVELOPMENT_PLAN_V4_2026-08-27.md` — complete UI execution plan and exact source binding.
- `HEPTA_UI_STATE_AUTHORITY_MATRIX_V1.json` — machine-readable capability and fail-closed authority truth.
- `HEPTA_VISUAL_QUALIFICATION_MATRIX_V1.json` — desktop/mobile target matrix; unexecuted rows remain `REQUIRED_NOT_RUN`.
- `HEPTA_UI_V4_IMPLEMENTATION_RECEIPT_TEMPLATE.json` — post-freeze receipt template, not a qualification result.
- `HEPTA_UI_V4_TRANCHE_2_STATUS_2026-08-27.md` — runtime CSS and Native source migration status.
- `HEPTA_UI_V4_TRANCHE_3_STATUS_2026-08-27.md` — runtime controller and platform-seam preparation status.
- `HEPTA_UI_V4_TRANCHE_4_STATUS_2026-08-27.md` — formal served bundle and Rust-served browser qualification status.
- `schemas/hepta.ui.v4.browser-qualification-receipt.v1.schema.json` — fixture-only browser receipt.
- `schemas/hepta.ui.v4.rust-served-browser-qualification.v1.schema.json` — formal Rust-served browser receipt.

The material contract lives at
`design-tokens/hepta-material-v4.contract.json`. The canonical Web and Native
overlays live at `apps/hepta-control-ui/styles.v4.css` and
`apps/hepta-native/src/shared/hepta_v4.rs`.

The source gates are:

- `scripts/hepta-ui-v4-static-gate`;
- `scripts/hepta-ui-v4-next-source-gate`;
- `scripts/hepta-ui-v4-tranche3-source-gate`;
- `scripts/hepta-ui-v4-served-bundle-gate`;
- `scripts/hepta-ui-v4-rust-served-browser-source-gate`.

Browser qualification is deliberately split:

- `scripts/hepta-ui-v4-browser-qualification.cjs` validates a local fixture and
  may not claim Rust runtime binding;
- `scripts/hepta-ui-v4-rust-served-browser-qualification.sh` builds and starts
  the real Rust UI server with isolated state and identity-safe cleanup;
- `scripts/hepta-ui-v4-rust-served-browser-qualification.cjs` validates the
  exact `/control-ui.js` bytes, ETag, runtime behavior, request boundary,
  accessibility/material constraints, and screenshots.
