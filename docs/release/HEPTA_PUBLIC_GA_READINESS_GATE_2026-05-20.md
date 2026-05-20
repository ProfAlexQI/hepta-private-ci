# Hepta Public GA Readiness Gate

Date: 2026-05-20
Scope: `hepta-codex` public-GA readiness gate

## Purpose

This gate makes the public GA boundary explicit and machine-readable.
It does not publish a release, read credentials, call providers, send channel
messages, hand off Telegram ownership, enable native POST real mutations, write
release artifacts, mutate LaunchAgents, or perform external network actions.

## New Surface

- endpoint: `/api/hepta-public-ga-readiness`
- source command label: `/hepta-public-ga-readiness --json`
- validation script: `scripts/hepta-codex-public-ga-readiness.sh`
- compatibility mode: `native_public_ga_readiness_gate`

## Current Result

The gate is expected to return `status=blocked` and `public_ga_ready=false`
until the remaining operator-approved external tracks are completed.

Local evidence currently synchronized by the gate:

- route parity: no missing routes
- merge/function completion report
- CLI command breadth inventory
- provider/search metadata inventory
- runtime/task/session dry-run inventory
- channel adapter disabled-status inventory
- local tooling/content planning inventory
- memory/capability absorption inventory
- release/hardening status gate
- provider/channel/runtime dry-run plan
- Hepta Native packaging gate
- gateway replacement readiness
- Telegram owner handoff guard
- Telegram production readiness guard
- native POST activation and gray-release evidence

## Remaining Public GA Blockers

- Telegram owner handoff and live poll/model/send soak require explicit operator approval.
- Native POST real mutation activation requires explicit scoped approval and rollback evidence.
- Credentialed provider/search live smoke requires explicit approval and redacted evidence.
- Real channel delivery smoke requires explicit approval.
- Old standalone Hepta CLI command breadth is not claimed fully migrated.
- Old release/hardening script execution compatibility is not claimed.
- Release artifact packing and external public release are not approved.

Cleared locally after the follow-up native packaging gate:

- Hepta Native source/package metadata/local smoke readiness is represented by
  `/api/hepta-native-packaging-gate`.

## Verification

Run:

```bash
scripts/hepta-codex-public-ga-readiness.sh
```

The script should pass when the report is synchronized and safely blocked.
It should not be interpreted as a public GA claim until the endpoint itself
returns `public_ga_ready=true` and external public release approval is explicit.

## Installed Evidence

Installed after release build:

- release sha256: `944b3d6006894bbd6cf0ca4e4eb51b392de655f70f8befd8d2537dd6b69a7a53`
- installed sha256: `944b3d6006894bbd6cf0ca4e4eb51b392de655f70f8befd8d2537dd6b69a7a53`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `5986`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-public-ga-readiness-20260521-001359`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-public-ga-readiness-20260521-001359.plist`

Live checks after install:

- `/health`: `ready`
- `/api/control-ui-route-parity`: `61/61`, missing `0`
- `/api/hepta-public-ga-readiness`: `status=blocked`, `public_ga_ready=false`, `local_gate_matrix_ready=true`, `local_reports_synchronized=true`, blocker count `11`
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed at `current_hepta_codex_script_total=14` and `native_gateway_source_command_count=61`
- `scripts/hepta-codex-watchdog.sh`: passed, release/installed SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`: passed `3/3`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.AbyfwW`

Safety state after install:

- active Telegram owner remains `legacy_openclaw`
- Hepta Telegram poll loop remains gated
- native POST real activation remains disabled
- old OpenClaw health remains live at `127.0.0.1:18789`

## Follow-Up: Hepta Native Packaging Gate

After the initial public GA readiness gate, the local Hepta Native packaging
track was promoted into the live matrix:

- endpoint: `/api/hepta-native-packaging-gate`
- script: `scripts/hepta-codex-native-packaging-gate.sh`
- local native package gate: `ready`
- public distribution artifact written: `false`
- signing/notarization/stapling: deferred
- aggregate script count: `15`
- aggregate source-command count: `62`
- route parity: `62/62`
- public GA blocker count: `10`

This clears the local native packaging readiness blocker while keeping release
artifact creation and external public release behind explicit approval.
