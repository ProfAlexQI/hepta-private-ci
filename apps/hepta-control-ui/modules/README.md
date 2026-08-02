# Hepta Control UI module boundaries

This directory is a boundary ledger only. Historical browser modules remain retired; the served UI is a Rust-embedded HTML/CSS snapshot with one separately governed external `/control-ui.js` progressive-enhancement asset.

## Boundaries absorbed by Rust

- `chat-state` — local state schema, migration, pruning, selected conversation, mobile pane state.
- `chat-render` — conversation rail, thread, composer, grouped replies, search rendering.
- `workspace-room` — room members, tasks, artifact preview, activity log, onboarding empty state.
- `live-data` — per-screen endpoint hydration, cached/degraded endpoint recovery.
- `task-actions` — scoped read-only artifact fetch and dry-run apply/rollback plans.
- `browser-fixtures` — content-rich seeded visual regression and journey evidence.
- `accessibility` — keyboard path, dialog semantics, labels, mobile layer tabs, reduced motion.
- `exec-approvals` — Hepta exec approvals target/scope editor, hash-rechecked apply preview, and bounded `POST /api/approvals/exec/apply` evidence contract.
- `productization` — P30-P39 policy extraction for strict cross-browser readiness, smoke summary, density, golden/hostile fixtures, and result drawer actions.

## Rule

Do not add browser modules here. The sole active script is `apps/hepta-control-ui/control-ui.js`; changes must preserve its strict same-origin read-only registry, update the generated route catalog/digest, update Rust checks in `codex-rs/hepta-core/src/control_ui.rs`, and keep `./scripts/hepta-control-ui-smoke.sh` green.

The static-first delivery rule remains: no browser module may become a second renderer or an ungoverned live client, and navigation must keep working when the enhancement script is unavailable.
