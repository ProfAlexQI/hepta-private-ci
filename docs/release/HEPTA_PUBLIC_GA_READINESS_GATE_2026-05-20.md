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
- public GA operator approval packet
- gateway replacement readiness
- Telegram owner handoff guard
- Telegram production readiness guard
- native POST activation and gray-release evidence

## Remaining Public GA Blockers

- Telegram owner handoff and live poll/model/send soak require explicit operator approval.
- Native POST real mutation activation requires explicit scoped approval and rollback evidence.
- Credentialed provider/search live smoke requires explicit approval and redacted evidence.
- Real channel delivery smoke requires explicit approval.
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

## Follow-Up: Legacy Compatibility Closure

After the native packaging gate, the old CLI/script family gap was promoted
into a local compatibility closure:

- endpoint: `/api/hepta-legacy-compatibility-closure`
- script: `scripts/hepta-codex-legacy-compatibility-closure.sh`
- old ops file family coverage: `65/65`
- release/hardening status gate coverage: `12/12`
- aggregate script count: `16`
- aggregate source-command count: `63`

This clears the broad old CLI breadth and release/hardening script compatibility
blockers only as local route/script/status coverage. Live provider, channel,
Telegram, native POST, artifact publishing, and external release tracks remain
separately blocked until explicit approval.

Installed after the legacy compatibility closure release build:

- release sha256: `f01d17324f1c4ecf885cd3d9380d888ba5c46edc6986d159168040b09b4b3bf8`
- installed sha256: `f01d17324f1c4ecf885cd3d9380d888ba5c46edc6986d159168040b09b4b3bf8`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `19635`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-legacy-compatibility-closure-20260521-015538`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-legacy-compatibility-closure-20260521-015538.plist`

Live checks after install:

- `/api/control-ui-route-parity`: `63/63`, missing `0`
- `/api/hepta-legacy-compatibility-closure`: `status=ready`
- `/api/hepta-public-ga-readiness`: `status=blocked`,
  `public_ga_ready=false`, `local_gate_matrix_ready=true`,
  `local_reports_synchronized=true`, blocker count `8`
- `scripts/hepta-codex-legacy-compatibility-closure.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed at `current_hepta_codex_script_total=16`
  and `native_gateway_source_command_count=63`
- `scripts/hepta-codex-watchdog.sh`: passed, release/installed SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=12 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=5 scripts/hepta-codex-live-soak.sh`: passed `12/12`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.IUfj3T`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`: passed,
  including Hepta Native `52` tests

Safety state after install:

- active Telegram owner remains `legacy_openclaw`
- Hepta Telegram poll loop remains gated
- native POST real activation remains disabled
- no provider/channel credentialed live smoke, native app signing,
  notarization, artifact publishing, or external public release was performed

## Follow-Up: Public GA Operator Approval Packet

After the legacy compatibility closure, the remaining public GA blockers were
turned into an explicit plan-only approval packet:

- endpoint: `/api/hepta-public-ga-operator-approval-packet`
- script: `scripts/hepta-codex-public-ga-operator-approval-packet.sh`
- safe default mode: `plan_only_no_live_mutation`
- required operator approval count: `8`
- aggregate script count: `17`
- aggregate source-command count: `64`

This does not reduce the remaining blocker count by itself. It makes the
approval order and rollback anchors machine-readable while continuing to block
gateway replacement, Telegram handoff/live soak, native POST real mutation,
credentialed provider/channel smoke, release artifact packing, and external
public release until explicit approval.

Installed after the operator approval packet release build:

- release sha256: `6de62f43ac4d3432207441bb85bf1713118a0bade42cdb2a3da25ac85fb9d96e`
- installed sha256: `6de62f43ac4d3432207441bb85bf1713118a0bade42cdb2a3da25ac85fb9d96e`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `25970`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-public-ga-operator-approval-packet-20260521-023824`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-public-ga-operator-approval-packet-20260521-023824.plist`

Live checks after install:

- `/api/control-ui-route-parity`: `64/64`, missing `0`
- `/api/hepta-public-ga-operator-approval-packet`: `status=ready`,
  `approval_packet_ready=true`, required approvals `8`
- `/api/hepta-public-ga-readiness`: `status=blocked`,
  `public_ga_ready=false`, `local_gate_matrix_ready=true`,
  `local_reports_synchronized=true`, blocker count `8`
- `scripts/hepta-codex-public-ga-operator-approval-packet.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed at `current_hepta_codex_script_total=17`
  and `native_gateway_source_command_count=64`
- `scripts/hepta-codex-watchdog.sh`: passed, release/installed SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=12 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=5 scripts/hepta-codex-live-soak.sh`: passed `12/12`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.PkGkb0`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`: passed,
  including Hepta Native `52` tests

## Follow-Up: Operator-Approved Evidence Flags

After the operator approval packet, the readiness gate was extended so completed
live evidence can be reflected without weakening the safe default state.

Default behavior remains unchanged: all new flags are absent/false unless an
operator-approved smoke or release action has actually completed.

New evidence flags:

- `HEPTA_PROVIDER_CREDENTIALED_SMOKE_VERIFIED=1` clears the provider/search live
  smoke blocker only after redacted credentialed smoke evidence exists.
- `HEPTA_CHANNEL_LIVE_DELIVERY_VERIFIED=1` clears channel read/send delivery
  readiness after selected channel delivery evidence exists.
- `HEPTA_CHANNEL_LIVE_READ_VERIFIED=1` and
  `HEPTA_CHANNEL_LIVE_SEND_VERIFIED=1` can represent read/send evidence
  separately.
- `HEPTA_RELEASE_ARTIFACT_PACK_VERIFIED=1` now also reflects through the
  release/hardening status gate, not only the aggregate public GA report.
- `HEPTA_PUBLIC_GA_RELEASE_APPROVED=1` reflects an explicit external public
  release approval in the release/hardening status gate.

These flags do not execute the smokes themselves and do not read credentials,
send channel messages, mutate LaunchAgents, or publish a release from the
readiness endpoint. They only make completed evidence visible to the aggregate
GA gate.
