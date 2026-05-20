# Hepta Release Hardening Status Gate

Date: 2026-05-20
Scope: remaining old Hepta release, external-production, ops, and hardening script families
Status: local-only status gate landed; live execution remains disabled

## Summary

This slice ports the remaining old standalone release/hardening script gap into a
machine-readable `hepta-codex` status route:

- endpoint: `/api/hepta-release-hardening-status-gate`
- source command: `/hepta-release-hardening-status-gate --json`
- script gate: `scripts/hepta-codex-release-hardening-status-gate.sh`

The route is intentionally a status gate, not an execution path. It does not
install LaunchAgents, pack release artifacts, run external production gates,
perform local imports, spawn autonomous coding subagents, touch Telegram
ownership, or mutate native POST state.

## Covered Old Script Families

The route accounts for 12 remaining old script families:

- `hepta-gateway-service*.sh`
- `hepta-watchdog-service*.sh`
- `hepta-release-artifact-pack.sh`
- `hepta-external-production-gates.sh`
- `hepta-external-production-o60-o69-gate.sh`
- `hepta-production-parity-gate.sh`
- `hepta-project-hardening-gate.sh`
- `hepta-release-architecture-gate.sh`
- `hepta-ops-status-gate.sh`
- `hepta-local-import.sh`
- `hepta-onboard-daemon-wizard-parity-gate.sh`
- `hepta-autonomous-coding-subagent-gate.sh`

All 12 now have local status-gate entries. Live execution enabled count remains
`0`.

## Safety Boundary

The endpoint reports all of the following as disabled:

- external production gate execution
- release artifact packing
- launchd service mutation
- recurring watchdog installation
- local import execution
- autonomous subagent spawn
- Telegram owner handoff/read/send
- channel read/send
- native POST mutation
- provider/model invocation
- credential read
- filesystem read/write
- external network read/send
- gateway mutation

## Script Contract

`scripts/hepta-codex-release-hardening-status-gate.sh` validates:

- release/hardening status gate is route-ready
- 12/12 local status gates are ready
- live execution enabled count is `0`
- external-production gate count is `3`
- launchd mutation required count is `3`
- artifact/local-import filesystem-write-required count is `2`
- operator approval required count is `12`
- all side-effect flags remain false
- release/hardening, memory/capability, local tooling/content, channel,
  runtime/session, provider, CLI, and merge-completion reports agree on route,
  script, and source-command counts.

## Current Counts

- current `hepta-codex` script total: `12`
- native gateway source-command count: `59`
- expected route parity after install: `59/59`

## Remaining Boundary

This closes the old-script status visibility gap, but it does not claim
production replacement. Any actual artifact packing, service install,
LaunchAgent mutation, external production push, local import, autonomous
subagent execution, Telegram owner handoff, channel delivery, or native POST
real mutation remains blocked until the operator explicitly asks for that
specific activation.
