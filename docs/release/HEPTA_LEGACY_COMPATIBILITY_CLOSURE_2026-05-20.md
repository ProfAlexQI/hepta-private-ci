# Hepta Legacy Compatibility Closure

Date: 2026-05-20
Scope: old standalone Hepta CLI/script family compatibility inside `hepta-codex`
Status: local route/script coverage ready; live external execution remains gated

## Purpose

The public GA gate previously carried two broad local blockers:

- old standalone Hepta CLI command breadth not fully migrated
- old release/hardening script execution compatibility not claimed

This closure narrows those blockers into a safer contract. It claims coverage of
the old command and script families through native read-only routes, inventory
reports, status gates, and dry-run plans. It does not re-enable dangerous live
execution.

## New Surface

- endpoint: `/api/hepta-legacy-compatibility-closure`
- source command label: `/hepta-legacy-compatibility-closure --json`
- script gate: `scripts/hepta-codex-legacy-compatibility-closure.sh`
- compatibility mode: `native_legacy_cli_script_compatibility_closure`

## Coverage Claim

The gate records:

- old Hepta ops files: `65`
- rough old slash/command references: `574`
- old script families: `20`
- ops file family coverage: `65`
- release/hardening script families represented as local status gates: `12`
- release/hardening status gates ready: `12`

This is a local compatibility claim, not a live-production claim. Credentialed
provider prompts, channel delivery, Telegram owner handoff, native POST real
mutation, artifact publishing, and external release remain separately blocked
until explicit approval.

## Supporting Endpoints

- `/api/hepta-cli-command-inventory`
- `/api/hepta-provider-metadata-inventory`
- `/api/hepta-runtime-session-dry-run-inventory`
- `/api/hepta-channel-adapter-status-inventory`
- `/api/hepta-local-tooling-content-inventory`
- `/api/hepta-memory-capability-absorption-inventory`
- `/api/hepta-release-hardening-status-gate`
- `/api/hepta-provider-channel-dry-run-plan`

## Safety Boundary

The closure endpoint and script must keep these side effects false:

- process spawn
- filesystem read/write
- release artifact write
- credential read
- provider/model invocation
- external network read
- channel read/send
- Telegram owner handoff/read/send
- native POST mutation
- gateway mutation
- external send

## Installed Evidence

Installed after release build:

- release sha256: `f01d17324f1c4ecf885cd3d9380d888ba5c46edc6986d159168040b09b4b3bf8`
- installed sha256: `f01d17324f1c4ecf885cd3d9380d888ba5c46edc6986d159168040b09b4b3bf8`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `19635`
- route parity: `63/63`, missing `0`
- public GA blocker count after closure: `8`

Live validation:

- `scripts/hepta-codex-legacy-compatibility-closure.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed at `current_hepta_codex_script_total=16`
  and `native_gateway_source_command_count=63`
- `scripts/hepta-codex-watchdog.sh`: passed with release/installed SHA match
- `HEPTA_CODEX_SOAK_SAMPLES=12 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=5 scripts/hepta-codex-live-soak.sh`: passed `12/12`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.IUfj3T`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`: passed,
  including Hepta Native `52` tests

External GA actions remain gated: Telegram ownership handoff, live
poll/model/send soak, native POST real mutation activation, credentialed
provider/channel live smoke, release artifact publishing, and public release
were not performed.
