# Hepta Control UI P0-P6 Convergence

Date: 2026-04-27
Status: local deterministic convergence complete

## Scope

This note closes the Hepta Control UI maturation ladder from P0 through P6:

- P0: safety, performance guardrails, bounded state, mutation confirmation.
- P1: operator workflows for chat, task publishing, task drilldown, workspace room, artifacts.
- P2: polish, accessibility markers, HeptaRuntime-style parity, progressive disclosure.
- P3: regression and quality gates, functional smoke, performance budgets.
- P4: maintainability and release hygiene, asset budgets, quality smoke.
- P5: real browser viewport and interaction regression.
- P6: release walkthrough, demo narrative, evidence manifest, operator handoff.

## Operator walkthrough

1. Start local UI: `cargo run --manifest-path codex-rs/Cargo.toml -p codex-cli --bin hepta -- --serve-ui 127.0.0.1:7373`.
2. Open Chat: verify three-column desktop layout with conversation rail, thread, and Workspace Room.
3. Use command palette (`⌘K`): jump to Tasks, Developer, Ops, or Benchmark.
4. Use thread search: confirm active-result search keeps focus.
5. Load task artifacts in Workspace Room: Detail, Evidence, Replay, Promotion, Handoff.
6. Insert artifact citation into composer and confirm grouped context chips render.
7. Plan apply/rollback: confirm dry-run card appears and command copy requires explicit review checkbox.
8. Open Developer: confirm P0/P1/P2/P3/P4/P5/P6 convergence ledger is 100.
9. Run smoke: `./scripts/hepta-control-ui-smoke.sh`.

## Screenshot manifest

The browser smoke writes viewport evidence to:

- `target/hepta-control-ui-browser-smoke/desktop.png`
- `target/hepta-control-ui-browser-smoke/narrow.png`
- `target/hepta-control-ui-browser-smoke/mobile.png`
- `target/hepta-control-ui-browser-smoke/manifest.json`

The manifest records viewport dimensions, screenshot byte counts, SHA-256 hashes, and responsive Workspace Room visibility.

## Gate commands

```bash
HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 cargo test --manifest-path codex-rs/Cargo.toml -q -p hepta-core control_ui
HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway
./scripts/hepta-control-ui-smoke.sh
```

## Boundary

The UI remains local-first. It is not a hosted SaaS and does not execute external side effects by default. Mutation paths retain explicit confirmation or dry-run review boundaries.

## Result

Expected audit state:

```text
audit=100 p0=True p1=True p2=True p3=True p4=True p5=True p6=True p0p6=True convergence=100
```
