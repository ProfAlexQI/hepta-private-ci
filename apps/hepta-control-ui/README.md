# Hepta Control UI v0

Status: complete for the local v0 UI scope; the browser UI is now Rust-rendered HTML/CSS with no served JavaScript artifacts.

## Overview

Hepta Control UI is a dependency-free, static-first local frontend for Hepta's Rust-native operator surface. It ships with a smoke-checked P0-P39 convergence ledger (including the prior P0-P21 convergence ledger and P0-P29 convergence ledger), Rust/no-JS contract smoke, browser viewport regression, content-rich visual/journey regression, perceptual visual diff baseline, hardening/performance/a11y/chaos gates, schema gates, soak/leak gates, strict cross-browser readiness, golden/hostile fixtures, productized result drawer gates, and release walkthrough gates. The old Node VM functional/performance/quality smoke labels are retained only as historical gate markers; the served UI no longer depends on browser JavaScript or ESM modules.

## Interaction model

- Chat-first workspace with one independent thread per agent, @agent mentions, and shared workspaces that can act as a Telegram-style agent room.
- Shared Global Brain / isolated Workspace Context boundary.
- Workspace Room for members, tasks, artifact previews, room activity, orchestration, and dry-run task actions.
- Hepta runtime control-plane bridge inside the Workspace Room: status, sessions, tasks, approvals, events/logs, and runtime modules are represented by the Rust renderer without browser JavaScript.
- Full Hepta runtime control-plane alignment matrix maps sessions, agents, tasks/subagents/ACP, approvals/allowlists, nodes, channels/docking, cron/routines, logs/events/diagnostics, skills/tools/plugins, config/debug/instances, provider auth/media, and mobile WebChat polish into Hepta routes and safety boundaries.
- OpenClaw 2026.5.12 UI polish is represented as no-JS Rust markers: persisted auto-scroll mode, blank-dashboard recovery, compact live/idle/terminal session badges, active-tab-only Nodes polling, split sample/live-adapter readiness wording, and terminal QR rendering guards.
- Exec approvals live editor parity mirrors upstream runtime's gateway/node target, per-agent scope, ask/security mode, allowlist diff review, redacted snapshot hash, role guard, and human-gated apply bridge as a Hepta dry-run card plus bounded local endpoint evidence.
- Scoped POST task/evidence/replay/promotion/handoff paths are preferred; legacy GET task detail endpoints are internal/raw diagnostics only.
- Mobile layers: Chats → Thread → Room.
- reply-mode selector supports direct, parallel, consensus, roundtable, and debate flows.
- Command index carries deep operator/developer surfaces without crowding the primary chat UI.

## Safety model

- Local-first Rust-rendered HTML/CSS served by the Hepta binary.
- No hosted SaaS identity, public ingress, or external provider execution claim.
- Mutation surfaces require explicit confirmation or dry-run review.
- Apply/rollback stays plan-first and copy-only unless the operator executes reviewed commands outside the UI.
- Browser-side JS artifacts are removed; legacy JavaScript endpoints are not served.

## Gates

```bash
HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 cargo test --manifest-path codex-rs/Cargo.toml -q -p hepta-core control_ui_report_is_complete_and_asset_backed
HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway
./scripts/hepta-control-ui-smoke.sh
```

Main smoke covers contract, Rust/no-JS asset checks, quality, browser route safety, release, maturity, hardening, build split marker parity, cross-browser matrix markers, strict cross-browser readiness markers, perceptual diff markers, schema, soak/leak markers, a11y snapshot markers, productization, golden fixtures, deep a11y markers, hostile/XSS fixtures, product result drawer markers, JSON summary, and server checks.

## Architecture notes

The shipped UI is generated from `hepta-core::control_ui` and served as HTML/CSS by the `codex-rs` `codex-cli --bin hepta` entrypoint. The former browser modules under `apps/hepta-control-ui/modules/` have been retired as JS artifacts; `apps/hepta-control-ui/modules/README.md` remains only as a boundary ledger for the Rust renderer's absorbed responsibilities:

- chat-state
- chat-render
- workspace-room
- live-data
- task-actions
- browser-fixtures
- accessibility
- exec-approvals
- productization

Visual/layout and perceptual baselines live under `apps/hepta-control-ui/baselines/`. JSON schema gates live under `apps/hepta-control-ui/schemas/`. Golden and hostile fixtures live under `apps/hepta-control-ui/fixtures/`.

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

The UI bundle is a local Control UI v0, not a hosted SaaS dashboard. It serves Rust-rendered HTML/CSS assets and local JSON endpoints from the Hepta binary when `--serve-ui` is used. It does not open public ingress, store secrets, or claim external production deployment.

The operator security contract is local-only: the server refuses non-loopback bind addresses unless explicitly overridden for a lab, responses carry browser security headers, POST actions return dry-run confirmation plans, and the command runner is read-only allowlist based.
