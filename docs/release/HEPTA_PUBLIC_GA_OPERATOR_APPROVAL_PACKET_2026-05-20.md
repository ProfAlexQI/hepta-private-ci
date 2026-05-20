# Hepta Public GA Operator Approval Packet

Date: 2026-05-20
Scope: plan-only approval packet for the remaining public GA blockers
Status: ready as a local packet; live/external actions remain gated

## Purpose

After the legacy compatibility closure, the local Hepta gate matrix is ready.
The remaining public GA blockers are intentionally approval-bound production
actions. This packet makes those approvals explicit and ordered without
performing any of them.

## New Surface

- endpoint: `/api/hepta-public-ga-operator-approval-packet`
- source command label: `/hepta-public-ga-operator-approval-packet --json`
- script gate: `scripts/hepta-codex-public-ga-operator-approval-packet.sh`
- compatibility mode: `native_public_ga_operator_approval_packet`

## Required Approvals

The packet tracks eight explicit approvals:

- gateway replacement plan and rollback anchor
- Telegram owner handoff from legacy OpenClaw to Hepta
- live Telegram poll/model/send soak
- one scoped native POST real mutation handler
- credentialed provider/search live smoke with redacted evidence
- real channel delivery smoke for selected adapters
- release artifact pack creation/signing/notarization
- external public GA release publication

## Safety Boundary

The packet is plan-only. Its safe default mode is
`plan_only_no_live_mutation`, and irreversible actions are blocked by default.

The endpoint and script keep these side effects false:

- public release publishing
- release artifact write
- launchd mutation
- credential read
- provider/model invocation
- channel read/send
- Telegram owner handoff/read/send
- native POST mutation
- process spawn
- filesystem read/write
- gateway mutation
- external network read
- external send

## Verification

Run:

```bash
scripts/hepta-codex-public-ga-operator-approval-packet.sh
```

The script passes when the packet is synchronized with
`/api/hepta-public-ga-readiness`, still blocks public GA by default, and lists
the remaining approval-bound blockers without executing them.

## Installed Evidence

Installed after release build:

- release sha256: `6de62f43ac4d3432207441bb85bf1713118a0bade42cdb2a3da25ac85fb9d96e`
- installed sha256: `6de62f43ac4d3432207441bb85bf1713118a0bade42cdb2a3da25ac85fb9d96e`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `25970`
- route parity: `64/64`, missing `0`
- public GA blocker count: `8`

Live validation:

- `scripts/hepta-codex-public-ga-operator-approval-packet.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed at `current_hepta_codex_script_total=17`
  and `native_gateway_source_command_count=64`
- `scripts/hepta-codex-watchdog.sh`: passed with release/installed SHA match
- `HEPTA_CODEX_SOAK_SAMPLES=12 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=5 scripts/hepta-codex-live-soak.sh`: passed `12/12`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.PkGkb0`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`: passed,
  including Hepta Native `52` tests

Safety state remained unchanged: Telegram owner is still `legacy_openclaw`,
Hepta Telegram polling is gated, native POST real activation is disabled, and
no credentialed provider/channel smoke, native signing/notarization, artifact
publishing, or external public release was performed.
