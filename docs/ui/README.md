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

The material contract lives at
`design-tokens/hepta-material-v4.contract.json`; the Web and Native overlays
live at `apps/hepta-control-ui/styles.v4.css` and
`apps/hepta-native/src/shared/hepta_v4.rs`; the static gate is
`scripts/hepta-ui-v4-static-gate`.
