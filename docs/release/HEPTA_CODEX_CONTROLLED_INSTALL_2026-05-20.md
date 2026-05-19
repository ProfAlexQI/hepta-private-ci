# Hepta Codex Controlled Install

Date: 2026-05-20
Scope: post-audit local install of `hepta-codex` without Telegram owner handoff
Status: installed; legacy Telegram owner preserved

## Installed Build

- source repo: `/Users/qianqi/.openclaw/workspace/hepta-codex`
- source commit: `5bb8577 fix: include Hepta Native icon resources`
- release binary: `codex-rs/target/release/hepta`
- installed binary: `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex`
- installed sha256: `31914dec00ca16793e396013951d350f63d5aa5cf00554c4f40690cc700e312e`

Backups created before replacement:

- binary: `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-post-audit-install-20260520-070751`
- plist: `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-post-audit-install-20260520-070751.plist`

## Boundary

This install intentionally did not change Telegram ownership.

- Old OpenClaw remains the active Telegram owner.
- Hepta Telegram poll loop gates remain disabled.
- Hepta live Telegram read/send/model-turn gates remain disabled.
- Native POST real-handler activation remains disabled.
- No provider invocation, live Telegram send, native POST mutation, external push, or owner handoff was performed.

The expected active state after this install is coexistence, not full replacement. In that state `/api/operator-security` can report `status=attention` because replacement readiness requires Hepta takeover gates and a production Telegram soak. That attention state is not an install failure when `/api/telegram-owner-handoff` reports `active_owner=legacy_openclaw`, `hepta_poll_loop_armed=false`, and `double_poller_risk=false`.

## Post-Install Smoke

Active service checks after replacement:

- `/health`: `ready`
- `/api/telegram-owner-handoff`: `legacy_owner`, `active_owner=legacy_openclaw`, `hepta_poll_loop_armed=false`, `double_poller_risk=false`
- `/api/telegram-poll-loop`: `gated`, no external read by status
- `/api/native-post-activation-plan`: disabled, blocked by `real_handler_gate_disabled`
- `/api/control-ui-route-parity`: ready, `route_count=51`, `missing_route_count=0`
- old OpenClaw `/health`: live

## Next Gate

Before any future Telegram takeover, require an explicit operator instruction naming the owner handoff. The safe sequence is still:

1. disable old OpenClaw Telegram;
2. confirm `/api/telegram-owner-handoff` has no double-poller risk;
3. arm Hepta delivery/read/model/send/poll gates together;
4. run a fresh Telegram inbound end-to-end check;
5. keep rollback backups available until the soak is green.
