# Hepta Codex Controlled Install

Date: 2026-05-20
Scope: post-audit local install of `hepta-codex` without Telegram owner handoff
Status: installed; legacy Telegram owner preserved; native POST dry-run canaries recorded

## Installed Build

- source repo: `/Users/qianqi/.openclaw/workspace/hepta-codex`
- source commit: `ed61f2b docs: record Hepta controlled install`
- release binary: `codex-rs/target/release/hepta`
- installed binary: `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex`
- installed sha256: `8aa6dd230a83054eb8eba528635cc8346e2e1d337fd91c8b941bb04dea8af333`

Backups created before replacement:

- binary: `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-post-audit-install-20260520-070751`
- binary: `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-coexistence-fields-20260520-074900`
- plist: `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-post-audit-install-20260520-070751.plist`
- plist: `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-coexistence-fields-20260520-074900.plist`

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

Continuation note: the later merge-completion/API inventory installs upgraded
the same controlled coexistence service first to `route_count=52`, then to a
planned `route_count=53` with `/api/hepta-cli-command-inventory`; see
`HEPTA_MERGE_COMPLETION_API_INSTALL_2026-05-20.md`.

After the coexistence-status patch, `/api/operator-security` intentionally remains
`status=attention` while reporting:

- `security_mode=legacy_owner_coexistence_ready`
- `legacy_owner_coexistence_ready=true`
- `attention_reason=telegram_replacement_not_requested`

That is the expected local install posture until an operator explicitly asks for
Telegram owner handoff.

## Native POST Dry-Run Canaries

Scoped single-handler native POST canaries were run through the installed binary
on temporary loopback servers, using the active native POST execution store. Each
temporary process selected exactly one handler and had only these native POST
gates enabled:

- `HEPTA_NATIVE_POST_REAL_HANDLERS=1`
- `HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED=1`
- `HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE=<single handler>`

The active `ai.hepta.gateway` service was not reconfigured and still reports
`activation_currently_enabled=false` with `activation_blocked_reason=real_handler_gate_disabled`.

Canary result for `task_publish`:

- route: `POST /api/tasks/publish`
- body mode: `confirm=true`, `dry_run=true`
- handler: `task_publish`
- harness status: `dry_run_recorded`
- gray release endpoint under the scoped temp process: `gray_release_ready=true`, `gray_release_phase=gray_release_ready`
- rollout evidence: `rollout_evidence_ready=true`
- execution stores after the first canary: `store_jsonl_valid=true`, `store_capacity_ok=true`, `total_line_count=8`
- no mutation: `task_published=false`, `real_mutation_performed=false`
- no external side effects: `external_side_effects=false`
- redaction held: `raw_request_body_exposed=false`, `raw_idempotency_key_exposed=false`

Canary result for `approval_apply`:

- route: `POST /api/approvals/exec/apply`
- body mode: `confirm=true`, `dry_run=true`
- handler: `approval_apply`
- harness status: `dry_run_recorded`
- gray release endpoint under the scoped temp process: `gray_release_ready=true`
- no mutation: `task_published=false`, `message_sent=false`
- no external side effects: `external_side_effects=false`
- redaction held: `raw_request_body_exposed=false`

Canary result for `chat_send`:

- route: `POST /api/chat`
- body mode: `confirm=true`, `dry_run=true`
- handler: `chat_send`
- harness status: `dry_run_recorded`
- gray release endpoint under the scoped temp process: `gray_release_ready=true`
- no mutation: `task_published=false`, `message_sent=false`, `chat_mutated=false`
- no external side effects: `external_side_effects=false`
- redaction held: `raw_request_body_exposed=false`

Execution store after all three handlers:

- `store_jsonl_valid=true`
- `store_capacity_ok=true`
- `total_line_count=16`
- per-store line count: `4` each in `audit.jsonl`, `idempotency.jsonl`, `rate-limit.jsonl`, and `rollback.jsonl`
- raw body and idempotency exposure remained false

Active-service boundary after the canary:

- `/health`: `ready`
- `/api/telegram-owner-handoff`: `active_owner=legacy_openclaw`, `hepta_poll_loop_armed=false`, `double_poller_risk=false`
- `/api/telegram-poll-loop`: `status=gated`, no external read or send by status
- `/api/native-post-activation-plan`: `activation_currently_enabled=false`

## Next Gate

Before any future Telegram takeover, require an explicit operator instruction naming the owner handoff. The safe sequence is still:

1. disable old OpenClaw Telegram;
2. confirm `/api/telegram-owner-handoff` has no double-poller risk;
3. arm Hepta delivery/read/model/send/poll gates together;
4. run a fresh Telegram inbound end-to-end check;
5. keep rollback backups available until the soak is green.
