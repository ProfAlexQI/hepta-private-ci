# Hepta Control UI v0

Status: static contract complete for local v0; browser, mutation/readback, and
live-adapter completion are not claimed.

## Overview

Hepta Control UI is a dependency-free, static-first local frontend for Hepta's Rust-native operator surface. It ships with a smoke-checked P0-P39 convergence ledger, including the prior P0-P21 convergence ledger and P0-P29 convergence ledger, Rust-embedded progressive-enhancement contract smoke, browser screenshot regression, perceptual visual diff baseline, a11y/chaos/schema gates, golden/hostile fixtures, productized result drawer gates, and release walkthrough gates. A small external script adds only same-origin, read-only inspection plus local copy/search behavior; anchors remain the no-JavaScript navigation fallback.

## Interaction model

- Chat-first workspace with independent agent threads, @agent mentions, and Telegram-style shared rooms.
- Telegram-shell first screen: the primary path is `data-control-ui-primary-path="telegram-chat-shell"` with a conversation rail, selected message thread, right-side evidence/approval panel, and bottom composer.
- Product-first runtime rail: Local review, Safety locked, and Evidence ready stay primary under `data-control-ui-runtime-rail="local-review-safety-evidence"`; route/tool navigation is collapsed under `data-control-ui-secondary-nav="collapsed"`.
- Product-first composer: the main footer keeps Plan first and safe preview visible, while reply mode and scroll controls remain one click away under `data-control-ui-composer-more="collapsed"`.
- Product-first work rail: the left conversation list is marked with `data-control-ui-work-rail="product-first"` and uses Review queue / Safety desk language instead of raw endpoint paths in the first-read surface.
- Compact narrow/mobile path: `data-control-ui-compact-product-path="narrow-mobile"` keeps the Telegram chat header, messages, and composer visible while diagnostics stay collapsed.
- Product copy guard: browser smoke fails if `old JS`, `blank module fallback`, `NO_REPLY`, `mutation=false`, or `payload hash` return.
- Read-only V2 trace fails closed; authority, live, and production state stay unverified.
- Shared Global Brain / isolated Workspace Context boundary.
- Workspace Room for members, tasks, artifact previews, room activity, orchestration, and dry-run task actions.
- Hepta runtime control-plane bridge inside the Workspace Room: status, sessions, tasks, approvals, events/logs, and runtime modules are represented by the Rust-served snapshot; only the fixed read-only registry is hydrated in-browser.
- Runtime alignment maps sessions, agents, tasks/subagents/ACP, approvals, nodes, channels, cron, logs, skills, config, provider auth/media, and mobile WebChat into Hepta routes and safety boundaries.
- Historical OpenClaw 2026.5.12 polish remains represented by Rust/static markers; the active enhancement intentionally implements only the current 21-route read-only registry, operator snapshot hydration, copy, and local search.
- Exec approvals mirror gateway/node target, per-agent scope, ask/security mode, allowlist diff, redacted snapshot hash, role guard, and human-gated apply bridge as dry-run evidence.
- Scoped POST task/evidence/replay/promotion/handoff paths are preferred; legacy GET task detail endpoints are internal/raw diagnostics only.
- Mobile layers: Chats → Thread → Room.
- reply-mode selector supports direct, parallel, consensus, roundtable, and debate flows.
- Command index carries deep operator/developer surfaces without crowding the primary chat UI.
- Merge completion audit is exposed as a read-only Control UI data source via
  `/api/hepta-merge-completion` and `/hepta-merge-completion --json`.

## Safety model

- Local-first HTML/CSS snapshot and digest-bound `/control-ui.js` asset embedded and served by the Hepta Rust binary.
- The external script uses a strict 21-route canonical GET registry, same-origin enforcement, `Accept: application/json`, latest-request-wins cancellation, an abort timeout, streaming UTF-8 byte bounds, strict JSON media types, and `textContent` rendering. It never calls a mutation endpoint.
- No hosted SaaS identity, public ingress, or external provider execution claim.
- Mutation surfaces require explicit confirmation or dry-run review.
- Apply/rollback stays plan-first and copy-only unless the operator executes reviewed commands outside the UI.
- Inline script, dynamic code execution, cross-origin fetch, and unregistered endpoint construction are prohibited. Without JavaScript, hash-anchor navigation and copy-ready command text remain available.

## Gates

```bash
HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 cargo test --manifest-path codex-rs/Cargo.toml -q -p hepta-core control_ui_report_is_complete_and_asset_backed
HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway
./scripts/hepta-control-ui-browser-smoke.sh
./scripts/hepta-control-ui-smoke.sh
./scripts/hepta-ui-product-readiness-gate.sh
```

Main smoke covers the Rust-embedded static assets and progressive-enhancement contract, all 21 allowlisted GET interactions, zero cross-origin browser requests, browser route safety, release, maturity, hardening, build split parity, cross-browser readiness, schema, soak/leak, a11y, productization, golden/hostile fixtures, product result drawer markers, JSON summary, server checks, `telegram-chat-shell`, work/runtime rail/composer markers, compact narrow/mobile markers, product-copy guards, and Chrome desktop/narrow/mobile screenshots.

The combined product-readiness gate emits source-bound JSON, screenshot, Native fixture, and packaging evidence. True Makepad-window capture remains opt-in through `HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SMOKE=1`; a locked or permission-blocked desktop must be recorded as blocked rather than promoted as passing.

## Architecture notes

`index.html` is the sole HTML snapshot. `hepta-core::control_ui` embeds its exact bytes and serves them at `/` and `/index.html`; no second HTML body exists. `/control-ui.js` is the sole executable browser asset and is separately digest-bound in the route catalog. `/styles.css` serves styles, `/assets/hepta-agent-logo.png` serves the bundled image, and `/gateway-status` serves the gateway page. Retired browser modules keep only a boundary ledger under `apps/hepta-control-ui/modules/`:

- chat-state
- chat-render
- workspace-room
- live-data
- task-actions
- browser-fixtures
- accessibility
- exec-approvals
- productization

Visual/perceptual baselines live under `baselines/`, schemas under `schemas/`, and golden/hostile fixtures under `fixtures/`.

## Screens

- Dashboard
- Config Surface
- Sessions
- Tasks
- Task Publisher
- Workers
- Operator Console
- Live Event Stream
- Conversation Transcript
- Agent Chat
- Diff Review
- Approvals
- Operator Security
- Gateway Monitor
- Hepta Operator Plane
- Multi-Agent Runtime
- Developer Console
- Artifacts
- Worker Handoff
- Ops Status
- Readiness
- Production Parity
- External Agent Benchmark
- Evidence
- Commands
- Runbook

## Run locally

```bash
cargo run --manifest-path codex-rs/Cargo.toml -p codex-cli --bin hepta -- --serve-ui 127.0.0.1:7373
```

Then open:

```text
http://127.0.0.1:7373/
```

## Boundary

The UI bundle is a local Control UI v0, not a hosted SaaS dashboard. With `--serve-ui`, it serves Rust-rendered HTML/CSS and local JSON endpoints from the Hepta binary. It does not open public ingress, store secrets, or claim external production deployment.

The operator security contract is local-only: the server refuses non-loopback bind addresses unless explicitly overridden for a lab, responses carry browser security headers, POST actions return dry-run confirmation plans, and the command runner is read-only allowlist based.
